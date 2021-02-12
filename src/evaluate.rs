//! # xdm::evaluate
//!
//! Evaluate a sequence constructor.

use dyn_clone::clone_box;
use crate::xdmerror::*;
use crate::item::*;
use crate::xpath::parse;
//use crate::sequence::*;

pub struct DynamicContext<'a> {
  pub context_item: Option<Box<dyn Item<'a> + 'a>>, // in some circumstances there is no context item
}

impl<'a> DynamicContext<'a> {
  pub fn new() -> DynamicContext<'a> {
    DynamicContext{context_item: None}
  }
  pub fn set_context_item(mut self, i: Box<dyn Item<'a> + 'a>) -> Self {
    self.context_item = Some(i);
    self
  }
  pub fn reset_context_item(mut self) -> Self {
    self.context_item = None;
    self
  }
}

pub type SequenceConstructorFunc<'a> = fn(&'a DynamicContext<'a>, Option<Vec<Vec<SequenceConstructor<'a>>>>, Option<Box<dyn Item<'a> + 'a>>, Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error>;

// TODO: define a factory function to create a new object and initialise fields to None
pub struct SequenceConstructor<'a> {
  pub func: SequenceConstructorFunc<'a>,		// the function to evaluate to construct the sequence
  pub data: Option<Box<dyn Item<'a> + 'a>>,			// literal data for the constructor
  pub args: Option<Vec<Vec<SequenceConstructor<'a>>>>,	// arguments for the constructor
  pub nodematch: Option<NodeMatch>,			// match nodes in the document
}

impl<'a> SequenceConstructor<'a> {
  pub fn new(f: SequenceConstructorFunc) -> SequenceConstructor {
    SequenceConstructor{func: f, data: None, args: None, nodematch: None}
  }
  pub fn set_data(mut self, d: Option<Box<dyn Item<'a> + 'a>>) -> Self {
    self.data = d;
    self
  }
  pub fn set_args(mut self, a: Option<Vec<Vec<SequenceConstructor<'a>>>>) -> Self {
    self.args = a;
    self
  }
  pub fn set_nodematch(mut self, n: Option<NodeMatch>) -> Self {
    self.nodematch = n;
    self
  }
}

#[derive(Clone)]
pub struct NodeMatch {
  pub axis: Axis,
  pub nodetest: NodeTest,
}

#[derive(Clone)]
pub enum NodeTest {
  Kind,
  Name(NameTest),
}

#[derive(Clone)]
pub struct NameTest {
  pub ns: Option<WildcardOrName>,
  pub prefix: Option<String>,
  pub name: Option<WildcardOrName>,
}

#[derive(Clone)]
pub enum WildcardOrName {
  Wildcard,
  Name(String),
}

#[derive(Clone)]
pub enum Axis {
  Child,
  Descendant,
  DescendantOrSelf,
  Attribute,
  Selfaxis,
  Following,
  FollowingOrSelf,
  Namespace,
  Parent,
  Ancestor,
  AncestorOrSelf,
  Preceding,
  PrecedingOrSelf,
  Unknown,
}

impl Axis {
  pub fn from(s: &str) -> Axis {
    match s {
      "child" => Axis::Child,
      "descendant" => Axis::Descendant,
      "descendant-or-self" => Axis::DescendantOrSelf,
      "attribute" => Axis::Attribute,
      "self" => Axis::Selfaxis,
      "following" => Axis::Following,
      "following-or-self" => Axis::FollowingOrSelf,
      "namespace" => Axis::Namespace,
      "parent" => Axis::Parent,
      "ancestor" => Axis::Ancestor,
      "ancestor-or-self" => Axis::AncestorOrSelf,
      "preceding" => Axis::Preceding,
      "preceding-or-self" => Axis::PrecedingOrSelf,
      _ => Axis::Unknown,
    }
  }
}

pub fn cons_literal<'a>(_d: &DynamicContext,
			_s: Option<Vec<Vec<SequenceConstructor>>>,
			i: Option<Box<dyn Item<'a> + 'a>>,
			_m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  match i {
    Some(j) => {
      Ok(vec![j])
    }
    None => Result::Err(Error{kind: ErrorKind::Unknown, message: format!("no item supplied")}),
  }
}

pub fn cons_context_item<'a>(d: &'a DynamicContext<'a>, _s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  match d.context_item {
    Some(c) => {
      Ok(vec![dyn_clone::clone_box(&*c)])
    }
    None => Result::Err(Error{kind: ErrorKind::Unknown, message: format!("no context item")}),
  }
}

// Evaluate each operand to a boolean result. Return true if any of the operands' result is true
// Optimsation: stop upon the first true result.
// Future: Evaluate every operand to check for dynamic errors
pub fn cons_or<'a>(d: &'a DynamicContext<'a>, s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  let mut b = false;
  match s {
    Some(t) => {
      for v in t {
        let r = eval(v, d).expect("evaluating operand failed");
	//let x = to_string(&r);
        b = r.to_bool();
        //println!("cons_or: evaluate operand \"{}\" to {}", x, b);
        if b {break};
      };
      Ok(vec![Box::new(Value::Boolean(b))])
    },
    None => Ok(vec![Box::new(Value::Boolean(false))]) // Rather than panic!, just return false
  }
}

// Evaluate each operand to a boolean result. Return false if any of the operands' result is false
// Optimsation: stop upon the first false result.
// Future: Evaluate every operand to check for dynamic errors
pub fn cons_and<'a>(d: &'a DynamicContext<'a>, s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  let mut b = true;
  match s {
    Some(t) => {
      for v in t {
        let r = eval(v, d).expect("evaluating operand failed");
	//let x = to_string(&r);
        b = r.to_bool();
        //println!("cons_and: evaluate operand \"{}\" to {}", x, b);
        if !b {break};
      };
      Ok(vec![Box::new(Value::Boolean(b))])
    },
    None => Ok(vec![Box::new(Value::Boolean(false))]) // Rather than panic!, just return false
  }
}

// Evaluate each operand to a sequence result. Calculate the union of all sequences.
// Future: Evaluate every operand to check for dynamic errors
pub fn cons_union<'a>(_d: &'a DynamicContext<'a>, _s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  // TODO
  Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not yet implemented"),})
}

// Evaluate each operand to a sequence result. Perform operations on the sequences.
// Future: Evaluate every operand to check for dynamic errors
pub fn cons_intersectexcept<'a>(_d: &'a DynamicContext<'a>, _s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  // TODO
  Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not yet implemented"),})
}

// Evaluate each operand to a sequence result.
// Future: Evaluate every operand to check for dynamic errors
pub fn cons_instanceof<'a>(_d: &'a DynamicContext<'a>, _s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  // TODO
  Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not yet implemented"),})
}

// Evaluate each operand to a sequence result.
// Future: Evaluate every operand to check for dynamic errors
pub fn cons_treat<'a>(_d: &'a DynamicContext<'a>, _s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  // TODO
  Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not yet implemented"),})
}

// Evaluate each operand to a sequence result.
// Future: Evaluate every operand to check for dynamic errors
pub fn cons_castable<'a>(_d: &'a DynamicContext<'a>, _s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  // TODO
  Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not yet implemented"),})
}

// Evaluate each operand to a sequence result.
// Future: Evaluate every operand to check for dynamic errors
pub fn cons_cast<'a>(_d: &'a DynamicContext<'a>, _s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  // TODO
  Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not yet implemented"),})
}

// Evaluate each operand to a sequence result.
// Future: Evaluate every operand to check for dynamic errors
pub fn cons_arrow<'a>(_d: &'a DynamicContext<'a>, _s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  // TODO
  Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not yet implemented"),})
}

// Evaluate each operand to a sequence result. All but last operands are +/- values. Last operand is value to operate upon.
pub fn cons_unary<'a>(_d: &'a DynamicContext<'a>, _s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  // TODO
  Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not yet implemented"),})
}

// Evaluate each operand to a sequence result.
// Future: Evaluate every operand to check for dynamic errors
pub fn cons_simplemap<'a>(_d: &'a DynamicContext<'a>, _s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  // TODO
  Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not yet implemented"),})
}

// Set dynamic context to document root.
pub fn cons_root<'a>(_d: &'a DynamicContext<'a>, _s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  // TODO
  Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not yet implemented"),})
}
// Return child nodes
pub fn cons_child<'a>(_d: &'a DynamicContext<'a>, _s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  // TODO
  Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not yet implemented"),})
}
// Return descendant-or-self nodes
pub fn cons_descendant_or_self<'a>(_d: &'a DynamicContext<'a>, _s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  // TODO
  Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not yet implemented"),})
}
// Evaluate step
pub fn cons_step<'a>(_d: &'a DynamicContext<'a>, _s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  // TODO
  Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not yet implemented"),})
}

// Evaluate each operand to a sequence result. Each operand changes the current context.
pub fn cons_relativepath<'a>(_d: &'a DynamicContext<'a>, _s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  // TODO
  Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not yet implemented"),})
}

// General comparisons evaluate each operand to a sequence.
// The items in the sequences are all then compared using the given operator
macro_rules! general_cmp {
  ( $x:ident, $y:expr ) => {
    pub fn $x<'a> (d: &'a DynamicContext<'a>, s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
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

fn general_comparison<'a>(d: &'a DynamicContext<'a>, left: &Vec<SequenceConstructor<'a>>, right: &Vec<SequenceConstructor<'a>>, op: Operator) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  let mut b = false;
  let left_seq = eval_ref(&left, d).expect("evaluating left-hand sequence failed");
  let right_seq = eval_ref(&right, d).expect("evaluating right-hand sequence failed");
  for l in left_seq {
    for r in right_seq {
      b = l.compare(r, op).unwrap();
      if b { break }
    }
    if b { break }
  };
  Ok(vec![Box::new(Value::Boolean(b))])
}

macro_rules! value_cmp {
  ( $x:ident, $y:expr ) => {
    pub fn $x<'a> (d: &'a DynamicContext<'a>, s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
      match s {
        Some(t) => {
	  if t.len() == 2 {
	    value_comparison(d, t[0], t[1], $y)
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
fn value_comparison<'a>(d: &'a DynamicContext<'a>, left: Vec<SequenceConstructor<'a>>, right: Vec<SequenceConstructor<'a>>, op: Operator) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  let left_seq = eval_ref(&left, d).expect("evaluating left-hand sequence failed");
  if left_seq.len() == 1 {
    let right_seq = eval_ref(&right, d).expect("evaluating right-hand sequence failed");
    if right_seq.len() == 1 {
      Ok(vec![Box::new(Value::Boolean(left_seq[0].compare(right_seq[0], op).unwrap()))])
    } else {
      Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("not a singleton sequence"),})
    }
  } else {
    Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("not a singleton sequence"),})
  }
}

// TODO
pub fn comparison_node_is<'a>(_d: &'a DynamicContext<'a>, _s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  Result::Err(Error{kind: ErrorKind::NotImplemented, message: String::from("not yet implemented")})
}
// TODO
pub fn comparison_node_before<'a>(_d: &'a DynamicContext<'a>, _s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  Result::Err(Error{kind: ErrorKind::NotImplemented, message: String::from("not yet implemented")})
}
// TODO
pub fn comparison_node_after<'a>(_d: &'a DynamicContext<'a>, _s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  Result::Err(Error{kind: ErrorKind::NotImplemented, message: String::from("not yet implemented")})
}

// Concatenate all of the operand's string values.
pub fn cons_string_concat<'a>(d: &'a DynamicContext<'a>, s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  let mut r = String::from("");
  match s {
    Some(t) => {
      for v in t {
        let q = eval(v, d).expect("evaluating operand failed");
	//let x = to_string(&r);
	r.push_str(q.to_string().as_str());
      };
      Ok(vec![Box::new(Value::String(r))])
    },
    None => Ok(vec![Box::new(Value::String(r))])
  }
}

pub fn cons_range<'a>(d: &'a DynamicContext<'a>, s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
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
	    let u = start[0].to_int().unwrap();
	    let v = end[0].to_int().unwrap();
	    if u > v {
		Ok(Vec::new())
	    } else if u == v {
		Ok(vec![Box::new(Value::Integer(u))])
	    } else {
		let mut r: Vec<Box<dyn Item<'a> + 'a>> = Vec::new();
		for i in u..=v {
		    r.push(Box::new(Value::Integer(i)))
		}
		Ok(r)
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
pub fn addsub<'a>(d: &'a DynamicContext<'a>, s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
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
	    u = v[0].to_double().unwrap();
	    let o = &eval_ref(&j[0], d).expect("evaluating operator failed").to_string();
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
	Ok(vec![Box::new(Value::Double(acc))])
      } else {
        Result::Err(Error{kind: ErrorKind::Unknown, message: String::from(format!("wrong number of operands: {} operands", t.len()))})
      }
    }
    None => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("no operands")})
  }
}
pub fn muldiv<'a>(d: &'a DynamicContext<'a>, s: Option<Vec<Vec<SequenceConstructor<'a>>>>, _i: Option<Box<dyn Item<'a> + 'a>>, _m: Option<NodeMatch>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
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
	    u = v[0].to_double().unwrap();
	    let o = &eval_ref(&j[0], d).expect("evaluating operator failed").to_string();
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
	Ok(vec![Box::new(Value::Double(acc))])
      } else {
        Result::Err(Error{kind: ErrorKind::Unknown, message: String::from(format!("wrong number of operands: {} operands", t.len()))})
      }
    }
    None => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("no operands")})
  }
}

pub fn eval<'a>(cons: Vec<SequenceConstructor<'a>>, ctxt: &'a DynamicContext<'a>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  let mut ret = Vec::new();

  for i in cons {
    let seq = (i.func)(ctxt, i.args, i.data, i.nodematch).expect("evaluation failed");
    for j in seq {
      ret.push(j);
    }
  }

  Ok(ret)
}

// TODO: consider making SequenceConstructor reference counted: cloning/copying will be expensive for large items or long sequences
pub fn eval_ref<'a>(cons: &Vec<SequenceConstructor<'a>>, ctxt: &'a DynamicContext<'a>) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
  let mut ret = Vec::new();

  for i in cons {
    let seq = (i.func)(ctxt, i.args, i.data, i.nodematch).expect("evaluation failed");
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
      let seq = cons_literal(&DynamicContext::new(), None, Some(Box::new(Value::Integer(456))), None).expect("unable to construct literal");
      if seq.len() == 1 {
        assert_eq!(seq[0].to_int().unwrap(), 456)
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn eval_context_item() {
      let seq = cons_context_item(&DynamicContext::new().set_context_item(Box::new(Value::Integer(123))),
          None, None, None).expect("unable to construct context_item");
      if seq.len() == 1 {
        assert_eq!(seq[0].to_int().unwrap(), 123)
      } else {
        panic!("sequence is not a singleton")
      }
    }

    // Sequence constructor

    #[test]
    fn cons_singleton_literal_sequence() {
      let mut c = Vec::new();
      c.push(SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::Integer(1)))));
      // should result in singleton sequence integer item 1
      let s = eval(c, &DynamicContext::new()).expect("failed to evaluate sequence constructor");
      if s.len() == 1 {
        assert_eq!(s[0].to_int().unwrap(), 1)
      } else {
        panic!("not a singleton sequence")
      }
    }

    #[test]
    fn cons_sequence() {
      let d = DynamicContext {
        context_item: Some(Box::new(Value::Integer(123))),
      };
      let mut c = Vec::new();
      c.push(SequenceConstructor::new(cons_context_item));
      c.push(SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::Integer(456)))));
      // should result in sequence of length 2
      let s = eval(c, &d).expect("failed to evaluate sequence constructor");
      if s.len() == 2 {
        assert_eq!(s[0].to_int().unwrap(), 123);
        assert_eq!(s[1].to_int().unwrap(), 456)
      } else {
        panic!("not a sequence of two items")
      }
    }

    // Parse then evaluate

    #[test]
    fn empty_sequence() {
      let s = eval(parse("()").expect("failed to parse expression \"()\""), &DynamicContext::new().set_context_item(Box::new(Value::Integer(123)))).expect("failed to evaluate expression \"()\""); // should result in empty sequence
      assert_eq!(s.len(), 0)
    }

    #[test]
    fn parse_singleton_literal_sequence() {
      let s = eval(parse("1").expect("failed to parse expression \"1\""), &DynamicContext::new().set_context_item(Box::new(Value::Integer(123)))).expect("failed to evaluate expression \"1\""); // should result in singleton sequence integer item 1
      if s.len() == 1 {
        assert_eq!(s[0].to_int().unwrap(), 1)
      } else {
        panic!("not a singleton sequence")
      }
    }

    #[test]
    fn parse_singleton_context_item_sequence() {
      let s = eval(parse(".").expect("failed to parse expression \".\""), &DynamicContext::new().set_context_item(Box::new(Value::Integer(123)))).expect("failed to evaluate expression \".\""); // should result in singleton sequence integer item 123
      if s.len() == 1 {
        assert_eq!(s[0].to_int().unwrap(), 123)
      } else {
        panic!("not a singleton sequence")
      }
    }

    #[test]
    fn parse_literal_sequence() {
      let s = eval(parse("1, 'abc', 2").expect("failed to parse expression \"1, 'abc', 2\""), &DynamicContext::new().set_context_item(Box::new(Value::Integer(123)))).expect("failed to evaluate expression \"1, 'abc', 2\""); // should result in 3 item sequence
      if s.len() == 3 {
        assert_eq!(s[0].to_int().unwrap(), 1);
        assert_eq!(s[1].to_string(), "abc");
        assert_eq!(s[2].to_int().unwrap(), 2)
      } else {
        panic!("sequence does not have 3 items")
      }
    }

    #[test]
    fn parse_literal_sequence_with_context_item() {
      let s = eval(parse("'abc', ., 456").expect("failed to parse expression \"'abc', ., 456\""), &DynamicContext::new().set_context_item(Box::new(Value::Integer(123)))).expect("failed to evaluate expression \"'abc', ., 456\""); // should result in the sequence ('abc', 123, 456)
      if s.len() == 3 {
        assert_eq!(s[1].to_int().unwrap(), 123);
        assert_eq!(s[0].to_string(), "abc");
	assert_eq!(s[2].to_int().unwrap(), 456)
      } else {
        panic!("sequence does not have 3 items")
      }
    }

    #[test]
    fn parse_multi_context_item_sequence() {
      let s = eval(parse(".,.").expect("failed to parse expression \".,.\""), &DynamicContext::new().set_context_item(Box::new(Value::Integer(123)))).expect("failed to evaluate expression \".,.\""); // should result in the sequence (123, 123)
      if s.len() == 2 {
        assert_eq!(s[0].to_int().unwrap(), 123);
        assert_eq!(s[1].to_int().unwrap(), 123)
      } else {
        panic!("sequence does not have 2 items")
      }
    }

    #[test]
    fn parse_or_int_1() {
      let s = eval(parse("0 or 1").expect("failed to parse expression \"0 or 1\""), &DynamicContext::new()).expect("failed to evaluate expression \"0 or 1\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_bool(), true)
      } else {
        panic!("sequence does not have 1 item")
      }
    }
    #[test]
    fn parse_or_int_0() {
      let s = eval(parse("0 or 0").expect("failed to parse expression \"0 or 0\""), &DynamicContext::new()).expect("failed to evaluate expression \"0 or 0\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_bool(), false)
      } else {
        panic!("sequence does not have 1 item")
      }
    }
    #[test]
    fn parse_or_multi_1() {
      let s = eval(parse("0 or 1.0 or 'abc'").expect("failed to parse expression \"0 or 1.0 or 'abc'\""), &DynamicContext::new()).expect("failed to evaluate expression \"0 or 1.0 or 'abc'\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_bool(), true)
      } else {
        panic!("sequence does not have 1 item")
      }
    }

    #[test]
    fn parse_and_int_0() {
      let s = eval(parse("0 and 1").expect("failed to parse expression \"0 and 1\""), &DynamicContext::new()).expect("failed to evaluate expression \"0 and 1\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_bool(), false)
      } else {
        panic!("sequence does not have 1 item")
      }
    }
    #[test]
    fn parse_and_int_1() {
      let s = eval(parse("1 and 1").expect("failed to parse expression \"1 and 1\""), &DynamicContext::new()).expect("failed to evaluate expression \"1 and 1\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_bool(), true)
      } else {
        panic!("sequence does not have 1 item")
      }
    }
    #[test]
    fn parse_and_multi_1() {
      let s = eval(parse("1 and 1.0 and 'abc'").expect("failed to parse expression \"1 and 1.0 and 'abc'\""), &DynamicContext::new()).expect("failed to evaluate expression \"1 and 1.0 and 'abc'\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_bool(), true)
      } else {
        panic!("sequence does not have 1 item")
      }
    }

    #[test]
    fn value_eq_int_true() {
      let s = eval(parse("1 eq 1").expect("failed to parse expression \"1 eq 1\""), &DynamicContext::new()).expect("failed to evaluate expression \"1 eq 1\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_bool(), true)
      } else {
        panic!("sequence does not have 1 item")
      }
    }
    #[test]
    fn value_eq_string_true() {
      let s = eval(parse("'abc' eq 'abc'").expect("failed to parse expression \"'abc' eq 'abc'\""), &DynamicContext::new()).expect("failed to evaluate expression \"'abc' eq 'abc'\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_bool(), true)
      } else {
        panic!("sequence does not have 1 item")
      }
    }
    #[test]
    fn value_eq_int_false() {
      let s = eval(parse("1 eq 0").expect("failed to parse expression \"1 eq 0\""), &DynamicContext::new()).expect("failed to evaluate expression \"1 eq 0\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_bool(), false)
      } else {
        panic!("sequence does not have 1 item")
      }
    }
    #[test]
    fn value_eq_string_false() {
      let s = eval(parse("'abc' eq 'def'").expect("failed to parse expression \"'abc' eq 'def'\""), &DynamicContext::new()).expect("failed to evaluate expression \"'abc' eq 'def'\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_bool(), false)
      } else {
        panic!("sequence does not have 1 item")
      }
    }
    #[test]
    fn general_eq_int_true() {
      let s = eval(parse("(1, 2) = (0, 1)").expect("failed to parse expression \"(1, 2) = (0, 1)\""), &DynamicContext::new()).expect("failed to evaluate expression \"(1, 2) = (0, 1)\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_bool(), true)
      } else {
        panic!("sequence does not have 1 item")
      }
    }
    #[test]
    fn general_eq_int_false() {
      let s = eval(parse("(1, 2) = (0, 3)").expect("failed to parse expression \"(1, 2) = (0, 3)\""), &DynamicContext::new()).expect("failed to evaluate expression \"(1, 2) = (0, 3)\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_bool(), false)
      } else {
        panic!("sequence does not have 1 item")
      }
    }

    #[test]
    fn parse_string_concat() {
      let s = eval(parse("1 || 'abc'").expect("failed to parse expression \"1 || 'abc'\""), &DynamicContext::new()).expect("failed to evaluate expression \"1 || 'abc'\"");
      if s.len() == 1 {
	assert_eq!(s[0].to_string(), "1abc")
      } else {
        panic!("sequence does not have 1 item")
      }
    }
    #[test]
    fn parse_string_concat_multi() {
      let s = eval(parse("1 || 'abc' || 2.3").expect("failed to parse expression \"1 || 'abc' || 2.3\""), &DynamicContext::new()).expect("failed to evaluate expression \"1 || 'abc' || 2.3\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_string(), "1abc2.3")
      } else {
        panic!("sequence does not have 1 item")
      }
    }

    #[test]
    fn parse_range() {
      let s = eval(parse("1 to 3").expect("failed to parse expression \"1 to 3\""), &DynamicContext::new()).expect("failed to evaluate expression \"1 to 3\"");
      if s.len() == 3 {
        assert_eq!(s[0].to_int().unwrap(), 1);
        assert_eq!(s[1].to_int().unwrap(), 2);
        assert_eq!(s[2].to_int().unwrap(), 3)
      } else {
        panic!(format!("sequence does not have 3 items, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_range_single() {
      let s = eval(parse("2 to 2").expect("failed to parse expression \"2 to 2\""), &DynamicContext::new()).expect("failed to evaluate expression \"2 to 2\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_int().unwrap(), 2)
      } else {
        panic!(format!("sequence does not have 1 items, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_range_empty() {
      let s = eval(parse("1 to ()").expect("failed to parse expression \"1 to ()\""), &DynamicContext::new()).expect("failed to evaluate expression \"1 to ()\"");
      assert_eq!(s.len(), 0)
    }
    #[test]
    fn parse_range_gt() {
      let s = eval(parse("10 to 1").expect("failed to parse expression \"10 to 1\""), &DynamicContext::new()).expect("failed to evaluate expression \"10 to 1\"");
      assert_eq!(s.len(), 0)
    }

    #[test]
    fn parse_addsub_plus_2() {
      let s = eval(parse("1 + 1").expect("failed to parse expression \"1 + 1\""), &DynamicContext::new()).expect("failed to evaluate expression \"1 + 1\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_double().unwrap(), 2.0)
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_addsub_plus_3() {
      let s = eval(parse("1 + 1 + 2").expect("failed to parse expression \"1 + 1 + 2\""), &DynamicContext::new()).expect("failed to evaluate expression \"1 + 1 + 2\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_double().unwrap(), 4.0)
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_addsub_minus_2() {
      let s = eval(parse("3 - 1").expect("failed to parse expression \"3 - 1\""), &DynamicContext::new()).expect("failed to evaluate expression \"3 - 1\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_double().unwrap(), 2.0)
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_addsub_minus_3() {
      let s = eval(parse("10 - 5 - 2").expect("failed to parse expression \"10 - 5 - 2\""), &DynamicContext::new()).expect("failed to evaluate expression \"10 - 5 - 2\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_double().unwrap(), 3.0)
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_addsub_mix() {
      let s = eval(parse("10 + 20 - 5 + 2").expect("failed to parse expression \"10 + 20 - 5 + 2\""), &DynamicContext::new()).expect("failed to evaluate expression \"10 + 20 - 5 + 2\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_double().unwrap(), 27.0)
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }

    #[test]
    fn parse_multiply_2() {
      let s = eval(parse("2 * 3").expect("failed to parse expression \"2 * 3\""), &DynamicContext::new()).expect("failed to evaluate expression \"2 * 3\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_double().unwrap(), 6.0)
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_multiply_3() {
      let s = eval(parse("2 * 3 * 4").expect("failed to parse expression \"2 * 3 * 4\""), &DynamicContext::new()).expect("failed to evaluate expression \"2 * 3 * 4\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_double().unwrap(), 24.0)
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_divide_2() {
      let s = eval(parse("3 div 2").expect("failed to parse expression \"3 div 2\""), &DynamicContext::new()).expect("failed to evaluate expression \"3 div 2\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_double().unwrap(), 1.5)
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_divide_3() {
      let s = eval(parse("100 div 10 div 2").expect("failed to parse expression \"100 div 10 div 2\""), &DynamicContext::new()).expect("failed to evaluate expression \"100 div 10 div 2\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_double().unwrap(), 5.0)
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_mod_2() {
      let s = eval(parse("3 mod 2").expect("failed to parse expression \"3 mod 2\""), &DynamicContext::new()).expect("failed to evaluate expression \"3 mod 2\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_double().unwrap(), 1.0)
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_muldiv_mix() {
      let s = eval(parse("6.5 * 2 div 0.5").expect("failed to parse expression \"6.5 * 2 div 0.5\""), &DynamicContext::new()).expect("failed to evaluate expression \"6.5 * 2 div 0.5\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_double().unwrap(), 26.0)
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
    #[test]
    fn parse_arithmetic_precedence() {
      let s = eval(parse("1 + 2 * 3 - 4").expect("failed to parse expression \"1 + 2 * 3 - 4\""), &DynamicContext::new()).expect("failed to evaluate expression \"1 + 2 * 3 - 4\"");
      if s.len() == 1 {
        assert_eq!(s[0].to_double().unwrap(), 3.0)
      } else {
        panic!(format!("sequence does not have 1 item, it has {} items", s.len()))
      }
    }
} 

