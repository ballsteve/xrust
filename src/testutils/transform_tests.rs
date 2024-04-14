use crate::Transform;
#[macro_export]
macro_rules! transform_tests (
    ( $x:ty, $y:expr, $z:expr ) => {
	#[allow(unused_imports)]
	use chrono::{DateTime, Datelike, FixedOffset, Local, Timelike};
	use xrust::value::Operator;
	use xrust::transform::{Transform, Axis, NodeMatch, NodeTest, KindTest, NameTest, WildcardOrName, ArithmeticOperand, ArithmeticOperator, Grouping};
	use xrust::transform::template::Template;

	#[test]
	fn tr_empty() {
	    let x = Transform::<$x>::Empty;
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
			.expect("evaluation failed");
	    assert_eq!(seq.len(), 0)
	}
	#[test]
	fn tr_singleton_literal() {
	    let x = Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("this is a test"))));
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
			.expect("evaluation failed");
	    assert_eq!(seq.to_string(), "this is a test")
	}
	#[test]
	fn tr_literal_element() {
	    let x = Transform::LiteralElement(
			QualifiedName::new(None, None, String::from("Test")),
			Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("content")))))
	    );
	    let mut mydoc = $y();
	    let mut ctxt = ContextBuilder::new()
			.result_document(mydoc)
			.build();
	    let seq = ctxt.dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");
	    assert_eq!(seq.to_xml(), "<Test>content</Test>")
	}

	#[test]
	fn tr_literal_element_nested() {
	    let x = Transform::LiteralElement(
			QualifiedName::new(None, None, String::from("Test")),
			Box::new(Transform::LiteralElement(
		    	QualifiedName::new(None, None, String::from("Level-1")),
		    	Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("content")))))
			))
	    );
	    let mut mydoc = $y();
	    let mut ctxt = ContextBuilder::new()
		.result_document(mydoc)
		.build();
	    let seq = ctxt.dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");
	    assert_eq!(seq.to_xml(), "<Test><Level-1>content</Level-1></Test>")
	}

	#[test]
	fn tr_element() {
	    let x = Transform::Element(
			Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("Test"))))),
			Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("content")))))
	    );
	    let mut mydoc = $y();
	    let mut ctxt = ContextBuilder::new()
			.result_document(mydoc)
			.build();
	    let seq = ctxt.dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");
	    assert_eq!(seq.to_xml(), "<Test>content</Test>")
	}

	#[test]
	fn tr_literal_text_1() {
		let x = Transform::LiteralText(Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("special character: < less than"))))), false);
	    let mut mydoc = $y();
	    let mut ctxt = ContextBuilder::new()
			.result_document(mydoc)
			.build();
	    let seq = ctxt.dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");
	    assert_eq!(seq.to_xml(), "special character: &lt; less than")
	}
	#[test]
	fn tr_literal_text_2() {
		let x = Transform::LiteralText(Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("special character: < less than"))))), true);
	    let mut mydoc = $y();
	    let mut ctxt = ContextBuilder::new()
			.result_document(mydoc)
			.build();
	    let seq = ctxt.dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");
	    assert_eq!(seq.to_xml(), "special character: < less than")
	}

	#[test]
	fn tr_literal_attribute() {
	    let x = Transform::LiteralElement(
		QualifiedName::new(None, None, String::from("Test")),
		Box::new(Transform::SequenceItems(vec![
		    Transform::LiteralAttribute(
				QualifiedName::new(None, None, String::from("foo")),
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("bar")))))
		    ),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("content")))),
		]))
	    );
	    let mut mydoc = $y();
	    let mut ctxt = ContextBuilder::new()
		.result_document(mydoc)
		.build();
	    let seq = ctxt.dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");
	    assert_eq!(seq.to_xml(), "<Test foo='bar'>content</Test>")
	}
	#[test]
	fn tr_literal_comment() {
	    let x = Transform::LiteralElement(
		QualifiedName::new(None, None, String::from("Test")),
		Box::new(Transform::SequenceItems(vec![
		    Transform::LiteralComment(
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("bar")))))
		    ),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("content")))),
		]))
	    );
	    let mut mydoc = $y();
	    let mut ctxt = ContextBuilder::new()
		.result_document(mydoc)
		.build();
	    let seq = ctxt.dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");
	    assert_eq!(seq.to_xml(), "<Test><!--bar-->content</Test>")
	}
	#[test]
	fn tr_literal_pi() {
	    let x = Transform::LiteralElement(
		QualifiedName::new(None, None, String::from("Test")),
		Box::new(Transform::SequenceItems(vec![
		    Transform::LiteralProcessingInstruction(
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("thepi"))))),
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("bar")))))
		    ),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("content")))),
		]))
	    );
	    let mut mydoc = $y();
	    let mut ctxt = ContextBuilder::new()
		.result_document(mydoc)
		.build();
	    let seq = ctxt.dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");
	    assert_eq!(seq.to_xml(), "<Test><?thepi bar?>content</Test>")
	}
	#[test]
	fn tr_generate_id_ctxt() {
	    let x = Transform::GenerateId(None);
	    let sd = $y();
	    let mut ctxt = ContextBuilder::new()
			.context(vec![Item::Node(sd)])
			.build();
	    let seq = ctxt.dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");
	    assert!(seq.to_string().len() > 1)
	}
	#[test]
	fn tr_generate_id_2() {
	    let x1 = Transform::GenerateId(Some(Box::new(Transform::Step(
		NodeMatch {
		    axis: Axis::Child,
		    nodetest: NodeTest::Name(NameTest::new(None, None, Some(WildcardOrName::Name(String::from("Test1")))))
		}
	    ))));
	    let x2 = Transform::GenerateId(Some(Box::new(Transform::Step(
		NodeMatch {
		    axis: Axis::Child,
		    nodetest: NodeTest::Name(NameTest::new(None, None, Some(WildcardOrName::Name(String::from("Test2")))))
		}
	    ))));
	    let mut sd = $y();
	    let n1 = sd.new_element(QualifiedName::new(None, None, String::from("Test1")))
			.expect("unable to create element");
	    sd.push(n1.clone())
			.expect("unable to append child");
	    let n2 = sd.new_element(QualifiedName::new(None, None, String::from("Test2")))
			.expect("unable to create element");
	    sd.push(n2.clone())
			.expect("unable to append child");
	    let mut ctxt = ContextBuilder::new()
			.context(vec![Item::Node(sd)])
			.build();

	    let seq1 = ctxt.dispatch(&mut StaticContext::<F>::new(), &x1).expect("evaluation failed");
	    let seq2 = ctxt.dispatch(&mut StaticContext::<F>::new(), &x2).expect("evaluation failed");

	    assert!(seq1.to_string().len() > 1);
	    assert!(seq2.to_string().len() > 1);
		assert_ne!(seq1.to_string(), seq2.to_string())
	}
	#[test]
	fn tr_message_1() {
		let mut receiver = String::from("no message received");
	    let x = Transform::LiteralElement(
		QualifiedName::new(None, None, String::from("Test")),
		Box::new(Transform::SequenceItems(vec![
		    Transform::Message(
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("bar"))))),
				None,
				Box::new(Transform::Empty),
				Box::new(Transform::Empty),
		    ),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("content")))),
		]))
	    );
	    let mut mydoc = $y();
	    let mut ctxt = ContextBuilder::new()
			.result_document(mydoc)
			.build();
		let mut stctxt = StaticContextBuilder::new()
		.message(|m| {receiver = String::from(m); Ok(())})
		.build();
	    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
	    assert_eq!(seq.to_xml(), "<Test>content</Test>");
		assert_eq!(receiver, "bar")
	}
	#[test]
	fn tr_message_2() {
		let mut messages: Vec<String> = vec![];
	    let x = Transform::LiteralElement(
		QualifiedName::new(None, None, String::from("Test")),
		Box::new(Transform::SequenceItems(vec![
		    Transform::Message(
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("first message"))))),
				None,
				Box::new(Transform::Empty),
				Box::new(Transform::Empty),
		    ),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("content")))),
		    Transform::Message(
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("second message"))))),
				None,
				Box::new(Transform::Empty),
				Box::new(Transform::Empty),
		    ),
		]))
	    );
	    let mut mydoc = $y();
	    let mut ctxt = ContextBuilder::new()
			.result_document(mydoc)
			.build();
		let mut stctxt = StaticContextBuilder::new()
		.message(|m| {messages.push(String::from(m)); Ok(())})
		.build();
	    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
	    assert_eq!(seq.to_xml(), "<Test>content</Test>");
		assert_eq!(messages.len(), 2);
		assert_eq!(messages[0], "first message");
		assert_eq!(messages[1], "second message");
	}
	#[test]
	fn tr_message_term_1() {
		let mut receiver = String::from("no message received");
	    let x = Transform::LiteralElement(
		QualifiedName::new(None, None, String::from("Test")),
		Box::new(Transform::SequenceItems(vec![
		    Transform::Message(
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("bar"))))),
				None,
				Box::new(Transform::Empty),
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("yes"))))),
		    ),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("content")))),
		]))
	    );
	    let mut mydoc = $y();
	    let mut ctxt = ContextBuilder::new()
			.result_document(mydoc)
			.build();
		let mut stctxt = StaticContextBuilder::new()
		.message(|m| {receiver = String::from(m); Ok(())})
		.build();
	    match ctxt.dispatch(&mut stctxt, &x) {
			Ok(_) => panic!("evaluation succeeded when it should have failed"),
			Err(e) => {
				assert_eq!(e.kind, ErrorKind::Terminated);
				assert_eq!(e.message, "bar");
				assert_eq!(e.code.unwrap().to_string(), "XTMM9000")
			}
		}
	}
	#[test]
	fn tr_set_attribute() {
	    // Setup a source document
	    let mut sd = $y();
	    let mut n = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(n.clone())
		.expect("unable to append child");

	    let x = Transform::SetAttribute(
			QualifiedName::new(None, None, String::from("foo")),
			Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("bar"))))),
	    );
	    let mut mydoc = $y();
	    let mut ctxt = ContextBuilder::new()
		.result_document(mydoc)
		.context(vec![Item::Node(n)])
		.build();
	    let seq = ctxt.dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");
	    assert_eq!(sd.to_xml(), "<Test foo='bar'></Test>")
	}
	#[test]
	fn tr_copy_literal() {
	    let x = Transform::Copy(
			Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("this is the original"))))),
			Box::new(Transform::<$x>::Empty)
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "this is the original")
	}
	#[test]
	fn tr_copy_context_literal() {
	    let x = Transform::Copy(
			Box::new(Transform::ContextItem),
			Box::new(Transform::<$x>::Empty)
	    );
	    let seq = ContextBuilder::new()
		    .context(vec![Item::<$x>::Value(Rc::new(Value::from("this is the original")))])
		    .build()
		.dispatch(&mut StaticContext::<F>::new(), &x)
	    .expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "this is the original")
	}
	#[test]
	fn tr_copy_context_node() {
	    // Setup a source document
	    let mut sd = $y();
	    let mut n = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(n.clone())
		.expect("unable to append child");
	    n.push(sd.new_text(Rc::new(Value::from("this is the original"))).expect("unable to create text node"))
		.expect("unable to add text node");

	    let x = Transform::Copy(
			Box::new(Transform::ContextItem),
			Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("this is the copy")))))
	    );

	    let mut mydoc = $y();
	    let mut ctxt = ContextBuilder::new()
		.result_document(mydoc)
		.context(vec![Item::Node(n)])
		.build();
	    let seq = ctxt.dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");

	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_xml(), "<Test>this is the copy</Test>");
	    assert_eq!(sd.to_xml(), "<Test>this is the original</Test>")
	}

	#[test]
	fn tr_current_node() {
	    // Setup a source document
	    let mut sd = $y();
	    let mut n = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(n.clone())
		.expect("unable to append child");
	    n.push(sd.new_text(Rc::new(Value::from("this is the original"))).expect("unable to create text node"))
		.expect("unable to add text node");

	    let x = Transform::CurrentItem;

	    let mut mydoc = $y();
	    let mut ctxt = ContextBuilder::new()
		.result_document(mydoc)
		.context(vec![Item::Node(n.clone())])
		.previous_context(Item::Node(n.clone()))
		.build();
	    let seq = ctxt.dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");

	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_xml(), "<Test>this is the original</Test>")
	}

	#[test]
	fn tr_deep_copy() {
	    // Setup a source document
	    let mut sd = $y();
	    let mut n = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(n.clone())
		.expect("unable to append child");
	    let mut u = sd.new_element(QualifiedName::new(None, None, String::from("inner")))
		.expect("unable to create element");
	    n.push(u.clone())
		.expect("unable to append child");
	    u.push(sd.new_text(Rc::new(Value::from("this is the original"))).expect("unable to create text node"))
		.expect("unable to add text node");

	    let x = Transform::DeepCopy(Box::new(Transform::ContextItem));

	    let mut ctxt = ContextBuilder::new()
		.context(vec![Item::Node(n)])
		.build();
	    let seq = ctxt.dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");

	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_xml(), "<Test><inner>this is the original</inner></Test>");
	}

	#[test]
	fn tr_seq_of_literals() {
	    let x = Transform::SequenceItems(
		vec![
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("this is a test")))),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1)))),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("end of test")))),
		]
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");
	    assert_eq!(seq.len(), 3);
	    assert_eq!(seq.to_string(), "this is a test1end of test")
	}
	#[test]
	fn tr_seq_of_seqs() {
	    let x = Transform::SequenceItems(
		vec![
		    Transform::SequenceItems(
			vec![
			    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("first sequence")))),
			    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1)))),
			]
		    ),
		    Transform::SequenceItems(
			vec![
			    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("second sequence")))),
			    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(2)))),
			]
		    ),
		]
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");
	    assert_eq!(seq.len(), 4);
	    assert_eq!(seq.to_string(), "first sequence1second sequence2")
	}

	#[test]
	fn tr_switch_when() {
	    let x = Transform::Switch(
		vec![
		    (Transform::ValueComparison(
				Operator::Equal,
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1))))),
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(2.0))))),
		    	),
			Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("comparison failed"))))
			),
		    (Transform::ValueComparison(
				Operator::Equal,
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1))))),
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1.0))))),
		    	),
			Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("comparison succeeded"))))
			),
		    (Transform::ValueComparison(
				Operator::Equal,
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1))))),
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(3.0))))),
		    	),
			Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("comparison failed"))))
			),
		],
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("otherwise clause")))))
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");
	    assert_eq!(seq.to_string(), "comparison succeeded")
	}
	#[test]
	fn tr_switch_otherwise() {
	    let x = Transform::Switch(
		vec![
		    (Transform::ValueComparison(
				Operator::Equal,
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1))))),
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(2.0))))),
		    	),
			Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("comparison failed"))))),
		    (Transform::ValueComparison(
				Operator::Equal,
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1))))),
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(11.0))))),
		    	),
			Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("comparison failed"))))),
		    (Transform::ValueComparison(
				Operator::Equal,
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1))))),
				Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(3.0))))),
		    ),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("comparison failed"))))),
		],
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("otherwise clause")))))
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");
	    assert_eq!(seq.to_string(), "otherwise clause")
	}

	#[test]
	fn tr_loop_lit() {
	    let x = Transform::Loop(
		vec![
			(String::from("x"),
			Transform::SequenceItems(vec![
					Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("one")))),
					Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("two")))),
					Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("three")))),
			]))
		],
		Box::new(Transform::Concat(vec![
		    Transform::VariableReference(String::from("x")),
		    Transform::VariableReference(String::from("x")),
		]))
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");
	    assert_eq!(seq.to_string(), "oneonetwotwothreethree")
	}

	#[test]
	fn tr_context_item() {
	    let x = Transform::ContextItem;
	    let c = Context::from(vec![Item::<$x>::Value(Rc::new(Value::from("the context item")))]);
	    let seq = c.dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "the context item")
	}

	#[test]
	fn tr_context_item_seq() {
	    let x = Transform::SequenceItems(
		vec![Transform::ContextItem, Transform::ContextItem]
	    );
	    let c = Context::from(vec![Item::<$x>::Value(Rc::new(Value::from("the context item")))]);
	    let seq = c.dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");
	    assert_eq!(seq.len(), 2);
	    assert_eq!(seq.to_string(), "the context itemthe context item")
	}

	#[test]
	fn tr_root() {
	    // Setup a source document
	    let mut sd = $y();
	    let mut n = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(n.clone())
		.expect("unable to append child");
	    let l1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    n.push(l1.clone())
		.expect("unable to append child");

	    let x = Transform::Root;

	    // Now evaluate the combinator with <Level-1> as the context item
	    let seq = Context::from(vec![Item::Node(l1)])
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_xml(), "<Test><Level-1></Level-1></Test>");
	}

	#[test]
	fn tr_path_of_lits() {
	    let x = Transform::Compose(
		vec![
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("step 1")))),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("step 2"))))
		]
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x).expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "step 2")
	}

	#[test]
	fn tr_step_child_1() {
	    // XPath == child::node()
	    let x = Transform::Step(
		NodeMatch {
		    axis: Axis::Child,
		    nodetest: NodeTest::Kind(KindTest::Any)
		}
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut n = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(n.clone())
		.expect("unable to append child");
	    let l1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    n.push(l1.clone())
		.expect("unable to append child");

	    // Now evaluate the combinator with <Test> as the context item
	    let seq = Context::from(vec![Item::Node(n)])
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_xml(), "<Level-1></Level-1>");
	}

	#[test]
	fn tr_step_child_many() {
	    // XPath == child::node()
	    let x = Transform::Step(
		NodeMatch {
		    axis: Axis::Child,
		    nodetest: NodeTest::Kind(KindTest::Any)
		}
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut n = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(n.clone())
		.expect("unable to append child");
	    let mut l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    n.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Rc::new(Value::from("first")))
		.expect("unable to create text node");
	    l1_1.push(t1)
		.expect("unable to append text node");
	    let mut l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    n.push(l1_2.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Rc::new(Value::from("second")))
		.expect("unable to create text node");
	    l1_2.push(t2)
		.expect("unable to append text node");

	    // Now evaluate the combinator with both <Level-1>s as the context items
	    let seq = Context::from(
		vec![
		    Item::Node(l1_1),
		    Item::Node(l1_2),
		]
	    )
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 2);
	    assert_eq!(seq.to_xml(), "firstsecond");
	}

	#[test]
	fn tr_step_self() {
	    // XPath == child::node()
	    let x = Transform::Step(
		NodeMatch {
		    axis: Axis::SelfAxis,
		    nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name(String::from("Level-1")))})
		}
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut n = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(n.clone())
		.expect("unable to append child");
	    let l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    n.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Rc::new(Value::from("first")))
		.expect("unable to create text node");
	    n.push(t1.clone())
		.expect("unable to append text node");
	    let l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    n.push(l1_2.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Rc::new(Value::from("second")))
		.expect("unable to create text node");
	    n.push(t2.clone())
		.expect("unable to append text node");
	    let et = sd.new_element(QualifiedName::new(None, None, String::from("extra")))
		.expect("unable to create element");
	    n.push(et.clone())
		.expect("unable to append child");

	    // Now evaluate the combinator with Test's children as the context items
	    let seq = Context::from(
		vec![
		    Item::Node(l1_1),
		    Item::Node(t1),
		    Item::Node(l1_2),
		    Item::Node(t2),
		    Item::Node(et),
		]
	    )
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 2);
	    assert_eq!(seq.to_xml(), "<Level-1></Level-1><Level-1></Level-1>");
	}

	#[test]
	fn tr_step_selfdoc_pos() {
	    let x = Transform::Step(
		NodeMatch {
		    axis: Axis::SelfDocument,
		    nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name(String::from("Level-1")))})
		}
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut n = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(n.clone())
		.expect("unable to append child");
	    let l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    n.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Rc::new(Value::from("first")))
		.expect("unable to create text node");
	    n.push(t1.clone())
		.expect("unable to append text node");
	    let l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    n.push(l1_2.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Rc::new(Value::from("second")))
		.expect("unable to create text node");
	    n.push(t2.clone())
		.expect("unable to append text node");
	    let et = sd.new_element(QualifiedName::new(None, None, String::from("extra")))
		.expect("unable to create element");
	    n.push(et.clone())
		.expect("unable to append child");

	    // Now evaluate the combinator with Test's document node as the context items
	    let seq = Context::from(
		vec![Item::Node(sd)]
	    )
		.dispatch(&mut StaticContext::<F>::new(), &x)
        .expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	}
	#[test]
	fn tr_step_selfdoc_neg() {
	    let x = Transform::Step(
		NodeMatch {
		    axis: Axis::SelfDocument,
		    nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name(String::from("Level-1")))})
		}
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut n = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(n.clone())
		.expect("unable to append child");
	    let l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    n.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Rc::new(Value::from("first")))
		.expect("unable to create text node");
	    n.push(t1.clone())
		.expect("unable to append text node");
	    let l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    n.push(l1_2.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Rc::new(Value::from("second")))
		.expect("unable to create text node");
	    n.push(t2.clone())
		.expect("unable to append text node");
	    let et = sd.new_element(QualifiedName::new(None, None, String::from("extra")))
		.expect("unable to create element");
	    n.push(et.clone())
		.expect("unable to append child");

	    // Now evaluate the combinator with Test's document element node as the context items
	    let seq = Context::from(
		vec![Item::Node(n)]
	    )
        .dispatch(&mut StaticContext::<F>::new(), &x)
        .expect("evaluation failed");
	    assert_eq!(seq.len(), 0);
	}

	#[test]
	fn tr_step_parent() {
	    // XPath == parent::*
	    let x = Transform::Step(
		NodeMatch {
		    axis: Axis::Parent,
		    nodetest: NodeTest::Kind(KindTest::Any)
		}
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut n = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(n.clone())
		.expect("unable to append child");
	    let l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    n.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Rc::new(Value::from("first")))
		.expect("unable to create text node");
	    n.push(t1.clone())
		.expect("unable to append text node");
	    let l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    n.push(l1_2.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Rc::new(Value::from("second")))
		.expect("unable to create text node");
	    n.push(t2.clone())
		.expect("unable to append text node");
	    let et = sd.new_element(QualifiedName::new(None, None, String::from("extra")))
		.expect("unable to create element");
	    n.push(et.clone())
		.expect("unable to append child");

	    // Now evaluate the combinator with Test's children as the context items
	    let seq = Context::from(
		vec![
		    Item::Node(l1_1),
		    Item::Node(t1),
		    Item::Node(l1_2),
		    Item::Node(t2),
		    Item::Node(et),
		]
	    )
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq[0].name().to_string(), "Test");
	}

	#[test]
	fn tr_step_parentdoc_pos() {
	    let x = Transform::Step(
		NodeMatch {
		    axis: Axis::ParentDocument,
		    nodetest: NodeTest::Kind(KindTest::Any)
		}
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut n = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(n.clone())
		.expect("unable to append child");
	    let l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    n.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Rc::new(Value::from("first")))
		.expect("unable to create text node");
	    n.push(t1.clone())
		.expect("unable to append text node");

	    // Now evaluate the combinator with the root node as the context items
	    let seq = Context::from(
		vec![
		    Item::Node(sd),
		]
	    )
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	}
	#[test]
	fn tr_step_parentdoc_neg() {
	    let x = Transform::Step(
		NodeMatch {
		    axis: Axis::ParentDocument,
		    nodetest: NodeTest::Kind(KindTest::Any)
		}
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Rc::new(Value::from("first")))
		.expect("unable to create text node");
	    t.push(t1.clone())
		.expect("unable to append text node");

	    // Now evaluate the combinator with the document element as the context items
	    let seq = Context::from(
		vec![
		    Item::Node(t),
		]
	    )
        .dispatch(&mut StaticContext::<F>::new(), &x)
        .expect("evaluation failed");
	    assert_eq!(seq.len(), 0);
	}

	#[test]
	fn tr_step_descendant() {
	    let x = Transform::Step(
		NodeMatch {
		    axis: Axis::Descendant,
		    nodetest: NodeTest::Kind(KindTest::Any)
		}
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Rc::new(Value::from("first")))
		.expect("unable to create text node");
	    t.push(t1.clone())
		.expect("unable to append text node");
	    let mut l2_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-2")))
		.expect("unable to create element");
	    l1_1.push(l2_1.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Rc::new(Value::from("second")))
		.expect("unable to create text node");
	    l2_1.push(t2.clone())
		.expect("unable to append text node");

	    // Now evaluate the combinator with the document element as the context items
	    let seq = Context::from(
		vec![
		    Item::Node(t),
		]
	    )
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 4);
	}

	#[test]
	fn tr_step_descendant_or_self() {
	    let x = Transform::Step(
		NodeMatch {
		    axis: Axis::DescendantOrSelf,
		    nodetest: NodeTest::Kind(KindTest::Any)
		}
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Rc::new(Value::from("first")))
		.expect("unable to create text node");
	    t.push(t1.clone())
		.expect("unable to append text node");
	    let mut l2_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-2")))
		.expect("unable to create element");
	    l1_1.push(l2_1.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Rc::new(Value::from("second")))
		.expect("unable to create text node");
	    l2_1.push(t2.clone())
		.expect("unable to append text node");

	    // Now evaluate the combinator with the document element as the context item
	    let seq = Context::from(
		vec![
		    Item::Node(t),
		]
	    )
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 5);
	}

	#[test]
	fn tr_step_descendant_or_self_or_root() {
	    let x = Transform::Step(
		NodeMatch {
		    axis: Axis::DescendantOrSelfOrRoot,
		    nodetest: NodeTest::Kind(KindTest::Any)
		}
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Rc::new(Value::from("first")))
		.expect("unable to create text node");
	    t.push(t1.clone())
		.expect("unable to append text node");
	    let mut l2_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-2")))
		.expect("unable to create element");
	    l1_1.push(l2_1.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Rc::new(Value::from("second")))
		.expect("unable to create text node");
	    l2_1.push(t2.clone())
		.expect("unable to append text node");

	    // Now evaluate the combinator with the root node as the context item
	    let seq = Context::from(
		vec![
		    Item::Node(sd),
		]
	    )
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 6);
	}

	#[test]
	fn tr_step_ancestor() {
	    let x = Transform::Step(
		NodeMatch {
		    axis: Axis::Ancestor,
		    nodetest: NodeTest::Kind(KindTest::Any)
		}
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Rc::new(Value::from("first")))
		.expect("unable to create text node");
	    t.push(t1.clone())
		.expect("unable to append text node");
	    let mut l2_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-2")))
		.expect("unable to create element");
	    l1_1.push(l2_1.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Rc::new(Value::from("second")))
		.expect("unable to create text node");
	    l2_1.push(t2.clone())
		.expect("unable to append text node");

	    // Now evaluate the combinator with the lowest node as the context item
	    let seq = Context::from(
		vec![
		    Item::Node(t2),
		]
	    )
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 3);
	}

	#[test]
	fn tr_step_ancestor_or_self() {
	    let x = Transform::Step(
		NodeMatch {
		    axis: Axis::AncestorOrSelf,
		    nodetest: NodeTest::Kind(KindTest::Any)
		}
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Rc::new(Value::from("first")))
		.expect("unable to create text node");
	    t.push(t1.clone())
		.expect("unable to append text node");
	    let mut l2_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-2")))
		.expect("unable to create element");
	    l1_1.push(l2_1.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Rc::new(Value::from("second")))
		.expect("unable to create text node");
	    l2_1.push(t2.clone())
		.expect("unable to append text node");

	    // Now evaluate the combinator with the lowest node as the context item
	    let seq = Context::from(
		vec![
		    Item::Node(t2),
		]
	    )
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 4);
	}

	#[test]
	fn tr_step_following_sibling() {
	    // XPath == following-sibling::node()
	    let x = Transform::Step(
		NodeMatch {
		    axis: Axis::FollowingSibling,
		    nodetest: NodeTest::Kind(KindTest::Any)
		}
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Rc::new(Value::from("first")))
		.expect("unable to create text node");
	    t.push(t1.clone())
		.expect("unable to append text node");
	    let l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_2.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Rc::new(Value::from("second")))
		.expect("unable to create text node");
	    t.push(t2.clone())
		.expect("unable to append text node");
	    let et = sd.new_element(QualifiedName::new(None, None, String::from("extra")))
		.expect("unable to create element");
	    t.push(et.clone())
		.expect("unable to append child");

	    // Now evaluate the combinator with Test's first child as the context items
	    let seq = Context::from(
		vec![
		    Item::Node(l1_1),
		]
	    )
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 4);
	}

	#[test]
	fn tr_step_preceding_sibling() {
	    // XPath == preceding-sibling::node()
	    let x = Transform::Step(
		NodeMatch {
		    axis: Axis::PrecedingSibling,
		    nodetest: NodeTest::Kind(KindTest::Any)
		}
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Rc::new(Value::from("first")))
		.expect("unable to create text node");
	    t.push(t1.clone())
		.expect("unable to append text node");
	    let l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_2.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Rc::new(Value::from("second")))
		.expect("unable to create text node");
	    t.push(t2.clone())
		.expect("unable to append text node");
	    let et = sd.new_element(QualifiedName::new(None, None, String::from("extra")))
		.expect("unable to create element");
	    t.push(et.clone())
		.expect("unable to append child");

	    // Now evaluate the combinator with Test's last child as the context items
	    let seq = Context::from(
		vec![
		    Item::Node(et),
		]
	    )
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 4);
	}

	#[test]
	fn tr_step_following() {
	    // XPath == following::node()
	    let x = Transform::Step(
		NodeMatch {
		    axis: Axis::Following,
		    nodetest: NodeTest::Kind(KindTest::Any)
		}
	    );

	    // Setup a source document
	    let mut sd = $y();
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
	    let seq = Context::from(
		vec![
		    Item::Node(seven),
		]
	    )
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 4);
	}

	#[test]
	fn tr_step_preceding() {
	    // XPath == preceding::node()
	    let x = Transform::Step(
		NodeMatch {
		    axis: Axis::Preceding,
		    nodetest: NodeTest::Kind(KindTest::Any)
		}
	    );

	    // Setup a source document
	    let mut sd = $y();
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
	    let seq = Context::from(
		vec![
		    Item::Node(six),
		]
	    )
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 6);
	}

	#[test]
	fn tr_path_step_child() {
	    // XPath == child::node()/child::node()
	    let x = Transform::Compose(
		vec![
		    Transform::Step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::Any)
			}
		    ),
		    Transform::Step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::Any)
			}
		    ),
		]
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Rc::new(Value::from("first")))
		.expect("unable to create text node");
	    l1_1.push(t1)
		.expect("unable to append text node");
	    let mut l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_2.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Rc::new(Value::from("second")))
		.expect("unable to create text node");
	    l1_2.push(t2)
		.expect("unable to append text node");

	    // Now evaluate the combinator with the Test element as the context item
	    let seq = Context::from(
		vec![
		    Item::Node(t),
		]
	    )
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 2);
	    assert_eq!(seq.to_xml(), "firstsecond");
	}

	#[test]
	fn tr_step_attribute() {
	    // XPath == child::node()/attribute::*
	    let x = Transform::Compose(
		vec![
		    Transform::Step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::Any)
			}
		    ),
		    Transform::Step(
			NodeMatch {
			    axis: Axis::Attribute,
			    nodetest: NodeTest::Name(NameTest{name: Some(WildcardOrName::Wildcard), ns: None, prefix: None})
			}
		    ),
		]
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Rc::new(Value::from("one")))
		.expect("unable to create text node");
	    l1_1.push(t1)
		.expect("unable to append text node");
	    let a1 = sd.new_attribute(QualifiedName::new(None, None, String::from("name")), Rc::new(Value::from("first")))
		.expect("unable to create attribute node");
	    l1_1.add_attribute(a1)
		.expect("unable to add attribute node");
	    let mut l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_2.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Rc::new(Value::from("two")))
		.expect("unable to create text node");
	    l1_2.push(t2)
		.expect("unable to append text node");
	    let a2 = sd.new_attribute(QualifiedName::new(None, None, String::from("name")), Rc::new(Value::from("second")))
		.expect("unable to create attribute node");
	    l1_2.add_attribute(a2)
		.expect("unable to add attribute node");

	    // Now evaluate the combinator with the Test element as the context item
	    let seq = Context::from(
		vec![
		    Item::Node(t),
		]
	    )
		.dispatch(&mut StaticContext::<F>::new(), &x)
        .expect("evaluation failed");
	    assert_eq!(seq.len(), 2);
	    assert_eq!(seq.to_string(), "firstsecond");
	}
	#[test]
	fn tr_step_self_attribute_pos() {
	    let x = Transform::Step(
		NodeMatch {
		    axis: Axis::SelfAttribute,
		    nodetest: NodeTest::Kind(KindTest::Any)
		}
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Rc::new(Value::from("one")))
		.expect("unable to create text node");
	    l1_1.push(t1)
		.expect("unable to append text node");
	    let a1 = sd.new_attribute(QualifiedName::new(None, None, String::from("name")), Rc::new(Value::from("first")))
		.expect("unable to create attribute node");
	    l1_1.add_attribute(a1)
		.expect("unable to add attribute node");
	    let mut l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_2.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Rc::new(Value::from("two")))
		.expect("unable to create text node");
	    l1_2.push(t2)
		.expect("unable to append text node");
	    let a2 = sd.new_attribute(QualifiedName::new(None, None, String::from("name")), Rc::new(Value::from("second")))
		.expect("unable to create attribute node");
	    l1_2.add_attribute(a2.clone())
		.expect("unable to add attribute node");

	    // Now evaluate the combinator with an attribute as the context item
	    let seq = Context::from(
		vec![
		    Item::Node(a2),
		]
	    )
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "second");
	}
	#[test]
	fn tr_step_self_attribute_neg() {
	    let x = Transform::Step(
		NodeMatch {
		    axis: Axis::SelfAttribute,
		    nodetest: NodeTest::Kind(KindTest::Any)
		}
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Rc::new(Value::from("one")))
		.expect("unable to create text node");
	    l1_1.push(t1)
		.expect("unable to append text node");
	    let a1 = sd.new_attribute(QualifiedName::new(None, None, String::from("name")), Rc::new(Value::from("first")))
		.expect("unable to create attribute node");
	    l1_1.add_attribute(a1)
		.expect("unable to add attribute node");
	    let mut l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_2.clone())
		.expect("unable to append child");
	    let t2 = sd.new_text(Rc::new(Value::from("two")))
		.expect("unable to create text node");
	    l1_2.push(t2)
		.expect("unable to append text node");
	    let a2 = sd.new_attribute(QualifiedName::new(None, None, String::from("name")), Rc::new(Value::from("second")))
		.expect("unable to create attribute node");
	    l1_2.add_attribute(a2.clone())
		.expect("unable to add attribute node");

	    // Now evaluate the combinator with an element as the context item
	    let seq = Context::from(
		vec![
		    Item::Node(l1_2),
		]
	    )
        .dispatch(&mut StaticContext::<F>::new(), &x)
        .expect("evaluation failed");
	    assert_eq!(seq.len(), 0);
	}

	#[test]
	fn tr_predicate() {
	    // XPath == child::node()[child::node()]
	    let x = Transform::Compose(
		vec![
		    Transform::Step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::Any)
			}
		    ),
		    Transform::Filter(Box::new(
			Transform::Step(
			    NodeMatch {
				axis: Axis::Child,
				nodetest: NodeTest::Kind(KindTest::Any)
			    }
			)
		    )),
		]
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1_1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1_1.clone())
		.expect("unable to append child");
	    let t1 = sd.new_text(Rc::new(Value::from("first")))
		.expect("unable to create text node");
	    l1_1.push(t1)
		.expect("unable to append text node");
	    let l1_2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-2")))
		.expect("unable to create element");
	    t.push(l1_2.clone())
		.expect("unable to append child");

	    // Now evaluate the combinator with the Test element as the context item
	    let seq = Context::from(
		vec![
		    Item::Node(t),
		]
	    )
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_xml(), "<Level-1>first</Level-1>");
	}

	#[test]
	fn tr_or_true() {
	    let x = Transform::Or(vec![
		Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(0)))),
		Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("false")))),
	    ]);
	    let seq = Context::new()
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), true)
	}
	#[test]
	fn tr_or_false() {
	    let x = Transform::Or(vec![
		Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(0)))),
	    ]);
	    let seq = Context::new()
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), false)
	}

	#[test]
	fn tr_and_true() {
	    let x = Transform::And(vec![
		Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1)))),
		Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("false")))),
	    ]);
	    let seq = Context::new()
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), true)
	}
	#[test]
	fn tr_and_false() {
	    let x = Transform::And(vec![
		Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("true")))),
		Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(0)))),
	    ]);
	    let seq = Context::new()
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), false)
	}

	#[test]
	fn tr_general_compare_true() {
	    let x = Transform::GeneralComparison(
		Operator::Equal,
		Box::new(Transform::SequenceItems(vec![
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("true")))),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("false")))),
		])),
		Box::new(Transform::SequenceItems(vec![
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(0)))),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("true")))),
		])),
	    );
	    let seq = Context::new()
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), true)
	}
	#[test]
	fn tr_general_compare_false() {
	    let x = Transform::GeneralComparison(
		Operator::Equal,
		Box::new(Transform::SequenceItems(vec![
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("true")))),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("false")))),
		])),
		Box::new(Transform::SequenceItems(vec![
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1)))),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("foo")))),
		])),
	    );
	    let seq = Context::new()
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), false)
	}

	#[test]
	fn tr_value_compare_true() {
	    let x = Transform::ValueComparison(
		Operator::Equal,
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("true"))))),
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("true"))))),
	    );
	    let seq = Context::new()
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), true)
	}

	#[test]
	fn tr_value_compare_false() {
	    let x = Transform::ValueComparison(
		Operator::Equal,
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("true"))))),
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("false"))))),
	    );
	    let seq = Context::new()
        .dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), false)
	}

	#[test]
	fn tr_range_empty() {
	    let x = Transform::Range(
		Box::new(Transform::<$x>::Empty),
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(10))))),
	    );
	    let seq = Context::new()
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 0);
	}

	#[test]
	fn tr_range_many() {
	    let x = Transform::Range(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1))))),
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(10))))),
	    );
	    let seq = Context::new()
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 10);
	    assert_eq!(seq.to_string(), "12345678910");
	}
	#[test]
	fn tr_range_one() {
	    let x = Transform::Range(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(5))))),
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(5))))),
	    );
	    let seq = Context::new()
        .dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "5");
	}

	#[test]
	fn tr_arithmetic_add() {
	    let x = Transform::Arithmetic(vec![
		ArithmeticOperand::new(ArithmeticOperator::Noop, Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(5))))),
		ArithmeticOperand::new(ArithmeticOperator::Add, Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(5))))),
	    ]);
	    let seq = Context::new()
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "10");
	}

	#[test]
	fn tr_var_declare() {
	    let x = Transform::VariableDeclaration(
			"foo".to_string(),
			Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("bar"))))),
			Box::new(Transform::VariableReference("foo".to_string())),
	    );
	    let seq = Context::new()
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "bar")
	}

	#[test]
	fn tr_union() {
	    // XPath == child::a|child::b
	    let x = Transform::Union(
		vec![
		    Transform::Step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name(String::from("a")))})
			}
		    ),
		    Transform::Step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name(String::from("b")))})
			}
		    ),
		]
	    );

	    // Setup a source document
	    let mut sd = $y();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut a = sd.new_element(QualifiedName::new(None, None, String::from("a")))
		.expect("unable to create element");
	    t.push(a.clone())
		.expect("unable to append child");
	    let t_a = sd.new_text(Rc::new(Value::from("first")))
		.expect("unable to create text node");
	    a.push(t_a)
		.expect("unable to append text node");
	    let mut b = sd.new_element(QualifiedName::new(None, None, String::from("b")))
		.expect("unable to create element");
	    t.push(b.clone())
		.expect("unable to append child");
	    let t_b = sd.new_text(Rc::new(Value::from("second")))
		.expect("unable to create text node");
	    b.push(t_b)
		.expect("unable to append text node");
	    let mut c = sd.new_element(QualifiedName::new(None, None, String::from("c")))
		.expect("unable to create element");
	    t.push(c.clone())
		.expect("unable to append child");
	    let t_c = sd.new_text(Rc::new(Value::from("third")))
		.expect("unable to create text node");
	    c.push(t_c)
		.expect("unable to append text node");

	    // Now evaluate the combinator with the Test element as the context item
	    let seq = Context::from(
		vec![
		    Item::Node(t),
		]
	    )
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 2);
	    assert_eq!(seq.to_xml(), "<a>first</a><b>second</b>");
	}

	#[test]
	fn tr_for_each() {
	    // Setup a source document
	    let mut sd = $y();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to element node");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1 = sd.new_element(QualifiedName::new(None, None, String::from("Level1")))
		.expect("unable to element node");
	    t.push(l1.clone())
		.expect("unable to append child");
	    l1.push(sd.new_text(Rc::new(Value::from("one"))).expect("unable to create text node"))
		.expect("unable to append text");
	    let mut l2 = sd.new_element(QualifiedName::new(None, None, String::from("Level1")))
		.expect("unable to element node");
	    t.push(l2.clone())
		.expect("unable to append child");
	    l2.push(sd.new_text(Rc::new(Value::from("two"))).expect("unable to create text node"))
		.expect("unable to append text");
	    let mut l3 = sd.new_element(QualifiedName::new(None, None, String::from("Level1")))
		.expect("unable to element node");
	    t.push(l3.clone())
		.expect("unable to append child");
	    l3.push(sd.new_text(Rc::new(Value::from("three"))).expect("unable to create text node"))
		.expect("unable to append text");

	    // xsl:for-each select="/child::* /child::*" body == xsl:text "found a Level-1"
	    let x = Transform::ForEach(None,
		Box::new(Transform::Compose(vec![
		    Transform::Root,
		    Transform::Step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::Any)
			}
		    ),
		    Transform::Step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::Any)
			}
		    ),
		])),
		Box::new(Transform::Literal(Item::Value(Rc::new(Value::from("found a Level-1"))))),
	    );

	    let seq = ContextBuilder::new()
			 .context(vec![Item::Node(sd)])
			 .build()
		.dispatch(&mut StaticContext::<F>::new(), &x)
	    .expect("evaluation failed");
	    assert_eq!(seq.len(), 3);
	    assert_eq!(seq.to_string(), "found a Level-1found a Level-1found a Level-1")
	}

	#[test]
	fn tr_group_by_1() {
	    // xsl:for-each-group select="1 to 50" group-by=". mod 10" body == xsl:text "group current-grouping-key size count(current-group)"
	    let x = Transform::ForEach(
			Some(Grouping::By(vec![Transform::Arithmetic(vec![
		    ArithmeticOperand::new(ArithmeticOperator::Noop, Transform::ContextItem),
		    ArithmeticOperand::new(ArithmeticOperator::Modulo, Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(10)))))
		])])),
		Box::new(Transform::Range(
		    Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1))))),
		    Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(50))))),
		)),
		Box::new(Transform::LiteralElement(
		    QualifiedName::new(None, None, String::from("group")),
		    Box::new(Transform::SequenceItems(vec![
			Transform::Literal(Item::Value(Rc::new(Value::from("key ")))),
			Transform::CurrentGroupingKey,
			Transform::Literal(Item::Value(Rc::new(Value::from(" #members ")))),
			Transform::Count(Box::new(Transform::CurrentGroup)),
		    ]))
		))
	    );

	    let mut resdoc = $y();
	    let seq = ContextBuilder::new()
			 .result_document(resdoc)
			 .build()
		.dispatch(&mut StaticContext::<F>::new(), &x)
	    .expect("evaluation failed");
	    assert_eq!(seq.len(), 10);
	    // the groups are not ordered, so it is difficult to test all of the groups are correct
	    //assert_eq!(seq[0].to_string(), "key 0 #members 10")
	}

	#[test]
	fn tr_group_adjacent_1() {
	    // xsl:for-each-group select="(a, a, b, c, c, c)" group-adjacent="." body == xsl:text "group current-grouping-key size count(current-group)"
	    let x = Transform::ForEach(
			Some(Grouping::Adjacent(vec![Transform::ContextItem])),
		Box::new(Transform::SequenceItems(vec![
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("a")))),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("a")))),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("b")))),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("c")))),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("c")))),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("c")))),
		])),
		Box::new(Transform::LiteralElement(
		    QualifiedName::new(None, None, String::from("group")),
		    Box::new(Transform::SequenceItems(vec![
			Transform::Literal(Item::Value(Rc::new(Value::from("key ")))),
			Transform::CurrentGroupingKey,
			Transform::Literal(Item::Value(Rc::new(Value::from(" #members ")))),
			Transform::Count(Box::new(Transform::CurrentGroup)),
		    ]))
		))
	    );

	    let mut resdoc = $y();
	    let seq = ContextBuilder::new()
			 .result_document(resdoc)
			 .build()
		.dispatch(&mut StaticContext::<F>::new(), &x)
	    .expect("evaluation failed");
	    assert_eq!(seq.len(), 3);
	    // the groups are not ordered, so it is difficult to test all of the groups are correct
	    //assert_eq!(seq[0].to_string(), "key 0 #members 10")
	}

	#[test]
	fn tr_apply_templates_builtins() {
	    // Setup a source document
	    let mut sd = $y();
	    let mut t = sd.new_text(Rc::new(Value::from("Test")))
		.expect("unable to text node");
	    sd.push(t.clone())
		.expect("unable to append child");

	    // Built-in template rule for "/"
	    let x = Transform::ApplyTemplates(Box::new(Transform::Root));
	    let ctxt = ContextBuilder::new()
		.template(Template::new(
		    // pattern "/",
			Pattern::try_from("/").expect("unable to create Pattern for \"/\""),
		    Transform::ApplyTemplates(Box::new(Transform::Step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::Any)
			}
		    ))), // body "apply-templates select=node()",
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.template(Template::new(
		    // pattern child::text()
			Pattern::try_from("child::text()").expect("unable to create Pattern for \"child::text()\""),
		    Transform::ContextItem, // body value-of select='.'
		    None,
		    vec![0],
		    None,
		    None,
		))
		.context(vec![Item::Node(sd)])
		.build();

	    // Now Evaluate the combinator with the source document root node as the context item
	    let seq = ctxt.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "Test")
	}

	#[test]
	fn tr_apply_templates_1() {
	    // Setup a source document
	    let mut sd = $y();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create new element");
	    sd.push(t.clone());
	    let c = sd.new_text(Rc::new(Value::from("content")))
		.expect("unable to text node");
	    t.push(c)
		.expect("unable to append child");

	    // Template rule for "Test", plus builtins
	    let x = Transform::ApplyTemplates(Box::new(Transform::Root));
	    let ctxt = ContextBuilder::new()
		.template(Template::new(
		    // pattern "Test"
			Pattern::try_from("child::Test").expect("unable to create Pattern for \"child::Test\""),
		    Transform::SequenceItems(vec![
			Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("before ")))),
			Transform::ApplyTemplates(Box::new(Transform::Step(
			    NodeMatch {
				axis: Axis::Child,
				nodetest: NodeTest::Kind(KindTest::Any)
			    }
			))),
			Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(" after")))),
		    ]), // body "before", "apply-templates select=node()", "after"
		    Some(0.0), // priority
		    vec![0], // import
		    Some(1), // document order
		    None, // mode
		))
		.template(Template::new(
		    // pattern "/",
			Pattern::try_from("/").expect("unable to create Pattern for \"/\""),
		    Transform::ApplyTemplates(Box::new(Transform::Step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::Any)
			}
		    ))), // body "apply-templates select=node()",
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.template(Template::new(
		    // pattern child::text()
			Pattern::try_from("child::text()").expect("unable to create Pattern for \"child::text()\""),
		    Transform::ContextItem, // body value-of select='.'
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.context(vec![Item::Node(sd)])
		.build();

	    // Now Evaluate the combinator with the source document root node as the context item
	    let seq = ctxt.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 3);
	    assert_eq!(seq.to_string(), "before content after")
	}

	#[test]
	fn tr_apply_templates_2() {
	    // Setup a source document
	    let mut sd = $y();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create new element");
	    sd.push(t.clone());
	    let c = sd.new_text(Rc::new(Value::from("content")))
		.expect("unable to text node");
	    t.push(c)
		.expect("unable to append child");

	    // Template rule for "Test", plus builtins
	    // Test template priorities
	    let x = Transform::ApplyTemplates(Box::new(Transform::Root));
	    let ctxt = ContextBuilder::new()
		.template(Template::new(
		    // pattern "Test"
			Pattern::try_from("child::Test").expect("unable to create Pattern for \"child::Test\""),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("priority 1 template")))),
		    Some(1.0), // priority
		    vec![0], // import
		    Some(1), // document order
		    None, // mode
		))
		.template(Template::new(
		    // pattern "*"
			Pattern::try_from("child::*").expect("unable to create Pattern for \"child::*\""),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("priority 0 template")))),
		    Some(0.0), // priority
		    vec![0], // import
		    Some(2), // document order
		    None, // mode
		))
		.template(Template::new(
		    // pattern "/",
			Pattern::try_from("/").expect("unable to create Pattern for \"/\""),
		    Transform::ApplyTemplates(Box::new(Transform::Step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::Any)
			}
		    ))), // body "apply-templates select=node()",
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.template(Template::new(
		    // pattern child::text()
			Pattern::try_from("child::text()").expect("unable to create Pattern for \"child::text()\""),
		    Transform::ContextItem, // body value-of select='.'
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.context(vec![Item::Node(sd)])
		.build();

	    // Now Evaluate the combinator with the source document root node as the context item
	    let seq = ctxt.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "priority 1 template")
	}

	#[test]
	fn tr_apply_templates_import() {
	    // Setup a source document
	    let mut sd = $y();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create new element");
	    sd.push(t.clone());
	    let c = sd.new_text(Rc::new(Value::from("content")))
		.expect("unable to text node");
	    t.push(c)
		.expect("unable to append child");

	    // Template rule for "Test", an overridden rule, plus builtins
	    // Test imported template
	    let x = Transform::ApplyTemplates(Box::new(Transform::Root));
	    let ctxt = ContextBuilder::new()
		.template(Template::new(
		    // pattern "Test"
			Pattern::try_from("child::Test").expect("unable to create Pattern for \"child::Test\""),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("priority 1 template, import level 1")))),
		    Some(1.0), // priority
		    vec![0, 1], // import
		    Some(1), // document order
		    None, // mode
		))
		.template(Template::new(
		    // pattern "Test"
			Pattern::try_from("child::Test").expect("unable to create Pattern for \"child::Test\""),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("priority 1 template, import level 0")))),
		    Some(1.0), // priority
		    vec![0], // import
		    Some(2), // document order
		    None, // mode
		))
		.template(Template::new(
		    // pattern "*"
			Pattern::try_from("child::*").expect("unable to create Pattern for \"child::*\""),
		    Transform::ApplyTemplates(Box::new(Transform::Step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::Any)
			}
		    ))), // body "apply-templates select=node()",
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.template(Template::new(
		    // pattern "/",
			Pattern::try_from("/").expect("unable to create Pattern for \"/\""),
		    Transform::ApplyTemplates(Box::new(Transform::Step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::Any)
			}
		    ))), // body "apply-templates select=node()",
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.template(Template::new(
		    // pattern child::text()
			Pattern::try_from("child::text()").expect("unable to create Pattern for \"child::text()\""),
		    Transform::ContextItem, // body value-of select='.'
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.context(vec![Item::Node(sd)])
		.build();

	    // Now Evaluate the combinator with the source document root node as the context item
	    let seq = ctxt.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "priority 1 template, import level 0")
	}

	#[test]
	fn tr_apply_templates_next_match() {
	    // Setup a source document
	    let mut sd = $y();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create new element");
	    sd.push(t.clone());
	    let c = sd.new_text(Rc::new(Value::from("content")))
		.expect("unable to text node");
	    t.push(c)
		.expect("unable to append child");

	    // Template rule for "Test", an overridden rule, plus builtins
	    let x = Transform::ApplyTemplates(Box::new(Transform::Root));
	    let ctxt = ContextBuilder::new()
		.template(Template::new(
		    // pattern "Test"
			Pattern::try_from("child::Test").expect("unable to create Pattern for \"child::Test\""),
		    Transform::SequenceItems(vec![
			Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("priority 1 template")))),
			Transform::NextMatch,
		    ]),
		    Some(1.0), // priority
		    vec![0], // import
		    Some(1), // document order
		    None, // mode
		))
		.template(Template::new(
		    // pattern "Test"
			Pattern::try_from("child::Test").expect("unable to create Pattern for \"child::Test\""),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("priority 0 template")))),
		    Some(0.0), // priority
		    vec![0], // import
		    Some(2), // document order
		    None, // mode
		))
		.template(Template::new(
		    // pattern "*"
			Pattern::try_from("child::*").expect("unable to create Pattern for \"child::*\""),
		    Transform::ApplyTemplates(Box::new(Transform::Step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::Any)
			}
		    ))), // body "apply-templates select=node()",
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.template(Template::new(
		    // pattern "/",
			Pattern::try_from("/").expect("unable to create Pattern for \"/\""),
		    Transform::ApplyTemplates(Box::new(Transform::Step(
			NodeMatch {
			    axis: Axis::Child,
			    nodetest: NodeTest::Kind(KindTest::Any)
			}
		    ))), // body "apply-templates select=node()",
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.template(Template::new(
		    // pattern child::text()
			Pattern::try_from("child::text()").expect("unable to create Pattern for \"child::text()\""),
		    Transform::ContextItem, // body value-of select='.'
		    None, // priority
		    vec![0], // import
		    None, // document order
		    None, // mode
		))
		.context(vec![Item::Node(sd)])
		.build();

	    // Now Evaluate the combinator with the source document root node as the context item
	    let seq = ctxt.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.to_string(), "priority 1 templatepriority 0 template")
	}

	#[test]
	fn tr_position() {
	    // XPath == position()
	    // NB. rust indexes start at 0, whereas XPath positions start at 1

	    let x = Transform::Position;
	    let seq = ContextBuilder::new()
		    .context(vec![
			Item::<$x>::Value(Rc::new(Value::from("one"))),
			Item::<$x>::Value(Rc::new(Value::from("two"))),
			Item::<$x>::Value(Rc::new(Value::from("three"))),
			Item::<$x>::Value(Rc::new(Value::from("four"))),
		    ])
		    .index(2)
		    .build()
		.dispatch(&mut StaticContext::<F>::new(), &x)
	    .expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "3")
	}

	#[test]
	fn tr_last() {
	    // XPath == last()
	    // NB. rust indexes start at 0, whereas XPath positions start at 1

	    let x = Transform::Last;
	    let seq = ContextBuilder::new()
		    .context(vec![
			Item::<$x>::Value(Rc::new(Value::from("one"))),
			Item::<$x>::Value(Rc::new(Value::from("two"))),
			Item::<$x>::Value(Rc::new(Value::from("three"))),
			Item::<$x>::Value(Rc::new(Value::from("four"))),
		    ])
		    .index(2)
		    .build()
		.dispatch(&mut StaticContext::<F>::new(), &x)
	    .expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "4")
	}

	#[test]
	fn tr_count_0() {
	    // XPath == count()

	    let x = Transform::Count(Box::new(Transform::Empty));
	    let seq = ContextBuilder::new()
		    .context(vec![
			Item::<$x>::Value(Rc::new(Value::from("one"))),
			Item::<$x>::Value(Rc::new(Value::from("two"))),
			Item::<$x>::Value(Rc::new(Value::from("three"))),
			Item::<$x>::Value(Rc::new(Value::from("four"))),
		    ])
		    .build()
		.dispatch(&mut StaticContext::<F>::new(), &x)
	    .expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "0")
	}

	#[test]
	fn tr_count_1() {
	    // XPath == count()

	    let x = Transform::Count(Box::new(Transform::SequenceItems(vec![
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("abc")))),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1)))),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("foo")))),
		])));
	    let seq = ContextBuilder::new()
		    .context(vec![
			Item::<$x>::Value(Rc::new(Value::from("one"))),
			Item::<$x>::Value(Rc::new(Value::from("two"))),
			Item::<$x>::Value(Rc::new(Value::from("three"))),
			Item::<$x>::Value(Rc::new(Value::from("four"))),
		    ])
		    .index(2)
		    .build()
		.dispatch(&mut StaticContext::<F>::new(), &x)
	    .expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "3")
	}

	#[test]
	fn tr_localname_0() {
	    // Setup a source document
	    let mut sd = $y();
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

	    let x = Transform::LocalName(Some(Box::new(Transform::ContextItem)));

	    // Now evaluate the combinator with <Level-1> as the context item
	    let seq = Context::from(vec![Item::Node(l1)])
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_xml(), "Level-1");
	}

	#[test]
	fn tr_name_0() {
	    // Setup a source document
	    let mut sd = $y();
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

	    let x = Transform::Name(Some(Box::new(Transform::ContextItem)));

	    // Now evaluate the combinator with <Level-1> as the context item
	    let seq = Context::from(vec![Item::Node(l1)])
		.dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_xml(), "eg:Level-1");
	}

	#[test]
	fn tr_string() {
	    // XPath == string(1.0)
	    let x = Transform::String(Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1.0))))));
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "1")
	}

	#[test]
	fn tr_concat_literal() {
	    // XPath == concat("abc", 1, "foo")
	    let x = Transform::Concat(
		vec![
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("abc")))),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1)))),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("foo")))),
		]
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "abc1foo")
	}

	#[test]
	fn tr_starts_with_pos() {
	    // XPath == starts-with("abc", "ab")
	    let x = Transform::StartsWith(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("abc"))))),
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("ab"))))),
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), true)
	}
	#[test]
	fn tr_starts_with_neg() {
	    // XPath == starts-with("abc", "x")
	    let x = Transform::StartsWith(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("abc"))))),
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("x"))))),
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), false)
	}

	#[test]
	fn tr_contains_pos() {
	    // XPath == contains("abcd", "bc")
	    let x = Transform::Contains(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("abcd"))))),
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("bc"))))),
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), true)
	}
	#[test]
	fn tr_contains_neg() {
	    // XPath == contains("abcd", "xyz")
	    let x = Transform::Contains(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("abcd"))))),
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("xyz"))))),
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), false)
	}

	#[test]
	fn tr_substring_2args() {
	    // XPath == substring("abcd", 2)
	    let x = Transform::Substring(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("abcd"))))),
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(2))))),
		None
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "bcd")
	}
	#[test]
	fn tr_substring_3args() {
	    // XPath == substring("abcd", 2, 2)
	    let x = Transform::Substring(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("abcd"))))),
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(2))))),
		Some(Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(2))))))
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "bc")
	}

	#[test]
	fn tr_substring_before() {
	    // XPath == substring-before("abcd", "bc")
	    let x = Transform::SubstringBefore(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("abcd"))))),
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("bc"))))),
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "a")
	}
	#[test]
	fn tr_substring_after() {
	    // XPath == substring-after("abcd", "bc")
	    let x = Transform::SubstringAfter(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("abcd"))))),
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("bc"))))),
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "d")
	}

	#[test]
	fn tr_normalize_space_1() {
	    // XPath == normalize-space(" a b  c	d\n")
	    let x = Transform::NormalizeSpace(
		Some(Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(" a b  c	d
"))))))
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "a b c d")
	}

	#[test]
	fn tr_translate_1() {
	    // XPath == translate("abcd", "bdc" "BD")
	    let x = Transform::Translate(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("abcd"))))),
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("bdc"))))),
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("BD"))))),
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "aBD")
	}

	#[test]
	fn tr_boolean_string_pos() {
	    // XPath == boolean("abcd")
	    let x = Transform::Boolean(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("abcd"))))),
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), true)
	}
	#[test]
	fn tr_boolean_string_neg() {
	    // XPath == boolean("")
	    let x = Transform::Boolean(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(""))))),
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), false)
	}
	#[test]
	fn tr_boolean_int_pos() {
	    // XPath == boolean(1)
	    let x = Transform::Boolean(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1))))),
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), true)
	}
	#[test]
	fn tr_boolean_int_neg() {
	    // XPath == boolean(0)
	    let x = Transform::Boolean(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(0))))),
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), false)
	}

	#[test]
	fn tr_not_pos() {
	    // XPath == not("abcd")
	    let x = Transform::Not(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("abcd"))))),
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), false)
	}
	#[test]
	fn tr_not_neg() {
	    // XPath == not(0)
	    let x = Transform::Not(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(0))))),
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), true)
	}

	#[test]
	fn tr_true_literal() {
	    // XPath == true()
	    let x = Transform::<$x>::True;
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), true)
	}
	#[test]
	fn tr_false_literal() {
	    // XPath == false()
	    let x = Transform::<$x>::False;
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_bool(), false)
	}

	#[test]
	fn tr_number() {
	    // XPath == number("124")
	    let x = Transform::Number(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("124"))))),
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_int().unwrap(), 124)
	}

	#[test]
	fn tr_sum() {
	    // XPath == sum((1, 2, 4))
	    let x = Transform::Sum(
		Box::new(Transform::SequenceItems(vec![
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1)))),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(2)))),
		    Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(4)))),
		]))
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_int().unwrap(), 7)
	}

	#[test]
	fn tr_floor() {
	    // XPath == floor((1.2))
	    let x = Transform::Floor(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1.2))))),
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq[0].to_double(), 1.0)
	}

	#[test]
	fn tr_ceiling() {
	    // XPath == ceiling((1.2))
	    let x = Transform::Ceiling(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1.2))))),
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq[0].to_double(), 2.0)
	}

	#[test]
	fn tr_round_1() {
	    // XPath == round((1.23456))
	    let x = Transform::Round(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1.23456))))),
		None,
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq[0].to_double(), 1.0)
	}
	#[test]
	fn tr_round_2() {
	    // XPath == round((1.23456, 4))
	    let x = Transform::Round(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(1.23456))))),
		Some(Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from(4)))))),
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert!(seq[0].to_double() - 1.2346 < 0.000001)
	}

	#[test]
	fn tr_current_date_time() {
	    // XPath == current-date-time()
	    let x = Transform::<$x>::CurrentDateTime;
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    match &seq[0] {
			Item::Value(v) => match **v {
				Value::DateTime(dt) => {
		    assert_eq!(dt.year(), Local::now().year());
		    assert_eq!(dt.month(), Local::now().month());
		    assert_eq!(dt.day(), Local::now().day());
		    assert_eq!(dt.hour(), Local::now().hour());
		    assert_eq!(dt.minute(), Local::now().minute());
		    assert_eq!(dt.second(), Local::now().second()); // It is possible for this to fail if the elapsed time to execute the function call and the test falls across a second quantum
				}
				_ => panic!("not a singleton dateTime value")
			}
			_ => panic!("not a value")
	    }
	}

	#[test]
	fn tr_current_date() {
	    // XPath == current-date()
	    let x = Transform::<$x>::CurrentDate;
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    match &seq[0] {
			Item::Value(v) => match **v {
				Value::Date(dt) => {
		    assert_eq!(dt.year(), Local::now().year());
		    assert_eq!(dt.month(), Local::now().month());
		    assert_eq!(dt.day(), Local::now().day());
				}
				_ => panic!("not a singleton date value")
			}
			_ => panic!("not a value")
	    }
	}

	#[test]
	fn tr_current_time() {
	    // XPath == current-time()
	    let x = Transform::<$x>::CurrentTime;
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    match &seq[0] {
		Item::Value(v) => match **v {
			Value::Time(dt) => {
		    assert_eq!(dt.hour(), Local::now().hour());
		    assert_eq!(dt.minute(), Local::now().minute());
		    assert_eq!(dt.second(), Local::now().second()); // It is possible for this to fail if the elapsed time to execute the function call and the test falls across a second quantum
			}
			_ => panic!("not a singleton time value")
		}
		_ => panic!("not a value")
	    }
	}

	#[test]
	fn tr_format_date_time() {
	    // XPath == format-dateTime("2022-01-03T04:05:06.789+10:00", "[H]:[m] [D]/[M]/[Y]")
	    let x = Transform::FormatDateTime(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("2022-01-03T04:05:06.789+10:00"))))),
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("[H]:[m] [D]/[M]/[Y]"))))),
		None,
		None,
		None,
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "04:05 03/01/2022")
	}

	#[test]
	fn tr_format_date() {
	    // XPath == format-date("2022-01-03", "[D]/[M]/[Y]")
	    let x = Transform::FormatDate(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("2022-01-03"))))),
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("[D]/[M]/[Y]"))))),
		None,
		None,
		None,
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "03/01/2022")
	}

	#[test]
	fn tr_format_time() {
	    // XPath == format-time("04:05:06.789+10:00", "[H]:[m]")
	    let x = Transform::FormatTime(
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("04:05:06.789"))))),
		Box::new(Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("[H]:[m]:[s]"))))),
		None,
		None,
		None,
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq.to_string(), "04:05:06")
	}

	#[test]
	fn tr_func_user_defined() {
	    // foo(bar='a test'; ('this is ', $bar))
	    let x = Transform::UserDefined(
			QualifiedName::new(None, None, String::from("mytest")),
		vec![
		    ("bar".to_string(), Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("a test")))))
		],
		Box::new(Transform::SequenceItems(
		    vec![
			Transform::Literal(Item::<$x>::Value(Rc::new(Value::from("this is ")))),
			Transform::VariableReference("bar".to_string()),
		    ]
		))
	    );
	    let seq = Context::new().dispatch(&mut StaticContext::<F>::new(), &x)
		.expect("evaluation failed");
	    assert_eq!(seq.len(), 2);
	    assert_eq!(seq.to_string(), "this is a test")
	}
    }
);
