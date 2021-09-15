/// Document and Node implementation using XDMTree

use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;
use petgraph::graph::Graph;
use crate::xdmgraph::{XDMTree, XDMTreeNode, NodeType as TreeNodeType};
use crate::item::*;
use crate::xdmerror::*;

impl Document for XDMTree {
  fn to_string(&self) -> String {
    get_doc_node(self).to_string()
  }
  fn to_xml(&self) -> String {
    get_doc_node(self).to_xml()
  }
  fn to_json(&self) -> String {
    // TODO
    "".to_string()
  }
  fn children(&self) -> Vec<Rc<dyn Node>> {
    let mut it = get_doc_node(self).child_iter();
    let mut result: Vec<Rc<dyn Node>> = vec![];
    loop {
      match it.next() {
        Some(n) => {
	  result.push(Rc::new(n))
	}
	None => break,
      }
    }
    result
  }
  fn get_root_element(&self) -> Option<Rc<dyn Node>> {
    // TODO: handle prologue, epilogue
    let mut it = get_doc_node(self).child_iter();
    let mut result = vec![];
    loop {
      match it.next() {
        Some(n) => {
	  result.push(Rc::new(n))
	}
	None => break,
      }
    }
    if result.len() == 1 {
      Some(result[0].clone())
    } else {
      None
    }
  }
  fn new_element(&self, name: &str, _ns: Option<&str>) -> Result<Rc<dyn Node>, Error> {
    // TODO: namespaces
    Ok(Rc::new(get_doc_node(self).new_element(QualifiedName::new(None, None, name.to_string()))))
  }
  fn new_text(&self, c: &str) -> Result<Rc<dyn Node>, Error> {
    Ok(Rc::new(get_doc_node(self).new_value(Value::String(c.to_string()))))
  }
  fn set_root_element(&mut self, r: &dyn Any) -> Result<(), Error> {
    let n: &XDMTreeNode = match r.downcast_ref::<XDMTreeNode>() {
      Some(m) => m,
      None => return Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "root element must be a XDMTreeNode".to_string()}),
    };
    // TODO: If the document already has a root element then remove it
    get_doc_node(self).append_child(n.clone());
    Ok(())
  }
}

impl Node for XDMTreeNode {
  fn as_any(&self) -> &dyn Any {
    self
  }
  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn to_string(&self) -> String {
    match self.get_doc().borrow()[self.get_index()] {
      TreeNodeType::Document => {
	self.get_first_child()
	  .map_or("".to_string(), |n| n.to_string())
      }
      TreeNodeType::Element(_) => {
      	let mut result = String::new();
	self.child_iter().for_each(
          |c| {
	    result.push_str(c.to_string().as_str());
	  }
        );
	result
      }
      TreeNodeType::Text(ref t) => {
        t.to_string()
      }
    }
  }
  fn to_xml(&self) -> String {
    self.to_xml_int()
  }
  fn to_json(&self) -> String {
    // TODO
    "".to_string()
  }

  // A non-empty node is always true
  fn to_bool(&self) -> bool {
    true
  }
  fn to_int(&self) -> Result<i64, Error> {
    match self.to_string().parse::<i64>() {
      Ok(i) => Ok(i),
      Result::Err(e) => Result::Err(Error{kind: ErrorKind::Unknown, message: e.to_string()}),
    }
  }
  fn to_double(&self) -> f64 {
    match self.to_string().parse::<f64>() {
      Ok(f) => f,
      Result::Err(_) => f64::NAN
    }
  }

  fn to_name(&self) -> QualifiedName {
    match self.get_doc().borrow()[self.get_index()] {
      TreeNodeType::Document => {
        QualifiedName::new(None, None, "".to_string())
      }
      TreeNodeType::Element(ref e) => {
      	// TODO: namespaces
	e.name.clone()
      }
      TreeNodeType::Text(_) => {
        QualifiedName::new(None, None, "".to_string())
      }
    }
  }

  fn doc(&self) -> Option<Rc<dyn Document>> {
    Some(Rc::new(self.get_doc()))
  }

  fn node_type(&self) -> NodeType {
    match self.get_doc().borrow()[self.get_index()] {
      TreeNodeType::Document => NodeType::Document,
      TreeNodeType::Element(_) => NodeType::Element,
      TreeNodeType::Text(_) => NodeType::Text,
    }
  }

  fn parent(&self) -> Option<Rc<dyn Node>> {
    match self.ancestor_iter().nth(0) {
      Some(p) => {
        Some(Rc::new(p))
      }
      None => None
    }
  }

  // TODO: redo as iterators
  fn ancestors(&self) -> Vec<Rc<dyn Node>> {
    //self.ancestor_iter().map(|n| Rc::new(n)).collect()
    let mut it = self.ancestor_iter();
    let mut result: Vec<Rc<dyn Node>> = vec![];
    loop {
      match it.next() {
        Some(a) => {
	  result.push(Rc::new(a))
	}
	None => break,
      }
    }
    result
  }
  fn children(&self) -> Vec<Rc<dyn Node>> {
    //self.child_iter().map(|n| Rc::new(n)).collect()
    let mut it = self.child_iter();
    let mut result: Vec<Rc<dyn Node>> = vec![];
    loop {
      match it.next() {
        Some(a) => {
	  result.push(Rc::new(a))
	}
	None => break,
      }
    }
    result
  }
  fn following_siblings(&self) -> Vec<Rc<dyn Node>> {
    //self.sibling_iter().map(|n| Rc::new(n)).collect()
    let mut it = self.sibling_iter();
    let mut result: Vec<Rc<dyn Node>> = vec![];
    loop {
      match it.next() {
        Some(a) => {
	  result.push(Rc::new(a))
	}
	None => break,
      }
    }
    result
  }
  fn preceding_siblings(&self) -> Vec<Rc<dyn Node>> {
    //self.preceding_sibling_iter().map(|n| Rc::new(n)).collect()
    let mut it = self.preceding_sibling_iter();
    let mut result: Vec<Rc<dyn Node>> = vec![];
    loop {
      match it.next() {
        Some(a) => {
	  result.push(Rc::new(a))
	}
	None => break,
      }
    }
    result
  }

  fn descendants(&self) -> Vec<Rc<dyn Node>> {
    match self.get_doc().borrow()[self.get_index()] {
      TreeNodeType::Document => {
        match self.get_first_child() {
	  Some(r) => vec![Rc::new(r)],
	  None => vec![],
	}
      }
      TreeNodeType::Element(_) => {
	self.child_iter()
	  .fold(
	    vec![Rc::new(self.clone())],
	    |mut a, n| {
	      let mut w = n.descendants();
	      a.append(&mut w);
	      a
	    }
	  )
      }
      TreeNodeType::Text(_) => {
        vec![Rc::new(self.clone())]
      }
    }
  }
  fn get_following_sibling(&self) -> Option<Rc<dyn Node>> {
    //self.sibling_iter().nth(0).map(|n| Rc::new(n))
    match self.sibling_iter().nth(0) {
      Some(n) => {
        Some(Rc::new(n))
      }
      None => None,
    }
  }
  fn get_preceding_sibling(&self) -> Option<Rc<dyn Node>> {
    //self.preceding_sibling_iter().nth(0).map(|n| Rc::new(n))
    match self.preceding_sibling_iter().nth(0) {
      Some(n) => {
        Some(Rc::new(n))
      }
      None => None,
    }
  }

  // TODO
  fn attribute(&self, _name: &str) -> Option<String> {
    None
  }

  fn is_element(&self) -> bool {
    match self.get_doc().borrow()[self.get_index()] {
      TreeNodeType::Element(_) => true,
      TreeNodeType::Document |
      TreeNodeType::Text(_) => false,
    }
  }

  fn add_child(&self, c: &dyn Any) -> Result<(), Error> {
    let e = match c.downcast_ref::<XDMTreeNode>() {
      Some(d) => d,
      None => return Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "child node must be a XDMTreeNode".to_string()}),
    };
    Ok(self.append_child(e.clone()))
  }
  fn add_text_child(&self, t: String) -> Result<(), Error> {
    let t = self.new_value(Value::String(t));
    self.append_child(t);
    Ok(())
  }
}

fn get_doc_node(g: &XDMTree) -> XDMTreeNode {
    let h = g.borrow();
    let r = match h.node_indices()
      .find(|i| match h[*i] {
          TreeNodeType::Document => true,
          _ => false,
        }) {
      Some(r) => {
        r
      }
      None => {
        panic!("no document node")
      }
    };
    XDMTreeNode::new_node(g.clone(), r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doc() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(t.clone());
      Item::Document(Rc::new(t));
    }

    #[test]
    fn node() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r);
      Item::Node(Rc::new(d));
    }

    #[test]
    fn node_xml() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r);
      let i = Item::Node(Rc::new(d));

      assert!(i.to_xml() == "<Test/>" || i.to_xml() == "<Test></Test>")
    }

    #[test]
    fn node_str() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      let u = d.new_value(Value::String("this is a test".to_string()));
      r.append_child(u);
      d.append_child(r);
      let i = Item::Node(Rc::new(d));

      assert_eq!(i.to_string(), "this is a test")
    }

    #[test]
    fn doc_name() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      let u = d.new_value(Value::String("this is a test".to_string()));
      r.append_child(u);
      d.append_child(r);
      let i = Item::Node(Rc::new(d));

      assert_eq!(i.to_name().get_localname(), "")
    }
    #[test]
    fn element_name() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      let u = d.new_value(Value::String("this is a test".to_string()));
      r.append_child(u);
      d.append_child(r.clone());
      let i = Item::Node(Rc::new(r));

      assert_eq!(i.to_name().get_localname(), "Test")
    }

    #[test]
    fn new_element() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let n = t.new_element("Data", None).expect("unable to create element");
      r.add_child(n.as_any()).expect("unable to add child");

      let e = Item::Document(Rc::new(t));

      assert_eq!(e.to_xml(), "<Test><Data></Data></Test>");
    }

    #[test]
    fn new_value() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let n = t.new_element("Data", None).expect("unable to create element");
      r.add_child(n.as_any()).expect("unable to add child");
      n.add_text_child("this is a test".to_string()).expect("unable to add text");

      let e = Item::Document(Rc::new(t));

      assert_eq!(e.to_xml(), "<Test><Data>this is a test</Data></Test>");
    }

    #[test]
    fn descend() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      r.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c1.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c2.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      c3.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("this is a test".to_string()).expect("unable to add text");

      assert_eq!(r.descendants().len(), 6);
    }

    #[test]
    fn ascend() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      r.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c1.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c2.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      c3.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("this is a test".to_string()).expect("unable to add text");

      assert_eq!(c4.ancestors().len(), 4);
    }

    #[test]
    fn siblings() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      r.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      r.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      r.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      r.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      assert_eq!(c1.following_siblings().len(), 3);
    }

    #[test]
    fn preceding_siblings() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      r.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      r.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      r.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      r.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      assert_eq!(c4.preceding_siblings().len(), 3);
    }

    // Evaluation tests

    #[test]
    fn eval_root() {
    }
}
