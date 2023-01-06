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

/// For each item in the current context, evaluate the given node matching operation.
/// TODO: predicates
pub(crate) fn step<N: Node>(nm: NodeMatch) -> impl Fn(Context<N>) -> TransResult<N> {
    move |ctxt| {
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

    #[test]
    fn step_child_1() {
	// XPath == child::node()
	let ev = step(
	    NodeMatch {
		axis: Axis::Child,
		nodetest: NodeTest::Kind(KindTest::AnyKindTest)
	    }
	);

	// Setup a source document
	let mut sd = NodeBuilder::new(NodeType::Document).build();
	let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
	    .expect("unable to create element");
	sd.push(t.clone())
	    .expect("unable to append child");
	let l1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
	    .expect("unable to create element");
	t.push(l1.clone())
	    .expect("unable to append child");

	// Now evaluate the combinator with <Test> as the context item
	let (_, seq) = ev(Context::from(vec![Rc::new(Item::Node(t))]))
	    .expect("evaluation failed");
	assert_eq!(seq.len(), 1);
	assert_eq!(seq.to_xml(), "<Level-1></Level-1>");
    }

    #[test]
    fn step_child_many() {
	// XPath == child::node()
	let ev = step(
	    NodeMatch {
		axis: Axis::Child,
		nodetest: NodeTest::Kind(KindTest::AnyKindTest)
	    }
	);

	// Setup a source document
	let mut sd = NodeBuilder::new(NodeType::Document).build();
	let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
	    .expect("unable to create element");
	sd.push(t.clone())
	    .expect("unable to append child");
	let mut l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
	    .expect("unable to create element");
	t.push(l1_1.clone())
	    .expect("unable to append child");
	let t1 = sd.new_text(Value::from("first"))
	    .expect("unable to create text node");
	l1_1.push(t1)
	    .expect("unable to append text node");
	let mut l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
	    .expect("unable to create element");
	t.push(l1_2.clone())
	    .expect("unable to append child");
	let t2 = sd.new_text(Value::from("second"))
	    .expect("unable to create text node");
	l1_2.push(t2)
	    .expect("unable to append text node");

	// Now evaluate the combinator with both <Level-1>s as the context items
	let (_, seq) = ev(Context::from(
	    vec![
		Rc::new(Item::Node(l1_1)),
		Rc::new(Item::Node(l1_2)),
	    ]
	)).expect("evaluation failed");
	assert_eq!(seq.len(), 2);
	assert_eq!(seq.to_xml(), "firstsecond");
    }

    #[test]
    fn path_step_child() {
	// XPath == child::node()/child::node()
	let ev = compose(
	    vec![
		step(
		    NodeMatch {
			axis: Axis::Child,
			nodetest: NodeTest::Kind(KindTest::AnyKindTest)
		    }
		),
		step(
		    NodeMatch {
			axis: Axis::Child,
			nodetest: NodeTest::Kind(KindTest::AnyKindTest)
		    }
		),
	    ]
	);

	// Setup a source document
	let mut sd = NodeBuilder::new(NodeType::Document).build();
	let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
	    .expect("unable to create element");
	sd.push(t.clone())
	    .expect("unable to append child");
	let mut l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
	    .expect("unable to create element");
	t.push(l1_1.clone())
	    .expect("unable to append child");
	let t1 = sd.new_text(Value::from("first"))
	    .expect("unable to create text node");
	l1_1.push(t1)
	    .expect("unable to append text node");
	let mut l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
	    .expect("unable to create element");
	t.push(l1_2.clone())
	    .expect("unable to append child");
	let t2 = sd.new_text(Value::from("second"))
	    .expect("unable to create text node");
	l1_2.push(t2)
	    .expect("unable to append text node");

	// Now evaluate the combinator with the Test element as the context item
	let (_, seq) = ev(Context::from(
	    vec![
		Rc::new(Item::Node(t)),
	    ]
	)).expect("evaluation failed");
	assert_eq!(seq.len(), 2);
	assert_eq!(seq.to_xml(), "firstsecond");
    }
}
