#[macro_export]
macro_rules! transcomb_tests (
    ( $x:ty, $y:expr ) => {
	//use std::rc::Rc;
	//use xrust::value::Value;
	//use xrust::item::{Sequence, SequenceTrait, Item};
	use xrust::evaluate::{Axis, NodeMatch, NodeTest, KindTest, NameTest, WildcardOrName,};
	use xrust::transcomb::{Context, ContextBuilder, Template, TransResult,
			       empty,
			       literal, literal_element, literal_attribute,
			       set_attribute,
			       copy, deep_copy,
			       context, root,
			       tc_sequence, compose, step, filter,
			       tc_or, tc_and,
			       tc_loop, switch,
			       general_comparison, value_comparison,
			       tc_range, arithmetic,
			       declare_variable, reference_variable,
			       for_each,
			       group_by, group_adjacent,
			       current_group, current_grouping_key,
			       apply_templates,
			       apply_imports, next_match,
			       position, last, tc_count,
			       local_name, name,
			       string, tc_concat, starts_with, contains, substring, substring_before, substring_after,
			       normalize_space, translate,
			       boolean, not, tc_true, tc_false,
			       number, sum, floor, ceiling, round,
			       current_date_time, current_date, current_time,
			       format_date_time, format_date, format_time,
			       function_user_defined,
	};

	#[test]
	fn tc_empty() {
	    let ev = empty::<$x>();
	    let seq = ev(&mut Context::new()).expect("evaluation failed");
	    assert_eq!(seq.len(), 0)
	}
	#[test]
	fn tc_singleton_literal() {
	    let ev = literal(Rc::new(Item::<$x>::Value(Value::from("this is a test"))));
	    let seq = ev(&mut Context::new()).expect("evaluation failed");
	    assert_eq!(seq.to_string(), "this is a test")
	}
	#[test]
	fn tc_literal_element() {
	    let ev = literal_element(
		QualifiedName::new(None, None, String::from("Test")),
		literal(Rc::new(Item::<$x>::Value(Value::from("content"))))
	    );
	    let mut mydoc = $y();
	    let mut ctxt = ContextBuilder::new()
		.result_document(mydoc)
		.build();
	    let seq = ev(&mut ctxt).expect("evaluation failed");
	    assert_eq!(seq.to_xml(), "<Test>content</Test>")
	}
	#[test]
	fn tc_literal_element_nested() {
	    let ev = literal_element(
		QualifiedName::new(None, None, String::from("Test")),
		literal_element(
		    QualifiedName::new(None, None, String::from("Level-1")),
		    literal(Rc::new(Item::<$x>::Value(Value::from("content"))))
		)
	    );
	    let mut mydoc = $y();
	    let mut ctxt = ContextBuilder::new()
		.result_document(mydoc)
		.build();
	    let seq = ev(&mut ctxt).expect("evaluation failed");
	    assert_eq!(seq.to_xml(), "<Test><Level-1>content</Level-1></Test>")
	}
	#[test]
	fn tc_literal_attribute() {
	    let ev = literal_element(
		QualifiedName::new(None, None, String::from("Test")),
		tc_sequence(vec![
		    literal_attribute(
			QualifiedName::new(None, None, String::from("foo")),
			literal(Rc::new(Item::<$x>::Value(Value::from("bar"))))
		    ),
		    literal(Rc::new(Item::<$x>::Value(Value::from("content")))),
		])
	    );
	    let mut mydoc = $y();
	    let mut ctxt = ContextBuilder::new()
		.result_document(mydoc)
		.build();
	    let seq = ev(&mut ctxt).expect("evaluation failed");
	    assert_eq!(seq.to_xml(), "<Test foo='bar'>content</Test>")
	}
	#[test]
	fn tc_set_attribute() {
	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");

	    let ev = set_attribute(
		QualifiedName::new(None, None, String::from("foo")),
		literal(Rc::new(Item::<$x>::Value(Value::from("bar")))),
	    );
	    let mut mydoc = $y();
	    let mut ctxt = ContextBuilder::new()
		.result_document(mydoc)
		.sequence(vec![Rc::new(Item::Node(t))])
		.build();
	    let seq = ev(&mut ctxt).expect("evaluation failed");
	    assert_eq!(sd.to_xml(), "<Test foo='bar'></Test>")
	}
	#[test]
	fn tc_copy_literal() {
	    let ev = copy(
		Some(literal(Rc::new(Item::<$x>::Value(Value::from("this is the original"))))),
		None
	    );
	    let seq = ev(&mut Context::new()).expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "this is the original")
	}
	#[test]
	fn tc_copy_context_literal() {
	    let ev = copy(
		None::<Box<dyn Fn(&mut Context<$x>) -> TransResult<$x>>>,
		None::<Box<dyn Fn(&mut Context<$x>) -> TransResult<$x>>>
	    );
	    let seq = ev(
		&mut ContextBuilder::new()
		    .sequence(vec![Rc::new(Item::<$x>::Value(Value::from("this is the original")))])
		    .build()
	    ).expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "this is the original")
	}
	#[test]
	fn tc_copy_context_node() {
	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    t.push(sd.new_text(Value::from("this is the original")).expect("unable to create text node"))
		.expect("unable to add text node");

	    let ev = copy(
		None::<Box<dyn Fn(&mut Context<$x>) -> TransResult<$x>>>,
		Some(literal((Rc::new(Item::<$x>::Value(Value::from("this is the copy"))))))
	    );

	    let mut mydoc = $y();
	    let mut ctxt = ContextBuilder::new()
		.result_document(mydoc)
		.sequence(vec![Rc::new(Item::Node(t))])
		.build();
	    let seq = ev(&mut ctxt).expect("evaluation failed");

	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_xml(), "<Test>this is the copy</Test>");
	    assert_eq!(sd.to_xml(), "<Test>this is the original</Test>")
	}
	#[test]
	fn tc_deep_copy() {
	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut u = sd.new_element(QualifiedName::new(None, None, String::from("inner")))
		.expect("unable to create element");
	    t.push(u.clone())
		.expect("unable to append child");
	    u.push(sd.new_text(Value::from("this is the original")).expect("unable to create text node"))
		.expect("unable to add text node");

	    let ev = deep_copy(
		None::<Box<dyn Fn(&mut Context<$x>) -> TransResult<$x>>>
	    );

	    let mut ctxt = ContextBuilder::new()
		.sequence(vec![Rc::new(Item::Node(t))])
		.build();
	    let seq = ev(&mut ctxt).expect("evaluation failed");

	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_xml(), "<Test><inner>this is the original</inner></Test>");
	}

	#[test]
	fn tc_seq_of_literals() {
	    let ev = tc_sequence(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from("this is a test")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from(1)))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("end of test")))),
		]
	    );
	    let seq = ev(&mut Context::new()).expect("evaluation failed");
	    assert_eq!(seq.len(), 3);
	    assert_eq!(seq.to_string(), "this is a test1end of test")
	}
	#[test]
	fn tc_seq_of_seqs() {
	    let ev = tc_sequence(
		vec![
		    tc_sequence(
			vec![
			    literal(Rc::new(Item::<$x>::Value(Value::from("first sequence")))),
			    literal(Rc::new(Item::<$x>::Value(Value::from(1)))),
			]
		    ),
		    tc_sequence(
			vec![
			    literal(Rc::new(Item::<$x>::Value(Value::from("second sequence")))),
			    literal(Rc::new(Item::<$x>::Value(Value::from(2)))),
			]
		    ),
		]
	    );
	    let seq = ev(&mut Context::new()).expect("evaluation failed");
	    assert_eq!(seq.len(), 4);
	    assert_eq!(seq.to_string(), "first sequence1second sequence2")
	}

	#[test]
	fn tc_switch_when() {
	    let ev = switch(
		vec![
		    (value_comparison(
			Operator::Equal,
			literal(Rc::new(Item::<$x>::Value(Value::from(1)))),
			literal(Rc::new(Item::<$x>::Value(Value::from(2.0)))),
		    ),
		    literal(Rc::new(Item::<$x>::Value(Value::from("comparison failed"))))),
		    (value_comparison(
			Operator::Equal,
			literal(Rc::new(Item::<$x>::Value(Value::from(1)))),
			literal(Rc::new(Item::<$x>::Value(Value::from(1.0)))),
		    ),
		    literal(Rc::new(Item::<$x>::Value(Value::from("comparison succeeded"))))),
		    (value_comparison(
			Operator::Equal,
			literal(Rc::new(Item::<$x>::Value(Value::from(1)))),
			literal(Rc::new(Item::<$x>::Value(Value::from(3.0)))),
		    ),
		    literal(Rc::new(Item::<$x>::Value(Value::from("comparison failed"))))),
		],
		literal(Rc::new(Item::<$x>::Value(Value::from("otherwise clause"))))
	    );
	    let seq = ev(&mut Context::new()).expect("evaluation failed");
	    assert_eq!(seq.to_string(), "comparison succeeded")
	}
	#[test]
	fn tc_switch_otherwise() {
	    let ev = switch(
		vec![
		    (value_comparison(
			Operator::Equal,
			literal(Rc::new(Item::<$x>::Value(Value::from(1)))),
			literal(Rc::new(Item::<$x>::Value(Value::from(2.0)))),
		    ),
		    literal(Rc::new(Item::<$x>::Value(Value::from("comparison failed"))))),
		    (value_comparison(
			Operator::Equal,
			literal(Rc::new(Item::<$x>::Value(Value::from(1)))),
			literal(Rc::new(Item::<$x>::Value(Value::from(11.0)))),
		    ),
		    literal(Rc::new(Item::<$x>::Value(Value::from("comparison failed"))))),
		    (value_comparison(
			Operator::Equal,
			literal(Rc::new(Item::<$x>::Value(Value::from(1)))),
			literal(Rc::new(Item::<$x>::Value(Value::from(3.0)))),
		    ),
		    literal(Rc::new(Item::<$x>::Value(Value::from("comparison failed"))))),
		],
		literal(Rc::new(Item::<$x>::Value(Value::from("otherwise clause"))))
	    );
	    let seq = ev(&mut Context::new()).expect("evaluation failed");
	    assert_eq!(seq.to_string(), "otherwise clause")
	}

	#[test]
	fn tc_loop_lit() {
	    let ev = tc_loop(
		(String::from("x"), tc_sequence(
		    vec![
			literal(Rc::new(Item::<$x>::Value(Value::from("one")))),
			literal(Rc::new(Item::<$x>::Value(Value::from("two")))),
			literal(Rc::new(Item::<$x>::Value(Value::from("three")))),
		    ]
		)),
		tc_concat(vec![
		    reference_variable(String::from("x")),
		    reference_variable(String::from("x")),
		])
	    );
	    let seq = ev(&mut Context::new()).expect("evaluation failed");
	    assert_eq!(seq.to_string(), "oneonetwotwothreethree")
	}

	#[test]
	fn tc_context_item() {
	    let ev = context();
	    let mut c = Context::from(vec![Rc::new(Item::<$x>::Value(Value::from("the context item")))]);
	    let seq = ev(&mut c).expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "the context item")
	}

	#[test]
	fn tc_context_item_seq() {
	    let ev = tc_sequence(
		vec![context(), context()]
	    );
	    let mut c = Context::from(vec![Rc::new(Item::<$x>::Value(Value::from("the context item")))]);
	    let seq = ev(&mut c).expect("evaluation failed");
	    assert_eq!(seq.len(), 2);
	    assert_eq!(seq.to_string(), "the context itemthe context item")
	}

	#[test]
	fn tc_root() {
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

	    let ev = root();

	    // Now evaluate the combinator with <Level-1> as the context item
	    let seq = ev(&mut Context::from(vec![Rc::new(Item::Node(l1))]))
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_xml(), "<Test><Level-1></Level-1></Test>");
	}

	#[test]
	fn tc_path_of_lits() {
	    let ev = compose(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from("step 1")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("step 2"))))
		]
	    );
	    let seq = ev(&mut Context::new()).expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "step 2")
	}

	#[test]
	fn tc_step_child_1() {
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
	    let seq = ev(&mut Context::from(vec![Rc::new(Item::Node(t))]))
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_xml(), "<Level-1></Level-1>");
	}

	#[test]
	fn tc_step_child_many() {
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
	    let seq = ev(&mut Context::from(
		vec![
		    Rc::new(Item::Node(l1_1)),
		    Rc::new(Item::Node(l1_2)),
		]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 2);
	    assert_eq!(seq.to_xml(), "firstsecond");
	}

	#[test]
	fn tc_step_self() {
	    // XPath == child::node()
	    let ev = step(
		NodeMatch {
		    axis: Axis::Selfaxis,
		    nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name(String::from("Level-1")))})
		}
	    );

	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Value::from("first"))
		.expect("unable to create text node");
	    t.push(t1.clone())
		.expect("unable to append text node");
	    let l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_2.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Value::from("second"))
		.expect("unable to create text node");
	    t.push(t2.clone())
		.expect("unable to append text node");
	    let et = sd.new_element(QualifiedName::new(None, None, String::from("extra")))
		.expect("unable to create element");
	    t.push(et.clone())
		.expect("unable to append child");

	    // Now evaluate the combinator with Test's children as the context items
	    let seq = ev(&mut Context::from(
		vec![
		    Rc::new(Item::Node(l1_1)),
		    Rc::new(Item::Node(t1)),
		    Rc::new(Item::Node(l1_2)),
		    Rc::new(Item::Node(t2)),
		    Rc::new(Item::Node(et)),
		]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 2);
	    assert_eq!(seq.to_xml(), "<Level-1></Level-1><Level-1></Level-1>");
	}

	#[test]
	fn tc_step_selfdoc_pos() {
	    let ev = step(
		NodeMatch {
		    axis: Axis::SelfDocument,
		    nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name(String::from("Level-1")))})
		}
	    );

	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Value::from("first"))
		.expect("unable to create text node");
	    t.push(t1.clone())
		.expect("unable to append text node");
	    let l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_2.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Value::from("second"))
		.expect("unable to create text node");
	    t.push(t2.clone())
		.expect("unable to append text node");
	    let et = sd.new_element(QualifiedName::new(None, None, String::from("extra")))
		.expect("unable to create element");
	    t.push(et.clone())
		.expect("unable to append child");

	    // Now evaluate the combinator with Test's document node as the context items
	    let seq = ev(&mut Context::from(
		vec![Rc::new(Item::Node(sd))]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	}
	#[test]
	fn tc_step_selfdoc_neg() {
	    let ev = step(
		NodeMatch {
		    axis: Axis::SelfDocument,
		    nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name(String::from("Level-1")))})
		}
	    );

	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Value::from("first"))
		.expect("unable to create text node");
	    t.push(t1.clone())
		.expect("unable to append text node");
	    let l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_2.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Value::from("second"))
		.expect("unable to create text node");
	    t.push(t2.clone())
		.expect("unable to append text node");
	    let et = sd.new_element(QualifiedName::new(None, None, String::from("extra")))
		.expect("unable to create element");
	    t.push(et.clone())
		.expect("unable to append child");

	    // Now evaluate the combinator with Test's document element node as the context items
	    let seq = ev(&mut Context::from(
		vec![Rc::new(Item::Node(t))]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 0);
	}

	#[test]
	fn tc_step_parent() {
	    // XPath == parent::*
	    let ev = step(
		NodeMatch {
		    axis: Axis::Parent,
		    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
		}
	    );

	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Value::from("first"))
		.expect("unable to create text node");
	    t.push(t1.clone())
		.expect("unable to append text node");
	    let l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_2.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Value::from("second"))
		.expect("unable to create text node");
	    t.push(t2.clone())
		.expect("unable to append text node");
	    let et = sd.new_element(QualifiedName::new(None, None, String::from("extra")))
		.expect("unable to create element");
	    t.push(et.clone())
		.expect("unable to append child");

	    // Now evaluate the combinator with Test's children as the context items
	    let seq = ev(&mut Context::from(
		vec![
		    Rc::new(Item::Node(l1_1)),
		    Rc::new(Item::Node(t1)),
		    Rc::new(Item::Node(l1_2)),
		    Rc::new(Item::Node(t2)),
		    Rc::new(Item::Node(et)),
		]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq[0].name().to_string(), "Test");
	}

	#[test]
	fn tc_step_parentdoc_pos() {
	    let ev = step(
		NodeMatch {
		    axis: Axis::ParentDocument,
		    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
		}
	    );

	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Value::from("first"))
		.expect("unable to create text node");
	    t.push(t1.clone())
		.expect("unable to append text node");

	    // Now evaluate the combinator with the root node as the context items
	    let seq = ev(&mut Context::from(
		vec![
		    Rc::new(Item::Node(sd)),
		]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	}
	#[test]
	fn tc_step_parentdoc_neg() {
	    let ev = step(
		NodeMatch {
		    axis: Axis::ParentDocument,
		    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
		}
	    );

	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Value::from("first"))
		.expect("unable to create text node");
	    t.push(t1.clone())
		.expect("unable to append text node");

	    // Now evaluate the combinator with the document element as the context items
	    let seq = ev(&mut Context::from(
		vec![
		    Rc::new(Item::Node(t)),
		]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 0);
	}

	#[test]
	fn tc_step_descendant() {
	    let ev = step(
		NodeMatch {
		    axis: Axis::Descendant,
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
	    t.push(t1.clone())
		.expect("unable to append text node");
	    let mut l2_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-2")))
		.expect("unable to create element");
	    l1_1.push(l2_1.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Value::from("second"))
		.expect("unable to create text node");
	    l2_1.push(t2.clone())
		.expect("unable to append text node");

	    // Now evaluate the combinator with the document element as the context items
	    let seq = ev(&mut Context::from(
		vec![
		    Rc::new(Item::Node(t)),
		]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 4);
	}

	#[test]
	fn tc_step_descendant_or_self() {
	    let ev = step(
		NodeMatch {
		    axis: Axis::DescendantOrSelf,
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
	    t.push(t1.clone())
		.expect("unable to append text node");
	    let mut l2_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-2")))
		.expect("unable to create element");
	    l1_1.push(l2_1.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Value::from("second"))
		.expect("unable to create text node");
	    l2_1.push(t2.clone())
		.expect("unable to append text node");

	    // Now evaluate the combinator with the document element as the context item
	    let seq = ev(&mut Context::from(
		vec![
		    Rc::new(Item::Node(t)),
		]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 5);
	}

	#[test]
	fn tc_step_descendant_or_self_or_root() {
	    let ev = step(
		NodeMatch {
		    axis: Axis::DescendantOrSelfOrRoot,
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
	    t.push(t1.clone())
		.expect("unable to append text node");
	    let mut l2_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-2")))
		.expect("unable to create element");
	    l1_1.push(l2_1.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Value::from("second"))
		.expect("unable to create text node");
	    l2_1.push(t2.clone())
		.expect("unable to append text node");

	    // Now evaluate the combinator with the root node as the context item
	    let seq = ev(&mut Context::from(
		vec![
		    Rc::new(Item::Node(sd)),
		]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 6);
	}

	#[test]
	fn tc_step_ancestor() {
	    let ev = step(
		NodeMatch {
		    axis: Axis::Ancestor,
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
	    t.push(t1.clone())
		.expect("unable to append text node");
	    let mut l2_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-2")))
		.expect("unable to create element");
	    l1_1.push(l2_1.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Value::from("second"))
		.expect("unable to create text node");
	    l2_1.push(t2.clone())
		.expect("unable to append text node");

	    // Now evaluate the combinator with the lowest node as the context item
	    let seq = ev(&mut Context::from(
		vec![
		    Rc::new(Item::Node(t2)),
		]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 3);
	}
	fn get_node<N: Node>(i: &Rc<Item<N>>) -> N {
	    match &**i {
		Item::Node(n) => n.clone(),
		_ => panic!("not a node"),
	    }
	}

	#[test]
	fn tc_step_ancestor_or_self() {
	    let ev = step(
		NodeMatch {
		    axis: Axis::AncestorOrSelf,
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
	    t.push(t1.clone())
		.expect("unable to append text node");
	    let mut l2_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-2")))
		.expect("unable to create element");
	    l1_1.push(l2_1.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Value::from("second"))
		.expect("unable to create text node");
	    l2_1.push(t2.clone())
		.expect("unable to append text node");

	    // Now evaluate the combinator with the lowest node as the context item
	    let seq = ev(&mut Context::from(
		vec![
		    Rc::new(Item::Node(t2)),
		]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 4);
	}

	#[test]
	fn tc_step_following_sibling() {
	    // XPath == following-sibling::node()
	    let ev = step(
		NodeMatch {
		    axis: Axis::FollowingSibling,
		    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
		}
	    );

	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Value::from("first"))
		.expect("unable to create text node");
	    t.push(t1.clone())
		.expect("unable to append text node");
	    let l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_2.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Value::from("second"))
		.expect("unable to create text node");
	    t.push(t2.clone())
		.expect("unable to append text node");
	    let et = sd.new_element(QualifiedName::new(None, None, String::from("extra")))
		.expect("unable to create element");
	    t.push(et.clone())
		.expect("unable to append child");

	    // Now evaluate the combinator with Test's first child as the context items
	    let seq = ev(&mut Context::from(
		vec![
		    Rc::new(Item::Node(l1_1)),
		]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 4);
	}

	#[test]
	fn tc_step_preceding_sibling() {
	    // XPath == preceding-sibling::node()
	    let ev = step(
		NodeMatch {
		    axis: Axis::PrecedingSibling,
		    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
		}
	    );

	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Value::from("first"))
		.expect("unable to create text node");
	    t.push(t1.clone())
		.expect("unable to append text node");
	    let l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_2.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Value::from("second"))
		.expect("unable to create text node");
	    t.push(t2.clone())
		.expect("unable to append text node");
	    let et = sd.new_element(QualifiedName::new(None, None, String::from("extra")))
		.expect("unable to create element");
	    t.push(et.clone())
		.expect("unable to append child");

	    // Now evaluate the combinator with Test's last child as the context items
	    let seq = ev(&mut Context::from(
		vec![
		    Rc::new(Item::Node(et)),
		]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 4);
	}

	#[test]
	fn tc_step_following() {
	    // XPath == following::node()
	    let ev = step(
		NodeMatch {
		    axis: Axis::Following,
		    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
		}
	    );

	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut one = sd.new_element(QualifiedName::new(None, None, String::from("Left-1")))
		.expect("unable to create element");
	    t.push(one.clone())
		.expect("unable to append child");
	    let mut two = sd.new_element(QualifiedName::new(None, None, String::from("Right-1")))
		.expect("unable to create element");
	    t.push(two.clone())
		.expect("unable to append child");
	    let three = sd.new_element(QualifiedName::new(None, None, String::from("Left-2")))
		.expect("unable to create element");
	    one.push(three.clone())
		.expect("unable to append child");
	    let mut four = sd.new_element(QualifiedName::new(None, None, String::from("Right-2")))
		.expect("unable to create element");
	    one.push(four.clone())
		.expect("unable to append child");
	    let five = sd.new_element(QualifiedName::new(None, None, String::from("Left-2")))
		.expect("unable to create element");
	    two.push(five.clone())
		.expect("unable to append child");
	    let six = sd.new_element(QualifiedName::new(None, None, String::from("Right-2")))
		.expect("unable to create element");
	    two.push(six.clone())
		.expect("unable to append child");

	    let seven = sd.new_element(QualifiedName::new(None, None, String::from("Left-3")))
		.expect("unable to create element");
	    four.push(seven.clone())
		.expect("unable to append child");
	    let eight = sd.new_element(QualifiedName::new(None, None, String::from("Right-3")))
		.expect("unable to create element");
	    four.push(eight.clone())
		.expect("unable to append child");

	    // Now evaluate the combinator with lowest left node as the context items
	    let seq = ev(&mut Context::from(
		vec![
		    Rc::new(Item::Node(seven)),
		]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 4);
	}

	#[test]
	fn tc_step_preceding() {
	    // XPath == preceding::node()
	    let ev = step(
		NodeMatch {
		    axis: Axis::Preceding,
		    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
		}
	    );

	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut one = sd.new_element(QualifiedName::new(None, None, String::from("Left-1")))
		.expect("unable to create element");
	    t.push(one.clone())
		.expect("unable to append child");
	    let mut two = sd.new_element(QualifiedName::new(None, None, String::from("Right-1")))
		.expect("unable to create element");
	    t.push(two.clone())
		.expect("unable to append child");
	    let three = sd.new_element(QualifiedName::new(None, None, String::from("Left-2")))
		.expect("unable to create element");
	    one.push(three.clone())
		.expect("unable to append child");
	    let mut four = sd.new_element(QualifiedName::new(None, None, String::from("Right-2")))
		.expect("unable to create element");
	    one.push(four.clone())
		.expect("unable to append child");
	    let five = sd.new_element(QualifiedName::new(None, None, String::from("Left-2")))
		.expect("unable to create element");
	    two.push(five.clone())
		.expect("unable to append child");
	    let six = sd.new_element(QualifiedName::new(None, None, String::from("Right-2")))
		.expect("unable to create element");
	    two.push(six.clone())
		.expect("unable to append child");

	    let seven = sd.new_element(QualifiedName::new(None, None, String::from("Left-3")))
		.expect("unable to create element");
	    four.push(seven.clone())
		.expect("unable to append child");
	    let eight = sd.new_element(QualifiedName::new(None, None, String::from("Right-3")))
		.expect("unable to create element");
	    four.push(eight.clone())
		.expect("unable to append child");

	    // Now evaluate the combinator with last node as the context item
	    let seq = ev(&mut Context::from(
		vec![
		    Rc::new(Item::Node(six)),
		]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 6);
	}

	#[test]
	fn tc_path_step_child() {
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
	    let seq = ev(&mut Context::from(
		vec![
		    Rc::new(Item::Node(t)),
		]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 2);
	    assert_eq!(seq.to_xml(), "firstsecond");
	}

	#[test]
	fn tc_step_attribute() {
	    // XPath == child::node()/attribute::*
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
			    axis: Axis::Attribute,
			    nodetest: NodeTest::Name(NameTest{name: Some(WildcardOrName::Wildcard), ns: None, prefix: None})
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
	    let t1 = sd.new_text(Value::from("one"))
		.expect("unable to create text node");
	    l1_1.push(t1)
		.expect("unable to append text node");
	    let a1 = sd.new_attribute(QualifiedName::new(None, None, String::from("name")), Value::from("first"))
		.expect("unable to create attribute node");
	    l1_1.add_attribute(a1)
		.expect("unable to add attribute node");
	    let mut l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_2.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Value::from("two"))
		.expect("unable to create text node");
	    l1_2.push(t2)
		.expect("unable to append text node");
	    let a2 = sd.new_attribute(QualifiedName::new(None, None, String::from("name")), Value::from("second"))
		.expect("unable to create attribute node");
	    l1_2.add_attribute(a2)
		.expect("unable to add attribute node");

	    // Now evaluate the combinator with the Test element as the context item
	    let seq = ev(&mut Context::from(
		vec![
		    Rc::new(Item::Node(t)),
		]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 2);
	    assert_eq!(seq.to_string(), "firstsecond");
	}
	#[test]
	fn tc_step_self_attribute_pos() {
	    let ev = step(
		NodeMatch {
		    axis: Axis::SelfAttribute,
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
	    let t1 = sd.new_text(Value::from("one"))
		.expect("unable to create text node");
	    l1_1.push(t1)
		.expect("unable to append text node");
	    let a1 = sd.new_attribute(QualifiedName::new(None, None, String::from("name")), Value::from("first"))
		.expect("unable to create attribute node");
	    l1_1.add_attribute(a1)
		.expect("unable to add attribute node");
	    let mut l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_2.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Value::from("two"))
		.expect("unable to create text node");
	    l1_2.push(t2)
		.expect("unable to append text node");
	    let a2 = sd.new_attribute(QualifiedName::new(None, None, String::from("name")), Value::from("second"))
		.expect("unable to create attribute node");
	    l1_2.add_attribute(a2.clone())
		.expect("unable to add attribute node");

	    // Now evaluate the combinator with an attribute as the context item
	    let seq = ev(&mut Context::from(
		vec![
		    Rc::new(Item::Node(a2)),
		]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "second");
	}
	#[test]
	fn tc_step_self_attribute_neg() {
	    let ev = step(
		NodeMatch {
		    axis: Axis::SelfAttribute,
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
	    let t1 = sd.new_text(Value::from("one"))
		.expect("unable to create text node");
	    l1_1.push(t1)
		.expect("unable to append text node");
	    let a1 = sd.new_attribute(QualifiedName::new(None, None, String::from("name")), Value::from("first"))
		.expect("unable to create attribute node");
	    l1_1.add_attribute(a1)
		.expect("unable to add attribute node");
	    let mut l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_2.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Value::from("two"))
		.expect("unable to create text node");
	    l1_2.push(t2)
		.expect("unable to append text node");
	    let a2 = sd.new_attribute(QualifiedName::new(None, None, String::from("name")), Value::from("second"))
		.expect("unable to create attribute node");
	    l1_2.add_attribute(a2.clone())
		.expect("unable to add attribute node");

	    // Now evaluate the combinator with an element as the context item
	    let seq = ev(&mut Context::from(
		vec![
		    Rc::new(Item::Node(l1_2)),
		]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 0);
	}

	#[test]
	fn tc_predicate() {
	    // XPath == child::node()[child::node()]
	    let ev = compose(
		vec![
		    step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
			}
		    ),
		    filter(
			step(
			    NodeMatch {
				axis: Axis::Child,
				nodetest: NodeTest::Kind(KindTest::AnyKindTest)
			    }
			)
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
	    let l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-2")))
		.expect("unable to create element");
	    t.push(l1_2.clone())
		.expect("unable to append child");

	    // Now evaluate the combinator with the Test element as the context item
	    let seq = ev(&mut Context::from(
		vec![
		    Rc::new(Item::Node(t)),
		]
	    )).expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_xml(), "<Level-1>first</Level-1>");
	}

	#[test]
	fn tc_or_true() {
	    let ev = tc_or(vec![
		literal(Rc::new(Item::<$x>::Value(Value::from(0)))),
		literal(Rc::new(Item::<$x>::Value(Value::from("false")))),
	    ]);
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), true)
	}
	#[test]
	fn tc_or_false() {
	    let ev = tc_or(vec![
		literal(Rc::new(Item::<$x>::Value(Value::from(0)))),
	    ]);
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), false)
	}

	#[test]
	fn tc_and_true() {
	    let ev = tc_and(vec![
		literal(Rc::new(Item::<$x>::Value(Value::from(1)))),
		literal(Rc::new(Item::<$x>::Value(Value::from("false")))),
	    ]);
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), true)
	}
	#[test]
	fn tc_and_false() {
	    let ev = tc_and(vec![
		literal(Rc::new(Item::<$x>::Value(Value::from("true")))),
		literal(Rc::new(Item::<$x>::Value(Value::from(0)))),
	    ]);
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), false)
	}

	#[test]
	fn tc_general_compare_true() {
	    let ev = general_comparison(
		Operator::Equal,
		tc_sequence(vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from("true")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("false")))),
		]),
		tc_sequence(vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from(0)))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("true")))),
		]),
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), true)
	}
	#[test]
	fn tc_general_compare_false() {
	    let ev = general_comparison(
		Operator::Equal,
		tc_sequence(vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from("true")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("false")))),
		]),
		tc_sequence(vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from(1)))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("foo")))),
		]),
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), false)
	}

	#[test]
	fn tc_value_compare_true() {
	    let ev = value_comparison(
		Operator::Equal,
		literal(Rc::new(Item::<$x>::Value(Value::from("true")))),
		literal(Rc::new(Item::<$x>::Value(Value::from("true")))),
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), true)
	}
	#[test]
	fn tc_value_compare_false() {
	    let ev = value_comparison(
		Operator::Equal,
		literal(Rc::new(Item::<$x>::Value(Value::from("true")))),
		literal(Rc::new(Item::<$x>::Value(Value::from("false")))),
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), false)
	}

	#[test]
	fn tc_range_empty() {
	    let ev = tc_range(
		empty(),
		literal(Rc::new(Item::<$x>::Value(Value::from(10)))),
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 0);
	}
	#[test]
	fn tc_range_many() {
	    let ev = tc_range(
		literal(Rc::new(Item::<$x>::Value(Value::from(1)))),
		literal(Rc::new(Item::<$x>::Value(Value::from(10)))),
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 10);
	    assert_eq!(seq.to_string(), "12345678910");
	}
	#[test]
	fn tc_range_one() {
	    let ev = tc_range(
		literal(Rc::new(Item::<$x>::Value(Value::from(5)))),
		literal(Rc::new(Item::<$x>::Value(Value::from(5)))),
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "5");
	}

	#[test]
	fn tc_arithmetic_add() {
	    let ev = arithmetic(vec![
		(ArithmeticOperator::Noop, literal(Rc::new(Item::<$x>::Value(Value::from(5))))),
		(ArithmeticOperator::Add, literal(Rc::new(Item::<$x>::Value(Value::from(5))))),
	    ]);
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "10");
	}

	#[test]
	fn tc_var_declare() {
	    let ev = tc_sequence(
		vec![
		    declare_variable(
			"foo".to_string(),
			literal(Rc::new(Item::<$x>::Value(Value::from("foo")))),
			reference_variable("foo".to_string()),
		    ),
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "foo")
	}

	#[test]
	fn tc_for_each() {
	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to element node");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1 = sd.new_element(QualifiedName::new(None, None, String::from("Level1")))
		.expect("unable to element node");
	    t.push(l1.clone())
		.expect("unable to append child");
	    l1.push(sd.new_text(Value::from("one")).expect("unable to create text node"))
		.expect("unable to append text");
	    let mut l2 = sd.new_element(QualifiedName::new(None, None, String::from("Level1")))
		.expect("unable to element node");
	    t.push(l2.clone())
		.expect("unable to append child");
	    l2.push(sd.new_text(Value::from("two")).expect("unable to create text node"))
		.expect("unable to append text");
	    let mut l3 = sd.new_element(QualifiedName::new(None, None, String::from("Level1")))
		.expect("unable to element node");
	    t.push(l3.clone())
		.expect("unable to append child");
	    l3.push(sd.new_text(Value::from("three")).expect("unable to create text node"))
		.expect("unable to append text");

	    // xsl:for-each select="/*/*" body == xsl:text "found a Level-1"
	    let ev = for_each(
		compose(vec![
		    root(),
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
		]),
		literal(Rc::new(Item::Value(Value::from("found a Level-1")))),
	    );

	    let seq = ev(&mut ContextBuilder::new()
			 .sequence(vec![Rc::new(Item::Node(sd))])
			 .build()
	    ).expect("evaluation failed");
	    assert_eq!(seq.len(), 3);
	    assert_eq!(seq.to_string(), "found a Level-1found a Level-1found a Level-1")
	}

	#[test]
	fn tc_group_by_1() {
	    // xsl:for-each-group select="1 to 50" group-by=". mod 10" body == xsl:text "group current-grouping-key size count(current-group)"
	    let ev = group_by(
		tc_range(
		    literal(Rc::new(Item::<$x>::Value(Value::from(1)))),
		    literal(Rc::new(Item::<$x>::Value(Value::from(50)))),
		),
		arithmetic(vec![
		    (ArithmeticOperator::Noop, context()),
		    (ArithmeticOperator::Modulo, literal(Rc::new(Item::<$x>::Value(Value::from(10)))))
		]),
		literal_element(
		    QualifiedName::new(None, None, String::from("group")),
		    tc_sequence(vec![
			literal(Rc::new(Item::Value(Value::from("key ")))),
			current_grouping_key(),
			literal(Rc::new(Item::Value(Value::from(" #members ")))),
			tc_count(vec![current_group()]),
		    ])
		)
	    );

	    let mut resdoc = $y();
	    let seq = ev(&mut ContextBuilder::new()
			 .result_document(resdoc)
			 .build()
	    ).expect("evaluation failed");
	    assert_eq!(seq.len(), 10);
	    // the groups are not ordered, so it is difficult to test all of the groups are correct
	    //assert_eq!(seq[0].to_string(), "key 0 #members 10")
	}

	#[test]
	fn tc_group_adjacent_1() {
	    // xsl:for-each-group select="(a, a, b, c, c, c)" group-adjacent="." body == xsl:text "group current-grouping-key size count(current-group)"
	    let ev = group_adjacent(
		tc_sequence(vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from("a")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("a")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("b")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("c")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("c")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("c")))),
		]),
		context(),
		literal_element(
		    QualifiedName::new(None, None, String::from("group")),
		    tc_sequence(vec![
			literal(Rc::new(Item::Value(Value::from("key ")))),
			current_grouping_key(),
			literal(Rc::new(Item::Value(Value::from(" #members ")))),
			tc_count(vec![current_group()]),
		    ])
		)
	    );

	    let mut resdoc = $y();
	    let seq = ev(&mut ContextBuilder::new()
			 .result_document(resdoc)
			 .build()
	    ).expect("evaluation failed");
	    assert_eq!(seq.len(), 3);
	    // the groups are not ordered, so it is difficult to test all of the groups are correct
	    //assert_eq!(seq[0].to_string(), "key 0 #members 10")
	}

	#[test]
	fn tc_apply_templates_builtins() {
	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_text(Value::from("Test"))
		.expect("unable to text node");
	    sd.push(t.clone())
		.expect("unable to append child");

	    // Built-in template rule for "/"
	    let ev = apply_templates(root::<$x>());
	    let mut ctxt = ContextBuilder::new()
		.template(Template::new(
		    step(
			NodeMatch {
			    axis: Axis::SelfDocument,
			    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
			}
		    ), // pattern "/",
		    apply_templates(step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
			}
		    )), // body "apply-templates select=node()",
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.template(Template::new(
		    step(
			NodeMatch {
			    axis: Axis::Selfaxis,
			    nodetest: NodeTest::Kind(KindTest::TextTest)
			}
		    ), // pattern child::text()
		    context(), // body value-of select='.'
		    None,
		    vec![0],
		    None,
		    None,
		))
		.sequence(vec![Rc::new(Item::Node(sd))])
		.build();

	    // Now Evaluate the combinator with the source document root node as the context item
	    let seq = ev(&mut ctxt)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "Test")
	}

	#[test]
	fn tc_apply_templates_1() {
	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create new element");
	    sd.push(t.clone());
	    let c = sd.new_text(Value::from("content"))
		.expect("unable to text node");
	    t.push(c)
		.expect("unable to append child");

	    // Template rule for "Test", plus builtins
	    let ev = apply_templates(root::<$x>());
	    let mut ctxt = ContextBuilder::new()
		.template(Template::new(
		    step(
			NodeMatch {
			    axis: Axis::Selfaxis,
			    nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name(String::from("Test")))})
			}
		    ), // pattern "Test"
		    tc_sequence(vec![
			literal((Rc::new(Item::<$x>::Value(Value::from("before "))))),
			apply_templates(step(
			    NodeMatch {
				axis: Axis::Child,
				nodetest: NodeTest::Kind(KindTest::AnyKindTest)
			    }
			)),
			literal((Rc::new(Item::<$x>::Value(Value::from(" after"))))),
		    ]), // body "before", "apply-templates select=node()", "after"
		    Some(0.0), // priority
		    vec![0], // import
		    Some(1), // document order
		    None, // mode
		))
		.template(Template::new(
		    step(
			NodeMatch {
			    axis: Axis::SelfDocument,
			    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
			}
		    ), // pattern "/",
		    apply_templates(step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
			}
		    )), // body "apply-templates select=node()",
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.template(Template::new(
		    step(
			NodeMatch {
			    axis: Axis::Selfaxis,
			    nodetest: NodeTest::Kind(KindTest::TextTest)
			}
		    ), // pattern child::text()
		    context(), // body value-of select='.'
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.sequence(vec![Rc::new(Item::Node(sd))])
		.build();

	    // Now Evaluate the combinator with the source document root node as the context item
	    let seq = ev(&mut ctxt)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 3);
	    assert_eq!(seq.to_string(), "before content after")
	}

	#[test]
	fn tc_apply_templates_2() {
	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create new element");
	    sd.push(t.clone());
	    let c = sd.new_text(Value::from("content"))
		.expect("unable to text node");
	    t.push(c)
		.expect("unable to append child");

	    // Template rule for "Test", plus builtins
	    // Test template priorities
	    let ev = apply_templates(root::<$x>());
	    let mut ctxt = ContextBuilder::new()
		.template(Template::new(
		    step(
			NodeMatch {
			    axis: Axis::Selfaxis,
			    nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name(String::from("Test")))})
			}
		    ), // pattern "Test"
		    literal((Rc::new(Item::<$x>::Value(Value::from("priority 1 template"))))),
		    Some(1.0), // priority
		    vec![0], // import
		    Some(1), // document order
		    None, // mode
		))
		.template(Template::new(
		    step(
			NodeMatch {
			    axis: Axis::Selfaxis,
			    nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})
			}
		    ), // pattern "*"
		    literal((Rc::new(Item::<$x>::Value(Value::from("priority 0 template"))))),
		    Some(0.0), // priority
		    vec![0], // import
		    Some(2), // document order
		    None, // mode
		))
		.template(Template::new(
		    step(
			NodeMatch {
			    axis: Axis::SelfDocument,
			    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
			}
		    ), // pattern "/",
		    apply_templates(step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
			}
		    )), // body "apply-templates select=node()",
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.template(Template::new(
		    step(
			NodeMatch {
			    axis: Axis::Selfaxis,
			    nodetest: NodeTest::Kind(KindTest::TextTest)
			}
		    ), // pattern child::text()
		    context(), // body value-of select='.'
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.sequence(vec![Rc::new(Item::Node(sd))])
		.build();

	    // Now Evaluate the combinator with the source document root node as the context item
	    let seq = ev(&mut ctxt)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "priority 1 template")
	}

	#[test]
	fn tc_apply_templates_import() {
	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create new element");
	    sd.push(t.clone());
	    let c = sd.new_text(Value::from("content"))
		.expect("unable to text node");
	    t.push(c)
		.expect("unable to append child");

	    // Template rule for "Test", an overridden rule, plus builtins
	    // Test imported template
	    let ev = apply_templates(root::<$x>());
	    let mut ctxt = ContextBuilder::new()
		.template(Template::new(
		    step(
			NodeMatch {
			    axis: Axis::Selfaxis,
			    nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name(String::from("Test")))})
			}
		    ), // pattern "Test"
		    literal((Rc::new(Item::<$x>::Value(Value::from("priority 1 template, import level 1"))))),
		    Some(1.0), // priority
		    vec![0, 1], // import
		    Some(1), // document order
		    None, // mode
		))
		.template(Template::new(
		    step(
			NodeMatch {
			    axis: Axis::Selfaxis,
			    nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name(String::from("Test")))})
			}
		    ), // pattern "Test"
		    literal((Rc::new(Item::<$x>::Value(Value::from("priority 1 template, import level 0"))))),
		    Some(1.0), // priority
		    vec![0], // import
		    Some(2), // document order
		    None, // mode
		))
		.template(Template::new(
		    step(
			NodeMatch {
			    axis: Axis::Selfaxis,
			    nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})
			}
		    ), // pattern "*"
		    apply_templates(step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
			}
		    )), // body "apply-templates select=node()",
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.template(Template::new(
		    step(
			NodeMatch {
			    axis: Axis::SelfDocument,
			    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
			}
		    ), // pattern "/",
		    apply_templates(step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
			}
		    )), // body "apply-templates select=node()",
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.template(Template::new(
		    step(
			NodeMatch {
			    axis: Axis::Selfaxis,
			    nodetest: NodeTest::Kind(KindTest::TextTest)
			}
		    ), // pattern child::text()
		    context(), // body value-of select='.'
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.sequence(vec![Rc::new(Item::Node(sd))])
		.build();

	    // Now Evaluate the combinator with the source document root node as the context item
	    let seq = ev(&mut ctxt)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "priority 1 template, import level 0")
	}

	#[test]
	fn tc_apply_templates_next_match() {
	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create new element");
	    sd.push(t.clone());
	    let c = sd.new_text(Value::from("content"))
		.expect("unable to text node");
	    t.push(c)
		.expect("unable to append child");

	    // Template rule for "Test", an overridden rule, plus builtins
	    let ev = apply_templates(root::<$x>());
	    let mut ctxt = ContextBuilder::new()
		.template(Template::new(
		    step(
			NodeMatch {
			    axis: Axis::Selfaxis,
			    nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name(String::from("Test")))})
			}
		    ), // pattern "Test"
		    tc_sequence(vec![
			literal((Rc::new(Item::<$x>::Value(Value::from("priority 1 template"))))),
			next_match::<$x>(),
		    ]),
		    Some(1.0), // priority
		    vec![0], // import
		    Some(1), // document order
		    None, // mode
		))
		.template(Template::new(
		    step(
			NodeMatch {
			    axis: Axis::Selfaxis,
			    nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name(String::from("Test")))})
			}
		    ), // pattern "Test"
		    literal((Rc::new(Item::<$x>::Value(Value::from("priority 0 template"))))),
		    Some(0.0), // priority
		    vec![0], // import
		    Some(2), // document order
		    None, // mode
		))
		.template(Template::new(
		    step(
			NodeMatch {
			    axis: Axis::Selfaxis,
			    nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})
			}
		    ), // pattern "*"
		    apply_templates(step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
			}
		    )), // body "apply-templates select=node()",
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.template(Template::new(
		    step(
			NodeMatch {
			    axis: Axis::SelfDocument,
			    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
			}
		    ), // pattern "/",
		    apply_templates(step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
			}
		    )), // body "apply-templates select=node()",
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.template(Template::new(
		    step(
			NodeMatch {
			    axis: Axis::Selfaxis,
			    nodetest: NodeTest::Kind(KindTest::TextTest)
			}
		    ), // pattern child::text()
		    context(), // body value-of select='.'
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.sequence(vec![Rc::new(Item::Node(sd))])
		.build();

	    // Now Evaluate the combinator with the source document root node as the context item
	    let seq = ev(&mut ctxt)
		.expect("evaluation failed");
	    assert_eq!(seq.to_string(), "priority 1 templatepriority 0 template")
	}

	#[test]
	fn tc_position() {
	    // XPath == position()
	    // NB. rust indexes start at 0, whereas XPath positions start at 1

	    let ev = position();
	    let seq = ev(
		&mut ContextBuilder::new()
		    .sequence(vec![
			Rc::new(Item::<$x>::Value(Value::from("one"))),
			Rc::new(Item::<$x>::Value(Value::from("two"))),
			Rc::new(Item::<$x>::Value(Value::from("three"))),
			Rc::new(Item::<$x>::Value(Value::from("four"))),
		    ])
		    .index(2)
		    .build()
	    ).expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "3")
	}

	#[test]
	fn tc_last() {
	    // XPath == last()
	    // NB. rust indexes start at 0, whereas XPath positions start at 1

	    let ev = last();
	    let seq = ev(
		&mut ContextBuilder::new()
		    .sequence(vec![
			Rc::new(Item::<$x>::Value(Value::from("one"))),
			Rc::new(Item::<$x>::Value(Value::from("two"))),
			Rc::new(Item::<$x>::Value(Value::from("three"))),
			Rc::new(Item::<$x>::Value(Value::from("four"))),
		    ])
		    .index(2)
		    .build()
	    ).expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "4")
	}

	#[test]
	fn tc_count_0() {
	    // XPath == count()

	    let ev = tc_count(Vec::<Box<dyn Fn(&mut Context<$x>) -> TransResult<$x>>>::new());
	    let seq = ev(
		&mut ContextBuilder::new()
		    .sequence(vec![
			Rc::new(Item::<$x>::Value(Value::from("one"))),
			Rc::new(Item::<$x>::Value(Value::from("two"))),
			Rc::new(Item::<$x>::Value(Value::from("three"))),
			Rc::new(Item::<$x>::Value(Value::from("four"))),
		    ])
		    .index(2)
		    .build()
	    ).expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "4")
	}

	#[test]
	fn tc_count_1() {
	    // XPath == count()

	    let ev = tc_count(vec![
		tc_sequence(vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from("abc")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from(1)))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("foo")))),
		])
	    ]);
	    let seq = ev(
		&mut ContextBuilder::new()
		    .sequence(vec![
			Rc::new(Item::<$x>::Value(Value::from("one"))),
			Rc::new(Item::<$x>::Value(Value::from("two"))),
			Rc::new(Item::<$x>::Value(Value::from("three"))),
			Rc::new(Item::<$x>::Value(Value::from("four"))),
		    ])
		    .index(2)
		    .build()
	    ).expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "3")
	}

	#[test]
	fn tc_localname_0() {
	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let l1 = sd.new_element(
		QualifiedName::new(
		    Some(String::from("urn::test-example.com")),
		    Some(String::from("eg")),
		    String::from("Level-1")
		)
	    ).expect("unable to create element");
	    t.push(l1.clone())
		.expect("unable to append child");

	    let ev = local_name(Vec::<Box<dyn Fn(&mut Context<$x>) -> TransResult<$x>>>::new());

	    // Now evaluate the combinator with <Level-1> as the context item
	    let seq = ev(&mut Context::from(vec![Rc::new(Item::Node(l1))]))
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_xml(), "Level-1");
	}

	#[test]
	fn tc_name_0() {
	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let l1 = sd.new_element(
		QualifiedName::new(
		    Some(String::from("urn::test-example.com")),
		    Some(String::from("eg")),
		    String::from("Level-1")
		)
	    ).expect("unable to create element");
	    t.push(l1.clone())
		.expect("unable to append child");

	    let ev = name(Vec::<Box<dyn Fn(&mut Context<$x>) -> TransResult<$x>>>::new());

	    // Now evaluate the combinator with <Level-1> as the context item
	    let seq = ev(&mut Context::from(vec![Rc::new(Item::Node(l1))]))
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_xml(), "eg:Level-1");
	}

	#[test]
	fn tc_string() {
	    // XPath == string(1.0)
	    let ev = string(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from(1.0)))),
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "1")
	}

	#[test]
	fn tc_concat_literal() {
	    // XPath == concat("abc", 1, "foo")
	    let ev = tc_concat(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from("abc")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from(1)))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("foo")))),
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "abc1foo")
	}

	#[test]
	fn tc_starts_with_pos() {
	    // XPath == starts-with("abc", "ab")
	    let ev = starts_with(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from("abc")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("ab")))),
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), true)
	}
	#[test]
	fn tc_starts_with_neg() {
	    // XPath == starts-with("abc", "x")
	    let ev = starts_with(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from("abc")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("x")))),
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), false)
	}

	#[test]
	fn tc_contains_pos() {
	    // XPath == contains("abcd", "bc")
	    let ev = contains(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from("abcd")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("bc")))),
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), true)
	}
	#[test]
	fn tc_contains_neg() {
	    // XPath == contains("abcd", "xyz")
	    let ev = contains(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from("abcd")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("xyz")))),
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), false)
	}

	#[test]
	fn tc_substring_2args() {
	    // XPath == substring("abcd", 2)
	    let ev = substring(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from("abcd")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from(2)))),
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "bcd")
	}
	#[test]
	fn tc_substring_3args() {
	    // XPath == substring("abcd", 2, 2)
	    let ev = substring(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from("abcd")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from(2)))),
		    literal(Rc::new(Item::<$x>::Value(Value::from(2)))),
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "bc")
	}

	#[test]
	fn tc_substring_before() {
	    // XPath == substring-before("abcd", "bc")
	    let ev = substring_before(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from("abcd")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("bc")))),
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "a")
	}
	#[test]
	fn tc_substring_after() {
	    // XPath == substring-after("abcd", "bc")
	    let ev = substring_after(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from("abcd")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("bc")))),
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "d")
	}

	#[test]
	fn tc_normalize_space_1() {
	    // XPath == normalize-space(" a b  c	d\n")
	    let ev = normalize_space(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from(" a b  c	d
")))),
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "a b c d")
	}

	#[test]
	fn tc_translate_1() {
	    // XPath == translate("abcd", "bdc" "BD")
	    let ev = translate(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from("abcd")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("bdc")))),
		    literal(Rc::new(Item::<$x>::Value(Value::from("BD")))),
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "aBD")
	}

	#[test]
	fn tc_boolean_string_pos() {
	    // XPath == boolean("abcd")
	    let ev = boolean(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from("abcd")))),
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), true)
	}
	#[test]
	fn tc_boolean_string_neg() {
	    // XPath == boolean("")
	    let ev = boolean(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from("")))),
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), false)
	}
	#[test]
	fn tc_boolean_int_pos() {
	    // XPath == boolean(1)
	    let ev = boolean(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from(1)))),
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), true)
	}
	#[test]
	fn tc_boolean_int_neg() {
	    // XPath == boolean(0)
	    let ev = boolean(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from(0)))),
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), false)
	}

	#[test]
	fn tc_not_pos() {
	    // XPath == not("abcd")
	    let ev = not(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from("abcd")))),
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), false)
	}
	#[test]
	fn tc_not_neg() {
	    // XPath == not(0)
	    let ev = not(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from(0)))),
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), true)
	}

	#[test]
	fn tc_true_literal() {
	    // XPath == true()
	    let ev = tc_true::<$x>();
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), true)
	}
	#[test]
	fn tc_false_literal() {
	    // XPath == false()
	    let ev = tc_false::<$x>();
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), false)
	}

	#[test]
	fn tc_number() {
	    // XPath == number("124")
	    let ev = number(
		vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from("124")))),
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_int().unwrap(), 124)
	}

	#[test]
	fn tc_sum() {
	    // XPath == sum((1, 2, 4))
	    let ev = sum(vec![
		tc_sequence(vec![
		    literal(Rc::new(Item::<$x>::Value(Value::from(1)))),
		    literal(Rc::new(Item::<$x>::Value(Value::from(2)))),
		    literal(Rc::new(Item::<$x>::Value(Value::from(4)))),
		])
	    ]);
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_int().unwrap(), 7)
	}

	#[test]
	fn tc_floor() {
	    // XPath == floor((1.2))
	    let ev = floor(vec![
		literal(Rc::new(Item::<$x>::Value(Value::from(1.2)))),
	    ]);
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq[0].to_double(), 1.0)
	}

	#[test]
	fn tc_ceiling() {
	    // XPath == ceiling((1.2))
	    let ev = ceiling(vec![
		literal(Rc::new(Item::<$x>::Value(Value::from(1.2)))),
	    ]);
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq[0].to_double(), 2.0)
	}

	#[test]
	fn tc_round_1() {
	    // XPath == round((1.23456))
	    let ev = round(vec![
		literal(Rc::new(Item::<$x>::Value(Value::from(1.23456)))),
	    ]);
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq[0].to_double(), 1.0)
	}
	#[test]
	fn tc_round_2() {
	    // XPath == round((1.23456, 4))
	    let ev = round(vec![
		literal(Rc::new(Item::<$x>::Value(Value::from(1.23456)))),
		literal(Rc::new(Item::<$x>::Value(Value::from(4)))),
	    ]);
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert!(seq[0].to_double() - 1.2346 < 0.000001)
	}

	#[test]
	fn tc_current_date_time() {
	    // XPath == current-date-time()
	    let ev = current_date_time::<$x>();
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    match &*seq[0] {
		Item::Value(Value::DateTime(dt)) => {
		    assert_eq!(dt.year(), Local::now().year());
		    assert_eq!(dt.month(), Local::now().month());
		    assert_eq!(dt.day(), Local::now().day());
		    assert_eq!(dt.hour(), Local::now().hour());
		    assert_eq!(dt.minute(), Local::now().minute());
		    assert_eq!(dt.second(), Local::now().second()); // It is possible for this to fail if the elapsed time to execute the function call and the test falls across a second quantum
		}
		_ => panic!("not a singleton dateTime value")
	    }
	}

	#[test]
	fn tc_current_date() {
	    // XPath == current-date()
	    let ev = current_date::<$x>();
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    match &*seq[0] {
		Item::Value(Value::Date(dt)) => {
		    assert_eq!(dt.year(), Local::now().year());
		    assert_eq!(dt.month(), Local::now().month());
		    assert_eq!(dt.day(), Local::now().day());
		}
		_ => panic!("not a singleton date value")
	    }
	}

	#[test]
	fn tc_current_time() {
	    // XPath == current-time()
	    let ev = current_time::<$x>();
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    match &*seq[0] {
		Item::Value(Value::Time(dt)) => {
		    assert_eq!(dt.hour(), Local::now().hour());
		    assert_eq!(dt.minute(), Local::now().minute());
		    assert_eq!(dt.second(), Local::now().second()); // It is possible for this to fail if the elapsed time to execute the function call and the test falls across a second quantum
		}
		_ => panic!("not a singleton time value")
	    }
	}

	#[test]
	fn tc_format_date_time() {
	    // XPath == format-dateTime("2022-01-03T04:05:06.789+10:00", "[H]:[m] [D]/[M]/[Y]")
	    let ev = format_date_time(vec![
		literal(Rc::new(Item::<$x>::Value(Value::from("2022-01-03T04:05:06.789+10:00")))),
		literal(Rc::new(Item::<$x>::Value(Value::from("[H]:[m] [D]/[M]/[Y]")))),
	    ]);
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "04:05 03/01/2022")
	}

	#[test]
	fn tc_format_date() {
	    // XPath == format-date("2022-01-03", "[D]/[M]/[Y]")
	    let ev = format_date(vec![
		literal(Rc::new(Item::<$x>::Value(Value::from("2022-01-03")))),
		literal(Rc::new(Item::<$x>::Value(Value::from("[D]/[M]/[Y]")))),
	    ]);
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "03/01/2022")
	}

	#[test]
	fn tc_format_time() {
	    // XPath == format-time("04:05:06.789+10:00", "[H]:[m]")
	    let ev = format_time(vec![
		literal(Rc::new(Item::<$x>::Value(Value::from("04:05:06.789")))),
		literal(Rc::new(Item::<$x>::Value(Value::from("[H]:[m]:[s]")))),
	    ]);
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "04:05:06")
	}

	#[test]
	fn tc_func_user_defined() {
	    // foo(bar='a test'; ('this is ', $bar))
	    let ev = function_user_defined(
		tc_sequence(
		    vec![
			literal(Rc::new(Item::<$x>::Value(Value::from("this is ")))),
			reference_variable("bar".to_string()),
		    ]
		),
		vec![
		    ("bar".to_string(), literal(Rc::new(Item::<$x>::Value(Value::from("a test")))))
		]
	    );
	    let seq = ev(&mut Context::new())
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 2);
	    assert_eq!(seq.to_string(), "this is a test")
	}
    }
);
