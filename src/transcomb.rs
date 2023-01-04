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
	Context{
	    seq: Sequence::new(),
	    i: 0,
	    depth: 0,
	}
    }
}

pub(crate) fn literal<N: Node>(val: Rc<Item<N>>) -> impl Fn(Context<N>) -> TransResult<N>
{
    move |ctxt| Ok((ctxt, vec![val.clone()]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singleton_literal() {
	let c = Context::new();
	let ev = literal(Rc::new(Item::<RNode>::Value(Value::from("this is a test"))));
	let (d, seq) = ev(c).expect("evaluation failed");
	assert_eq!(seq.to_string(), "this is a test")
    }
}
