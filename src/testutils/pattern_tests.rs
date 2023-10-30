#[macro_export]
macro_rules! pattern_tests (
    ( $t:ty , $x:expr , $y:expr ) => {
	use xrust::pattern::Pattern;

	#[test]
	#[should_panic]
	fn pattern_empty() {
    	let p: Pattern<$t> = Pattern::try_from("").expect("unable to parse empty string");
	}

	#[test]
	fn pattern_predicate_1_pos() {
            let p: Pattern<$t> = Pattern::try_from(".[self::a]").expect("unable to parse \".[self::a]\"");

	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut a = sd.new_element(QualifiedName::new(None, None, String::from("a")))
		.expect("unable to create element");
	    t.push(a.clone())
		.expect("unable to append child");
	    let t_a = sd.new_text(Value::from("first"))
		.expect("unable to create text node");
	    a.push(t_a)
		.expect("unable to append text node");
	    let mut b = sd.new_element(QualifiedName::new(None, None, String::from("b")))
		.expect("unable to create element");
	    t.push(b.clone())
		.expect("unable to append child");
	    let t_b = sd.new_text(Value::from("second"))
		.expect("unable to create text node");
	    b.push(t_b)
		.expect("unable to append text node");

	    assert_eq!(p.matches(&Context::new(), &Rc::new(Item::Node(a))), true);
	}
	#[test]
	fn pattern_predicate_1_neg() {
            let p: Pattern<$t> = Pattern::try_from(".[self::a]").expect("unable to parse \".[self::a]\"");

	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut a = sd.new_element(QualifiedName::new(None, None, String::from("a")))
		.expect("unable to create element");
	    t.push(a.clone())
		.expect("unable to append child");
	    let t_a = sd.new_text(Value::from("first"))
		.expect("unable to create text node");
	    a.push(t_a)
		.expect("unable to append text node");
	    let mut b = sd.new_element(QualifiedName::new(None, None, String::from("b")))
		.expect("unable to create element");
	    t.push(b.clone())
		.expect("unable to append child");
	    let t_b = sd.new_text(Value::from("second"))
		.expect("unable to create text node");
	    b.push(t_b)
		.expect("unable to append text node");

	    assert_eq!(p.matches(&Context::new(), &Rc::new(Item::Node(b))), false);
	}
	#[test]
	fn pattern_sel_1_pos() {
            let p: Pattern<$t> = Pattern::try_from("child::a").expect("unable to parse \"child::a\"");

	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut a = sd.new_element(QualifiedName::new(None, None, String::from("a")))
		.expect("unable to create element");
	    t.push(a.clone())
		.expect("unable to append child");
	    let t_a = sd.new_text(Value::from("first"))
		.expect("unable to create text node");
	    a.push(t_a)
		.expect("unable to append text node");
	    let mut b = sd.new_element(QualifiedName::new(None, None, String::from("b")))
		.expect("unable to create element");
	    t.push(b.clone())
		.expect("unable to append child");
	    let t_b = sd.new_text(Value::from("second"))
		.expect("unable to create text node");
	    b.push(t_b)
		.expect("unable to append text node");

	    assert_eq!(p.matches(&Context::new(), &Rc::new(Item::Node(a))), true);
	}
	#[test]
	fn pattern_sel_1_neg() {
            let p: Pattern<$t> = Pattern::try_from("child::a").expect("unable to parse \"child::a\"");

	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut a = sd.new_element(QualifiedName::new(None, None, String::from("a")))
		.expect("unable to create element");
	    t.push(a.clone())
		.expect("unable to append child");
	    let t_a = sd.new_text(Value::from("first"))
		.expect("unable to create text node");
	    a.push(t_a)
		.expect("unable to append text node");
	    let mut b = sd.new_element(QualifiedName::new(None, None, String::from("b")))
		.expect("unable to create element");
	    t.push(b.clone())
		.expect("unable to append child");
	    let t_b = sd.new_text(Value::from("second"))
		.expect("unable to create text node");
	    b.push(t_b)
		.expect("unable to append text node");

	    assert_eq!(p.matches(&Context::new(), &Rc::new(Item::Node(b))), false);
	}
	#[test]
	fn pattern_sel_2_pos() {
            let p: Pattern<$t> = Pattern::try_from("child::Test/child::a")
			.expect("unable to parse \"child::Test/child::a\"");

	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut a = sd.new_element(QualifiedName::new(None, None, String::from("a")))
		.expect("unable to create element");
	    t.push(a.clone())
		.expect("unable to append child");
	    let t_a = sd.new_text(Value::from("first"))
		.expect("unable to create text node");
	    a.push(t_a)
		.expect("unable to append text node");
	    let mut b = sd.new_element(QualifiedName::new(None, None, String::from("b")))
		.expect("unable to create element");
	    t.push(b.clone())
		.expect("unable to append child");
	    let t_b = sd.new_text(Value::from("second"))
		.expect("unable to create text node");
	    b.push(t_b)
		.expect("unable to append text node");

	    assert_eq!(p.matches(&Context::new(), &Rc::new(Item::Node(a))), true);
	}
	#[test]
	fn pattern_sel_2_neg() {
            let p: Pattern<$t> = Pattern::try_from("child::Test/child::a").expect("unable to parse \"child::Test/child::a\"");

	    // Setup a source document
	    let mut sd = NodeBuilder::new(NodeType::Document).build();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("NotATest")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut a = sd.new_element(QualifiedName::new(None, None, String::from("a")))
		.expect("unable to create element");
	    t.push(a.clone())
		.expect("unable to append child");
	    let t_a = sd.new_text(Value::from("first"))
		.expect("unable to create text node");
	    a.push(t_a)
		.expect("unable to append text node");
	    let mut b = sd.new_element(QualifiedName::new(None, None, String::from("b")))
		.expect("unable to create element");
	    t.push(b.clone())
		.expect("unable to append child");
	    let t_b = sd.new_text(Value::from("second"))
		.expect("unable to create text node");
	    b.push(t_b)
		.expect("unable to append text node");

	    assert_eq!(p.matches(&Context::new(), &Rc::new(Item::Node(a))), false);
	}
	}
);
