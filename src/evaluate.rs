//! # Evaluate a sequence constructor
//!
//! Evaluate a sequence constructor to produce a sequence.
//!
//! This library uses the traits defined in [Item], so it is independent of the tree implementation.

use crate::item::{Item, Node, NodeType, Sequence, SequenceTrait};
use crate::output::OutputDefinition;
use crate::parsepicture::parse as picture_parse;
use crate::qname::*;
use crate::value::{Operator, Value};
use crate::xdmerror::*;
use chrono::Utc;
#[allow(unused_imports)]
use chrono::{DateTime, Datelike, FixedOffset, Local, Timelike};
use std::cell::{RefCell, RefMut};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::Formatter;
use std::rc::Rc;
use unicode_segmentation::UnicodeSegmentation;
use url::Url;

// The dynamic evaluation context.
//
// The dynamic context stores parts that can change as evaluation proceeds,
// such as the value of declared variables.
pub struct DynamicContext<N: Node> {
    vars: RefCell<HashMap<String, Vec<Sequence<N>>>>,
    depth: RefCell<usize>,
    current_grouping_key: RefCell<Vec<Option<Rc<Item<N>>>>>,
    current_group: RefCell<Vec<Option<Sequence<N>>>>,
    current_import: RefCell<usize>,
    deps: RefCell<Vec<Url>>, // URIs for included/imported stylesheets
}

impl<N: Node> Default for DynamicContext<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N: Node> DynamicContext<N> {
    pub fn new() -> Self {
        DynamicContext {
            vars: RefCell::new(HashMap::new()),
            depth: RefCell::new(0),
            current_grouping_key: RefCell::new(vec![None]),
            current_group: RefCell::new(vec![None]),
            current_import: RefCell::new(0),
            deps: RefCell::new(vec![]),
        }
    }
    /// Retrieve the dependencies for the stylesheet
    // TODO: make this an iterator
    pub fn dependencies(&self) -> Vec<Url> {
        self.deps.borrow().clone()
    }
    /// Add a dependency
    pub fn add_dependency(&self, u: Url) {
        self.deps.borrow_mut().push(u);
    }

    fn push_current_grouping_key(&self, k: Item<N>) {
        self.current_grouping_key
            .borrow_mut()
            .push(Some(Rc::new(k)));
    }
    fn pop_current_grouping_key(&self) {
        self.current_grouping_key.borrow_mut().pop();
    }

    fn push_current_group(&self, g: Sequence<N>) {
        self.current_group.borrow_mut().push(Some(g));
    }
    fn pop_current_group(&self) {
        self.current_group.borrow_mut().pop();
    }
    fn depth_incr(&self) {
        let mut d = self.depth.borrow_mut();
        *d += 1;
    }
    fn depth_decr(&self) {
        let mut d = self.depth.borrow_mut();
        *d -= 1;
    }
    fn import_incr(&self) {
        let mut d = self.current_import.borrow_mut();
        *d += 1;
    }
    fn import_decr(&self) {
        let mut d = self.current_import.borrow_mut();
        *d -= 1;
    }

    // Push a new scope for a variable
    fn var_push(&self, v: &str, s: Sequence<N>) {
        let mut h: RefMut<HashMap<String, Vec<Sequence<N>>>>;
        let mut t: Option<&mut Vec<Sequence<N>>>;

        h = self.vars.borrow_mut();
        t = h.get_mut(v);
        match t.as_mut() {
            Some(u) => {
                // If the variable already has a value, then this is a new, inner scope
                u.push(s);
            }
            None => {
                // Otherwise this is the first scope for the variable
                h.insert(v.to_string(), vec![s]);
            }
        }
    }
    // Pop scope for a variable
    // Prerequisite: scope must have already been pushed
    fn var_pop(&self, v: &str) {
        self.vars.borrow_mut().get_mut(v).map(|u| u.pop());
    }

    // Stylesheet parameters. Overrides the previous value if it is already set.
    // TODO: namespaced name
    pub fn set_parameter(&self, name: String, value: Sequence<N>) {
        self.vars.borrow_mut().insert(name, vec![value]);
    }
}

/// A sequence constructor evaluator.
/// This interprets the sequence constructor to produce a sequence.
/// IDEA: make the evaluate method an iterator, emitting one sequence item at a time
/// IDEA: Combine the sequence constructor and the evaluator. Perhaps a closure?
pub struct Evaluator<N: Node> {
    dc: DynamicContext<N>,
    templates: Vec<Template<N>>,
    builtin_templates: Vec<Template<N>>, // TODO: use import precedence for builtins
    od: OutputDefinition,                // Output definition for the final result tree
    base: Option<Url>,                   // The base URL of the primary stylesheet
}

impl<N: Node> Default for Evaluator<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N: Node> Evaluator<N> {
    /// Create a dynamic context.
    pub fn new() -> Self {
        Evaluator {
            dc: DynamicContext::new(),
            templates: Vec::new(),
            builtin_templates: Vec::new(),
            od: OutputDefinition::new(),
            base: None,
        }
    }
    pub fn from_dynamic_context(dc: DynamicContext<N>) -> Self {
        Evaluator {
            dc,
            templates: Vec::new(),
            builtin_templates: Vec::new(),
            od: OutputDefinition::new(),
            base: None,
        }
    }

    /// Base URI
    pub fn baseurl(&self) -> Option<Url> {
        self.base.clone()
    }
    /// Set the Base URL
    pub fn set_baseurl(&mut self, url: Url) {
        self.base = Some(url);
    }

    /// Add a template to the dynamic context. The first argument is the pattern. The second argument is the body of the template. The third argument is the mode. The fourth argument is the priority. The fifth argument is the import precedence.
    pub fn add_template(
        &mut self,
        p: Vec<Constructor<N>>,
        b: Vec<Constructor<N>>,
        m: Option<String>,
        pr: f64,
        im: usize,
    ) {
        self.templates.push(Template {
            pattern: p,
            body: b,
            mode: m,
            priority: pr,
            import: im,
        });
    }
    /// Add a template to the set of builtin templates in the dynamic context. See above for arguments.
    pub fn add_builtin_template(
        &mut self,
        p: Vec<Constructor<N>>,
        b: Vec<Constructor<N>>,
        m: Option<String>,
        pr: f64,
        im: usize,
    ) {
        self.builtin_templates.push(Template {
            pattern: p,
            body: b,
            mode: m,
            priority: pr,
            import: im,
        });
    }
    /// Determine if an item matches a pattern and return the highest priority sequence constructor for that template.
    /// If import precedence is None, then return the lowest import precedence. Otherwise return the matching template with the highest priority that has an imoprt precedence higher than the given value.
    /// If no template is found, returns None.
    pub fn find_match(
        &self,
        i: &Rc<Item<N>>,
        im: Option<usize>,
        rd: &N,
    ) -> Result<Vec<Constructor<N>>, Error> {
        eprintln!("find_match for {:?}", i);
        let mut r: Vec<&Template<N>> = vec![];
        eprintln!("searching {} templates", self.templates.len());
        let it = self.templates.iter();
        for t in it {
            if self.item_matches(&t.pattern, i, rd)? {
                r.push(t)
            }
        }
        let s: Option<&Template<N>> = r
            .iter()
            .cloned()
            .filter(|j| im.map_or(true, |k| j.import >= k))
            .reduce(|a, b| if a.priority < b.priority { b } else { a });

        if let Some(b) = s {
            eprintln!("found template");
            Ok(b.body.clone())
        } else {
            // Try builtin templates
            eprintln!("trying built-in templates");
            let mut w: Vec<&Template<N>> = vec![];
            let builtins = self.builtin_templates.iter();
            for u in builtins {
                if self.item_matches(&u.pattern, i, rd)? {
                    w.push(u)
                }
            }
            let v = w
                .iter()
                .reduce(|a, b| if a.priority < b.priority { b } else { a });

            if let Some(b) = v {
                Ok(b.body.clone())
            } else {
                Ok(vec![])
            }
        }
    }

    // TODO: return borrowed/reference
    pub fn get_output_definition(&self) -> OutputDefinition {
        self.od.clone()
    }
    pub fn set_output_definition(&mut self, od: OutputDefinition) {
        self.od = od;
    }

    // Printout templates, for debugging.
    pub fn dump_templates(&self) {
        self.templates.iter().for_each(
	    |t| {
		println!("Template (mode \"{}\" priority {} import precedence {}) matching pattern:\n{}\nBody:\n{}",
			 t.mode.as_ref().map_or("--no mode--", |u| u.as_str()),
			 t.priority,
			 t.import,
			 format_constructor(&t.pattern, 4),
			 format_constructor(&t.body, 4)
		);
	    }
	);
        self.builtin_templates.iter().for_each(
	    |t| {
		println!("Builtin template (mode \"{}\" priority {} import precedence {}) matching pattern:\n{}\nBody:\n{}",
			 t.mode.as_ref().map_or("--no mode--", |u| u.as_str()),
			 t.priority,
			 t.import,
			 format_constructor(&t.pattern, 4),
			 format_constructor(&t.body, 4)
		);
	    }
	)
    }

    /// Evaluate a sequence constructor, given a dynamic context.
    ///
    /// The dynamic context consists of the supplied context, as well as the context item. The context item, which is optional, consists of a [Sequence] and an index to an item. If the context sequence is supplied, then the index (posn) must also be supplied and be a valid index for the sequence.
    ///
    /// Any nodes created by the sequence constructor are created in the result Tree.
    pub fn evaluate(
        &self,
        ctxt: Option<Sequence<N>>,
        posn: Option<usize>,
        c: &[Constructor<N>],
        rd: &N, // Result document
    ) -> Result<Sequence<N>, Error> {
        // Evaluate all sequence constructors. This will result in a sequence of sequences.
        // If an error occurs, propagate the first error (TODO: return all errors)
        // Otherwise, flatten the sequences into a single sequence

        let (results, errors): (Vec<_>, Vec<_>) = c
            .iter()
            .map(|a| self.evaluate_one(ctxt.clone(), posn, a, rd))
            .partition(Result::is_ok);
        if !errors.is_empty() {
            Result::Err(errors.get(0).map(|e| e.clone().err().unwrap()).unwrap())
        } else {
            Ok(results
                .iter()
                .flat_map(|a| {
                    let b: Sequence<N> = a.clone().ok().unwrap_or_default();
                    b
                })
                .collect::<Vec<Rc<Item<N>>>>())
        }
    }

    // Evaluate an item constructor, given a context
    // If a constructor returns a non-singleton sequence, then it is unpacked
    fn evaluate_one(
        &self,
        ctxt: Option<Sequence<N>>,
        posn: Option<usize>,
        c: &Constructor<N>,
        rd: &N,
    ) -> Result<Sequence<N>, Error> {
        match c {
            Constructor::Literal(l) => {
                let mut seq = Sequence::new();
                seq.push_value(l.clone());
                Ok(seq)
            }

            // This creates a Node in the current result document
            Constructor::LiteralElement(n, c) => {
                let mut l = rd.new_element(n.clone())?;

                // add content to newly created element
                let seq = self.evaluate(ctxt, posn, c, rd)?;
                seq.iter().try_for_each(|i| {
                    // Item could be a Node or text
                    match &**i {
                        Item::Node(t) => l.push(t.clone()),
                        _ => {
                            // Values become a text node in the result tree
                            let v = Value::from(i.to_string());
                            let t = rd.new_text(v)?;
                            l.push(t)
                        }
                    }
                })?;

                Ok(vec![Rc::new(Item::Node(l))])
            }
            // This creates a Node in the current result document
            Constructor::LiteralAttribute(n, v) => {
                let w = self.evaluate(ctxt, posn, v, rd)?;
                let x = Value::from(w.to_string());
                let l = rd.new_attribute(n.clone(), x)?;
                Ok(vec![Rc::new(Item::Node(l))])
            }
            Constructor::Copy(i, c) => {
                let orig = if i.is_empty() {
                    // Copy the context item
                    if ctxt.is_some() {
                        vec![ctxt.as_ref().unwrap()[posn.unwrap()].clone()]
                    } else {
                        self.evaluate(ctxt.clone(), posn, i, rd)?
                    }
                } else {
                    self.evaluate(ctxt.clone(), posn, i, rd)?
                };

                let mut results = Sequence::new();
                for j in orig {
                    let m = self.item_copy(j.clone(), c, ctxt.clone(), posn, rd)?;
                    results.push(m);
                }
                Ok(results)
            }
            // Does the same as identity stylesheet template
            Constructor::DeepCopy(sel) => {
                let orig = self.evaluate(ctxt.clone(), posn, sel, rd)?;

                let mut results = Sequence::new();
                for j in orig {
                    let m = self.item_deep_copy(j.clone(), ctxt.clone(), posn, rd)?;
                    results.push(m);
                }
                Ok(results)
            }
            Constructor::ContextItem => {
                if ctxt.is_some() {
                    let mut seq = Sequence::new();
                    seq.push_item(&ctxt.as_ref().unwrap()[posn.unwrap()]);
                    Ok(seq)
                } else {
                    Result::Err(Error {
                        kind: ErrorKind::DynamicAbsent,
                        message: "no context item".to_string(),
                    })
                }
            }
            Constructor::SetAttribute(n, v) => {
                // The context item must be an element node (TODO: use an expression to select the element)
                // If the element does not have an attribute with the given name, create it
                // Otherwise replace the attribute's value with the supplied value
                if let Some(ref c) = ctxt {
                    // Work out if an attribute is required, then set it
                    let atnode: Option<N>;
                    match &*c[posn.unwrap()] {
                        Item::Node(nd) => match nd.node_type() {
                            NodeType::Element => {
                                let attval = self.evaluate(ctxt.clone(), posn, v, rd)?;
                                if attval.len() == 1 {
                                    match &*attval[0] {
                                        Item::Value(av) => {
                                            atnode = Some(rd.new_attribute(n.clone(), av.clone())?);
                                        }
                                        _ => {
                                            let w = Value::from(attval.to_string());
                                            atnode = Some(rd.new_attribute(n.clone(), w)?);
                                        }
                                    }
                                } else {
                                    let w = Value::from(attval.to_string());
                                    atnode = Some(rd.new_attribute(n.clone(), w)?);
                                }
                            }
                            _ => {
                                return Result::Err(Error {
                                    kind: ErrorKind::TypeError,
                                    message: "context item is not an element".to_string(),
                                })
                            }
                        },
                        _ => {
                            return Result::Err(Error {
                                kind: ErrorKind::TypeError,
                                message: "context item must be a mutable element node".to_string(),
                            })
                        }
                    }
                    if let Some(a) = atnode {
                        ctxt.unwrap()[posn.unwrap()]
                            .add_attribute(a)
                            .expect("unable to add attribute");
                    }
                    Ok(vec![])
                } else {
                    Result::Err(Error {
                        kind: ErrorKind::DynamicAbsent,
                        message: "no context item".to_string(),
                    })
                }
            }
            Constructor::Or(v) => {
                // Evaluate each operand to a boolean result. Return true if any of the operands' result is true
                // Optimisation: stop upon the first true result.
                // Future: Evaluate every operand to check for dynamic errors
                let mut b = false;
                for i in v {
                    let k = self.evaluate(ctxt.clone(), posn, i, rd)?;
                    b = k.to_bool();
                    if b {
                        break;
                    };
                }
                let mut seq = Sequence::new();
                seq.push_value(Value::Boolean(b));
                Ok(seq)
            }
            Constructor::And(v) => {
                // Evaluate each operand to a boolean result. Return false if any of the operands' result is false
                // Optimisation: stop upon the first false result.
                // Future: Evaluate every operand to check for dynamic errors
                let mut b = true;
                for i in v {
                    let k = self.evaluate(ctxt.clone(), posn, i, rd)?;
                    b = k.to_bool();
                    if !b {
                        break;
                    };
                }
                let mut seq = Sequence::new();
                seq.push_value(Value::from(b));
                Ok(seq)
            }
            Constructor::GeneralComparison(o, v) => {
                if v.len() == 2 {
                    let mut seq = Sequence::new();
                    let b = self.general_comparison(ctxt, posn, *o, &v[0], &v[1], rd)?;
                    seq.push_value(Value::from(b));
                    Ok(seq)
                } else {
                    Result::Err(Error {
                        kind: ErrorKind::Unknown,
                        message: "incorrect number of operands".to_string(),
                    })
                }
            }
            Constructor::ValueComparison(o, v) => {
                if v.len() == 2 {
                    let mut seq = Sequence::new();
                    let b = self.value_comparison(ctxt, posn, *o, &v[0], &v[1], rd)?;
                    seq.push_value(Value::from(b));
                    Ok(seq)
                } else {
                    Result::Err(Error {
                        kind: ErrorKind::Unknown,
                        message: "incorrect number of operands".to_string(),
                    })
                }
            }
            Constructor::Concat(v) => {
                let mut r = String::new();
                for u in v {
                    let t = self.evaluate(ctxt.clone(), posn, u, rd)?;
                    r.push_str(t.to_string().as_str());
                }
                let mut seq = Sequence::new();
                seq.push_value(Value::from(r));
                Ok(seq)
            }
            Constructor::Range(v) => {
                if v.len() == 2 {
                    // Evaluate the two operands: they must both be literal integer items
                    let start = self.evaluate(ctxt.clone(), posn, &v[0], rd)?;
                    let end = self.evaluate(ctxt, posn, &v[1], rd)?;
                    if start.is_empty() || end.is_empty() {
                        // empty sequence is the result
                        Ok(vec![])
                    } else if start.len() == 1 {
                        if end.len() == 1 {
                            let i = start[0].to_int().unwrap();
                            let j = end[0].to_int().unwrap();

                            match i.cmp(&j) {
                                Ordering::Greater => {
                                    // empty sequence result
                                    Ok(vec![])
                                }
                                Ordering::Equal => {
                                    let mut seq = Sequence::new();
                                    seq.push_value(Value::Integer(i));
                                    Ok(seq)
                                }
                                Ordering::Less => {
                                    let mut result = Sequence::new();
                                    for k in i..=j {
                                        result.push_value(Value::from(k))
                                    }
                                    Ok(result)
                                }
                            }
                        } else {
                            Result::Err(Error {
                                kind: ErrorKind::Unknown,
                                message: String::from("end operand must be singleton"),
                            })
                        }
                    } else {
                        Result::Err(Error {
                            kind: ErrorKind::Unknown,
                            message: String::from("start operand must be singleton"),
                        })
                    }
                } else {
                    Result::Err(Error {
                        kind: ErrorKind::Unknown,
                        message: "incorrect number of operands".to_string(),
                    })
                }
            }
            Constructor::Arithmetic(v) => {
                // Type: the result will be a number, but integer or double?
                // If all of the operands are integers, then the result is integer otherwise double
                // TODO: check the type of all operands to determine type of result (can probably do this in static analysis phase)
                // In the meantime, let's assume the result will be double and convert any integers

                let mut acc: f64 = 0.0;

                for j in v {
                    let k = self.evaluate(ctxt.clone(), posn, &j.operand, rd)?;
                    let u: f64;
                    if k.len() != 1 {
                        return Result::Err(Error {
                            kind: ErrorKind::TypeError,
                            message: String::from("type error (not a singleton sequence)"),
                        });
                    } else {
                        u = k[0].to_double();
                        match j.op {
                            ArithmeticOperator::Noop => acc = u,
                            ArithmeticOperator::Add => acc += u,
                            ArithmeticOperator::Subtract => acc -= u,
                            ArithmeticOperator::Multiply => acc *= u,
                            ArithmeticOperator::Divide => acc /= u,
                            ArithmeticOperator::IntegerDivide => acc /= u, // TODO: convert to integer
                            ArithmeticOperator::Modulo => acc %= u,
                        }
                    }
                }
                let mut seq = Sequence::new();
                seq.push_value(Value::from(acc));
                Ok(seq)
            }
            Constructor::Root => {
                if ctxt.is_some() {
                    match &*ctxt.as_ref().unwrap()[posn.unwrap()] {
                        Item::Node(n) => match n.node_type() {
                            NodeType::Document => Ok(vec![Rc::new(Item::Node(n.clone()))]),
                            _ => n
                                .ancestor_iter()
                                .last()
                                .map_or(Ok(vec![]), |m| Ok(vec![Rc::new(Item::Node(m))])),
                        },
                        _ => Result::Err(Error {
                            kind: ErrorKind::ContextNotNode,
                            message: "context item is not a node".to_string(),
                        }),
                    }
                } else {
                    Result::Err(Error {
                        kind: ErrorKind::ContextNotNode,
                        message: "no context".to_string(),
                    })
                }
            }
            Constructor::Path(s) => {
                // s is a vector of sequence constructors.
                // Each item is a step in the path.
                // Each step creates a new context for the next step
                // TODO: if initial context is None then error

                // accumulator - each time around the loop this will be the new context
                let u: Sequence<N> = ctxt.unwrap_or_default();

                // TODO: Don't Panic
                let result = s.iter().fold(u, |a, c| {
                    // evaluate this step for each item in the context
                    // Add the result of each evaluation to an accummulator sequence
                    // Eliminate duplicates
                    let mut b: Sequence<N> = Vec::new();
                    for i in 0..a.len() {
                        let d = self
                            .evaluate(Some(a.clone()), Some(i), c, rd)
                            .expect("failed to evaluate step");
                        // Instead of just appending the step result to the accumulator,
                        // we need to check each item in the step result to see if it already
                        // exists in the accumulator.
                        // TODO: optimise this checking
                        //b.append(&mut d);
                        d.iter().for_each(|e| {
                            if b.iter().fold(true, |acc, f| match (&**e, &**f) {
                                (Item::Node(g), Item::Node(h)) => {
                                    if g.is_same(h) {
                                        false
                                    } else {
                                        acc
                                    }
                                }
                                _ => acc,
                            }) {
                                b.push(e.clone())
                            }
                        });
                    }
                    b
                });
                Ok(result)
            }
            Constructor::Step(nm, p) => {
                // For this step to be valid the source document must not be None
                // Performing this check every time will be a performance drain.
                // Perhaps there can be an 'unchecked' variant, or some kind of static analysis?

                if ctxt.is_some() {
                    match &*(ctxt.as_ref().unwrap()[posn.unwrap()]) {
                        Item::Node(n) => {
                            match nm.axis {
                                Axis::Selfaxis => {
                                    if is_node_match::<N>(&nm.nodetest, n) {
                                        let mut seq = Sequence::new();
                                        seq.push_node(n.clone());
                                        Ok(self.predicates(seq, p, rd)?)
                                    } else {
                                        Ok(Sequence::new())
                                    }
                                }
                                Axis::Child => {
                                    let seq = n
                                        .child_iter()
                                        .filter(|c| is_node_match::<N>(&nm.nodetest, c))
                                        .fold(Sequence::new(), |mut c, a| {
                                            c.push_node(a);
                                            c
                                        });

                                    Ok(self.predicates(seq, p, rd)?)
                                }
                                Axis::Parent => {
                                    match n.parent() {
                                        Some(p) => {
                                            let mut s = Sequence::new();
                                            s.push_node(p);
                                            Ok(s)
                                        }
                                        None => {
                                            // empty sequence is the result
                                            Ok(vec![])
                                        }
                                    }
                                }
                                Axis::ParentDocument => {
                                    // Only matches the Document.
                                    // If no parent then return the Document
                                    // NB. Document is a special kind of Node
                                    match n.node_type() {
                                        NodeType::Document => {
                                            // The context is the document
                                            Ok(vec![Rc::clone(
                                                &ctxt.as_ref().unwrap()[posn.unwrap()],
                                            )])
                                        }
                                        _ => Ok(vec![]),
                                    }
                                }
                                Axis::Descendant => {
                                    let seq = n
                                        .descend_iter()
                                        .filter(|c| is_node_match::<N>(&nm.nodetest, c))
                                        .fold(Sequence::new(), |mut c, a| {
                                            c.push_node(a);
                                            c
                                        });

                                    Ok(self.predicates(seq, p, rd)?)
                                }
                                Axis::DescendantOrSelf => {
                                    let mut seq = n
                                        .descend_iter()
                                        .filter(|c| is_node_match::<N>(&nm.nodetest, c))
                                        .fold(Sequence::new(), |mut c, a| {
                                            c.push_node(a);
                                            c
                                        });
                                    if is_node_match::<N>(&nm.nodetest, n) {
                                        seq.insert(0, Rc::new(Item::Node(n.clone())));
                                    }

                                    Ok(self.predicates(seq, p, rd)?)
                                }
                                Axis::DescendantOrSelfOrRoot => {
                                    let mut seq = n
                                        .descend_iter()
                                        .filter(|c| is_node_match::<N>(&nm.nodetest, c))
                                        .fold(Sequence::new(), |mut c, a| {
                                            c.push_node(a);
                                            c
                                        });
                                    if is_node_match::<N>(&nm.nodetest, n) {
                                        seq.insert(0, Rc::new(Item::Node(n.clone())));
                                    }
                                    seq.insert(0, Rc::new(Item::Node(n.owner_document())));

                                    Ok(self.predicates(seq, p, rd)?)
                                }
                                Axis::Ancestor => {
                                    let seq = n
                                        .ancestor_iter()
                                        .filter(|p| is_node_match::<N>(&nm.nodetest, p))
                                        .fold(Sequence::new(), |mut c, a| {
                                            c.push_node(a);
                                            c
                                        });

                                    Ok(self.predicates(seq, p, rd)?)
                                }
                                Axis::AncestorOrSelf => {
                                    let mut seq = n
                                        .ancestor_iter()
                                        .filter(|c| is_node_match::<N>(&nm.nodetest, c))
                                        .fold(Sequence::new(), |mut c, a| {
                                            c.push_node(a);
                                            c
                                        });

                                    if is_node_match::<N>(&nm.nodetest, n) {
                                        seq.push_item(&Rc::new(Item::Node(n.clone())));
                                    }

                                    Ok(self.predicates(seq, p, rd)?)
                                }
                                Axis::FollowingSibling => {
                                    let seq = n
                                        .next_iter()
                                        .filter(|c| is_node_match::<N>(&nm.nodetest, c))
                                        .fold(Sequence::new(), |mut c, a| {
                                            c.push_node(a);
                                            c
                                        });

                                    Ok(self.predicates(seq, p, rd)?)
                                }
                                Axis::PrecedingSibling => {
                                    let seq = n
                                        .prev_iter()
                                        .filter(|c| is_node_match::<N>(&nm.nodetest, c))
                                        .fold(Sequence::new(), |mut c, a| {
                                            c.push_node(a);
                                            c
                                        });

                                    Ok(self.predicates(seq, p, rd)?)
                                }
                                Axis::Following => {
                                    // XPath 3.3.2.1: the following axis contains all nodes that are descendants of the root of the tree in which the context node is found, are not descendants of the context node, and occur after the context node in document order.
                                    // iow, for each ancestor-or-self node, include every next sibling and its descendants

                                    // Start with following siblings of self
                                    let mut d = n.next_iter().fold(Vec::new(), |mut acc, a| {
                                        acc.push(a.clone());
                                        let mut b = a.descend_iter().collect();
                                        acc.append(&mut b);
                                        acc
                                    });

                                    // Now traverse ancestors
                                    n.ancestor_iter().for_each(|a| {
                                        a.next_iter().for_each(|b| {
                                            d.push(b.clone());
                                            b.descend_iter().for_each(|c| d.push(c));
                                        })
                                    });
                                    let seq = d
                                        .iter()
                                        .filter(|e| is_node_match::<N>(&nm.nodetest, *e))
                                        .fold(Sequence::new(), |mut h, g| {
                                            h.push_node(g.clone());
                                            h
                                        });
                                    Ok(self.predicates(seq, p, rd)?)
                                }
                                Axis::Preceding => {
                                    // XPath 3.3.2.1: the preceding axis contains all nodes that are descendants of the root of the tree in which the context node is found, are not ancestors of the context node, and occur before the context node in document order.
                                    // iow, for each ancestor-or-self node, include every previous sibling and its descendants

                                    let mut d: Vec<N> = Vec::new();

                                    // Start with preceding siblings of self
                                    n.prev_iter().for_each(|a| {
                                        d.push(a.clone());
                                        let mut b = a.descend_iter().collect();
                                        d.append(&mut b);
                                    });

                                    // Now traverse ancestors
                                    n.ancestor_iter().for_each(|a| {
                                        a.prev_iter().for_each(|b| {
                                            d.push(b.clone());
                                            let mut sib_descs: Vec<N> = b.descend_iter().collect();
                                            d.append(&mut sib_descs)
                                        })
                                    });
                                    let seq = d
                                        .iter()
                                        .filter(|e| is_node_match::<N>(&nm.nodetest, *e))
                                        .fold(Sequence::new(), |mut h, g| {
                                            h.push_node(g.clone());
                                            h
                                        });
                                    Ok(self.predicates(seq, p, rd)?)
                                }
                                Axis::Attribute => {
                                    let attrs = n
                                        .attribute_iter()
                                        .filter(|a| is_node_match::<N>(&nm.nodetest, a))
                                        .fold(Sequence::new(), |mut acc, i| {
                                            acc.push_node(i);
                                            acc
                                        });
                                    Ok(self.predicates(attrs, p, rd)?)
                                }
                                Axis::SelfDocument => {
                                    if n.node_type() == NodeType::Document {
                                        Ok(vec![Rc::clone(&ctxt.as_ref().unwrap()[posn.unwrap()])])
                                    } else {
                                        Ok(vec![])
                                    }
                                }
                                Axis::SelfAttribute => {
                                    if n.node_type() == NodeType::Attribute {
                                        Ok(vec![Rc::clone(&ctxt.as_ref().unwrap()[posn.unwrap()])])
                                    } else {
                                        Ok(vec![])
                                    }
                                }
                                _ => {
                                    // Not yet implemented
                                    Result::Err(Error {
                                        kind: ErrorKind::NotImplemented,
                                        message: "not yet implemented (node)".to_string(),
                                    })
                                }
                            }
                        }
                        _ => Result::Err(Error {
                            kind: ErrorKind::ContextNotNode,
                            message: "context item is not a node".to_string(),
                        }),
                    }
                } else {
                    Result::Err(Error {
                        kind: ErrorKind::DynamicAbsent,
                        message: "no context item".to_string(),
                    })
                }
            }
            Constructor::FunctionCall(h, a) => {
                match h.body {
                    Some(g) => {
                        // Evaluate the arguments
                        let mut b = Vec::new();
                        for c in a {
                            let r = self.evaluate(ctxt.clone(), posn, c, rd)?;
                            b.push(r)
                        }
                        // Invoke the function
                        Ok(g(self, ctxt, posn, b, rd)?)
                    }
                    None => Result::Err(Error {
                        kind: ErrorKind::NotImplemented,
                        message: format!("call to undefined function \"{}\"", h.name),
                    }),
                }
            }
            Constructor::VariableDeclaration(v, a) => {
                let s = self.evaluate(ctxt, posn, a, rd)?;
                //     	let mut t: Vec<Sequence>;
                self.dc.var_push(v, s);
                //      	match dc.vars.borrow().get(v) {
                //          Some(u) => {
                //	    t = u.to_vec();
                //	    t.push(s)
                //	  }
                //	  None => {
                //	    t = vec![s]
                //	  }
                //        }
                //      	dc.vars.insert(v.to_string(), t);
                Ok(Sequence::new())
            }
            Constructor::VariableReference(v) => match self.dc.vars.borrow().get(v) {
                Some(s) => match s.last() {
                    Some(t) => Ok(t.clone()),
                    None => Result::Err(Error {
                        kind: ErrorKind::Unknown,
                        message: "no value for variable".to_string(),
                    }),
                },
                None => Result::Err(Error {
                    kind: ErrorKind::Unknown,
                    message: format!("reference to undefined variable \"{}\"", v),
                }),
            },
            Constructor::Loop(v, b) => {
                // TODO: this supports only one variable binding - need to support more than one binding
                // Evaluate the variable value
                // Iterate over the items in the sequence
                // Set the variable value to the item
                // Evaluate the body, collecting the results

                if v.is_empty() {
                    Result::Err(Error {
                        kind: ErrorKind::Unknown,
                        message: "no variable bindings".to_string(),
                    })
                } else {
                    let mut result: Sequence<N> = vec![];
                    if let Constructor::VariableDeclaration(v, a) = &v[0] {
                        let s = self.evaluate(ctxt.clone(), posn, a, rd)?;

                        for i in s {
                            // Push the new value for this variable
                            self.dc.var_push(v, vec![i]);
                            let mut x = self.evaluate(ctxt.clone(), posn, b, rd)?;
                            result.append(&mut x);
                            // Pop the value for this variable
                            self.dc.var_pop(v);
                        }
                    }
                    Ok(result)
                }
            }
            Constructor::Switch(v, o) => {
                // 'v' are pairs of test,body
                // 'o' is the otherwise clause
                // evaluate tests to a boolean until the first true result; evaluate it's body as the result
                // of all tests fail then evaluate otherwise clause

                let mut candidate = self.evaluate(ctxt.clone(), posn, o, rd)?;
                for t in v.chunks(2) {
                    let x = self.evaluate(ctxt.clone(), posn, &t[0], rd)?;
                    if x.to_bool() {
                        candidate = self.evaluate(ctxt, posn, &t[1], rd)?;
                        break;
                    }
                }
                Ok(candidate)
            }
            Constructor::ApplyTemplates(s) => {
                // Evaluate 's' to find the nodes to apply templates to
                // For each node, find a matching template and evaluate its sequence constructor. The result of that becomes an item in the new sequence

                let sel = self.evaluate(ctxt, posn, s, rd)?;
                // TODO: Don't Panic
                let result = sel.iter().fold(vec![], |mut acc, i| {
                    let mut matching_template: Vec<&Template<N>> = vec![];
                    for t in &self.templates {
                        let e = self
                            .evaluate(Some(vec![i.clone()]), Some(0), &t.pattern, rd)
                            .expect("evaluating pattern failed");
                        if !e.is_empty() {
                            matching_template.push(t)
                        }
                    }

                    if !matching_template.is_empty() {
                        // find the template(s) with the lowest priority
                        matching_template
                            .sort_unstable_by(|s, t| s.priority.partial_cmp(&t.priority).unwrap());
                        let l = matching_template[0].priority;
                        let mut mt_lowest: Vec<&Template<N>> = matching_template
                            .into_iter()
                            .take_while(|t| t.priority == l)
                            .collect();

                        // It's OK to have more than one matching template, if they all have different import precedence
                        mt_lowest.sort_unstable_by_key(|t| t.import);
                        let mut p = mt_lowest[0].import;
                        mt_lowest.iter().skip(1).for_each(|t| {
                            if t.import == p {
                                panic!("too many matching templates")
                            } else {
                                p = t.import;
                            }
                        });

                        // Use the template with the lowest import precedence
                        // Unless we're inside an apply-imports
                        let mut u = mt_lowest
                            .iter()
                            .take(1)
                            .flat_map(|t| {
                                self.dc.depth_incr();
                                let rs = self
                                    .evaluate(Some(vec![i.clone()]), Some(0), &t.body, rd)
                                    .expect("failed to evaluate template body");
                                self.dc.depth_decr();
                                rs
                            })
                            .collect::<Sequence<N>>();
                        acc.append(&mut u);
                    } else {
                        // If no templates match then apply a built-in template
                        // See XSLT 6.7.
                        // TODO: use import precedence to implement this feature
                        let builtin_template: Vec<&Template<N>> = self
                            .builtin_templates
                            .iter()
                            .filter(|t| {
                                let e = self
                                    .evaluate(Some(vec![i.clone()]), Some(0), &t.pattern, rd)
                                    .expect("failed to evaluate pattern");
                                !e.is_empty()
                            })
                            .scan(-2.0, |prio, t| {
                                if *prio < t.priority {
                                    *prio = t.priority;
                                    Some(t)
                                } else {
                                    None
                                }
                            })
                            .collect();
                        if builtin_template.len() > 1 {
                            panic!("too many matching builtin templates")
                        }
                        let mut u = builtin_template
                            .iter()
                            .flat_map(|t| {
                                self.dc.depth_incr();
                                let rs = self
                                    .evaluate(Some(vec![i.clone()]), Some(0), &t.body, rd)
                                    .expect("failed to evaluate template body");
                                self.dc.depth_decr();
                                rs
                            })
                            .collect::<Sequence<N>>();
                        acc.append(&mut u);
                    }
                    acc
                });
                Ok(result)
            }
            Constructor::ApplyImports => {
                // Evaluate templates with higher import precedence
                // Find a matching template with import precedence greater than the current precedence
                // and evaluate its sequence constructor.
                // The result of that becomes an item in the new sequence

                let mut result = vec![];
                let mut matching_template: Vec<&Template<N>> = vec![];
                for t in &self.templates {
                    let e = self
                        .evaluate(ctxt.clone(), posn, &t.pattern, rd)
                        .expect("evaluating pattern failed");
                    if !e.is_empty() {
                        matching_template.push(t)
                    }
                }

                if !matching_template.is_empty() {
                    // find the template(s) with the lowest priority
                    matching_template
                        .sort_unstable_by(|s, t| s.priority.partial_cmp(&t.priority).unwrap());
                    let l = matching_template[0].priority;
                    let mut mt_lowest: Vec<&Template<N>> = matching_template
                        .into_iter()
                        .take_while(|t| t.priority == l)
                        .collect();

                    // No need to check for multiple matches here,
                    // since this was checked by the apply-templates
                    mt_lowest.sort_unstable_by_key(|t| t.import);

                    // Find the template with the lowest import precedence
                    // higher than the current precedence
                    let mut u = mt_lowest
                        .iter()
                        .skip_while(|t| t.import <= *self.dc.current_import.borrow())
                        .take(1)
                        .flat_map(|t| {
                            self.dc.depth_incr();
                            self.dc.import_incr();
                            let rs = self
                                .evaluate(ctxt.clone(), posn, &t.body, rd)
                                .expect("failed to evaluate template body");
                            self.dc.depth_decr();
                            self.dc.import_decr();
                            rs
                        })
                        .collect::<Sequence<N>>();
                    result.append(&mut u);
                } else {
                    // If no templates match then apply a built-in template
                    // See XSLT 6.7.
                    // TODO: use import precedence to implement this feature
                    let builtin_template: Vec<&Template<N>> = self
                        .builtin_templates
                        .iter()
                        .filter(|t| {
                            let e = self
                                .evaluate(ctxt.clone(), posn, &t.pattern, rd)
                                .expect("failed to evaluate pattern");
                            !e.is_empty()
                        })
                        .scan(-2.0, |prio, t| {
                            if *prio < t.priority {
                                *prio = t.priority;
                                Some(t)
                            } else {
                                None
                            }
                        })
                        .collect();
                    if builtin_template.len() > 1 {
                        panic!("too many matching builtin templates")
                    }
                    let mut u = builtin_template
                        .iter()
                        .flat_map(|t| {
                            self.dc.depth_incr();
                            let rs = self
                                .evaluate(ctxt.clone(), posn, &t.body, rd)
                                .expect("failed to evaluate template body");
                            self.dc.depth_decr();
                            rs
                        })
                        .collect::<Sequence<N>>();
                    result.append(&mut u);
                }
                Ok(result)
            }
            Constructor::ForEach(s, t, g) => {
                // Evaluate 's' to find the nodes to iterate over
                // Use 'g' to group the nodes
                // Evaluate 't' for each group
                let sel = self.evaluate(ctxt, posn, s, rd)?;
                // Divide sel into groups: each item in groups is an individual group
                let mut groups = Vec::new();
                match g {
                    Some(Grouping::By(h)) => {
                        // 'h' is an expression that when evaluated for an item results in zero or more grouping keys.
                        // Items are placed in the group with a matching key
                        let mut map = HashMap::new();
                        for i in 0..sel.len() {
                            let keys = self.evaluate(Some(sel.clone()), Some(i), h, rd)?;
                            for j in keys {
                                let e: &mut Vec<Rc<Item<N>>> =
                                    map.entry(j.to_string()).or_default();
                                e.push(sel[i].clone());
                            }
                        }
                        // Now construct the groups and a pair-wise vector of keys
                        for (k, v) in map.iter() {
                            groups.push((Some(k.clone()), v.to_vec()));
                        }
                    }
                    Some(Grouping::Adjacent(h)) => {
                        // 'h' is an expression that is evaluated for every item in 'sel'.
                        // It must evaluate to a single item.
                        // The first item starts the first group.
                        // For the second and subsequent items, if the result of 'h; is the same as the previous item's 'h'
                        // then it is added to the current group. Otherwise it starts a new group.
                        if !sel.is_empty() {
                            let mut curgrp = vec![sel[0].clone()];
                            let mut curkey = self.evaluate(Some(sel.clone()), Some(1), h, rd)?;
                            if curkey.len() != 1 {
                                return Result::Err(Error {
                                    kind: ErrorKind::Unknown,
                                    message:
                                        "group-adjacent attribute must evaluate to a single item"
                                            .to_string(),
                                });
                            }
                            for i in 1..sel.len() {
                                let thiskey = self.evaluate(Some(sel.clone()), Some(i), h, rd)?;
                                if thiskey.len() == 1 {
                                    if curkey[0].compare(&*thiskey[0], Operator::Equal)? {
                                        // Append to the current group
                                        curgrp.push(sel[i].clone());
                                    } else {
                                        // Close previous group, start a new group with this item as its first member
                                        groups.push((Some(curkey.to_string()), curgrp.clone()));
                                        curgrp = vec![sel[i].clone()];
                                        curkey = thiskey;
                                    }
                                } else {
                                    return Result::Err(Error{kind: ErrorKind::TypeError, message: "group-adjacent attribute must evaluate to a single item".to_string()});
                                }
                            }
                            // Close the last group
                            groups.push((Some(curkey.to_string()), curgrp));
                        } // else result is empty sequence
                    }
                    Some(Grouping::StartingWith(_h)) => {}
                    Some(Grouping::EndingWith(_h)) => {}
                    None => {
                        for i in sel {
                            groups.push((None, vec![i.clone()]));
                        }
                    }
                }

                let result = groups.iter().fold(vec![], |mut result, grp| {
                    let (o, v) = grp;
                    // set current-grouping-key, current-group
                    match o {
                        Some(u) => {
                            self.dc
                                .push_current_grouping_key(Item::Value(Value::from(u.to_string())));
                            self.dc.push_current_group(v.clone());
                        }
                        None => {}
                    }
                    // TODO: Don't Panic
                    let mut tmp = self
                        .evaluate(Some(v.to_vec()), Some(0), t, rd)
                        .expect("failed to evaluate template");
                    result.append(&mut tmp);
                    // Restore current-grouping-key, current-group
                    self.dc.pop_current_grouping_key();
                    self.dc.pop_current_group();
                    result
                });
                Ok(result)
            }
            Constructor::NotImplemented(m) => Result::Err(Error {
                kind: ErrorKind::NotImplemented,
                message: format!("sequence constructor not implemented: {}", m),
            }),
        }
    }

    // Deep copy an item
    fn item_deep_copy(
        &self,
        orig: Rc<Item<N>>,
        ctxt: Option<Sequence<N>>,
        posn: Option<usize>,
        rd: &N,
    ) -> Result<Rc<Item<N>>, Error> {
        let cp = self.item_copy(orig.clone(), &[], ctxt, posn, rd)?;

        if let Item::Node(n) = &*orig {
            if n.node_type() == NodeType::Element {
                let mut _cur = match *cp {
                    Item::Node(ref m) => m,
                    _ => {
                        return Result::Err(Error {
                            kind: ErrorKind::Unknown,
                            message: "unable to copy element node".to_string(),
                        })
                    }
                };
                // To handle borrowing correctly:
                // Iterate over the attributes
                // Work out what attributes need to be created
                // Then create them
                let new: Vec<N> = n.attribute_iter().collect();
                // TODO: Don't Panic
                new.iter().for_each(|a| {
                    let _at = rd
                        .new_attribute(a.name(), a.value())
                        .expect("unable to create attribute");
                    // TODO: cannot borrow as mutable
                    //cur.add_attribute(at)
                    //    .expect("unable to add attribute");
                });
                // TODO
                //            n.child_iter()
                //                .for_each(|c| {
                //                cur.push(self.item_deep_copy(Rc::new(Item::Node(c)), ctxt.clone(), posn, rd))
                //                });
            }
        }

        /*
        // If this item is an element node, then copy all of its attributes and children
        match &*orig {
            Item::Node(n) => {
                 if n.node_type() == NodeType::Element {
                     let mut _cur = match *cp {
                         Item::Node(ref m) => m,
                         _ => {
                             return Result::Err(Error {
                                 kind: ErrorKind::Unknown,
                                 message: "unable to copy element node".to_string(),
                             })
                         }
                     };
                     // To handle borrowing correctly:
                     // Iterate over the attributes
                     // Work out what attributes need to be created
                     // Then create them
                     let new: Vec<N> = n.attribute_iter().collect();
                     // TODO: Don't Panic
                     new.iter().for_each(|a| {
                         let _at = rd
                             .new_attribute(a.name(), a.value())
                             .expect("unable to create attribute");
                         // TODO: cannot borrow as mutable
                         //cur.add_attribute(at)
                         //    .expect("unable to add attribute");
                     });
                     // TODO
                     //            n.child_iter()
                     //                .for_each(|c| {
                     //                cur.push(self.item_deep_copy(Rc::new(Item::Node(c)), ctxt.clone(), posn, rd))
                     //                });
                 }
            }
            _ => {}
        }*/

        Ok(cp)
    }

    // Copy an item
    fn item_copy(
        &self,
        orig: Rc<Item<N>>,
        content: &[Constructor<N>],
        ctxt: Option<Sequence<N>>,
        posn: Option<usize>,
        rd: &N,
    ) -> Result<Rc<Item<N>>, Error> {
        match &*orig {
            Item::Value(_) => Ok(orig.clone()),
            Item::Node(n) => {
                match n.node_type() {
                    NodeType::Element => {
                        let qn = n.name();
                        match rd.new_element(qn) {
                            Ok(e) => {
                                // Add content to the new element
                                // TODO: Don't Panic
                                let r = self.evaluate(ctxt, posn, content, rd)?;
                                r.iter().for_each(|i| {
                                    // Item could be a Node or text
                                    match &**i {
                                        Item::Node(t) => {
                                            match t.node_type() {
                                                NodeType::Element | NodeType::Text => {
                                                    // TODO: cannot borrow e as mutable
                                                    //e.push(t)
                                                    //  .expect("unable to add child node");
                                                }
                                                NodeType::Attribute => {
                                                    // TODO: cannot borrow as mutable
                                                    //e.add_attribute(t)
                                                    //  .expect("unable to add attribute node");
                                                }
                                                _ => {} // TODO: work out what to do with documents, etc
                                            }
                                        }
                                        _ => {
                                            // Values become a text node in the result tree
                                            let x = Value::from(i.to_string());
                                            let _h =
                                                rd.new_text(x).expect("unable to create text node");
                                            // TODO: cannot borrow as mutable
                                            //e.push(h)
                                            //  .expect("unable to add child text node");
                                        }
                                    }
                                });
                                Ok(Rc::new(Item::Node(e)))
                            }
                            _ => Result::Err(Error {
                                kind: ErrorKind::Unknown,
                                message: "unable to create element node".to_string(),
                            }),
                        }
                    }
                    NodeType::Text => {
                        let x = Value::from(n.to_string());
                        match rd.new_text(x) {
                            Ok(m) => Ok(Rc::new(Item::Node(m))),
                            _ => Result::Err(Error {
                                kind: ErrorKind::Unknown,
                                message: "unable to create text node".to_string(),
                            }),
                        }
                    }
                    NodeType::Attribute => {
                        // TODO: add a 'to_value' method
                        let qn = n.name();
                        let x = Value::from(n.to_string());
                        match rd.new_attribute(qn, x) {
                            Ok(a) => Ok(Rc::new(Item::Node(a))),
                            _ => Result::Err(Error {
                                kind: ErrorKind::Unknown,
                                message: "unable to create attribute node".to_string(),
                            }),
                        }
                    }
                    _ => Result::Err(Error {
                        kind: ErrorKind::NotImplemented,
                        message: "select expression not implemented".to_string(),
                    }),
                }
            }
            _ => Result::Err(Error {
                kind: ErrorKind::NotImplemented,
                message: "not implemented".to_string(),
            }),
        }
    }

    // Filter the sequence with each of the predicates
    fn predicates(
        &self,
        s: Sequence<N>,
        p: &Vec<Vec<Constructor<N>>>,
        rd: &N,
    ) -> Result<Sequence<N>, Error> {
        if p.is_empty() {
            Ok(s)
        } else {
            let mut result = s;

            // iterate over the predicates
            for q in p {
                let mut new: Sequence<N> = Vec::new();

                // for each predicate, evaluate each item in s to a boolean
                for i in 0..result.len() {
                    let b = self.evaluate(Some(result.clone()), Some(i), q, rd)?;
                    if b.to_bool() {
                        new.push(result[i].clone());
                    }
                }
                result.clear();
                result.append(&mut new);
            }

            Ok(result)
        }
    }

    /// Determine if an item matches a pattern.
    /// The sequence constructor is a pattern: the steps of a path in reverse.
    pub fn item_matches(
        &self,
        pat: &[Constructor<N>],
        i: &Rc<Item<N>>,
        rd: &N,
    ) -> Result<bool, Error> {
        let e = self.evaluate(Some(vec![i.clone()]), Some(0), pat, rd)?;

        // If anything is left in the context then the pattern matched
        if !e.is_empty() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn general_comparison(
        &self,
        ctxt: Option<Sequence<N>>,
        posn: Option<usize>,
        op: Operator,
        left: &[Constructor<N>],
        right: &[Constructor<N>],
        rd: &N,
    ) -> Result<bool, Error> {
        let mut b = false;
        let left_seq = self.evaluate(ctxt.clone(), posn, left, rd)?;
        let right_seq = self.evaluate(ctxt, posn, right, rd)?;
        for l in left_seq {
            for r in &right_seq {
                b = l.compare(r, op).unwrap();
                if b {
                    break;
                }
            }
            if b {
                break;
            }
        }
        Ok(b)
    }

    // Operands must be singletons
    fn value_comparison(
        &self,
        ctxt: Option<Sequence<N>>,
        posn: Option<usize>,
        op: Operator,
        left: &[Constructor<N>],
        right: &[Constructor<N>],
        rd: &N,
    ) -> Result<bool, Error> {
        let left_seq = self.evaluate(ctxt.clone(), posn, left, rd)?;
        if left_seq.is_empty() {
            return Result::Err(Error {
                kind: ErrorKind::TypeError,
                message: String::from("left-hand sequence is empty"),
            });
        }
        // TODO: Don't Panic
        if left_seq.len() == 1 {
            let right_seq = self.evaluate(ctxt, posn, right, rd)?;
            if right_seq.len() == 1 {
                Ok(left_seq[0].compare(&*right_seq[0], op).unwrap())
            } else {
                Result::Err(Error {
                    kind: ErrorKind::TypeError,
                    message: String::from("right-hand sequence is not a singleton sequence"),
                })
            }
        } else {
            Result::Err(Error {
                kind: ErrorKind::TypeError,
                message: String::from("left-hand sequence is not a singleton sequence"),
            })
        }
    }
}

/// Specifies how a sequence is to be constructed.
///
/// These are usually included in a Vector, where each Constructor builds an item. If the constructor results in a singleton, then it becomes an item in the [Sequence], otherwise the sequence is unpacked into the parent [Sequence].
#[derive(Clone)]
pub enum Constructor<N: Node> {
    /// A literal, atomic value
    Literal(Value),
    /// A literal element. This will become a node in the result tree.
    /// TODO: this may be merged with the Literal option in a later version.
    /// Arguments are: element name, content
    LiteralElement(QualifiedName, Vec<Constructor<N>>),
    /// A literal attribute. This will become a node in the result tree.
    /// TODO: allow for attribute value templates
    /// Arguments are: attribute name, value
    LiteralAttribute(QualifiedName, Vec<Constructor<N>>),
    /// Construct a node by copying something. The first argument is what to copy; an empty vector selects the current item. The second argument constructs the content.
    Copy(Vec<Constructor<N>>, Vec<Constructor<N>>),
    DeepCopy(Vec<Constructor<N>>),
    /// The context item from the dynamic context
    ContextItem,
    /// Logical OR. Each element of the outer vector is an operand.
    Or(Vec<Vec<Constructor<N>>>),
    /// Logical AND. Each element of the outer vector is an operand.
    And(Vec<Vec<Constructor<N>>>),
    // Union,
    // IntersectExcept,
    // InstanceOf,
    // Treat,
    // Castable,
    // Cast,
    // Arrow,
    // Unary,
    // SimpleMap,
    /// Root node of the context item
    Root,
    /// A path in a tree of nodes.
    /// Each element of the outer vector is a step in the path.
    /// The result of each step becomes the new context for the next step.
    Path(Vec<Vec<Constructor<N>>>),
    /// A step in a path.
    /// The second argument is zero or more predicates.
    /// Each item in the result sequence is evaluated against each predicate as a boolean.
    /// If the predicate evaluates to true it is kept, otherwise it is discarded.
    Step(NodeMatch, Vec<Vec<Constructor<N>>>),
    /// XPath general comparison.
    /// Each element of the outer vector is a comparator.
    /// If the comparator is a sequence then each item is compared.
    GeneralComparison(Operator, Vec<Vec<Constructor<N>>>),
    /// XPath value comparison. Compares single items.
    ValueComparison(Operator, Vec<Vec<Constructor<N>>>),
    // Is,
    // Before,
    // After,
    /// Concatentate string values
    Concat(Vec<Vec<Constructor<N>>>),
    /// Construct a range of integers
    Range(Vec<Vec<Constructor<N>>>),
    /// Perform addition, subtraction, multiply, divide
    Arithmetic(Vec<ArithmeticOperand<N>>),
    /// Call a function
    FunctionCall(Function<N>, Vec<Vec<Constructor<N>>>),
    /// Declare a variable.
    /// The variable will be available for subsequent constructors
    VariableDeclaration(String, Vec<Constructor<N>>), // TODO: support QName
    /// Reference a variable.
    VariableReference(String), // TODO: support QName
    /// Repeating constructor (i.e. 'for').
    /// The first argument declares variables.
    /// The second argument is the body of the loop.
    Loop(Vec<Constructor<N>>, Vec<Constructor<N>>),
    /// Selects an arm to evaluate.
    /// The first argument is pairs of (test,body) clauses.
    /// The second argument is the otherwise clause
    Switch(Vec<Vec<Constructor<N>>>, Vec<Constructor<N>>),
    /// Find a matching template and evaluate its sequence constructor.
    /// The argument is the select attribute.
    ApplyTemplates(Vec<Constructor<N>>),
    /// Find a matching template at the next import precedence
    /// and evaluate its sequence constructor.
    ApplyImports,
    /// Evaluate a sequence constructor for each item, possibly grouped.
    /// First argument is the select expression, second argument is the template,
    /// third argument is the (optional) grouping spec.
    ForEach(
        Vec<Constructor<N>>,
        Vec<Constructor<N>>,
        Option<Grouping<N>>,
    ),
    /// Set the value of an attribute. Context item must be an element node.
    /// First argument is the name of the attribute, second attribute is the value to set
    SetAttribute(QualifiedName, Vec<Constructor<N>>),
    /// Something that is not yet implemented
    NotImplemented(String),
}

/// Determine how a collection is to be divided into groups.
/// This enum would normally be inside an Option. The None value means that the collection is not to be grouped.
#[derive(Clone)]
pub enum Grouping<N: Node> {
    By(Vec<Constructor<N>>),
    StartingWith(Vec<Constructor<N>>),
    EndingWith(Vec<Constructor<N>>),
    Adjacent(Vec<Constructor<N>>),
}

// Apply the node test to a Node.
// TODO: Make this a method of the Node trait?
pub(crate) fn is_node_match<N: Node>(nt: &NodeTest, n: &N) -> bool {
    match nt {
        NodeTest::Name(t) => {
            match n.node_type() {
                NodeType::Element | NodeType::Attribute => {
                    // TODO: namespaces
                    match &t.name {
                        Some(a) => match a {
                            WildcardOrName::Wildcard => true,
                            WildcardOrName::Name(s) => *s == n.name().get_localname(),
                        },
                        None => false,
                    }
                }
                _ => false,
            }
        }
        NodeTest::Kind(k) => {
            match k {
                KindTest::DocumentTest => matches!(n.node_type(), NodeType::Document),
                KindTest::ElementTest => matches!(n.node_type(), NodeType::Element),
                KindTest::PITest => matches!(n.node_type(), NodeType::ProcessingInstruction),
                KindTest::CommentTest => matches!(n.node_type(), NodeType::Comment),
                KindTest::TextTest => matches!(n.node_type(), NodeType::Text),
                KindTest::AnyKindTest => match n.node_type() {
                    NodeType::Document => false,
                    _ => true,
                },
                KindTest::AttributeTest
                | KindTest::SchemaElementTest
                | KindTest::SchemaAttributeTest
                | KindTest::NamespaceNodeTest => false, // TODO: not yet implemented
            }
        }
    }
}

#[derive(Clone)]
pub struct NodeMatch {
    pub axis: Axis,
    pub nodetest: NodeTest,
}

impl fmt::Display for NodeMatch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(format!("NodeMatch {}::{}", self.axis, self.nodetest).as_str())
    }
}

#[derive(Clone)]
pub enum NodeTest {
    Kind(KindTest),
    Name(NameTest),
}

impl TryFrom<&str> for NodeTest {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        // Import this from xpath.rs?
        let tok: Vec<&str> = s.split(':').collect();
        match tok.len() {
            1 => {
                // unprefixed
                if tok[0] == "*" {
                    Ok(NodeTest::Name(NameTest {
                        name: Some(WildcardOrName::Wildcard),
                        ns: None,
                        prefix: None,
                    }))
                } else {
                    Ok(NodeTest::Name(NameTest {
                        name: Some(WildcardOrName::Name(tok[0].to_string())),
                        ns: None,
                        prefix: None,
                    }))
                }
            }
            2 => {
                // prefixed
                if tok[0] == "*" {
                    if tok[1] == "*" {
                        Ok(NodeTest::Name(NameTest {
                            name: Some(WildcardOrName::Wildcard),
                            ns: Some(WildcardOrName::Wildcard),
                            prefix: None,
                        }))
                    } else {
                        Ok(NodeTest::Name(NameTest {
                            name: Some(WildcardOrName::Name(tok[1].to_string())),
                            ns: Some(WildcardOrName::Wildcard),
                            prefix: None,
                        }))
                    }
                } else if tok[1] == "*" {
                    Ok(NodeTest::Name(NameTest {
                        name: Some(WildcardOrName::Wildcard),
                        ns: None,
                        prefix: Some(tok[0].to_string()),
                    }))
                } else {
                    Ok(NodeTest::Name(NameTest {
                        name: Some(WildcardOrName::Name(tok[1].to_string())),
                        ns: None,
                        prefix: Some(tok[0].to_string()),
                    }))
                }
            }
            _ => Result::Err(Error {
                kind: ErrorKind::TypeError,
                message: "invalid NodeTest".to_string(),
            }),
        }
    }
}

impl fmt::Display for NodeTest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let result = match self {
            NodeTest::Name(nt) => nt.to_string(),
            NodeTest::Kind(kt) => kt.to_string().to_string(),
        };
        f.write_str(result.as_str())
    }
}

#[derive(Clone)]
pub enum KindTest {
    DocumentTest,
    ElementTest,
    AttributeTest,
    SchemaElementTest,
    SchemaAttributeTest,
    PITest,
    CommentTest,
    TextTest,
    NamespaceNodeTest,
    AnyKindTest,
}

impl KindTest {
    pub fn to_string(&self) -> &'static str {
        match self {
            KindTest::DocumentTest => "DocumentTest",
            KindTest::ElementTest => "ElementTest",
            KindTest::AttributeTest => "AttributeTest",
            KindTest::SchemaElementTest => "SchemaElementTest",
            KindTest::SchemaAttributeTest => "SchemaAttributeTest",
            KindTest::PITest => "PITest",
            KindTest::CommentTest => "CommentTest",
            KindTest::TextTest => "TextTest",
            KindTest::NamespaceNodeTest => "NamespaceNodeTest",
            KindTest::AnyKindTest => "AnyKindTest",
        }
    }
}

#[derive(Clone)]
pub struct NameTest {
    pub ns: Option<WildcardOrName>,
    pub prefix: Option<String>,
    pub name: Option<WildcardOrName>,
}

impl fmt::Display for NameTest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let result = if self.name.is_some() {
            match self.name.as_ref().unwrap() {
                WildcardOrName::Wildcard => "*".to_string(),
                WildcardOrName::Name(n) => n.to_string(),
            }
        } else {
            "--no name--".to_string()
        };
        f.write_str(result.as_str())
    }
}

#[derive(Clone)]
pub enum WildcardOrName {
    Wildcard,
    Name(String),
}

#[derive(Copy, Clone)]
pub enum Axis {
    Child,
    Descendant,
    DescendantOrSelf,
    DescendantOrSelfOrRoot,
    Attribute,
    SelfAttribute, // a special axis, only for matching an attribute in a a pattern match
    Selfaxis,
    SelfDocument, // a special axis, only for matching the Document in a pattern match
    Following,
    FollowingSibling,
    Namespace,
    Parent,
    ParentDocument, // a special axis, only for matching in a pattern match. Matches the parent as well as the Document.
    Ancestor,
    AncestorOrSelf,
    Preceding,
    PrecedingSibling,
    Unknown,
}

impl From<&str> for Axis {
    fn from(s: &str) -> Self {
        match s {
            "child" => Axis::Child,
            "descendant" => Axis::Descendant,
            "descendant-or-self" => Axis::DescendantOrSelf,
            "attribute" => Axis::Attribute,
            "self" => Axis::Selfaxis,
            "following" => Axis::Following,
            "following-sibling" => Axis::FollowingSibling,
            "namespace" => Axis::Namespace,
            "parent" => Axis::Parent,
            "ancestor" => Axis::Ancestor,
            "ancestor-or-self" => Axis::AncestorOrSelf,
            "preceding" => Axis::Preceding,
            "preceding-sibling" => Axis::PrecedingSibling,
            _ => Axis::Unknown,
        }
    }
}

impl Axis {
    fn opposite(&self) -> Axis {
        // SelfDocument opposite is undefined
        match self {
            Axis::Child => Axis::Parent,
            Axis::Descendant => Axis::Ancestor,
            Axis::DescendantOrSelf => Axis::AncestorOrSelf,
            Axis::DescendantOrSelfOrRoot => Axis::AncestorOrSelf,
            Axis::Attribute => Axis::SelfAttribute,
            Axis::Selfaxis => Axis::Selfaxis,
            Axis::Following => Axis::Preceding,
            Axis::FollowingSibling => Axis::PrecedingSibling,
            Axis::Namespace => Axis::Parent,
            Axis::Parent => Axis::Child,
            Axis::Ancestor => Axis::Descendant,
            Axis::AncestorOrSelf => Axis::DescendantOrSelf,
            Axis::Preceding => Axis::Following,
            Axis::PrecedingSibling => Axis::FollowingSibling,
            _ => Axis::Unknown,
        }
    }
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let result = match self {
            Axis::Child => "child".to_string(),
            Axis::Descendant => "descendant".to_string(),
            Axis::DescendantOrSelf => "descendant-or-self".to_string(),
            Axis::DescendantOrSelfOrRoot => "descendant-or-self-or-root".to_string(),
            Axis::Attribute => "attribute".to_string(),
            Axis::SelfAttribute => "self-attribute".to_string(),
            Axis::Selfaxis => "self".to_string(),
            Axis::SelfDocument => "self-document".to_string(),
            Axis::Following => "following".to_string(),
            Axis::FollowingSibling => "following-sibling".to_string(),
            Axis::Namespace => "namespace".to_string(),
            Axis::Parent => "parent".to_string(),
            Axis::ParentDocument => "parent-document".to_string(),
            Axis::Ancestor => "ancestor".to_string(),
            Axis::AncestorOrSelf => "ancestor-or-self".to_string(),
            Axis::Preceding => "preceding".to_string(),
            Axis::PrecedingSibling => "preceding-sibling".to_string(),
            _ => "unknown".to_string(),
        };
        f.write_str(result.as_str())
    }
}

#[derive(Copy, Clone)]
pub enum ArithmeticOperator {
    Noop,
    Add,
    Multiply,
    Divide,
    IntegerDivide,
    Subtract,
    Modulo,
}

impl fmt::Display for ArithmeticOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArithmeticOperator::Add => f.write_str("+"),
            ArithmeticOperator::Multiply => f.write_str("*"),
            ArithmeticOperator::Divide => f.write_str("div"),
            ArithmeticOperator::IntegerDivide => f.write_str("idiv"),
            ArithmeticOperator::Subtract => f.write_str("-"),
            ArithmeticOperator::Modulo => f.write_str("mod"),
            ArithmeticOperator::Noop => f.write_str("noop"),
        }
    }
}

impl From<&str> for ArithmeticOperator {
    fn from(a: &str) -> Self {
        match a {
            "+" => ArithmeticOperator::Add,
            "*" => ArithmeticOperator::Multiply,
            "div" => ArithmeticOperator::Divide,
            "idiv" => ArithmeticOperator::IntegerDivide,
            "-" => ArithmeticOperator::Subtract,
            "mod" => ArithmeticOperator::Modulo,
            _ => ArithmeticOperator::Noop,
        }
    }
}

#[derive(Clone)]
pub struct ArithmeticOperand<N: Node> {
    pub op: ArithmeticOperator,
    pub operand: Vec<Constructor<N>>,
}

/// A pattern is basically a Sequence Constructor in reverse.
/// An item is evaluated against the expression, and if the result is a non-empty sequence then the pattern has matched.
///
/// Converts a Sequence Constructor to a pattern, consuming the constructor. The Constructor must be a Path. The result Constructor is also a path, but it's steps are in reverse.
pub fn to_pattern<N: Node>(sc: Vec<Constructor<N>>) -> Result<Vec<Constructor<N>>, Error> {
    if sc.len() == 1 {
        match sc[0] {
            Constructor::Root => Ok(vec![Constructor::Step(
                NodeMatch {
                    axis: Axis::SelfDocument,
                    nodetest: NodeTest::Kind(KindTest::AnyKindTest),
                },
                vec![],
            )]),
            Constructor::Path(ref s) => {
                if s.is_empty() {
                    return Result::Err(Error {
                        kind: ErrorKind::TypeError,
                        message: "sequence constructor must not be empty".to_string(),
                    });
                }
                let mut p: Vec<Vec<Constructor<N>>> = Vec::new();
                let mut it = s.iter().rev();
                let step0 = it.next().unwrap(); // We've already checked that there is at least one step
                let mut last_axis;
                if step0.len() == 1 {
                    match step0[0] {
                        Constructor::Root => {
                            p.push(vec![Constructor::Step(
                                NodeMatch {
                                    axis: Axis::SelfDocument,
                                    nodetest: NodeTest::Kind(KindTest::AnyKindTest),
                                },
                                vec![],
                            )]);
                            last_axis = Axis::SelfDocument;
                        }
                        Constructor::Step(
                            NodeMatch {
                                axis: a,
                                nodetest: ref nt,
                            },
                            _,
                        ) => {
                            p.push(vec![Constructor::Step(
                                NodeMatch {
                                    axis: match a {
                                        Axis::Child | Axis::Selfaxis => Axis::Selfaxis,
                                        _ => a.opposite(),
                                    },
                                    nodetest: nt.clone(),
                                },
                                vec![],
                            )]);
                            last_axis = a.opposite();
                        }
                        _ => {
                            return Result::Err(Error {
                                kind: ErrorKind::TypeError,
                                message: "sequence constructor must be a step (1)".to_string(),
                            })
                        }
                    };
                } else {
                    return Result::Err(Error {
                        kind: ErrorKind::TypeError,
                        message: "sequence constructor must be steps".to_string(),
                    });
                }

                loop {
                    let n = it.next();
                    if n.is_none() {
                        break;
                    };
                    if n.unwrap().len() != 1 {
                        return Result::Err(Error {
                            kind: ErrorKind::TypeError,
                            message: "sequence constructor must be a step (2)".to_string(),
                        });
                    };

                    // TODO: predicates
                    match n.unwrap()[0] {
                        Constructor::Root => p.push(vec![Constructor::Step(
                            NodeMatch {
                                axis: Axis::ParentDocument,
                                nodetest: NodeTest::Kind(KindTest::AnyKindTest),
                            },
                            vec![],
                        )]),
                        Constructor::Step(
                            NodeMatch {
                                axis: _,
                                nodetest: ref nt,
                            },
                            _,
                        ) => p.push(vec![Constructor::Step(
                            NodeMatch {
                                axis: last_axis,
                                nodetest: nt.clone(),
                            },
                            vec![],
                        )]),
                        _ => {
                            return Result::Err(Error {
                                kind: ErrorKind::TypeError,
                                message: "sequence constructor must be a step (3)".to_string(),
                            })
                        }
                    }

                    last_axis = match n.unwrap()[0] {
                        Constructor::Step(NodeMatch { axis: a, .. }, _) => a.opposite(),
                        _ => Axis::Unknown,
                    }
                }
                Ok(vec![Constructor::Path(p)])
            }
            Constructor::Step(
                NodeMatch {
                    axis: a,
                    nodetest: ref nt,
                },
                _,
            ) => Ok(vec![Constructor::Step(
                NodeMatch {
                    axis: match a {
                        Axis::Child | Axis::Selfaxis => Axis::Selfaxis,
                        _ => a.opposite(),
                    },
                    nodetest: nt.clone(),
                },
                vec![],
            )]),
            _ => Result::Err(Error {
                kind: ErrorKind::TypeError,
                message: "sequence constructor must be a path".to_string(),
            }),
        }
    } else {
        Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: "sequence constructor must be a singleton".to_string(),
        })
    }
}

/// A template associating a pattern to a sequence constructor
#[derive(Clone)]
pub struct Template<N: Node> {
    pattern: Vec<Constructor<N>>,
    body: Vec<Constructor<N>>,
    priority: f64,
    mode: Option<String>,
    import: usize,
}

impl<N: Node> fmt::Debug for Template<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "match {} prio {}, import {}",
            format_constructor(&self.pattern, 0),
            self.priority,
            self.import
        )
    }
}

/// # Static context
///
/// Provide a static context and analysis for a [Sequence] [Constructor].
///
/// Currently, this stores the set of functions and variables available to a constructor.
pub struct StaticContext<N: Node> {
    pub funcs: RefCell<HashMap<String, Function<N>>>,
    pub vars: RefCell<HashMap<String, Vec<Sequence<N>>>>, // each entry in the vector is an inner scope of the variable
}

impl<N: Node> Default for StaticContext<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N: Node> StaticContext<N> {
    /// Creates a new StaticContext.
    pub fn new() -> Self {
        StaticContext {
            funcs: RefCell::new(HashMap::new()),
            vars: RefCell::new(HashMap::new()),
        }
    }
    /// Creates a new StaticContext and initializes it with the pre-defined XPath functions.
    ///
    /// Currently, this is the functions defined for XPath 1.0:
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
    /// These functions are defined for XPath 2.0:
    ///
    /// * current-dateTime()
    /// * current-date()
    /// * current-time()
    /// * format-dateTime()
    /// * format-date()
    /// * format-time()
    pub fn new_with_builtins() -> Self {
        let sc = StaticContext {
            funcs: RefCell::new(HashMap::new()),
            vars: RefCell::new(HashMap::new()),
        };
        sc.funcs.borrow_mut().insert(
            "position".to_string(),
            Function {
                name: "position".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_position),
            },
        );
        sc.funcs.borrow_mut().insert(
            "last".to_string(),
            Function {
                name: "last".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_last),
            },
        );
        sc.funcs.borrow_mut().insert(
            "count".to_string(),
            Function {
                name: "count".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_count),
            },
        );
        sc.funcs.borrow_mut().insert(
            "local-name".to_string(),
            Function {
                name: "local-name".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_localname),
            },
        );
        sc.funcs.borrow_mut().insert(
            "name".to_string(),
            Function {
                name: "name".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_name),
            },
        );
        sc.funcs.borrow_mut().insert(
            "string".to_string(),
            Function {
                name: "string".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_string),
            },
        );
        sc.funcs.borrow_mut().insert(
            "concat".to_string(),
            Function {
                name: "concat".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_concat),
            },
        );
        sc.funcs.borrow_mut().insert(
            "starts-with".to_string(),
            Function {
                name: "starts-with".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_startswith),
            },
        );
        sc.funcs.borrow_mut().insert(
            "contains".to_string(),
            Function {
                name: "contains".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_contains),
            },
        );
        sc.funcs.borrow_mut().insert(
            "substring".to_string(),
            Function {
                name: "substring".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_substring),
            },
        );
        sc.funcs.borrow_mut().insert(
            "substring-before".to_string(),
            Function {
                name: "substring-before".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_substringbefore),
            },
        );
        sc.funcs.borrow_mut().insert(
            "substring-after".to_string(),
            Function {
                name: "substring-after".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_substringafter),
            },
        );
        sc.funcs.borrow_mut().insert(
            "normalize-space".to_string(),
            Function {
                name: "normalize-space".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_normalizespace),
            },
        );
        sc.funcs.borrow_mut().insert(
            "translate".to_string(),
            Function {
                name: "translate".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_translate),
            },
        );
        sc.funcs.borrow_mut().insert(
            "boolean".to_string(),
            Function {
                name: "boolean".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_boolean),
            },
        );
        sc.funcs.borrow_mut().insert(
            "not".to_string(),
            Function {
                name: "not".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_not),
            },
        );
        sc.funcs.borrow_mut().insert(
            "true".to_string(),
            Function {
                name: "true".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_true),
            },
        );
        sc.funcs.borrow_mut().insert(
            "false".to_string(),
            Function {
                name: "false".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_false),
            },
        );
        sc.funcs.borrow_mut().insert(
            "number".to_string(),
            Function {
                name: "number".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_number),
            },
        );
        sc.funcs.borrow_mut().insert(
            "sum".to_string(),
            Function {
                name: "sum".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_sum),
            },
        );
        sc.funcs.borrow_mut().insert(
            "floor".to_string(),
            Function {
                name: "floor".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_floor),
            },
        );
        sc.funcs.borrow_mut().insert(
            "ceiling".to_string(),
            Function {
                name: "ceiling".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_ceiling),
            },
        );
        sc.funcs.borrow_mut().insert(
            "round".to_string(),
            Function {
                name: "round".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_round),
            },
        );
        sc.funcs.borrow_mut().insert(
            "current-dateTime".to_string(),
            Function {
                name: "current-dateTime".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_current_date_time),
            },
        );
        sc.funcs.borrow_mut().insert(
            "current-date".to_string(),
            Function {
                name: "current-date".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_current_date),
            },
        );
        sc.funcs.borrow_mut().insert(
            "current-time".to_string(),
            Function {
                name: "current-time".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_current_time),
            },
        );
        sc.funcs.borrow_mut().insert(
            "format-dateTime".to_string(),
            Function {
                name: "format-dateTime".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_format_date_time),
            },
        );
        sc.funcs.borrow_mut().insert(
            "format-date".to_string(),
            Function {
                name: "format-date".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_format_date),
            },
        );
        sc.funcs.borrow_mut().insert(
            "format-time".to_string(),
            Function {
                name: "format-time".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_format_time),
            },
        );

        sc
    }
    /// Create a new StaticContext with builtin functions defined,
    /// including additional functions defined by XSLT.
    pub fn new_with_xslt_builtins() -> Self {
        let sc = StaticContext::new_with_builtins();

        sc.funcs.borrow_mut().insert(
            "current-grouping-key".to_string(),
            Function {
                name: "current-grouping-key".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_current_grouping_key),
            },
        );
        sc.funcs.borrow_mut().insert(
            "current-group".to_string(),
            Function {
                name: "current-group".to_string(),
                nsuri: None,
                prefix: None,
                params: vec![],
                body: Some(func_current_group),
            },
        );

        sc
    }
    /// Register an extension function
    pub fn extension_function(&mut self, name: String, _ns: String, f: Function<N>) {
        // TODO: namespace
        self.funcs.borrow_mut().insert(name, f);
    }
    /// Declares a function in the static context. The first argument is the name of the function. The second argument is the namespace URI (not currently supported). The third argument defines the arity of the function, and the types of each parameter (not currently supported).
    pub fn declare_function(&self, n: String, _ns: String, p: Vec<Param>) {
        self.funcs.borrow_mut().insert(
            n.clone(),
            Function {
                name: n,
                nsuri: None,
                prefix: None,
                body: None,
                params: p,
            },
        );
    }
    /// Declares a variable in the static context. The first argument is the name of the variable. The second argument is the namespace URI (not currently supported).
    pub fn declare_variable(&self, n: String, _ns: String) {
        self.vars.borrow_mut().insert(n, vec![]);
    }

    /// Perform static analysis of a sequence constructor.
    ///
    /// This checks that functions and variables are declared. It also rewrites the constructors to provide the implementation of functions that are used in expressions.
    pub fn static_analysis(&mut self, e: &mut Vec<Constructor<N>>) {
        // TODO: return Result
        // TODO: iterate through the tree structure instead of doing a recursive depth first search. This should mean that the method would not have to use interior mutability
        for d in e {
            match d {
                Constructor::Switch(v, o) => {
                    for i in v {
                        self.static_analysis(i)
                    }
                    self.static_analysis(o);
                }
                Constructor::Loop(v, a) => {
                    self.static_analysis(v);
                    self.static_analysis(a);
                }
                Constructor::SetAttribute(_, v) => {
                    self.static_analysis(v);
                }
                Constructor::FunctionCall(f, a) => {
                    // Fill in function body
                    match self.funcs.borrow().get(&f.name) {
                        Some(g) => {
                            f.body.replace(g.body.unwrap());
                        }
                        None => {
                            // TODO: Don't Panic
                            panic!("call to unknown function \"{}\"", f.name)
                        }
                    }
                    for i in a {
                        self.static_analysis(i)
                    }
                }
                Constructor::VariableDeclaration(v, a) => {
                    self.declare_variable(v.to_string(), "".to_string());
                    self.static_analysis(a)
                }
                Constructor::VariableReference(_v) => {
                    // TODO: check that variable has been declared
                }
                Constructor::Or(a)
                | Constructor::And(a)
                | Constructor::Path(a)
                | Constructor::Concat(a)
                | Constructor::Range(a) => {
                    for i in a {
                        self.static_analysis(i)
                    }
                }
                Constructor::Step(_, a) => {
                    for i in a {
                        self.static_analysis(i)
                    }
                }
                Constructor::GeneralComparison(_, a) | Constructor::ValueComparison(_, a) => {
                    for i in a {
                        self.static_analysis(i)
                    }
                }
                Constructor::Arithmetic(a) => {
                    for i in a {
                        self.static_analysis(&mut i.operand)
                    }
                }
                Constructor::ApplyTemplates(s) => self.static_analysis(s),
                Constructor::ForEach(s, t, _g) => {
                    self.static_analysis(s);
                    self.static_analysis(t);
                }
                Constructor::Copy(_, c) | Constructor::LiteralElement(_, c) => {
                    self.static_analysis(c)
                }
                Constructor::DeepCopy(c) => {
                    self.static_analysis(c);
                }
                Constructor::Literal(_)
                | Constructor::LiteralAttribute(_, _)
                | Constructor::ContextItem
                | Constructor::Root
                | Constructor::ApplyImports
                | Constructor::NotImplemented(_) => {}
            }
        }
    }
}

// Functions

pub type FunctionImpl<N> = fn(
    &Evaluator<N>,
    Option<Sequence<N>>, // Context
    Option<usize>,       // Context position
    Vec<Sequence<N>>,    // Actual parameters
    &N,
) -> Result<Sequence<N>, Error>;

#[derive(Clone)]
pub struct Function<N: Node> {
    name: String,
    nsuri: Option<String>,
    prefix: Option<String>,
    params: Vec<Param>, // The number of parameters in the vector is the arity of the function
    body: Option<FunctionImpl<N>>, // Function implementation must be provided during static analysis
}

impl<N: Node> Function<N> {
    pub fn new(n: String, p: Vec<Param>, i: Option<FunctionImpl<N>>) -> Self {
        Function {
            name: n,
            nsuri: None,
            prefix: None,
            params: p,
            body: i,
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_nsuri(&self) -> Option<String> {
        self.nsuri.clone()
    }
    pub fn get_prefix(&self) -> Option<String> {
        self.prefix.clone()
    }
    // TODO: make this an iterator over the formal parameters
    pub fn get_params(&self) -> Vec<Param> {
        self.params.clone()
    }
    pub fn get_body(&self) -> Option<FunctionImpl<N>> {
        self.body
    }
}

// A formal parameter
#[derive(Clone)]
pub struct Param {
    name: String,
    datatype: String, // TODO
}

impl Param {
    pub fn new(n: String, t: String) -> Param {
        Param {
            name: n,
            datatype: t,
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_datatype(&self) -> String {
        self.datatype.clone()
    }
}

pub fn func_position<N: Node>(
    _: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    posn: Option<usize>,
    _args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    match posn {
        Some(u) => Ok(vec![Rc::new(Item::Value(Value::Integer(u as i64 + 1)))]),
        None => Result::Err(Error {
            kind: ErrorKind::DynamicAbsent,
            message: String::from("no context item"),
        }),
    }
}

pub fn func_last<N: Node>(
    _: &Evaluator<N>,
    ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    _args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    match ctxt {
        Some(u) => Ok(vec![Rc::new(Item::Value(Value::Integer(u.len() as i64)))]),
        None => Result::Err(Error {
            kind: ErrorKind::DynamicAbsent,
            message: String::from("no context item"),
        }),
    }
}

pub fn func_count<N: Node>(
    _: &Evaluator<N>,
    ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    match args.len() {
        0 => {
            // count the context items
            match ctxt {
                Some(u) => Ok(vec![Rc::new(Item::Value(Value::Integer(u.len() as i64)))]),
                None => Result::Err(Error {
                    kind: ErrorKind::DynamicAbsent,
                    message: String::from("no context item"),
                }),
            }
        }
        1 => {
            // count the argument items
            Ok(vec![Rc::new(Item::Value(Value::Integer(
                args[0].len() as i64
            )))])
        }
        _ => Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        }),
    }
}

pub fn func_localname<N: Node>(
    _: &Evaluator<N>,
    ctxt: Option<Sequence<N>>,
    posn: Option<usize>,
    _args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    match ctxt {
        Some(u) => {
            // Current item must be a node
            match *u[posn.unwrap()] {
                Item::Node(ref n) => Ok(vec![Rc::new(Item::Value(Value::String(
                    n.name().get_localname(),
                )))]),
                _ => Result::Err(Error {
                    kind: ErrorKind::TypeError,
                    message: String::from("not a node"),
                }),
            }
        }
        None => Result::Err(Error {
            kind: ErrorKind::DynamicAbsent,
            message: String::from("no context item"),
        }),
    }
}

// TODO: handle qualified names
pub fn func_name<N: Node>(
    _e: &Evaluator<N>,
    ctxt: Option<Sequence<N>>,
    posn: Option<usize>,
    _args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    match ctxt {
        Some(u) => {
            // Current item must be a node
            match *u[posn.unwrap()] {
                Item::Node(ref n) => {
                    // TODO: handle QName prefixes
                    Ok(vec![Rc::new(Item::Value(Value::String(
                        n.name().get_localname(),
                    )))])
                }
                _ => Result::Err(Error {
                    kind: ErrorKind::TypeError,
                    message: String::from("not a node"),
                }),
            }
        }
        None => Result::Err(Error {
            kind: ErrorKind::DynamicAbsent,
            message: String::from("no context item"),
        }),
    }
}

// TODO: implement string value properly
pub fn func_string<N: Node>(
    _e: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    match args.len() {
        1 => {
            // return string value
            Ok(vec![Rc::new(Item::Value(Value::String(
                args[0].to_string(),
            )))])
        }
        _ => Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        }),
    }
}

pub fn func_concat<N: Node>(
    _e: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    Ok(vec![Rc::new(Item::Value(Value::String(args.iter().fold(
        String::new(),
        |mut a, b| {
            a.push_str(b.to_string().as_str());
            a
        },
    ))))])
}

pub fn func_startswith<N: Node>(
    _e: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have exactly 2 arguments
    if args.len() == 2 {
        // arg[0] is the string to search
        // arg[1] is what to search for
        Ok(vec![Rc::new(Item::Value(Value::Boolean(
            args[0]
                .to_string()
                .starts_with(args[1].to_string().as_str()),
        )))])
    } else {
        Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        })
    }
}

pub fn func_contains<N: Node>(
    _e: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have exactly 2 arguments
    if args.len() == 2 {
        // arg[0] is the string to search
        // arg[1] is what to search for
        Ok(vec![Rc::new(Item::Value(Value::Boolean(
            args[0].to_string().contains(args[1].to_string().as_str()),
        )))])
    } else {
        Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        })
    }
}

pub fn func_substring<N: Node>(
    _e: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have 2 or 3 arguments
    match args.len() {
        2 => {
            // arg[0] is the string to search
            // arg[1] is the index to start at
            // 2-argument version takes the rest of the string
            Ok(vec![Rc::new(Item::Value(Value::String(
                args[0]
                    .to_string()
                    .graphemes(true)
                    .skip(args[1].to_int()? as usize - 1)
                    .collect(),
            )))])
        }
        3 => {
            // arg[0] is the string to search
            // arg[1] is the index to start at
            // arg[2] is the length of the substring to extract
            Ok(vec![Rc::new(Item::Value(Value::String(
                args[0]
                    .to_string()
                    .graphemes(true)
                    .skip(args[1].to_int()? as usize - 1)
                    .take(args[2].to_int()? as usize)
                    .collect(),
            )))])
        }
        _ => Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        }),
    }
}

pub fn func_substringbefore<N: Node>(
    _e: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have 2 arguments
    match args.len() {
        2 => {
            // arg[0] is the string to search
            // arg[1] is the string to find
            match args[0].to_string().find(args[1].to_string().as_str()) {
                Some(i) => {
                    match args[0].to_string().get(0..i) {
                        Some(s) => Ok(vec![Rc::new(Item::Value(Value::String(String::from(s))))]),
                        None => {
                            // This shouldn't happen!
                            Result::Err(Error {
                                kind: ErrorKind::Unknown,
                                message: String::from("unable to extract substring"),
                            })
                        }
                    }
                }
                None => Ok(vec![]),
            }
        }
        _ => Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        }),
    }
}

pub fn func_substringafter<N: Node>(
    _e: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have 2 arguments
    match args.len() {
        2 => {
            // arg[0] is the string to search
            // arg[1] is the string to find
            match args[0].to_string().find(args[1].to_string().as_str()) {
                Some(i) => {
                    match args[0]
                        .to_string()
                        .get(i + args[1].to_string().len()..args[0].to_string().len())
                    {
                        Some(s) => Ok(vec![Rc::new(Item::Value(Value::String(String::from(s))))]),
                        None => {
                            // This shouldn't happen!
                            Result::Err(Error {
                                kind: ErrorKind::Unknown,
                                message: String::from("unable to extract substring"),
                            })
                        }
                    }
                }
                None => Ok(vec![]),
            }
        }
        _ => Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        }),
    }
}

pub fn func_normalizespace<N: Node>(
    _e: &Evaluator<N>,
    ctxt: Option<Sequence<N>>,
    posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have 0 or 1 arguments
    let s: Result<Option<String>, Error> = match args.len() {
        0 => {
            // Use the current item
            match ctxt {
                Some(c) => Ok(Some(c[posn.unwrap()].to_string())),
                None => Ok(None),
            }
        }
        1 => Ok(Some(args[0].to_string())),
        _ => Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        }),
    };

    match s {
        Ok(u) => match u {
            Some(t) => Ok(vec![Rc::new(Item::Value(Value::String(
                t.split_whitespace().collect(),
            )))]),
            None => Result::Err(Error {
                kind: ErrorKind::DynamicAbsent,
                message: String::from("no context item"),
            }),
        },
        Result::Err(e) => Result::Err(e),
    }
}

pub fn func_translate<N: Node>(
    _e: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have 3 arguments
    match args.len() {
        3 => {
            // arg[0] is the string to search
            // arg[1] is the map chars
            // arg[2] is the translate chars
            let o = args[1].to_string();
            let m: Vec<&str> = o.graphemes(true).collect();
            let u = args[2].to_string();
            let t: Vec<&str> = u.graphemes(true).collect();
            let mut result: String = String::new();

            for c in args[0].to_string().graphemes(true) {
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
                /*
                for i in 0..m.len() {
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
                 */
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
            Ok(vec![Rc::new(Item::Value(Value::String(result)))])
        }
        _ => Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        }),
    }
}

pub fn func_boolean<N: Node>(
    _: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have 1 arguments
    match args.len() {
        1 => Ok(vec![Rc::new(Item::Value(Value::Boolean(
            args[0].to_bool(),
        )))]),
        _ => Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        }),
    }
}

pub fn func_not<N: Node>(
    _: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have 1 arguments
    match args.len() {
        1 => Ok(vec![Rc::new(Item::Value(Value::Boolean(
            !args[0].to_bool(),
        )))]),
        _ => Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        }),
    }
}

pub fn func_true<N: Node>(
    _: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have 0 arguments
    match args.len() {
        0 => Ok(vec![Rc::new(Item::Value(Value::Boolean(true)))]),
        _ => Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        }),
    }
}

pub fn func_false<N: Node>(
    _: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have 0 arguments
    match args.len() {
        0 => Ok(vec![Rc::new(Item::Value(Value::Boolean(false)))]),
        _ => Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        }),
    }
}

pub fn func_number<N: Node>(
    _: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have 1 argument
    match args.len() {
        1 => {
            match args[0].len() {
                1 => {
                    // TODO: if the item is already an integer, then just clone it
                    // First try converting to an integer
                    match args[0][0].to_int() {
                        Ok(i) => Ok(vec![Rc::new(Item::Value(Value::Integer(i)))]),
                        Result::Err(_) => {
                            // If that fails, convert to double
                            // NB. this can't fail. At worst it returns NaN
                            Ok(vec![Rc::new(Item::Value(Value::Double(
                                args[0][0].to_double(),
                            )))])
                        }
                    }
                }
                _ => Result::Err(Error {
                    kind: ErrorKind::TypeError,
                    message: String::from("not a singleton sequence"),
                }),
            }
        }
        _ => Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        }),
    }
}

pub fn func_sum<N: Node>(
    _: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have 1 argument
    match args.len() {
        1 => Ok(vec![Rc::new(Item::Value(Value::Double(
            args[0].iter().fold(0.0, |mut acc, i| {
                acc += i.to_double();
                acc
            }),
        )))]),
        _ => Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        }),
    }
}

pub fn func_floor<N: Node>(
    _: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have 1 argument which is a singleton
    match args.len() {
        1 => match args[0].len() {
            1 => Ok(vec![Rc::new(Item::Value(Value::Double(
                args[0][0].to_double().floor(),
            )))]),
            _ => Result::Err(Error {
                kind: ErrorKind::TypeError,
                message: String::from("not a singleton sequence"),
            }),
        },
        _ => Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        }),
    }
}

pub fn func_ceiling<N: Node>(
    _: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have 1 argument which is a singleton
    match args.len() {
        1 => match args[0].len() {
            1 => Ok(vec![Rc::new(Item::Value(Value::Double(
                args[0][0].to_double().ceil(),
            )))]),
            _ => Result::Err(Error {
                kind: ErrorKind::TypeError,
                message: String::from("not a singleton sequence"),
            }),
        },
        _ => Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        }),
    }
}

pub fn func_round<N: Node>(
    _: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have 1 or 2 arguments
    match args.len() {
        1 => {
            // precision is 0 (i.e. round to nearest whole number
            match args[0].len() {
                1 => Ok(vec![Rc::new(Item::Value(Value::Double(
                    args[0][0].to_double().round(),
                )))]),
                _ => Result::Err(Error {
                    kind: ErrorKind::TypeError,
                    message: String::from("not a singleton sequence"),
                }),
            }
        }
        2 => match (args[0].len(), args[1].len()) {
            (1, 1) => Ok(vec![Rc::new(Item::Value(Value::Double(
                args[0][0]
                    .to_double()
                    .powi(args[1][0].to_int().unwrap() as i32)
                    .round()
                    .powi(-(args[1][0].to_int().unwrap() as i32)),
            )))]),
            _ => Result::Err(Error {
                kind: ErrorKind::TypeError,
                message: String::from("not a singleton sequence"),
            }),
        },
        _ => Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        }),
    }
}

pub fn func_current_date_time<N: Node>(
    _: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    _args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have 0 arguments
    // TODO: check number of arguments
    // TODO: do the check in static analysis phase

    Ok(vec![Rc::new(Item::Value(Value::DateTime(Local::now())))])
}

pub fn func_current_date<N: Node>(
    _: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    _args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have 0 arguments
    // TODO: check number of arguments
    // TODO: do the check in static analysis phase

    Ok(vec![Rc::new(Item::Value(Value::Date(
        Utc::now().date_naive(),
    )))])
}

pub fn func_current_time<N: Node>(
    _: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    _args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have 0 arguments
    // TODO: check number of arguments
    // TODO: do the check in static analysis phase

    Ok(vec![Rc::new(Item::Value(Value::Time(Local::now())))])
}

pub fn func_format_date_time<N: Node>(
    _e: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have 2 or 5 arguments
    // TODO: implement 5 argument version

    match args.len() {
        2 => {
            // First argument is the dateTime value
            // Second argument is the picture
            let pic = match picture_parse(&args[1].to_string()) {
                Ok(p) => p,
                Err(_) => {
                    return Result::Err(Error {
                        kind: ErrorKind::Unknown,
                        message: String::from("bad picture"),
                    })
                }
            };

            match args[0].len() {
                0 => Ok(vec![]), // Empty value returns empty sequence
                1 => {
                    match *args[0][0] {
                        Item::Value(Value::DateTime(dt)) => Ok(vec![Rc::new(Item::Value(
                            Value::String(dt.format(&pic).to_string()),
                        ))]),
                        Item::Value(Value::String(ref s)) => {
                            // Try and coerce into a DateTime value
                            match DateTime::<FixedOffset>::parse_from_rfc3339(s.as_str()) {
                                Ok(dt) => Ok(vec![Rc::new(Item::Value(Value::String(
                                    dt.format(&pic).to_string(),
                                )))]),
                                Err(_) => Result::Err(Error {
                                    kind: ErrorKind::TypeError,
                                    message: String::from("unable to determine date value"),
                                }),
                            }
                        }
                        _ => Result::Err(Error {
                            kind: ErrorKind::TypeError,
                            message: String::from("not a dateTime value"),
                        }),
                    }
                }
                _ => Result::Err(Error {
                    kind: ErrorKind::TypeError,
                    message: String::from("not a singleton sequence"),
                }),
            }
        }
        5 => Result::Err(Error {
            kind: ErrorKind::NotImplemented,
            message: String::from("not yet implemented"),
        }),
        _ => Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        }),
    }
}

pub fn func_format_date<N: Node>(
    _e: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have 2 or 5 arguments
    // TODO: implement 5 argument version

    match args.len() {
        2 => {
            // First argument is the date value
            // Second argument is the picture
            let pic = match picture_parse(&args[1].to_string()) {
                Ok(p) => p,
                Err(_) => {
                    return Result::Err(Error {
                        kind: ErrorKind::Unknown,
                        message: String::from("bad picture"),
                    })
                }
            };
            match args[0].len() {
                0 => Ok(vec![]), // Empty value returns empty sequence
                1 => {
                    match *args[0][0] {
                        Item::Value(Value::Date(dt)) => Ok(vec![Rc::new(Item::Value(
                            Value::String(dt.format(&pic).to_string()),
                        ))]),
                        Item::Value(Value::String(ref s)) => {
                            // Try and coerce into a Date value
                            let a = format!("{}T00:00:00Z", s);
                            match DateTime::<FixedOffset>::parse_from_rfc3339(a.as_str()) {
                                Ok(dt) => Ok(vec![Rc::new(Item::Value(Value::String(
                                    dt.date_naive().format(&pic).to_string(),
                                )))]),
                                Err(_) => Result::Err(Error {
                                    kind: ErrorKind::TypeError,
                                    message: String::from("unable to determine date value"),
                                }),
                            }
                        }
                        _ => Result::Err(Error {
                            kind: ErrorKind::TypeError,
                            message: String::from("not a date value"),
                        }),
                    }
                }
                _ => Result::Err(Error {
                    kind: ErrorKind::TypeError,
                    message: String::from("not a singleton sequence"),
                }),
            }
        }
        5 => Result::Err(Error {
            kind: ErrorKind::NotImplemented,
            message: String::from("not yet implemented"),
        }),
        _ => Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        }),
    }
}

pub fn func_format_time<N: Node>(
    _e: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    // must have 2 or 5 arguments
    // TODO: implement 5 argument version

    match args.len() {
        2 => {
            // First argument is the time value
            // Second argument is the picture
            let pic = match picture_parse(&args[1].to_string()) {
                Ok(p) => p,
                Err(_) => {
                    return Result::Err(Error {
                        kind: ErrorKind::Unknown,
                        message: String::from("bad picture"),
                    })
                }
            };
            match args[0].len() {
                0 => Ok(vec![]), // Empty value returns empty sequence
                1 => {
                    match *args[0][0] {
                        Item::Value(Value::Time(dt)) => Ok(vec![Rc::new(Item::Value(
                            Value::String(dt.format(&pic).to_string()),
                        ))]),
                        Item::Value(Value::String(ref s)) => {
                            // Try and coerce into a DateTime value
                            let a = format!("1900-01-01T{}Z", s);
                            match DateTime::<FixedOffset>::parse_from_rfc3339(a.as_str()) {
                                Ok(dt) => Ok(vec![Rc::new(Item::Value(Value::String(
                                    dt.time().format(&pic).to_string(),
                                )))]),
                                Err(_) => Result::Err(Error {
                                    kind: ErrorKind::TypeError,
                                    message: String::from("unable to determine time value"),
                                }),
                            }
                        }
                        _ => Result::Err(Error {
                            kind: ErrorKind::TypeError,
                            message: String::from("not a time value"),
                        }),
                    }
                }
                _ => Result::Err(Error {
                    kind: ErrorKind::TypeError,
                    message: String::from("not a singleton sequence"),
                }),
            }
        }
        5 => Result::Err(Error {
            kind: ErrorKind::NotImplemented,
            message: String::from("not yet implemented"),
        }),
        _ => Result::Err(Error {
            kind: ErrorKind::TypeError,
            message: String::from("wrong number of arguments"),
        }),
    }
}

pub fn func_current_grouping_key<N: Node>(
    e: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    _args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    match e.dc.current_grouping_key.borrow().last() {
        Some(k) => match k {
            Some(l) => Ok(vec![l.clone()]),
            None => Ok(vec![]),
        },
        None => Result::Err(Error {
            kind: ErrorKind::DynamicAbsent,
            message: String::from("no current grouping key"),
        }),
    }
}

pub fn func_current_group<N: Node>(
    e: &Evaluator<N>,
    _ctxt: Option<Sequence<N>>,
    _posn: Option<usize>,
    _args: Vec<Sequence<N>>,
    _rd: &N,
) -> Result<Sequence<N>, Error> {
    match e.dc.current_group.borrow().last() {
        Some(k) => match k {
            Some(l) => Ok(l.clone()),
            None => Ok(vec![]),
        },
        None => Result::Err(Error {
            kind: ErrorKind::DynamicAbsent,
            message: String::from("no current group"),
        }),
    }
}

pub fn format_constructor<N: Node>(c: &Vec<Constructor<N>>, i: usize) -> String {
    let mut result = String::new();
    for v in c {
        result.push_str(", ");
        let t = match v {
            Constructor::Literal(l) => {
                format!("{:in$} Construct literal \"{}\"", "", l, in=i)
            }
            Constructor::LiteralAttribute(qn, v) => {
                format!("{:in$} Construct literal attribute \"{}\" with value \"{}\"", "",
	  qn.get_localname(),
	  format_constructor(v, i + 4),
	  in=i)
            }
            Constructor::LiteralElement(qn, c) => {
                format!("{:in$} Construct literal element \"{}\" with content:\n{}", "", qn.get_localname(),
	  format_constructor(c, i + 4),
	  in=i)
            }
            Constructor::Copy(_sel, c) => {
                format!("{:in$} Construct copy with content:\n{}", "",
	  format_constructor(c, i + 4),
	  in=i)
            }
            Constructor::DeepCopy(c) => {
                format!("{:in$} Construct deep copy with content:\n{}", "",
	  format_constructor(c, i + 4),
	  in=i)
            }
            Constructor::ContextItem => {
                format!("{:in$} Construct context item", "", in=i)
            }
            Constructor::SetAttribute(qn, v) => {
                format!("{:in$} Construct set attribute named \"{}\":\n{}", "",
	  qn.get_localname(),
	  format_constructor(v, i + 4),
	  in=i)
            }
            Constructor::Or(v) => {
                format!(
                  "{:in$} Construct OR of:\n{}\n{}", "",
                  format_constructor(&v[0], i + 4),
                  format_constructor(&v[1], i + 4),
                  in=i,
                )
            }
            Constructor::And(v) => {
                format!(
                  "{:in$} Construct AND of:\n{}\n{}", "",
                  format_constructor(&v[0], i + 4),
                  format_constructor(&v[1], i + 4),
                  in=i,
                )
            }
            Constructor::Root => {
                format!("{:in$} Construct document root", "", in=i)
            }
            Constructor::Step(nm, p) => {
                format!(
                  "{:in$} Construct step {}{}", "",
                  nm,
                  if !p.is_empty() {format!("\npredicates: {}", format_constructor(&p[0], 0))} else {"".to_string()},
                  in=i
                )
            }
            Constructor::Path(v) => {
                let mut s = format!("{:in$} Construct relative path:\n", "", in=i);
                for u in v {
                    s.push_str(&format_constructor(u, i + 4))
                }
                s
            }
            Constructor::GeneralComparison(_o, _v) => {
                format!("{:in$} general comparison constructor", "", in=i)
            }
            Constructor::ValueComparison(o, v) => {
                format!("{:in$} value comparison constructor {} of:\n{}\n{}", "",
	o.to_string(),
	format_constructor(&v[0], i + 4),
	format_constructor(&v[1], i + 4),
	in=i)
            }
            Constructor::Concat(_v) => {
                format!("{:in$} concat constructor", "", in=i)
            }
            Constructor::Range(_v) => {
                format!("{:in$} range constructor", "", in=i)
            }
            Constructor::Arithmetic(_v) => {
                format!("{:in$} arithmetic constructor", "", in=i)
            }
            Constructor::FunctionCall(f, a) => {
                format!("{:in$} function call to \"{}\" ({}) with {} arguments", "",
	  f.name,
	  f.body.map_or_else(|| "not defined", |_| "is defined"),
	  a.len(),
	  in=i)
            }
            Constructor::VariableDeclaration(v, _) => {
                format!("{:in$} variable declaration constructor named \"{}\"", "", v, in=i)
            }
            Constructor::VariableReference(v) => {
                format!("{:in$} variable reference constructor named \"{}\"", "", v, in=i)
            }
            Constructor::Loop(_, _) => {
                format!("{:in$} loop constructor", "", in=i)
            }
            Constructor::Switch(_, _) => {
                format!("{:in$} switch constructor", "", in=i)
            }
            Constructor::ApplyTemplates(_) => {
                format!("{:in$} apply-templates constructor", "", in=i)
            }
            Constructor::ApplyImports => {
                format!("{:in$} apply-imports constructor", "", in=i)
            }
            Constructor::ForEach(_, _, _) => {
                format!("{:in$} for-each constructor", "", in=i)
            }
            Constructor::NotImplemented(m) => {
                format!("{:in$} NotImplemented constructor: {}", "", m, in=i)
            }
        };
        result.push_str(&t);
    }
    result
}
