//! # Transformation Combinator

use std::rc::Rc;
use std::collections::HashMap;

use crate::item::{Item, Node, NodeType, Sequence, SequenceTrait};
use crate::qname::*;
use crate::value::{Operator, Value};
use crate::evaluate::{Axis, NodeMatch, NodeTest, KindTest, is_node_match};
use crate::xdmerror::*;
use crate::intmuttree::{RNode, NodeBuilder};

pub(crate) type TransResult<N> = Result<Sequence<N>, Error>;

/// The transformation context (i.e. the dynamic context, plus some parts of the static context)
// Idea: instead of having one dynamic context that is mutable,
// make the context immutable but with shared components. Then when a new context is required, clone it and add in extra components
#[derive(Clone)]
pub struct Context<N: Node> {
    seq: Sequence<N>,	// The current context
    i: usize,		// Which item in the sequence is the current context
    depth: usize,	// Depth of evaluation
    // templates
    // built-in templates
    // variables
    vars: HashMap<String, Sequence<N>>,
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
	}
    }
}

impl<N: Node> From<Sequence<N>> for Context<N> {
    fn from(seq: Sequence<N>) -> Self {
	Context {
	    seq,
	    i: 0,
	    depth: 0,
	    vars: HashMap::new(),
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
    pub fn variables(mut self, v: HashMap<String, Sequence<N>>) -> Self {
	self.0.vars = v;
	self
    }
    pub fn build(self) -> Context<N> {
	self.0
    }
}

/// Creates a singleton sequence with the given value
pub fn literal<N: Node + 'static>(val: Rc<Item<N>>) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
{
    Box::new(move |ctxt| Ok(vec![val.clone()]))
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

/// A user defined function. Each argument is declared as a variable in a new [Context]. The body of the function is then evaluated and it's result is returned.
pub fn function_user_defined<F, N: Node>(body: F, arguments: Vec<F>) -> Box<dyn Fn(&mut Context<N>) -> TransResult<N>>
where
    F: Fn(&mut Context<N>) -> TransResult<N> + 'static
{
    Box::new(move |ctxt| {
	Err(Error::new(ErrorKind::NotImplemented, String::from("not yet implemented")))
    })
}
