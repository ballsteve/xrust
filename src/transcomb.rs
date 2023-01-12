//! # Transformation Combinator

use std::rc::Rc;
use std::collections::HashMap;

use crate::item::{Item, Node, Sequence, SequenceTrait};
use crate::qname::QualifiedName;
use crate::value::Value;
use crate::evaluate::{Axis, NodeMatch, is_node_match};
use crate::xdmerror::*;

pub(crate) type TransResult<N> = Result<Sequence<N>, Error>;

/// The transformation context (i.e. the dynamic context, plus some parts of the static context)
// Idea: instead of having one dynamic context that is mutable,
// make the context immutable but with shared components. Then when a new context is required, clone it and add in extra components
#[derive(Clone)]
pub struct Context<N: Node> {
    seq: Sequence<N>,	// The current context
    i: usize,		// Which item in the sequence is the current context
    depth: usize,	// Depth of evaluation
    rd: Option<N>,	// Result document
    // templates
    // built-in templates
    // variables
    vars: HashMap<String, Vec<Sequence<N>>>,
    // grouping
    // import level
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
	    rd: None,
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
    fn set_result_document(&mut self, rd: N) {
	self.rd = Some(rd);
    }
}

impl<N: Node> From<Sequence<N>> for Context<N> {
    fn from(seq: Sequence<N>) -> Self {
	Context {
	    seq,
	    i: 0,
	    depth: 0,
	    vars: HashMap::new(),
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
    pub fn build(self) -> Context<N> {
	self.0
    }
}

/// Creates a singleton sequence with the given value
pub fn literal<N: Node + 'static>(val: Rc<Item<N>>) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
{
    Box::new(move |_ctxt| Ok(vec![val.clone()]))
}

/// Creates a singleton sequence with a new element node. The function is evaluated to create the content of the element.
pub fn literal_element<F, N: Node + 'static>(qn: QualifiedName, c: F) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static
{
    Box::new(move |ctxt| {
	if ctxt.rd.is_none() {
	    return Err(Error::new(ErrorKind::Unknown, String::from("context has no result document")))
	}
	let r = ctxt.rd.clone().unwrap();

	let mut e = r.new_element(qn.clone())?;
	c(ctxt)?.iter()
	    .try_for_each(|i| {
		// Item could be a Node or text
		match &**i {
		    Item::Node(t) => e.push(t.clone()),
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

/// Creates a singleton sequence with the context item as its value
pub fn context<N: Node>() -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
{
    Box::new(move |ctxt| Ok(vec![ctxt.seq[ctxt.i].clone()]))
}

/// Creates a sequence. Each function in the supplied vector creates an item in the sequence. The original context is passed to each function.
pub fn tc_sequence<F, N: Node>(items: Vec<F>) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static
{
    Box::new(move |ctxt| {
	match items.iter()
	    .try_fold(
		vec![],
		|mut acc, f| {
		    match f(ctxt) {
			Ok(mut s) => {
			    acc.append(&mut s);
			    Ok(acc)
			}
			Err(err) => Err(err),
		    }
		}
	    ) {
		Ok(r) => Ok(r),
		Err(err) => Err(err)
	    }
    })
}

/// Each function in the supplied vector is evaluated. The sequence returned by a function is used as the context for the next function.
pub fn compose<F, N: Node>(steps: Vec<F>) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static
{
    Box::new(move |ctxt| {
	match steps.iter()
	    .try_fold(
		ctxt.seq.clone(),
		|_, f| {
		    match f(ctxt) {
			Ok(s) => {
			    ctxt.seq = s.clone();
			    Ok(s)
			}
			Err(err) => Err(err),
		    }
		}
	    ) {
		Ok(r) => Ok(r),
		Err(err) => Err(err)
	    }
    })
}

/// For each item in the current context, evaluate the given node matching operation.
pub fn step<N: Node>(nm: NodeMatch) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>> {
    Box::new(move |ctxt| {
	match ctxt.seq.iter()
	    .try_fold(
		vec![],
		|mut acc, i| {
		    match &**i {
			Item::Node(n) => {
			    match nm.axis {
				Axis::Child => {
				    let mut s = n.child_iter()
					.filter(|c| is_node_match::<N>(&nm.nodetest, c))
					.fold(
					    Sequence::new(),
					    |mut c, a| {
						c.push_node(a.clone());
						c
					    }
					);
				    // TODO: predicates
				    acc.append(&mut s);
				    Ok(acc)
				}
				_ => Err(Error::new(ErrorKind::NotImplemented, String::from("coming soon")))
			    }
			}
			_ => Err(Error::new(ErrorKind::Unknown, String::from("context item is not a node")))
		    }
		}
	    ) {
		Ok(r) => Ok(r),
		Err(err) => Err(err)
	    }
    })
}

/// Remove items that don't match the predicate.
pub fn filter<F, N: Node>(predicate: F) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static
{
    Box::new(move |ctxt| {
	match ctxt.seq.iter()
	    .try_fold(
		vec![],
		|mut acc, i| {
		    let s = match predicate(&mut Context::from(vec![i.clone()])) {
			Ok(t) => t,
			Err(err) => return Err(err),
		    };
		    if s.to_bool() == true {
			acc.push(i.clone())
		    }
		    Ok(acc)
		}
	    ) {
		Ok(r) => Ok(r),
		Err(err) => Err(err)
	    }
    })
}

/// Declare a variable in scope for a function. Returns the result of the function.
pub fn declare_variable<F, N: Node>(name: String, value: F, f: F) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static
{
    Box::new(move |ctxt| {
	match value(ctxt) {
	    Ok(s) => {
		ctxt.var_push(name.clone(), s);
		let r = f(ctxt);
		ctxt.var_pop(name.clone());
		r
	    }
	    Err(err) => Err(err)
	}
    })
}
pub fn reference_variable<N: Node>(name: String) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
{
    Box::new(move |ctxt| {
	match ctxt.vars.get(name.as_str()) {
	    Some(u) => {
		match u.last() {
		    Some(t) => Ok(t.clone()),
		    None => Err(Error::new(ErrorKind::Unknown, format!("variable \"{}\" is no longer in scope", name)))
		}
	    }
	    None => Err(Error::new(ErrorKind::Unknown, format!("unknown variable \"{}\"", name)))
	}
    })
}

/// XPath concat function. All arguments are concatenated into a single string value.
pub fn function_concat<F, N: Node>(arguments: Vec<F>) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static
{
    Box::new(move |ctxt| {
	match arguments.iter()
	    .try_fold(
		String::new(),
		|mut acc, a| {
		    match a(ctxt) {
			Ok(b) => {
			    acc.push_str(b.to_string().as_str());
			    Ok(acc)
			}
			Err(err) => Err(err)
		    }
		}
	    ) {
		Ok(r) => Ok(vec![Rc::new(Item::Value(Value::from(r)))]),
		Err(err) => Err(err)
	    }
    })
}

/// A user defined function. Each argument is declared as a variable in the [Context]. The body of the function is then evaluated and it's result is returned.
pub fn function_user_defined<F, N: Node>(body: F, arguments: Vec<(String, F)>) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static
{
    Box::new(move |ctxt| {
	arguments.iter()
	    .try_for_each(|(n, a)| {
		match a(ctxt) {
		    Ok(b) => {
			ctxt.var_push(n.clone(), b);
			Ok(())
		    }
		    Err(err) => Err(err),
		}
	    })?;
	let result = body(ctxt);
	arguments.iter()
	    .for_each(|(n, _)| ctxt.var_pop(n.clone()));
	result
    })
}
