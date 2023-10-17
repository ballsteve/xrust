//! # Transformation Combinator

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

use url::Url;

use crate::parsepicture::parse as picture_parse;
//use chrono::Utc;
#[allow(unused_imports)]
use chrono::{DateTime, Datelike, FixedOffset, Local, Timelike};
use unicode_segmentation::UnicodeSegmentation;

use crate::evaluate::{is_node_match, ArithmeticOperator, Axis, NodeMatch};
use crate::pattern::Pattern;
use crate::item::{Item, Node, NodeType, Sequence, SequenceTrait};
use crate::output::OutputDefinition;
use crate::qname::QualifiedName;
use crate::value::{Operator, Value};
use crate::xdmerror::*;

pub type TransResult<'a, N> = Result<Sequence<N>, Error>;
pub type Combinator<'a, N, F> = Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>;

/// The transformation context (i.e. the dynamic context, plus some parts of the static context)
// Idea: instead of having one dynamic context that is mutable,
// make the context immutable but with shared components. Then when a new context is required, clone it and add in extra components
//#[derive(Clone)]
pub struct Context<'a, N: Node + 'a, F>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    seq: Sequence<N>, // The current context
    i: usize,         // Which item in the sequence is the current context
    depth: usize,     // Depth of evaluation
    rd: Option<N>,    // Result document
    // No distinction between built-in and stylesheet-defined templates. Built-in templates have no priority and no document order.
    templates: Vec<Rc<Template<'a, N, F>>>,
    //builtin_templates: Vec<Rc<Template<N>>>,
    current_templates: Vec<Rc<Template<'a, N, F>>>,
    // variables
    vars: HashMap<String, Vec<Sequence<N>>>,
    // grouping
    current_grouping_key: Option<Value>,
    current_group: Sequence<N>,
    // output defn
    od: OutputDefinition,
    // base URI
    base_url: Option<Url>,
}

impl<'a, N: Node + 'a, F> Context<'a, N, F>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    pub fn new() -> Self {
        Context {
            seq: Sequence::new(),
            i: 0,
            depth: 0,
            vars: HashMap::new(),
            templates: Vec::new(),
            current_templates: Vec::new(),
            rd: None,
            current_grouping_key: None,
            current_group: Vec::new(),
            od: OutputDefinition::new(),
            base_url: None,
        }
    }
    pub fn make_copy(&self) -> Self {
        Context {
            seq: self.seq.clone(),
            i: self.i.clone(),
            depth: self.depth.clone(),
            vars: self.vars.clone(),
            templates: self.templates.clone(),
            current_templates: self.current_templates.clone(),
            rd: self.rd.clone(),
            current_grouping_key: self.current_grouping_key.clone(),
            current_group: self.current_group.clone(),
            od: self.od.clone(),
            base_url: self.base_url.clone(),
        }
    }

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

    fn baseurl(&self) -> Option<Url> {
        self.base_url.clone()
    }
    fn set_baseurl(&mut self, url: Url) {
        self.base_url = Some(url);
    }

    /// Evaluate finds a match for the sequence and evaluates the body of the template, returning the resulting [Sequence]
    fn evaluate(&mut self) -> Result<Sequence<N>, Error> {
        apply_templates(context())(self)
    }
}

impl<'a, N: Node + 'a, F> From<Sequence<N>> for Context<'a, N, F>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    fn from(seq: Sequence<N>) -> Self {
        Context {
            seq,
            i: 0,
            depth: 0,
            vars: HashMap::new(),
            templates: Vec::new(),
            current_templates: Vec::new(),
            current_grouping_key: None,
            current_group: Vec::new(),
            rd: None,
            od: OutputDefinition::new(),
            base_url: None,
        }
    }
}

/// Builder for a [Context]
pub struct ContextBuilder<'a, N: Node + 'a,
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>(Context<'a, N, F>);

impl<'a, N: Node + 'a, F> ContextBuilder<'a, N, F>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
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
    pub fn template(mut self, t: Template<'a, N, F>) -> Self {
        self.0.templates.push(Rc::new(t));
        self
    }
    pub fn current_templates(mut self, c: Vec<Rc<Template<'a, N, F>>>) -> Self {
        self.0.current_templates = c;
        self
    }
    pub fn current_group(mut self, c: Sequence<N>) -> Self {
        self.0.current_group = c;
        self
    }
    pub fn current_grouping_key(mut self, k: Value) -> Self {
        self.0.current_grouping_key = Some(k);
        self
    }
    pub fn output_definition(mut self, od: OutputDefinition) -> Self {
        self.0.od = od;
        self
    }
    pub fn base_url(mut self, b: Url) -> Self {
        self.0.base_url = Some(b);
        self
    }
    pub fn build(self) -> Context<'a, N, F> {
        self.0
    }
}

impl<'a, N: Node + 'a, F> From<Context<'a, N, F>> for ContextBuilder<'a, N, F>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    fn from(c: Context<'a, N, F>) -> Self {
        ContextBuilder(c)
    }
}
impl<'a, N: Node + 'a, F> From<&mut Context<'a, N, F>> for ContextBuilder<'a, N, F>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    fn from(c: &mut Context<'a, N, F>) -> Self {
        ContextBuilder(c.make_copy())
    }
}

/// An import tree

/// Creates an empty sequence
pub fn empty<'a, N: Node + 'a, F>() -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N>>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |_| Ok(Sequence::new()))
}

/// Creates a singleton sequence with the given value
pub fn literal<'a, N: Node + 'a, F>(
    val: Rc<Item<N>>,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |_| Ok(vec![val.clone()]))
}

/// Creates a singleton sequence with a new element node. The function is evaluated to create the content of the element.
pub fn literal_element<'a, F: 'a, N: Node>(
    qn: QualifiedName,
    c: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
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
pub fn literal_attribute<'a, F: 'a, N: Node>(
    qn: QualifiedName,
    v: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
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

/// Set an attribute on the context item, which must be an element-type node.
/// (TODO: use an expression to select the element)
/// If the element does not have an attribute with the given name, create it.
/// Otherwise replace the attribute's value with the supplied value
pub fn set_attribute<'a, F: 'a, N: Node>(
    atname: QualifiedName,
    v: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        if ctxt.rd.is_none() {
            return Err(Error::new(
                ErrorKind::Unknown,
                String::from("context has no result document"),
            ));
        }
        match &*ctxt.seq[ctxt.i] {
            Item::Node(n) => match n.node_type() {
                NodeType::Element => {
                    let od = n.owner_document();
                    let attval = v(ctxt)?;
                    if attval.len() == 1 {
                        match &*attval[0] {
                            Item::Value(av) => {
                                n.add_attribute(od.new_attribute(atname.clone(), av.clone())?)?;
                            }
                            _ => {
                                n.add_attribute(od.new_attribute(
                                    atname.clone(),
                                    Value::from(attval.to_string()),
                                )?)?;
                            }
                        }
                    } else {
                        n.add_attribute(
                            od.new_attribute(atname.clone(), Value::from(attval.to_string()))?,
                        )?;
                    }
                }
                _ => {
                    return Err(Error::new(
                        ErrorKind::Unknown,
                        String::from("context item is not an element-type node"),
                    ))
                }
            },
            _ => {
                return Err(Error::new(
                    ErrorKind::Unknown,
                    String::from("context item is not a node"),
                ))
            }
        }
        Ok(vec![])
    })
}

/// Shallow copy of an item. The first argument selects the items to be copied. If not specified then the context item is copied. The content of the item can, optionally, be added.
pub fn copy<'a, F: 'a, N: Node>(
    i: Option<F>,
    c: Option<F>,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        // If item (i) is None then copy the context item
        let orig = if i.is_some() {
            (i.as_ref().unwrap())(ctxt)?
        } else {
            vec![ctxt.seq[ctxt.i].clone()]
        };
        let mut result: Sequence<N> = Vec::new();
        for k in orig {
            let cp = k.shallow_copy()?;
            result.push(Rc::new(cp.clone()));
            if c.is_some() {
                match cp {
                    Item::Node(mut im) => {
                        for j in (c.as_ref().unwrap())(ctxt)? {
                            match &*j {
                                Item::Value(v) => im.push(im.new_text(v.clone())?)?,
                                Item::Node(n) => im.push(n.clone())?,
                                _ => {
                                    return Err(Error::new(
                                        ErrorKind::NotImplemented,
                                        String::from("not yet implemented"),
                                    ))
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(result)
    })
}

/// Deep copy of an item. The first argument selects the items to be copied. If not specified then the context item is copied.
pub fn deep_copy<'a, F: 'a, N: Node>(
    i: Option<F>,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        // If item (i) is None then copy the context item
        let orig = if i.is_some() {
            (i.as_ref().unwrap())(ctxt)?
        } else {
            vec![ctxt.seq[ctxt.i].clone()]
        };
        let mut result: Sequence<N> = Vec::new();
        for k in orig {
            result.push(Rc::new(k.deep_copy()?));
        }
        Ok(result)
    })
}

/// Creates a singleton sequence with the context item as its value
pub fn context<'a, N: Node + 'a, F>() -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| Ok(vec![ctxt.seq[ctxt.i].clone()]))
}

/// Returns a sequence with the source document's root node as it's item
pub fn root<'a, N: Node + 'a, F>() -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
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

/// Returns a sequence with the parent node only if it is the source document's root node
pub fn parent_root<'a, N: Node + 'a, F>() -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        if ctxt.seq.len() != 0 {
            match &*ctxt.seq[0] {
                Item::Node(n) => match n.parent() {
                    Some(_) => match n.node_type() {
                        NodeType::Document => Ok(vec![Rc::new(Item::Node(n.clone()))]),
                        _ => Ok(vec![]),
                    },
                    None => Ok(vec![]),
                },
                _ => Ok(vec![]), // could be an error
            }
        } else {
            Ok(vec![])
        }
    })
}

/// Creates a sequence. Each function in the supplied vector creates an item in the sequence. The original context is passed to each function.
pub fn tc_sequence<'a, F: 'a, N: Node + 'a>(
    items: Vec<F>,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
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
pub fn compose<'a, F: 'a, N: Node>(
    steps: Vec<F>,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
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

/// Each function in the supplied vector is evaluated, and the resulting sequences are combined into a single sequence.
/// TODO: eliminate duplicates
pub fn union<'a, F: 'a, N: Node>(
    branches: Vec<F>,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        let mut result = vec![];
        for b in &branches {
            let mut c = b(ctxt)?;
            result.append(&mut c)
        }
        Ok(result)
    })
}

/// For each item in the current context, evaluate the given node matching operation.
pub fn step<'a, N: Node + 'a, F>(
    nm: NodeMatch,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
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
pub fn tc_loop<'a, F: 'a, N: Node>(
    v: (String, F),
    b: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
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
pub fn switch<'a, F: 'a, N: Node>(
    v: Vec<(F, F)>,
    o: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
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
pub fn filter<'a, F: 'a, N: Node>(
    predicate: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
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
pub fn tc_or<'a, F: 'a, N: Node>(
    v: Vec<F>,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
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
pub fn tc_and<'a, F: 'a, N: Node>(
    v: Vec<F>,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
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
pub fn general_comparison<'a, F: 'a, N: Node>(
    o: Operator,
    l: F,
    r: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
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
pub fn value_comparison<'a, F: 'a, N: Node>(
    o: Operator,
    l: F,
    r: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
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
pub fn tc_range<'a, F: 'a, N: Node>(
    start: F,
    end: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
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
pub fn arithmetic<'a, F: 'a, N: Node>(
    ops: Vec<(ArithmeticOperator, F)>,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
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
pub fn declare_variable<'a, F: 'a, N: Node>(
    name: String,
    value: F,
    f: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
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
pub fn reference_variable<'a, N: Node + 'a, F>(
    name: String,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
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

/// Evaluate a combinator for each item.
pub fn for_each<'a, F: 'a, N: Node>(
    s: F,
    body: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        let mut result: Sequence<N> = Vec::new();

        for i in s(ctxt)? {
            let mut v = body(&mut ContextBuilder::from(ctxt).sequence(vec![i]).build())?;
            result.append(&mut v);
        }
        Ok(result)
    })
}

/// Evaluate a combinator for each group of items.
pub fn group_by<'a, F: 'a, N: Node>(
    s: F,
    by: F,
    body: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        // Each 'by' expression is evaluated to a string key and stored in the hashmap
        let mut groups = HashMap::new();
        s(ctxt)?.iter().try_for_each(|i| {
            // There may be multiple keys returned.
            // For each one, add this item into the group for that key
            by(&mut ContextBuilder::from(ctxt)
                .sequence(vec![i.clone()])
                .build())?
            .iter()
            .for_each(|k| {
                let e: &mut Vec<Rc<Item<N>>> = groups.entry(k.to_string()).or_default();
                e.push(i.clone());
            });
            Ok(())
        })?;

        // Now evaluate the body for each group
        groups.iter().try_fold(vec![], |mut result, (k, v)| {
            // Set current-group and current-grouping-key
            let mut r = body(
                &mut ContextBuilder::from(ctxt)
                    .current_grouping_key(Value::from(k.clone()))
                    .current_group(v.clone())
                    .build(),
            )?;
            result.append(&mut r);
            Ok(result)
        })
    })
}

/// Evaluate a combinator for each group of items. 'adj' is an expression that is evaluated for each selected item. It must resolve to a singleton item. The first item starts the first group. For the second and subsequent items, if the 'adj' item is the same as the previous item then the item is added to the same group. Otherwise a new group is started.
pub fn group_adjacent<'a, F: 'a, N: Node>(
    s: F,
    adj: F,
    body: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        let mut groups = Vec::new();
        let sel = s(ctxt)?;
        if sel.is_empty() {
            return Ok(vec![]);
        } else {
            let mut curgrp = vec![sel[0].clone()];
            let mut curkey = adj(&mut ContextBuilder::from(ctxt)
                .sequence(vec![sel[1].clone()])
                .build())?;
            if curkey.len() != 1 {
                return Err(Error::new(
                    ErrorKind::Unknown,
                    String::from("group-adjacent attribute must evaluate to a single item"),
                ));
            }
            sel.iter().skip(1).try_for_each(|i| {
                let thiskey = adj(&mut ContextBuilder::from(ctxt)
                    .sequence(vec![i.clone()])
                    .build())?;
                if thiskey.len() == 1 {
                    if curkey[0].compare(&*thiskey[0], Operator::Equal)? {
                        // Append to the current group
                        curgrp.push(i.clone())
                    } else {
                        // Close the previous group, start a new group with this item as its first member
                        groups.push((curkey.to_string(), curgrp.clone()));
                        curgrp = vec![i.clone()];
                        curkey = thiskey;
                    }
                    Ok(())
                } else {
                    Err(Error::new(
                        ErrorKind::Unknown,
                        String::from("group-adjacent attribute must evaluate to a single item"),
                    ))
                }
            })?;
            // Close the last group
            groups.push((curkey.to_string(), curgrp))
        }

        // Now evaluate the body for each group
        groups.iter().try_fold(vec![], |mut result, (k, v)| {
            // Set current-group and current-grouping-key
            let mut r = body(
                &mut ContextBuilder::from(ctxt)
                    .current_grouping_key(Value::from(k.clone()))
                    .current_group(v.clone())
                    .build(),
            )?;
            result.append(&mut r);
            Ok(result)
        })
    })
}

/// Evaluate a combinator for each group of items.
pub fn group_starting_with<'a, F: 'a, N: Node>(
    _s: F,
    _body: F,
    _pat: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |_ctxt| {
        Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not implemented"),
        ))
    })
}

/// Evaluate a combinator for each group of items.
pub fn group_ending_with<'a, F: 'a, N: Node>(
    _s: F,
    _body: F,
    _pat: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |_ctxt| {
        Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not implemented"),
        ))
    })
}

/// Apply templates to the select expression.
pub fn apply_templates<'a, F: 'a, N: Node>(
    s: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
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
                    let mut candidates: Vec<Rc<Template<N, F>>> = templates
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
                &mut ContextBuilder::from(ctxt)
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
pub fn apply_imports<'a, F: 'a, N: Node>(
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        // Find the template with the next highest level within the same import tree
        // current_templates[0] is the currently matching template
        let cur = &(ctxt.current_templates[0]);
        let next: Vec<Rc<Template<N, F>>> = ctxt
            .current_templates
            .iter()
            .skip(1)
            .skip_while(|t| t.import.len() == cur.import.len()) // import level is the same (iow, different priority templates in the same import level)
            .cloned()
            .collect();

        if !next.is_empty() {
            (next[0].body)(
                &mut ContextBuilder::from(ctxt)
                    .current_templates(next.clone())
                    .build(),
            )
        } else {
            Ok(vec![])
        }
    })
}

/// Apply the next template that matches.
pub fn next_match<'a, N: Node + 'a, F>() -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        if ctxt.current_templates.len() > 2 {
            (ctxt.current_templates[1].body)(
                &mut ContextBuilder::from(ctxt)
                    .current_templates(ctxt.current_templates.iter().skip(1).cloned().collect())
                    .build(),
            )
        } else {
            Ok(vec![])
        }
    })
}

// Find all potential templates. Evaluate the match pattern against this item.
// Sort the result by priority and import precedence.
fn match_templates<'a, N: Node + 'a, F>(
    ctxt: &mut Context<'a, N, F>,
    i: &Rc<Item<N>>,
) -> Result<Vec<Rc<Template<'a, N, F>>>, Error>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    let mut candidates = ctxt.templates.iter().try_fold(vec![], |mut cand, t| {
        let e = t.pattern.matches(i.clone());
        if e {
            cand.push(t.clone())
        }
        Ok(cand)
    })?;
    if candidates.len() != 0 {
        // Find the template(s) with the lowest priority.

        candidates.sort_unstable_by(|a, b| (*a).cmp(&*b));
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
pub struct Template<'a, N: Node + 'a, F>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    //    pattern: Box<dyn Fn(&mut Context<'a, N>) -> TransResult<'a, N>>,
    //    body: Box<dyn Fn(&mut Context<'a, N>) -> TransResult<'a, N>>,
    pattern: Pattern<'a, N, F>,
    body: Combinator<'a, N, F>,
    priority: Option<f64>,
    import: Vec<usize>,
    document_order: Option<usize>,
    mode: Option<String>,
    phantom: PhantomData<N>,
}

impl<'a, N: Node, F> Template<'a, N, F>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    pub fn new(
        //        pattern: Box<dyn Fn(&mut Context<'a, N>) -> TransResult<'a, N>>,
        //        body: Box<dyn Fn(&mut Context<'a, N>) -> TransResult<'a, N>>,
        pattern: Pattern<'a, N, F>,
        body: Combinator<'a, N, F>,
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

impl<'a, N: Node + 'a, F> fmt::Debug for Template<'a, N, F>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
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
impl<'a, N: Node + 'a, F> PartialOrd for Template<'a, N, F>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
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
impl<'a, N: Node + 'a, F> Ord for Template<'a, N, F>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).map_or(Ordering::Equal, |o| o)
    }
}

impl<'a, N: Node + 'a, F> PartialEq for Template<'a, N, F>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
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
impl<'a, N: Node + 'a, F> Eq for Template<'a, N, F>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{}

/// Currently, these are the functions defined for XPath 1.0:
///
/// * position()
/// * last()
/// * count()
/// * local-name()
/// * name()
/// * string()
/// * concat()
/// * starts-with()
/// * contains()
/// * substring()
/// * substring-before()
/// * substring-after()
/// * normalize-space()
/// * translate()
/// * boolean()
/// * not()
/// * true()
/// * false()
/// * number()
/// * sum()
/// * floor()
/// * ceiling()
/// * round()
///
/// These functions are defined for XPath 2.0:
///
/// * current-dateTime()
/// * current-date()
/// * current-time()
/// * format-dateTime()
/// * format-date()
/// * format-time()
/// * current-group()
/// * current-grouping-key()

/// XPath position function.
pub fn position<'a, N: Node + 'a, F>() -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| Ok(vec![Rc::new(Item::Value(Value::from(ctxt.i as i64 + 1)))]))
}

/// XPath last function.
pub fn last<'a, N: Node + 'a, F>() -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        Ok(vec![Rc::new(Item::Value(Value::from(
            ctxt.seq.len() as i64
        )))])
    })
}

/// XPath count function.
pub fn tc_count<'a, F: 'a, N: Node>(
    s: Option<F>,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        s.as_ref().map_or_else(
            || {
                Ok(vec![Rc::new(Item::Value(Value::from(
                    ctxt.seq.len() as i64
                )))])
            },
            |i| {
                Ok(vec![Rc::new(Item::Value(Value::from(
                    i(ctxt)?.len() as i64,
                )))])
            },
        )
    })
}

/// XPath local-name function.
pub fn local_name<'a, F: 'a, N: Node>(
    s: Option<F>,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        s.as_ref().map_or_else(
            || {
                // Get the name of the context item
                match *ctxt.seq[ctxt.i] {
                    Item::Node(ref m) => Ok(vec![Rc::new(Item::Value(Value::from(
                        m.name().get_localname(),
                    )))]),
                    _ => Err(Error::new(
                        ErrorKind::TypeError,
                        String::from("type error: not a node"),
                    )),
                }
            },
            |t| {
                // Get the name of the singleton node
                let n = t(ctxt)?;
                match n.len() {
                    0 => Ok(vec![Rc::new(Item::Value(Value::from("")))]),
                    1 => match *n[0] {
                        Item::Node(ref m) => Ok(vec![Rc::new(Item::Value(Value::from(
                            m.name().get_localname(),
                        )))]),
                        _ => Err(Error::new(
                            ErrorKind::TypeError,
                            String::from("type error: not a node"),
                        )),
                    },
                    _ => Err(Error::new(
                        ErrorKind::TypeError,
                        String::from("type error: not a singleton node"),
                    )),
                }
            },
        )
    })
}

/// XPath name function.
pub fn name<'a, F: 'a, N: Node>(
    s: Option<F>,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        s.as_ref().map_or_else(
            || {
                // Get the name of the context item
                match *ctxt.seq[ctxt.i] {
                    Item::Node(ref m) => Ok(vec![Rc::new(Item::Value(Value::from(
                        m.name().to_string(),
                    )))]),
                    _ => Err(Error::new(
                        ErrorKind::TypeError,
                        String::from("type error: not a node"),
                    )),
                }
            },
            |t| {
                // Get the name of the singleton node
                let n = t(ctxt)?;
                match n.len() {
                    0 => Ok(vec![Rc::new(Item::Value(Value::from("")))]),
                    1 => match *n[0] {
                        Item::Node(ref m) => Ok(vec![Rc::new(Item::Value(Value::from(
                            m.name().to_string(),
                        )))]),
                        _ => Err(Error::new(
                            ErrorKind::TypeError,
                            String::from("type error: not a node"),
                        )),
                    },
                    _ => Err(Error::new(
                        ErrorKind::TypeError,
                        String::from("type error: not a singleton node"),
                    )),
                }
            },
        )
    })
}

/// XPath string function.
pub fn string<'a, F: 'a, N: Node>(
    s: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        Ok(vec![Rc::new(Item::Value(Value::from(
            s(ctxt)?.to_string(),
        )))])
    })
}

/// XPath concat function. All arguments are concatenated into a single string value.
pub fn tc_concat<'a, F: 'a, N: Node>(
    arguments: Vec<F>,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
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

/// XPath starts-with function.
pub fn starts_with<'a, F: 'a, N: Node>(
    s: F,
    t: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        // s is the string to search, t is what to search for
        Ok(vec![Rc::new(Item::Value(Value::from(
            s(ctxt)?
                .to_string()
                .starts_with(t(ctxt)?.to_string().as_str()),
        )))])
    })
}

/// XPath contains function.
pub fn contains<'a, F: 'a, N: Node>(
    s: F,
    t: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        // s is the string to search, t is what to search for
        Ok(vec![Rc::new(Item::Value(Value::from(
            s(ctxt)?.to_string().contains(t(ctxt)?.to_string().as_str()),
        )))])
    })
}

/// XPath substring function.
pub fn substring<'a, F: 'a, N: Node>(
    s: F,
    t: F,
    l: Option<F>,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        // must have two or three arguments.
        // s is the string to search,
        // t is the index to start at,
        // l is the length of the substring at extract (or the rest of the string if missing)
        l.as_ref().map_or_else(
            || {
                Ok(vec![Rc::new(Item::Value(Value::from(
                    s(ctxt)?
                        .to_string()
                        .graphemes(true)
                        .skip(t(ctxt)?.to_int()? as usize - 1)
                        .collect::<String>(),
                )))])
            },
            |m| {
                Ok(vec![Rc::new(Item::Value(Value::from(
                    s(ctxt)?
                        .to_string()
                        .graphemes(true)
                        .skip(t(ctxt)?.to_int()? as usize - 1)
                        .take(m(ctxt)?.to_int()? as usize)
                        .collect::<String>(),
                )))])
            },
        )
    })
}

/// XPath substring-before function.
pub fn substring_before<'a, F: 'a, N: Node>(
    s: F,
    t: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        // s is the string to search,
        // t is the string to find.
        let u = s(ctxt)?.to_string();
        match u.find(t(ctxt)?.to_string().as_str()) {
            Some(i) => {
                match u.get(0..i) {
                    Some(v) => Ok(vec![Rc::new(Item::Value(Value::from(v)))]),
                    None => {
                        // This shouldn't happen!
                        Err(Error::new(
                            ErrorKind::Unknown,
                            String::from("unable to extract substring"),
                        ))
                    }
                }
            }
            None => Ok(vec![]),
        }
    })
}

/// XPath substring-after function.
pub fn substring_after<'a, F: 'a, N: Node>(
    s: F,
    t: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        // s is the string to search,
        // t is the string to find.
        let u = s(ctxt)?.to_string();
        let v = t(ctxt)?.to_string();
        match u.find(v.as_str()) {
            Some(i) => {
                match u.get(i + v.len()..u.len()) {
                    Some(w) => Ok(vec![Rc::new(Item::Value(Value::from(w)))]),
                    None => {
                        // This shouldn't happen!
                        Err(Error::new(
                            ErrorKind::Unknown,
                            String::from("unable to extract substring"),
                        ))
                    }
                }
            }
            None => Ok(vec![]),
        }
    })
}

/// XPath normalize-space function.
pub fn normalize_space<'a, F: 'a, N: Node>(
    n: Option<F>,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        let s: Result<String, Error> = n.as_ref().map_or_else(
            || {
                // Use the current item
                Ok(ctxt.seq[ctxt.i].to_string())
            },
            |m| {
                let t = m(ctxt)?;
                Ok(t.to_string())
            },
        );
        // intersperse is the right iterator to use, but it is only available in nightly at the moment
        s.map(|u| {
            vec![Rc::new(Item::Value(Value::from(
                u.split_whitespace().collect::<Vec<&str>>().join(" "),
            )))]
        })
    })
}

/// XPath translate function.
pub fn translate<'a, F: 'a, N: Node>(
    s: F,
    map: F,
    trn: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        // s is the string to search
        // map are the map chars
        // trn are the translate chars
        let o = map(ctxt)?.to_string();
        let m: Vec<&str> = o.graphemes(true).collect();
        let u = trn(ctxt)?.to_string();
        let t: Vec<&str> = u.graphemes(true).collect();
        let mut result: String = String::new();

        for c in s(ctxt)?.to_string().graphemes(true) {
            let mut a: Option<Option<usize>> = Some(None);
            for (i, _item) in m.iter().enumerate() {
                if c == m[i] {
                    if i < t.len() {
                        a = Some(Some(i));
                        break;
                    } else {
                        // omit this character
                        a = None
                    }
                } else {
                    // keep looking for a match
                }
            }
            match a {
                Some(None) => {
                    result.push_str(c);
                }
                Some(Some(j)) => result.push_str(t[j]),
                None => {
                    // omit char
                }
            }
        }
        Ok(vec![Rc::new(Item::Value(Value::from(result)))])
    })
}

/// XPath boolean function.
pub fn boolean<'a, F: 'a, N: Node>(
    b: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        Ok(vec![Rc::new(Item::Value(Value::Boolean(
            b(ctxt)?.to_bool(),
        )))])
    })
}

/// XPath not function.
pub fn not<'a, F: 'a, N: Node>(n: F) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        Ok(vec![Rc::new(Item::Value(Value::Boolean(
            !n(ctxt)?.to_bool(),
        )))])
    })
}

/// XPath true function.
pub fn tc_true<'a, N: Node + 'a, F>() -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |_| Ok(vec![Rc::new(Item::Value(Value::Boolean(true)))]))
}

/// XPath false function.
pub fn tc_false<'a, N: Node + 'a, F>() -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |_| Ok(vec![Rc::new(Item::Value(Value::Boolean(false)))]))
}

/// XPath number function.
pub fn number<'a, F: 'a, N: Node>(
    num: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        let n = num(ctxt)?;
        match n.len() {
            1 => {
                // First try converting to an integer
                match n[0].to_int() {
                    Ok(i) => Ok(vec![Rc::new(Item::Value(Value::Integer(i)))]),
                    _ => {
                        // Otherwise convert to double.
                        // NB. This can't fail. At worst it returns NaN.
                        Ok(vec![Rc::new(Item::Value(Value::Double(n[0].to_double())))])
                    }
                }
            }
            _ => Err(Error::new(
                ErrorKind::TypeError,
                String::from("not a singleton sequence"),
            )),
        }
    })
}

/// XPath sum function.
pub fn sum<'a, F: 'a, N: Node>(s: F) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        Ok(vec![Rc::new(Item::Value(Value::Double(
            s(ctxt)?.iter().fold(0.0, |mut acc, i| {
                acc += i.to_double();
                acc
            }),
        )))])
    })
}

/// XPath floor function.
pub fn floor<'a, F: 'a, N: Node>(
    f: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        let n = f(ctxt)?;
        match n.len() {
            1 => Ok(vec![Rc::new(Item::Value(Value::Double(
                n[0].to_double().floor(),
            )))]),
            _ => Err(Error::new(
                ErrorKind::TypeError,
                String::from("not a singleton sequence"),
            )),
        }
    })
}

/// XPath ceiling function.
pub fn ceiling<'a, F: 'a, N: Node>(
    c: F,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        let n = c(ctxt)?;
        match n.len() {
            1 => Ok(vec![Rc::new(Item::Value(Value::Double(
                n[0].to_double().ceil(),
            )))]),
            _ => Err(Error::new(
                ErrorKind::TypeError,
                String::from("not a singleton sequence"),
            )),
        }
    })
}

/// XPath round function.
pub fn round<'a, F: 'a, N: Node>(
    r: F,
    pr: Option<F>,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        pr.as_ref().map_or_else(
            || {
                // precision is 0, i.e. round to nearest whole number
                let n = r(ctxt)?;
                match n.len() {
                    1 => Ok(vec![Rc::new(Item::Value(Value::Double(
                        n[0].to_double().round(),
                    )))]),
                    _ => Err(Error::new(
                        ErrorKind::TypeError,
                        String::from("not a singleton sequence"),
                    )),
                }
            },
            |p| {
                let n = r(ctxt)?;
                let m = p(ctxt)?;
                match (n.len(), m.len()) {
                    (1, 1) => Ok(vec![Rc::new(Item::Value(Value::Double(
                        ((n[0].to_double() * (10.0_f64).powi(m[0].to_int().unwrap() as i32))
                            .round())
                            * (10.0_f64).powi(-m[0].to_int().unwrap() as i32),
                    )))]),
                    _ => Err(Error::new(
                        ErrorKind::TypeError,
                        String::from("not a singleton sequence"),
                    )),
                }
            },
        )
    })
}

/// XPath current-date-time function.
pub fn current_date_time<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |_| Ok(vec![Rc::new(Item::Value(Value::DateTime(Local::now())))]))
}

/// XPath current-date function.
pub fn current_date<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |_| {
        Ok(vec![Rc::new(Item::Value(Value::Date(
            Local::now().date_naive(),
        )))])
    })
}

/// XPath current-time function.
pub fn current_time<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |_| Ok(vec![Rc::new(Item::Value(Value::Time(Local::now())))]))
}

/// XPath format-date-time function.
/// NB. language, calendar, and place are not implemented.
pub fn format_date_time<'a, F: 'a, N: Node>(
    value: F,
    picture: F,
    _language: Option<F>,
    _calendar: Option<F>,
    _place: Option<F>,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        let dt = value(ctxt)?;
        let pic = picture_parse(&picture(ctxt)?.to_string())?;
        match dt.len() {
            0 => Ok(vec![]), // Empty value returns empty sequence
            1 => {
                match *dt[0] {
                    Item::Value(Value::DateTime(i)) => Ok(vec![Rc::new(Item::Value(
                        Value::String(i.format(&pic).to_string()),
                    ))]),
                    Item::Value(Value::String(ref s)) => {
                        // Try and coerce into a DateTime value
                        match DateTime::<FixedOffset>::parse_from_rfc3339(s.as_str()) {
                            Ok(j) => Ok(vec![Rc::new(Item::Value(Value::String(
                                j.format(&pic).to_string(),
                            )))]),
                            _ => Err(Error::new(
                                ErrorKind::TypeError,
                                String::from("unable to determine date value"),
                            )),
                        }
                    }
                    _ => Err(Error::new(
                        ErrorKind::TypeError,
                        String::from("not a dateTime value"),
                    )),
                }
            }
            _ => Err(Error::new(
                ErrorKind::TypeError,
                String::from("not a singleton sequence"),
            )),
        }
    })
}

/// XPath format-date function.
/// NB. language, calendar, and place are not implemented.
pub fn format_date<'a, F: 'a, N: Node>(
    value: F,
    picture: F,
    _language: Option<F>,
    _calendar: Option<F>,
    _place: Option<F>,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        let dt = value(ctxt)?;
        let pic = picture_parse(&picture(ctxt)?.to_string())?;
        match dt.len() {
            0 => Ok(vec![]), // Empty value returns empty sequence
            1 => {
                match *dt[0] {
                    Item::Value(Value::Date(i)) => Ok(vec![Rc::new(Item::Value(Value::String(
                        i.format(&pic).to_string(),
                    )))]),
                    Item::Value(Value::String(ref s)) => {
                        // Try and coerce into a DateTime value
                        let a = format!("{}T00:00:00Z", s);
                        match DateTime::<FixedOffset>::parse_from_rfc3339(a.as_str()) {
                            Ok(j) => Ok(vec![Rc::new(Item::Value(Value::String(
                                j.date_naive().format(&pic).to_string(),
                            )))]),
                            _ => Err(Error::new(
                                ErrorKind::TypeError,
                                String::from("unable to determine date value"),
                            )),
                        }
                    }
                    _ => Err(Error::new(
                        ErrorKind::TypeError,
                        String::from("not a date value"),
                    )),
                }
            }
            _ => Err(Error::new(
                ErrorKind::TypeError,
                String::from("not a singleton sequence"),
            )),
        }
    })
}

/// XPath format-time function.
/// NB. language, calendar, and place are not implemented.
pub fn format_time<'a, F: 'a, N: Node>(
    value: F,
    picture: F,
    _language: Option<F>,
    _calendar: Option<F>,
    _place: Option<F>,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        let dt = value(ctxt)?;
        let pic = picture_parse(&picture(ctxt)?.to_string())?;
        match dt.len() {
            0 => Ok(vec![]), // Empty value returns empty sequence
            1 => {
                match *dt[0] {
                    Item::Value(Value::Time(i)) => Ok(vec![Rc::new(Item::Value(Value::String(
                        i.format(&pic).to_string(),
                    )))]),
                    Item::Value(Value::String(ref s)) => {
                        // Try and coerce into a DateTime value
                        let a = format!("1900-01-01T{}Z", s);
                        match DateTime::<FixedOffset>::parse_from_rfc3339(a.as_str()) {
                            Ok(j) => Ok(vec![Rc::new(Item::Value(Value::String(
                                j.format(&pic).to_string(),
                            )))]),
                            _ => Err(Error::new(
                                ErrorKind::TypeError,
                                String::from("unable to determine time value"),
                            )),
                        }
                    }
                    _ => Err(Error::new(
                        ErrorKind::TypeError,
                        String::from("not a time value"),
                    )),
                }
            }
            _ => Err(Error::new(
                ErrorKind::TypeError,
                String::from("not a singleton sequence"),
            )),
        }
    })
}

/// XSLT current-group function.
pub fn current_group<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| Ok(ctxt.current_group.clone()))
}

/// XSLT current-grouping-key function.
pub fn current_grouping_key<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |ctxt| {
        ctxt.current_grouping_key.clone().map_or_else(
            || {
                Err(Error::new(
                    ErrorKind::TypeError,
                    String::from("no current grouping key"),
                ))
            },
            |k| Ok(vec![Rc::new(Item::Value(k))]),
        )
    })
}

/// A user defined function. Each argument is declared as a variable in the [Context]. The body of the function is then evaluated and it's result is returned.
pub fn function_user_defined<'a, F: 'a, N: Node + 'a>(
    body: F,
    arguments: Vec<(String, F)>,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
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

/// Not implemented error.
pub fn not_implemented<'a, N: Node + 'a, F>(
    msg: String,
) -> Box<dyn Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |_ctxt| {
        Err(Error::new(
            ErrorKind::NotImplemented,
            format!("not implemented: {}", msg),
        ))
    })
}
