//! # Evaluate a sequence constructor
//!
//! Evaluate a sequence constructor to produce a sequence.
//!
//! This library uses the traits defined in [Item], so it is independent of the tree implementation.

use std::rc::Rc;
use std::ops::ControlFlow;
use unicode_segmentation::UnicodeSegmentation;
use crate::qname::*;
use crate::xdmerror::*;
use crate::item::{Sequence, SequenceTrait, Item, Value, Document, Node, NodeType, Operator, OutputDefinition};
//use decimal::d128;
use std::collections::HashMap;
use std::cell::{RefCell, RefMut};

/// The dynamic evaluation context.
///
/// The dynamic context stores the value of declared variables.
//#[derive(Clone)]
pub struct DynamicContext<'a> {
  vars: RefCell<HashMap<String, Vec<Sequence>>>,
  templates: Vec<Template>,
  builtin_templates: Vec<Template>,	// TODO: use import precedence for builtins
  depth: RefCell<usize>,
  current_grouping_key: RefCell<Vec<Option<Rc<Item>>>>,
  current_group: RefCell<Vec<Option<Sequence>>>,
  doc: Option<Rc<dyn Document>>,
  // TODO: accept a closure that is a 'Document factory'
  //makedoc: Box<dyn Fn() -> Rc<dyn Document>>,
  resultdoc: Option<&'a dyn Document>,
  od: OutputDefinition,	// Output definition for the final result tree
}

impl<'a> DynamicContext<'a> {
  /// Create a dynamic context.
  pub fn new(resultdoc: Option<&'a dyn Document>) -> DynamicContext<'a> {
    DynamicContext{
      vars: RefCell::new(HashMap::new()),
      templates: Vec::new(),
      builtin_templates: Vec::new(),
      depth: RefCell::new(0),
      current_grouping_key: RefCell::new(vec![None]),
      current_group: RefCell::new(vec![None]),
      doc: None,
      resultdoc: resultdoc,
      od: OutputDefinition::new(),
    }
  }

  /// Add a template to the dynamic context. The first argument is the pattern. The second argument is the body of the template. The third argument is the mode. The fourth argument is the priority.
  pub fn add_template(&mut self,
    p: Vec<Constructor>,
    b: Vec<Constructor>,
    m: Option<String>,
    pr: f64,
  ) {
    self.templates.push(Template{pattern: p, body: b, mode: m, priority: pr});
  }
  /// Add a template to the set of builtin templates in the dynamic context. See above for arguments.
  pub fn add_builtin_template(&mut self,
    p: Vec<Constructor>,
    b: Vec<Constructor>,
    m: Option<String>,
    pr: f64,
  ) {
    self.builtin_templates.push(Template{pattern: p, body: b, mode: m, priority: pr});
  }
  /// Determine if an item matches a pattern and return the highest priority sequence constructor for that template.
  /// If no template is found, returns None.
  pub fn find_match(&self, i: &Rc<Item>) -> Vec<Constructor> {
    let r: Option<&Template> = self.templates.iter()
      .filter(|t| item_matches(self, &t.pattern, i))
      .reduce(|a, b| if a.priority < b.priority {b} else {a});

    if r.is_some() {
      r.unwrap().body.clone()
    } else {
      // Try builtin templates
      let s: Option<&Template> = self.builtin_templates.iter()
        .filter(|t| item_matches(self, &t.pattern, i))
	.reduce(|a, b| if a.priority < b.priority {b} else {a});

      if s.is_some() {
        s.unwrap().body.clone()
      } else {
        vec![]
      }
    }
  }

  pub fn push_current_grouping_key(&self, k: Item) {
    self.current_grouping_key.borrow_mut().push(Some(Rc::new(k)));
  }
  pub fn pop_current_grouping_key(&self) {
    self.current_grouping_key.borrow_mut().pop();
  }

  pub fn push_current_group(&self, g: Sequence) {
    self.current_group.borrow_mut().push(Some(g));
  }
  pub fn pop_current_group(&self) {
    self.current_group.borrow_mut().pop();
  }

  pub fn set_doc(&mut self, d: Rc<dyn Document>) {
    self.doc.replace(d);
  }

  pub fn incr_depth(&self) {
    let cur = *self.depth.borrow();
    self.depth.replace(cur + 1);
  }
  pub fn decr_depth(&self) {
    let cur = *self.depth.borrow();
    self.depth.replace(cur - 1);
  }
  // TODO: return borrowed/reference
  pub fn get_output_definition(&self) -> OutputDefinition {
    self.od.clone()
  }
  pub fn set_output_definition(&mut self, od: OutputDefinition) {
    self.od = od;
  }

  // Stylesheet parameters. Overrides the previous value if it is already set.
  // TODO: namespaced name
  pub fn set_parameter(&mut self, name: String, value: Sequence) {
    self.vars.borrow_mut().insert(name, vec![value]);
  }

  // Printout templates, for debugging.
  pub fn dump_templates(&self) {
    self.templates.iter().for_each(
      |t| {
        println!("Template (mode: {} priority {}) matching pattern:\n{}\nBody:\n{}",
	  t.mode.as_ref().map_or("--no mode--", |u| u.as_str()),
	  t.priority,
	  format_constructor(&t.pattern, 4),
	  format_constructor(&t.body, 4)
	);
      }
    )
  }
}

/// Evaluate a sequence constructor, given a dynamic context.
///
/// The dynamic context consists of the supplied context, as well as the context item. The context item, which is optional, consists of a [Sequence] and an index to an item. If the context sequence is supplied, then the index (posn) must also be supplied and be a valid index for the sequence.
pub fn evaluate(
    dc: &DynamicContext,
    ctxt: Option<Sequence>,
    posn: Option<usize>,
    c: &Vec<Constructor>
  ) -> Result<Sequence, Error> {

  let result: Sequence = c.iter().map(|a| evaluate_one(dc, ctxt.clone(), posn, a).expect("evaluation of item failed")).flatten().collect();
  Ok(result)
}

// Evaluate an item constructor, given a context
// If a constructor returns a non-singleton sequence, then it is unpacked
fn evaluate_one(
    dc: &DynamicContext,
    ctxt: Option<Sequence>,
    posn: Option<usize>,
    c: &Constructor
  ) -> Result<Sequence, Error> {

  match c {
    Constructor::Literal(l) => {
	let mut seq = Sequence::new();
	seq.new_value(l.clone());
	Ok(seq)
    }
    // This creates a Node in the current result document
    Constructor::LiteralElement(n, c) => {
      let l = match dc.resultdoc {
        Some(doc) => {
	  doc.new_element(n.clone()).expect("unable to create Node")
	}
	None => return Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "no result document".to_string()})
      };

      // add content to newly created element
      evaluate(dc, ctxt.clone(), posn, c).expect("failed to evaluate element content").iter()
        .for_each(
	  |i| {
	    // Item could be a Node or text
	    match **i {
	      Item::Node(ref t) => {
		l.append_child(t.as_any()).expect("unable to add child node");
	      }
	      _ => {
	        // Values become a text node in the result tree
		l.append_text_child(Value::String(i.to_string())).expect("unable to add text child node");
	      }
	    }
	  }
	);

      Ok(vec![Rc::new(Item::Node(l))])
    }
    // This creates a Node in the current result document
    Constructor::LiteralAttribute(n, v) => {
      let w = evaluate(dc, ctxt.clone(), posn, v)
        .expect("failed to evaluate attribute value");
      let l = match dc.resultdoc {
        Some(doc) => {
	  doc.new_attribute(n.clone(), Value::String(w.to_string())).expect("unable to create Node")
	}
	None => return Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "no result document".to_string()})
      };

      Ok(vec![Rc::new(Item::Node(l))])
    }
    Constructor::Copy(i, c) => {
      let orig = if i.is_empty() {
        // Copy the context item
	if ctxt.is_some() {
	  vec![ctxt.as_ref().unwrap()[posn.unwrap()].clone()]
	} else {
	  evaluate(dc, ctxt.clone(), posn, i)
	    .expect("failed to evaluate select expression")
	}
      } else {
	evaluate(dc, ctxt.clone(), posn, i)
	  .expect("failed to evaluate select expression")
      };

      let mut result = vec![];
      orig.iter()
        .for_each(
	  |i| {
	    result.push(item_copy(i.clone(), dc, c, ctxt.clone(), posn)
	      .expect("unable to copy item"))
	  }
	);
      Ok(result)
    }
    // Does the same as identity stylesheet template
    Constructor::DeepCopy(sel) => {
      let orig = evaluate(dc, ctxt.clone(), posn, sel)
	    .expect("failed to evaluate select expression");

      let mut result = vec![];
      orig.iter()
        .for_each(
	  |i| {
	    result.push(item_deep_copy((*i).clone(), dc, ctxt.clone(), posn)
	      .expect("unable to copy item"))
	  }
	);
      Ok(result)
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
    Constructor::SetAttribute(n, v) => {
      // The context item must be an element node (TODO: use an expression to select the element)
      // If the element does not have an attribute with the given name, create it
      // Otherwise replace the attribute's value with the supplied value
      if ctxt.is_some() {
        match &*(ctxt.as_ref().unwrap()[posn.unwrap()]) {
	  Item::Node(nd) => {
	    match nd.node_type() {
	      NodeType::Element => {
	        let attval = evaluate(dc, ctxt.clone(), posn, v).expect("unable to evaluate attribute value");
		if attval.len() == 1 {
		  match &*attval[0] {
		    Item::Value(av) => nd.set_attribute(n.clone(), av.clone()),
		    _ => nd.set_attribute(n.clone(), Value::String(attval.to_string())),
		  }
		} else {
		  nd.set_attribute(n.clone(), Value::String(attval.to_string()))
		}
		Ok(vec![])
	      }
	      _ => Result::Err(Error{kind: ErrorKind::TypeError, message: "context item is not an element".to_string()})
	    }
	  }
	  _ => Result::Err(Error{kind: ErrorKind::TypeError, message: "context item must be an element node".to_string()})
	}
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
      seq.new_value(Value::String(r));
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
	  Item::Document(_) => {
	    Ok(vec![Rc::clone(&ctxt.unwrap()[posn.unwrap()])])
	  }
	  // Some implementations represent the document as a special kind of node
	  Item::Node(n) => {
	    match dc.doc {
	      Some(ref d) => {
	        Ok(vec![Rc::new(Item::Document(Rc::clone(&d)))])
	      }
	      None => {
	        // Try to navigate to the Document from the Node
		match n.owner_document() {
		  Some(d) => Ok(vec![Rc::new(Item::Document(d))]),
		  None => Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: String::from("no current document (root)")}),
		}
	      }
	    }
	  }
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

      Ok(s.iter().fold(
	    u,
	    |a, c| {
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
	match &*(ctxt.as_ref().unwrap()[posn.unwrap()]) {
	  Item::Document(d) => {
	    match nm.axis {
	      Axis::Child => {
	        match d.get_root_element() {
		  Some(n) => {
		    if is_node_match(&nm.nodetest, &n) {
		      let seq = vec![Rc::new(Item::Node(n))];
		      Ok(predicates(dc, seq, p))
		    } else {
		      Ok(vec![])
		    }
		  }
		  None => {
		    Ok(vec![])
		  }
		}
	      }
	      // Only used for pattern matching: matches "/"
	      Axis::SelfDocument |
	      Axis::ParentDocument => {
	        Ok(vec![Rc::clone(&ctxt.unwrap()[posn.unwrap()])])
	      }
	      Axis::Parent |
	      Axis::Selfaxis |
	      Axis::Attribute => Ok(vec![]),
	      _ => {
	        // Not yet implemented
		Result::Err(Error{kind: ErrorKind::NotImplemented, message: "not yet implemented (document)".to_string()})
	      }
	    }
	  }
	  Item::Node(n) => {
	    match nm.axis {
	      Axis::Selfaxis => {
	        if is_node_match(&nm.nodetest, &n) {
		  let mut seq = Sequence::new();
		  seq.new_node(Rc::clone(n));
	      	  Ok(predicates(dc, seq, p))
		} else {
	      	  Ok(Sequence::new())
		}
	      }
	      Axis::Child => {
		let seq = n.children().iter()
		      .filter(|c| is_node_match(&nm.nodetest, &c))
		      .fold(Sequence::new(), |mut c, a| {c.new_node(Rc::clone(a)); c});
		Ok(predicates(dc, seq, p))
	      }
	      Axis::Parent => {
	        match n.parent() {
		  Some(p) => {
		    let mut seq = Sequence::new();
		    seq.new_node(Rc::clone(&p));
      		    Ok(seq)
		  }
		  None => {
	            // empty sequence is the result
      		    Ok(vec![])
		  }
		}
	      }
	      Axis::ParentDocument => {
	        // Only matches the Document.
		// If no parent then return the Document
	        match n.parent() {
		  Some(_) => {
      		    Ok(vec![])
		  }
		  None => {
	    	    match dc.doc {
	      	      Some(ref d) => {
	                Ok(vec![Rc::new(Item::Document(Rc::clone(&d)))])
	      	      }
	      	      None => {
		        match n.owner_document() {
			  Some(d) => Ok(vec![Rc::new(Item::Document(d))]),
			  None => Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: String::from("no current document (parent)")})
			}
		      }
	    	    }
		  }
		}
	      }
	      Axis::Descendant => {
		let seq = n.descendants().iter()
		  .filter(|c| is_node_match(&nm.nodetest, &c))
		  .fold(Sequence::new(), |mut c, a| {c.new_node(Rc::clone(a)); c});
	      	Ok(predicates(dc, seq, p))
	      }
	      Axis::DescendantOrSelf => {
		let mut seq = n.descendants().iter()
		  .filter(|c| is_node_match(&nm.nodetest, &c))
		  .fold(Sequence::new(), |mut c, a| {c.new_node(Rc::clone(a)); c});
		if is_node_match(&nm.nodetest, &n) {
		  seq.insert(0, Rc::new(Item::Node(Rc::clone(n))));
		}
	      	Ok(predicates(dc, seq, p))
	      }
	      Axis::Ancestor => {
		let seq = n.ancestors().iter()
		  .filter(|p| is_node_match(&nm.nodetest, &p))
		  .fold(Sequence::new(), |mut c, a| {c.new_node(Rc::clone(a)); c});
	      	Ok(predicates(dc, seq, p))
	      }
	      Axis::AncestorOrSelf => {
		let mut seq = n.ancestors().iter()
		  .filter(|c| is_node_match(&nm.nodetest, &c))
		  .fold(Sequence::new(), |mut c, a| {c.new_node(Rc::clone(a)); c});
		if is_node_match(&nm.nodetest, &n) {
		  seq.insert(0, Rc::new(Item::Node(Rc::clone(n))));
		}
	      	Ok(predicates(dc, seq, p))
	      }
	      Axis::FollowingSibling => {
	        let seq = n.following_siblings().iter()
		  .filter(|c| is_node_match(&nm.nodetest, &c))
		  .fold(Sequence::new(), |mut c, a| {c.new_node(Rc::clone(a)); c});
	      	Ok(predicates(dc, seq, p))
	      }
	      Axis::PrecedingSibling => {
		let seq = n.preceding_siblings().iter()
		  .filter(|c| is_node_match(&nm.nodetest, &c))
		  .fold(Sequence::new(), |mut c, a| {c.new_node(Rc::clone(a)); c});
	      	Ok(predicates(dc, seq, p))
	      }
	      Axis::Following => {
	        // XPath 3.3.2.1: the following axis contains all nodes that are descendants of the root of the tree in which the context node is found, are not descendants of the context node, and occur after the context node in document order.
		// iow, for each ancestor-or-self node, include every next sibling and its descendants

		let mut d: Vec<Rc<dyn Node>> = Vec::new();

		// Start with following siblings of self
		for a in n.following_siblings() {
		  d.push(a.clone());
		  let mut b = a.descendants();
		  d.append(&mut b);
		}

		// Now traverse ancestors
		let anc: Vec<Rc<dyn Node>> = n.ancestors();
		for a in anc {
		  let sibs: Vec<Rc<dyn Node>> = a.following_siblings();
		  for b in sibs {
		    d.push(b.clone());
		    let mut sib_descs: Vec<Rc<dyn Node>> = b.descendants();
		    d.append(&mut sib_descs)
		  }
		}
	        let seq = d.iter()
		  .filter(|e| is_node_match(&nm.nodetest, &e))
		  .fold(Sequence::new(), |mut f, g| {f.new_node(Rc::clone(g)); f});
	      	Ok(predicates(dc, seq, p))
	      }
	      Axis::Preceding => {
	        // XPath 3.3.2.1: the preceding axis contains all nodes that are descendants of the root of the tree in which the context node is found, are not ancestors of the context node, and occur before the context node in document order.
		// iow, for each ancestor-or-self node, include every previous sibling and its descendants

		let mut d: Vec<Rc<dyn Node>> = Vec::new();

		// Start with preceding siblings of self
		for a in n.preceding_siblings() {
		  d.push(a.clone());
		  let mut b = a.descendants();
		  d.append(&mut b);
		}

		// Now traverse ancestors
		let anc: Vec<Rc<dyn Node>> = n.ancestors();
		for a in anc {
		  let sibs: Vec<Rc<dyn Node>> = a.preceding_siblings();
		  for b in sibs {
		    d.push(b.clone());
		    let mut sib_descs: Vec<Rc<dyn Node>> = b.descendants();
		    d.append(&mut sib_descs)
		  }
		}
	        let seq = d.iter()
		  .filter(|e| is_node_match(&nm.nodetest, &e))
		  .fold(Sequence::new(), |mut f, g| {f.new_node(Rc::clone(g)); f});
	      	Ok(predicates(dc, seq, p))
	      }
	      Axis::Attribute => {
		let attrs = n.attributes().iter()
		  .filter(|c| is_node_match(&nm.nodetest, &c))
		  .fold(Sequence::new(), |mut c, a| {c.new_node(Rc::clone(a)); c});
		Ok(predicates(dc, attrs, p))
	      }
	      Axis::SelfDocument => Ok(vec![]),
	      _ => {
	        // Not yet implemented
		Result::Err(Error{kind: ErrorKind::NotImplemented, message: "not yet implemented (node)".to_string()})
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
      	  // Evaluate the arguments
      	  let mut b = Vec::new();
      	  for c in a {
            b.push(evaluate(dc, ctxt.clone(), posn, c).expect("argument evaluation failed"))
      	  }
      	  // Invoke the function
      	  g(dc, ctxt, posn, b)
	}
	None => {
	  Result::Err(Error{kind: ErrorKind::NotImplemented, message: format!("call to undefined function \"{}\"", f.name)})
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
        match v.chunks(2).try_fold(
          evaluate(dc, ctxt.clone(), posn, o).expect("failed to evaluate otherwise clause"),
	  |acc, t| {
	    if evaluate(dc, ctxt.clone(), posn, &t[0]).expect("failed to evaluate clause test").to_bool() {
	      ControlFlow::Break(evaluate(dc, ctxt.clone(), posn, &t[1]).expect("failed to evaluate clause body"))
	    } else {
	      ControlFlow::Continue(acc)
	    }
	  }
        ) {
	  ControlFlow::Continue(r) => r,
	  ControlFlow::Break(r) => r,
	}
      )
    }
    Constructor::ApplyTemplates(s) => {
      // Evaluate 's' to find the nodes to apply templates to
      // For each node, find a matching template and evaluate its sequence constructor. The result of that becomes an item in the new sequence

      let sel = evaluate(dc, ctxt.clone(), posn, s).expect("failed to evaluate select expression");
      let result = sel.iter().fold(
          vec![],
          |mut acc, i| {
	    let matching_template: Vec<&Template> = dc.templates.iter()
	      .filter(|t| {
	        //item_matches(dc, &t.pattern, i)
		let e = evaluate(dc, Some(vec![i.clone()]), Some(0), &t.pattern).expect("failed to evaluate pattern");
	        if e.len() == 0 {false} else {true}
	      })
	      .scan(-2.0,
	        |prio, t| {
		  if *prio < t.priority {
		    *prio = t.priority;
		    Some(t)
		  } else {
		    None
		  }
		}
	      )
	      .collect();
	    // there must be at most one matching template
	    if matching_template.len() > 1 {
	      //return Result::Err(Error{kind: ErrorKind::TypeError, message: "too many matching templates".to_string()})
	      panic!("too many matching templates")
	    }
	    // If no templates match then apply a built-in template
	    // See XSLT 6.7.
	    // TODO: use import precedence to implement this feature
	    if matching_template.len() == 0 {
	      let builtin_template: Vec<&Template> = dc.builtin_templates.iter()
	        .filter(|t| {
		  let e = evaluate(dc, Some(vec![i.clone()]), Some(0), &t.pattern).expect("failed to evaluate pattern");
	          if e.len() == 0 {false} else {true}
	        })
	        .scan(-2.0,
	          |prio, t| {
		    if *prio < t.priority {
		      *prio = t.priority;
		      Some(t)
		    } else {
		      None
		    }
		  }
	        )
	        .collect();
	      if builtin_template.len() > 1 {
	        panic!("too many matching builtin templates")
	      }
	      let mut u = builtin_template.iter()
	        .flat_map(|t| {
		  dc.incr_depth();
		  let rs = evaluate(dc, Some(vec![i.clone()]), Some(0), &t.body).expect("failed to evaluate template body");
	    	  dc.decr_depth();
		  rs
	        })
	        .collect::<Sequence>();
	      acc.append(&mut u);
	    } else {
	      let mut u = matching_template.iter()
	        .flat_map(|t| {
		  dc.incr_depth();
		  let rs = evaluate(dc, Some(vec![i.clone()]), Some(0), &t.body).expect("failed to evaluate template body");
	    	  dc.decr_depth();
		  rs
	        })
	        .collect::<Sequence>();
	      acc.append(&mut u);
	    }
	    acc
	  }
        );
      Ok(result)
    }
    Constructor::ForEach(s, t, g) => {
      // Evaluate 's' to find the nodes to iterate over
      // Use 'g' to group the nodes
      // Evaluate 't' for each group
      let sel = evaluate(dc, ctxt.clone(), posn, s).expect("failed to evaluate select expression");
      // Divide sel into groups: each item in groups is an individual group
      let mut groups = Vec::new();
      match g {
        Some(Grouping::By(h)) => {
	  // 'h' is an expression that when evaluated for an item results in zero or more grouping keys.
	  // Items are placed in the group with a matching key
	  let mut map = HashMap::new();
	  for i in 0..sel.len() {
	    let keys = evaluate(dc, Some(sel.clone()), Some(i), h).expect("failed to evaluate key");
	    for j in keys {
	      let e = map.entry(j.to_string()).or_insert(vec![]);
	      e.push(sel[i].clone());
	    }
	  }
	  // Now construct the groups and a pair-wise vector of keys
	  for (k, v) in map.iter() {
	    groups.push((Some(k.clone()), v.to_vec()));
	  }
	}
        Some(Grouping::Adjacent(h)) => {
	  // 'h' is an expression that is evaluated for every item in 'sel'.
	  // It must evaluate to a single item.
	  // The first item starts the first group.
	  // For the second and subsequent items, if the result of 'h; is the same as the previous item's 'h'
	  // then it is added to the current group. Otherwise it starts a new group.
	  if sel.len() > 0 {
	    let mut curgrp = vec![sel[0].clone()];
	    let mut curkey = evaluate(dc, Some(sel.clone()), Some(1), h).expect("failed to evaluate key");
	    if curkey.len() != 1 {
	      return Result::Err(Error{kind: ErrorKind::Unknown, message: "group-adjacent attribute must evaluate to a single item".to_string()})
	    }
	    for i in 1..sel.len() {
	      let thiskey = evaluate(dc, Some(sel.clone()), Some(i), h).expect("failed to evaluate key");
	      if thiskey.len() == 1 {
		if curkey[0].compare(&*thiskey[0], Operator::Equal).expect("unable to compare keys") {
		  // Append to the current group
		  curgrp.push(sel[i].clone());
		} else {
		  // Close previous group, start a new group with this item as its first member
		  groups.push((Some(curkey.to_string()), curgrp.clone()));
		  curgrp = vec![sel[i].clone()];
		  curkey = thiskey;
		}
	      } else {
      	        return Result::Err(Error{kind: ErrorKind::TypeError, message: "group-adjacent attribute must evaluate to a single item".to_string()})
	      }
	    }
	    // Close the last group
	    groups.push((Some(curkey.to_string()), curgrp));
	  } // else result is empty sequence
	}
        Some(Grouping::StartingWith(_h)) => {
	}
        Some(Grouping::EndingWith(_h)) => {
	}
        None => {
	  for i in sel {
            groups.push((None, vec![i.clone()]));
	  }
	}
      }

      Ok(groups.iter().fold(
        vec![],
	|mut result, grp| {
	  let (o, v) = grp;
	  // set current-grouping-key, current-group
	  match o {
	    Some(u) => {
	      dc.push_current_grouping_key(Item::Value(Value::String(u.to_string())));
	      dc.push_current_group(v.clone());
	    }
	    None => {}
	  }
	  let mut tmp = evaluate(dc, Some(v.to_vec()), Some(0), t).expect("failed to evaluate template");
	  result.append(&mut tmp);
	  // Restore current-grouping-key, current-group
	  dc.pop_current_grouping_key();
	  dc.pop_current_group();
	  result
	}
      ))
    }
    Constructor::NotImplemented(m) => {
      Result::Err(Error{kind: ErrorKind::NotImplemented, message: format!("sequence constructor not implemented: {}", m)})
    }
  }
}

// Deep copy an item
fn item_deep_copy(
  orig: Rc<Item>,
  dc: &DynamicContext,
  ctxt: Option<Sequence>,
  posn: Option<usize>,
) -> Result<Rc<Item>, Error> {

  let cp = item_copy(orig.clone(), dc, &vec![], ctxt.clone(), posn).expect("unable to copy item");

  // If this item is an element node, then copy all of its attributes and children
  match *orig {
    Item::Node(ref n) => {
      match n.node_type() {
        NodeType::Element => {
	  let cur = match *cp {
	    Item::Node(ref m) => m,
	    _ => {
	      return Result::Err(Error{kind: ErrorKind::Unknown, message: "unable to copy element node".to_string()})
	    }
	  };
	  n.attributes().iter()
	    .for_each(|a| {
	      cur.set_attribute(a.to_name(), Value::String(a.to_string()));
	    });
	  let child_list = n.children();
	  child_list.iter()
	    .for_each(|c| {
	      let cpc = item_deep_copy(Rc::new(Item::Node(c.clone())), dc, ctxt.clone(), posn)
	        .expect("unable to copy item");
	      match *cpc {
	        Item::Node(ref cpcn) => {
	      	  cur.append_child(cpcn.as_any()).expect("unable to append copied child");
		}
		_ => {} // this should never happen
	      }
	    });
	}
	_ => {}
      }
    }
    _ => {}
  }

  Ok(cp)
}

// Copy an item
fn item_copy(
  orig: Rc<Item>,
  dc: &DynamicContext,
  content: &Vec<Constructor>,
  ctxt: Option<Sequence>,
  posn: Option<usize>,
) -> Result<Rc<Item>, Error> {
  match *orig {
    Item::Value(_) => {
      Ok(orig.clone())
    }
    Item::Node(ref n) => {
      let doc = match dc.resultdoc {
        Some(d) => d,
	None => {
	  return Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "no result document".to_string()})
	}
      };

      match n.node_type() {
        NodeType::Element => {
	  match doc.new_element(n.to_name()) {
	    Ok(e) => {
	      // Add content to the new element
	      evaluate(dc, ctxt.clone(), posn, content)
	        .expect("failed to evaluate element content")
		.iter()
        	  .for_each(|i| {
	    	    // Item could be a Node or text
	    	    match **i {
	      	      Item::Node(ref t) => {
		        match t.node_type() {
		        NodeType::Element |
		        NodeType::Text => {
		          e.append_child(t.as_any()).expect("unable to add child node");
		        }
		        NodeType::Attribute => {
		          e.add_attribute_node(t.as_any()).expect("unable to add attribute node");
		        }
		        _ => {} // TODO: work out what to do with documents, etc
		      }
	      	    }
	      	    _ => {
	              // Values become a text node in the result tree
		      e.append_text_child(Value::String(i.to_string()))
		        .expect("unable to add text child node");
	      	    }
	    	  }
	        });
	      Ok(Rc::new(Item::Node(e)))
	    }
	    _ => {
	      return Result::Err(Error{kind: ErrorKind::Unknown, message: "unable to create element node".to_string()})
	    }
	  }
	}
	NodeType::Text => {
	  match doc.new_text(Value::String(n.to_string())) {
	    Ok(m) => {
	      Ok(Rc::new(Item::Node(m)))
	    }
	    _ => {
	      return Result::Err(Error{kind: ErrorKind::Unknown, message: "unable to create text node".to_string()})
	    }
	  }
	}
	NodeType::Attribute => {
	  // TODO: add a 'to_value' method
	  match doc.new_attribute(n.to_name(), Value::String(n.to_string())) {
	    Ok(a) => {
	      Ok(Rc::new(Item::Node(a)))
	    }
	    _ => {
	      Result::Err(Error{kind: ErrorKind::Unknown, message: "unable to create attribute node".to_string()})
	    }
	  }
	}
	_ => {
	  Result::Err(Error{kind: ErrorKind::NotImplemented, message: "select expression not implemented".to_string()})
	}
      }
    }
    _ => {
      Result::Err(Error{kind: ErrorKind::NotImplemented, message: "not implemented".to_string()})
    }
  }
}

// Push a new scope for a variable
fn var_push(dc: &DynamicContext, v: &str, i: &Rc<Item>) {
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
fn predicates(dc: &DynamicContext, s: Sequence, p: &Vec<Vec<Constructor>>) -> Sequence {
  if p.is_empty() {
    s
  } else {
    let mut result = s.clone();

    // iterate over the predicates
    for q in p {
      let mut new: Sequence = Vec::new();

      // for each predicate, evaluate each item in s to a boolean
      for i in 0..result.len() {
        let b = evaluate(dc, Some(result.clone()), Some(i), q).expect("evaluating predicate failed");
	if b.to_bool() == true {
	  new.push(result[i].clone());
	}
      }
      result.clear();
      result.append(&mut new);
    }

    result
  }
}

/// Specifies how a sequence is to be constructed.
///
/// These are usually included in a Vector, where each Constructor builds an item. If the constructor results in a singleton, then it becomes an item in the [Sequence], otherwise the sequence is unpacked into the parent [Sequence].
#[derive(Clone)]
pub enum Constructor {
  /// A literal, atomic value
  Literal(Value),
  /// A literal element. This will become a node in the result tree.
  /// TODO: this may be merged with the Literal option in a later version.
  /// Arguments are: element name, content
  LiteralElement(QualifiedName, Vec<Constructor>),
  /// A literal attribute. This will become a node in the result tree.
  /// TODO: allow for attribute value templates
  /// Arguments are: attribute name, value
  LiteralAttribute(QualifiedName, Vec<Constructor>),
  /// Construct a node by copying something. The first argument is what to copy; an empty vector selects the current item. The second argument constructs the content.
  Copy(Vec<Constructor>, Vec<Constructor>),
  DeepCopy(Vec<Constructor>),
  /// The context item from the dynamic context
  ContextItem,
  /// Logical OR. Each element of the outer vector is an operand.
  Or(Vec<Vec<Constructor>>),
  /// Logical AND. Each element of the outer vector is an operand.
  And(Vec<Vec<Constructor>>),
  // Union,
  // IntersectExcept,
  // InstanceOf,
  // Treat,
  // Castable,
  // Cast,
  // Arrow,
  // Unary,
  // SimpleMap,
  /// Root node of the context item
  Root,
  /// A path in a tree of nodes.
  /// Each element of the outer vector is a step in the path.
  /// The result of each step becomes the new context for the next step.
  Path(Vec<Vec<Constructor>>),
  /// A step in a path.
  /// The second argument is zero or more predicates.
  /// Each item in the result sequence is evaluated against each predicate as a boolean.
  /// If the predicate evaluates to true it is kept, otherwise it is discarded.
  Step(NodeMatch, Vec<Vec<Constructor>>),
  /// XPath general comparison.
  /// Each element of the outer vector is a comparator.
  /// If the comparator is a sequence then each item is compared.
  GeneralComparison(Operator, Vec<Vec<Constructor>>),
  /// XPath value comparison. Compares single items.
  ValueComparison(Operator, Vec<Vec<Constructor>>),
  // Is,
  // Before,
  // After,
  /// Concatentate string values
  Concat(Vec<Vec<Constructor>>),
  /// Construct a range of integers
  Range(Vec<Vec<Constructor>>),
  /// Perform addition, subtraction, multiply, divide
  Arithmetic(Vec<ArithmeticOperand>),
  /// Call a function
  FunctionCall(Function, Vec<Vec<Constructor>>),
  /// Declare a variable.
  /// The variable will be available for subsequent constructors
  VariableDeclaration(String, Vec<Constructor>),	// TODO: support QName
  /// Reference a variable.
  VariableReference(String),				// TODO: support QName
  /// Repeating constructor (i.e. 'for').
  /// The first argument declares variables.
  /// The second argument is the body of the loop.
  Loop(Vec<Constructor>, Vec<Constructor>),
  /// Selects an arm to evaluate.
  /// The first argument is pairs of (test,body) clauses.
  /// The second argument is the otherwise clause
  Switch(Vec<Vec<Constructor>>, Vec<Constructor>),
  /// Find a matching template and evaluate its sequence constructor.
  /// The argument is the select attribute.
  ApplyTemplates(Vec<Constructor>),
  /// Evaluate a sequence constructor for each item, possibly grouped.
  /// First argument is the select expression, second argument is the template,
  /// third argument is the (optional) grouping spec.
  ForEach(Vec<Constructor>, Vec<Constructor>, Option<Grouping>),
  /// Set the value of an attribute. Context item must be an element node.
  /// First argument is the name of the attribute, second attribute is the value to set
  SetAttribute(QualifiedName, Vec<Constructor>),
  /// Something that is not yet implemented
  NotImplemented(String),
}

/// Determine how a collection is to be divided into groups.
/// This enum would normally be inside an Option. The None value means that the collection is not to be grouped.
#[derive(Clone)]
pub enum Grouping {
  By(Vec<Constructor>),
  StartingWith(Vec<Constructor>),
  EndingWith(Vec<Constructor>),
  Adjacent(Vec<Constructor>),
}

/// Determine if an item matches a pattern.
/// The sequence constructor is a pattern: the steps of a path in reverse.
pub fn item_matches(dc: &DynamicContext, pat: &Vec<Constructor>, i: &Rc<Item>) -> bool {
  let e = evaluate(dc, Some(vec![i.clone()]), Some(0), pat)
    .expect("pattern evaluation failed");

  // If anything is left in the context then the pattern matched
  if e.len() != 0 {
    true
  } else {
    false
  }
}

// Apply the node test to a Node.
fn is_node_match(nt: &NodeTest, n: &Rc<dyn Node>) -> bool {
  match nt {
    NodeTest::Name(t) => {
      match n.node_type() {
        NodeType::Element |
	NodeType::Attribute => {
      	  // TODO: namespaces
      	  match &t.name {
            Some(a) => {
	      match a {
	        WildcardOrName::Wildcard => {
	      	  true
	    	}
	    	WildcardOrName::Name(s) => {
	      	  *s == n.to_name().get_localname()
	    	}
	      }
	    }
	    None => {
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
          match n.node_type() {
	    NodeType::Document => true,
	    _ => false,
	  }
        }
        KindTest::ElementTest => {
          match n.node_type() {
	    NodeType::Element => true,
	    _ => false,
	  }
        }
        KindTest::PITest => {
          match n.node_type() {
	    NodeType::ProcessingInstruction => true,
	    _ => false,
	  }
        }
        KindTest::CommentTest => {
      	  match n.node_type() {
	    NodeType::Comment => true,
	    _ => false,
	  }
        }
        KindTest::TextTest => {
      	  match n.node_type() {
	    NodeType::Text => true,
	    _ => false,
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
  pub fn from(s: &str) -> Result<NodeTest, Error> {
    // Import this from xpath.rs?
    let tok: Vec<&str> = s.split(':').collect();
    match tok.len() {
      1 => {
        // unprefixed
	if tok[0] == "*" {
	  Ok(NodeTest::Name(NameTest{name: Some(WildcardOrName::Wildcard), ns: None, prefix: None}))
	} else {
	  Ok(NodeTest::Name(NameTest{name: Some(WildcardOrName::Name(tok[0].to_string())), ns: None, prefix: None}))
	}
      }
      2 => {
        // prefixed
	if tok[0] == "*" {
	  if tok[1] == "*" {
	    Ok(NodeTest::Name(NameTest{name: Some(WildcardOrName::Wildcard), ns: Some(WildcardOrName::Wildcard), prefix: None}))
	  } else {
	    Ok(NodeTest::Name(NameTest{name: Some(WildcardOrName::Name(tok[1].to_string())), ns: Some(WildcardOrName::Wildcard), prefix: None}))
	  }
	} else {
	  if tok[1] == "*" {
	    Ok(NodeTest::Name(NameTest{name: Some(WildcardOrName::Wildcard), ns: None, prefix: Some(tok[0].to_string())}))
	  } else {
	    Ok(NodeTest::Name(NameTest{name: Some(WildcardOrName::Name(tok[1].to_string())), ns: None, prefix: Some(tok[0].to_string())}))
	  }
	}
      }
      _ => Result::Err(Error{kind: ErrorKind::TypeError, message: "invalid NodeTest".to_string()})
    }
  }
  pub fn to_string(&self) -> String {
      match self {
        NodeTest::Name(nt) => {
	  nt.to_string()
	}
	NodeTest::Kind(kt) => {
	  kt.to_string().to_string()
	}
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

impl KindTest {
  pub fn to_string(&self) -> &'static str {
    match self {
      KindTest::DocumentTest => "DocumentTest",
      KindTest::ElementTest => "ElementTest",
      KindTest::AttributeTest => "AttributeTest",
      KindTest::SchemaElementTest => "SchemaElementTest",
      KindTest::SchemaAttributeTest => "SchemaAttributeTest",
      KindTest::PITest => "PITest",
      KindTest::CommentTest => "CommentTest",
      KindTest::TextTest => "TextTest",
      KindTest::NamespaceNodeTest => "NamespaceNodeTest",
      KindTest::AnyKindTest => "AnyKindTest",
    }
  }
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
  SelfDocument, // a special axis, only for matching the Document in a pattern match
  Following,
  FollowingSibling,
  Namespace,
  Parent,
  ParentDocument, // a special axis, only for matching in a pattern match. Matches the parent as well as the Document.
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
      Axis::SelfDocument => "self-document".to_string(),
      Axis::Following => "following".to_string(),
      Axis::FollowingSibling => "following-sibling".to_string(),
      Axis::Namespace => "namespace".to_string(),
      Axis::Parent => "parent".to_string(),
      Axis::ParentDocument => "parent-document".to_string(),
      Axis::Ancestor => "ancestor".to_string(),
      Axis::AncestorOrSelf => "ancestor-or-self".to_string(),
      Axis::Preceding => "preceding".to_string(),
      Axis::PrecedingSibling => "preceding-sibling".to_string(),
      _ => "unknown".to_string(),
    }
  }
  fn opposite(&self) -> Axis {
    // SelfDocument opposite is undefined
    match self {
      Axis::Child => Axis::Parent,
      Axis::Descendant => Axis::Ancestor,
      Axis::DescendantOrSelf => Axis::AncestorOrSelf,
      Axis::Attribute => Axis::Parent,
      Axis::Selfaxis => Axis::Selfaxis,
      Axis::Following => Axis::Preceding,
      Axis::FollowingSibling => Axis::PrecedingSibling,
      Axis::Namespace => Axis::Parent,
      Axis::Parent => Axis::Child,
      Axis::Ancestor => Axis::Descendant,
      Axis::AncestorOrSelf => Axis::DescendantOrSelf,
      Axis::Preceding => Axis::Following,
      Axis::PrecedingSibling => Axis::FollowingSibling,
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
pub struct ArithmeticOperand {
  pub op: ArithmeticOperator,
  pub operand: Vec<Constructor>,
}

fn general_comparison(dc: &DynamicContext, ctxt: Option<Sequence>, posn: Option<usize>, op: Operator, left: &Vec<Constructor>, right: &Vec<Constructor>) -> Result<bool, Error> {
  let mut b = false;
  let left_seq = evaluate(dc, ctxt.clone(), posn, left).expect("evaluating left-hand sequence failed");
  let right_seq = evaluate(dc, ctxt.clone(), posn, right).expect("evaluating right-hand sequence failed");
  for l in left_seq {
    for r in &right_seq {
      b = l.compare(&*r, op).unwrap();
      if b { break }
    }
    if b { break }
  };
  Ok(b)
}

/// A pattern is basically a Sequence Constructor in reverse.
/// An item is evaluated against the expression, and if the result is a non-empty sequence then the pattern has matched.
///
/// Converts a Sequence Constructor to a pattern, consuming the constructor. The Constructor must be a Path. The result Constructor is also a path, but it's steps are in reverse.
pub fn to_pattern(sc: Vec<Constructor>) -> Result<Vec<Constructor>, Error> {
    if sc.len() == 1 {
      match sc[0] {
	Constructor::Root => {
	  Ok(vec![
	    Constructor::Step(
	      NodeMatch {
	        axis: Axis::SelfDocument,
	        nodetest: NodeTest::Kind(KindTest::AnyKindTest),
	      },
	      vec![]
	    )
	  ])
	}
	Constructor::Path(ref s) => {
          if s.len() == 0 {
            return Result::Err(Error{kind: ErrorKind::TypeError, message: "sequence constructor must not be empty".to_string()})
	  }
	  let mut p: Vec<Vec<Constructor>> = Vec::new();
	  let mut it = s.iter().rev();
	  let step0 = it.next().unwrap(); // We've already checked that there is at least one step
	  let mut last_axis;
	  if step0.len() == 1 {
	    match step0[0] {
              Constructor::Root => {
	        p.push(vec![
		  Constructor::Step(
		    NodeMatch{axis: Axis::SelfDocument, nodetest: NodeTest::Kind(KindTest::AnyKindTest)},
		    vec![]
		  )
		]);
		last_axis = Axis::SelfDocument;
	      }
	      Constructor::Step(NodeMatch{axis: a, nodetest: ref nt}, _) => {
	        p.push(vec![
	          Constructor::Step(
	            NodeMatch{
		      axis: match a {
	                Axis::Child |
	          	Axis::Selfaxis => {
			  Axis::Selfaxis
			}
	         	_ => {
			  a.opposite()
			}
	              },
		      nodetest: nt.clone()
		    },
		    vec![],
	          )
	        ]);
	        last_axis = a.opposite();
	      }
	      _ => return Result::Err(Error{kind: ErrorKind::TypeError, message: "sequence constructor must be a step (1)".to_string()}),
	    };
	  } else {
	    return Result::Err(Error{kind: ErrorKind::TypeError, message: "sequence constructor must be steps".to_string()})
	  }

	  loop {
	    let n = it.next();
	    if n.is_none() {break};
	    if n.unwrap().len() != 1 {return Result::Err(Error{kind: ErrorKind::TypeError, message: "sequence constructor must be a step (2)".to_string()})};

	    // TODO: predicates
	    match n.unwrap()[0] {
	      Constructor::Root => p.push(
	        vec![
		  Constructor::Step(
		    NodeMatch{
		      axis: Axis::ParentDocument,
		      nodetest: NodeTest::Kind(KindTest::AnyKindTest),
		    },
		    vec![],
		  )
		]
	      ),
	      Constructor::Step(NodeMatch{axis: _, nodetest: ref nt}, _) => p.push(
	        vec![
	          Constructor::Step(
	            NodeMatch{
		      axis: last_axis,
		      nodetest: nt.clone()
		    },
		    vec![],
	          )
	        ]
	      ),
	      _ => return Result::Err(Error{kind: ErrorKind::TypeError, message: "sequence constructor must be a step (3)".to_string()}),
	    }

	    last_axis = match n.unwrap()[0] {
	      Constructor::Step(NodeMatch{axis: a, ..}, _) => a.opposite(),
	      _ => Axis::Unknown,
	    }
	  }
	  Ok(vec![Constructor::Path(p)])
        }
	Constructor::Step(NodeMatch{axis: a, nodetest: ref nt}, _) => {
	  Ok(vec![
	    Constructor::Step(
	      NodeMatch{
	        axis: match a {
	          Axis::Child |
	          Axis::Selfaxis => {
		    Axis::Selfaxis
		  }
	          _ => {
		    a.opposite()
		  }
	        },
		nodetest: nt.clone()
	      },
	      vec![],
	    )
	  ])
	}
        _ => {
	  Result::Err(Error{kind: ErrorKind::TypeError, message: "sequence constructor must be a path".to_string()})
        }
      }
    } else {
      Result::Err(Error{kind: ErrorKind::TypeError, message: "sequence constructor must be a singleton".to_string()})
    }
}

/// A template associating a pattern to a sequence constructor
#[derive(Clone)]
pub struct Template {
  pattern: Vec<Constructor>,
  body: Vec<Constructor>,
  priority: f64,
  mode: Option<String>,
}

/// # Static context
///
/// Provide a static context and analysis for a [Sequence] [Constructor].
///
/// Currently, this stores the set of functions and variables available to a constructor.
pub struct StaticContext {
  pub funcs: RefCell<HashMap<String, Function>>,
  pub vars: RefCell<HashMap<String, Vec<Sequence>>>, // each entry in the vector is an inner scope of the variable
}

impl StaticContext {
  /// Creates a new StaticContext.
  pub fn new() -> StaticContext {
    StaticContext{
      funcs: RefCell::new(HashMap::new()),
      vars: RefCell::new(HashMap::new()),
    }
  }
  /// Creates a new StaticContext and initializes it with the pre-defined XPath functions.
  ///
  /// Currently, this is the functions defined for XPath 1.0:
  ///
  /// * position()
  /// * last()
  /// * count()
  /// * local-name()
  /// * name()
  /// * string()
  /// * concat()
  /// * starts-with()
  /// * contains()
  /// * substring()
  /// * substring-before()
  /// * substring-after()
  /// * normalize-space()
  /// * translate()
  /// * boolean()
  /// * not()
  /// * true()
  /// * false()
  /// * number()
  /// * sum()
  /// * floor()
  /// * ceiling()
  /// * round()
  pub fn new_with_builtins() -> StaticContext {
    let sc = StaticContext{
      funcs: RefCell::new(HashMap::new()),
      vars: RefCell::new(HashMap::new()),
    };
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
    sc.funcs.borrow_mut().insert("string".to_string(),
      Function{
        name: "string".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_string)
      }
    );
    sc.funcs.borrow_mut().insert("concat".to_string(),
      Function{
        name: "concat".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_concat)
      }
    );
    sc.funcs.borrow_mut().insert("starts-with".to_string(),
      Function{
        name: "starts-with".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_startswith)
      }
    );
    sc.funcs.borrow_mut().insert("contains".to_string(),
      Function{
        name: "contains".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_contains)
      }
    );
    sc.funcs.borrow_mut().insert("substring".to_string(),
      Function{
        name: "substring".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_substring)
      }
    );
    sc.funcs.borrow_mut().insert("substring-before".to_string(),
      Function{
        name: "substring-before".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_substringbefore)
      }
    );
    sc.funcs.borrow_mut().insert("substring-after".to_string(),
      Function{
        name: "substring-after".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_substringafter)
      }
    );
    sc.funcs.borrow_mut().insert("normalize-space".to_string(),
      Function{
        name: "normalize-space".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_normalizespace)
      }
    );
    sc.funcs.borrow_mut().insert("translate".to_string(),
      Function{
        name: "translate".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_translate)
      }
    );
    sc.funcs.borrow_mut().insert("boolean".to_string(),
      Function{
        name: "boolean".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_boolean)
      }
    );
    sc.funcs.borrow_mut().insert("not".to_string(),
      Function{
        name: "not".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_not)
      }
    );
    sc.funcs.borrow_mut().insert("true".to_string(),
      Function{
        name: "true".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_true)
      }
    );
    sc.funcs.borrow_mut().insert("false".to_string(),
      Function{
        name: "false".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_false)
      }
    );
    sc.funcs.borrow_mut().insert("number".to_string(),
      Function{
        name: "number".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_number)
      }
    );
    sc.funcs.borrow_mut().insert("sum".to_string(),
      Function{
        name: "sum".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_sum)
      }
    );
    sc.funcs.borrow_mut().insert("floor".to_string(),
      Function{
        name: "floor".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_floor)
      }
    );
    sc.funcs.borrow_mut().insert("ceiling".to_string(),
      Function{
        name: "ceiling".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_ceiling)
      }
    );
    sc.funcs.borrow_mut().insert("round".to_string(),
      Function{
        name: "round".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_round)
      }
    );

    sc
  }
  /// Create a new StaticContext with builtin functions defined,
  /// including additional functions defined by XSLT.
  pub fn new_with_xslt_builtins() -> StaticContext {
    let sc = StaticContext::new_with_builtins();

    sc.funcs.borrow_mut().insert("current-grouping-key".to_string(),
      Function{
        name: "current-grouping-key".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_current_grouping_key)
      }
    );
    sc.funcs.borrow_mut().insert("current-group".to_string(),
      Function{
        name: "current-group".to_string(),
	nsuri: None,
	prefix: None,
	params: vec![],
	body: Some(func_current_group)
      }
    );

    sc
  }
  /// Register an extension function
  pub fn extension_function(&mut self, name: String, _ns: String, f: Function) {
    // TODO: namespace
    self.funcs.borrow_mut().insert(name, f);
  }
  /// Declares a function in the static context. The first argument is the name of the function. The second argument is the namespace URI (not currently supported). The third argument defines the arity of the function, and the types of each parameter (not currently supported).
  pub fn declare_function(&self, n: String, _ns: String, p: Vec<Param>) {
    self.funcs.borrow_mut().insert(n.clone(), Function{name: n, nsuri: None, prefix: None, body: None, params: p});
  }
  /// Declares a variable in the static context. The first argument is the name of the variable. The second argument is the namespace URI (not currently supported).
  pub fn declare_variable(&self, n: String, _ns:String) {
    self.vars.borrow_mut().insert(n.clone(), vec![]);
  }
}

/// Perform static analysis of a sequence constructor.
///
/// This checks that functions and variables are declared. It also rewrites the constructors to provide the implementation of functions that are used in expressions.
pub fn static_analysis(e: &mut Vec<Constructor>, sc: &StaticContext) {
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
      Constructor::SetAttribute(_, v) => {
        static_analysis(v, sc);
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
      Constructor::ApplyTemplates(s) => {
	static_analysis(s, sc)
      }
      Constructor::ForEach(s, t, _g) => {
	static_analysis(s, sc);
	static_analysis(t, sc);
      }
      Constructor::Copy(_, c) |
      Constructor::LiteralElement(_, c) => {
	static_analysis(c, sc)
      }
      Constructor::DeepCopy(c) => {
	static_analysis(c, sc);
      }
      Constructor::Literal(_) |
      Constructor::LiteralAttribute(_, _) |
      Constructor::ContextItem |
      Constructor::Root |
      Constructor::NotImplemented(_) => {}
    }
  }
}

// Functions

pub type FunctionImpl = fn(
    &DynamicContext,
    Option<Sequence>,		// Context
    Option<usize>,		// Context position
    Vec<Sequence>,		// Actual parameters
  ) -> Result<Sequence, Error>;

#[derive(Clone)]
pub struct Function {
  name: String,
  nsuri: Option<String>,
  prefix: Option<String>,
  params: Vec<Param>,	// The number of parameters in the vector is the arity of the function
  body: Option<FunctionImpl>,	// Function implementation must be provided during static analysis
}

impl Function {
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
  pub fn new(n: String, t: String) -> Param {
    Param{name: n, datatype: t}
  }
}

fn func_position(_: &DynamicContext, _ctxt: Option<Sequence>, posn: Option<usize>, _args: Vec<Sequence>) -> Result<Sequence, Error> {
  match posn {
    Some(u) => {
      Ok(vec![Rc::new(Item::Value(Value::Integer(u as i64 + 1)))])
    }
    None => Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: String::from("no context item"),})
  }
}

fn func_last(_: &DynamicContext, ctxt: Option<Sequence>, _posn: Option<usize>, _args: Vec<Sequence>) -> Result<Sequence, Error> {
  match ctxt {
    Some(u) => {
      Ok(vec![Rc::new(Item::Value(Value::Integer(u.len() as i64)))])
    }
    None => Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: String::from("no context item"),})
  }
}

pub fn func_count(_: &DynamicContext, ctxt: Option<Sequence>, _posn: Option<usize>, args: Vec<Sequence>) -> Result<Sequence, Error> {
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

pub fn func_localname(_: &DynamicContext, ctxt: Option<Sequence>, posn: Option<usize>, _args: Vec<Sequence>) -> Result<Sequence, Error> {
  match ctxt {
    Some(u) => {
      // Current item must be a node
      match *u[posn.unwrap()] {
        Item::Node(ref n) => {
      	  Ok(vec![Rc::new(Item::Value(Value::String(n.to_name().get_localname())))])
	}
	_ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("not a node"),})
      }
    }
    None => Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: String::from("no context item"),})
  }
}

// TODO: handle qualified names
pub fn func_name(_: &DynamicContext, ctxt: Option<Sequence>, posn: Option<usize>, _args: Vec<Sequence>) -> Result<Sequence, Error> {
  match ctxt {
    Some(u) => {
      // Current item must be a node
      match *u[posn.unwrap()] {
        Item::Node(ref n) => {
      	  // TODO: handle QName prefixes
	  Ok(vec![Rc::new(Item::Value(Value::String(n.to_name().get_localname())))])
	}
	_ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("not a node"),})
      }
    }
    None => Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: String::from("no context item"),})
  }
}

// TODO: implement string value properly
pub fn func_string(_: &DynamicContext, _ctxt: Option<Sequence>, _posn: Option<usize>, args: Vec<Sequence>) -> Result<Sequence, Error> {
  match args.len() {
    1 => {
      // return string value
      Ok(vec![Rc::new(Item::Value(Value::String(args[0].to_string())))])
    }
    _ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("wrong number of arguments"),})
  }
}

pub fn func_concat(_: &DynamicContext, _ctxt: Option<Sequence>, _posn: Option<usize>, args: Vec<Sequence>) -> Result<Sequence, Error> {
  Ok(vec![Rc::new(Item::Value(Value::String(
    args.iter().fold(
      String::new(),
      |mut a, b| {
        a.push_str(b.to_string().as_str());
	a
      }
    )
  )))])
}

pub fn func_startswith(_: &DynamicContext, _ctxt: Option<Sequence>, _posn: Option<usize>, args: Vec<Sequence>) -> Result<Sequence, Error> {
  // must have exactly 2 arguments
  if args.len() == 2 {
     // arg[0] is the string to search
     // arg[1] is what to search for
     Ok(vec![Rc::new(Item::Value(Value::Boolean(
       args[0].to_string().starts_with(args[1].to_string().as_str())
    )))])
  } else {
    Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("wrong number of arguments"),})
  }
}

pub fn func_contains(_: &DynamicContext, _ctxt: Option<Sequence>, _posn: Option<usize>, args: Vec<Sequence>) -> Result<Sequence, Error> {
  // must have exactly 2 arguments
  if args.len() == 2 {
     // arg[0] is the string to search
     // arg[1] is what to search for
     Ok(vec![Rc::new(Item::Value(Value::Boolean(
       args[0].to_string().contains(args[1].to_string().as_str())
    )))])
  } else {
    Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("wrong number of arguments"),})
  }
}

pub fn func_substring(_: &DynamicContext, _ctxt: Option<Sequence>, _posn: Option<usize>, args: Vec<Sequence>) -> Result<Sequence, Error> {
  // must have 2 or 3 arguments
  match args.len() {
    2 => {
     // arg[0] is the string to search
     // arg[1] is the index to start at
     // 2-argument version takes the rest of the string
     Ok(vec![Rc::new(Item::Value(Value::String(
       args[0].to_string().graphemes(true).skip(args[1].to_int().expect("not an integer") as usize - 1).collect()
     )))])
    }
    3 => {
     // arg[0] is the string to search
     // arg[1] is the index to start at
     // arg[2] is the length of the substring to extract
     Ok(vec![Rc::new(Item::Value(Value::String(
       args[0].to_string().graphemes(true).skip(args[1].to_int().expect("not an integer") as usize - 1).take(args[2].to_int().expect("not an integer") as usize).collect()
     )))])
    }
    _ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("wrong number of arguments"),})
  }
}

pub fn func_substringbefore(_: &DynamicContext, _ctxt: Option<Sequence>, _posn: Option<usize>, args: Vec<Sequence>) -> Result<Sequence, Error> {
  // must have 2 arguments
  match args.len() {
    2 => {
     // arg[0] is the string to search
     // arg[1] is the string to find
     match args[0].to_string().find(args[1].to_string().as_str()) {
       Some(i) => {
         match args[0].to_string().get(0..i) {
	   Some(s) => {
     	     Ok(vec![Rc::new(Item::Value(Value::String(
	       String::from(s)
     	     )))])
	   }
	   None => {
	     // This shouldn't happen!
	     Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("unable to extract substring"),})
	   }
	 }
       }
       None => {
         Ok(vec![])
       }
     }
    }
    _ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("wrong number of arguments"),})
  }
}

pub fn func_substringafter(_: &DynamicContext, _ctxt: Option<Sequence>, _posn: Option<usize>, args: Vec<Sequence>) -> Result<Sequence, Error> {
  // must have 2 arguments
  match args.len() {
    2 => {
     // arg[0] is the string to search
     // arg[1] is the string to find
     match args[0].to_string().find(args[1].to_string().as_str()) {
       Some(i) => {
         match args[0].to_string().get(i + args[1].to_string().len()..args[0].to_string().len()) {
	   Some(s) => {
     	     Ok(vec![Rc::new(Item::Value(Value::String(
	       String::from(s)
     	     )))])
	   }
	   None => {
	     // This shouldn't happen!
	     Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("unable to extract substring"),})
	   }
	 }
       }
       None => {
         Ok(vec![])
       }
     }
    }
    _ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("wrong number of arguments"),})
  }
}

pub fn func_normalizespace(_: &DynamicContext, ctxt: Option<Sequence>, posn: Option<usize>, args: Vec<Sequence>) -> Result<Sequence, Error> {
  // must have 0 or 1 arguments
  let s: Result<Option<String>, Error> = match args.len() {
    0 => {
      // Use the current item
      match ctxt {
        Some(c) => {
	  Ok(Some(c[posn.unwrap()].to_string()))
	}
	None => Ok(None)
      }
    }
    1 => {
      Ok(Some(args[0].to_string()))
    }
    _ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("wrong number of arguments"),})
  };

  match s {
    Ok(u) => {
      match u {
        Some(t) => {
          Ok(vec![Rc::new(Item::Value(Value::String(
            t.split_whitespace().collect()
          )))])
        }
        None => Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: String::from("no context item"),})
      }
    }
    Result::Err(e) => {
      Result::Err(e)
    }
  }
}

pub fn func_translate(_: &DynamicContext, _ctxt: Option<Sequence>, _posn: Option<usize>, args: Vec<Sequence>) -> Result<Sequence, Error> {
  // must have 3 arguments
  match args.len() {
    3 => {
      // arg[0] is the string to search
      // arg[1] is the map chars
      // arg[2] is the translate chars
      let o = args[1].to_string();
      let m: Vec<&str> = o.graphemes(true).collect();
      let u = args[2].to_string();
      let t: Vec<&str> = u.graphemes(true).collect();
      let mut result: String = String::new();

      for c in args[0].to_string().graphemes(true) {
	let mut a: Option<Option<usize>> = Some(None);
        for i in 0..m.len() {
	  if c == m[i] {
	    if i < t.len() {
	      a = Some(Some(i));
	      break
            } else {
              // omit this character
	      a = None
            }
	  } else {
	    // keep looking for a match
	  }
        }
	match a {
	  Some(None) => {
	    result.push_str(c);
	  }
	  Some(Some(j)) => {
	    result.push_str(t[j])
	  }
	  None => {
	    // omit char
	  }
	}
      }
      Ok(vec![Rc::new(Item::Value(Value::String(result)))])
    }
    _ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("wrong number of arguments"),})
  }
}

pub fn func_boolean(_: &DynamicContext, _ctxt: Option<Sequence>, _posn: Option<usize>, args: Vec<Sequence>) -> Result<Sequence, Error> {
  // must have 1 arguments
  match args.len() {
    1 => {
      Ok(vec![Rc::new(Item::Value(Value::Boolean(args[0].to_bool())))])
    }
    _ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("wrong number of arguments"),})
  }
}

pub fn func_not(_: &DynamicContext, _ctxt: Option<Sequence>, _posn: Option<usize>, args: Vec<Sequence>) -> Result<Sequence, Error> {
  // must have 1 arguments
  match args.len() {
    1 => {
      Ok(vec![Rc::new(Item::Value(Value::Boolean(!args[0].to_bool())))])
    }
    _ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("wrong number of arguments"),})
  }
}

pub fn func_true(_: &DynamicContext, _ctxt: Option<Sequence>, _posn: Option<usize>, args: Vec<Sequence>) -> Result<Sequence, Error> {
  // must have 0 arguments
  match args.len() {
    0 => {
      Ok(vec![Rc::new(Item::Value(Value::Boolean(true)))])
    }
    _ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("wrong number of arguments"),})
  }
}

pub fn func_false(_: &DynamicContext, _ctxt: Option<Sequence>, _posn: Option<usize>, args: Vec<Sequence>) -> Result<Sequence, Error> {
  // must have 0 arguments
  match args.len() {
    0 => {
      Ok(vec![Rc::new(Item::Value(Value::Boolean(false)))])
    }
    _ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("wrong number of arguments"),})
  }
}

pub fn func_number(_: &DynamicContext, _ctxt: Option<Sequence>, _posn: Option<usize>, args: Vec<Sequence>) -> Result<Sequence, Error> {
  // must have 1 argument
  match args.len() {
    1 => {
      match args[0].len() {
        1 => {
	  // TODO: if the item is already an integer, then just clone it
      	  // First try converting to an integer
	  match args[0][0].to_int() {
	    Ok(i) => {
      	      Ok(vec![Rc::new(Item::Value(Value::Integer(i)))])
	    }
	    Result::Err(_) => {
      	      // If that fails, convert to double
	      // NB. this can't fail. At worst it returns NaN
      	      Ok(vec![Rc::new(Item::Value(Value::Double(args[0][0].to_double())))])
	    }
	  }
	}
	_ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("not a singleton sequence"),})
      }
    }
    _ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("wrong number of arguments"),})
  }
}

pub fn func_sum(_: &DynamicContext, _ctxt: Option<Sequence>, _posn: Option<usize>, args: Vec<Sequence>) -> Result<Sequence, Error> {
  // must have 1 argument
  match args.len() {
    1 => {
      Ok(vec![Rc::new(Item::Value(Value::Double(args[0].iter().fold(0.0, |mut acc, i| {acc += i.to_double(); acc}))))])
    }
    _ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("wrong number of arguments"),})
  }
}

pub fn func_floor(_: &DynamicContext, _ctxt: Option<Sequence>, _posn: Option<usize>, args: Vec<Sequence>) -> Result<Sequence, Error> {
  // must have 1 argument which is a singleton
  match args.len() {
    1 => {
      match args[0].len() {
        1 => {
      	  Ok(vec![Rc::new(Item::Value(Value::Double(args[0][0].to_double().floor())))])
	}
	_ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("not a singleton sequence"),})
      }
    }
    _ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("wrong number of arguments"),})
  }
}

pub fn func_ceiling(_: &DynamicContext, _ctxt: Option<Sequence>, _posn: Option<usize>, args: Vec<Sequence>) -> Result<Sequence, Error> {
  // must have 1 argument which is a singleton
  match args.len() {
    1 => {
      match args[0].len() {
        1 => {
      	  Ok(vec![Rc::new(Item::Value(Value::Double(args[0][0].to_double().ceil())))])
	}
	_ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("not a singleton sequence"),})
      }
    }
    _ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("wrong number of arguments"),})
  }
}

pub fn func_round(_: &DynamicContext, _ctxt: Option<Sequence>, _posn: Option<usize>, args: Vec<Sequence>) -> Result<Sequence, Error> {
  // must have 1 or 2 arguments
  match args.len() {
    1 => {
      // precision is 0 (i.e. round to nearest whole number
      match args[0].len() {
        1 => {
      	  Ok(vec![Rc::new(Item::Value(Value::Double(args[0][0].to_double().round())))])
	}
	_ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("not a singleton sequence"),})
      }
    }
    2 => {
      match (args[0].len(), args[1].len()) {
        (1, 1) => {
      	  Ok(vec![Rc::new(Item::Value(Value::Double(args[0][0].to_double().powi(args[1][0].to_int().unwrap() as i32).round().powi(-1 * args[1][0].to_int().unwrap() as i32))))])
	}
	_ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("not a singleton sequence"),})
      }
    }
    _ => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("wrong number of arguments"),})
  }
}

pub fn func_current_grouping_key(dc: &DynamicContext, _ctxt: Option<Sequence>, _posn: Option<usize>, _args: Vec<Sequence>) -> Result<Sequence, Error> {
  match dc.current_grouping_key.borrow().last() {
    Some(k) => {
      match k {
        Some(l) => Ok(vec![l.clone()]),
	None => Ok(vec![]),
      }
    }
    None => {
      Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: String::from("no current grouping key"),})
    }
  }
}

pub fn func_current_group(dc: &DynamicContext, _ctxt: Option<Sequence>, _posn: Option<usize>, _args: Vec<Sequence>) -> Result<Sequence, Error> {
  match dc.current_group.borrow().last() {
    Some(k) => {
      match k {
        Some(l) => Ok(l.clone()),
	None => Ok(vec![]),
      }
    }
    None => {
      Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: String::from("no current group"),})
    }
  }
}

// Operands must be singletons
fn value_comparison(dc: &DynamicContext, ctxt: Option<Sequence>, posn: Option<usize>, op: Operator, left: &Vec<Constructor>, right: &Vec<Constructor>) -> Result<bool, Error> {
  let left_seq = evaluate(dc, ctxt.clone(), posn, left).expect("evaluating left-hand sequence failed");
  if left_seq.len() == 0 {
    return Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("left-hand sequence is empty"),})
  }
  if left_seq.len() == 1 {
    let right_seq = evaluate(dc, ctxt.clone(), posn, right).expect("evaluating right-hand sequence failed");
    if right_seq.len() == 1 {
      Ok(left_seq[0].compare(&*right_seq[0], op).unwrap())
    } else {
      Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("right-hand sequence is not a singleton sequence"),})
    }
  } else {
    Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("left-hand sequence is not a singleton sequence"),})
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
      Constructor::LiteralAttribute(qn, v) => {
        format!("{:in$} Construct literal attribute \"{}\" with value \"{}\"", "",
	  qn.get_localname(),
	  format_constructor(&v, i + 4),
	  in=i)
      }
      Constructor::LiteralElement(qn, c) => {
        format!("{:in$} Construct literal element \"{}\" with content:\n{}", "", qn.get_localname(),
	  format_constructor(&c, i + 4),
	  in=i)
      }
      Constructor::Copy(_sel, c) => {
        format!("{:in$} Construct copy with content:\n{}", "",
	  format_constructor(&c, i + 4),
	  in=i)
      }
      Constructor::DeepCopy(c) => {
        format!("{:in$} Construct deep copy with content:\n{}", "",
	  format_constructor(&c, i + 4),
	  in=i)
      }
      Constructor::ContextItem => {
        format!("{:in$} Construct context item", "", in=i)
      }
      Constructor::SetAttribute(qn, v) => {
        format!("{:in$} Construct set attribute named \"{}\":\n{}", "",
	  qn.get_localname(),
	  format_constructor(&v, i + 4),
	  in=i)
      }
      Constructor::Or(v) => {
        format!(
	  "{:in$} Construct OR of:\n{}\n{}", "",
	  format_constructor(&v[0], i + 4),
	  format_constructor(&v[1], i + 4),
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
      Constructor::ApplyTemplates(_) => {
        format!("{:in$} apply-templates constructor", "", in=i)
      }
      Constructor::ForEach(_, _, _) => {
        format!("{:in$} for-each constructor", "", in=i)
      }
      Constructor::NotImplemented(m) => {
        format!("{:in$} NotImplemented constructor: {}", "", m, in=i)
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
      let dc = DynamicContext::new(None);
      let cons = vec![Constructor::Literal(Value::String("foobar".to_string()))];
      let s = evaluate(&dc, None, None, &cons)
	.expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s.to_string(), "foobar")
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn literal_int() {
      let dc = DynamicContext::new(None);
      let cons = vec![Constructor::Literal(Value::Integer(456))];
      let s = evaluate(&dc, None, None, &cons)
	.expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s[0].to_int().unwrap(), 456)
      } else {
        panic!("sequence is not a singleton")
      }
    }

//    #[test]
//    fn literal_decimal() {
//      let dc = DynamicContext::new(None);
//      let cons = vec![Constructor::Literal(Value::Decimal(d128!(34.56)))];
//      let s = evaluate(&dc, None, None, &cons)
//        .expect("evaluation failed");
//      if s.len() == 1 {
//        assert_eq!(s.to_string(), "34.56")
//      } else {
//        panic!("sequence is not a singleton")
//      }
//    }

    #[test]
    fn literal_bool() {
      let dc = DynamicContext::new(None);
      let cons = vec![Constructor::Literal(Value::Boolean(false))];
      let s = evaluate(&dc, None, None, &cons)
        .expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s.to_bool(), false)
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn literal_double() {
      let dc = DynamicContext::new(None);
      let cons = vec![Constructor::Literal(Value::Double(4.56))];
      let s = evaluate(&dc, None, None, &cons)
        .expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s[0].to_double(), 4.56)
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn sequence_literal() {
      let dc = DynamicContext::new(None);
      let cons = vec![
	  Constructor::Literal(Value::String("foo".to_string())),
	  Constructor::Literal(Value::String("bar".to_string())),
	];
      let s = evaluate(&dc, None, None, &cons)
        .expect("evaluation failed");
      if s.len() == 2 {
        assert_eq!(s.to_string(), "foobar")
      } else {
        panic!("sequence does not have two items")
      }
    }

    #[test]
    fn sequence_literal_mixed() {
      let dc = DynamicContext::new(None);
      let cons = vec![
	  Constructor::Literal(Value::String("foo".to_string())),
	  Constructor::Literal(Value::Integer(123)),
	];
      let s = evaluate(&dc, None, None, &cons)
        .expect("evaluation failed");
      if s.len() == 2 {
        assert_eq!(s.to_string(), "foo123")
      } else {
        panic!("sequence does not have two items")
      }
    }

    #[test]
    fn context_item() {
      let dc = DynamicContext::new(None);
      let s = vec![Rc::new(Item::Value(Value::String("foobar".to_string())))];
      let cons = vec![Constructor::ContextItem];
      let result = evaluate(&dc, Some(s), Some(0), &cons)
        .expect("evaluation failed");
      if result.len() == 1 {
        assert_eq!(result[0].to_string(), "foobar")
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn context_item_2() {
      let dc = DynamicContext::new(None);
      let cons = vec![
	  Constructor::ContextItem,
	  Constructor::ContextItem,
	];
      let result = evaluate(&dc, Some(vec![Rc::new(Item::Value(Value::String("foobar".to_string())))]), Some(0), &cons)
        .expect("evaluation failed");
      if result.len() == 2 {
        assert_eq!(result.to_string(), "foobarfoobar")
      } else {
        panic!("sequence does not have two items")
      }
    }

    #[test]
    fn or() {
      let dc = DynamicContext::new(None);
      let cons = vec![
	  Constructor::Or(
	    vec![
	      vec![Constructor::Literal(Value::Boolean(true))],
	      vec![Constructor::Literal(Value::Boolean(false))],
	    ]
	  )
	];
      let s = evaluate(&dc, None, None, &cons)
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
      let dc = DynamicContext::new(None);
      let cons = vec![
	  Constructor::And(
	    vec![
	      vec![Constructor::Literal(Value::Boolean(true))],
	      vec![Constructor::Literal(Value::Boolean(false))],
	    ]
	  )
	];
      let s = evaluate(&dc, None, None, &cons)
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
      let dc = DynamicContext::new(None);
      let cons = vec![
	  Constructor::ValueComparison(
	    Operator::Equal,
	    vec![
	      vec![Constructor::Literal(Value::Integer(1))],
	      vec![Constructor::Literal(Value::Integer(1))],
	    ]
	  )
	];
      let s = evaluate(&dc, None, None, &cons)
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
      let dc = DynamicContext::new(None);
      let cons = vec![
	  Constructor::ValueComparison(
	    Operator::Equal,
	    vec![
	      vec![Constructor::Literal(Value::Integer(1))],
	      vec![Constructor::Literal(Value::Integer(2))],
	    ]
	  )
	];
      let s = evaluate(&dc, None, None, &cons)
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
      let dc = DynamicContext::new(None);
      let cons = vec![
	  Constructor::ValueComparison(
	    Operator::Equal,
	    vec![
	      vec![Constructor::Literal(Value::String("foo".to_string()))],
	      vec![Constructor::Literal(Value::String("foo".to_string()))],
	    ]
	  )
	];
      let s = evaluate(&dc, None, None, &cons)
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
      let dc = DynamicContext::new(None);
      let cons = vec![
	  Constructor::ValueComparison(
	    Operator::Equal,
	    vec![
	      vec![Constructor::Literal(Value::String("foo".to_string()))],
	      vec![Constructor::Literal(Value::String("bar".to_string()))],
	    ]
	  )
	];
      let s = evaluate(&dc, None, None, &cons)
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
      let dc = DynamicContext::new(None);
      let cons = vec![
	  Constructor::GeneralComparison(
            Operator::Equal,
	    vec![
	      vec![Constructor::Literal(Value::String("foo".to_string()))],
	      vec![
	        Constructor::Literal(Value::String("bar".to_string())),
	        Constructor::Literal(Value::String("foo".to_string())),
	      ]
	    ]
	  )
	];
      let s = evaluate(&dc, None, None, &cons)
	.expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s.to_bool(), true)
      } else {
        panic!("sequence is not a singleton")
      }
    }
    #[test]
    fn general_comparison_string_false() {
      let dc = DynamicContext::new(None);
      let cons = vec![
	  Constructor::GeneralComparison(
            Operator::Equal,
	    vec![
	      vec![Constructor::Literal(Value::String("foo".to_string()))],
	      vec![
	        Constructor::Literal(Value::String("bar".to_string())),
	        Constructor::Literal(Value::String("oof".to_string())),
	      ]
	    ]
	  )
	];
      let s = evaluate(&dc, None, None, &cons)
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
      let dc = DynamicContext::new(None);
      let cons = vec![
	  Constructor::Concat(
	    vec![
	      vec![Constructor::Literal(Value::String("foo".to_string()))],
	      vec![
	        Constructor::Literal(Value::String("bar".to_string())),
	        Constructor::Literal(Value::String("oof".to_string())),
	      ]
	    ]
	  )
	];
      let s = evaluate(&dc, None, None, &cons)
	.expect("evaluation failed");
      if s.len() == 1 {
        assert_eq!(s.to_string(), "foobaroof")
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn range() {
      let dc = DynamicContext::new(None);
      let cons = vec![
	  Constructor::Range(
	    vec![
	      vec![Constructor::Literal(Value::Integer(0))],
	      vec![Constructor::Literal(Value::Integer(9))],
	    ]
	  )
	];
      let s = evaluate(&dc, None, None, &cons)
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
      let dc = DynamicContext::new(None);
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
      let s = evaluate(&dc, None, None, &cons)
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
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("position".to_string(), vec![], Some(func_position)),
	vec![]
      );
      let s = vec![
        Rc::new(Item::Value(Value::String("a".to_string()))),
        Rc::new(Item::Value(Value::String("b".to_string()))),
      ];
      let vc = vec![c];
      let r = evaluate(&dc, Some(s), Some(1), &vc).expect("evaluation failed");
      assert_eq!(r.to_string(), "2")
    }
    #[test]
    fn function_call_last() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("last".to_string(), vec![], Some(func_last)),
	vec![]
      );
      let s = vec![
        Rc::new(Item::Value(Value::String("a".to_string()))),
        Rc::new(Item::Value(Value::String("b".to_string()))),
        Rc::new(Item::Value(Value::String("c".to_string()))),
      ];
      let vc = vec![c];
      let r = evaluate(&dc, Some(s), Some(1), &vc).expect("evaluation failed");
      assert_eq!(r.to_string(), "3")
    }
    #[test]
    fn function_call_count() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new(
	  "count".to_string(),
	  vec![Param::new("i".to_string(), "t".to_string())],
	  Some(func_count)
	),
	vec![
	  vec![
            Constructor::Literal(Value::String("a".to_string())),
            Constructor::Literal(Value::String("b".to_string())),
            Constructor::Literal(Value::String("c".to_string())),
	  ]
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.to_string(), "3")
    }
    #[test]
    fn function_call_string_1() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("string".to_string(), vec![], Some(func_string)),
	vec![
	  vec![
            Constructor::Literal(Value::String("a".to_string())),
            Constructor::Literal(Value::String("b".to_string())),
            Constructor::Literal(Value::String("c".to_string())),
	  ]
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.to_string(), "abc")
    }
    #[test]
    fn function_call_concat_1() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("concat".to_string(), vec![], Some(func_concat)),
	vec![
	  vec![Constructor::Literal(Value::String("a".to_string()))],
          vec![Constructor::Literal(Value::String("b".to_string()))],
          vec![Constructor::Literal(Value::String("c".to_string()))],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.to_string(), "abc")
    }
    #[test]
    fn function_call_startswith_pos() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("starts-with".to_string(), vec![], Some(func_startswith)),
	vec![
	  vec![Constructor::Literal(Value::String("abc".to_string()))],
          vec![Constructor::Literal(Value::String("a".to_string()))],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.to_bool(), true)
    }
    #[test]
    fn function_call_startswith_neg() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("starts-with".to_string(), vec![], Some(func_startswith)),
	vec![
	  vec![Constructor::Literal(Value::String("abc".to_string()))],
          vec![Constructor::Literal(Value::String("b".to_string()))],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.to_bool(), false)
    }
    #[test]
    fn function_call_contains_pos() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("contains".to_string(), vec![], Some(func_contains)),
	vec![
	  vec![Constructor::Literal(Value::String("abc".to_string()))],
          vec![Constructor::Literal(Value::String("b".to_string()))],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.to_bool(), true)
    }
    #[test]
    fn function_call_contains_neg() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("contains".to_string(), vec![], Some(func_contains)),
	vec![
	  vec![Constructor::Literal(Value::String("abc".to_string()))],
          vec![Constructor::Literal(Value::String("d".to_string()))],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.to_bool(), false)
    }
    #[test]
    fn function_call_substring_2() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("substring".to_string(), vec![], Some(func_substring)),
	vec![
	  vec![Constructor::Literal(Value::String("abc".to_string()))],
          vec![Constructor::Literal(Value::Integer(2))],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.to_string(), "bc")
    }
    #[test]
    fn function_call_substring_3() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("substring".to_string(), vec![], Some(func_substring)),
	vec![
	  vec![Constructor::Literal(Value::String("abcde".to_string()))],
          vec![Constructor::Literal(Value::Integer(2))],
          vec![Constructor::Literal(Value::Integer(3))],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.to_string(), "bcd")
    }
    #[test]
    fn function_call_substring_before_1() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("substring-before".to_string(), vec![], Some(func_substringbefore)),
	vec![
	  vec![Constructor::Literal(Value::String("abcde".to_string()))],
          vec![Constructor::Literal(Value::String("bc".to_string()))],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.to_string(), "a")
    }
    #[test]
    fn function_call_substring_before_neg() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("substring-before".to_string(), vec![], Some(func_substringbefore)),
	vec![
	  vec![Constructor::Literal(Value::String("abcde".to_string()))],
          vec![Constructor::Literal(Value::String("fg".to_string()))],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.to_string(), "")
    }
    #[test]
    fn function_call_substring_after_1() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("substring-after".to_string(), vec![], Some(func_substringafter)),
	vec![
	  vec![Constructor::Literal(Value::String("abcde".to_string()))],
          vec![Constructor::Literal(Value::String("bc".to_string()))],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.to_string(), "de")
    }
    #[test]
    fn function_call_substring_after_neg_1() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("substring-after".to_string(), vec![], Some(func_substringafter)),
	vec![
	  vec![Constructor::Literal(Value::String("abcde".to_string()))],
          vec![Constructor::Literal(Value::String("fg".to_string()))],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.to_string(), "")
    }
    #[test]
    fn function_call_substring_after_neg_2() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("substring-after".to_string(), vec![], Some(func_substringafter)),
	vec![
	  vec![Constructor::Literal(Value::String("abcde".to_string()))],
          vec![Constructor::Literal(Value::String("de".to_string()))],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.to_string(), "")
    }
    #[test]
    fn function_call_normalizespace() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("normalize-space".to_string(), vec![], Some(func_normalizespace)),
	vec![
	  vec![Constructor::Literal(Value::String("	a b   c\nd e 	".to_string()))],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.to_string(), "abcde")
    }
    #[test]
    fn function_call_translate() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("translate".to_string(), vec![], Some(func_translate)),
	vec![
	  vec![Constructor::Literal(Value::String("abcdeabcde".to_string()))],
	  vec![Constructor::Literal(Value::String("ade".to_string()))],
	  vec![Constructor::Literal(Value::String("XY".to_string()))],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.to_string(), "XbcYXbcY")
    }
    // TODO: test using non-ASCII characters
    #[test]
    fn function_call_boolean_true() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("boolean".to_string(), vec![], Some(func_boolean)),
	vec![
	  vec![Constructor::Literal(Value::String("abcdeabcde".to_string()))],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      match *r[0] {
        Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
	_ => panic!("not a singleton boolean true value")
      }
    }
    #[test]
    fn function_call_boolean_false() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("boolean".to_string(), vec![], Some(func_boolean)),
	vec![
	  vec![],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      match *r[0] {
        Item::Value(Value::Boolean(b)) => assert_eq!(b, false),
	_ => panic!("not a singleton boolean false value")
      }
    }
    #[test]
    fn function_call_not_false() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("not".to_string(), vec![], Some(func_not)),
	vec![
	  vec![Constructor::Literal(Value::Boolean(true))],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      match *r[0] {
        Item::Value(Value::Boolean(b)) => assert_eq!(b, false),
	_ => panic!("not a singleton boolean false value")
      }
    }
    #[test]
    fn function_call_not_true() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("not".to_string(), vec![], Some(func_not)),
	vec![
	  vec![],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      match *r[0] {
        Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
	_ => panic!("not a singleton boolean true value")
      }
    }
    #[test]
    fn function_call_true() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("true".to_string(), vec![], Some(func_true)),
	vec![
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      match *r[0] {
        Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
	_ => panic!("not a singleton boolean true value")
      }
    }
    #[test]
    fn function_call_false() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("false".to_string(), vec![], Some(func_false)),
	vec![
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      match *r[0] {
        Item::Value(Value::Boolean(b)) => assert_eq!(b, false),
	_ => panic!("not a singleton boolean false value")
      }
    }
    #[test]
    fn function_call_number_int() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("number".to_string(), vec![], Some(func_number)),
	vec![
	  vec![Constructor::Literal(Value::String("123".to_string()))]
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      match *r[0] {
        Item::Value(Value::Integer(i)) => assert_eq!(i, 123),
	_ => panic!("not a singleton integer value")
      }
    }
    #[test]
    fn function_call_number_double() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("number".to_string(), vec![], Some(func_number)),
	vec![
	  vec![Constructor::Literal(Value::String("123.456".to_string()))]
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      match *r[0] {
        Item::Value(Value::Double(d)) => assert_eq!(d, 123.456),
	_ => panic!("not a singleton double value")
      }
    }
    // TODO: test NaN result
    #[test]
    fn function_call_sum() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("sum".to_string(), vec![], Some(func_sum)),
	vec![
	    vec![Constructor::Literal(Value::String("123.456".to_string())),
	         Constructor::Literal(Value::String("10".to_string())),
	         Constructor::Literal(Value::String("-20".to_string())),
	         Constructor::Literal(Value::String("0".to_string())),
	    ],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      match *r[0] {
        Item::Value(Value::Double(d)) => assert_eq!(d, 123.456 + 10.0 - 20.0),
	_ => panic!("not a singleton double value")
      }
    }
    #[test]
    fn function_call_floor() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("floor".to_string(), vec![], Some(func_floor)),
	vec![
	  vec![Constructor::Literal(Value::String("123.456".to_string()))],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      match *r[0] {
        Item::Value(Value::Double(d)) => assert_eq!(d, 123.0),
	_ => panic!("not a singleton double value")
      }
    }
    #[test]
    fn function_call_ceiling() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("ceiling".to_string(), vec![], Some(func_ceiling)),
	vec![
	  vec![Constructor::Literal(Value::String("123.456".to_string()))],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      match *r[0] {
        Item::Value(Value::Double(d)) => assert_eq!(d, 124.0),
	_ => panic!("not a singleton double value")
      }
    }
    #[test]
    fn function_call_round_down() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("round".to_string(), vec![], Some(func_round)),
	vec![
	  vec![Constructor::Literal(Value::String("123.456".to_string()))],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      match *r[0] {
        Item::Value(Value::Double(d)) => assert_eq!(d, 123.0),
	_ => panic!("not a singleton double value")
      }
    }
    #[test]
    fn function_call_round_up() {
      let dc = DynamicContext::new(None);
      let c = Constructor::FunctionCall(
        Function::new("round".to_string(), vec![], Some(func_round)),
	vec![
	  vec![Constructor::Literal(Value::String("123.654".to_string()))],
        ]
      );
      let vc = vec![c];
      let r = evaluate(&dc, None, None, &vc).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      match *r[0] {
        Item::Value(Value::Double(d)) => assert_eq!(d, 124.0),
	_ => panic!("not a singleton double value")
      }
    }

    // Variables
    #[test]
    fn var_ref() {
      let dc = DynamicContext::new(None);
      let c = vec![
        Constructor::VariableDeclaration("foo".to_string(), vec![Constructor::Literal(Value::String("my variable".to_string()))]),
	Constructor::VariableReference("foo".to_string()),
      ];
      let r = evaluate(&dc, None, None, &c).expect("evaluation failed");
      assert_eq!(r.to_string(), "my variable")
    }

    // Loops
    #[test]
    fn loop_1() {
      let dc = DynamicContext::new(None);
      // This is "for $x in ('a', 'b', 'c') return $x"
      let c = vec![
        Constructor::Loop(
	  vec![Constructor::VariableDeclaration(
	    "x".to_string(),
	    vec![
	      Constructor::Literal(Value::String("a".to_string())),
	      Constructor::Literal(Value::String("b".to_string())),
	      Constructor::Literal(Value::String("c".to_string())),
	    ]
	  )],
	  vec![Constructor::VariableReference("x".to_string())]
	)
      ];
      let r = evaluate(&dc, None, None, &c).expect("evaluation failed");
      assert_eq!(r.len(), 3);
      assert_eq!(r.to_string(), "abc")
    }

    // Switch
    #[test]
    fn switch_1() {
      let dc = DynamicContext::new(None);
      // implements "if (1) then 'one' else 'not one'"
      let c = vec![
        Constructor::Switch(
	  vec![
	    vec![
	      Constructor::Literal(Value::Integer(1))
	    ],
	    vec![
	      Constructor::Literal(Value::String("one".to_string()))
	    ]
	  ],
	  vec![Constructor::Literal(Value::String("not one".to_string()))]
	)
      ];
      let r = evaluate(&dc, None, None, &c).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      assert_eq!(r.to_string(), "one")
    }
    #[test]
    fn switch_2() {
      let dc = DynamicContext::new(None);
      // implements "if (0) then 'one' else 'not one'"
      let c = vec![
        Constructor::Switch(
	  vec![
	    vec![
	      Constructor::Literal(Value::Integer(0))
	    ],
	    vec![
	      Constructor::Literal(Value::String("one".to_string()))
	    ]
	  ],
	  vec![Constructor::Literal(Value::String("not one".to_string()))]
	)
      ];
      let r = evaluate(&dc, None, None, &c).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      assert_eq!(r.to_string(), "not one")
    }    
    #[test]
    fn switch_3() {
      let dc = DynamicContext::new(None);
      let c = vec![
        Constructor::Switch(
	  vec![
	    vec![
	      Constructor::Literal(Value::Integer(0))
	    ],
	    vec![
	      Constructor::Literal(Value::String("one".to_string()))
	    ],
	    vec![
	      Constructor::Literal(Value::Integer(1))
	    ],
	    vec![
	      Constructor::Literal(Value::String("two".to_string()))
	    ],
	    vec![
	      Constructor::Literal(Value::Integer(0))
	    ],
	    vec![
	      Constructor::Literal(Value::String("three".to_string()))
	    ],
	  ],
	  vec![Constructor::Literal(Value::String("not any".to_string()))]
	)
      ];
      let r = evaluate(&dc, None, None, &c).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      assert_eq!(r.to_string(), "two")
    }
    // The first clause to pass should return the result
    #[test]
    fn switch_4() {
      let dc = DynamicContext::new(None);
      let c = vec![
        Constructor::Switch(
	  vec![
	    vec![
	      Constructor::Literal(Value::Integer(0))
	    ],
	    vec![
	      Constructor::Literal(Value::String("one".to_string()))
	    ],
	    vec![
	      Constructor::Literal(Value::Integer(1))
	    ],
	    vec![
	      Constructor::Literal(Value::String("two".to_string()))
	    ],
	    vec![
	      Constructor::Literal(Value::Integer(1))
	    ],
	    vec![
	      Constructor::Literal(Value::String("three".to_string()))
	    ],
	  ],
	  vec![Constructor::Literal(Value::String("not any".to_string()))]
	)
      ];
      let r = evaluate(&dc, None, None, &c).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      assert_eq!(r.to_string(), "two")
    }    

    // Patterns
    // Need a concrete type to test patterns

    // Templates
    // Need a concrete type to test patterns

    // Literal result element
    // Need a concrete type to test literal result elements

    // for-each, for-each-group
    // See libxml-evaluate test

}

