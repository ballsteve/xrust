#[macro_export]
macro_rules! transcomb_tests (
    ( $x:ty, $y:expr ) => {
	//use std::rc::Rc;
	//use xrust::value::Value;
	//use xrust::item::{Sequence, SequenceTrait, Item};
	use xrust::evaluate::{Axis, NodeMatch, NodeTest, KindTest};
	use xrust::transcomb::{Context, ContextBuilder,
			       literal, literal_element, literal_attribute,
			       context, tc_sequence, compose, step, filter,
			       tc_or,
			       declare_variable, reference_variable,
			       function_concat,
			       function_user_defined,
	};

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
	fn tc_func_concat() {
	    // XPath == concat("abc", 1, "foo")
	    let ev = function_concat(
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
