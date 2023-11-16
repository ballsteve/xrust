#[macro_export]
macro_rules! xpath_tests (
    ( $t:ty , $x:expr , $y:expr ) => {
	use xrust::parser::xpath::parse;
	use xrust::xdmerror::ErrorKind;
	use xrust::transform::context::{Context, ContextBuilder};

	#[test]
	fn xpath_empty() {
		let ev = parse::<$t>("").expect("not an XPath expression");
	    let seq = Context::new().dispatch(&ev).expect("evaluation failed");
	    assert_eq!(seq.len(), 0);
	}

	#[test]
	fn xpath_step_1_pos() {
		let ev = parse::<$t>("child::a").expect("not an XPath expression");
	    let rd = $x();
	    let mut ctxt = ContextBuilder::new()
		.current(vec![$y()])
		.result_document(rd)
		.build();
	    let seq = ctxt.dispatch(&ev).expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	    assert_eq!(seq[0].name().to_string(), "a");
	}
	#[test]
	fn xpath_path_1_pos() {
		let ev = parse::<$t>("/child::a").expect("not an XPath expression");
	    let rd = $x();
	    let mut ctxt = ContextBuilder::new()
		.current(vec![$y()])
		.result_document(rd)
		.build();
	    let seq = ctxt.dispatch(&ev).expect("evaluation failed");
	    assert_eq!(seq.len(), 1);
	}
	#[test]
	fn xpath_path_1_neg() {
		let ev = parse::<$t>("/child::b").expect("not an XPath expression");
	    let rd = $x();
	    let mut ctxt = ContextBuilder::new()
		.current(vec![$y()])
		.result_document(rd)
		.build();
	    let seq = ctxt.dispatch(&ev).expect("evaluation failed");
	    if seq.len() != 0 {
		match &*seq[0] {
		    Item::Node(n) => {
				assert!(true)
		    }
		    _ => panic!("not a node")
		}
	    }
	    assert_eq!(seq.len(), 0);
	}
	#[test]
	fn xpath_path_2() {
            let ev = parse::<$t>("/child::a/child::b").expect("not an XPath expression");
	    let rd = $x();
	    let mut ctxt = ContextBuilder::new()
		.current(vec![$y()])
		.result_document(rd)
		.build();
	    let seq = ctxt.dispatch(&ev).expect("evaluation failed");
	    assert_eq!(seq.len(), 2);
	}

	#[test]
	fn xpath_parse_union() {
            let e = parse::<$t>("'a' | 'b'").expect("failed to parse expression \"'a' | 'b'\"");
	    assert_eq!(ErrorKind::NotImplemented, Context::new().dispatch(&e).expect_err("is implemented").kind)
	}

	#[test]
	fn xpath_parse_intersectexcept() {
            let e = parse::<$t>("'a' intersect 'b' except 'c'").expect("failed to parse expression \"'a' intersect 'b' except 'c'\"");
	    assert_eq!(ErrorKind::NotImplemented, Context::new().dispatch(&e).expect_err("is implemented").kind)
	}

	#[test]
	fn xpath_parse_instanceof() {
            let e = parse::<$t>("'a' instance of empty-sequence()").expect("failed to parse expression \"'a' instance of empty-sequence()\"");
	    assert_eq!(ErrorKind::NotImplemented, Context::new().dispatch(&e).expect_err("is implemented").kind)
	}

	#[test]
	fn xpath_parse_treat() {
            let e = parse::<$t>("'a' treat as empty-sequence()").expect("failed to parse expression \"'a' treat as empty-sequence()\"");
	    assert_eq!(ErrorKind::NotImplemented, Context::new().dispatch(&e).expect_err("is implemented").kind)
	}

	#[test]
	fn xpath_parse_castable() {
            let e = parse::<$t>("'a' castable as type?").expect("failed to parse expression \"'a' castable as type\"");
	    assert_eq!(ErrorKind::NotImplemented, Context::new().dispatch(&e).expect_err("is implemented").kind)
	}

	#[test]
	fn xpath_parse_cast() {
            let e = parse::<$t>("'a' cast as type?").expect("failed to parse expression \"'a' cast as type\"");
	    assert_eq!(ErrorKind::NotImplemented, Context::new().dispatch(&e).expect_err("is implemented").kind)
	}

	#[test]
	fn xpath_parse_arrow() {
            let e = parse::<$t>("'a' => spec()").expect("failed to parse expression \"'a' => spec()\"");
	    assert_eq!(ErrorKind::NotImplemented, Context::new().dispatch(&e).expect_err("is implemented").kind)
	}

	#[test]
	fn xpath_parse_unary() {
            let e = parse::<$t>("+'a'").expect("failed to parse expression \"+'a'\"");
	    assert_eq!(ErrorKind::NotImplemented, Context::new().dispatch(&e).expect_err("is implemented").kind)
	}

	#[test]
	fn xpath_parse_simplemap() {
            let e = parse::<$t>("'a'!'b'").expect("failed to parse expression \"'a'!'b'\"");
	    assert_eq!(ErrorKind::NotImplemented, Context::new().dispatch(&e).expect_err("is implemented").kind)
	}

	// Parses to a singleton integer sequence constructor
	#[test]
	fn xpath_int() {
	    let e = parse::<$t>("1")
		.expect("failed to parse expression \"1\"");
	    let rd = $x();
	    let s = ContextBuilder::new()
		      .result_document(rd)
		      .build()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s[0].to_int().unwrap(), 1);
	}
	// Parses to a singleton double/decimal sequence constructor
	#[test]
	fn xpath_decimal() {
	    let e = parse::<$t>("1.2")
		.expect("failed to parse expression \"1.2\"");
	    let rd = $x();
	    let s = ContextBuilder::new()
		      .result_document(rd)
		      .build()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s[0].to_double(), 1.2);
	}
	// Parses to a singleton double sequence constructor
	#[test]
	fn xpath_exponent() {
	    let e = parse::<$t>("1.2e2")
		.expect("failed to parse expression \"1.2e2\"");
	    let rd = $x();
	    let s = ContextBuilder::new()
		      .result_document(rd)
		      .build()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s[0].to_double(), 120.0);
	}
	// Parses to a singleton string
	#[test]
	fn xpath_string_apos() {
	    let e = parse::<$t>("'abc'")
		.expect("failed to parse expression \"'abc'\"");
	    let rd = $x();
	    let s = ContextBuilder::new()
		      .result_document(rd)
		      .build()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s[0].to_string(), "abc");
	}
	// Parses to a singleton string
	#[test]
	fn xpath_string_apos_esc() {
	    let e = parse::<$t>("'abc''def'")
		.expect("failed to parse expression \"'abc''def'\"");
	    let rd = $x();
	    let s = ContextBuilder::new()
		      .result_document(rd)
		      .build()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s[0].to_string(), "abc'def");
	}
	// Parses to a singleton string
	#[test]
	fn xpath_string_quot() {
	    let e = parse::<$t>(r#""abc""#)
		.expect("failed to parse expression \"\"abc\"\"");
	    let rd = $x();
	    let s = ContextBuilder::new()
		      .result_document(rd)
		      .build()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s[0].to_string(), "abc");
	}
	// Parses to a singleton string
	#[test]
	fn xpath_string_quot_esc() {
	    let e = parse::<$t>(r#""abc""def""#)
		.expect("failed to parse expression \"\"abc\"\"def\"\"");
	    let rd = $x();
	    let s = ContextBuilder::new()
		      .result_document(rd)
		      .build()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s[0].to_string(), r#"abc"def"#);
	}

	// Sequences
	#[test]
	fn xpath_literal_sequence() {
	    let e = parse::<$t>("1,'abc',2")
		.expect("failed to parse expression \"\"1,'abc',2\"");
	    let rd = $x();
	    let s = ContextBuilder::new()
		      .result_document(rd)
		      .build()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 3);
	    assert_eq!(s[0].to_int().unwrap(), 1);
	    assert_eq!(s[1].to_string(), "abc");
	    assert_eq!(s[2].to_int().unwrap(), 2);
	}
	#[test]
	fn xpath_literal_sequence_ws() {
	    let e = parse::<$t>("1 , 'abc', 2")
		.expect("failed to parse expression \"\"1 , 'abc', 2\"");
	    let rd = $x();
		let s = ContextBuilder::new()
			.result_document(rd)
			.build()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 3);
	    assert_eq!(s[0].to_int().unwrap(), 1);
	    assert_eq!(s[1].to_string(), "abc");
	    assert_eq!(s[2].to_int().unwrap(), 2);
	}

	// Comments
	#[test]
	fn xpath_comment() {
	    let e = parse::<$t>("1(::),(: a comment :)'abc', (: outer (: inner :) outer :) 2")
		.expect("failed to parse \"1(::),(: a comment :)'abc', (: outer (: inner :) outer :) 2\"");
	    let rd = $x();
		let s = ContextBuilder::new()
			.result_document(rd)
			.build()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 3);
	    assert_eq!(s[0].to_int().unwrap(), 1);
	    assert_eq!(s[1].to_string(), "abc");
	    assert_eq!(s[2].to_int().unwrap(), 2);
	}

	// Parses to a singleton context item sequence constructor
	#[test]
	fn xpath_context_item() {
	    let e = parse::<$t>(".")
		.expect("failed to parse expression \".\"");
	    let rd = $x();
	    let s = ContextBuilder::new()
		      .current(vec![Rc::new(Item::Value(Value::from("foobar")))])
		      .result_document(rd)
			.build()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s[0].to_string(), "foobar");
	}

	// Parentheses
	#[test]
	fn xpath_parens_singleton() {
	    let e = parse::<$t>("(1)")
		.expect("failed to parse expression \"(1)\"");
	    let rd = $x();
		let s = ContextBuilder::new()
			.result_document(rd)
			.build()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s[0].to_int().unwrap(), 1);
	}

	// Steps

	#[test]
	fn xpath_root_step_1() {
	    let e = parse::<$t>("/child::a")
		.expect("failed to parse expression \"/child::a\"");
	    let rd = $x();
	    let s = ContextBuilder::new()
		      .result_document(rd.clone())
		      .current(vec![$y()])
			.build()
			.dispatch(&e)
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
	#[test]
	fn xpath_root_step_2() {
	    let e = parse::<$t>("/child::a/child::b")
		.expect("failed to parse expression \"/child::a/child::b\"");
	    let rd = $x();
	    let s = ContextBuilder::new()
		      .result_document(rd)
		      .current(vec![$y()])
	    	      .build()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 2);
	    match &*s[0] {
		Item::Node(n) => {
		    assert_eq!(n.node_type(), NodeType::Element);
		    assert_eq!(n.name().to_string(), "b")
		}
		_ => panic!("not a node")
	    }
	    match &*s[1] {
		Item::Node(n) => {
		    assert_eq!(n.node_type(), NodeType::Element);
		    assert_eq!(n.name().to_string(), "b")
		}
		_ => panic!("not a node")
	    }
	}

	#[test]
	fn xpath_root_desc_or_self_1() {
	    let e = parse::<$t>("//child::a")
		.expect("failed to parse expression \"//child::a\"");
	    let rd = $x();
	    let s = ContextBuilder::new()
		      .result_document(rd)
		      .current(vec![$y()])
		      .build()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 5);
	    for t in s {
		match &*t {
		    Item::Node(n) => {
			assert_eq!(n.node_type(), NodeType::Element);
			assert_eq!(n.name().to_string(), "a")
		    }
		    _ => panic!("not a node")
		}
	    }
	}
	#[test]
	fn xpath_root_desc_or_self_2() {
	    let e = parse::<$t>("//child::a/child::b")
		.expect("failed to parse expression \"//child::a/child::b\"");
	    let rd = $x();
	    let s = ContextBuilder::new()
		      .result_document(rd)
		      .current(vec![$y()])
			.build()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 10);
	    for t in s {
		match &*t {
		    Item::Node(n) => {
			assert_eq!(n.node_type(), NodeType::Element);
			assert_eq!(n.name().to_string(), "b")
		    }
		    _ => panic!("not a node")
		}
	    }
	}
	#[test]
	fn xpath_root_desc_or_self_3() {
	    let e = parse::<$t>("//child::a//child::b")
		.expect("failed to parse expression \"//child::a//child::b\"");
	    let rd = $x();
	    let s = ContextBuilder::new()
		      .result_document(rd)
		      .current(vec![$y()])
		      .build()
			.dispatch(&e)
			.expect("evaluation failed");
	    for i in 0..s.len() {
		match &*s[i] {
		    Item::Node(n) => {
			eprintln!("item {} is a {} element with id {}", i, n.name(), n.get_attribute(&QualifiedName::new(None, None, "id".to_string())))
		    }
		    _ => eprintln!("item {} not a node", i)
		}
	    }
	    assert_eq!(s.len(), 10);
	    for t in s {
		match &*t {
		    Item::Node(n) => {
			assert_eq!(n.node_type(), NodeType::Element);
			assert_eq!(n.name().to_string(), "b")
		    }
		    _ => panic!("not a node")
		}
	    }
	}
	#[test]
	fn xpath_rel_path_1() {
	    let e = parse::<$t>("child::a/child::b")
		.expect("failed to parse expression \"child::a/child::b\"");
	    let rd = $x();
	    let s = ContextBuilder::new()
		      .result_document(rd)
		      .current(vec![$y()])
		      .build()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 2);
	    for t in s {
		match &*t {
		    Item::Node(n) => {
			assert_eq!(n.node_type(), NodeType::Element);
			assert_eq!(n.name().to_string(), "b")
		    }
		    _ => panic!("not a node")
		}
	    }
	}
	#[test]
	fn xpath_rel_path_2() {
	    let e = parse::<$t>("child::a//child::b")
		.expect("failed to parse expression \"child::a//child::b\"");
	    let rd = $x();
		let s = ContextBuilder::new()
			.result_document(rd)
			.current(vec![$y()])
			.build()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 10);
	    for t in s {
		match &*t {
		    Item::Node(n) => {
			assert_eq!(n.node_type(), NodeType::Element);
			assert_eq!(n.name().to_string(), "b")
		    }
		    _ => panic!("not a node")
		}
	    }
	}
	#[test]
	fn xpath_step_1() {
	    let e = parse::<$t>("child::a")
		.expect("failed to parse expression \"child::a\"");
	    let rd = $x();
		let s = ContextBuilder::new()
			.result_document(rd)
			.current(vec![$y()])
			.build()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    for t in s {
		match &*t {
		    Item::Node(n) => {
			assert_eq!(n.node_type(), NodeType::Element);
			assert_eq!(n.name().to_string(), "a")
		    }
		    _ => panic!("not a node")
		}
	    }
	}
	#[test]
	fn xpath_step_2() {
	    let e = parse::<$t>("child::bc")
		.expect("failed to parse expression \"child::bc\"");
	    let rd = $x();
		let s = ContextBuilder::new()
			.result_document(rd)
			.current(vec![$y()])
			.build()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 0);
	}
	#[test]
	fn xpath_step_wild() {
	    let e = parse::<$t>("child::*")
		.expect("failed to parse expression \"child::*\"");
	    let rd = $x();
		let s = ContextBuilder::new()
			.result_document(rd)
			.current(vec![$y()])
			.build()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    for t in s {
		match &*t {
		    Item::Node(n) => {
			assert_eq!(n.node_type(), NodeType::Element);
			assert_eq!(n.name().to_string(), "a")
		    }
		    _ => panic!("not a node")
		}
	    }
	}

	// Functions
	#[test]
	fn xpath_fncall_string() {
	    let mut e = parse::<$t>("string(('a', 'b', 'c'))")
		.expect("failed to parse expression \"string(('a', 'b', 'c'))\"");
	    let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.to_string(), "abc")
	}
	#[test]
	fn xpath_fncall_concat() {
	    let mut e = parse::<$t>("concat('a', 'b', 'c')")
		.expect("failed to parse expression \"concat('a', 'b', 'c')\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.to_string(), "abc")
	}
	#[test]
	fn xpath_fncall_startswith_pos() {
	    let mut e = parse::<$t>("starts-with('abc', 'a')")
		.expect("failed to parse expression \"starts-with('abc', 'a')\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.to_bool(), true)
	}
	#[test]
	fn xpath_fncall_startswith_neg() {
	    let mut e = parse::<$t>("starts-with('abc', 'b')")
		.expect("failed to parse expression \"starts-with('abc', 'a')\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.to_bool(), false)
	}
	#[test]
	fn xpath_fncall_contains_pos() {
	    let mut e = parse::<$t>("contains('abc', 'b')")
		.expect("failed to parse expression \"contains('abc', 'b')\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.to_bool(), true)
	}
	#[test]
	fn xpath_fncall_contains_neg() {
	    let mut e = parse::<$t>("contains('abc', 'd')")
		.expect("failed to parse expression \"contains('abc', 'd')\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.to_bool(), false)
	}
	#[test]
	fn xpath_fncall_substring_2arg() {
	    let mut e = parse::<$t>("substring('abcdefg', 4)")
		.expect("failed to parse expression \"substring('abcdefg', 4)\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.to_string(), "defg")
	}
	#[test]
	fn xpath_fncall_substring_3arg() {
	    let mut e = parse::<$t>("substring('abcdefg', 4, 2)")
		.expect("failed to parse expression \"substring('abcdefg', 4, 2)\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.to_string(), "de")
	}
	#[test]
	fn xpath_fncall_substringbefore_pos() {
	    let mut e = parse::<$t>("substring-before('abc', 'b')")
		.expect("failed to parse expression \"substring-before('abc', 'b')\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.to_string(), "a")
	}
	#[test]
	fn xpath_fncall_substringbefore_neg() {
	    let mut e = parse::<$t>("substring-before('abc', 'd')")
		.expect("failed to parse expression \"substring-before('abc', 'd')\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.to_string(), "")
	}
	#[test]
	fn xpath_fncall_substringafter_pos_1() {
	    let mut e = parse::<$t>("substring-after('abc', 'b')")
		.expect("failed to parse expression \"substring-after('abc', 'b')\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.to_string(), "c")
	}
	#[test]
	fn xpath_fncall_substringafter_pos_2() {
	    let mut e = parse::<$t>("substring-after('abc', 'c')")
		.expect("failed to parse expression \"substring-after('abc', 'b')\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.to_string(), "")
	}
	#[test]
	fn xpath_fncall_substringafter_neg() {
	    let mut e = parse::<$t>("substring-after('abc', 'd')")
		.expect("failed to parse expression \"substring-after('abc', 'd')\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.to_string(), "")
	}
	#[test]
	fn xpath_fncall_normalizespace() {
	    let mut e = parse::<$t>("normalize-space('	a  b\nc 	')")
		.expect("failed to parse expression \"normalize-space('	a  b\nc 	')\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.to_string(), "a b c")
	}
	#[test]
	fn xpath_fncall_translate() {
	    let mut e = parse::<$t>("translate('abcdeabcde', 'ade', 'XY')")
		.expect("failed to parse expression \"translate('abcdeabcde', 'ade', 'XY')\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.to_string(), "XbcYXbcY")
	}
	#[test]
	fn xpath_fncall_boolean_true() {
	    let mut e = parse::<$t>("boolean('abcdeabcde')")
		.expect("failed to parse expression \"boolean('abcdeabcde')\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
		_ => panic!("not a singleton boolean true value")
	    }
	}
	#[test]
	fn xpath_fncall_boolean_false() {
	    let mut e = parse::<$t>("boolean('')")
		.expect("failed to parse expression \"boolean('')\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Boolean(b)) => assert_eq!(b, false),
		_ => panic!("not a singleton boolean false value")
	    }
	}
	#[test]
	fn xpath_fncall_not_true() {
	    let mut e = parse::<$t>("not('')")
		.expect("failed to parse expression \"not('')\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
		_ => panic!("not a singleton boolean true value")
	    }
	}
	#[test]
	fn xpath_fncall_not_false() {
	    let mut e = parse::<$t>("not('abc')")
		.expect("failed to parse expression \"not('abc')\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Boolean(b)) => assert_eq!(b, false),
		_ => panic!("not a singleton boolean false value")
	    }
	}
	#[test]
	fn xpath_fncall_true() {
	    let mut e = parse::<$t>("true()")
		.expect("failed to parse expression \"true()\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
		_ => panic!("not a singleton boolean true value")
	    }
	}
	#[test]
	fn xpath_fncall_false() {
	    let mut e = parse::<$t>("false()")
		.expect("failed to parse expression \"false()\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Boolean(b)) => assert_eq!(b, false),
		_ => panic!("not a singleton boolean false value")
	    }
	}
	#[test]
	fn xpath_fncall_number_int() {
	    let mut e = parse::<$t>("number('123')")
		.expect("failed to parse expression \"number('123')\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Integer(i)) => assert_eq!(i, 123),
		_ => panic!("not a singleton integer value, got \"{}\"", s.to_string())
	    }
	}
	#[test]
	fn xpath_fncall_number_double() {
	    let mut e = parse::<$t>("number('123.456')")
		.expect("failed to parse expression \"number('123.456')\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Double(d)) => assert_eq!(d, 123.456),
		_ => panic!("not a singleton double value")
	    }
	}
	#[test]
	fn xpath_fncall_sum() {
	    let mut e = parse::<$t>("sum(('123.456', 10, 20, '0'))")
		.expect("failed to parse expression \"sum(('123.456', 10, 20, '0'))\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Double(d)) => assert_eq!(d, 123.456 + 10.0 + 20.0),
		_ => panic!("not a singleton double value")
	    }
	}
	#[test]
	fn xpath_fncall_floor() {
	    let mut e = parse::<$t>("floor(123.456)")
		.expect("failed to parse expression \"floor(123.456)\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Double(d)) => assert_eq!(d, 123.0),
		_ => panic!("not a singleton double value")
	    }
	}
	#[test]
	fn xpath_fncall_ceiling() {
	    let mut e = parse::<$t>("ceiling(123.456)")
		.expect("failed to parse expression \"ceiling(123.456)\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Double(d)) => assert_eq!(d, 124.0),
		_ => panic!("not a singleton double value")
	    }
	}
	#[test]
	fn xpath_fncall_round_down() {
	    let mut e = parse::<$t>("round(123.456)")
		.expect("failed to parse expression \"round(123.456)\"");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    match *s[0] {
		Item::Value(Value::Double(d)) => assert_eq!(d, 123.0),
		_ => panic!("not a singleton double value")
	    }
	}
	#[test]
	fn xpath_fncall_round_up() {
	    let mut e = parse::<$t>("round(123.654)")
		.expect("failed to parse expression \"round(123.654)\"");
		let s = Context::new()
			.dispatch(&e)
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
	    let mut e = parse::<$t>("let $x := 'a' return ($x, $x)")
		.expect("failed to parse let expression");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.to_string(), "aa")
	}
	#[test]
	fn xpath_let_2() {
	    let mut e = parse::<$t>("let $x := 'a', $y := 'b' return ($x, $y)")
		.expect("failed to parse let expression");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 2);
	    assert_eq!(s.to_string(), "ab")
	}

	// Loops
	#[test]
	fn xpath_for_1() {
	    let mut e = parse::<$t>("for $x in ('a', 'b', 'c') return ($x, $x)")
		.expect("failed to parse for expression");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 6);
	    assert_eq!(s.to_string(), "aabbcc")
	}
	#[test]
	fn xpath_for_2() {
	    let mut e = parse::<$t>("for $x in (1, 2, 3) return $x * 2")
		.expect("failed to parse for expression");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 3);
	    assert_eq!(s.to_string(), "246")
	}

	#[test]
	fn xpath_if_1() {
	    let mut e = parse::<$t>("if (1) then 'one' else 'not one'")
		.expect("failed to parse if expression");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s.to_string(), "one")
	}
	#[test]
	fn xpath_if_2() {
	    let mut e = parse::<$t>("if (0) then 'one' else 'not one'")
		.expect("failed to parse if expression");
		let s = Context::new()
			.dispatch(&e)
			.expect("evaluation failed");
	    assert_eq!(s.len(), 1);
	    assert_eq!(s.to_string(), "not one")
	}

	}

);
