/// XDM Node structure using petgraph.

use std::rc::Rc;
use std::cell::RefCell;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Direction;
use crate::item::{Value, QualifiedName};

pub type XDMTree = Rc<RefCell<Graph<NodeType, EdgeType>>>;

#[derive(Clone)]
pub struct XDMTreeNode {
  g: XDMTree,
  n: NodeIndex,
}

pub struct ElementType {
  pub name: QualifiedName,
}

pub enum NodeType {
  Document,
  Element(ElementType),
  Text(Value),
  // Comment(String),
  // PI(String, String),
  // Attribute(Value),
}

pub enum EdgeType {
  Document,
  FirstChild,
  Parent,
  NextSibling,
  PrecedingSibling,
  // Attribute,
}

impl XDMTreeNode {
  pub fn new(g: XDMTree) -> XDMTreeNode {
    let n = g.borrow_mut().add_node(NodeType::Document);
    XDMTreeNode{g: g.clone(), n: n}
  }
  pub fn new_node(g: XDMTree, n: NodeIndex) -> XDMTreeNode {
    XDMTreeNode{g, n}
  }
  pub fn get_doc(&self) -> XDMTree {
    self.g.clone()
  }
  pub fn get_index(&self) -> NodeIndex {
    self.n.clone()
  }
  pub fn get_doc_node(&self) -> XDMTreeNode {
    match self.g.borrow().node_indices()
      .find(|i| match self.g.borrow()[*i] {
          NodeType::Document => true,
          _ => false,
        }) {
      Some(r) => XDMTreeNode{g: self.g.clone(), n: r},
      None => {
        panic!("no document node")
      }
    }
  }
  pub fn child_iter(&self) -> Children {
    Children::new(self.clone())
  }
  pub fn ancestor_iter(&self) -> Ancestors {
    Ancestors::new(self.clone())
  }
  pub fn sibling_iter(&self) -> Siblings {
    Siblings::new(self.clone())
  }
  pub fn preceding_sibling_iter(&self) -> PrecedingSiblings {
    PrecedingSiblings::new(self.clone())
  }
  pub fn get_first_child(&self) -> Option<XDMTreeNode> {
    let c1: Vec<NodeIndex> = self.g.borrow()
      .neighbors_directed(self.n, Direction::Outgoing)
      .filter(|i| self.g.borrow().edges_connecting(self.n, *i)
	.fold(false, |s, e| {
	  match e.weight() {
	    EdgeType::FirstChild => true,
	    _ => s,
	  }
        })
      )
      .collect();
    if c1.len() == 1 {
      Some(XDMTreeNode{g: self.g.clone(), n: c1[0]})
    } else {
      None
    }
  }
  pub fn get_last_sibling(&self) -> Option<XDMTreeNode> {
    self.sibling_iter().last()
  }

  pub fn new_element(&self, name: QualifiedName) -> XDMTreeNode {
    let r = self.get_doc_node();
    let mut b = self.g.borrow_mut();
    let n: NodeIndex = b.add_node(NodeType::Element(ElementType{
        name,
      }));
    b.add_edge(n, r.n, EdgeType::Document);
    XDMTreeNode{g: self.g.clone(), n: n}
  }
  pub fn new_value(&self, v: Value) -> XDMTreeNode {
    let r = self.get_doc_node();
    let mut b = self.g.borrow_mut();
    let n: NodeIndex = b.add_node(NodeType::Text(v));
    b.add_edge(n, r.n, EdgeType::Document);
    XDMTreeNode{g: self.g.clone(), n: n}
  }

  pub fn append_child(&self, child: XDMTreeNode) {
    // Does the parent have any children?
    // If not then this is the first child,
    // otherwise find the last child and add this node as it's next sibling
    let fc = self.get_first_child();
    let (first, sib) = match fc {
      Some(c) => {
        match c.get_last_sibling() {
	  Some(d) => {
            //println!("append_child: have first child and sibling");
	    (None, Some(d.n))
	  }
	  None => {
            //println!("append_child: have first child but no sibling");
	    (None, Some(c.n))
	  }
	}
      }
      None => {
            //println!("append_child: no first child");
        (Some(child.n), None)
      }
    };
    let mut b = self.g.borrow_mut();
    b.add_edge(child.n, self.n, EdgeType::Parent);
    match (first, sib) {
      (None, Some(d)) => {
	b.add_edge(d, child.n, EdgeType::NextSibling);
        b.add_edge(child.n, d, EdgeType::PrecedingSibling);
      }
      (Some(d), None) => {
	b.add_edge(self.n, d, EdgeType::FirstChild);
      }
      _ => {}
    }
  }

  pub fn to_xml_int(&self) -> String {
    //println!("to_xml_int({})", self.n.index());
    match &self.g.borrow()[self.n] {
      NodeType::Element(e) => {
        //println!("Element");
	let mut ret: String = String::new();
      	ret.push_str("<");
      	ret.push_str(e.name.get_localname().as_str());
      	ret.push_str(">");
      	self.child_iter().for_each(
          |c| {
	    ret.push_str(c.to_xml_int().as_str());
	  }
        );
      	ret.push_str("</");
      	ret.push_str(e.name.get_localname().as_str());
      	ret.push_str(">");
      	ret
      }
      NodeType::Text(t) => {
        //println!("Text");
        t.to_string()
      }
      NodeType::Document => {
        //println!("Document");
	self.get_first_child()
	  .map_or("".to_string(), |n| n.to_xml_int())
      }
    }
  }
}

pub struct Children {
  parent: XDMTreeNode,
  node: Option<XDMTreeNode>,
}

impl Children {
  fn new(parent: XDMTreeNode) -> Children {
    Children{parent: parent, node: None}
  }
}

impl Iterator for Children {
  type Item = XDMTreeNode;

  fn next(&mut self) -> Option<Self::Item> {
    match &self.node {
      Some(n) => {
        // get the next sibling
	match n.sibling_iter().nth(0) {
	  Some(c) => {
	    self.node = Some(c.clone());
	    Some(c)
	  }
	  None => None,
	}
      }
      None => {
        // get the first child
	match self.parent.get_first_child() {
	  Some(c) => {
	    self.node = Some(c.clone());
	    Some(c)
	  }
	  None => None,
	}
      }
    }
  }
}

pub struct Ancestors {
  node: XDMTreeNode,
}

impl Ancestors {
  fn new(node: XDMTreeNode) -> Ancestors {
    Ancestors{node: node}
  }
}

impl Iterator for Ancestors {
  type Item = XDMTreeNode;

  fn next(&mut self) -> Option<Self::Item> {
    // get the parent
    let v: Vec<NodeIndex> = self.node.g.borrow()
      .neighbors_directed(self.node.n, Direction::Outgoing)
      .filter(|i| self.node.g.borrow()
        .edges_connecting(self.node.n, *i)
	.fold(false, |s, e| {
	  match e.weight() {
	    EdgeType::Parent => true,
	    _ => s,
	  }
	})
      )
      .collect();
    if v.len() == 1 {
      self.node.n = v[0];
      Some(self.node.clone())
    } else {
      None
    }
  }
}

pub struct Siblings {
  node: XDMTreeNode,
}

impl Siblings {
  fn new(node: XDMTreeNode) -> Siblings {
    Siblings{node: node}
  }
}

impl Iterator for Siblings {
  type Item = XDMTreeNode;

  fn next(&mut self) -> Option<Self::Item> {
    let v: Vec<NodeIndex> = self.node.g.borrow()
      .neighbors_directed(self.node.n, Direction::Outgoing)
      .filter(|i| self.node.g.borrow()
        .edges_connecting(self.node.n, *i)
        .fold(false, |s, e| {
	  match e.weight() {
	    EdgeType::NextSibling => true,
	    _ => s,
	  }
	})
      )
      .collect();
    if v.len() == 1 {
      self.node.n = v[0];
      Some(self.node.clone())
    } else {
      None
    }
  }
}

pub struct PrecedingSiblings {
  node: XDMTreeNode,
}

impl PrecedingSiblings {
  fn new(node: XDMTreeNode) -> PrecedingSiblings {
    PrecedingSiblings{node: node}
  }
}

impl Iterator for PrecedingSiblings {
  type Item = XDMTreeNode;

  fn next(&mut self) -> Option<Self::Item> {
    let v: Vec<NodeIndex> = self.node.g.borrow()
      .neighbors_directed(self.node.n, Direction::Outgoing)
      .filter(|i| self.node.g.borrow()
        .edges_connecting(self.node.n, *i)
        .fold(false, |s, e| {
	  match e.weight() {
	    EdgeType::PrecedingSibling => true,
	    _ => s,
	  }
	})
      )
      .collect();
    if v.len() == 1 {
      self.node.n = v[0];
      Some(self.node.clone())
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_doc() {
        let t = Rc::new(RefCell::new(Graph::new()));
	XDMTreeNode::new(t.clone());
	assert_eq!(t.borrow().node_count(), 1);
    }

    #[test]
    fn new_element() {
        let t = Rc::new(RefCell::new(Graph::new()));
	let d = XDMTreeNode::new(t.clone());
	let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
	d.append_child(r);
	assert_eq!(d.get_doc().borrow().node_count(), 2);
	assert_eq!(d.to_xml_int(), "<Test></Test>");
    }

    #[test]
    fn new_value() {
        let t = Rc::new(RefCell::new(Graph::new()));
	let d = XDMTreeNode::new(t.clone());
	let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
	d.append_child(r.clone());
	let u = d.new_value(Value::String("this is a test".to_string()));
	r.append_child(u);
	assert_eq!(t.borrow().node_count(), 3);
	assert_eq!(d.to_xml_int(), "<Test>this is a test</Test>");
    }

    #[test]
    fn multi_elements() {
        let t = Rc::new(RefCell::new(Graph::new()));
	let d = XDMTreeNode::new(t.clone());
	let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
	d.append_child(r.clone());
	let c1 = d.new_element(QualifiedName::new(None, None, "Data".to_string()));
	let u1 = d.new_value(Value::String("one".to_string()));
	c1.append_child(u1);
	r.append_child(c1.clone());
	let c2 = d.new_element(QualifiedName::new(None, None, "Data".to_string()));
	let u2 = d.new_value(Value::String("two".to_string()));
	c2.append_child(u2);
	r.append_child(c2.clone());
	assert_eq!(t.borrow().node_count(), 6);
	assert_eq!(d.to_xml_int(), "<Test><Data>one</Data><Data>two</Data></Test>");
    }

    #[test]
    fn children() {
        let t = Rc::new(RefCell::new(Graph::new()));
	let d = XDMTreeNode::new(t.clone());
	let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
	d.append_child(r.clone());
	let c1 = d.new_element(QualifiedName::new(None, None, "Data".to_string()));
	let u1 = d.new_value(Value::String("one".to_string()));
	c1.append_child(u1);
	r.append_child(c1.clone());
	let c2 = d.new_element(QualifiedName::new(None, None, "Data".to_string()));
	let u2 = d.new_value(Value::String("two".to_string()));
	c2.append_child(u2);
	r.append_child(c2.clone());

	assert_eq!(r.child_iter().collect::<Vec<XDMTreeNode>>().len(), 2);
    }
    #[test]
    fn descend() {
        let t = Rc::new(RefCell::new(Graph::new()));
	let d = XDMTreeNode::new(t.clone());
	let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
	d.append_child(r.clone());
	let c1 = d.new_element(QualifiedName::new(None, None, "Data".to_string()));
	r.append_child(c1.clone());
	let c2 = d.new_element(QualifiedName::new(None, None, "Data".to_string()));
	c1.append_child(c2.clone());
	let c3 = d.new_element(QualifiedName::new(None, None, "Data".to_string()));
	c2.append_child(c3.clone());

	assert_eq!(r.to_xml_int(), "<Test><Data><Data><Data></Data></Data></Data></Test>");
    }

    #[test]
    fn siblings() {
        let t = Rc::new(RefCell::new(Graph::new()));
	let d = XDMTreeNode::new(t.clone());
	let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
	d.append_child(r.clone());
	let c1 = d.new_element(QualifiedName::new(None, None, "Data".to_string()));
	let u1 = d.new_value(Value::String("one".to_string()));
	c1.append_child(u1);
	r.append_child(c1.clone());
	let c2 = d.new_element(QualifiedName::new(None, None, "Data".to_string()));
	let u2 = d.new_value(Value::String("two".to_string()));
	c2.append_child(u2);
	r.append_child(c2.clone());

	assert_eq!(c1.sibling_iter().collect::<Vec<XDMTreeNode>>().len(), 1);
    }

    #[test]
    fn preceding_siblings() {
        let t = Rc::new(RefCell::new(Graph::new()));
	let d = XDMTreeNode::new(t.clone());
	let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
	d.append_child(r.clone());
	let c1 = d.new_element(QualifiedName::new(None, None, "Data".to_string()));
	let u1 = d.new_value(Value::String("one".to_string()));
	c1.append_child(u1);
	r.append_child(c1.clone());
	let c2 = d.new_element(QualifiedName::new(None, None, "Data".to_string()));
	let u2 = d.new_value(Value::String("two".to_string()));
	c2.append_child(u2);
	r.append_child(c2.clone());

	assert_eq!(c2.preceding_sibling_iter().collect::<Vec<XDMTreeNode>>().len(), 1);
    }

    #[test]
    fn ancestors() {
        let t = Rc::new(RefCell::new(Graph::new()));
	let d = XDMTreeNode::new(t.clone());
	let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
	d.append_child(r.clone());
	let c1 = d.new_element(QualifiedName::new(None, None, "Data".to_string()));
	let u1 = d.new_value(Value::String("one".to_string()));
	c1.append_child(u1);
	r.append_child(c1.clone());
	let c2 = d.new_element(QualifiedName::new(None, None, "Data".to_string()));
	let u2 = d.new_value(Value::String("two".to_string()));
	c2.append_child(u2.clone());
	r.append_child(c2.clone());

	assert_eq!(u2.ancestor_iter().collect::<Vec<XDMTreeNode>>().len(), 2);
    }
}
