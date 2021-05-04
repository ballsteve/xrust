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
use std::collections::HashMap;
use std::cell::{RefCell, RefMut};
use json::{JsonValue, object};

pub struct DynamicContext<'a> {
  vars: RefCell<HashMap<String, Vec<Sequence<'a>>>>,
}

impl<'a> DynamicContext<'a> {
  pub fn new() -> DynamicContext<'a> {
    DynamicContext{vars: RefCell::new(HashMap::new())}
  }
}

// Evaluate a sequence constructor, given a context
pub fn evaluate<'a>(
    dc: &DynamicContext<'a>,
    ctxt: Option<Sequence<'a>>,
    posn: Option<usize>,
    c: &'a Vec<Constructor<'a>>
  ) -> Result<Sequence<'a>, Error> {

  Ok(c.iter().map(|a| evaluate_one(dc, ctxt.clone(), posn, a).expect("evaluation of item failed")).flatten().collect())
}

// Evaluate an item constructor, given a context
// If a constructor returns a non-singleton sequence, then it is unpacked
fn evaluate_one<'a>(
    dc: &DynamicContext<'a>,
    ctxt: Option<Sequence<'a>>,
    posn: Option<usize>,
    c: &'a Constructor<'a>
  ) -> Result<Sequence<'a>, Error> {

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
	let k = evaluate(dc, ctxt.clone(), posn, i).expect("evaluating operand failed");
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
	let k = evaluate(dc, ctxt.clone(), posn, i).expect("evaluating operand failed");
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
	  general_comparison(dc, ctxt, posn, *o, &v[0], &v[1])
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
	  value_comparison(dc, ctxt, posn, *o, &v[0], &v[1])
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
	let t = evaluate(dc, ctxt.clone(), posn, u).expect("evaluating operand failed");
	r.push_str(t.to_string().as_str());
      }
      let mut seq = Sequence::new();
      seq.new_value(Value::StringOwned(r));
      Ok(seq)
    }
    Constructor::Range(v) => {
      if v.len() == 2 {
        // Evaluate the two operands: they must both be literal integer items
	let start = evaluate(dc, ctxt.clone(), posn, &v[0]).expect("evaluating start operand failed");
	let end = evaluate(dc, ctxt.clone(), posn, &v[1]).expect("evaluating end operand failed");
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
	let k = evaluate(dc, ctxt.clone(), posn, &j.operand).expect("evaluating operand failed");
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
	  Item::JsonValue(j) => Result::Err(Error{kind: ErrorKind::NotImplemented, message: "json unable to get containing document".to_string()}),
	  _ => Result::Err(Error{kind: ErrorKind::ContextNotNode, message: "context item is not a node".to_string()})
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

      //println!("path start: u=\"{}\" {} items", u.to_string(), u.len());

      Ok(s.iter().fold(
	    u,
	    |a, c| {
	      //println!("path iteration: a=\"{}\" {} items", a.to_string(), a.len());
	      // evaluate this step for each item in the context
	      // Add the result of each evaluation to an accummulator sequence
	      let mut b: Sequence = Vec::new();
	      for i in 0..a.len() {
	        let mut d = evaluate(dc, Some(a.clone()), Some(i), c).expect("failed to evaluate step");
		b.append(&mut d);
	      }
	      b
	    }
      ))
    }
    Constructor::Step(nm, p) => {
      if ctxt.is_some() {
        //println!("Constructor::Step: ctxt \"{}\" size = {} posn = {}", ctxt.as_ref().unwrap().to_string(), ctxt.as_ref().unwrap().len(), posn.unwrap());
	match &*(ctxt.as_ref().unwrap()[posn.unwrap()]) {
	  Item::Node(n) => {
	    match nm.axis {
	      Axis::Child => {
	        let mut result: Sequence = Vec::new();
	    	n.iter_rc().for_each(|c| result.new_node(c.clone()));
	    	Ok(result)
	      }
	      Axis::Parent => {
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
	      _ => {
	        // Not yet implemented
		Result::Err(Error{kind: ErrorKind::NotImplemented, message: "not yet implemented".to_string()})
	      }
	    }
	  }
	  Item::XNode(n) => {
	    match nm.axis {
	      Axis::Child => {
	        if n.has_children() {
		  let seq = n.children()
		      .filter(|c| is_node_match(&nm.nodetest, c))
		      .fold(Sequence::new(), |mut c, a| {c.new_xnode(a); c});
		  //println!("path: child axis: seq=\"{}\"", seq.to_string());
	      	  Ok(predicates(dc, seq, p))
	    	} else {
	      	  Ok(Sequence::new())
	    	}
	      }
	      Axis::Parent => {
	        match n.parent() {
	      	  Some(p) => {
	            Ok(vec![Rc::new(Item::XNode(p))])
	      	  }
	      	  None => {
	            Ok(vec![])
	      	  }
	    	}
	      }
	      Axis::Descendant => {
	        // The descendant axis does not include itself,
		// but the descendant function does
	        let seq = n.descendants()
		  .skip(1)
		  .filter(|c| is_node_match(&nm.nodetest, c))
		  .fold(Sequence::new(), |mut c, a| {c.new_xnode(a); c});
	      	Ok(predicates(dc, seq, p))
	      }
	      Axis::DescendantOrSelf => {
	        // In this case the descendant function gives us what we want
	        let seq = n.descendants()
		  .filter(|c| is_node_match(&nm.nodetest, c))
		  .fold(Sequence::new(), |mut c, a| {c.new_xnode(a); c});
	      	Ok(predicates(dc, seq, p))
	      }
	      Axis::Ancestor => {
	        // The ancestor axis does not include itself,
		// but the ancestors function does
	        let seq = n.ancestors()
		  .skip(1)
		  .filter(|c| is_node_match(&nm.nodetest, c))
		  .fold(Sequence::new(), |mut c, a| {c.new_xnode(a); c});
	      	Ok(predicates(dc, seq, p))
	      }
	      Axis::AncestorOrSelf => {
	        // In this case the ancestors function gives us what we want
	        let seq = n.ancestors()
		  .filter(|c| is_node_match(&nm.nodetest, c))
		  .fold(Sequence::new(), |mut c, a| {c.new_xnode(a); c});
	      	Ok(predicates(dc, seq, p))
	      }
	      Axis::FollowingSibling => {
	        // The following-sibling axis does not include itself,
		// but the next_siblings function does
	        let seq = n.next_siblings()
		  .skip(1)
		  .filter(|c| is_node_match(&nm.nodetest, c))
		  .fold(Sequence::new(), |mut c, a| {c.new_xnode(a); c});
	      	Ok(predicates(dc, seq, p))
	      }
	      Axis::PrecedingSibling => {
	        // The preceding-sibling axis does not include itself,
		// but the prev_siblings function does
	        let seq = n.prev_siblings()
		  .skip(1)
		  .filter(|c| is_node_match(&nm.nodetest, c))
		  .fold(Sequence::new(), |mut c, a| {c.new_xnode(a); c});
	      	Ok(predicates(dc, seq, p))
	      }
	      Axis::Following => {
	        // XPath 3.3.2.1: the following axis contains all nodes that are descendants of the root of the tree in which the context node is found, are not descendants of the context node, and occur after the context node in document order.
		// iow, for each ancestor node, include every next sibling and its descendants
		let anc: Vec<Node> = n.ancestors()
		  .skip(1)
		  .collect();
		let mut d: Vec<Node> = Vec::new();
		for a in anc {
		  let sibs: Vec<Node> = a.next_siblings()
		      .skip(1)
		      .collect();
		  for b in sibs {
		    let mut sib_descs: Vec<Node> = b.descendants().collect();
		    d.append(&mut sib_descs)
		  }
		}
	        let seq = d.iter()
		  .filter(|e| is_node_match(&nm.nodetest, e))
		  .fold(Sequence::new(), |mut f, g| {f.new_xnode(*g); f});
	      	Ok(predicates(dc, seq, p))
	      }
	      Axis::Preceding => {
	        // XPath 3.3.2.1: the preceding axis contains all nodes that are descendants of the root of the tree in which the context node is found, are not ancestors of the context node, and occur before the context node in document order.
		// iow, for each ancestor-or-self node, include every previous sibling and its descendants
		let anc: Vec<Node> = n.ancestors()
		  .collect();
		let mut d: Vec<Node> = Vec::new();
		for a in anc {
		  let sibs: Vec<Node> = a.prev_siblings()
		      .skip(1)
		      .collect();
		  for b in sibs {
		    let mut sib_descs: Vec<Node> = b.descendants().collect();
		    d.append(&mut sib_descs)
		  }
		}
	        let seq = d.iter()
		  .filter(|e| is_node_match(&nm.nodetest, e))
		  .fold(Sequence::new(), |mut f, g| {f.new_xnode(*g); f});
	      	Ok(predicates(dc, seq, p))
	      }
	      _ => {
	        // Not yet implemented
		Result::Err(Error{kind: ErrorKind::NotImplemented, message: "not yet implemented".to_string()})
	      }
	    }
	  }
	  Item::JsonValue(j) => {
	    match nm.axis {
	      Axis::Child => {
	        //println!("jsonvalue child axis, j is {}", jsonvalue_kind(j));
		let mut seq: Sequence = Vec::new();
		match j {
		  JsonValue::Object(_) => {
	            seq = j.entries()
		      .filter(|(key, val)| is_jsonvalue_match(&nm.nodetest, key))
		      .fold(Sequence::new(), |mut c, (key, val)| {
		        //println!("adding json value \"{}\" is a {}", key, jsonvalue_kind(val));
			c.new_jvalue(val.clone());
			c
		      });
		  }
		  JsonValue::Array(_) => {
	            seq = j.members()
		      .fold(Sequence::new(), |mut c, val| {
		        //println!("adding json array member is a {}", jsonvalue_kind(val));
			c.new_jvalue(val.clone());
			c
		      });
		  }
		  _ => {}
		}
		//println!("path: child axis: seq=\"{}\" ({} items)", seq.to_string(), seq.len());
	      	Ok(predicates(dc, seq, p))
	      }
	      Axis::Parent => {
		Result::Err(Error{kind: ErrorKind::NotImplemented, message: "json does not implement parent axis".to_string()})
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
    Constructor::FunctionCall(f, a) => {
      match f.body {
        Some(g) => {
	  //println!("Constructor::FunctionCall - function \"{}\" is defined", f.name);
      	  // Evaluate the arguments
      	  let mut b = Vec::new();
      	  for c in a {
            b.push(evaluate(dc, ctxt.clone(), posn, c).expect("argument evaluation failed"))
      	  }
      	  // Invoke the function
      	  g(ctxt, posn, b)
	}
	None => {
	  //println!("Constructor::FunctionCall - function is NOT defined");
	  Result::Err(Error{kind: ErrorKind::NotImplemented, message: "call to undefined function".to_string()})
	}
      }
    }
    Constructor::VariableDeclaration(v, a) => {
      let s = evaluate(dc, ctxt, posn, a).expect("failed to evaluate variable value");
      let mut t: Vec<Sequence>;
      match dc.vars.borrow().get(v) {
        Some(u) => {
	  t = u.to_vec();
	  t.push(s)
	}
	None => {
	  t = vec![s]
	}
      }
      dc.vars.borrow_mut().insert(v.to_string(), t);
      Ok(Sequence::new())
    }
    Constructor::VariableReference(v) => {
      match dc.vars.borrow().get(v) {
        Some(s) => {
	  match s.last() {
	    Some(t) => Ok(t.clone()),
	    None => Result::Err(Error{kind: ErrorKind::Unknown, message: "no value for variable".to_string()})
	  }
	}
	None => {
      	  Result::Err(Error{kind: ErrorKind::Unknown, message: format!("reference to undefined variable \"{}\"", v)})
	}
      }
    }
    Constructor::Loop(v, b) => {
      // TODO: this supports only one variable binding - need to support more than one binding
      // Evaluate the variable value
      // Iterate over the items in the sequence
      // Set the variable value to the item
      // Evaluate the body, collecting the results

      if v.is_empty() {
      	Result::Err(Error{kind: ErrorKind::Unknown, message: "no variable bindings".to_string()})
      } else {
        let mut result: Sequence = vec![];
        match &v[0] {
          Constructor::VariableDeclaration(v, a) => {

	    let s: Sequence = evaluate(dc, ctxt.clone(), posn, &a)
	      .expect("failed to evaluate variable binding");

	    for i in s {
	      // Push the new value for this variable
	      var_push(dc, v, &i);
	      //println!("dc has value \"{}\" for \"x\"", dc.vars.borrow().get("x").unwrap().iter().map(|z| z.to_string()).collect::<String>());
	      let mut x = evaluate(dc, ctxt.clone(), posn, b).expect("failed to evaluate loop body");
	      result.append(&mut x);
	      // Pop the value for this variable
	      var_pop(dc, v);
	    }
	  }
	  _ => {
	    // Error: no variable bindings
	  }
      	}
	Ok(result)
      }
    }
    Constructor::Switch(v, o) => {
      // 'v' are pairs of test,body
      // 'o' is the otherwise clause
      // evaluate tests to a boolean until the first true result; evaluate it's body as the result
      // of all tests fail then evaluate otherwise clause

      Ok(
        v.chunks(2).fold(
          evaluate(dc, ctxt.clone(), posn, o).expect("failed to evaluate otherwise clause"),
	  |acc, t| {
	    if evaluate(dc, ctxt.clone(), posn, &t[0]).expect("failed to evaluate clause test").to_bool() {
	      evaluate(dc, ctxt.clone(), posn, &t[1]).expect("failed to evaluate clause body")
	    } else {
	      acc
	    }
	  }
        )
      )
    }
    Constructor::NotImplemented => {
      Result::Err(Error{kind: ErrorKind::NotImplemented, message: "sequence constructor not implemented".to_string()})
    }
  }
}

fn jsonvalue_kind(j: &JsonValue) -> &str {
  match j {
    JsonValue::Null => "null",
    JsonValue::Short(_) => "short",
    JsonValue::String(_) => "string",
    JsonValue::Number(_) => "number",
    JsonValue::Boolean(_) => "boolean",
    JsonValue::Object(_) => "object",
    JsonValue::Array(_) => "array",
  }
}

// Push a new scope for a variable
fn var_push<'a>(dc: &DynamicContext<'a>, v: &str, i: &Rc<Item<'a>>) {
  let mut h: RefMut<HashMap<String, Vec<Sequence>>>;
  let mut t: Option<&mut Vec<Sequence>>;

  h = dc.vars.borrow_mut();
  t = h.get_mut(v);
  match t.as_mut() {
    Some(u) => {
      // If the variable already has a value, then this is a new, inner scope
      u.push(vec![i.clone()]);
    }
    None => {
      // Otherwise this is the first scope for the variable
      h.insert(v.to_string(), vec![vec![i.clone()]]);
    }
  }
}
// Pop scope for a variable
// Prerequisite: scope must have already been pushed
fn var_pop(dc: &DynamicContext, v: &str) {
  let mut h: RefMut<HashMap<String, Vec<Sequence>>>;
  let t: Option<&mut Vec<Sequence>>;

  h = dc.vars.borrow_mut();
  t = h.get_mut(v);
  t.map(|u| u.pop());
}

// Filter the sequence with each of the predicates
fn predicates<'a>(dc: &DynamicContext<'a>, s: Sequence<'a>, p: &'a Vec<Vec<Constructor<'a>>>) -> Sequence<'a> {
  if p.is_empty() {
    //println!("predicates: no predicates so return complete sequence");
    s
  } else {
    let mut result = s.clone();

    // iterate over the predicates
    for q in p {
      //println!("evaluating predicate for {} items", result.len());
      let mut new: Sequence = Vec::new();

      // for each predicate, evaluate each item in s to a boolean
      for i in 0..result.len() {
        //println!("predicate: evaluating ctxt item {} of {} to bool", i, result.len());
        let b = evaluate(dc, Some(result.clone()), Some(i), q).expect("evaluating predicate failed");
	if b.to_bool() == true {
	  //println!("item {} is a keeper", i);
	  new.push(result[i].clone());
	}
      }
      //println!("replacing result with new of {} items", new.len());
      result.clear();
      result.append(&mut new);
    }

    //println!("predicates returning sequence with {} items", result.len());
    result
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
  //Child(NodeMatch),			// Child nodes of the context item
  //Parent(NodeMatch),			// Parent element of the context item
  //DescendantOrSelf(NodeMatch),		// Descendants of the context item
  Path(Vec<Vec<Constructor<'a>>>),	// Step in the path
  Step(NodeMatch, Vec<Vec<Constructor<'a>>>),	// Next step of the path, with predicates
  GeneralComparison(Operator, Vec<Vec<Constructor<'a>>>),	// General comparison
  ValueComparison(Operator, Vec<Vec<Constructor<'a>>>),	// Value comparison
  // Is,
  // Before,
  // After,
  Concat(Vec<Vec<Constructor<'a>>>),	// Concatentate string values
  Range(Vec<Vec<Constructor<'a>>>),		// Range of integers
  Arithmetic(Vec<ArithmeticOperand<'a>>),	// Addition, subtraction, multiply, divide
  FunctionCall(Function<'a>, Vec<Vec<Constructor<'a>>>),
  VariableDeclaration(String, Vec<Constructor<'a>>),	// TODO: support QName
  VariableReference(String),				// TODO: support QName
  Loop(Vec<Constructor<'a>>, Vec<Constructor<'a>>),	// first arg is variables, second arg is body
  Switch(Vec<Vec<Constructor<'a>>>, Vec<Constructor<'a>>),	// first arg is pairs of test,body. second arg is otherwise clause
  NotImplemented,	// TODO: implement everything so this can be removed
}

fn is_node_match(nt: &NodeTest, n: &Node) -> bool {
  //println!("is_node_match: matching \"{}\" - node has tag name \"{}\"", nt.to_string(), n.tag_name().name());

  match nt {
    NodeTest::Name(t) => {
      match n.node_type() {
        roxmltree::NodeType::Element => {
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
      	_ => false
      }
    }
    NodeTest::Kind(k) => {
      match k {
        KindTest::DocumentTest => {
          if n.node_type() == roxmltree::NodeType::Root {
	    true
	  } else {
	    false
	  }
        }
        KindTest::ElementTest => {
          if n.node_type() == roxmltree::NodeType::Element {
	    true
	  } else {
	    false
	  }
        }
        KindTest::PITest => {
          if n.node_type() == roxmltree::NodeType::PI {
	    true
	  } else {
	    false
	  }
        }
        KindTest::CommentTest => {
      	  if n.node_type() == roxmltree::NodeType::Comment {
	    true
	  } else {
	    false
	  }
        }
        KindTest::TextTest => {
      	  if n.node_type() == roxmltree::NodeType::Text {
	    true
	  } else {
	    false
	  }
        }
        KindTest::AnyKindTest => true,
        KindTest::AttributeTest |
	KindTest::SchemaElementTest |
        KindTest::SchemaAttributeTest |
        KindTest::NamespaceNodeTest => false, // TODO: not yet implemented
      }
    }
  }
}
fn is_jsonvalue_match(nt: &NodeTest, n: &str) -> bool {
  //println!("is_jsonvalue_match: matching \"{}\" - node has tag name \"{}\"", nt.to_string(), n);
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
	      s == n
	    }
	  }
	}
	None => {
	  //println!("no name test");
	  false
	}
      }
    }
    NodeTest::Kind(k) => {
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

impl NodeMatch {
  fn to_string(&self) -> String {
    format!("NodeMatch {}::{}", self.axis.to_string(), self.nodetest.to_string())
  }
}

#[derive(Clone)]
pub enum NodeTest {
  Kind(KindTest),
  Name(NameTest),
}

impl NodeTest {
  pub fn to_string(&self) -> String {
      match self {
        NodeTest::Name(nt) => {
	  nt.to_string()
	}
	_ => "--no test--".to_string()
      }
  }
}

#[derive(Clone)]
pub enum KindTest {
  DocumentTest,
  ElementTest,
  AttributeTest,
  SchemaElementTest,
  SchemaAttributeTest,
  PITest,
  CommentTest,
  TextTest,
  NamespaceNodeTest,
  AnyKindTest,
}

#[derive(Clone)]
pub struct NameTest {
  pub ns: Option<WildcardOrName>,
  pub prefix: Option<String>,
  pub name: Option<WildcardOrName>,
}

impl NameTest {
  pub fn to_string(&self) -> String {
    if self.name.is_some() {
      match self.name.as_ref().unwrap() {
        WildcardOrName::Wildcard => {
	  "*".to_string()
	}
	WildcardOrName::Name(n) => {
      	  n.to_string()
	}
      }
    } else {
      "--no name--".to_string()
    }
  }
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
  FollowingSibling,
  Namespace,
  Parent,
  Ancestor,
  AncestorOrSelf,
  Preceding,
  PrecedingSibling,
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
      "following-sibling" => Axis::FollowingSibling,
      "namespace" => Axis::Namespace,
      "parent" => Axis::Parent,
      "ancestor" => Axis::Ancestor,
      "ancestor-or-self" => Axis::AncestorOrSelf,
      "preceding" => Axis::Preceding,
      "preceding-sibling" => Axis::PrecedingSibling,
      _ => Axis::Unknown,
    }
  }
  pub fn to_string(&self) -> String {
    match self {
      Axis::Child => "child".to_string(),
      Axis::Descendant => "descendant".to_string(),
      Axis::DescendantOrSelf => "descendant-or-self".to_string(),
      Axis::Attribute => "attribute".to_string(),
      Axis::Selfaxis => "self".to_string(),
      Axis::Following => "following".to_string(),
      Axis::FollowingSibling => "following-sibling".to_string(),
      Axis::Namespace => "namespace".to_string(),
      Axis::Parent => "parent".to_string(),
      Axis::Ancestor => "ancestor".to_string(),
      Axis::AncestorOrSelf => "ancestor-or-self".to_string(),
      Axis::Preceding => "preceding".to_string(),
      Axis::PrecedingSibling => "preceding-sibling".to_string(),
      _ => "unknown".to_string(),
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

fn general_comparison<'a>(dc: &DynamicContext<'a>, ctxt: Option<Sequence<'a>>, posn: Option<usize>, op: Operator, left: &'a Vec<Constructor<'a>>, right: &'a Vec<Constructor<'a>>) -> Result<bool, Error> {
  let mut b = false;
  let left_seq = evaluate(dc, ctxt.clone(), posn, left).expect("evaluating left-hand sequence failed");
  //println!("left sequence ({} items) = \"{}\"", left_seq.len(), left_seq.to_string());
  let right_seq = evaluate(dc, ctxt.clone(), posn, right).expect("evaluating right-hand sequence failed");
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

// Static context and analysis

pub struct StaticContext<'a> {
  pub funcs: RefCell<HashMap<String, Function<'a>>>,
  pub vars: RefCell<HashMap<String, Vec<Sequence<'a>>>>, // each entry in the vector is an inner scope of the variable
}

impl<'a> StaticContext<'a> {
  pub fn new() -> StaticContext<'a> {
    StaticContext{funcs: RefCell::new(HashMap::new()), vars: RefCell::new(HashMap::new())}
  }
  pub fn new_with_builtins() -> StaticContext<'a> {
    let sc = StaticContext{funcs: RefCell::new(HashMap::new()), vars: RefCell::new(HashMap::new())};
    sc.funcs.borrow_mut().insert("position".to_string(),
      Function{
        name: "position".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_position)
      }
    );
    sc.funcs.borrow_mut().insert("last".to_string(),
      Function{
        name: "last".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_last)
      }
    );
    sc.funcs.borrow_mut().insert("count".to_string(),
      Function{
        name: "count".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_count)
      }
    );
    sc.funcs.borrow_mut().insert("local-name".to_string(),
      Function{
        name: "local-name".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_localname)
      }
    );
    sc.funcs.borrow_mut().insert("name".to_string(),
      Function{
        name: "name".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_name)
      }
    );
    sc
  }
  pub fn declare_function(&self, n: String, _ns: String, p: Vec<Param>) {
    self.funcs.borrow_mut().insert(n.clone(), Function{name: n, nsuri: None, prefix: None, body: None, params: p});
  }
  pub fn declare_variable(&self, n: String, _ns:String) {
    self.vars.borrow_mut().insert(n.clone(), vec![]);
  }
}

// Rewrite constructors
pub fn static_analysis<'a>(e: &mut Vec<Constructor<'a>>, sc: &'a StaticContext<'a>) {
  for d in e {
    match d {
      Constructor::Switch(v, o) => {
        for i in v {
	  static_analysis(i, sc)
	}
	static_analysis(o, sc);
      }
      Constructor::Loop(v, a) => {
	static_analysis(v, sc);
	static_analysis(a, sc);
      }
      Constructor::FunctionCall(f, a) => {
        // Fill in function body
	match sc.funcs.borrow().get(&f.name) {
	  Some(g) => {
	    f.body.replace(g.body.unwrap());
	  }
	  None => {
	    panic!("call to unknown function \"{}\"", f.name)
	  }
	}
        for i in a {
	  static_analysis(i, sc)
	}
      }
      Constructor::VariableDeclaration(v, a) => {
        sc.declare_variable(v.to_string(), "".to_string());
	static_analysis(a, sc)
      }
      Constructor::VariableReference(_v) => {
        // TODO: check that variable has been declared
      }
      Constructor::Or(a) |
      Constructor::And(a) |
      Constructor::Path(a) |
      Constructor::Concat(a) |
      Constructor::Range(a) => {
        for i in a {
	  static_analysis(i, sc)
	}
      }
      Constructor::Step(_, a) => {
        for i in a {
	  static_analysis(i, sc)
	}
      }
      Constructor::GeneralComparison(_, a) |
      Constructor::ValueComparison(_, a) => {
        for i in a {
	  static_analysis(i, sc)
	}
      }
      Constructor::Arithmetic(a) => {
        for i in a {
	  static_analysis(&mut i.operand, sc)
	}
      }
      Constructor::Literal(_) |
      Constructor::ContextItem |
      Constructor::Root |
      Constructor::NotImplemented => {}
    }
  }
}

// Functions

pub type FunctionImpl<'a> = fn(
    Option<Sequence<'a>>,		// Context
    Option<usize>,		// Context position
    Vec<Sequence<'a>>,		// Actual parameters
  ) -> Result<Sequence<'a>, Error>;

#[derive(Clone)]
pub struct Function<'a> {
  name: String,
  nsuri: Option<String>,
  prefix: Option<String>,
  params: Vec<Param>,	// The number of parameters in the vector is the arity of the function
  body: Option<FunctionImpl<'a>>,	// Function implementation must be provided during static analysis
}

impl Function<'_> {
  pub fn new(n: String, p: Vec<Param>, i: Option<FunctionImpl>) -> Function {
    Function{name: n, nsuri: None, prefix: None, params: p, body: i}
  }
}

// A formal parameter
#[derive(Clone)]
pub struct Param {
  name: String,
  datatype: String,	// TODO
}

impl Param {
  fn new(n: String, t: String) -> Param {
    Param{name: n, datatype: t}
  }
}

fn func_position<'a>(_ctxt: Option<Sequence<'a>>, posn: Option<usize>, _args: Vec<Sequence<'a>>) -> Result<Sequence<'a>, Error> {
  match posn {
    Some(u) => {
      //println!("func_position returning {}", u + 1);
      Ok(vec![Rc::new(Item::Value(Value::Integer(u as i64 + 1)))])
    }
    None => Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: String::from("no context item"),})
  }
}

fn func_last<'a>(ctxt: Option<Sequence<'a>>, _posn: Option<usize>, _args: Vec<Sequence<'a>>) -> Result<Sequence<'a>, Error> {
  match ctxt {
    Some(u) => {
      //println!("func_last returning {}", u.len());
      Ok(vec![Rc::new(Item::Value(Value::Integer(u.len() as i64)))])
    }
    None => Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: String::from("no context item"),})
  }
}

fn func_count<'a>(ctxt: Option<Sequence<'a>>, _posn: Option<usize>, args: Vec<Sequence<'a>>) -> Result<Sequence<'a>, Error> {
  match args.len() {
    0 => {
      // count the context items
      match ctxt {
        Some(u) => Ok(vec![Rc::new(Item::Value(Value::Integer(u.len() as i64)))]),
        None => Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: String::from("no context item"),})
      }
    }
    1 => {
      // count the argument items
      Ok(vec![Rc::new(Item::Value(Value::Integer(args[0].len() as i64)))])
    }
    _ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("wrong number of arguments"),})
  }
}

fn func_localname<'a>(ctxt: Option<Sequence<'a>>, posn: Option<usize>, _args: Vec<Sequence<'a>>) -> Result<Sequence<'a>, Error> {
  match ctxt {
    Some(u) => {
      // Current item must be a node
      match *u[posn.unwrap()] {
        Item::XNode(n) => {
      	  Ok(vec![Rc::new(Item::Value(Value::String(n.tag_name().name())))])
	}
	Item::Node(_) |
	Item::JsonValue(_) => Result::Err(Error{kind: ErrorKind::NotImplemented, message: String::from("not yet implemented"),}),
	_ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("not a node"),})
      }
    }
    None => Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: String::from("no context item"),})
  }
}

// TODO: handle qualified names
fn func_name<'a>(ctxt: Option<Sequence<'a>>, posn: Option<usize>, _args: Vec<Sequence<'a>>) -> Result<Sequence<'a>, Error> {
  match ctxt {
    Some(u) => {
      // Current item must be a node
      match *u[posn.unwrap()] {
        Item::XNode(n) => {
      	  Ok(vec![Rc::new(Item::Value(Value::String(n.tag_name().name())))])
	}
	Item::Node(_) |
	Item::JsonValue(_) => Result::Err(Error{kind: ErrorKind::NotImplemented, message: String::from("not yet implemented"),}),
	_ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("not a node"),})
      }
    }
    None => Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: String::from("no context item"),})
  }
}

// Operands must be singletons
fn value_comparison<'a>(dc: &DynamicContext<'a>, ctxt: Option<Sequence<'a>>, posn: Option<usize>, op: Operator, left: &'a Vec<Constructor<'a>>, right: &'a Vec<Constructor<'a>>) -> Result<bool, Error> {
  let left_seq = evaluate(dc, ctxt.clone(), posn, left).expect("evaluating left-hand sequence failed");
  if left_seq.len() == 1 {
    let right_seq = evaluate(dc, ctxt.clone(), posn, right).expect("evaluating right-hand sequence failed");
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
        format!("{:in$} Construct literal \"{}\"", "", l.to_string(), in=i)
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
      Constructor::Step(nm, p) => {
        format!(
	  "{:in$} Construct step {}{}", "",
	  nm.to_string(),
	  if p.len() != 0 {format!("\npredicates: {}", format_constructor(&p[0], 0))} else {"".to_string()},
	  in=i
	)
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
      Constructor::ValueComparison(o, v) => {
        format!("{:in$} value comparison constructor {} of:\n{}\n{}", "",
	o.to_string(),
	format_constructor(&v[0], i + 4),
	format_constructor(&v[1], i + 4),
	in=i)
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
      Constructor::FunctionCall(f, a) => {
        format!("{:in$} function call to \"{}\" ({}) with {} arguments", "",
	  f.name,
	  f.body.map_or_else(|| "not defined", |_| "is defined"),
	  a.len(),
	  in=i)
      }
      Constructor::VariableDeclaration(v, _) => {
        format!("{:in$} variable declaration constructor named \"{}\"", "", v, in=i)
      }
      Constructor::VariableReference(v) => {
        format!("{:in$} variable reference constructor named \"{}\"", "", v, in=i)
      }
      Constructor::Loop(_, _) => {
        format!("{:in$} loop constructor", "", in=i)
      }
      Constructor::Switch(_, _) => {
        format!("{:in$} switch constructor", "", in=i)
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
      let s = evaluate(&DynamicContext::new(), None, None, &cons)
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
      let s = evaluate(&DynamicContext::new(), None, None, &cons)
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
      let s = evaluate(&DynamicContext::new(), None, None, &cons)
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
      let s = evaluate(&DynamicContext::new(), None, None, &cons)
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
      let s = evaluate(&DynamicContext::new(), None, None, &cons)
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
      let s = evaluate(&DynamicContext::new(), None, None, &cons)
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
      let s = evaluate(&DynamicContext::new(), None, None, &cons)
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
      let result = evaluate(&DynamicContext::new(), Some(s), Some(0), &cons)
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
      let result = evaluate(&DynamicContext::new(), Some(vec![Rc::new(Item::Value(Value::String("foobar")))]), Some(0), &cons)
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
      let s = evaluate(&DynamicContext::new(), None, None, &cons)
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
      let s = evaluate(&DynamicContext::new(), None, None, &cons)
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
      let s = evaluate(&DynamicContext::new(), None, None, &cons)
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
      let s = evaluate(&DynamicContext::new(), None, None, &cons)
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
      let s = evaluate(&DynamicContext::new(), None, None, &cons)
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
      let s = evaluate(&DynamicContext::new(), None, None, &cons)
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
      let s = evaluate(&DynamicContext::new(), None, None, &cons)
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
      let s = evaluate(&DynamicContext::new(), None, None, &cons)
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
      let s = evaluate(&DynamicContext::new(), None, None, &cons)
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
      let s = evaluate(&DynamicContext::new(), None, None, &cons)
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
      let s = evaluate(&DynamicContext::new(), None, None, &cons)
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
      let e = evaluate(&DynamicContext::new(), Some(vec![Rc::new(Item::Node(d.front().unwrap().front().unwrap().clone()))]), Some(0), &cons).expect("evaluation failed");
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
      let e = evaluate(&DynamicContext::new(), Some(vec![Rc::new(Item::XNode(d.root()))]), Some(0), &cons).expect("evaluation failed");
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
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Child,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(vec![Rc::new(Item::Node(d.front().unwrap().clone()))]), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() == 1 {
        assert_eq!(e[0].to_string(), "Test text")
      } else {
        panic!("sequence is not a singleton: \"{}\"", e.to_string())
      }
    }
    #[test]
    fn xnode_child_all() {
      let d = roxmltree::Document::parse("<Test><text/></Test>").expect("failed to parse XML");
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Child,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(vec![Rc::new(Item::XNode(d.root().first_child().unwrap()))]), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() == 1 {
        assert_eq!(e[0].to_string(), "<text/>")
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

      let cons = vec![Constructor::Step(
	  NodeMatch{
	    axis: Axis::Parent,
	    nodetest: NodeTest::Name(NameTest{
	      ns: None,
	      prefix: None,
	      name: Some(WildcardOrName::Wildcard)
	    })
	  },
	  vec![]
	)];
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &cons)
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

      let cons = vec![Constructor::Step(
	  NodeMatch{
	    axis: Axis::Parent,
	    nodetest: NodeTest::Name(NameTest{
	      ns: None,
	      prefix: None,
	      name: Some(WildcardOrName::Wildcard)
	    })
	  },
	  vec![]
	)];
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() == 1 {
        assert_eq!(e[0].to_string(), "<Root><Child/></Root>")
      } else {
        panic!("sequence is not a singleton: \"{}\"", e.to_string())
      }
    }
    #[test]
    fn xnode_descendant_1() {
      let d = roxmltree::Document::parse("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Descendant,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap()))]), Some(0), &cons)
        .expect("evaluation failed");
      assert_eq!(e.len(), 6);
      //for t in 0..e.len() {
        //println!("Item {}: \"{}\"", t + 1, e[t].to_string())
      //}
      assert_eq!(e[1].to_string(), "<level3>1 1 1</level3>")
    }
    #[test]
    fn xnode_descendantorself_1() {
      let d = roxmltree::Document::parse("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::DescendantOrSelf,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap()))]), Some(0), &cons)
        .expect("evaluation failed");
      assert_eq!(e.len(), 7);
      //for t in 0..e.len() {
        //println!("Item {}: \"{}\"", t + 1, e[t].to_string())
      //}
      assert_eq!(e[2].to_string(), "<level3>1 1 1</level3>")
    }
    #[test]
    fn xnode_ancestor_1() {
      let d = roxmltree::Document::parse("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Ancestor,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap().first_child().unwrap().first_child().unwrap()))]), Some(0), &cons)
        .expect("evaluation failed");
      //for i in 0..e.len() {
        //println!("item {} is a {}", i, e[i].to_name())
      //}
      assert_eq!(e.len(), 3);
    }
    #[test]
    fn xnode_ancestororself_1() {
      let d = roxmltree::Document::parse("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::AncestorOrSelf,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap().first_child().unwrap().first_child().unwrap()))]), Some(0), &cons)
        .expect("evaluation failed");
      //for i in 0..e.len() {
        //println!("item {} is a {}", i, e[i].to_name())
      //}
      assert_eq!(e.len(), 4);
    }
    #[test]
    fn xnode_followingsibling_1() {
      let d = roxmltree::Document::parse("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::FollowingSibling,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap().first_child().unwrap().first_child().unwrap()))]), Some(0), &cons)
        .expect("evaluation failed");
      //for i in 0..e.len() {
        //println!("item {} is a {}", i, e[i].to_name())
      //}
      assert_eq!(e.len(), 1);
      assert_eq!(e.to_string(), "<level3>1 1 2</level3>");
    }
    #[test]
    fn xnode_precedingsibling_1() {
      let d = roxmltree::Document::parse("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::PrecedingSibling,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap().first_child().unwrap().last_child().unwrap()))]), Some(0), &cons)
        .expect("evaluation failed");
      //for i in 0..e.len() {
        //println!("item {} is a {}", i, e[i].to_name())
      //}
      assert_eq!(e.len(), 1);
      assert_eq!(e.to_string(), "<level3>1 1 1</level3>");
    }
    #[test]
    fn xnode_following_1() {
      let d = roxmltree::Document::parse("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Following,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap().first_child().unwrap().last_child().unwrap()))]), Some(0), &cons)
        .expect("evaluation failed");
      //for i in 0..e.len() {
        //println!("item {} is a {} = \"{}\"", i, e[i].to_name(), e[i].to_string())
      //}
      assert_eq!(e.len(), 4);
      assert_eq!(e.to_string(), "<level2><level3>1 2 1</level3><level3>1 2 2</level3></level2><level3>1 2 1</level3><level3>1 2 2</level3><level1>not me</level1>");
    }
    #[test]
    fn xnode_preceding_1() {
      let d = roxmltree::Document::parse("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Preceding,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(vec![Rc::new(Item::XNode(d.root().first_child().unwrap().last_child().unwrap()))]), Some(0), &cons)
        .expect("evaluation failed");
      //for i in 0..e.len() {
        //println!("item {} is a {} = \"{}\"", i, e[i].to_name(), e[i].to_string())
      //}
      assert_eq!(e.len(), 7);
      assert_eq!(e[0].to_name(), "level1");
      assert_eq!(e[1].to_name(), "level2");
      assert_eq!(e[2].to_name(), "level3");
      assert_eq!(e[2].to_string(), "<level3>1 1 1</level3>");
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
              vec![Constructor::Step(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})}, vec![])],
              vec![Constructor::Step(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})}, vec![])],
            ]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &cons)
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
              vec![Constructor::Step(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})}, vec![])],
              vec![Constructor::Step(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})}, vec![])],
            ]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &cons)
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
              vec![Constructor::Step(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Test".to_string()))})}, vec![])],
            ]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &cons)
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
              vec![Constructor::Step(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Test".to_string()))})}, vec![])],
            ]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() == 0 {
        assert!(true)
      } else {
        //println!("result: {}", e.to_string());
      	assert_eq!(e.len(), 0);
      }
    }

    // Kind Tests
    #[test]
    fn xnode_kind_element_1() {
      let d = roxmltree::Document::parse("<Test><level1>1<level2/>2<level2/>3<level2/>4<level2/>5<level2/>6<level2/>7</level1></Test>").expect("failed to parse XML");
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Child,
	      nodetest: NodeTest::Kind(KindTest::ElementTest)
	    },
	    vec![]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap()))]), Some(0), &cons)
        .expect("evaluation failed");
      assert_eq!(e.len(), 6);
      assert_eq!(e[0].to_name(), "level2");
      assert_eq!(e[1].to_name(), "level2");
      assert_eq!(e[2].to_name(), "level2");
      assert_eq!(e[3].to_name(), "level2");
      assert_eq!(e[4].to_name(), "level2");
      assert_eq!(e[5].to_name(), "level2");
    }
    #[test]
    fn xnode_kind_text_1() {
      let d = roxmltree::Document::parse("<Test><level1>1<level2/>2<level2/>3<level2/>4<level2/>5<level2/>6<level2/>7</level1></Test>").expect("failed to parse XML");
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Child,
	      nodetest: NodeTest::Kind(KindTest::TextTest)
	    },
	    vec![]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap()))]), Some(0), &cons)
        .expect("evaluation failed");
      assert_eq!(e.len(), 7);
      assert_eq!(e[0].to_string(), "1");
      assert_eq!(e[1].to_string(), "2");
      assert_eq!(e[2].to_string(), "3");
      assert_eq!(e[3].to_string(), "4");
      assert_eq!(e[4].to_string(), "5");
      assert_eq!(e[5].to_string(), "6");
      assert_eq!(e[6].to_string(), "7");
    }
    #[test]
    fn xnode_kind_any_1() {
      let d = roxmltree::Document::parse("<Test><level1>1<level2/>2<level2/>3<level2/>4<level2/>5<level2/>6<level2/>7</level1></Test>").expect("failed to parse XML");
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Child,
	      nodetest: NodeTest::Kind(KindTest::AnyKindTest)
	    },
	    vec![]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap()))]), Some(0), &cons)
        .expect("evaluation failed");
      assert_eq!(e.len(), 13);
      assert_eq!(e[0].to_string(), "1");
      assert_eq!(e[1].to_name(), "level2");
      assert_eq!(e[2].to_string(), "2");
      assert_eq!(e[3].to_name(), "level2");
      assert_eq!(e[4].to_string(), "3");
      assert_eq!(e[5].to_name(), "level2");
      assert_eq!(e[6].to_string(), "4");
      assert_eq!(e[7].to_name(), "level2");
      assert_eq!(e[8].to_string(), "5");
      assert_eq!(e[9].to_name(), "level2");
      assert_eq!(e[10].to_string(), "6");
      assert_eq!(e[11].to_name(), "level2");
      assert_eq!(e[12].to_string(), "7");
    }

    #[test]
    fn xnode_predicate_pos() {
      let d = roxmltree::Document::parse("<Test><Level2></Level2></Test>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap()))];
      // This constructor is "/Test[Level2]"
      let cons = vec![
	  Constructor::Path(
	    vec![
	      vec![Constructor::Root],
              vec![Constructor::Step(
	        NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Test".to_string()))})},
		vec![vec![Constructor::Step(
	          NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Level2".to_string()))})},
		  vec![]
		)]]
	      )],
            ]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() == 1 {
        assert_eq!(e[0].to_string(), "<Test><Level2/></Test>");
	//println!("constructor:\n{}\n", format_constructor(&cons, 0));
	//panic!("blah")
      } else {
        panic!("sequence does not have 1 item: \"{}\"", e.to_string())
      }
    }
    #[test]
    fn xnode_predicate_neg() {
      let d = roxmltree::Document::parse("<Test><Level2></Level2></Test>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap()))];
      // This constructor is "/Test[foo]"
      let cons = vec![
	  Constructor::Path(
	    vec![
	      vec![Constructor::Root],
              vec![Constructor::Step(
	        NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Test".to_string()))})},
		vec![vec![Constructor::Step(
	          NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("foo".to_string()))})},
		  vec![]
		)]]
	      )],
            ]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &cons)
        .expect("evaluation failed");
      assert_eq!(e.len(), 0);
    }

    #[test]
    fn function_call_position() {
      let c = Constructor::FunctionCall(
        Function::new("position".to_string(), vec![], Some(func_position)),
	vec![]
      );
      let s = vec![
        Rc::new(Item::Value(Value::String("a"))),
        Rc::new(Item::Value(Value::String("b"))),
      ];
      let vc = vec![c];
      let r = evaluate(&DynamicContext::new(), Some(s), Some(1), &vc).expect("evaluation failed");
      assert_eq!(r.to_string(), "2")
    }
    #[test]
    fn function_call_last() {
      let c = Constructor::FunctionCall(
        Function::new("last".to_string(), vec![], Some(func_last)),
	vec![]
      );
      let s = vec![
        Rc::new(Item::Value(Value::String("a"))),
        Rc::new(Item::Value(Value::String("b"))),
        Rc::new(Item::Value(Value::String("c"))),
      ];
      let vc = vec![c];
      let r = evaluate(&DynamicContext::new(), Some(s), Some(1), &vc).expect("evaluation failed");
      assert_eq!(r.to_string(), "3")
    }
    #[test]
    fn function_call_count() {
      let c = Constructor::FunctionCall(
        Function::new("count".to_string(), vec![Param::new("i".to_string(), "t".to_string())], Some(func_count)),
	vec![
	  vec![
            Constructor::Literal(Value::String("a")),
            Constructor::Literal(Value::String("b")),
            Constructor::Literal(Value::String("c")),
	  ]
        ]
      );
      let vc = vec![c];
      let r = evaluate(&DynamicContext::new(), None, None, &vc).expect("evaluation failed");
      assert_eq!(r.to_string(), "3")
    }
    #[test]
    fn function_call_local_name() {
      let d = roxmltree::Document::parse("<Test><Level2></Level2></Test>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap()))];
      let c = Constructor::FunctionCall(
        Function::new("local-name".to_string(), vec![], Some(func_localname)),
	vec![]
      );
      let vc = vec![c];
      let r = evaluate(&DynamicContext::new(), Some(s), Some(0), &vc).expect("evaluation failed");
      assert_eq!(r.to_string(), "Test")
    }
    #[test]
    fn function_call_name() {
      let d = roxmltree::Document::parse("<Test><Level2></Level2></Test>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap()))];
      let c = Constructor::FunctionCall(
        Function::new("name".to_string(), vec![], Some(func_name)),
	vec![]
      );
      let vc = vec![c];
      let r = evaluate(&DynamicContext::new(), Some(s), Some(0), &vc).expect("evaluation failed");
      assert_eq!(r.to_string(), "Test")
    }

    // Variables
    #[test]
    fn var_ref() {
      let c = vec![
        Constructor::VariableDeclaration("foo".to_string(), vec![Constructor::Literal(Value::String("my variable"))]),
	Constructor::VariableReference("foo".to_string()),
      ];
      let r = evaluate(&DynamicContext::new(), None, None, &c).expect("evaluation failed");
      assert_eq!(r.to_string(), "my variable")
    }

    // Loops
    #[test]
    fn loop_1() {
      // This is "for $x in ('a', 'b', 'c') return $x"
      let c = vec![
        Constructor::Loop(
	  vec![Constructor::VariableDeclaration(
	    "x".to_string(),
	    vec![
	      Constructor::Literal(Value::String("a")),
	      Constructor::Literal(Value::String("b")),
	      Constructor::Literal(Value::String("c")),
	    ]
	  )],
	  vec![Constructor::VariableReference("x".to_string())]
	)
      ];
      let r = evaluate(&DynamicContext::new(), None, None, &c).expect("evaluation failed");
      assert_eq!(r.len(), 3);
      assert_eq!(r.to_string(), "abc")
    }

    // Switch
    #[test]
    fn switch_1() {
      // implements "if (1) then 'one' else 'not one'"
      let c = vec![
        Constructor::Switch(
	  vec![
	    vec![
	      Constructor::Literal(Value::Integer(1))
	    ],
	    vec![
	      Constructor::Literal(Value::String("one"))
	    ]
	  ],
	  vec![Constructor::Literal(Value::String("not one"))]
	)
      ];
      let r = evaluate(&DynamicContext::new(), None, None, &c).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      assert_eq!(r.to_string(), "one")
    }
    #[test]
    fn switch_2() {
      // implements "if (0) then 'one' else 'not one'"
      let c = vec![
        Constructor::Switch(
	  vec![
	    vec![
	      Constructor::Literal(Value::Integer(0))
	    ],
	    vec![
	      Constructor::Literal(Value::String("one"))
	    ]
	  ],
	  vec![Constructor::Literal(Value::String("not one"))]
	)
      ];
      let r = evaluate(&DynamicContext::new(), None, None, &c).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      assert_eq!(r.to_string(), "not one")
    }    
    #[test]
    fn switch_3() {
      let c = vec![
        Constructor::Switch(
	  vec![
	    vec![
	      Constructor::Literal(Value::Integer(0))
	    ],
	    vec![
	      Constructor::Literal(Value::String("one"))
	    ],
	    vec![
	      Constructor::Literal(Value::Integer(1))
	    ],
	    vec![
	      Constructor::Literal(Value::String("two"))
	    ],
	    vec![
	      Constructor::Literal(Value::Integer(0))
	    ],
	    vec![
	      Constructor::Literal(Value::String("three"))
	    ],
	  ],
	  vec![Constructor::Literal(Value::String("not any"))]
	)
      ];
      let r = evaluate(&DynamicContext::new(), None, None, &c).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      assert_eq!(r.to_string(), "two")
    }    

    // JSON
    #[test]
    fn json_1() {
      let json = object!{
        anint: 200,
	abool: true,
	alist: {
	  three: [
	    "three one",
	    "three two",
	    "three three"
	  ]
	}
      };
      let s = vec![Rc::new(Item::JsonValue(json))];
      let cons = vec![
	  Constructor::Path(
	    vec![
              vec![Constructor::Step(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})}, vec![])],
            ]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() != 0 {
        assert_eq!(e.len(), 3);
        assert_eq!(e[0].to_string(), "200");
        assert_eq!(e[1].to_string(), "true");
        assert_eq!(e[2].to_string(), "{
\"three\": [
\"three one\",
\"three two\",
\"three three\"
]
}");
      } else {
        panic!("empty sequence result")
      }
    }
    #[test]
    fn json_2() {
      let json = object!{
        anint: 200,
	abool: true,
	alist: {
	  three: [
	    "three one",
	    "three two",
	    "three three"
	  ]
	}
      };
      let s = vec![Rc::new(Item::JsonValue(json))];
      let cons = vec![
	  Constructor::Path(
	    vec![
              vec![Constructor::Step(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})}, vec![])],
              vec![Constructor::Step(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})}, vec![])],
              vec![Constructor::Step(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})}, vec![])],
            ]
	  )
	];
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() != 0 {
        assert_eq!(e.len(), 3);
        assert_eq!(e[0].to_string(), "\"three one\"");
        assert_eq!(e[1].to_string(), "\"three two\"");
        assert_eq!(e[2].to_string(), "\"three three\"");
      } else {
        panic!("empty sequence result")
      }
    }
} 

