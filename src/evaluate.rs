//! # xdm::evaluate
//!
//! Evaluate a sequence constructor.

use std::rc::Rc;
use crate::xdmerror::*;
use crate::item::*;
use decimal::d128;
//use crate::parsexml::parse;
use trees::{RcNode, Tree};
use roxmltree::Node;

// Evaluate a sequence constructor, given a context
pub fn evaluate<'a>(ctxt: Option<Sequence<'a>>, posn: Option<usize>, c: &'a Vec<Constructor<'a>>) -> Result<Sequence<'a>, Error> {
  Ok(c.iter().map(|a| evaluate_one(ctxt.clone(), posn, a).expect("evaluation of item failed")).flatten().collect())
}

// Evaluate an item constructor, given a context
// If a constructor returns a non-singleton sequence, then it is unpacked
fn evaluate_one<'a>(ctxt: Option<Sequence<'a>>, posn: Option<usize>, c: &'a Constructor) -> Result<Sequence<'a>, Error> {
  match c {
    Constructor::Literal(l) => {
	let mut seq = Sequence::new();
	seq.new_value(l.clone());
	Ok(seq)
    }
    Constructor::ContextItem => {
      if ctxt.is_some() {
	let mut seq = Sequence::new();
	seq.add(&ctxt.as_ref().unwrap()[posn.unwrap()]);
	Ok(seq)
      } else {
	Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "no context item".to_string()})
      }
    }
    Constructor::Or(v) => {
      // Evaluate each operand to a boolean result. Return true if any of the operands' result is true
      // Optimisation: stop upon the first true result.
      // Future: Evaluate every operand to check for dynamic errors
      let mut b = false;
      for i in v {
	let k = evaluate(ctxt.clone(), posn, i).expect("evaluating operand failed");
	b = k.to_bool();
	if b {break};
      }
      let mut seq = Sequence::new();
      seq.new_value(Value::Boolean(b));
      Ok(seq)
    }
    Constructor::And(v) => {
      // Evaluate each operand to a boolean result. Return false if any of the operands' result is false
      // Optimisation: stop upon the first false result.
      // Future: Evaluate every operand to check for dynamic errors
      let mut b = true;
      for i in v {
	let k = evaluate(ctxt.clone(), posn, i).expect("evaluating operand failed");
	b = k.to_bool();
	if !b {break};
      }
      let mut seq = Sequence::new();
      seq.new_value(Value::Boolean(b));
      Ok(seq)
    }
    Constructor::GeneralComparison(o, v) => {
      if v.len() == 2 {
	let mut seq = Sequence::new();
	seq.new_value(Value::Boolean(
	  general_comparison(ctxt, posn, *o, &v[0], &v[1])
	    .expect("comparison evaluation failed")
	  ));
      	Ok(seq)
      } else {
	Result::Err(Error{kind: ErrorKind::Unknown, message: "incorrect number of operands".to_string()})
      }
    }
    Constructor::ValueComparison(o, v) => {
      if v.len() == 2 {
	let mut seq = Sequence::new();
	seq.new_value(Value::Boolean(
	  value_comparison(ctxt, posn, *o, &v[0], &v[1])
	    .expect("comparison evaluation failed")
	));
      	Ok(seq)
      } else {
	Result::Err(Error{kind: ErrorKind::Unknown, message: "incorrect number of operands".to_string()})
      }
    }
    Constructor::Concat(v) => {
      let mut r = String::new();
      for u in v {
	let t = evaluate(ctxt.clone(), posn, u).expect("evaluating operand failed");
	r.push_str(t.to_string().as_str());
      }
      let mut seq = Sequence::new();
      seq.new_value(Value::StringOwned(r));
      Ok(seq)
    }
    Constructor::Range(v) => {
      if v.len() == 2 {
        // Evaluate the two operands: they must both be literal integer items
	let start = evaluate(ctxt.clone(), posn, &v[0]).expect("evaluating start operand failed");
	let end = evaluate(ctxt.clone(), posn, &v[1]).expect("evaluating end operand failed");
	if start.len() == 0 || end.len() == 0 {
	  // empty sequence is the result
	  Ok(vec![])
	} else if start.len() == 1 {
	  if end.len() == 1 {
	    let i = start[0].to_int().unwrap();
	    let j = end[0].to_int().unwrap();
	    if i > j {
	      // empty sequence result
	      Ok(vec![])
	    } else if i == j {
	      let mut seq = Sequence::new();
	      seq.new_value(Value::Integer(i));
      	      Ok(seq)
	    } else {
	      let mut result = Sequence::new();
	      for k in i..=j {
	        result.new_value(Value::Integer(k))
	      }
	      Ok(result)
	    }
	  } else {
	    Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("end operand must be singleton")})
	  }
	} else {
	  Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("start operand must be singleton")})
	}
      } else {
	Result::Err(Error{kind: ErrorKind::Unknown, message: "incorrect number of operands".to_string()})
      }
    }
    Constructor::Arithmetic(v) => {
      // Type: the result will be a number, but integer or double?
      // If all of the operands are integers, then the result is integer otherwise double
      // TODO: check the type of all operands to determine type of result (can probably do this in static analysis phase)
      // In the meantime, let's assume the result will be double and convert any integers

      let mut acc: f64 = 0.0;

      for j in v {
	let k = evaluate(ctxt.clone(), posn, &j.operand).expect("evaluating operand failed");
	let u: f64;
	if k.len() != 1 {
	  return Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("type error (not a singleton sequence)")});
	} else {
	  u = k[0].to_double();
	  match j.op {
	    ArithmeticOperator::Noop => acc = u,
	    ArithmeticOperator::Add => acc += u,
	    ArithmeticOperator::Subtract => acc -= u,
	    ArithmeticOperator::Multiply => acc *= u,
	    ArithmeticOperator::Divide => acc /= u,
	    ArithmeticOperator::IntegerDivide => acc /= u, // TODO: convert to integer
	    ArithmeticOperator::Modulo => acc = acc % u,
	  }
	}
      }
      let mut seq = Sequence::new();
      seq.new_value(Value::Double(acc));
      Ok(seq)
    }
    Constructor::Root => {
      if ctxt.is_some() {
        match &*(ctxt.as_ref().unwrap()[posn.unwrap()]) {
	  Item::Node(n) => {
	    let mut seq = Sequence::new();
	    seq.new_node(find_root(n.clone()));
      	    Ok(seq)
	  }
	  Item::XNode(n) => {
	    let mut seq = Sequence::new();
	    seq.new_xnode(n.document().root());
	    Ok(seq)
	  }
	  _ => Result::Err(Error{kind: ErrorKind::ContextNotNode, message: "context item is not a node".to_string()})
	}
      } else {
	Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "no context item".to_string()})
      }
    }
    Constructor::Child(_nm) => {
      // TODO: interpret the node match. At the moment, this implements child::node()
      if ctxt.is_some() {
        match &*(ctxt.as_ref().unwrap()[posn.unwrap()]) {
	  Item::Node(n) => {
	    let mut result: Sequence = Vec::new();
	    n.iter_rc().for_each(|c| result.new_node(c.clone()));
	    Ok(result)
	  }
	  Item::XNode(n) => {
	    if n.has_children() {
	      Ok(n.children().fold(Sequence::new(), |mut c, a| {c.new_xnode(a); c}))
	    } else {
	      Ok(Sequence::new())
	    }
	  }
	  _ => Result::Err(Error{kind: ErrorKind::ContextNotNode, message: "context item is not a node".to_string()})
	}
      } else {
	Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "no context item".to_string()})
      }
    }
    Constructor::Parent(_nm) => {
      // TODO: interpret the node match. At the moment, this implements parent::*
      if ctxt.is_some() {
	match &*(ctxt.as_ref().unwrap()[posn.unwrap()]) {
	  Item::Node(n) => {
	    match n.parent() {
	      Some(p) => {
		let mut seq = Sequence::new();
		seq.new_node(p.clone());
      		Ok(seq)
	      }
	      None => {
	        // empty sequence is the result
		let seq = Sequence::new();
      		Ok(seq)
	      }
	    }
	  }
	  Item::XNode(n) => {
	    match n.parent() {
	      Some(p) => {
	        Ok(vec![Rc::new(Item::XNode(p))])
	      }
	      None => {
	        Ok(vec![])
	      }
	    }
	  }
	  _ => Result::Err(Error{kind: ErrorKind::ContextNotNode, message: "context item is not a node".to_string()})
	}
      } else {
	Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "no context item".to_string()})
      }
    }
    Constructor::DescendantOrSelf(_nm) => {
      // TODO: interpret the node match. At the moment, this implements child::node()
      if ctxt.is_some() {
	match &*(ctxt.as_ref().unwrap()[posn.unwrap()]) {
	  Item::Node(n) => {
	    let mut result: Sequence = Vec::new();
	    n.iter_rc().for_each(|c| result.new_node(c.clone()));
	    Ok(result)
	  }
	  Item::XNode(n) => {
	    let mut result = Sequence::new();
	    result.new_xnode(*n);
	    n.descendants().for_each(|d| result.new_xnode(d));
	    Ok(result)
	  }
	  _ => return Result::Err(Error{kind: ErrorKind::ContextNotNode, message: "context item is not a node".to_string()})
	}
      } else {
	Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "no context item".to_string()})
      }
    }
    Constructor::Path(s) => {
      // s is a vector of sequence constructors
      // Each step creates a new context for the next step
      // TODO: if initial context is None then error

      let u: Sequence; // accumulator - each time around the loop this will be the new context

      if ctxt.is_some() {
        u = ctxt.unwrap().clone()
      } else {
        u = vec![]
      }

      Ok(s.iter().fold(
	    u,
	    |a, c| {
	      evaluate(Some(a), Some(0), c).expect("failed to evaluate step")
	    }
      ))
    }
    Constructor::Step(nm) => {
      // TODO: interpret the node match. At the moment, this implements child::node()
      if ctxt.is_some() {
	match &*(ctxt.as_ref().unwrap()[posn.unwrap()]) {
	  Item::Node(n) => {
	    let mut result: Sequence = Vec::new();
	    n.iter_rc().for_each(|c| result.new_node(c.clone()));
	    Ok(result)
	  }
	  Item::XNode(n) => {
	    match nm.axis {
	      Axis::Child => {
	        if n.has_children() {
	      	  Ok(n.children()
		    .filter(|c| is_node_match(&nm.nodetest, c))
		    .fold(Sequence::new(), |mut c, a| {c.new_xnode(a); c})
		  )
	    	} else {
	      	  Ok(Sequence::new())
	    	}
	      }
	      Axis::Parent => {
	        // TODO
	      	  Ok(Sequence::new())
	      }
	      Axis::Descendant => {
	        // TODO
	      	  Ok(Sequence::new())
	      }
	      _ => {
	        // Not yet implemented
		Result::Err(Error{kind: ErrorKind::NotImplemented, message: "not yet implemented".to_string()})
	      }
	    }
	  }
	  _ => Result::Err(Error{kind: ErrorKind::ContextNotNode, message: "context item is not a node".to_string()})
	}
      } else {
	Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "no context item".to_string()})
      }
    }
    Constructor::NotImplemented => {
      Result::Err(Error{kind: ErrorKind::NotImplemented, message: "sequence constructor not implemented".to_string()})
    }
  }
}

fn find_root(n: RcNode<NodeDefn>) -> RcNode<NodeDefn> {
  if n.is_root() {
    n.clone()
  } else {
    find_root(n.parent().unwrap())
  }
}

// Defines how we can construct a sequence
#[derive(Clone)]
pub enum Constructor<'a> {
  Literal(Value<'a>),		// A literal, scalar value
  ContextItem,			// The context item from the dynamic context
  Or(Vec<Vec<Constructor<'a>>>),	// Logical OR. Each element of the outer vector is an operand.
  And(Vec<Vec<Constructor<'a>>>),	// Logical AND
  // Union,
  // IntersectExcept,
  // InstanceOf,
  // Treat,
  // Castable,
  // Cast,
  // Arrow,
  // Unary,
  // SimpleMap,
  Root,				// Root node of the context item
  Child(NodeMatch),			// Child nodes of the context item
  Parent(NodeMatch),			// Parent element of the context item
  DescendantOrSelf(NodeMatch),		// Descendants of the context item
  Path(Vec<Vec<Constructor<'a>>>),	// Step in the path
  Step(NodeMatch),	// Next step of the path
  GeneralComparison(Operator, Vec<Vec<Constructor<'a>>>),	// General comparison
  ValueComparison(Operator, Vec<Vec<Constructor<'a>>>),	// Value comparison
  // Is,
  // Before,
  // After,
  Concat(Vec<Vec<Constructor<'a>>>),	// Concatentate string values
  Range(Vec<Vec<Constructor<'a>>>),		// Range of integers
  Arithmetic(Vec<ArithmeticOperand<'a>>),	// Addition, subtraction, multiply, divide
  NotImplemented,	// TODO: implement everything so this can be removed
}


fn is_node_match(nt: &NodeTest, n: &Node) -> bool {
  //println!("is_node_match: node has tag name \"{}\"", n.tag_name().name());
  match nt {
    NodeTest::Name(t) => {
      // TODO: namespaces
      match &t.name {
        Some(a) => {
	  match a {
	    WildcardOrName::Wildcard => {
	      //println!("wildcard");
	      true
	    }
	    WildcardOrName::Name(s) => {
	      //println!("does {} == {} ? {}", s, n.tag_name().name(), s == n.tag_name().name());
	      s == n.tag_name().name()
	    }
	  }
	}
	None => {
	  //println!("no name test");
	  false
	}
      }
    }
    NodeTest::Kind => {
      // TODO
      //println!("node test kind");
      false
    }
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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub enum ArithmeticOperator {
  Noop,
  Add,
  Multiply,
  Divide,
  IntegerDivide,
  Subtract,
  Modulo,
}
impl ArithmeticOperator {
  pub fn from(a: &str) -> ArithmeticOperator {
    match a {
      "+" => ArithmeticOperator::Add,
      "*" => ArithmeticOperator::Multiply,
      "div" => ArithmeticOperator::Divide,
      "idiv" => ArithmeticOperator::IntegerDivide,
      "-" => ArithmeticOperator::Subtract,
      "mod" => ArithmeticOperator::Modulo,
      _ => ArithmeticOperator::Noop,
    }
  }
}

#[derive(Clone)]
pub struct ArithmeticOperand<'a> {
  pub op: ArithmeticOperator,
  pub operand: Vec<Constructor<'a>>,
}

fn general_comparison<'a>(ctxt: Option<Sequence>, posn: Option<usize>, op: Operator, left: &Vec<Constructor<'a>>, right: &Vec<Constructor<'a>>) -> Result<bool, Error> {
  let mut b = false;
  let left_seq = evaluate(ctxt.clone(), posn, left).expect("evaluating left-hand sequence failed");
  //println!("left sequence ({} items) = \"{}\"", left_seq.len(), left_seq.to_string());
  let right_seq = evaluate(ctxt.clone(), posn, right).expect("evaluating right-hand sequence failed");
  //println!("right sequence ({} items) = \"{}\"", right_seq.len(), right_seq.to_string());
  for l in left_seq {
    for r in &right_seq {
      //println!("compare \"{}\" to \"{}\"", l.to_string(), r.to_string());
      b = l.compare(&*r, op).unwrap();
      //println!("result = {}", b);
      if b { break }
    }
    if b { break }
  };
  //println!("final result = {}", b);
  Ok(b)
}

// Operands must be singletons
fn value_comparison<'a>(ctxt: Option<Sequence>, posn: Option<usize>, op: Operator, left: &Vec<Constructor<'a>>, right: &Vec<Constructor<'a>>) -> Result<bool, Error> {
  let left_seq = evaluate(ctxt.clone(), posn, left).expect("evaluating left-hand sequence failed");
  if left_seq.len() == 1 {
    let right_seq = evaluate(ctxt.clone(), posn, right).expect("evaluating right-hand sequence failed");
    if right_seq.len() == 1 {
      Ok(left_seq[0].compare(&*right_seq[0], op).unwrap())
    } else {
      Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("not a singleton sequence"),})
    }
  } else {
    Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("not a singleton sequence"),})
  }
}

pub fn format_constructor(c: &Vec<Constructor>, i: usize) -> String {
  let mut result = String::new();
  for v in c {
    result.push_str(", ");
    let t =
    match v {
      Constructor::Literal(l) => {
        format!("{:in$} Construct literal", "", in=i)
      }
      Constructor::ContextItem => {
        format!("{:in$} Construct context item", "", in=i)
      }
      Constructor::Or(v) => {
        format!(
	  "{:in$} Construct OR of:\n{}\n{}", "",
	  format_constructor(&v[0], 4),
	  format_constructor(&v[1], 4),
	  in=i,
	)
      }
      Constructor::And(v) => {
        format!(
	  "{:in$} Construct AND of:\n{}\n{}", "",
	  format_constructor(&v[0], i + 4),
	  format_constructor(&v[1], i + 4),
	  in=i,
	)
      }
      Constructor::Root => {
        format!("{:in$} Construct document root", "", in=i)
      }
      Constructor::Child(_nm) => {
        format!("{:in$} Construct child axis", "", in=i)
      }
      Constructor::Parent(_nm) => {
        format!("{:in$} Construct parent axis", "", in=i)
      }
      Constructor::DescendantOrSelf(_nm) => {
        format!("{:in$} Construct descendant-or-self axis", "", in=i)
      }
      Constructor::Step(_nm) => {
        format!("{:in$} Construct step axis", "", in=i)
      }
      Constructor::Path(v) => {
        let mut s = format!("{:in$} Construct relative path:\n", "", in=i);
	for u in v {
	  s.push_str(&format_constructor(u, i + 4))
	}
	s
      }
      Constructor::GeneralComparison(_o, _v) => {
        format!("{:in$} general comparison constructor", "", in=i)
      }
      Constructor::ValueComparison(_o, _v) => {
        format!("{:in$} value comparison constructor", "", in=i)
      }
      Constructor::Concat(_v) => {
        format!("{:in$} concat constructor", "", in=i)
      }
      Constructor::Range(_v) => {
        format!("{:in$} range constructor", "", in=i)
      }
      Constructor::Arithmetic(_v) => {
        format!("{:in$} arithmetic constructor", "", in=i)
      }
      Constructor::NotImplemented => {
        format!("{:in$} NotImplemented constructor", "", in=i)
      }
    };
    result.push_str(&t);
  }
  result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_string() {
      let cons = vec![Constructor::Literal(Value::String("foobar"))];
      let s = evaluate(None, None, &cons)
	.expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s.to_string(), "foobar")
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn literal_int() {
      let cons = vec![Constructor::Literal(Value::Integer(456))];
      let s = evaluate(None, None, &cons)
	.expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s[0].to_int().unwrap(), 456)
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn literal_decimal() {
      let cons = vec![Constructor::Literal(Value::Decimal(d128!(34.56)))];
      let s = evaluate(None, None, &cons)
        .expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s.to_string(), "34.56")
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn literal_bool() {
      let cons = vec![Constructor::Literal(Value::Boolean(false))];
      let s = evaluate(None, None, &cons)
        .expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s.to_bool(), false)
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn literal_double() {
      let cons = vec![Constructor::Literal(Value::Double(4.56))];
      let s = evaluate(None, None, &cons)
        .expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s[0].to_double(), 4.56)
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn sequence_literal() {
      let cons = vec![
	  Constructor::Literal(Value::String("foo")),
	  Constructor::Literal(Value::String("bar")),
	];
      let s = evaluate(None, None, &cons)
        .expect("evaluation failed");
      if s.len() == 2 {
        assert_eq!(s.to_string(), "foobar")
      } else {
        panic!("sequence does not have two items")
      }
    }

    #[test]
    fn sequence_literal_mixed() {
      let cons = vec![
	  Constructor::Literal(Value::String("foo")),
	  Constructor::Literal(Value::Integer(123)),
	];
      let s = evaluate(None, None, &cons)
        .expect("evaluation failed");
      if s.len() == 2 {
        assert_eq!(s.to_string(), "foo123")
      } else {
        panic!("sequence does not have two items")
      }
    }

    #[test]
    fn context_item() {
      let s = vec![Rc::new(Item::Value(Value::String("foobar")))];
      let cons = vec![Constructor::ContextItem];
      let result = evaluate(Some(s), Some(0), &cons)
        .expect("evaluation failed");
      if result.len() == 1 {
        assert_eq!(result[0].to_string(), "foobar")
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn context_item_2() {
      let cons = vec![
	  Constructor::ContextItem,
	  Constructor::ContextItem,
	];
      let result = evaluate(Some(vec![Rc::new(Item::Value(Value::String("foobar")))]), Some(0), &cons)
        .expect("evaluation failed");
      if result.len() == 2 {
        assert_eq!(result.to_string(), "foobarfoobar")
      } else {
        panic!("sequence does not have two items")
      }
    }

    #[test]
    fn or() {
      let cons = vec![
	  Constructor::Or(
	    vec![
	      vec![Constructor::Literal(Value::Boolean(true))],
	      vec![Constructor::Literal(Value::Boolean(false))],
	    ]
	  )
	];
      let s = evaluate(None, None, &cons)
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
      let cons = vec![
	  Constructor::And(
	    vec![
	      vec![Constructor::Literal(Value::Boolean(true))],
	      vec![Constructor::Literal(Value::Boolean(false))],
	    ]
	  )
	];
      let s = evaluate(None, None, &cons)
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
      let cons = vec![
	  Constructor::ValueComparison(
	    Operator::Equal,
	    vec![
	      vec![Constructor::Literal(Value::Integer(1))],
	      vec![Constructor::Literal(Value::Integer(1))],
	    ]
	  )
	];
      let s = evaluate(None, None, &cons)
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
      let cons = vec![
	  Constructor::ValueComparison(
	    Operator::Equal,
	    vec![
	      vec![Constructor::Literal(Value::Integer(1))],
	      vec![Constructor::Literal(Value::Integer(2))],
	    ]
	  )
	];
      let s = evaluate(None, None, &cons)
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
      let cons = vec![
	  Constructor::ValueComparison(
	    Operator::Equal,
	    vec![
	      vec![Constructor::Literal(Value::String("foo"))],
	      vec![Constructor::Literal(Value::String("foo"))],
	    ]
	  )
	];
      let s = evaluate(None, None, &cons)
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
      let cons = vec![
	  Constructor::ValueComparison(
	    Operator::Equal,
	    vec![
	      vec![Constructor::Literal(Value::String("foo"))],
	      vec![Constructor::Literal(Value::String("bar"))],
	    ]
	  )
	];
      let s = evaluate(None, None, &cons)
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
      let cons = vec![
	  Constructor::GeneralComparison(
            Operator::Equal,
	    vec![
	      vec![Constructor::Literal(Value::String("foo"))],
	      vec![
	        Constructor::Literal(Value::String("bar")),
	        Constructor::Literal(Value::String("foo")),
	      ]
	    ]
	  )
	];
      let s = evaluate(None, None, &cons)
	.expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s.to_bool(), true)
      } else {
        panic!("sequence is not a singleton")
      }
    }
    #[test]
    fn general_comparison_string_false() {
      let cons = vec![
	  Constructor::GeneralComparison(
            Operator::Equal,
	    vec![
	      vec![Constructor::Literal(Value::String("foo"))],
	      vec![
	        Constructor::Literal(Value::String("bar")),
	        Constructor::Literal(Value::String("oof")),
	      ]
	    ]
	  )
	];
      let s = evaluate(None, None, &cons)
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
      let cons = vec![
	  Constructor::Concat(
	    vec![
	      vec![Constructor::Literal(Value::String("foo"))],
	      vec![
	        Constructor::Literal(Value::String("bar")),
	        Constructor::Literal(Value::String("oof")),
	      ]
	    ]
	  )
	];
      let s = evaluate(None, None, &cons)
	.expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s.to_string(), "foobaroof")
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn range() {
      let cons = vec![
	  Constructor::Range(
	    vec![
	      vec![Constructor::Literal(Value::Integer(0))],
	      vec![Constructor::Literal(Value::Integer(9))],
	    ]
	  )
	];
      let s = evaluate(None, None, &cons)
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
      let cons = vec![
	  Constructor::Arithmetic(
	    vec![
	      ArithmeticOperand{
	        op: ArithmeticOperator::Noop,
	        operand: vec![Constructor::Literal(Value::Double(1.0))]
	      },
	      ArithmeticOperand{
	        op: ArithmeticOperator::Add,
	        operand: vec![Constructor::Literal(Value::Double(1.0))]
	      }
	    ]
	  )
	];
      let s = evaluate(None, None, &cons)
	.expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s[0].to_double(), 2.0)
      } else {
        panic!("sequence is not a singleton")
      }
    }
    // TODO: ranges resulting in empty sequence, start = end, negative tests

    // Nodes

    #[test]
    fn node_root() {
      let d = RcNode::from(Tree::new(NodeDefn::new(NodeType::Document)));
      let mut e = Tree::new(NodeDefn::new(NodeType::Element).set_name("Test".to_string()));
      let t = Tree::new(NodeDefn::new(NodeType::Text).set_value("Test text".to_string()));
      e.push_back(t);
      d.push_back(e);
      let cons = vec![Constructor::Root];
      let e = evaluate(Some(vec![Rc::new(Item::Node(d.front().unwrap().front().unwrap().clone()))]), Some(0), &cons).expect("evaluation failed");
      if e.len() == 1 {
        assert_eq!(e[0].to_string(), "<Test>Test text</Test>")
      } else {
        panic!("sequence is not a singleton")
      }
    }
    #[test]
    fn xnode_root() {
      let d = roxmltree::Document::parse("<Test>test text</Test>").expect("failed to parse XML");
      let cons = vec![Constructor::Root];
      let e = evaluate(Some(vec![Rc::new(Item::XNode(d.root()))]), Some(0), &cons).expect("evaluation failed");
      if e.len() == 1 {
        assert_eq!(e[0].to_string(), "<Test>test text</Test>")
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn node_child_all() {
      let d = RcNode::from(Tree::new(NodeDefn::new(NodeType::Document)));
      let mut e = Tree::new(NodeDefn::new(NodeType::Element).set_name("Test".to_string()));
      let t = Tree::new(NodeDefn::new(NodeType::Text).set_value("Test text".to_string()));
      e.push_back(t);
      d.push_back(e);
      let cons = vec![
	  Constructor::Child(
	    NodeMatch{
	      axis: Axis::Child,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    }
	  )
	];
      let e = evaluate(Some(vec![Rc::new(Item::Node(d.front().unwrap().clone()))]), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() == 1 {
        assert_eq!(e[0].to_string(), "Test text")
      } else {
        panic!("sequence is not a singleton: \"{}\"", e.to_string())
      }
    }
    #[test]
    fn xnode_child_all() {
      let d = roxmltree::Document::parse("<Test>test text</Test>").expect("failed to parse XML");
      let cons = vec![
	  Constructor::Child(
	    NodeMatch{
	      axis: Axis::Child,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    }
	  )
	];
      let e = evaluate(Some(vec![Rc::new(Item::XNode(d.root().first_child().unwrap()))]), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() == 1 {
        assert_eq!(e[0].to_string(), "test text")
      } else {
        panic!("sequence is not a singleton: \"{}\"", e.to_string())
      }
    }
    #[test]
    fn node_parent_any() {
      let d = RcNode::from(Tree::new(NodeDefn::new(NodeType::Document)));
      let mut e = Tree::new(NodeDefn::new(NodeType::Element).set_name("Root".to_string()));
      let t = Tree::new(NodeDefn::new(NodeType::Element).set_name("Child".to_string()));
      e.push_back(t);
      d.push_back(e);
      let s = vec![Rc::new(Item::Node(d.front().unwrap().front().unwrap().clone()))];

      let cons = vec![Constructor::Parent(
	  NodeMatch{
	    axis: Axis::Parent,
	    nodetest: NodeTest::Name(NameTest{
	      ns: None,
	      prefix: None,
	      name: Some(WildcardOrName::Wildcard)
	    })
	  }
	)];
      let e = evaluate(Some(s), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() == 1 {
        assert_eq!(e[0].to_string(), "<Root><Child/></Root>")
      } else {
        panic!("sequence is not a singleton: \"{}\"", e.to_string())
      }
    }
    #[test]
    fn xnode_parent_any() {
      let d = roxmltree::Document::parse("<Root><Child></Child></Root>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap()))];

      let cons = vec![Constructor::Parent(
	  NodeMatch{
	    axis: Axis::Parent,
	    nodetest: NodeTest::Name(NameTest{
	      ns: None,
	      prefix: None,
	      name: Some(WildcardOrName::Wildcard)
	    })
	  }
	)];
      let e = evaluate(Some(s), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() == 1 {
        assert_eq!(e[0].to_string(), "<Root><Child/></Root>")
      } else {
        panic!("sequence is not a singleton: \"{}\"", e.to_string())
      }
    }

    //#[test]
    fn node_path() {
      let d = RcNode::from(Tree::<NodeDefn>::from_tuple(
        (NodeDefn::new(NodeType::Document),
          (NodeDefn::new(NodeType::Element).set_name("Level1".to_string()),
	    (NodeDefn::new(NodeType::Element).set_name("Level2".to_string()),
	     NodeDefn::new(NodeType::Text).set_value("one".to_string())),
	    (NodeDefn::new(NodeType::Element).set_name("Level2".to_string()),
	     NodeDefn::new(NodeType::Text).set_value("two".to_string())),
	    (NodeDefn::new(NodeType::Element).set_name("Level2".to_string()),
	     NodeDefn::new(NodeType::Text).set_value("three".to_string()))
	  )
	)
      ));
      let s = vec![Rc::new(Item::Node(d.clone()))];
      let cons = vec![
	  Constructor::Root,
	  Constructor::Path(
	    vec![
              vec![Constructor::Child(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})})],
              vec![Constructor::Child(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})})],
            ]
	  )
	];
      let e = evaluate(Some(s), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() == 3 {
        assert_eq!(e[0].to_string(), "<Level2>one</Level2>");
        assert_eq!(e[1].to_string(), "<Level2>two</Level2>");
        assert_eq!(e[2].to_string(), "<Level2>three</Level2>");
      } else {
        panic!("sequence does not have 3 items: \"{}\"", e.to_string())
      }
    }
    #[test]
    fn xnode_path() {
      let d = roxmltree::Document::parse("<Level1><Level2>one</Level2><Level2>two</Level2><Level2>three</Level2></Level1>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap()))];
      let cons = vec![
	  Constructor::Path(
	    vec![
	      vec![Constructor::Root],
              vec![Constructor::Child(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})})],
              vec![Constructor::Child(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})})],
            ]
	  )
	];
      let e = evaluate(Some(s), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() == 3 {
        assert_eq!(e[0].to_string(), "<Level2>one</Level2>");
        assert_eq!(e[1].to_string(), "<Level2>two</Level2>");
        assert_eq!(e[2].to_string(), "<Level2>three</Level2>");
	//println!("constructor:\n{}\n", format_constructor(&cons, 0));
	//panic!("blah")
      } else {
        panic!("sequence does not have 3 items: \"{}\"", e.to_string())
      }
    }
    #[test]
    fn xnode_nametest_pos() {
      let d = roxmltree::Document::parse("<Test/>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap()))];
      let cons = vec![
	  Constructor::Path(
	    vec![
	      vec![Constructor::Root],
              vec![Constructor::Step(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Test".to_string()))})})],
            ]
	  )
	];
      let e = evaluate(Some(s), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() == 1 {
        assert_eq!(e[0].to_string(), "<Test/>");
	//println!("constructor:\n{}\n", format_constructor(&cons, 0));
	//panic!("blah")
      } else {
        panic!("sequence does not have 1 item: \"{}\"", e.to_string())
      }
    }
    #[test]
    fn xnode_nametest_neg() {
      let d = roxmltree::Document::parse("<Foobar/>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap()))];
      let cons = vec![
	  Constructor::Path(
	    vec![
	      vec![Constructor::Root],
              vec![Constructor::Step(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Test".to_string()))})})],
            ]
	  )
	];
      let e = evaluate(Some(s), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() == 0 {
        assert!(true)
      } else {
        println!("result: {}", e.to_string());
      	assert_eq!(e.len(), 0);
      }
    }
} 

