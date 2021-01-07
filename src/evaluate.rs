//! # xdm::evaluate
//!
//! Evaluate a sequence constructor.

use crate::xdmerror::*;
use crate::item::*;
use crate::xpath::parse;
use crate::sequence::{effective_boolean_value, stringvalue};

pub struct DynamicContext {
  pub context_item: Option<Item>, // in some circumstances there is no context item
}

pub type SequenceConstructorFunc = fn(&DynamicContext, Option<Vec<Vec<SequenceConstructor>>>, Option<Item>) -> Result<Vec<Item>, Error>;

// TODO: define a factory function to create a new object and initialise fields to None
#[derive(Clone)]
pub struct SequenceConstructor {
  pub func: SequenceConstructorFunc,		// the function to evaluate to construct the sequence
  pub data: Option<Item>,			// literal data for the constructor
  pub args: Option<Vec<Vec<SequenceConstructor>>>,	// arguments for the constructor
}

// Comparison operators
#[derive(Copy, Clone)]
enum Operator {
  Equal,
  NotEqual,
  LessThan,
  LessThanEqual,
  GreaterThan,
  GreaterThanEqual,
  Is,
  Before,
  After,
}

pub fn cons_literal(_d: &DynamicContext, _s: Option<Vec<Vec<SequenceConstructor>>>, i: Option<Item>) -> Result<Vec<Item>, Error> {
  match i {
    Some(j) => {
      let mut seq = Vec::new();
      seq.push(j.clone()); // pass a reference?
      Ok(seq)
    }
    None => Result::Err(Error{kind: ErrorKind::Unknown, message: format!("no item supplied")}),
  }
}

pub fn cons_context_item(d: &DynamicContext, _s: Option<Vec<Vec<SequenceConstructor>>>, _i: Option<Item>) -> Result<Vec<Item>, Error> {
  match &d.context_item {
    Some(c) => {
      let mut seq = Vec::new();
      seq.push(c.clone()); // TODO: pass a reference, rather than cloning
      Ok(seq)
    }
    None => Result::Err(Error{kind: ErrorKind::Unknown, message: format!("no context item")}),
  }
}

// Evaluate each operand to a boolean result. Return true if any of the operands' result is true
// Optimsation: stop upon the first true result.
// Future: Evaluate every operand to check for dynamic errors
pub fn cons_or(d: &DynamicContext, s: Option<Vec<Vec<SequenceConstructor>>>, _i: Option<Item>) -> Result<Vec<Item>, Error> {
  let mut b = false;
  match s {
    Some(t) => {
      for v in t {
        let r = eval(v, d).expect("evaluating operand failed");
	//let x = stringvalue(&r);
        b = effective_boolean_value(r);
        //println!("cons_or: evaluate operand \"{}\" to {}", x, b);
        if b {break};
      };
      Ok(vec![Item::Value(Value::Boolean(b))])
    },
    None => Ok(vec![Item::Value(Value::Boolean(false))]) // Rather than panic!, just return false
  }
}

// Evaluate each operand to a boolean result. Return false if any of the operands' result is false
// Optimsation: stop upon the first false result.
// Future: Evaluate every operand to check for dynamic errors
pub fn cons_and(d: &DynamicContext, s: Option<Vec<Vec<SequenceConstructor>>>, _i: Option<Item>) -> Result<Vec<Item>, Error> {
  let mut b = true;
  match s {
    Some(t) => {
      for v in t {
        let r = eval(v, d).expect("evaluating operand failed");
	//let x = stringvalue(&r);
        b = effective_boolean_value(r);
        //println!("cons_and: evaluate operand \"{}\" to {}", x, b);
        if !b {break};
      };
      Ok(vec![Item::Value(Value::Boolean(b))])
    },
    None => Ok(vec![Item::Value(Value::Boolean(false))]) // Rather than panic!, just return false
  }
}

// Evaluate each operand to a sequence result. Calculate the union of all sequences.
// Future: Evaluate every operand to check for dynamic errors
pub fn cons_union(d: &DynamicContext, s: Option<Vec<Vec<SequenceConstructor>>>, _i: Option<Item>) -> Result<Vec<Item>, Error> {
  // TODO
  Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not yet implemented"),})
}

// Evaluate each operand to a sequence result. Perform operations on the sequences.
// Future: Evaluate every operand to check for dynamic errors
pub fn cons_intersectexcept(d: &DynamicContext, s: Option<Vec<Vec<SequenceConstructor>>>, _i: Option<Item>) -> Result<Vec<Item>, Error> {
  // TODO
  Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not yet implemented"),})
}

// General comparisons evaluate each operand to a sequence.
// The items in the sequences are all then compared using the given operator
macro_rules! general_cmp {
  ( $x:ident, $y:expr ) => {
    pub fn $x (d: &DynamicContext, s: Option<Vec<Vec<SequenceConstructor>>>, _i: Option<Item>) -> Result<Vec<Item>, Error> {
      match s {
        Some(t) => {
	  if t.len() == 2 {
	    general_comparison(d, &t[0], &t[1], $y)
	  } else {
	    panic!("need exactly two sequence constructors")
	  }
	},
	None => panic!("no sequence constructors"),
      }
    }
  };
}
general_cmp!(comparison_general_equal, Operator::Equal);
general_cmp!(comparison_general_notequal, Operator::NotEqual);
general_cmp!(comparison_general_lessthan, Operator::LessThan);
general_cmp!(comparison_general_lessthanequal, Operator::LessThanEqual);
general_cmp!(comparison_general_greaterthan, Operator::GreaterThan);
general_cmp!(comparison_general_greaterthanequal, Operator::GreaterThanEqual);

fn general_comparison(d: &DynamicContext, left: &Vec<SequenceConstructor>, right: &Vec<SequenceConstructor>, op: Operator) -> Result<Vec<Item>, Error> {
  let mut b = false;
  let left_seq = eval_ref(&left, d).expect("evaluating left-hand sequence failed");
  let right_seq = eval_ref(&right, d).expect("evaluating right-hand sequence failed");
  for l in &left_seq {
    for r in &right_seq {
      b = item_compare(&l, &r, op);
      if b { break }
    }
    if b { break }
  };
  Ok(vec![Item::Value(Value::Boolean(b))])
}

macro_rules! value_cmp {
  ( $x:ident, $y:expr ) => {
    pub fn $x (d: &DynamicContext, s: Option<Vec<Vec<SequenceConstructor>>>, _i: Option<Item>) -> Result<Vec<Item>, Error> {
      match s {
        Some(t) => {
	  if t.len() == 2 {
	    value_comparison(d, t[0].clone(), t[1].clone(), $y)
	  } else {
	    panic!("need exactly two sequence constructors")
	  }
	}
	None => panic!("no sequence constructors"),
      }
    }
  }
}
value_cmp!(comparison_value_equal, Operator::Equal);
value_cmp!(comparison_value_notequal, Operator::NotEqual);
value_cmp!(comparison_value_lessthan, Operator::LessThan);
value_cmp!(comparison_value_lessthanequal, Operator::LessThanEqual);
value_cmp!(comparison_value_greaterthan, Operator::GreaterThan);
value_cmp!(comparison_value_greaterthanequal, Operator::GreaterThanEqual);

// Operands must be singletons
fn value_comparison(d: &DynamicContext, left: Vec<SequenceConstructor>, right: Vec<SequenceConstructor>, op: Operator) -> Result<Vec<Item>, Error> {
  let left_seq = eval_ref(&left, d).expect("evaluating left-hand sequence failed");
  if left_seq.len() == 1 {
    let right_seq = eval_ref(&right, d).expect("evaluating right-hand sequence failed");
    if right_seq.len() == 1 {
      Ok(vec![Item::Value(Value::Boolean(item_compare(&left_seq[0], &right_seq[0], op)))])
    } else {
      Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("not a singleton sequence"),})
    }
  } else {
    Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("not a singleton sequence"),})
  }
}

// TODO: type coersion
// TODO: will probably have to implement comparison in the item module (as a trait?)
fn item_compare(left: &Item, right: &Item, op: Operator) -> bool {
  match op {
    Operator::Equal => left == right,
    Operator::NotEqual => left != right,
    Operator::LessThan => left < right,
    Operator::LessThanEqual => left <= right,
    Operator::GreaterThan => left > right,
    Operator::GreaterThanEqual => left >= right,
    Operator::Is => false,	//
    Operator::Before => false,	// TODO: Not yet implemented
    Operator::After => false,	//
  }
}

// TODO
pub fn comparison_node_is(_d: &DynamicContext, _s: Option<Vec<Vec<SequenceConstructor>>>, _i: Option<Item>) -> Result<Vec<Item>, Error> {
  Result::Err(Error{kind: ErrorKind::NotImplemented, message: String::from("not yet implemented")})
}
// TODO
pub fn comparison_node_before(_d: &DynamicContext, _s: Option<Vec<Vec<SequenceConstructor>>>, _i: Option<Item>) -> Result<Vec<Item>, Error> {
  Result::Err(Error{kind: ErrorKind::NotImplemented, message: String::from("not yet implemented")})
}
// TODO
pub fn comparison_node_after(_d: &DynamicContext, _s: Option<Vec<Vec<SequenceConstructor>>>, _i: Option<Item>) -> Result<Vec<Item>, Error> {
  Result::Err(Error{kind: ErrorKind::NotImplemented, message: String::from("not yet implemented")})
}

// Concatenate all of the operand's string values.
pub fn cons_string_concat(d: &DynamicContext, s: Option<Vec<Vec<SequenceConstructor>>>, _i: Option<Item>) -> Result<Vec<Item>, Error> {
  let mut r = String::from("");
  match s {
    Some(t) => {
      for v in t {
        let q = eval(v, d).expect("evaluating operand failed");
	//let x = stringvalue(&r);
	r.push_str(stringvalue(&q).as_str());
      };
      Ok(vec![Item::Value(Value::String(r))])
    },
    None => Ok(vec![Item::Value(Value::String(r))])
  }
}

pub fn cons_range(d: &DynamicContext, s: Option<Vec<Vec<SequenceConstructor>>>, _i: Option<Item>) -> Result<Vec<Item>, Error> {
  match s {
    Some(t) => {
      if t.len() == 2 {
        // Evaluate the two operands: they must both be literal integer items
	let start = eval_ref(&t[0], d).expect("evaluating start operand failed");
	let end = eval_ref(&t[1], d).expect("evaluating end operand failed");
	if start.len() == 0 || end.len() == 0 {
	  Ok(Vec::new())
	} else if start.len() == 1 {
	  if end.len() == 1 {
	    match start[0] {
	      Item::Value(Value::Integer(u)) => {
	        match end[0] {
	          Item::Value(Value::Integer(v)) => {
		    if u > v {
		      Ok(Vec::new())
		    } else if u == v {
		      Ok(vec![Item::Value(Value::Integer(u))])
		    } else {
		      let mut r = Vec::new();
		      for i in u..=v {
		        r.push(Item::Value(Value::Integer(i)))
		      }
		      Ok(r)
		    }
		  }
	      	  _ => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("end operand must be literal integer")})
		}
	      }
	      _ => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("start operand must be literal integer")})
	    }
	  } else {
	    Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("end operand must be singleton")})
	  }
	} else {
	  Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("start operand must be singleton")})
	}
      } else {
        Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("need exactly two operands")})
      }
    }
    None => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("no operands")})
  }
}

// Each item in the tuple is a pair
pub fn addsub(d: &DynamicContext, s: Option<Vec<Vec<SequenceConstructor>>>, _i: Option<Item>) -> Result<Vec<Item>, Error> {
  match s {
    Some(t) => {
      if t.len() % 2 == 0 {

	// Type: the result will be a number, but integer or double?
	// If all of the operands are integers, then the result is integer otherwise double
	// TODO: check the type of all operands to determine type of result
	// In the meantime, let's assume the result will be double and convert any integers

	let mut acc: f64 = 0.0;

        for j in t.chunks(2) {
	  let v = eval_ref(&j[1], d).expect("evaluating operand failed");
	  let u: f64;

	  if v.len() != 1 {
	    return Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("type error (not a singleton sequence)")});
	  } else {
	    match v[0] {
	      Item::Value(Value::Integer(w)) => {
	        u = w as f64
	      }
	      Item::Value(Value::Double(d)) => {
	        u = d
	      }
	      _ => {
	        return Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("type error (not a numeric value)")});
	      }
	    }
	    let o = stringvalue(&eval_ref(&j[0], d).expect("evaluating operator failed"));
            match o.as_str() {
	      "" => {
	        acc = u; // value must be singleton numeric value
	      }
	      "+" => {
	        acc += u; // value must be singleton numeric value
	      }
	      "-" => {
	        acc -= u; // value must be singleton numeric value
	      }
	      _ => {
	        return Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("invalid operator")});
	      }
	    }
	  }
	}
	Ok(vec![Item::Value(Value::Double(acc))])
      } else {
        Result::Err(Error{kind: ErrorKind::Unknown, message: String::from(format!("wrong number of operands: {} operands", t.len()))})
      }
    }
    None => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("no operands")})
  }
}
pub fn muldiv(d: &DynamicContext, s: Option<Vec<Vec<SequenceConstructor>>>, _i: Option<Item>) -> Result<Vec<Item>, Error> {
  match s {
    Some(t) => {
      if t.len() % 2 == 0 {

	// Type: the result will be a number, but integer or double?
	// If all of the operands are integers, then the result is integer otherwise double
	// TODO: check the type of all operands to determine type of result
	// In the meantime, let's assume the result will be double and convert any integers

	let mut acc: f64 = 0.0;

        for j in t.chunks(2) {
	  let v = eval_ref(&j[1], d).expect("evaluating operand failed");
	  let u: f64;

	  if v.len() != 1 {
	    return Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("type error (not a singleton sequence)")});
	  } else {
	    match v[0] {
	      Item::Value(Value::Integer(w)) => {
	        u = w as f64
	      }
	      Item::Value(Value::Double(d)) => {
	        u = d
	      }
	      _ => {
	        return Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("type error (not a numeric value)")});
	      }
	    }
	    let o = stringvalue(&eval_ref(&j[0], d).expect("evaluating operator failed"));
            match o.as_str() {
	      "" => {
	        acc = u; // value must be singleton numeric value
	      }
	      "*" => {
	        acc *= u; // value must be singleton numeric value
	      }
	      "div" => {
	        acc /= u; // value must be singleton numeric value
	      }
	      "idiv" => {
	        acc /= u; // TODO: convert to integer
	      }
	      "mod" => {
	        acc = acc % u; // value must be singleton numeric value
	      }
	      _ => {
	        return Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("invalid operator")});
	      }
	    }
	  }
	}
	Ok(vec![Item::Value(Value::Double(acc))])
      } else {
        Result::Err(Error{kind: ErrorKind::Unknown, message: String::from(format!("wrong number of operands: {} operands", t.len()))})
      }
    }
    None => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("no operands")})
  }
}

pub fn eval(cons: Vec<SequenceConstructor>, ctxt: &DynamicContext) -> Result<Vec<Item>, Error> {
  let mut ret = Vec::new();

  for i in cons {
    let seq = (i.func)(ctxt, i.args, i.data).expect("evaluation failed");
    for j in seq {
      ret.push(j);
    }
  }

  Ok(ret)
}

// TODO: consider making SequenceConstructor reference counted: cloning/copying will be expensive for large items or long sequences
pub fn eval_ref(cons: &Vec<SequenceConstructor>, ctxt: &DynamicContext) -> Result<Vec<Item>, Error> {
  let mut ret = Vec::new();

  for i in cons {
    let seq = (i.func)(ctxt, i.args.clone(), i.data.clone()).expect("evaluation failed");
    for j in seq {
      ret.push(j);
    }
  }

  Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Just evaluate

    #[test]
    fn eval_literal() {
      let d = DynamicContext {
        context_item: None,
      };
      let seq = cons_literal(&d, None, Some(Item::Value(Value::Integer(456)))).expect("unable to construct literal");
      if seq.len() == 1 {
        match seq[0] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 456),
	  _ => panic!("item is not a literal integer value")
	}
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn eval_context_item() {
      let d = DynamicContext {
        context_item: Some(Item::Value(Value::Integer(123))),
      };
      let seq = cons_context_item(&d, None, None).expect("unable to construct context_item");
      if seq.len() == 1 {
        match seq[0] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 123),
	  _ => panic!("item is not a literal integer value")
	}
      } else {
        panic!("sequence is not a singleton")
      }
    }

    // Sequence constructor

    #[test]
    fn cons_singleton_literal_sequence() {
      let d = DynamicContext {
        context_item: None,
      };
      let mut c = Vec::new();
      c.push(SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::Integer(1))), args: None});
      // should result in singleton sequence integer item 1
      let s = eval(c, &d).expect("failed to evaluate sequence constructor");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 1),
	  _ => panic!("item is not a literal integer value")
	}
      } else {
        panic!("not a singleton sequence")
      }
    }

    #[test]
    fn cons_sequence() {
      let d = DynamicContext {
        context_item: Some(Item::Value(Value::Integer(123))),
      };
      let mut c = Vec::new();
      c.push(SequenceConstructor{func: cons_context_item, data: None, args: None});
      c.push(SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::Integer(456))), args: None});
      // should result in sequence of length 2
      let s = eval(c, &d).expect("failed to evaluate sequence constructor");
      if s.len() == 2 {
        match s[0] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 123),
	  _ => panic!("item is not a literal integer value")
	}
        match s[1] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 456),
	  _ => panic!("item is not a literal integer value")
	}
      } else {
        panic!("not a sequence of two items")
      }
    }

    // Parse then evaluate

    #[test]
    fn empty_sequence() {
      let d = DynamicContext {
        context_item: Some(Item::Value(Value::Integer(123))),
      };
      let s = eval(parse("()").expect("failed to parse expression \"()\""), &d).expect("failed to evaluate expression \"()\""); // should result in empty sequence
      assert_eq!(s.len(), 0)
    }

    #[test]
    fn parse_singleton_literal_sequence() {
      let d = DynamicContext {
        context_item: Some(Item::Value(Value::Integer(123))),
      };
      let s = eval(parse("1").expect("failed to parse expression \"1\""), &d).expect("failed to evaluate expression \"1\""); // should result in singleton sequence integer item 1
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 1),
	  _ => panic!("item is not a literal integer value")
	}
      } else {
        panic!("not a singleton sequence")
      }
    }

    #[test]
    fn parse_singleton_context_item_sequence() {
      let d = DynamicContext {
        context_item: Some(Item::Value(Value::Integer(123))),
      };
      let s = eval(parse(".").expect("failed to parse expression \".\""), &d).expect("failed to evaluate expression \".\""); // should result in singleton sequence integer item 123
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 123),
	  _ => panic!("item is not a literal integer value")
	}
      } else {
        panic!("not a singleton sequence")
      }
    }

    #[test]
    fn parse_literal_sequence() {
      let d = DynamicContext {
        context_item: Some(Item::Value(Value::Integer(123))),
      };
      let s = eval(parse("1, 'abc', 2").expect("failed to parse expression \"1, 'abc', 2\""), &d).expect("failed to evaluate expression \"1, 'abc', 2\""); // should result in 3 item sequence
      if s.len() == 3 {
        match s[0] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 1),
	  _ => panic!("item is not a literal integer value")
	}
        match &s[1] {
	  Item::Value(Value::String(v)) => assert_eq!(v, "abc"),
	  _ => panic!("item is not a literal string value")
	}
        match s[2] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 2),
	  _ => panic!("item is not a literal integer value")
	}
      } else {
        panic!("sequence does not have 3 items")
      }
    }

    #[test]
    fn parse_literal_sequence_with_context_item() {
      let d = DynamicContext {
        context_item: Some(Item::Value(Value::Integer(123))),
      };
      let s = eval(parse("'abc', ., 456").expect("failed to parse expression \"'abc', ., 456\""), &d).expect("failed to evaluate expression \"'abc', ., 456\""); // should result in the sequence ('abc', 123, 456)
      if s.len() == 3 {
        match s[1] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 123),
	  _ => panic!("item is not a literal integer value")
	}
        match &s[0] {
	  Item::Value(Value::String(v)) => assert_eq!(v, "abc"),
	  _ => panic!("item is not a literal string value")
	}
        match s[2] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 456),
	  _ => panic!("item is not a literal integer value")
	}
      } else {
        panic!("sequence does not have 3 items")
      }
    }

    #[test]
    fn parse_multi_context_item_sequence() {
      let d = DynamicContext {
        context_item: Some(Item::Value(Value::Integer(123))),
      };
      let s = eval(parse(".,.").expect("failed to parse expression \".,.\""), &d).expect("failed to evaluate expression \".,.\""); // should result in the sequence (123, 123)
      if s.len() == 2 {
        match s[0] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 123),
	  _ => panic!("item is not a literal integer value")
	}
        match s[1] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 123),
	  _ => panic!("item is not a literal integer value")
	}
      } else {
        panic!("sequence does not have 2 items")
      }
    }

    #[test]
    fn parse_or_int_1() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("0 or 1").expect("failed to parse expression \"0 or 1\""), &d).expect("failed to evaluate expression \"0 or 1\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
	  _ => panic!("item is not a literal boolean value")
	}
      } else {
        panic!("sequence does not have 1 item")
      }
    }
    #[test]
    fn parse_or_int_0() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("0 or 0").expect("failed to parse expression \"0 or 0\""), &d).expect("failed to evaluate expression \"0 or 0\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Boolean(b)) => assert_eq!(b, false),
	  _ => panic!("item is not a literal boolean value")
	}
      } else {
        panic!("sequence does not have 1 item")
      }
    }
    #[test]
    fn parse_or_multi_1() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("0 or 1.0 or 'abc'").expect("failed to parse expression \"0 or 1.0 or 'abc'\""), &d).expect("failed to evaluate expression \"0 or 1.0 or 'abc'\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
	  _ => panic!("item is not a literal boolean value")
	}
      } else {
        panic!("sequence does not have 1 item")
      }
    }

    #[test]
    fn parse_and_int_0() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("0 and 1").expect("failed to parse expression \"0 and 1\""), &d).expect("failed to evaluate expression \"0 and 1\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Boolean(b)) => assert_eq!(b, false),
	  _ => panic!("item is not a literal boolean value")
	}
      } else {
        panic!("sequence does not have 1 item")
      }
    }
    #[test]
    fn parse_and_int_1() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("1 and 1").expect("failed to parse expression \"1 and 1\""), &d).expect("failed to evaluate expression \"1 and 1\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
	  _ => panic!("item is not a literal boolean value")
	}
      } else {
        panic!("sequence does not have 1 item")
      }
    }
    #[test]
    fn parse_and_multi_1() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("1 and 1.0 and 'abc'").expect("failed to parse expression \"1 and 1.0 and 'abc'\""), &d).expect("failed to evaluate expression \"1 and 1.0 and 'abc'\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
	  _ => panic!("item is not a literal boolean value")
	}
      } else {
        panic!("sequence does not have 1 item")
      }
    }

    #[test]
    fn value_eq_int_true() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("1 eq 1").expect("failed to parse expression \"1 eq 1\""), &d).expect("failed to evaluate expression \"1 eq 1\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
	  _ => panic!("item is not a literal boolean value")
	}
      } else {
        panic!("sequence does not have 1 item")
      }
    }
    #[test]
    fn value_eq_string_true() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("'abc' eq 'abc'").expect("failed to parse expression \"'abc' eq 'abc'\""), &d).expect("failed to evaluate expression \"'abc' eq 'abc'\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
	  _ => panic!("item is not a literal boolean value")
	}
      } else {
        panic!("sequence does not have 1 item")
      }
    }
    #[test]
    fn value_eq_int_false() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("1 eq 0").expect("failed to parse expression \"1 eq 0\""), &d).expect("failed to evaluate expression \"1 eq 0\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Boolean(b)) => assert_eq!(b, false),
	  _ => panic!("item is not a literal boolean value")
	}
      } else {
        panic!("sequence does not have 1 item")
      }
    }
    #[test]
    fn value_eq_string_false() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("'abc' eq 'def'").expect("failed to parse expression \"'abc' eq 'def'\""), &d).expect("failed to evaluate expression \"'abc' eq 'def'\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Boolean(b)) => assert_eq!(b, false),
	  _ => panic!("item is not a literal boolean value")
	}
      } else {
        panic!("sequence does not have 1 item")
      }
    }
    #[test]
    fn general_eq_int_true() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("(1, 2) = (0, 1)").expect("failed to parse expression \"(1, 2) = (0, 1)\""), &d).expect("failed to evaluate expression \"(1, 2) = (0, 1)\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
	  _ => panic!("item is not a literal boolean value")
	}
      } else {
        panic!("sequence does not have 1 item")
      }
    }
    #[test]
    fn general_eq_int_false() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("(1, 2) = (0, 3)").expect("failed to parse expression \"(1, 2) = (0, 3)\""), &d).expect("failed to evaluate expression \"(1, 2) = (0, 3)\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Boolean(b)) => assert_eq!(b, false),
	  _ => panic!("item is not a literal boolean value")
	}
      } else {
        panic!("sequence does not have 1 item")
      }
    }

    #[test]
    fn parse_string_concat() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("1 || 'abc'").expect("failed to parse expression \"1 || 'abc'\""), &d).expect("failed to evaluate expression \"1 || 'abc'\"");
      if s.len() == 1 {
        match &s[0] {
	  Item::Value(Value::String(r)) => assert_eq!(r, "1abc"),
	  _ => panic!("item is not a literal string value")
	}
      } else {
        panic!("sequence does not have 1 item")
      }
    }
    #[test]
    fn parse_string_concat_multi() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("1 || 'abc' || 2.3").expect("failed to parse expression \"1 || 'abc' || 2.3\""), &d).expect("failed to evaluate expression \"1 || 'abc' || 2.3\"");
      if s.len() == 1 {
        match &s[0] {
	  Item::Value(Value::String(r)) => assert_eq!(r, "1abc2.3"),
	  _ => panic!("item is not a literal string value")
	}
      } else {
        panic!("sequence does not have 1 item")
      }
    }

    #[test]
    fn parse_range() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("1 to 3").expect("failed to parse expression \"1 to 3\""), &d).expect("failed to evaluate expression \"1 to 3\"");
      if s.len() == 3 {
        match s[0] {
	  Item::Value(Value::Integer(r)) => assert_eq!(r, 1),
	  _ => panic!("item is not a literal integer value")
	};
        match s[1] {
	  Item::Value(Value::Integer(r)) => assert_eq!(r, 2),
	  _ => panic!("item is not a literal integer value")
	};
        match s[2] {
	  Item::Value(Value::Integer(r)) => assert_eq!(r, 3),
	  _ => panic!("item is not a literal integer value")
	}
      } else {
        panic!(format!("sequence does not have 3 items, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_range_single() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("2 to 2").expect("failed to parse expression \"2 to 2\""), &d).expect("failed to evaluate expression \"2 to 2\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Integer(r)) => assert_eq!(r, 2),
	  _ => panic!("item is not a literal integer value")
	};
      } else {
        panic!(format!("sequence does not have 1 items, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_range_empty() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("1 to ()").expect("failed to parse expression \"1 to ()\""), &d).expect("failed to evaluate expression \"1 to ()\"");
      assert_eq!(s.len(), 0)
    }
    #[test]
    fn parse_range_gt() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("10 to 1").expect("failed to parse expression \"10 to 1\""), &d).expect("failed to evaluate expression \"10 to 1\"");
      assert_eq!(s.len(), 0)
    }

    #[test]
    fn parse_addsub_plus_2() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("1 + 1").expect("failed to parse expression \"1 + 1\""), &d).expect("failed to evaluate expression \"1 + 1\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Double(r)) => assert_eq!(r, 2.0),
	  _ => panic!("item is not a literal double value")
	};
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_addsub_plus_3() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("1 + 1 + 2").expect("failed to parse expression \"1 + 1 + 2\""), &d).expect("failed to evaluate expression \"1 + 1 + 2\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Double(r)) => assert_eq!(r, 4.0),
	  _ => panic!("item is not a literal double value")
	};
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_addsub_minus_2() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("3 - 1").expect("failed to parse expression \"3 - 1\""), &d).expect("failed to evaluate expression \"3 - 1\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Double(r)) => assert_eq!(r, 2.0),
	  _ => panic!("item is not a literal double value")
	};
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_addsub_minus_3() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("10 - 5 - 2").expect("failed to parse expression \"10 - 5 - 2\""), &d).expect("failed to evaluate expression \"10 - 5 - 2\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Double(r)) => assert_eq!(r, 3.0),
	  _ => panic!("item is not a literal double value")
	};
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_addsub_mix() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("10 + 20 - 5 + 2").expect("failed to parse expression \"10 + 20 - 5 + 2\""), &d).expect("failed to evaluate expression \"10 + 20 - 5 + 2\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Double(r)) => assert_eq!(r, 27.0),
	  _ => panic!("item is not a literal double value")
	};
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }

    #[test]
    fn parse_multiply_2() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("2 * 3").expect("failed to parse expression \"2 * 3\""), &d).expect("failed to evaluate expression \"2 * 3\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Double(r)) => assert_eq!(r, 6.0),
	  _ => panic!("item is not a literal double value")
	};
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_multiply_3() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("2 * 3 * 4").expect("failed to parse expression \"2 * 3 * 4\""), &d).expect("failed to evaluate expression \"2 * 3 * 4\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Double(r)) => assert_eq!(r, 24.0),
	  _ => panic!("item is not a literal double value")
	};
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_divide_2() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("3 div 2").expect("failed to parse expression \"3 div 2\""), &d).expect("failed to evaluate expression \"3 div 2\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Double(r)) => assert_eq!(r, 1.5),
	  _ => panic!("item is not a literal double value")
	};
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_divide_3() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("100 div 10 div 2").expect("failed to parse expression \"100 div 10 div 2\""), &d).expect("failed to evaluate expression \"100 div 10 div 2\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Double(r)) => assert_eq!(r, 5.0),
	  _ => panic!("item is not a literal double value")
	};
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_mod_2() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("3 mod 2").expect("failed to parse expression \"3 mod 2\""), &d).expect("failed to evaluate expression \"3 mod 2\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Double(r)) => assert_eq!(r, 1.0),
	  _ => panic!("item is not a literal double value")
	};
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_muldiv_mix() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("6.5 * 2 div 0.5").expect("failed to parse expression \"6.5 * 2 div 0.5\""), &d).expect("failed to evaluate expression \"6.5 * 2 div 0.5\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Double(r)) => assert_eq!(r, 26.0),
	  _ => panic!("item is not a literal double value")
	};
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_arithmetic_precedence() {
      let d = DynamicContext {
        context_item: None,
      };
      let s = eval(parse("1 + 2 * 3 - 4").expect("failed to parse expression \"1 + 2 * 3 - 4\""), &d).expect("failed to evaluate expression \"1 + 2 * 3 - 4\"");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Double(r)) => assert_eq!(r, 3.0),
	  _ => panic!("item is not a literal double value")
	};
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
} 

