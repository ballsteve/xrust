#[macro_export]
macro_rules! evaluate_tests (
    ( $x:expr ) => {
	use chrono::{Local, Datelike, Timelike};
	#[cfg(test)]
	use rust_decimal_macros::dec;
	use xrust::value::{Operator};
	use xrust::item::{NodeType};
	use xrust::evaluate::{
	    Constructor, Evaluator,
	    Function, Param,
	    ArithmeticOperand, ArithmeticOperator,
	    func_true, func_false,
	    func_boolean, func_not,
	    func_translate, func_normalizespace,
	    func_count, func_last, func_position,
	    func_string,
	    func_substring, func_substringafter, func_substringbefore,
	    func_startswith,
	    func_contains, func_concat,
	    func_sum, func_number,
	    func_round, func_ceiling, func_floor,
	    func_current_time, func_current_date, func_current_date_time,
	    func_format_time, func_format_date, func_format_date_time,
	};

	#[test]
	fn literal_string() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let cons = vec![Constructor::Literal(Value::from("foobar"))];
	    let s = e.evaluate(None, None, &cons, &rd)
		.expect("evaluation failed");
	    if s.len() == 1 {
		assert_eq!(s.to_string(), "foobar")
	    } else {
		panic!("sequence is not a singleton")
	    }
	}

	#[test]
	fn literal_int() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let cons = vec![Constructor::Literal(Value::Integer(456))];
	    let s = e.evaluate(None, None, &cons, &rd)
		.expect("evaluation failed");
	    if s.len() == 1 {
		assert_eq!(s[0].to_int().unwrap(), 456)
	    } else {
		panic!("sequence is not a singleton")
	    }
	}

	#[test]
	fn literal_decimal() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let cons = vec![Constructor::Literal(Value::Decimal(dec!(34.56)))];
	    let s = e.evaluate(None, None, &cons, &rd)
		.expect("evaluation failed");
	    if s.len() == 1 {
		assert_eq!(s.to_string(), "34.56")
	    } else {
		panic!("sequence is not a singleton")
	    }
	}

	#[test]
	fn literal_bool() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let cons = vec![Constructor::Literal(Value::from(false))];
	    let s = e.evaluate(None, None, &cons, &rd)
		.expect("evaluation failed");
	    if s.len() == 1 {
		assert_eq!(s.to_bool(), false)
	    } else {
		panic!("sequence is not a singleton")
	    }
	}

	#[test]
	fn literal_double() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let cons = vec![Constructor::Literal(Value::from(4.56))];
	    let s = e.evaluate(None, None, &cons, &rd)
		.expect("evaluation failed");
	    if s.len() == 1 {
		assert_eq!(s[0].to_double(), 4.56)
	    } else {
		panic!("sequence is not a singleton")
	    }
	}

	#[test]
	fn sequence_literal() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let cons = vec![
		Constructor::Literal(Value::from("foo")),
		Constructor::Literal(Value::from("bar")),
	    ];
	    let s = e.evaluate(None, None, &cons, &rd)
		.expect("evaluation failed");
	    if s.len() == 2 {
		assert_eq!(s.to_string(), "foobar")
	    } else {
		panic!("sequence does not have two items")
	    }
	}

	#[test]
	fn sequence_literal_mixed() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let cons = vec![
		Constructor::Literal(Value::from("foo")),
		Constructor::Literal(Value::Integer(123)),
	    ];
	    let s = e.evaluate(None, None, &cons, &rd)
		.expect("evaluation failed");
	    if s.len() == 2 {
		assert_eq!(s.to_string(), "foo123")
	    } else {
		panic!("sequence does not have two items")
	    }
	}

	#[test]
	fn context_item() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let s = vec![Rc::new(Item::Value(Value::from("foobar")))];
	    let cons = vec![Constructor::ContextItem];
	    let result = e.evaluate(Some(s), Some(0), &cons, &rd)
		.expect("evaluation failed");
	    if result.len() == 1 {
		assert_eq!(result[0].to_string(), "foobar")
	    } else {
		panic!("sequence is not a singleton")
	    }
	}

	#[test]
	fn context_item_2() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let cons = vec![
		Constructor::ContextItem,
		Constructor::ContextItem,
	    ];
	    let result = e.evaluate(Some(vec![Rc::new(Item::Value(Value::from("foobar")))]), Some(0), &cons, &rd)
		.expect("evaluation failed");
	    if result.len() == 2 {
		assert_eq!(result.to_string(), "foobarfoobar")
	    } else {
		panic!("sequence does not have two items")
	    }
	}

	#[test]
	fn or() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let cons = vec![
		Constructor::Or(
		    vec![
			vec![Constructor::Literal(Value::from(true))],
			vec![Constructor::Literal(Value::from(false))],
		    ]
		)
	    ];
	    let s = e.evaluate(None, None, &cons, &rd)
		.expect("evaluation failed");
	    if s.len() == 1 {
		assert_eq!(s.to_bool(), true)
	    } else {
		panic!("sequence is not a singleton")
	    }
	}
	// TODO: test more than two operands

	#[test]
	fn and() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let cons = vec![
		Constructor::And(
		    vec![
			vec![Constructor::Literal(Value::from(true))],
			vec![Constructor::Literal(Value::from(false))],
		    ]
		)
	    ];
	    let s = e.evaluate(None, None, &cons, &rd)
		.expect("evaluation failed");
	    if s.len() == 1 {
		assert_eq!(s.to_bool(), false)
	    } else {
		panic!("sequence is not a singleton")
	    }
	}
	// TODO: test more than two operands

	#[test]
	fn value_comparison_int_true() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let cons = vec![
		Constructor::ValueComparison(
		    Operator::Equal,
		    vec![
			vec![Constructor::Literal(Value::Integer(1))],
			vec![Constructor::Literal(Value::Integer(1))],
		    ]
		)
	    ];
	    let s = e.evaluate(None, None, &cons, &rd)
		.expect("evaluation failed");
	    if s.len() == 1 {
		assert_eq!(s.to_bool(), true)
	    } else {
		panic!("sequence is not a singleton")
	    }
	}
	// TODO: negative test: more than two operands
	#[test]
	fn value_comparison_int_false() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let cons = vec![
		Constructor::ValueComparison(
		    Operator::Equal,
		    vec![
			vec![Constructor::Literal(Value::Integer(1))],
			vec![Constructor::Literal(Value::Integer(2))],
		    ]
		)
	    ];
	    let s = e.evaluate(None, None, &cons, &rd)
		.expect("evaluation failed");
	    if s.len() == 1 {
		assert_eq!(s.to_bool(), false)
	    } else {
		panic!("sequence is not a singleton")
	    }
	}
	// TODO: negative test: more than two operands
	#[test]
	fn value_comparison_string_true() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let cons = vec![
		Constructor::ValueComparison(
		    Operator::Equal,
		    vec![
			vec![Constructor::Literal(Value::from("foo"))],
			vec![Constructor::Literal(Value::from("foo"))],
		    ]
		)
	    ];
	    let s = e.evaluate(None, None, &cons, &rd)
		.expect("evaluation failed");
	    if s.len() == 1 {
		assert_eq!(s.to_bool(), true)
	    } else {
		panic!("sequence is not a singleton")
	    }
	}
	// TODO: negative test: more than two operands
	#[test]
	fn value_comparison_string_false() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let cons = vec![
		Constructor::ValueComparison(
		    Operator::Equal,
		    vec![
			vec![Constructor::Literal(Value::from("foo"))],
			vec![Constructor::Literal(Value::from("bar"))],
		    ]
		)
	    ];
	    let s = e.evaluate(None, None, &cons, &rd)
		.expect("evaluation failed");
	    if s.len() == 1 {
		assert_eq!(s.to_bool(), false)
	    } else {
		panic!("sequence is not a singleton")
	    }
	}
	// TODO: negative test: more than two operands
	// TODO: compare other data types, mixed data types
	// TODO: other value comparisons: notequal, lt, gt, etc

	#[test]
	fn general_comparison_string_true() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let cons = vec![
		Constructor::GeneralComparison(
		    Operator::Equal,
		    vec![
			vec![Constructor::Literal(Value::from("foo"))],
			vec![
			    Constructor::Literal(Value::from("bar")),
			    Constructor::Literal(Value::from("foo")),
			]
		    ]
		)
	    ];
	    let s = e.evaluate(None, None, &cons, &rd)
		.expect("evaluation failed");
	    if s.len() == 1 {
		assert_eq!(s.to_bool(), true)
	    } else {
		panic!("sequence is not a singleton")
	    }
	}
	#[test]
	fn general_comparison_string_false() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let cons = vec![
		Constructor::GeneralComparison(
		    Operator::Equal,
		    vec![
			vec![Constructor::Literal(Value::from("foo"))],
			vec![
			    Constructor::Literal(Value::from("bar")),
			    Constructor::Literal(Value::from("oof")),
			]
		    ]
		)
	    ];
	    let s = e.evaluate(None, None, &cons, &rd)
		.expect("evaluation failed");
	    if s.len() == 1 {
		assert_eq!(s.to_bool(), false)
	    } else {
		panic!("sequence is not a singleton")
	    }
	}
	// TODO: test multi-item first sequence against multi-item second sequence; mixed types, etc

	#[test]
	fn concat() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let cons = vec![
		Constructor::Concat(
		    vec![
			vec![Constructor::Literal(Value::from("foo"))],
			vec![
			    Constructor::Literal(Value::from("bar")),
			    Constructor::Literal(Value::from("oof")),
			]
		    ]
		)
	    ];
	    let s = e.evaluate(None, None, &cons, &rd)
		.expect("evaluation failed");
	    if s.len() == 1 {
		assert_eq!(s.to_string(), "foobaroof")
	    } else {
		panic!("sequence is not a singleton")
	    }
	}

	#[test]
	fn range() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let cons = vec![
		Constructor::Range(
		    vec![
			vec![Constructor::Literal(Value::Integer(0))],
			vec![Constructor::Literal(Value::Integer(9))],
		    ]
		)
	    ];
	    let s = e.evaluate(None, None, &cons, &rd)
		.expect("evaluation failed");
	    if s.len() == 10 {
		assert_eq!(s.to_string(), "0123456789")
	    } else {
		panic!("sequence does not have 10 items")
	    }
	}
	// TODO: ranges resulting in empty sequence, start = end, negative tests

	#[test]
	fn arithmetic_double_add() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let cons = vec![
		Constructor::Arithmetic(
		    vec![
			ArithmeticOperand{
			    op: ArithmeticOperator::Noop,
			    operand: vec![Constructor::Literal(Value::from(1.0))]
			},
			ArithmeticOperand{
			    op: ArithmeticOperator::Add,
			    operand: vec![Constructor::Literal(Value::from(1.0))]
			}
		    ]
		)
	    ];
	    let s = e.evaluate(None, None, &cons, &rd)
		.expect("evaluation failed");
	    if s.len() == 1 {
		assert_eq!(s[0].to_double(), 2.0)
	    } else {
		panic!("sequence is not a singleton")
	    }
	}
	// TODO: ranges resulting in empty sequence, start = end, negative tests

	// Documents and Nodes require a concrete type to test

	#[test]
	fn function_call_position() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("position".to_string(), vec![], Some(func_position)),
		vec![]
	    );
	    let s = vec![
		Rc::new(Item::Value(Value::from("a"))),
		Rc::new(Item::Value(Value::from("b"))),
	    ];
	    let vc = vec![c];
	    let r = e.evaluate(Some(s), Some(1), &vc, &rd)
		.expect("evaluation failed");
	    assert_eq!(r.to_string(), "2")
	}
	#[test]
	fn function_call_last() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("last".to_string(), vec![], Some(func_last)),
		vec![]
	    );
	    let s = vec![
		Rc::new(Item::Value(Value::from("a"))),
		Rc::new(Item::Value(Value::from("b"))),
		Rc::new(Item::Value(Value::from("c"))),
	    ];
	    let vc = vec![c];
	    let r = e.evaluate(Some(s), Some(1), &vc, &rd)
		.expect("evaluation failed");
	    assert_eq!(r.to_string(), "3")
	}
	#[test]
	fn function_call_count() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new(
		    "count".to_string(),
		    vec![Param::new("i".to_string(), "t".to_string())],
		    Some(func_count)
		),
		vec![
		    vec![
			Constructor::Literal(Value::from("a")),
			Constructor::Literal(Value::from("b")),
			Constructor::Literal(Value::from("c")),
		    ]
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd)
		.expect("evaluation failed");
	    assert_eq!(r.to_string(), "3")
	}
	#[test]
	fn function_call_string_1() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("string".to_string(), vec![], Some(func_string)),
		vec![
		    vec![
			Constructor::Literal(Value::from("a")),
			Constructor::Literal(Value::from("b")),
			Constructor::Literal(Value::from("c")),
		    ]
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd)
		.expect("evaluation failed");
	    assert_eq!(r.to_string(), "abc")
	}
	#[test]
	fn function_call_concat_1() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("concat".to_string(), vec![], Some(func_concat)),
		vec![
		    vec![Constructor::Literal(Value::from("a"))],
		    vec![Constructor::Literal(Value::from("b"))],
		    vec![Constructor::Literal(Value::from("c"))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd)
		.expect("evaluation failed");
	    assert_eq!(r.to_string(), "abc")
	}
	#[test]
	fn function_call_startswith_pos() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("starts-with".to_string(), vec![], Some(func_startswith)),
		vec![
		    vec![Constructor::Literal(Value::from("abc"))],
		    vec![Constructor::Literal(Value::from("a"))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd)
		.expect("evaluation failed");
	    assert_eq!(r.to_bool(), true)
	}
	#[test]
	fn function_call_startswith_neg() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("starts-with".to_string(), vec![], Some(func_startswith)),
		vec![
		    vec![Constructor::Literal(Value::from("abc"))],
		    vec![Constructor::Literal(Value::from("b"))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd)
		.expect("evaluation failed");
	    assert_eq!(r.to_bool(), false)
	}
	#[test]
	fn function_call_contains_pos() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("contains".to_string(), vec![], Some(func_contains)),
		vec![
		    vec![Constructor::Literal(Value::from("abc"))],
		    vec![Constructor::Literal(Value::from("b"))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd)
		.expect("evaluation failed");
	    assert_eq!(r.to_bool(), true)
	}
	#[test]
	fn function_call_contains_neg() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("contains".to_string(), vec![], Some(func_contains)),
		vec![
		    vec![Constructor::Literal(Value::from("abc"))],
		    vec![Constructor::Literal(Value::from("d"))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd)
		.expect("evaluation failed");
	    assert_eq!(r.to_bool(), false)
	}
	#[test]
	fn function_call_substring_2() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("substring".to_string(), vec![], Some(func_substring)),
		vec![
		    vec![Constructor::Literal(Value::from("abc"))],
		    vec![Constructor::Literal(Value::Integer(2))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd)
		.expect("evaluation failed");
	    assert_eq!(r.to_string(), "bc")
	}
	#[test]
	fn function_call_substring_3() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("substring".to_string(), vec![], Some(func_substring)),
		vec![
		    vec![Constructor::Literal(Value::from("abcde"))],
		    vec![Constructor::Literal(Value::Integer(2))],
		    vec![Constructor::Literal(Value::Integer(3))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd)
		.expect("evaluation failed");
	    assert_eq!(r.to_string(), "bcd")
	}
	#[test]
	fn function_call_substring_before_1() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("substring-before".to_string(), vec![], Some(func_substringbefore)),
		vec![
		    vec![Constructor::Literal(Value::from("abcde"))],
		    vec![Constructor::Literal(Value::from("bc"))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd)
		.expect("evaluation failed");
	    assert_eq!(r.to_string(), "a")
	}
	#[test]
	fn function_call_substring_before_neg() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("substring-before".to_string(), vec![], Some(func_substringbefore)),
		vec![
		    vec![Constructor::Literal(Value::from("abcde"))],
		    vec![Constructor::Literal(Value::from("fg"))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd)
		.expect("evaluation failed");
	    assert_eq!(r.to_string(), "")
	}
	#[test]
	fn function_call_substring_after_1() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("substring-after".to_string(), vec![], Some(func_substringafter)),
		vec![
		    vec![Constructor::Literal(Value::from("abcde"))],
		    vec![Constructor::Literal(Value::from("bc"))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd)
		.expect("evaluation failed");
	    assert_eq!(r.to_string(), "de")
	}
	#[test]
	fn function_call_substring_after_neg_1() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("substring-after".to_string(), vec![], Some(func_substringafter)),
		vec![
		    vec![Constructor::Literal(Value::from("abcde"))],
		    vec![Constructor::Literal(Value::from("fg"))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.to_string(), "")
	}
	#[test]
	fn function_call_substring_after_neg_2() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("substring-after".to_string(), vec![], Some(func_substringafter)),
		vec![
		    vec![Constructor::Literal(Value::from("abcde"))],
		    vec![Constructor::Literal(Value::from("de"))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.to_string(), "")
	}
	#[test]
	fn function_call_normalizespace() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("normalize-space".to_string(), vec![], Some(func_normalizespace)),
		vec![
		    vec![Constructor::Literal(Value::from("	a b   c\nd e 	"))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.to_string(), "abcde")
	}
	#[test]
	fn function_call_translate() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("translate".to_string(), vec![], Some(func_translate)),
		vec![
		    vec![Constructor::Literal(Value::from("abcdeabcde"))],
		    vec![Constructor::Literal(Value::from("ade"))],
		    vec![Constructor::Literal(Value::from("XY"))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.to_string(), "XbcYXbcY")
	}
	// TODO: test using non-ASCII characters
	#[test]
	fn function_call_boolean_true() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("boolean".to_string(), vec![], Some(func_boolean)),
		vec![
		    vec![Constructor::Literal(Value::from("abcdeabcde"))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    match *r[0] {
		Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
		_ => panic!("not a singleton boolean true value")
	    }
	}
	#[test]
	fn function_call_boolean_false() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("boolean".to_string(), vec![], Some(func_boolean)),
		vec![
		    vec![],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    match *r[0] {
		Item::Value(Value::Boolean(b)) => assert_eq!(b, false),
		_ => panic!("not a singleton boolean false value")
	    }
	}
	#[test]
	fn function_call_not_false() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("not".to_string(), vec![], Some(func_not)),
		vec![
		    vec![Constructor::Literal(Value::from(true))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    match *r[0] {
		Item::Value(Value::Boolean(b)) => assert_eq!(b, false),
		_ => panic!("not a singleton boolean false value")
	    }
	}
	#[test]
	fn function_call_not_true() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("not".to_string(), vec![], Some(func_not)),
		vec![
		    vec![],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    match *r[0] {
		Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
		_ => panic!("not a singleton boolean true value")
	    }
	}
	#[test]
	fn function_call_true() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("true".to_string(), vec![], Some(func_true)),
		vec![
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    match *r[0] {
		Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
		_ => panic!("not a singleton boolean true value")
	    }
	}
	#[test]
	fn function_call_false() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("false".to_string(), vec![], Some(func_false)),
		vec![
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    match *r[0] {
		Item::Value(Value::Boolean(b)) => assert_eq!(b, false),
		_ => panic!("not a singleton boolean false value")
	    }
	}
	#[test]
	fn function_call_number_int() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("number".to_string(), vec![], Some(func_number)),
		vec![
		    vec![Constructor::Literal(Value::from("123"))]
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    match *r[0] {
		Item::Value(Value::Integer(i)) => assert_eq!(i, 123),
		_ => panic!("not a singleton integer value")
	    }
	}
	#[test]
	fn function_call_number_double() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("number".to_string(), vec![], Some(func_number)),
		vec![
		    vec![Constructor::Literal(Value::from("123.456"))]
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    match *r[0] {
		Item::Value(Value::Double(d)) => assert_eq!(d, 123.456),
		_ => panic!("not a singleton double value")
	    }
	}
	// TODO: test NaN result
	#[test]
	fn function_call_sum() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("sum".to_string(), vec![], Some(func_sum)),
		vec![
		    vec![Constructor::Literal(Value::from("123.456")),
			 Constructor::Literal(Value::from("10")),
			 Constructor::Literal(Value::from("-20")),
			 Constructor::Literal(Value::from("0")),
		    ],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    match *r[0] {
		Item::Value(Value::Double(d)) => assert_eq!(d, 123.456 + 10.0 - 20.0),
		_ => panic!("not a singleton double value")
	    }
	}
	#[test]
	fn function_call_floor() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("floor".to_string(), vec![], Some(func_floor)),
		vec![
		    vec![Constructor::Literal(Value::from("123.456"))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    match *r[0] {
		Item::Value(Value::Double(d)) => assert_eq!(d, 123.0),
		_ => panic!("not a singleton double value")
	    }
	}
	#[test]
	fn function_call_ceiling() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("ceiling".to_string(), vec![], Some(func_ceiling)),
		vec![
		    vec![Constructor::Literal(Value::from("123.456"))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    match *r[0] {
		Item::Value(Value::Double(d)) => assert_eq!(d, 124.0),
		_ => panic!("not a singleton double value")
	    }
	}
	#[test]
	fn function_call_round_down() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("round".to_string(), vec![], Some(func_round)),
		vec![
		    vec![Constructor::Literal(Value::from("123.456"))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    match *r[0] {
		Item::Value(Value::Double(d)) => assert_eq!(d, 123.0),
		_ => panic!("not a singleton double value")
	    }
	}
	#[test]
	fn function_call_round_up() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("round".to_string(), vec![], Some(func_round)),
		vec![
		    vec![Constructor::Literal(Value::from("123.654"))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    match *r[0] {
		Item::Value(Value::Double(d)) => assert_eq!(d, 124.0),
		_ => panic!("not a singleton double value")
	    }
	}

	// Date/time related functions

	#[test]
	fn function_call_current_date() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("current-date".to_string(), vec![], Some(func_current_date)),
		vec![]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    match &*r[0] {
		Item::Value(Value::Date(d)) => {
		    assert_eq!(d.year(), Local::today().year());
		    assert_eq!(d.month(), Local::today().month());
		    assert_eq!(d.day(), Local::today().day());
		}
		_ => panic!("not a singleton date value")
	    }
	}

	#[test]
	fn function_call_current_time() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("current-time".to_string(), vec![], Some(func_current_time)),
		vec![]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    match &*r[0] {
		Item::Value(Value::Time(t)) => {
		    assert_eq!(t.hour(), Local::now().hour());
		    assert_eq!(t.minute(), Local::now().minute());
		    assert_eq!(t.second(), Local::now().second()); // It is possible for this to fail if the elapsed time to execute the function call and the test falls across a second quantum
		}
		_ => panic!("not a singleton time value")
	    }
	}

	#[test]
	fn function_call_current_date_time() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("current-dateTime".to_string(), vec![], Some(func_current_date_time)),
		vec![]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    match &*r[0] {
		Item::Value(Value::DateTime(dt)) => {
		    assert_eq!(dt.year(), Local::today().year());
		    assert_eq!(dt.month(), Local::today().month());
		    assert_eq!(dt.day(), Local::today().day());
		    assert_eq!(dt.hour(), Local::now().hour());
		    assert_eq!(dt.minute(), Local::now().minute());
		    assert_eq!(dt.second(), Local::now().second()); // It is possible for this to fail if the elapsed time to execute the function call and the test falls across a second quantum
		}
		_ => panic!("not a singleton dateTime value")
	    }
	}

	#[test]
	fn function_call_format_date() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("format-date".to_string(), vec![], Some(func_format_date)),
		vec![
		    vec![Constructor::Literal(Value::from("2022-01-03"))],
		    vec![Constructor::Literal(Value::from("[D] [M] [Y]"))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    match &*r[0] {
		Item::Value(Value::String(d)) => assert_eq!(d, "03 01 2022"),
		_ => panic!("not a singleton string value")
	    }
	}

	#[test]
	fn function_call_format_date_time() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("format-dateTime".to_string(), vec![], Some(func_format_date_time)),
		vec![
		    vec![Constructor::Literal(Value::from("2022-01-03T04:05:06.789+10:00"))],
		    vec![Constructor::Literal(Value::from("[H]:[m] [D]/[M]/[Y]"))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    match &*r[0] {
		Item::Value(Value::String(d)) => assert_eq!(d, "04:05 03/01/2022"),
		_ => panic!("not a singleton string value")
	    }
	}

	#[test]
	fn function_call_format_time() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = Constructor::FunctionCall(
		Function::new("format-time".to_string(), vec![], Some(func_format_time)),
		vec![
		    vec![Constructor::Literal(Value::from("04:05:06.789"))],
		    vec![Constructor::Literal(Value::from("[H]:[m]:[s]"))],
		]
	    );
	    let vc = vec![c];
	    let r = e.evaluate(None, None, &vc, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    match &*r[0] {
		Item::Value(Value::String(d)) => assert_eq!(d, "04:05:06"),
		_ => panic!("not a singleton string value")
	    }
	}

	// Variables
	#[test]
	fn var_ref() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = vec![
		Constructor::VariableDeclaration("foo".to_string(), vec![Constructor::Literal(Value::from("my variable"))]),
		Constructor::VariableReference("foo".to_string()),
	    ];
	    let r = e.evaluate(None, None, &c, &rd).expect("evaluation failed");
	    assert_eq!(r.to_string(), "my variable")
	}

	// Loops
	#[test]
	fn loop_1() {
	    let e = Evaluator::new();
	    let rd = $x();
	    // This is "for $x() in ('a', 'b', 'c') return $x()"
	    let c = vec![
		Constructor::Loop(
		    vec![Constructor::VariableDeclaration(
			"x".to_string(),
			vec![
			    Constructor::Literal(Value::from("a")),
			    Constructor::Literal(Value::from("b")),
			    Constructor::Literal(Value::from("c")),
			]
		    )],
		    vec![Constructor::VariableReference("x".to_string())]
		)
	    ];
	    let r = e.evaluate(None, None, &c, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 3);
	    assert_eq!(r.to_string(), "abc")
	}

	// Switch
	#[test]
	fn switch_1() {
	    let e = Evaluator::new();
	    let rd = $x();
	    // implements "if (1) then 'one' else 'not one'"
	    let c = vec![
		Constructor::Switch(
		    vec![
			vec![
			    Constructor::Literal(Value::Integer(1))
			],
			vec![
			    Constructor::Literal(Value::from("one"))
			]
		    ],
		    vec![Constructor::Literal(Value::from("not one"))]
		)
	    ];
	    let r = e.evaluate(None, None, &c, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    assert_eq!(r.to_string(), "one")
	}
	#[test]
	fn switch_2() {
	    let e = Evaluator::new();
	    let rd = $x();
	    // implements "if (0) then 'one' else 'not one'"
	    let c = vec![
		Constructor::Switch(
		    vec![
			vec![
			    Constructor::Literal(Value::Integer(0))
			],
			vec![
			    Constructor::Literal(Value::from("one"))
			]
		    ],
		    vec![Constructor::Literal(Value::from("not one"))]
		)
	    ];
	    let r = e.evaluate(None, None, &c, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    assert_eq!(r.to_string(), "not one")
	}
	#[test]
	fn switch_3() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = vec![
		Constructor::Switch(
		    vec![
			vec![
			    Constructor::Literal(Value::Integer(0))
			],
			vec![
			    Constructor::Literal(Value::from("one"))
			],
			vec![
			    Constructor::Literal(Value::Integer(1))
			],
			vec![
			    Constructor::Literal(Value::from("two"))
			],
			vec![
			    Constructor::Literal(Value::Integer(0))
			],
			vec![
			    Constructor::Literal(Value::from("three"))
			],
		    ],
		    vec![Constructor::Literal(Value::from("not any"))]
		)
	    ];
	    let r = e.evaluate(None, None, &c, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    assert_eq!(r.to_string(), "two")
	}
	// The first clause to pass should return the result
	#[test]
	fn switch_4() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = vec![
		Constructor::Switch(
		    vec![
			vec![
			    Constructor::Literal(Value::Integer(0))
			],
			vec![
			    Constructor::Literal(Value::from("one"))
			],
			vec![
			    Constructor::Literal(Value::Integer(1))
			],
			vec![
			    Constructor::Literal(Value::from("two"))
			],
			vec![
			    Constructor::Literal(Value::Integer(1))
			],
			vec![
			    Constructor::Literal(Value::from("three"))
			],
		    ],
		    vec![Constructor::Literal(Value::from("not any"))]
		)
	    ];
	    let r = e.evaluate(None, None, &c, &rd).expect("evaluation failed");
	    assert_eq!(r.len(), 1);
	    assert_eq!(r.to_string(), "two")
	}

	// Patterns
	// Need a concrete type to test patterns

	// Templates
	// Need a concrete type to test patterns

	// Literal result element
	#[test]
	fn literal_result() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = vec![
		Constructor::LiteralElement(
		    QualifiedName::new(None, None, String::from("Test")),
		    vec![]
		)
	    ];
	    let r = e.evaluate(None, None, &c, &rd).expect("evaluation failed");
	    assert_eq!(r.to_xml(), "<Test></Test>")
	}
	#[test]
	fn literal_result_text() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = vec![
		Constructor::LiteralElement(
		    QualifiedName::new(None, None, String::from("Test")),
		    vec![
			Constructor::Literal(Value::from("data"))
		    ]
		)
	    ];
	    let r = e.evaluate(None, None, &c, &rd).expect("evaluation failed");
	    assert_eq!(r.to_xml(), "<Test>data</Test>")
	}
	#[test]
	fn literal_result_content() {
	    let e = Evaluator::new();
	    let rd = $x();
	    let c = vec![
		Constructor::LiteralElement(
		    QualifiedName::new(None, None, String::from("Test")),
		    vec![
			Constructor::Literal(Value::from("data")),
			Constructor::LiteralElement(
			    QualifiedName::new(None, None, String::from("Level-1")),
			    vec![
				Constructor::Literal(Value::from("deeper"))
			    ]
			)
		    ]
		)
	    ];
	    let r = e.evaluate(None, None, &c, &rd).expect("evaluation failed");
	    assert_eq!(r.to_xml(), "<Test>data<Level-1>deeper</Level-1></Test>")
	}

	// for-each, for-each-group

    }
);
