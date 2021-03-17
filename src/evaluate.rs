//! # xdm::evaluate
//!
//! Evaluate a sequence constructor.

use std::rc::Rc;
use crate::xdmerror::*;
use crate::item::*;
use decimal::d128;
//use crate::parsexml::parse;
use trees::{RcNode, Tree};

// The context for evaluating an XPath expression
#[derive(Clone)]
pub struct XPath<'a> {
  context: Option<Sequence<'a>>,		// The sequence currently being evaluated
  // The focus of the evaluation can be defined by the context sequence, above, plus the position of the context_item.
  // context_item = context[posn]
  // context_size = context.len()
  posn: Option<usize>,			// Context position
  constructor: Vec<Constructor<'a>>,
}

impl<'a> XPath<'a> {
  pub fn new() -> XPath<'a> {
    XPath{context: None, posn: None, constructor: vec![]}
  }
  pub fn clone(&self) -> XPath<'a> {
    XPath{context: self.context.clone(), posn: self.posn, constructor: vec![]}
  }
  pub fn set_context(&mut self, s: Sequence<'a>) {
    self.context = Some(s);
    self.posn = Some(0);
  }
  pub fn reset_context(&mut self) {
    self.context = None;
    self.posn = None;
  }
  pub fn set_position(mut self, p: usize) -> Self {
    self.posn = Some(p);
    self
  }
  pub fn context(&self) -> &Option<Sequence> {
    &self.context
  }
  pub fn position(&self) -> &Option<usize> {
    &self.posn
  }
  pub fn current_item(&self) -> Option<&Rc<Item>> {
    if self.context.is_some() {
      Some(&self.context.as_ref().unwrap()[self.posn.unwrap()])
    } else {
      None
    }
  }
  pub fn add_constructor(mut self, c: Constructor<'a>) -> Self {
    self.constructor.push(c);
    self
  }
  pub fn add_constructor_seq(mut self, s: Vec<Constructor<'a>>) -> Self {
    for d in s {
    	self.constructor.push(d);
    }
    self
  }
  pub fn set_constructor(mut self, c: Vec<Constructor<'a>>) -> Self {
    self.constructor.clear();
    for d in c {
    	self.constructor.push(d);
    }
    self
  }

  // This is where the action is!
  pub fn evaluate(&self) -> Result<Sequence, Error> {
    let mut ret: Sequence = Vec::new();

    // Each constructor will add an item to the result sequence
    // If a constructor returns a non-singleton sequence, then it is unpacked
    for c in &self.constructor {
      match c {
        Constructor::Literal(l) => {
	  ret.new_value(l.clone())
	}
        Constructor::ContextItem => {
	  if self.context.is_some() {
	    ret.add(&self.context.as_ref().unwrap()[self.posn.unwrap()])
	  } else {
	    return Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "no context item".to_string()})
	  }
	}
        Constructor::Or(v) => {
	  // Evaluate each operand to a boolean result. Return true if any of the operands' result is true
	  // Optimisation: stop upon the first true result.
	  // Future: Evaluate every operand to check for dynamic errors
	  let mut b = false;
	  for i in v {
	    let j = self.clone().set_constructor(i.clone());
	    let k = j.evaluate().expect("evaluating operand failed");
	    b = k.to_bool();
	    if b {break};
	  }
	  ret.new_value(Value::Boolean(b))
	}
        Constructor::And(v) => {
	  // Evaluate each operand to a boolean result. Return false if any of the operands' result is false
	  // Optimisation: stop upon the first false result.
	  // Future: Evaluate every operand to check for dynamic errors
	  let mut b = true;
	  for i in v {
	    let j = self.clone().set_constructor(i.clone());
	    let k = j.evaluate().expect("evaluating operand failed");
	    b = k.to_bool();
	    if !b {break};
	  }
	  ret.new_value(Value::Boolean(b))
	}
        Constructor::GeneralComparison(o, v) => {
	  if v.len() == 2 {
	    ret.new_value(Value::Boolean(general_comparison(self, *o, &v[0], &v[1]).expect("comparison evaluation failed")))
	  } else {
	    return Result::Err(Error{kind: ErrorKind::Unknown, message: "incorrect number of operands".to_string()})
	  }
	}
        Constructor::ValueComparison(o, v) => {
	  if v.len() == 2 {
	    ret.new_value(Value::Boolean(value_comparison(self, *o, &v[0], &v[1]).expect("comparison evaluation failed")))
	  } else {
	    return Result::Err(Error{kind: ErrorKind::Unknown, message: "incorrect number of operands".to_string()})
	  }
	}
        Constructor::Concat(v) => {
	  let mut r = String::new();
	  for u in v {
	    let s = self.clone().set_constructor(u.clone());
	    let t = s.evaluate().expect("evaluating operand failed");
	    r.push_str(t.to_string().as_str());
	  }
	  ret.new_value(Value::StringOwned(r));
	}
        Constructor::Range(v) => {
	  if v.len() == 2 {
            // Evaluate the two operands: they must both be literal integer items
	    let startc = self.clone().set_constructor(v[0].clone());
	    let start = startc.evaluate().expect("evaluating start operand failed");
	    let endc = self.clone().set_constructor(v[1].clone());
	    let end = endc.evaluate().expect("evaluating end operand failed");
	    if start.len() == 0 || end.len() == 0 {
	      // empty sequence is the result
	    } else if start.len() == 1 {
	      if end.len() == 1 {
	        let i = start[0].to_int().unwrap();
	    	let j = end[0].to_int().unwrap();
	    	if i > j {
		  // empty sequence result
	    	} else if i == j {
		  ret.new_value(Value::Integer(i))
	    	} else {
		  for k in i..=j {
		    ret.new_value(Value::Integer(k))
		  }
		}
	      } else {
	        return Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("end operand must be singleton")})
	      }
	    } else {
	      return Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("start operand must be singleton")})
	    }
	  } else {
	    return Result::Err(Error{kind: ErrorKind::Unknown, message: "incorrect number of operands".to_string()})
	  }
	}
        Constructor::Arithmetic(v) => {
	  // Type: the result will be a number, but integer or double?
	  // If all of the operands are integers, then the result is integer otherwise double
	  // TODO: check the type of all operands to determine type of result (can probably do this in static analysis phase)
	  // In the meantime, let's assume the result will be double and convert any integers

	  let mut acc: f64 = 0.0;

          for j in v {
	    let i = self.clone().set_constructor(j.operand.clone());
	    let k = i.evaluate().expect("evaluating operand failed");
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
	  ret.new_value(Value::Double(acc))
	}
	Constructor::Root => {
	  if self.context.is_some() {
	    match &*(self.context.as_ref().unwrap()[self.posn.unwrap()]) {
	      Item::Node(n) => {
	        ret.new_node(find_root(n.clone()));
	      }
	      _ => return Result::Err(Error{kind: ErrorKind::ContextNotNode, message: "context item is not a node".to_string()})
	    }
	  } else {
	    return Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "no context item".to_string()})
	  }
	}
	Constructor::Child(nm) => {
	  // TODO: interpret the node match. At the moment, this implements child::node()
	  if self.context.is_some() {
	    match &*(self.context.as_ref().unwrap()[self.posn.unwrap()]) {
	      Item::Node(n) => {
	        n.iter_rc().for_each(|c| ret.new_node(c.clone()));
	      }
	      _ => return Result::Err(Error{kind: ErrorKind::ContextNotNode, message: "context item is not a node".to_string()})
	    }
	  } else {
	    return Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "no context item".to_string()})
	  }
	}
	Constructor::Parent(nm) => {
	  // TODO: interpret the node match. At the moment, this implements parent::*
	  if self.context.is_some() {
	    match &*(self.context.as_ref().unwrap()[self.posn.unwrap()]) {
	      Item::Node(n) => {
	        match n.parent() {
		  Some(p) => {
		    ret.new_node(p.clone())
		  }
		  None => () // empty sequence is the result
		}
	      }
	      _ => return Result::Err(Error{kind: ErrorKind::ContextNotNode, message: "context item is not a node".to_string()})
	    }
	  } else {
	    return Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "no context item".to_string()})
	  }
	}
	Constructor::Step(_nm) => {
	  return Result::Err(Error{kind: ErrorKind::NotImplemented, message: "step sequence constructor not implemented".to_string()})
	}
	Constructor::NotImplemented => {
	  return Result::Err(Error{kind: ErrorKind::NotImplemented, message: "sequence constructor not implemented".to_string()})
	}
      }
    }

    Ok(ret)
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
  // DescendantOrSelf(NodeMatch),		// Descendants of the context item
  Step(NodeMatch),				// Next step of the path
  // RelativePath,			// Next step of the path
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

fn general_comparison<'a>(ctxt: &XPath, op: Operator, left: &Vec<Constructor<'a>>, right: &Vec<Constructor<'a>>) -> Result<bool, Error> {
  let mut b = false;
  let left_seq_a = ctxt.clone().set_constructor(left.clone());
  let left_seq = left_seq_a.evaluate().expect("evaluating left-hand sequence failed");
  //println!("left sequence ({} items) = \"{}\"", left_seq.len(), left_seq.to_string());
  let right_seq_a = ctxt.clone().set_constructor(right.clone());
  let right_seq = right_seq_a.evaluate().expect("evaluating right-hand sequence failed");
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
fn value_comparison<'a>(ctxt: &XPath, op: Operator, left: &Vec<Constructor<'a>>, right: &Vec<Constructor<'a>>) -> Result<bool, Error> {
  let left_seq_a = ctxt.clone().set_constructor(left.clone());
  let left_seq = left_seq_a.evaluate().expect("evaluating left-hand sequence failed");
  if left_seq.len() == 1 {
    let right_seq_a = ctxt.clone().set_constructor(right.clone());
    let right_seq = right_seq_a.evaluate().expect("evaluating right-hand sequence failed");
    if right_seq.len() == 1 {
      Ok(left_seq[0].compare(&*right_seq[0], op).unwrap())
    } else {
      Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("not a singleton sequence"),})
    }
  } else {
    Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("not a singleton sequence"),})
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_string() {
      let ctxt = XPath::new().add_constructor(Constructor::Literal(Value::String("foobar")));
      let s = ctxt.evaluate().expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s.to_string(), "foobar")
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn literal_int() {
      let ctxt = XPath::new().add_constructor(Constructor::Literal(Value::Integer(456)));
      let s = ctxt.evaluate().expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s[0].to_int().unwrap(), 456)
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn literal_decimal() {
      let ctxt = XPath::new().add_constructor(Constructor::Literal(Value::Decimal(d128!(34.56))));
      let s = ctxt.evaluate().expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s.to_string(), "34.56")
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn literal_bool() {
      let ctxt = XPath::new().add_constructor(Constructor::Literal(Value::Boolean(false)));
      let s = ctxt.evaluate().expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s.to_bool(), false)
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn literal_double() {
      let ctxt = XPath::new().add_constructor(Constructor::Literal(Value::Double(4.56)));
      let s = ctxt.evaluate().expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s[0].to_double(), 4.56)
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn sequence_literal() {
      let ctxt = XPath::new().add_constructor(Constructor::Literal(Value::String("foo"))).add_constructor(Constructor::Literal(Value::String("bar")));
      let s = ctxt.evaluate().expect("evaluation failed");
      if s.len() == 2 {
        assert_eq!(s.to_string(), "foobar")
      } else {
        panic!("sequence does not have two items")
      }
    }

    #[test]
    fn sequence_literal_mixed() {
      let ctxt = XPath::new().add_constructor(Constructor::Literal(Value::String("foo"))).add_constructor(Constructor::Literal(Value::Integer(123)));
      let s = ctxt.evaluate().expect("evaluation failed");
      if s.len() == 2 {
        assert_eq!(s.to_string(), "foo123")
      } else {
        panic!("sequence does not have two items")
      }
    }

    #[test]
    fn context_item() {
      let mut ctxt = XPath::new().add_constructor(Constructor::ContextItem);
      let s = vec![Rc::new(Item::Value(Value::String("foobar")))];
      ctxt.set_context(s);
      let result = ctxt.evaluate().expect("evaluation failed");
      if result.len() == 1 {
        assert_eq!(result[0].to_string(), "foobar")
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn context_item_2() {
      let mut ctxt = XPath::new().add_constructor(Constructor::ContextItem).add_constructor(Constructor::ContextItem);
      let s = vec![Rc::new(Item::Value(Value::String("foobar")))];
      ctxt.set_context(s);
      let result = ctxt.evaluate().expect("evaluation failed");
      if result.len() == 2 {
        assert_eq!(result.to_string(), "foobarfoobar")
      } else {
        panic!("sequence does not have two items")
      }
    }

    #[test]
    fn or() {
      let ctxt = XPath::new().add_constructor(Constructor::Or(vec![vec![Constructor::Literal(Value::Boolean(true))], vec![Constructor::Literal(Value::Boolean(false))]]));
      let s = ctxt.evaluate().expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s.to_bool(), true)
      } else {
        panic!("sequence is not a singleton")
      }
    }
    // TODO: test more than two operands

    #[test]
    fn and() {
      let ctxt = XPath::new().add_constructor(Constructor::And(vec![vec![Constructor::Literal(Value::Boolean(true))], vec![Constructor::Literal(Value::Boolean(false))]]));
      let s = ctxt.evaluate().expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s.to_bool(), false)
      } else {
        panic!("sequence is not a singleton")
      }
    }
    // TODO: test more than two operands

    #[test]
    fn value_comparison_int_true() {
      let ctxt = XPath::new().add_constructor(Constructor::ValueComparison(Operator::Equal, vec![vec![Constructor::Literal(Value::Integer(1))], vec![Constructor::Literal(Value::Integer(1))]]));
      let s = ctxt.evaluate().expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s.to_bool(), true)
      } else {
        panic!("sequence is not a singleton")
      }
    }
    // TODO: negative test: more than two operands
    #[test]
    fn value_comparison_int_false() {
      let ctxt = XPath::new().add_constructor(Constructor::ValueComparison(Operator::Equal, vec![vec![Constructor::Literal(Value::Integer(1))], vec![Constructor::Literal(Value::Integer(2))]]));
      let s = ctxt.evaluate().expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s.to_bool(), false)
      } else {
        panic!("sequence is not a singleton")
      }
    }
    // TODO: negative test: more than two operands
    #[test]
    fn value_comparison_string_true() {
      let ctxt = XPath::new().add_constructor(Constructor::ValueComparison(Operator::Equal, vec![vec![Constructor::Literal(Value::String("foo"))], vec![Constructor::Literal(Value::String("foo"))]]));
      let s = ctxt.evaluate().expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s.to_bool(), true)
      } else {
        panic!("sequence is not a singleton")
      }
    }
    // TODO: negative test: more than two operands
    #[test]
    fn value_comparison_string_false() {
      let ctxt = XPath::new().add_constructor(Constructor::ValueComparison(Operator::Equal, vec![vec![Constructor::Literal(Value::String("foo"))], vec![Constructor::Literal(Value::String("bar"))]]));
      let s = ctxt.evaluate().expect("evaluation failed");
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
      let ctxt = XPath::new().add_constructor(Constructor::GeneralComparison(
        Operator::Equal,
	vec![
	  vec![Constructor::Literal(Value::String("foo"))],
	  vec![Constructor::Literal(Value::String("bar")), Constructor::Literal(Value::String("foo"))]
	])
      );
      let s = ctxt.evaluate().expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s.to_bool(), true)
      } else {
        panic!("sequence is not a singleton")
      }
    }
    #[test]
    fn general_comparison_string_false() {
      let ctxt = XPath::new().add_constructor(Constructor::GeneralComparison(
        Operator::Equal,
	vec![
	  vec![Constructor::Literal(Value::String("foo"))],
	  vec![Constructor::Literal(Value::String("bar")), Constructor::Literal(Value::String("oof"))]
	])
      );
      let s = ctxt.evaluate().expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s.to_bool(), false)
      } else {
        panic!("sequence is not a singleton")
      }
    }
    // TODO: test multi-item first sequence against multi-item second sequence; mixed types, etc

    #[test]
    fn concat() {
      let ctxt = XPath::new().add_constructor(Constructor::Concat(
	vec![
	  vec![Constructor::Literal(Value::String("foo"))],
	  vec![Constructor::Literal(Value::String("bar")), Constructor::Literal(Value::String("oof"))]
	])
      );
      let s = ctxt.evaluate().expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s.to_string(), "foobaroof")
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn range() {
      let ctxt = XPath::new().add_constructor(Constructor::Range(
	vec![
	  vec![Constructor::Literal(Value::Integer(0))],
	  vec![Constructor::Literal(Value::Integer(9))]
	])
      );
      let s = ctxt.evaluate().expect("evaluation failed");
      if s.len() == 10 {
        assert_eq!(s.to_string(), "0123456789")
      } else {
        panic!("sequence does not have 10 items")
      }
    }
    // TODO: ranges resulting in empty sequence, start = end, negative tests

    #[test]
    fn arithmetic_double_add() {
      let ctxt = XPath::new().add_constructor(Constructor::Arithmetic(
	vec![
	  ArithmeticOperand{op: ArithmeticOperator::Noop, operand: vec![Constructor::Literal(Value::Double(1.0))]},
	  ArithmeticOperand{op: ArithmeticOperator::Add, operand: vec![Constructor::Literal(Value::Double(1.0))]}
	])
      );
      let s = ctxt.evaluate().expect("evaluation failed");
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
      let mut ctxt = XPath::new().add_constructor(Constructor::Root);
      let d = RcNode::from(Tree::new(NodeDefn::new(NodeType::Document)));
      let mut e = Tree::new(NodeDefn::new(NodeType::Element).set_name("Test".to_string()));
      let t = Tree::new(NodeDefn::new(NodeType::Text).set_value("Test text".to_string()));
      e.push_back(t);
      d.push_back(e);
      let s = vec![Rc::new(Item::Node(d.front().unwrap().front().unwrap().clone()))];
      ctxt.set_context(s);
      let e = ctxt.evaluate().expect("evaluation failed");
      if e.len() == 1 {
        assert_eq!(e[0].to_string(), "<Test>Test text</Test>")
      } else {
        panic!("sequence is not a singleton")
      }
    }
    #[test]
    fn node_child_all() {
      let mut ctxt = XPath::new().add_constructor(Constructor::Child(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})}));
      let d = RcNode::from(Tree::new(NodeDefn::new(NodeType::Document)));
      let mut e = Tree::new(NodeDefn::new(NodeType::Element).set_name("Test".to_string()));
      let t = Tree::new(NodeDefn::new(NodeType::Text).set_value("Test text".to_string()));
      e.push_back(t);
      d.push_back(e);
      let s = vec![Rc::new(Item::Node(d.front().unwrap().clone()))];
      ctxt.set_context(s);
      let e = ctxt.evaluate().expect("evaluation failed");
      if e.len() == 1 {
        assert_eq!(e[0].to_string(), "Test text")
      } else {
        panic!(format!("sequence is not a singleton: \"{}\"", e.to_string()))
      }
    }
    #[test]
    fn node_parent_any() {
      let mut ctxt = XPath::new().add_constructor(Constructor::Parent(NodeMatch{axis: Axis::Parent, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})}));
      let d = RcNode::from(Tree::new(NodeDefn::new(NodeType::Document)));
      let mut e = Tree::new(NodeDefn::new(NodeType::Element).set_name("Root".to_string()));
      let t = Tree::new(NodeDefn::new(NodeType::Element).set_name("Child".to_string()));
      e.push_back(t);
      d.push_back(e);
      let s = vec![Rc::new(Item::Node(d.front().unwrap().front().unwrap().clone()))];
      ctxt.set_context(s);
      let e = ctxt.evaluate().expect("evaluation failed");
      if e.len() == 1 {
        assert_eq!(e[0].to_string(), "<Root><Child/></Root>")
      } else {
        panic!(format!("sequence is not a singleton: \"{}\"", e.to_string()))
      }
    }

} 

