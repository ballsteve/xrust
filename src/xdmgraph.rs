/// XDM Node structure using petgraph.

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Direction;
use crate::item::{Value, QualifiedName};
use crate::parsexml::*;
use crate::xdmerror::*;

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
    match &self.g.borrow()[self.n] {
      NodeType::Element(e) => {
	let mut ret: String = String::new();
      	ret.push_str("<");
      	match e.name.get_prefix() {
	  Some(p) => {
	    ret.push_str(p.as_str());
	    ret.push(':');
	  }
	  _ => {},
	}
      	ret.push_str(e.name.get_localname().as_str());
	// TODO: don't emit namespace declaration if it is already declared in ancestor element
	match e.name.get_nsuri_ref() {
	  Some(uri) => {
	    println!("found nsuri");
	    ret.push(' ');
	    ret.push_str("xmlns:");
	    // TODO: handle default namespace declaration
	    ret.push_str(e.name.get_prefix().unwrap().as_str());
	    ret.push_str("='");
	    ret.push_str(uri);
	    ret.push_str("'");
	  }
	  None => {
	    println!("no nsuri");
	  }
	}
      	ret.push_str(">");
      	self.child_iter().for_each(
          |c| {
	    ret.push_str(c.to_xml_int().as_str());
	  }
        );
      	ret.push_str("</");
      	match e.name.get_prefix() {
	  Some(p) => {
	    ret.push_str(p.as_str());
	    ret.push(':');
	  }
	  _ => {},
	}
	ret.push_str(e.name.get_localname().as_str());
      	ret.push_str(">");
      	ret
      }
      NodeType::Text(t) => {
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

/// Parse XML and return a fully populated XDMTree
pub fn from(input: &str) -> Result<XDMTreeNode, Error> {
  let d = match parse(input) {
    Ok(x) => x,
    Err(e) => return Result::Err(e),
  };
  // Map namespace prefix to namespace URI
  if d.content.len() == 0 {
    Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("no content")})
  } else {
    let mut ns: HashMap<String, String> = HashMap::new();
    let t = Rc::new(RefCell::new(Graph::new()));
    let r = XDMTreeNode::new(t.clone());
    parse_node(&d.content[0], r.clone(), &mut ns);
    Ok(r)
  }
}
fn parse_node(
  e: &XMLNode,
  parent: XDMTreeNode,
  ns: &mut HashMap<String, String>
) {
  match e {
    XMLNode::Element(n, a, c) => {
      // NB. the parsexml parser could do the namespace resolution,
      // but we'll do it here since we have to make a pass through the
      // structure anyway.

      // Add any namespace declarations to the hashmap
      a.iter()
        .filter(|b| {
          match b {
	    XMLNode::Attribute(qn, _) => {
	      match qn.get_prefix() {
	        Some(p) => {
		  if p == "xmlns" {
		    true
	      	  } else {
	            false
	      	  }
		}
		_ => false,
	      }
	    }
	    _ => false,
	  }
        })
        .for_each(|b| {
	  match b {
	    XMLNode::Attribute(qn, v) => {
	      // add map from prefix to uri in hashmap
	      match ns.insert(qn.get_localname(), v.to_string()) {
		Some(_) => {}, // TODO: handle inner scope of declaration
		None => {},
	      }
	    }
	    _ => {}
	  }
	});
      // Add the element to the tree
      let newns = match n.get_prefix() {
        Some(p) => ns.get(&p),
	None => None,
      };
      let new = parent.new_element(
        QualifiedName::new(
	  newns.map(|m| m.clone()),
	  n.get_prefix(),
	  n.get_localname()
	)
      );
      parent.append_child(new.clone());
      c.iter().cloned().for_each(|f| {
        parse_node(&f, new.clone(), ns)
      });
    }
    XMLNode::Text(v) => {
      let u = parent.new_value(v.clone());
      parent.append_child(u);
    }
    _ => {
      // TODO: Not yet implemented
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

    // Parsing XML

    #[test]
    fn parse_empty() {
      let r = from("<Test/>").expect("unable to parse \"<Test/>\"");

      let c = r.child_iter().collect::<Vec<XDMTreeNode>>();
      assert_eq!(c.len(), 1);
      assert_eq!(c[0].to_xml_int(), "<Test></Test>");
    }

    #[test]
    fn parse_empty_qualified() {
      let r = from("<x:Test xmlns:x='urn:my-test'/>").expect("unable to parse \"<x:Test xlmns:x='urn:my-test'/>\"");

      let c = r.child_iter().collect::<Vec<XDMTreeNode>>();
      assert_eq!(c.len(), 1);
      assert_eq!(c[0].to_xml_int(), "<x:Test xmlns:x='urn:my-test'></x:Test>");
    }

    #[test]
    fn parse_text() {
      let r = from("<Test>foobar</Test>").expect("unable to parse \"<Test><foobar</Test>\"");

      let c = r.child_iter().collect::<Vec<XDMTreeNode>>();
      assert_eq!(c.len(), 1);
      assert_eq!(c[0].to_xml_int(), "<Test>foobar</Test>");
    }

    #[test]
    fn parse_element_children() {
      let r = from("<Test><a/><b/><c/></Test>").expect("unable to parse \"<Test><a/><b/><c/></Test>\"");

      let c = r.child_iter().collect::<Vec<XDMTreeNode>>();
      assert_eq!(c.len(), 1);
      assert!(c[0].to_xml_int() == "<Test><a/><b/><c/></Test>" ||
        c[0].to_xml_int() == "<Test><a></a><b></b><c></c></Test>"
      );
    }

    #[test]
    fn parse_mixed() {
      let r = from("<Test>i1<child>one</child>i2<child>two</child>i3</Test>").expect("unable to parse \"<Test>i1<child>one</child>i2<child>two</child>i3</Test>\"");

      let c = r.child_iter().collect::<Vec<XDMTreeNode>>();
      assert_eq!(c.len(), 1);
      assert_eq!(c[0].to_xml_int(), "<Test>i1<child>one</child>i2<child>two</child>i3</Test>");
    }
}
