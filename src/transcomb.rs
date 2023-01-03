//! # Transformation Combinator

use crate::item::{Item, Node, NodeType, Sequence, SequenceTrait};
use crate::qname::*;
use crate::value::{Operator, Value};
use crate::xdmerror::*;

pub(crate) type TransResult<Sequence> = Result<(Context, Sequence)>;

/// The transformation context (i.e. the dynamic context, plus some parts of the static context)
// Idea: instead of having one dynamic context that is mutable,
// make the context immutable but with shared components. Then when a new context is required, clone it and add in extra components
pub struct Context {
    seq: Sequence,	// The current context
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

impl Context {
    pub fn new() -> Self {
	Evaluator{
	    seq: Sequence::new(),
	    i: 0,
	    depth: 0,
	}
    }
}

pub(crate) fn literal(val: Item) -> impl Fn(Context) -> TransResult<V>
{
    Ok((vec![], vec![val.clone()]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singleton_literal() {
	let ev = literal(Item::Value(Value::from("this is a test")));
	let seq = ev().expect("evaluation failed");
	assert_eq!(seq.to_string(), "this is a test")
    }
}



