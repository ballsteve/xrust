//! # Transformation Combinator

use std::rc::Rc;

use crate::item::{Item, Node, NodeType, Sequence, SequenceTrait};
use crate::qname::*;
use crate::value::{Operator, Value};
use crate::xdmerror::*;
use crate::intmuttree::RNode;

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
pub(crate) fn literal<N: Node>(val: Rc<Item<N>>) -> impl Fn(Context<N>) -> TransResult<N>
{
    move |ctxt| Ok((ctxt, vec![val.clone()]))
}

/// Creates a singleton sequence with the context item as its value
pub(crate) fn context<N: Node>() -> impl Fn(Context<N>) -> TransResult<N>
{
    move |ctxt| Ok((ctxt.clone(), vec![ctxt.seq[ctxt.i].clone()]))
}

/// Creates a sequence. Each function in the supplied vector creates an item in the sequence. The original context is passed to each function.
pub(crate) fn sequence<F, N: Node>(items: Vec<F>) -> impl Fn(Context<N>) -> TransResult<N>
where
    F: Fn(Context<N>) -> TransResult<N>
{
    move |ctxt| {
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
    }
}

/// Each function in the supplied vector is evaluated. The sequence returned by a function is used as the context for the next function.
pub(crate) fn compose<F, N: Node>(steps: Vec<F>) -> impl Fn(Context<N>) -> TransResult<N>
where
    F: Fn(Context<N>) -> TransResult<N>
{
    move |ctxt| {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singleton_literal() {
	let ev = literal(Rc::new(Item::<RNode>::Value(Value::from("this is a test"))));
	let (_, seq) = ev(Context::new()).expect("evaluation failed");
	assert_eq!(seq.to_string(), "this is a test")
    }

    #[test]
    fn seq_of_literals() {
	let ev = sequence(
	    vec![
		literal(Rc::new(Item::<RNode>::Value(Value::from("this is a test")))),
		literal(Rc::new(Item::<RNode>::Value(Value::from(1)))),
		literal(Rc::new(Item::<RNode>::Value(Value::from("end of test")))),
	    ]
	);
	let (_, seq) = ev(Context::new()).expect("evaluation failed");
	assert_eq!(seq.len(), 3);
	assert_eq!(seq.to_string(), "this is a test1end of test")
    }

    #[test]
    fn seq_of_seqs() {
	let ev = sequence(
	    vec![
		sequence(
		    vec![
			literal(Rc::new(Item::<RNode>::Value(Value::from("first sequence")))),
			literal(Rc::new(Item::<RNode>::Value(Value::from(1)))),
		    ]
		),
		sequence(
		    vec![
			literal(Rc::new(Item::<RNode>::Value(Value::from("second sequence")))),
			literal(Rc::new(Item::<RNode>::Value(Value::from(2)))),
		    ]
		),
	    ]
	);
	let (_, seq) = ev(Context::new()).expect("evaluation failed");
	assert_eq!(seq.len(), 4);
	assert_eq!(seq.to_string(), "first sequence1second sequence2")
    }

    #[test]
    fn context_item() {
	let ev = context();
	let c = Context::from(vec![Rc::new(Item::<RNode>::Value(Value::from("the context item")))]);
	let (_, seq) = ev(c).expect("evaluation failed");
	assert_eq!(seq.len(), 1);
	assert_eq!(seq.to_string(), "the context item")
    }

    #[test]
    fn context_item_seq() {
	let ev = sequence(
	    vec![context(), context()]
	);
	let c = Context::from(vec![Rc::new(Item::<RNode>::Value(Value::from("the context item")))]);
	let (_, seq) = ev(c).expect("evaluation failed");
	assert_eq!(seq.len(), 2);
	assert_eq!(seq.to_string(), "the context itemthe context item")
    }

    #[test]
    fn path_of_lits() {
	let ev = compose(
	    vec![
		literal(Rc::new(Item::<RNode>::Value(Value::from("step 1")))),
		literal(Rc::new(Item::<RNode>::Value(Value::from("step 2"))))
	    ]
	);
	let (_, seq) = ev(Context::new()).expect("evaluation failed");
	assert_eq!(seq.len(), 1);
	assert_eq!(seq.to_string(), "step 2")
    }
}
