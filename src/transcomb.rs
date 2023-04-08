//! # Transformation Combinator

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::evaluate::{is_node_match, ArithmeticOperator, Axis, NodeMatch};
use crate::item::{Item, Node, NodeType, Sequence, SequenceTrait};
use crate::qname::QualifiedName;
use crate::value::{Operator, Value};
use crate::xdmerror::*;

pub(crate) type TransResult<N> = Result<Sequence<N>, Error>;

/// The transformation context (i.e. the dynamic context, plus some parts of the static context)
// Idea: instead of having one dynamic context that is mutable,
// make the context immutable but with shared components. Then when a new context is required, clone it and add in extra components
#[derive(Clone)]
pub struct Context<N: Node> {
    seq: Sequence<N>, // The current context
    i: usize,         // Which item in the sequence is the current context
    depth: usize,     // Depth of evaluation
    rd: Option<N>,    // Result document
    // No distinction between built-in and stylesheet-defined templates. Built-in templates have no priority and no document order.
    templates: Vec<Rc<Template<N>>>,
    //builtin_templates: Vec<Rc<Template<N>>>,
    current_templates: Vec<Rc<Template<N>>>,
    // variables
    vars: HashMap<String, Vec<Sequence<N>>>,
    // grouping
    // output defn
    // base URI
}

impl<N: Node> Context<N> {
    pub fn new() -> Self {
        Context {
            seq: Sequence::new(),
            i: 0,
            depth: 0,
            vars: HashMap::new(),
            templates: Vec::new(),
            current_templates: Vec::new(),
            rd: None,
        }
    }
    //    pub fn copy_with_sequence(&self, s: Sequence<N>) -> Self {
    //        Context {
    //            seq: s,
    //            i: self.i,
    //            depth: self.depth,
    //	    import: self.import,
    //            vars: self.vars.clone(),
    //            templates: self.templates.clone(),
    //            builtin_templates: self.builtin_templates.clone(),
    //            current_templates: self.current_templates.clone(),
    //            rd: self.rd.clone(),
    //        }
    //    }
    //    pub fn copy_with_import(&self, i: usize) -> Self {
    //        Context {
    //            seq: self.seq.clone(),
    //            i: self.i,
    //            depth: self.depth,
    //	    import: i,
    //            vars: self.vars.clone(),
    //            templates: self.templates.clone(),
    //            builtin_templates: self.builtin_templates.clone(),
    //            current_templates: self.current_templates.clone(),
    //            rd: self.rd.clone(),
    //        }
    //    }
    //    pub fn copy_with_current(&self, c: &Vec<Template<N>>) -> Self {
    //        Context {
    //            seq: self.seq.clone(),
    //            i: self.i,
    //            depth: self.depth,
    //	    import: self.import,
    //            vars: self.vars.clone(),
    //            templates: self.templates.clone(),
    //            builtin_templates: self.builtin_templates.clone(),
    //            current_templates: c.clone(),
    //            rd: self.rd.clone(),
    //        }
    //    }

    fn var_push(&mut self, name: String, value: Sequence<N>) {
        match self.vars.get_mut(name.as_str()) {
            Some(u) => {
                // If the variable already has a value, then this is a new, inner scope
                u.push(value);
            }
            None => {
                // Otherwise this is the first scope for the variable
                self.vars.insert(name, vec![value]);
            }
        }
    }
    fn var_pop(&mut self, name: String) {
        self.vars.get_mut(name.as_str()).map(|u| u.pop());
    }
    //    fn set_result_document(&mut self, rd: N) {
    //	self.rd = Some(rd);
    //    }
}

impl<N: Node> From<Sequence<N>> for Context<N> {
    fn from(seq: Sequence<N>) -> Self {
        Context {
            seq,
            i: 0,
            depth: 0,
            vars: HashMap::new(),
            templates: Vec::new(),
            current_templates: Vec::new(),
            rd: None,
        }
    }
}

/// Builder for a [Context]
pub struct ContextBuilder<N: Node>(Context<N>);

impl<N: Node> ContextBuilder<N> {
    pub fn new() -> Self {
        ContextBuilder(Context::new())
    }
    pub fn sequence(mut self, s: Sequence<N>) -> Self {
        self.0.seq = s;
        self
    }
    pub fn index(mut self, i: usize) -> Self {
        self.0.i = i;
        self
    }
    pub fn depth(mut self, d: usize) -> Self {
        self.0.depth = d;
        self
    }
    pub fn variables(mut self, v: HashMap<String, Vec<Sequence<N>>>) -> Self {
        self.0.vars = v;
        self
    }
    pub fn result_document(mut self, rd: N) -> Self {
        self.0.rd = Some(rd);
        self
    }
    pub fn template(mut self, t: Template<N>) -> Self {
        self.0.templates.push(Rc::new(t));
        self
    }
    pub fn current_templates(mut self, c: Vec<Rc<Template<N>>>) -> Self {
        self.0.current_templates = c;
        self
    }
    pub fn build(self) -> Context<N> {
        self.0
    }
}

impl<N: Node> From<Context<N>> for ContextBuilder<N> {
    fn from(c: Context<N>) -> Self {
        ContextBuilder(c.clone())
    }
}

/// An import tree

/// Creates an empty sequence
pub fn empty<N: Node>() -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>> {
    Box::new(move |_| Ok(Sequence::new()))
}

/// Creates a singleton sequence with the given value
pub fn literal<N: Node + 'static>(
    val: Rc<Item<N>>,
) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>> {
    Box::new(move |_| Ok(vec![val.clone()]))
}

/// Creates a singleton sequence with a new element node. The function is evaluated to create the content of the element.
pub fn literal_element<F, N: Node + 'static>(
    qn: QualifiedName,
    c: F,
) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static,
{
    Box::new(move |ctxt| {
        if ctxt.rd.is_none() {
            return Err(Error::new(
                ErrorKind::Unknown,
                String::from("context has no result document"),
            ));
        }
        let r = ctxt.rd.clone().unwrap();

        let mut e = r.new_element(qn.clone())?;
        c(ctxt)?.iter().try_for_each(|i| {
            // Item could be a Node or text
            match &**i {
                Item::Node(t) => match t.node_type() {
                    NodeType::Attribute => e.add_attribute(t.clone()),
                    _ => e.push(t.clone()),
                },
                _ => {
                    // Add the Value as a text node
                    let n = r.new_text(Value::from(i.to_string()))?;
                    e.push(n)
                }
            }
        })?;
        Ok(vec![Rc::new(Item::Node(e))])
    })
}

/// Creates a singleton sequence with a new attribute node. The function is evaluated to create the value of the attribute.
/// TODO: AVT for attribute name
pub fn literal_attribute<F, N: Node + 'static>(
    qn: QualifiedName,
    v: F,
) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static,
{
    Box::new(move |ctxt| {
        if ctxt.rd.is_none() {
            return Err(Error::new(
                ErrorKind::Unknown,
                String::from("context has no result document"),
            ));
        }
        let a = ctxt
            .rd
            .clone()
            .unwrap()
            .new_attribute(qn.clone(), Value::from(v(ctxt)?.to_string()))?;
        Ok(vec![Rc::new(Item::Node(a))])
    })
}

/// Creates a singleton sequence with the context item as its value
pub fn context<N: Node>() -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>> {
    Box::new(move |ctxt| Ok(vec![ctxt.seq[ctxt.i].clone()]))
}

/// Returns a sequence with the source document's root node as it's item
pub fn root<N: Node>() -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>> {
    Box::new(move |ctxt| {
        if ctxt.seq.len() != 0 {
            // TODO: check all of the context. If any item is not a Node then error
            match &*ctxt.seq[0] {
                Item::Node(n) => match n.node_type() {
                    NodeType::Document => Ok(vec![Rc::new(Item::Node(n.clone()))]),
                    _ => n
                        .ancestor_iter()
                        .last()
                        .map_or(Ok(vec![]), |m| Ok(vec![Rc::new(Item::Node(m))])),
                },
                _ => Err(Error::new(
                    ErrorKind::ContextNotNode,
                    String::from("context item is not a node"),
                )),
            }
        } else {
            Err(Error::new(
                ErrorKind::ContextNotNode,
                String::from("no context"),
            ))
        }
    })
}

/// Creates a sequence. Each function in the supplied vector creates an item in the sequence. The original context is passed to each function.
pub fn tc_sequence<F, N: Node>(items: Vec<F>) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static,
{
    Box::new(move |ctxt| {
        match items.iter().try_fold(vec![], |mut acc, f| match f(ctxt) {
            Ok(mut s) => {
                acc.append(&mut s);
                Ok(acc)
            }
            Err(err) => Err(err),
        }) {
            Ok(r) => Ok(r),
            Err(err) => Err(err),
        }
    })
}

/// Each function in the supplied vector is evaluated. The sequence returned by a function is used as the context for the next function.
pub fn compose<F, N: Node>(steps: Vec<F>) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static,
{
    Box::new(move |ctxt| {
        match steps
            .iter()
            .try_fold(ctxt.seq.clone(), |_, f| match f(ctxt) {
                Ok(s) => {
                    ctxt.seq = s.clone();
                    Ok(s)
                }
                Err(err) => Err(err),
            }) {
            Ok(r) => Ok(r),
            Err(err) => Err(err),
        }
    })
}

/// For each item in the current context, evaluate the given node matching operation.
pub fn step<N: Node>(nm: NodeMatch) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>> {
    Box::new(move |ctxt| {
        match ctxt.seq.iter().try_fold(vec![], |mut acc, i| {
            match &**i {
                Item::Node(n) => {
                    match nm.axis {
                        Axis::Selfaxis => {
                            if is_node_match::<N>(&nm.nodetest, n) {
                                acc.push(i.clone());
                                Ok(acc)
                            } else {
                                Ok(acc)
                            }
                        }
                        Axis::SelfDocument => {
                            if n.node_type() == NodeType::Document {
                                acc.push(i.clone());
                                Ok(acc)
                            } else {
                                Ok(acc)
                            }
                        }
                        Axis::Child => {
                            let mut s = n
                                .child_iter()
                                .filter(|c| is_node_match::<N>(&nm.nodetest, c))
                                .fold(Sequence::new(), |mut c, a| {
                                    c.push_node(a.clone());
                                    c
                                });
                            acc.append(&mut s);
                            Ok(acc)
                        }
                        Axis::Parent => match n.parent() {
                            Some(p) => {
                                acc.push_node(p.clone());
                                Ok(acc)
                            }
                            None => Ok(acc),
                        },
                        Axis::ParentDocument => {
                            // Only matches the Document.
                            // If no parent then return the Document
                            // NB. Document is a special kind of Node
                            match n.node_type() {
                                NodeType::Document => {
                                    // The context is the document
                                    acc.push(i.clone());
                                    Ok(acc)
                                }
                                _ => Ok(acc),
                            }
                        }
                        Axis::Descendant => {
                            n.descend_iter()
                                .filter(|c| is_node_match::<N>(&nm.nodetest, c))
                                .for_each(|c| acc.push_node(c.clone()));

                            Ok(acc)
                        }
                        Axis::DescendantOrSelf => {
                            if is_node_match::<N>(&nm.nodetest, n) {
                                acc.push(i.clone())
                            }
                            n.descend_iter()
                                .filter(|c| is_node_match::<N>(&nm.nodetest, c))
                                .for_each(|c| acc.push_node(c.clone()));
                            Ok(acc)
                        }
                        Axis::DescendantOrSelfOrRoot => {
                            acc.push_node(n.owner_document().clone());
                            if is_node_match::<N>(&nm.nodetest, n) {
                                acc.push(i.clone())
                            }
                            n.descend_iter()
                                .filter(|c| is_node_match::<N>(&nm.nodetest, c))
                                .for_each(|c| acc.push_node(c.clone()));
                            Ok(acc)
                        }
                        Axis::Ancestor => {
                            n.ancestor_iter()
                                .filter(|c| is_node_match::<N>(&nm.nodetest, c))
                                .for_each(|c| acc.push_node(c.clone()));

                            Ok(acc)
                        }
                        Axis::AncestorOrSelf => {
                            n.ancestor_iter()
                                .filter(|c| is_node_match::<N>(&nm.nodetest, c))
                                .for_each(|c| acc.push_node(c.clone()));
                            if is_node_match::<N>(&nm.nodetest, n) {
                                acc.push(i.clone())
                            }
                            Ok(acc)
                        }
                        Axis::FollowingSibling => {
                            n.next_iter()
                                .filter(|c| is_node_match::<N>(&nm.nodetest, c))
                                .for_each(|c| acc.push_node(c.clone()));

                            Ok(acc)
                        }
                        Axis::PrecedingSibling => {
                            n.prev_iter()
                                .filter(|c| is_node_match::<N>(&nm.nodetest, c))
                                .for_each(|c| acc.push_node(c.clone()));

                            Ok(acc)
                        }
                        Axis::Following => {
                            // XPath 3.3.2.1: the following axis contains all nodes that are descendants of the root of the tree in which the context node is found, are not descendants of the context node, and occur after the context node in document order.
                            // iow, for each ancestor-or-self node, include every next sibling and its descendants

                            let mut bcc = vec![];

                            // Start with following siblings of self
                            n.next_iter().for_each(|a| {
                                bcc.push(a.clone());
                                a.descend_iter().for_each(|b| bcc.push(b.clone()));
                            });

                            // Now traverse ancestors
                            n.ancestor_iter().for_each(|a| {
                                a.next_iter().for_each(|b| {
                                    bcc.push(b.clone());
                                    b.descend_iter().for_each(|c| bcc.push(c.clone()));
                                })
                            });
                            bcc.iter()
                                .filter(|e| is_node_match::<N>(&nm.nodetest, *e))
                                .for_each(|g| {
                                    acc.push_node(g.clone());
                                });
                            Ok(acc)
                        }
                        Axis::Preceding => {
                            // XPath 3.3.2.1: the preceding axis contains all nodes that are descendants of the root of the tree in which the context node is found, are not ancestors of the context node, and occur before the context node in document order.
                            // iow, for each ancestor-or-self node, include every previous sibling and its descendants

                            let mut bcc = vec![];

                            // Start with preceding siblings of self
                            n.prev_iter().for_each(|a| {
                                bcc.push(a.clone());
                                a.descend_iter().for_each(|b| bcc.push(b.clone()));
                            });

                            // Now traverse ancestors
                            n.ancestor_iter().for_each(|a| {
                                a.prev_iter().for_each(|b| {
                                    bcc.push(b.clone());
                                    b.descend_iter().for_each(|c| bcc.push(c.clone()));
                                })
                            });
                            bcc.iter()
                                .filter(|e| is_node_match::<N>(&nm.nodetest, *e))
                                .for_each(|g| {
                                    acc.push_node(g.clone());
                                });
                            Ok(acc)
                        }
                        Axis::Attribute => {
                            n.attribute_iter()
                                .filter(|a| is_node_match::<N>(&nm.nodetest, a))
                                .for_each(|a| acc.push_node(a.clone()));
                            Ok(acc)
                        }
                        Axis::SelfAttribute => {
                            if n.node_type() == NodeType::Attribute {
                                acc.push_node(n.clone())
                            }
                            Ok(acc)
                        }
                        _ => Err(Error::new(
                            ErrorKind::NotImplemented,
                            String::from("coming soon"),
                        )),
                    }
                }
                _ => Err(Error::new(
                    ErrorKind::Unknown,
                    String::from("context item is not a node"),
                )),
            }
        }) {
            Ok(mut r) => {
                // Eliminate duplicates
                r.dedup_by(|a, b| {
                    get_node(a).map_or(false, |aa| get_node(b).map_or(false, |bb| aa.is_same(bb)))
                });
                Ok(r)
            }
            Err(err) => Err(err),
        }
    })
}

fn get_node<N: Node>(i: &mut Rc<Item<N>>) -> Result<&N, Error> {
    match &**i {
        Item::Node(n) => Ok(n),
        _ => Err(Error::new(ErrorKind::Unknown, String::from("not a node"))),
    }
}

/// Iterate over the items in a sequence.
// TODO: Allow multiple variables
pub fn tc_loop<F, N: Node>(v: (String, F), b: F) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static,
{
    Box::new(move |ctxt| {
        let mut result = vec![];
        let s = v.1(ctxt)?;
        match s.iter().try_for_each(|i| {
            ctxt.var_push(v.0.clone(), vec![i.clone()]);
            let mut r = match b(ctxt) {
                Ok(t) => t,
                Err(err) => return Err(err),
            };
            ctxt.var_pop(v.0.clone());
            result.append(&mut r);
            Ok(())
        }) {
            Ok(()) => Ok(result),
            Err(err) => Err(err),
        }
    })
}

/// Choose a sequence to return.
pub fn switch<F, N: Node>(v: Vec<(F, F)>, o: F) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static,
{
    Box::new(move |ctxt| {
        let mut candidate = o(ctxt)?;
        for (t, w) in &v {
            let r = t(ctxt)?;
            if r.to_bool() {
                candidate = w(ctxt)?;
                break;
            }
        }
        Ok(candidate)
    })
}

/// Remove items that don't match the predicate.
pub fn filter<F, N: Node>(predicate: F) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static,
{
    Box::new(move |ctxt| {
        match ctxt.seq.iter().try_fold(vec![], |mut acc, i| {
            let s = match predicate(&mut Context::from(vec![i.clone()])) {
                Ok(t) => t,
                Err(err) => return Err(err),
            };
            if s.to_bool() == true {
                acc.push(i.clone())
            }
            Ok(acc)
        }) {
            Ok(r) => Ok(r),
            Err(err) => Err(err),
        }
    })
}

/// Return the disjunction of all of the given functions.
pub fn tc_or<F, N: Node>(v: Vec<F>) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static,
{
    Box::new(move |ctxt| {
        // Future: Evaluate every operand to check for dynamic errors
        let mut b = false;
        let mut i = 0;
        loop {
            match v.get(i) {
                Some(a) => {
                    if a(ctxt)?.to_bool() {
                        b = true;
                        break;
                    }
                    i += 1;
                }
                None => break,
            }
        }
        Ok(vec![Rc::new(Item::Value(Value::from(b)))])
    })
}

/// Return the conjunction of all of the given functions.
pub fn tc_and<F, N: Node>(v: Vec<F>) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static,
{
    Box::new(move |ctxt| {
        // Future: Evaluate every operand to check for dynamic errors
        let mut b = true;
        let mut i = 0;
        loop {
            match v.get(i) {
                Some(a) => {
                    if !a(ctxt)?.to_bool() {
                        b = false;
                        break;
                    }
                    i += 1;
                }
                None => break,
            }
        }
        Ok(vec![Rc::new(Item::Value(Value::from(b)))])
    })
}

/// General comparison of two sequences.
pub fn general_comparison<F, N: Node>(
    o: Operator,
    l: F,
    r: F,
) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static,
{
    Box::new(move |ctxt| {
        let left = l(ctxt)?;
        let right = r(ctxt)?;

        let mut b = false;
        for i in left {
            for j in &right {
                b = i.compare(&*j, o).unwrap();
                if b {
                    break;
                }
            }
            if b {
                break;
            }
        }

        Ok(vec![Rc::new(Item::Value(Value::from(b)))])
    })
}

/// Value comparison of two singelton sequences.
pub fn value_comparison<F, N: Node>(
    o: Operator,
    l: F,
    r: F,
) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static,
{
    Box::new(move |ctxt| {
        let left = l(ctxt)?;
        if left.len() != 1 {
            return Err(Error::new(
                ErrorKind::TypeError,
                String::from("left-hand sequence is not a singleton sequence"),
            ));
        }
        let right = r(ctxt)?;
        if right.len() != 1 {
            return Err(Error::new(
                ErrorKind::TypeError,
                String::from("right-hand sequence is not a singleton sequence"),
            ));
        }

        Ok(vec![Rc::new(Item::Value(Value::from(
            left[0].compare(&*right[0], o)?,
        )))])
    })
}

/// Generate a sequence with a range of integers.
pub fn tc_range<F, N: Node>(start: F, end: F) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static,
{
    Box::new(move |ctxt| {
        let s = start(ctxt)?;
        let e = end(ctxt)?;
        if s.len() == 0 || e.len() == 0 {
            // Empty sequence is the result
            return Ok(vec![]);
        }
        if s.len() != 1 || e.len() != 1 {
            return Err(Error::new(
                ErrorKind::TypeError,
                String::from("operands must be singleton sequence"),
            ));
        }
        let i = s[0].to_int()?;
        let j = e[0].to_int()?;
        if i > j {
            // empty sequence result
            Ok(vec![])
        } else if i == j {
            let mut seq = Sequence::new();
            seq.push_value(Value::Integer(i));
            Ok(seq)
        } else {
            let mut result = Sequence::new();
            for k in i..=j {
                result.push_value(Value::from(k))
            }
            Ok(result)
        }
    })
}

/// Perform an arithmetic operation.
pub fn arithmetic<F, N: Node>(
    ops: Vec<(ArithmeticOperator, F)>,
) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static,
{
    // Type: the result will be a number, but integer or double?
    // If all of the operands are integers, then the result is integer otherwise double
    // TODO: check the type of all operands to determine type of result (can probably do this in static analysis phase)
    // In the meantime, let's assume the result will be double and convert any integers
    Box::new(move |ctxt| {
        let mut acc = 0.0;
        for (op, i) in &ops {
            let j = match i(ctxt) {
                Ok(s) => s,
                Err(_) => {
                    acc = f64::NAN;
                    break;
                }
            };
            if j.len() != 1 {
                acc = f64::NAN;
                break;
            }
            let u = j[0].to_double();
            match op {
                ArithmeticOperator::Noop => acc = u,
                ArithmeticOperator::Add => acc += u,
                ArithmeticOperator::Subtract => acc -= u,
                ArithmeticOperator::Multiply => acc *= u,
                ArithmeticOperator::Divide => acc /= u,
                ArithmeticOperator::IntegerDivide => acc /= u, // TODO: convert to integer
                ArithmeticOperator::Modulo => acc = acc % u,
            }
        }
        Ok(vec![Rc::new(Item::Value(Value::from(acc)))])
    })
}

/// Declare a variable in scope for a function. Returns the result of the function.
pub fn declare_variable<F, N: Node>(
    name: String,
    value: F,
    f: F,
) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static,
{
    Box::new(move |ctxt| match value(ctxt) {
        Ok(s) => {
            ctxt.var_push(name.clone(), s);
            let r = f(ctxt);
            ctxt.var_pop(name.clone());
            r
        }
        Err(err) => Err(err),
    })
}
pub fn reference_variable<N: Node>(name: String) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>> {
    Box::new(move |ctxt| match ctxt.vars.get(name.as_str()) {
        Some(u) => match u.last() {
            Some(t) => Ok(t.clone()),
            None => Err(Error::new(
                ErrorKind::Unknown,
                format!("variable \"{}\" is no longer in scope", name),
            )),
        },
        None => Err(Error::new(
            ErrorKind::Unknown,
            format!("unknown variable \"{}\"", name),
        )),
    })
}

/// Apply templates to the select expression.
pub fn apply_templates<F, N: Node>(s: F) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static,
{
    Box::new(move |ctxt| {
        // s is the select expression. Evaluate it, and then iterate over it's items.
        // Each iteration becomes an item in the result sequence.
        s(ctxt)?.iter().try_fold(vec![], |mut result, i| {
            let templates = match_templates(ctxt, i)?;
            // If there are two or more templates with the same priority and import level, then take the one that has the higher document order
            let matching = if templates.len() > 1 {
                if templates[0].priority == templates[1].priority
                    && templates[0].import.len() == templates[1].import.len()
                {
                    let mut candidates: Vec<Rc<Template<N>>> = templates
                        .iter()
                        .take_while(|t| {
                            t.priority == templates[0].priority
                                && t.import.len() == templates[0].import.len()
                        })
                        .cloned()
                        .collect();
                    candidates.sort_unstable_by(|a, b| {
                        a.document_order.map_or(Ordering::Greater, |v| {
                            b.document_order.map_or(Ordering::Less, |u| v.cmp(&u))
                        })
                    });
                    candidates.last().unwrap().clone()
                } else {
                    templates[0].clone()
                }
            } else {
                templates[0].clone()
            };
            // Create a new context using the current templates, then evaluate the highest priority and highest import precedence
            let mut u = (matching.body)(
                &mut ContextBuilder::from(ctxt.clone())
                    .sequence(vec![i.clone()])
                    .current_templates(templates)
                    .build(),
            )?;
            result.append(&mut u);
            Ok(result)
        })
    })
}

/// Apply template with a higher import precedence.
pub fn apply_imports<F, N: Node>() -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static,
{
    Box::new(move |ctxt| {
        // Find the template with the next highest level within the same import tree
        // current_templates[0] is the currently matching template
        let cur = &(ctxt.current_templates[0]);
        let next: Vec<Rc<Template<N>>> = ctxt
            .current_templates
            .iter()
            .skip(1)
            .skip_while(|t| t.import.len() == cur.import.len()) // import level is the same (iow, different priority templates in the same import level)
            .cloned()
            .collect();

        if !next.is_empty() {
            (next[0].body)(
                &mut ContextBuilder::from(ctxt.clone())
                    .current_templates(next.clone())
                    .build(),
            )
        } else {
            Ok(vec![])
        }
    })
}

// Find all potential templates. Evaluate the match pattern against this item.
// Sort the result by priority and import precedence.
fn match_templates<N: Node>(
    ctxt: &mut Context<N>,
    i: &Rc<Item<N>>,
) -> Result<Vec<Rc<Template<N>>>, Error> {
    eprintln!(
        "match_templates: there are {} templates",
        ctxt.templates.len()
    );
    let mut candidates = ctxt.templates.iter().try_fold(vec![], |mut cand, t| {
        let e = (t.pattern)(&mut Context::from(vec![i.clone()]))?;
        if !e.is_empty() {
            cand.push(t.clone())
        }
        Ok(cand)
    })?;
    if candidates.len() != 0 {
        eprintln!("{} templates match:", candidates.len());
        candidates.iter().for_each(|t| eprintln!("{:?}", t));
        // Find the template(s) with the lowest priority.

        candidates.sort_unstable_by(|a, b| (*a).cmp(&*b));
        eprintln!("after sorting:");
        candidates.iter().for_each(|t| eprintln!("{:?}", t));
        Ok(candidates)
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            String::from("no matching template"),
        ))
    }
}

/// A template associates a pattern to a sequence constructor.
/// The import tree is represented by a vector of usize that is a signature for where the template was imported into the stylesheet. Templates from the primary stylesheet have an import precedence of 0.
/// Built-in templates have no priority and no document order and are considered to be in the primary stylesheet.
//#[derive(Clone)]
pub struct Template<N: Node> {
    pattern: Box<dyn Fn(&mut Context<N>) -> TransResult<N> + 'static>,
    body: Box<dyn Fn(&mut Context<N>) -> TransResult<N> + 'static>,
    priority: Option<f64>,
    import: Vec<usize>,
    document_order: Option<usize>,
    mode: Option<String>,
    phantom: PhantomData<N>,
}

impl<N: Node> Template<N> {
    pub fn new(
        pattern: Box<dyn Fn(&mut Context<N>) -> TransResult<N> + 'static>,
        body: Box<dyn Fn(&mut Context<N>) -> TransResult<N> + 'static>,
        priority: Option<f64>,
        import: Vec<usize>,
        document_order: Option<usize>,
        mode: Option<String>,
    ) -> Self {
        Template {
            pattern,
            body,
            priority,
            import,
            document_order,
            mode,
            phantom: PhantomData,
        }
    }
}

impl<N: Node> fmt::Debug for Template<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "match {{}} prio {:?}, import {:?}, doc order {:?}",
            //format_constructor(&self.pattern, 0),
            self.priority,
            self.import,
            self.document_order,
        )
    }
}

/// Templates are ordered in decreasing order of priority, and increasing order of import precedence.
/// The length of the import vector indicates the depth of a template in the import tree.
/// Absent values are always less.
impl<N: Node> PartialOrd for Template<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // An absent priority is always lower
        self.priority.map_or(Some(Ordering::Greater), |v| {
            other.priority.map_or(Some(Ordering::Greater), |u| {
                if v == u {
                    Some(self.import.len().cmp(&other.import.len()))
                } else if v < u {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Less)
                }
            })
        })
    }
}
impl<N: Node> Ord for Template<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).map_or(Ordering::Equal, |o| o)
    }
}

impl<N: Node> PartialEq for Template<N> {
    fn eq(&self, other: &Self) -> bool {
        self.priority.map_or_else(
            || other.priority.map_or(true, |_| false),
            |v| {
                other.priority.map_or(false, |u| {
                    if v == u {
                        self.import.len() == other.import.len()
                    } else {
                        false
                    }
                })
            },
        )
    }
}
impl<N: Node> Eq for Template<N> {}

/// XPath concat function. All arguments are concatenated into a single string value.
pub fn function_concat<F, N: Node>(
    arguments: Vec<F>,
) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static,
{
    Box::new(move |ctxt| {
        match arguments
            .iter()
            .try_fold(String::new(), |mut acc, a| match a(ctxt) {
                Ok(b) => {
                    acc.push_str(b.to_string().as_str());
                    Ok(acc)
                }
                Err(err) => Err(err),
            }) {
            Ok(r) => Ok(vec![Rc::new(Item::Value(Value::from(r)))]),
            Err(err) => Err(err),
        }
    })
}

/// A user defined function. Each argument is declared as a variable in the [Context]. The body of the function is then evaluated and it's result is returned.
pub fn function_user_defined<F, N: Node>(
    body: F,
    arguments: Vec<(String, F)>,
) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static,
{
    Box::new(move |ctxt| {
        arguments.iter().try_for_each(|(n, a)| match a(ctxt) {
            Ok(b) => {
                ctxt.var_push(n.clone(), b);
                Ok(())
            }
            Err(err) => Err(err),
        })?;
        let result = body(ctxt);
        arguments.iter().for_each(|(n, _)| ctxt.var_pop(n.clone()));
        result
    })
}
