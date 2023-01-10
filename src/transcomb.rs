//! # Transformation Combinator

use std::rc::Rc;

use crate::item::{Item, Node, NodeType, Sequence, SequenceTrait};
use crate::qname::*;
use crate::value::{Operator, Value};
use crate::evaluate::{Axis, NodeMatch, NodeTest, KindTest, is_node_match};
use crate::xdmerror::*;
use crate::intmuttree::{RNode, NodeBuilder};

pub(crate) type TransResult<N> = Result<(Context<N>, Sequence<N>), Error>;

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
	}
    }
}

impl<N: Node> From<Sequence<N>> for Context<N> {
    fn from(seq: Sequence<N>) -> Self {
	Context {
	    seq,
	    i: 0,
	    depth: 0,
	}
    }
}

/// Creates a singleton sequence with the given value
pub fn literal<N: Node + 'static>(val: Rc<Item<N>>) -> Box<dyn Fn(Context<N>) -> TransResult<N>>
{
    Box::new(move |ctxt| Ok((ctxt, vec![val.clone()])))
}

/// Creates a singleton sequence with the context item as its value
pub fn context<N: Node>() -> Box<dyn Fn(Context<N>) -> TransResult<N>>
{
    Box::new(move |ctxt| Ok((ctxt.clone(), vec![ctxt.seq[ctxt.i].clone()])))
}

/// Creates a sequence. Each function in the supplied vector creates an item in the sequence. The original context is passed to each function.
pub fn tc_sequence<F, N: Node>(items: Vec<F>) -> Box<dyn Fn(Context<N>) -> TransResult<N>>
where
    F: Fn(Context<N>) -> TransResult<N> + 'static
{
    Box::new(move |ctxt| {
	match items.iter()
	    .try_fold(
		vec![],
		|mut acc, f| {
		    match f(ctxt.clone()) {
			Ok((_, mut s)) => {
			    acc.append(&mut s);
			    Ok(acc)
			}
			Err(err) => Err(err),
		    }
		}
	    ) {
		Ok(r) => Ok((ctxt.clone(), r)),
		Err(err) => Err(err)
	    }
    })
}

/// Each function in the supplied vector is evaluated. The sequence returned by a function is used as the context for the next function.
pub fn compose<F, N: Node>(steps: Vec<F>) -> Box<dyn Fn(Context<N>) -> TransResult<N>>
where
    F: Fn(Context<N>) -> TransResult<N> + 'static
{
    Box::new(move |ctxt| {
	let mut new_context = ctxt.clone();
	match steps.iter()
	    .try_fold(
		vec![],
		|_, f| {
		    match f(new_context.clone()) {
			Ok((_, s)) => {
			    new_context = Context::from(s.clone());
			    Ok(s)
			}
			Err(err) => Err(err),
		    }
		}
	    ) {
		Ok(r) => Ok((ctxt.clone(), r)),
		Err(err) => Err(err)
	    }
    })
}

/// For each item in the current context, evaluate the given node matching operation.
pub fn step<N: Node>(nm: NodeMatch) -> Box<dyn Fn(Context<N>) -> TransResult<N>> {
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
		Ok(r) => Ok((ctxt.clone(), r)),
		Err(err) => Err(err)
	    }
    })
}

/// Remove items that don't match the predicate.
pub fn filter<F, N: Node>(predicate: F) -> Box<dyn Fn(Context<N>) -> TransResult<N>>
where
    F: Fn(Context<N>) -> TransResult<N> + 'static
{
    Box::new(move |ctxt| {
	match ctxt.seq.iter()
	    .try_fold(
		vec![],
		|mut acc, i| {
		    let s = match predicate(Context::from(vec![i.clone()])) {
			Ok((_, t)) => t,
			Err(err) => return Err(err),
		    };
		    if s.to_bool() == true {
			acc.push(i.clone())
		    }
		    Ok(acc)
		}
	    ) {
		Ok(r) => Ok((Context::from(r.clone()), r)),
		Err(err) => Err(err)
	    }
    })
}
