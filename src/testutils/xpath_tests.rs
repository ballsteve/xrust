#[macro_export]
macro_rules! xpath_tests (
    ( $x:expr , $y:expr ) => {
	use xrust::xpath::parse;
	use xrust::evaluate::StaticContext;

	#[test]
	fn nomxpath_parse_empty() {
            let e = parse::<RNode>("()").expect("failed to parse expression \"()\"");
	    assert_eq!(e.len(), 0)
	}

	#[test]
	fn nomxpath_parse_union() {
            let e = parse::<RNode>("'a' | 'b'").expect("failed to parse expression \"'a' | 'b'\"");
	    if e.len() == 1 {
		assert!(true) // TODO: check the sequence constructor
	    } else {
		panic!("sequence is not a singleton")
	    }
	}

	#[test]
	fn nomxpath_parse_intersectexcept() {
            let e = parse::<RNode>("'a' intersect 'b' except 'c'").expect("failed to parse expression \"'a' intersect 'b' except 'c'\"");
	    if e.len() == 1 {
		assert!(true) // TODO: check the sequence constructor
	    } else {
		panic!("sequence is not a singleton")
	    }
	}

	#[test]
	fn nomxpath_parse_instanceof() {
            let e = parse::<RNode>("'a' instance of empty-sequence()").expect("failed to parse expression \"'a' instance of empty-sequence()\"");
	    if e.len() == 1 {
		assert!(true) // TODO: check the sequence constructor
	    } else {
		panic!("sequence is not a singleton")
	    }
	}

	#[test]
	fn nomxpath_parse_treat() {
            let e = parse::<RNode>("'a' treat as empty-sequence()").expect("failed to parse expression \"'a' treat as empty-sequence()\"");
	    if e.len() == 1 {
		assert!(true) // TODO: check the sequence constructor
	    } else {
		panic!("sequence is not a singleton")
	    }
	}

	#[test]
	fn nomxpath_parse_castable() {
            let e = parse::<RNode>("'a' castable as type").expect("failed to parse expression \"'a' castable as type\"");
	    if e.len() == 1 {
		assert!(true) // TODO: check the sequence constructor
	    } else {
		panic!("sequence is not a singleton")
	    }
	}

	#[test]
	fn nomxpath_parse_cast() {
            let e = parse::<RNode>("'a' cast as type").expect("failed to parse expression \"'a' cast as type\"");
	    if e.len() == 1 {
		assert!(true) // TODO: check the sequence constructor
	    } else {
		panic!("sequence is not a singleton")
	    }
	}

	#[test]
	fn nomxpath_parse_arrow() {
            let e = parse::<RNode>("'a' => spec()").expect("failed to parse expression \"'a' => spec()\"");
	    if e.len() == 1 {
		assert!(true) // TODO: check the sequence constructor
	    } else {
		panic!("sequence is not a singleton")
	    }
	}

	#[test]
	fn nomxpath_parse_unary() {
            let e = parse::<RNode>("+'a'").expect("failed to parse expression \"+'a'\"");
	    if e.len() == 1 {
		assert!(true) // TODO: check the sequence constructor
	    } else {
		panic!("sequence is not a singleton")
	    }
	}

	#[test]
	fn nomxpath_parse_simplemap() {
            let e = parse::<RNode>("'a'!'b'").expect("failed to parse expression \"'a'!'b'\"");
	    if e.len() == 1 {
		assert!(true) // TODO: check the sequence constructor
	    } else {
		panic!("sequence is not a singleton")
	    }
	}

	// Parses to a singleton integer sequence constructor
	#[test]
	fn xpath_int() {
	    let x = parse::<RNode>("1")
		.expect("failed to parse expression \"1\"");
	    assert_eq!(x.len(), 1);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(None, None, &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s[0].to_int().unwrap(), 1);
	}
	// Parses to a singleton double/decimal sequence constructor
	#[test]
	fn xpath_decimal() {
	    let x = parse::<RNode>("1.2")
		.expect("failed to parse expression \"1.2\"");
	    assert_eq!(x.len(), 1);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(None, None, &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s[0].to_double(), 1.2);
	}
	// Parses to a singleton double sequence constructor
	#[test]
	fn xpath_exponent() {
	    let x = parse::<RNode>("1.2e2")
		.expect("failed to parse expression \"1.2e2\"");
	    assert_eq!(x.len(), 1);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(None, None, &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s[0].to_double(), 120.0);
	}
	// Parses to a singleton string
	#[test]
	fn xpath_string_apos() {
	    let x = parse::<RNode>("'abc'")
		.expect("failed to parse expression \"'abc'\"");
	    assert_eq!(x.len(), 1);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(None, None, &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s[0].to_string(), "abc");
	}
	// Parses to a singleton string
	#[test]
	fn xpath_string_apos_esc() {
	    let x = parse::<RNode>("'abc''def'")
		.expect("failed to parse expression \"'abc''def'\"");
	    assert_eq!(x.len(), 1);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(None, None, &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s[0].to_string(), "abc'def");
	}
	// Parses to a singleton string
	#[test]
	fn xpath_string_quot() {
	    let x = parse::<RNode>(r#""abc""#)
		.expect("failed to parse expression \"\"abc\"\"");
	    assert_eq!(x.len(), 1);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(None, None, &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s[0].to_string(), "abc");
	}
	// Parses to a singleton string
	#[test]
	fn xpath_string_quot_esc() {
	    let x = parse::<RNode>(r#""abc""def""#)
		.expect("failed to parse expression \"\"abc\"\"def\"\"");
	    assert_eq!(x.len(), 1);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(None, None, &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s[0].to_string(), r#"abc"def"#);
	}

	// Sequences
	#[test]
	fn xpath_literal_sequence() {
	    let x = parse::<RNode>("1,'abc',2")
		.expect("failed to parse expression \"\"1,'abc',2\"");
	    assert_eq!(x.len(), 3);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(None, None, &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 3);
	    assert_eq!(s[0].to_int().unwrap(), 1);
	    assert_eq!(s[1].to_string(), "abc");
	    assert_eq!(s[2].to_int().unwrap(), 2);
	}
	#[test]
	fn xpath_literal_sequence_ws() {
	    let x = parse::<RNode>("1 , 'abc', 2")
		.expect("failed to parse expression \"\"1 , 'abc', 2\"");
	    assert_eq!(x.len(), 3);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(None, None, &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 3);
	    assert_eq!(s[0].to_int().unwrap(), 1);
	    assert_eq!(s[1].to_string(), "abc");
	    assert_eq!(s[2].to_int().unwrap(), 2);
	}

	// Comments
	#[test]
	fn xpath_comment() {
	    let x = parse::<RNode>("1(::),(: a comment :)'abc', (: outer (: inner :) outer :) 2")
		.expect("failed to parse \"1(::),(: a comment :)'abc', (: outer (: inner :) outer :) 2\"");
	    assert_eq!(x.len(), 3);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(None, None, &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 3);
	    assert_eq!(s[0].to_int().unwrap(), 1);
	    assert_eq!(s[1].to_string(), "abc");
	    assert_eq!(s[2].to_int().unwrap(), 2);
	}

	// Parses to a singleton context item sequence constructor
	#[test]
	fn xpath_context_item() {
	    let x = parse::<RNode>(".")
		.expect("failed to parse expression \".\"");
	    assert_eq!(x.len(), 1);
	    let ctxt = vec![Rc::new(Item::Value(Value::from("foobar")))];
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(Some(ctxt), Some(0), &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s[0].to_string(), "foobar");
	}

	// Parentheses
	#[test]
	fn xpath_parens_singleton() {
	    let x = parse::<RNode>("(1)")
		.expect("failed to parse expression \"(1)\"");
	    assert_eq!(x.len(), 1);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(None, None, &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s[0].to_int().unwrap(), 1);
	}

	// Steps

	use xrust::evaluate::format_constructor;
	#[test]
	fn xpath_root_step_1() {
	    let x = parse::<RNode>("/child::a")
		.expect("failed to parse expression \"/child::a\"");
	    eprintln!("xpath expression:\n{}", format_constructor(&x, 0));
	    assert_eq!(x.len(), 1);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(Some(vec![$y()]), Some(0), &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match &*s[0] {
		Item::Node(n) => {
		    assert_eq!(n.node_type(), NodeType::Element);
		    assert_eq!(n.name().to_string(), "a")
		}
		_ => panic!("not a node")
	    }
	}
/*
	#[test]
	fn xpath_root_step_2() {
	    let x = parse::<RNode>("/child::a/child::b")
		.expect("failed to parse expression \"/child::a/child::b\"");
	    assert_eq!(x.len(), 1);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(Some(vec![$y()]), Some(0), &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 2);
	    match &*s[0] {
		Item::Node(n) => {
		    assert_eq!(n.node_type(), NodeType::Element);
		    assert_eq!(n.name().to_string(), "b")
		}
	    }
	    match &*s[1] {
		Item::Node(n) => {
		    assert_eq!(n.node_type(), NodeType::Element);
		    assert_eq!(n.name().to_string(), "b")
		}
	    }
	}

	#[test]
	fn xpath_root_desc_or_self_1() {
	    let x = parse::<RNode>("//child::a")
		.expect("failed to parse expression \"//child::a\"");
	    assert_eq!(x.len(), 1);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(Some(vec![$y()]), Some(0), &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 5);
	    for t in s {
		match t {
		    Item::Node(n) => {
			assert_eq!(n.node_type(), NodeType::Element);
			assert_eq!(n.name().to_string(), "a")
		    }
		}
	    }
	}
	#[test]
	fn xpath_root_desc_or_self_2() {
	    let x = parse::<RNode>("//child::a/child::b")
		.expect("failed to parse expression \"//child::a/child::b\"");
	    assert_eq!(x.len(), 1);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(Some(vec![$y()]), Some(0), &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 10);
	    for t in s {
		match t {
		    Item::Node(n) => {
			assert_eq!(n.node_type(), NodeType::Element);
			assert_eq!(n.name().to_string(), "b")
		    }
		}
	    }
	}
	#[test]
	fn xpath_root_desc_or_self_3() {
	    let x = parse::<RNode>("//child::a//child::b")
		.expect("failed to parse expression \"//child::a//child::b\"");
	    assert_eq!(x.len(), 1);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(Some(vec![$y()]), Some(0), &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 10);
	    for t in s {
		match t {
		    Item::Node(n) => {
			assert_eq!(n.node_type(), NodeType::Element);
			assert_eq!(n.name().to_string(), "b")
		    }
		}
	    }
	}
	#[test]
	fn xpath_rel_path_1() {
	    let x = parse::<RNode>("child::a/child::b")
		.expect("failed to parse expression \"child::a/child::b\"");
	    assert_eq!(x.len(), 1);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(Some(vec![$y()]), Some(0), &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 2);
	    for t in s {
		match t {
		    Item::Node(n) => {
			assert_eq!(n.node_type(), NodeType::Element);
			assert_eq!(n.name().to_string(), "b")
		    }
		}
	    }
	}
	#[test]
	fn xpath_rel_path_2() {
	    let x = parse::<RNode>("child::a//child::b")
		.expect("failed to parse expression \"child::a//child::b\"");
	    assert_eq!(x.len(), 1);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(Some(vec![$y()]), Some(0), &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 10);
	    for t in s {
		match t {
		    Item::Node(n) => {
			assert_eq!(n.node_type(), NodeType::Element);
			assert_eq!(n.name().to_string(), "b")
		    }
		}
	    }
	}
	#[test]
	fn xpath_step_1() {
	    let x = parse::<RNode>("child::a")
		.expect("failed to parse expression \"child::a\"");
	    assert_eq!(x.len(), 1);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(Some(vec![$y()]), Some(0), &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    for t in s {
		match t {
		    Item::Node(n) => {
			assert_eq!(n.node_type(), NodeType::Element);
			assert_eq!(n.name().to_string(), "a")
		    }
		}
	    }
	}
	#[test]
	fn xpath_step_2() {
	    let x = parse::<RNode>("child::bc")
		.expect("failed to parse expression \"child::bc\"");
	    assert_eq!(x.len(), 1);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(Some(vec![$y()]), Some(0), &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 0);
	}
	#[test]
	fn xpath_step_wild() {
	    let x = parse::<RNode>("child::*")
		.expect("failed to parse expression \"child::*\"");
	    assert_eq!(x.len(), 1);
	    let rd = $x();
	    let s = Evaluator::new()
		.evaluate(Some(vec![$y()]), Some(0), &x, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    for t in s {
		match t {
		    Item::Node(n) => {
			assert_eq!(n.node_type(), NodeType::Element);
			assert_eq!(n.name().to_string(), "a")
		    }
		}
	    }
	}

	// Functions
	#[test]
	fn xpath_fncall_string() {
	    let mut e = parse::<RNode>("string(('a', 'b', 'c'))")
		.expect("failed to parse expression \"string(('a', 'b', 'c'))\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.to_string(), "abc")
	}
	#[test]
	fn xpath_fncall_concat() {
	    let mut e = parse::<RNode>("concat('a', 'b', 'c')")
		.expect("failed to parse expression \"concat('a', 'b', 'c')\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.to_string(), "abc")
	}
	#[test]
	fn xpath_fncall_startswith_pos() {
	    let mut e = parse::<RNode>("starts-with('abc', 'a')")
		.expect("failed to parse expression \"starts-with('abc', 'a')\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.to_bool(), true)
	}
	#[test]
	fn xpath_fncall_startswith_neg() {
	    let mut e = parse::<RNode>("starts-with('abc', 'b')")
		.expect("failed to parse expression \"starts-with('abc', 'a')\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.to_bool(), false)
	}
	#[test]
	fn xpath_fncall_contains_pos() {
	    let mut e = parse::<RNode>("contains('abc', 'b')")
		.expect("failed to parse expression \"contains('abc', 'b')\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.to_bool(), true)
	}
	#[test]
	fn xpath_fncall_contains_neg() {
	    let mut e = parse::<RNode>("contains('abc', 'd')")
		.expect("failed to parse expression \"contains('abc', 'd')\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.to_bool(), false)
	}
	#[test]
	fn xpath_fncall_substringbefore_pos() {
	    let mut e = parse::<RNode>("substring-before('abc', 'b')")
		.expect("failed to parse expression \"substring-before('abc', 'b')\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.to_string(), "a")
	}
	#[test]
	fn xpath_fncall_substringbefore_neg() {
	    let mut e = parse::<RNode>("substring-before('abc', 'd')")
		.expect("failed to parse expression \"substring-before('abc', 'd')\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.to_string(), "")
	}
	#[test]
	fn xpath_fncall_substringafter_pos_1() {
	    let mut e = parse::<RNode>("substring-after('abc', 'b')")
		.expect("failed to parse expression \"substring-after('abc', 'b')\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.to_string(), "c")
	}
	#[test]
	fn xpath_fncall_substringafter_pos_2() {
	    let mut e = parse::<RNode>("substring-after('abc', 'c')")
		.expect("failed to parse expression \"substring-after('abc', 'b')\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.to_string(), "")
	}
	#[test]
	fn xpath_fncall_substringafter_neg() {
	    let mut e = parse::<RNode>("substring-after('abc', 'd')")
		.expect("failed to parse expression \"substring-after('abc', 'd')\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.to_string(), "")
	}
	#[test]
	fn xpath_fncall_normalizespace() {
	    let mut e = parse::<RNode>("normalize-space('	a  b\nc 	')")
		.expect("failed to parse expression \"normalize-space('	a  b\nc 	')\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.to_string(), "abc")
	}
	#[test]
	fn xpath_fncall_translate() {
	    let mut e = parse::<RNode>("translate('abcdeabcde', 'ade', 'XY')")
		.expect("failed to parse expression \"translate('abcdeabcde', 'ade', 'XY')\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.to_string(), "XbcYXbcY")
	}
	#[test]
	fn xpath_fncall_boolean_true() {
	    let mut e = parse::<RNode>("boolean('abcdeabcde')")
		.expect("failed to parse expression \"boolean('abcdeabcde')\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
		_ => panic!("not a singleton boolean true value")
	    }
	}
	#[test]
	fn xpath_fncall_boolean_false() {
	    let mut e = parse::<RNode>("boolean('')")
		.expect("failed to parse expression \"boolean('')\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Boolean(b)) => assert_eq!(b, false),
		_ => panic!("not a singleton boolean false value")
	    }
	}
	#[test]
	fn xpath_fncall_not_true() {
	    let mut e = parse::<RNode>("not('')")
		.expect("failed to parse expression \"not('')\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
		_ => panic!("not a singleton boolean true value")
	    }
	}
	#[test]
	fn xpath_fncall_not_false() {
	    let mut e = parse::<RNode>("not('abc')")
		.expect("failed to parse expression \"not('abc')\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Boolean(b)) => assert_eq!(b, false),
		_ => panic!("not a singleton boolean false value")
	    }
	}
	#[test]
	fn xpath_fncall_true() {
	    let mut e = parse::<RNode>("true()")
		.expect("failed to parse expression \"true()\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
		_ => panic!("not a singleton boolean true value")
	    }
	}
	#[test]
	fn xpath_fncall_false() {
	    let mut e = parse::<RNode>("false()")
		.expect("failed to parse expression \"false()\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Boolean(b)) => assert_eq!(b, false),
		_ => panic!("not a singleton boolean false value")
	    }
	}
	#[test]
	fn xpath_fncall_number_int() {
	    let mut e = parse::<RNode>("number('123')")
		.expect("failed to parse expression \"number('123')\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Integer(i)) => assert_eq!(i, 123),
		_ => panic!("not a singleton integer value, got \"{}\"", s.to_string())
	    }
	}
	#[test]
	fn xpath_fncall_number_double() {
	    let mut e = parse::<RNode>("number('123.456')")
		.expect("failed to parse expression \"number('123.456')\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Double(d)) => assert_eq!(d, 123.456),
		_ => panic!("not a singleton double value")
	    }
	}
	#[test]
	fn xpath_fncall_sum() {
	    let mut e = parse::<RNode>("sum(('123.456', 10, 20, '0'))")
		.expect("failed to parse expression \"sum(('123.456', 10, 20, '0'))\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Double(d)) => assert_eq!(d, 123.456 + 10.0 + 20.0),
		_ => panic!("not a singleton double value")
	    }
	}
	#[test]
	fn xpath_fncall_floor() {
	    let mut e = parse::<RNode>("floor(123.456)")
		.expect("failed to parse expression \"floor(123.456)\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Double(d)) => assert_eq!(d, 123.0),
		_ => panic!("not a singleton double value")
	    }
	}
	#[test]
	fn xpath_fncall_ceiling() {
	    let mut e = parse::<RNode>("ceiling(123.456)")
		.expect("failed to parse expression \"ceiling(123.456)\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Double(d)) => assert_eq!(d, 124.0),
		_ => panic!("not a singleton double value")
	    }
	}
	#[test]
	fn xpath_fncall_round_down() {
	    let mut e = parse::<RNode>("round(123.456)")
		.expect("failed to parse expression \"round(123.456)\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Double(d)) => assert_eq!(d, 123.0),
		_ => panic!("not a singleton double value")
	    }
	}
	#[test]
	fn xpath_fncall_round_up() {
	    let mut e = parse::<RNode>("round(123.654)")
		.expect("failed to parse expression \"round(123.654)\"");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Double(d)) => assert_eq!(d, 124.0),
		_ => panic!("not a singleton double value")
	    }
	}

	// Variables
	#[test]
	fn xpath_let_1() {
	    let mut e = parse::<RNode>("let $x := 'a' return ($x, $x)")
		.expect("failed to parse let expression");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.to_string(), "aa")
	}
	#[test]
	fn xpath_let_2() {
	    let mut e = parse::<RNode>("let $x := 'a', $y := 'b' return ($x, $y)")
		.expect("failed to parse let expression");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 2);
	    assert_eq!(s.to_string(), "ab")
	}

	// Loops
	#[test]
	fn xpath_for_1() {
	    let mut e = parse::<RNode>("for $x in ('a', 'b', 'c') return ($x, $x)")
		.expect("failed to parse let expression");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 6);
	    assert_eq!(s.to_string(), "aabbcc")
	}
	#[test]
	fn xpath_for_2() {
	    let mut e = parse::<RNode>("for $x in (1, 2, 3) return $x * 2")
		.expect("failed to parse let expression");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 3);
	    assert_eq!(s.to_string(), "246")
	}

	#[test]
	fn xpath_if_1() {
	    let mut e = parse::<RNode>("if (1) then 'one' else 'not one'")
		.expect("failed to parse let expression");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s.to_string(), "one")
	}
	#[test]
	fn xpath_if_2() {
	    let mut e = parse::<RNode>("if (0) then 'one' else 'not one'")
		.expect("failed to parse let expression");
	    StaticContext::new_with_builtins().static_analysis(&mut e);
	    let rd = $x();
	    let s = Evaluator::new().evaluate(None, None, &e, &rd)
		.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s.to_string(), "not one")
	}*/
    }
);
