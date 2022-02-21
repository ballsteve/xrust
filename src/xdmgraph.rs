/// XDM Node structure using petgraph.

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use petgraph::stable_graph::*;
use petgraph::Direction;
use petgraph::visit::*;
use crate::qname::QualifiedName;
use crate::item::{Value, OutputDefinition};
use crate::parsexml::*;
use crate::xdmerror::*;

pub type XDMTree = Rc<RefCell<StableGraph<NodeType, EdgeType>>>;

#[derive(Clone)]
pub struct XDMTreeNode {
  g: XDMTree,
  n: NodeIndex,
}

#[derive(Clone)]
pub enum NodeType {
  Document,
  Element(QualifiedName),
  Text(Value),
  // Comment(String),
  // PI(String, String),
  Attribute(QualifiedName, Value),
}

pub enum EdgeType {
  Document,
  FirstChild,
  Parent,
  NextSibling,
  PrecedingSibling,
  Attribute(QualifiedName),	// Attribute name is duplicated in both the edge and the node, since attributes are also nodes in their own right
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
  // Get the name for the node
  pub fn get_name(&self) -> QualifiedName {
    match self.g.borrow()[self.n] {
      NodeType::Element(ref qn) => qn.clone(),
      NodeType::Attribute(ref qn, _) => qn.clone(),
      _ => QualifiedName::new(None, None, "".to_string()),
    }
  }
  // Get the value for the node
  pub fn get_value(&self) -> Value {
    match self.g.borrow()[self.n] {
      NodeType::Text(ref v) => v.clone(),
      NodeType::Attribute(_, ref v) => v.clone(),
      _ => Value::String("".to_string()),
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
    let h = self.g.borrow();
    let c1: Vec<NodeIndex> = h
      .edges_directed(self.n, Direction::Outgoing)
      .filter(|e| match e.weight() {
        EdgeType::FirstChild => true,
	_ => false,
      })
      .map(|e| {
        match h.edge_endpoints(e.id()) {
	  Some((_, t)) => {
	    t
	  }
	  None => panic!("unable to find first child")
	}
      })
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

  pub fn new_element_node(&self, name: QualifiedName) -> XDMTreeNode {
    let r = self.get_doc_node();
    let mut b = self.g.borrow_mut();
    let n: NodeIndex = b.add_node(NodeType::Element(name));
    b.add_edge(n, r.n, EdgeType::Document);
    XDMTreeNode{g: self.g.clone(), n: n}
  }
  pub fn new_value_node(&self, v: Value) -> XDMTreeNode {
    let r = self.get_doc_node();
    let mut b = self.g.borrow_mut();
    let n: NodeIndex = b.add_node(NodeType::Text(v));
    b.add_edge(n, r.n, EdgeType::Document);
    XDMTreeNode{g: self.g.clone(), n: n}
  }

  /// Set the value for a text, attribute, comment or PI node
  pub fn node_value(&self, v: Value) {
    match self.g.borrow()[self.n] {
      NodeType::Attribute(ref qn, _) => {
        self.g.borrow_mut().node_weight_mut(self.n).replace(&mut NodeType::Attribute(qn.clone(), v));
      }
      NodeType::Text(_) => {
        self.g.borrow_mut().node_weight_mut(self.n).replace(&mut NodeType::Text(v));
      }
      _ => {}
    }
  }

  pub fn append_child_node(&self, child: XDMTreeNode) -> Result<(), Error> {
    // First, child must not be an attribute
    match child.g.borrow()[child.n] {
      NodeType::Attribute(_, _) => return Result::Err(Error{kind: ErrorKind::Unknown, message: "cannot append an attribute node".to_string()}),
      _ => {}
    }

    // Are the two nodes in the same Graph?
    // If not, make a deep-copy of the child
    let nchild: XDMTreeNode;
    if Rc::ptr_eq(&self.g, &child.g) {
      nchild = child;
    } else {
      match child.g.borrow()[child.n] {
        NodeType::Text(ref v) => {
	  nchild = self.new_value_node(v.clone());
	}
	_ => {
	  // TODO
	  nchild = self.new_element_node(QualifiedName::new(None, None, "TODO".to_string()));
	}
      }
    }
    // Does the parent have any children?
    // If not then this is the first child,
    // otherwise find the last child and add this node as it's next sibling
    let fc = self.get_first_child();
    let (first, sib) = match fc {
      Some(c) => {
        match c.get_last_sibling() {
	  Some(d) => {
	    (None, Some(d.n))
	  }
	  None => {
	    (None, Some(c.n))
	  }
	}
      }
      None => {
        (Some(nchild.n), None)
      }
    };
    self.g.borrow().node_indices()
      .for_each(|i| {
        let mut result = String::new();
	match self.g.borrow()[i] {
	    NodeType::Element(ref qn) => {
	      result.push_str("element \"");
	      result.push_str(qn.get_localname().as_str());
	      result.push_str("\"");
	    }
	    NodeType::Document => result.push_str("Document"),
	    NodeType::Text(ref v) => {
	      result.push_str("text: ");
	      result.push_str(v.to_string().as_str());
	    }
	    _ => result.push_str("--not an element--"),
	  };
      });
    let mut b = self.g.borrow_mut();
    b.add_edge(nchild.n, self.n, EdgeType::Parent);
    match (first, sib) {
      (None, Some(d)) => {
	b.add_edge(d, nchild.n, EdgeType::NextSibling);
        b.add_edge(nchild.n, d, EdgeType::PrecedingSibling);
      }
      (Some(d), None) => {
	b.add_edge(self.n, d, EdgeType::FirstChild);
      }
      _ => {}
    };

    Ok(())
  }

  // Creates an attribute type node, but not attached (yet) to an element
  pub fn new_attribute_node(&self, name: QualifiedName, v: Value) -> XDMTreeNode {
    let r = self.get_doc_node();
    let mut b = self.g.borrow_mut();
    let n: NodeIndex = b.add_node(NodeType::Attribute(name, v));

    b.add_edge(n, r.n, EdgeType::Document);

    XDMTreeNode{
      g: self.g.clone(),
      n: n,
    }
  }

  // Attach an attribute node to an element
  pub fn add_attribute(&self, a: XDMTreeNode) -> Result<(), Error> {
    // The node must be an element node
    match self.g.borrow()[self.n] {
      NodeType::Element(_) => {}
      _ => return Result::Err(Error{kind: ErrorKind::Unknown, message: "not an element".to_string()})
    }

    let mut b = self.g.borrow_mut();

    match b[a.n] {
      NodeType::Attribute(ref qn, _) => {
        let newqn = qn.clone();
	b.add_edge(self.n, a.n, EdgeType::Attribute(newqn));
      }
      _ => {}
    }
    b.add_edge(a.n, self.n, EdgeType::Parent);

    Ok(())
  }
  pub fn get_attribute_node(&self, name: &QualifiedName) -> Option<XDMTreeNode> {
    let h = self.g.borrow();
    let a: Vec<Option<NodeIndex>> = h
      .edges_directed(self.n, Direction::Outgoing)
      .filter(|e| match e.weight() {
        EdgeType::Attribute(att) => {
	  match (name.get_nsuri(), att.get_nsuri()) {
	    (Some(namens), Some(attns)) => {
	      // prefixed
	      namens == attns &&
	      name.get_localname() == att.get_localname()
	    }
	    (None, None) => {
	      // unprefixed
	      name.get_localname() == att.get_localname()
	    }
	    _ => false,
	  }
	}
	_ => false,
      })
      .map(|e| match h.edge_endpoints(e.id()) {
        Some((_, t)) => Some(t),
	None => None,
      })
      .collect();
    if a.len() == 1 {
      match a[0] {
        Some(i) => {
	  match &h[i] {
	    NodeType::Attribute(_, _) => Some(XDMTreeNode{g: self.g.clone(), n: i}),
	    _ => None,
	  }
	}
	_ => None,
      }
    } else {
      None
    }
  }
  pub fn get_attribute(&self, name: &QualifiedName) -> Option<Value> {
    match self.get_attribute_node(name) {
      Some(a) => {
        match self.g.borrow()[a.n] {
	  NodeType::Attribute(_, ref v) => Some(v.clone()),
	  _ => None,
	}
      }
      None => None,
    }
  }
  pub fn attr_node_iter(&self) -> Box<Attributes> {
    Box::new(Attributes::new(self.clone()))
  }

  // Removes a node from the tree.
  pub fn remove_node(&self) {
    let parent = match self.ancestor_iter().nth(0) {
      Some(p) => p,
      None => return,
    };

    let nt: NodeType;
    {
      nt = self.g.borrow()[self.n].clone();
    }
    match nt {
      NodeType::Element(_) => {
        // Remove all attributes
	self.attr_node_iter()
	  .for_each(|a| a.remove_node());

    	// Remove children
    	self.child_iter()
      	  .for_each(|c| c.remove_node());

    	// Remove and repoint siblings
	remove_repoint_siblings(self.g.clone(), parent.n, self.n);

    	// Remove parent edge
    	// Remove document edge
	remove_docparent(self.g.clone(), self.n);
      }
      NodeType::Attribute(_, _) => {
	// Detach from the parent element (if any)
    	let mut edges: Vec<EdgeIndex> = vec![];
	{
	  self.g.borrow().edges_directed(self.n, Direction::Incoming)
	    .for_each(|e| {
	      match e.weight() {
	        EdgeType::Attribute(_) => {
	          edges.push(e.id());
	        }
	      	EdgeType::Document |
	      	EdgeType::Parent => {
	          edges.push(e.id());
	        }
	        _ => {}
	      }
	    });
	}
	{
	  let mut h = self.g.borrow_mut();
	  edges.iter().for_each(|i| {h.remove_edge(*i);});
	}
      }
      NodeType::Text(_) => {
    	// Remove parent edge
    	// Remove document edge
	remove_docparent(self.g.clone(), self.n);
	// repoint sibling edges
	remove_repoint_siblings(self.g.clone(), parent.n, self.n);
      }
      _ => {}
    }
    // Remove node
    self.g.borrow_mut().remove_node(self.n);
  }

  pub fn to_xml_int(&self, od: Option<&OutputDefinition>, indent: usize) -> String {
    let h = self.g.borrow();
    match &h[self.n] {
      NodeType::Element(ref qn) => {
	let mut ret: String = String::new();
      	ret.push_str("<");
      	match qn.get_prefix() {
	  Some(p) => {
	    ret.push_str(p.as_str());
	    ret.push(':');
	  }
	  _ => {},
	}
      	ret.push_str(qn.get_localname().as_str());
	// TODO: don't emit namespace declaration if it is already declared in ancestor element
	match qn.get_nsuri_ref() {
	  Some(uri) => {
	    ret.push(' ');
	    match qn.get_prefix() {
	      Some(p) => {
	        ret.push_str("xmlns:");
	    	ret.push_str(p.as_str());
	      }
	      None => {
	        // Default namespace
	        ret.push_str("xmlns");
	      }
	    }
	    ret.push_str("='");
	    ret.push_str(uri);
	    ret.push_str("'");
	  }
	  None => {}
	}
	self.attr_node_iter()
	  .for_each(|a| {
	    ret.push(' ');
	    ret.push_str(a.get_name().to_string().as_str());
	    ret.push_str("='");
	    ret.push_str(a.get_value().to_string().as_str());
	    ret.push_str("'");
	  });
      	ret.push_str(">");

	// Content of the element.
	// If the indent option is enabled, if no child is a text node then add spacing
	let do_indent: bool = od.as_ref().map_or(
	  false,
	  |o| {
	    if o.get_indent() {
	      self.child_iter().fold(
	        true,
	        |a, c| {
	          match &h[c.n] {
	            NodeType::Text(_) => false,
		    _ => a,
	          }
	        }
	      )
	    } else {
	      false
	    }
	  }
	);

	self.child_iter().for_each(
          |c| {
	    if do_indent {
	      ret.push('\n');
	      (0..indent).for_each(|_| ret.push(' '));
	    };
	    // TODO: parameterise the number of spaces to indent
	    ret.push_str(c.to_xml_int(od, indent + 2).as_str());
	  }
        );
	if do_indent {
	  ret.push('\n');
	  (0..(indent - 2)).for_each(|_| ret.push(' '));
	};
      	ret.push_str("</");
      	match qn.get_prefix() {
	  Some(p) => {
	    ret.push_str(p.as_str());
	    ret.push(':');
	  }
	  _ => {},
	}
	ret.push_str(qn.get_localname().as_str());
      	ret.push_str(">");
      	ret
      }
      NodeType::Text(t) => {
        // TODO: escape special characters
	t.to_string()
      }
      NodeType::Document => {
	self.get_first_child()
	  .map_or("".to_string(), |n| n.to_xml_int(od, indent))
      }
      NodeType::Attribute(_, _) => {"".to_string()} // these are handled in the element arm
    }
  }
}

fn remove_docparent(g: XDMTree, n: NodeIndex) {
  let mut edge: Option<EdgeIndex> = None;
  g.borrow().edges_directed(n, Direction::Outgoing)
    .for_each(|e| {
      match e.weight() {
        EdgeType::Document |
	EdgeType::Parent => {
	  edge = Some(e.id());
	}
	_ => {}
      }
    });
  edge.map(|e| g.borrow_mut().remove_edge(e));
}
fn remove_repoint_siblings(g: XDMTree, parent: NodeIndex, n: NodeIndex) {
  let mut preceding: Option<(NodeIndex, EdgeIndex)> = None;
  let mut following: Option<(NodeIndex, EdgeIndex)> = None;
  g.borrow().edges_directed(n, Direction::Outgoing)
    .for_each(|e| {
      match e.weight() {
        EdgeType::PrecedingSibling => {
	  preceding = Some((e.target(), e.id()));
	}
	EdgeType::NextSibling => {
	  following = Some((e.target(), e.id()));
	}
	_ => {}
      }
    });
  match (preceding, following) {
    (Some((pre, preid)), Some((next, nextid))) => {
      // In the middle: join the preceding to the following
      g.borrow_mut().remove_edge(preid);
      g.borrow_mut().remove_edge(nextid);
      g.borrow_mut().add_edge(pre, next, EdgeType::NextSibling);
      g.borrow_mut().add_edge(next, pre, EdgeType::PrecedingSibling);
    }
    (Some((pre, preid)), None) => {
      // At the end of the line
      g.borrow_mut().remove_edge(preid);
      let mut edge: Option<EdgeIndex> = None;
      g.borrow().edges_directed(pre, Direction::Outgoing)
	.for_each(|e| {
	  match e.weight() {
	    EdgeType::NextSibling => {
	      edge = Some(e.id());
	    }
	    _ => {}
	  }
	});
      edge.map(|e| g.borrow_mut().remove_edge(e));
    }
    (None, Some((next, nextid))) => {
      // First child
      g.borrow_mut().remove_edge(nextid);
      let mut edge: Option<EdgeIndex> = None;
      g.borrow().edges_directed(next, Direction::Outgoing)
	.for_each(|e| {
	  match e.weight() {
	    EdgeType::PrecedingSibling => {
	      edge = Some(e.id());
	    }
	    _ => {}
	  }
	});
      edge.map(|e| g.borrow_mut().remove_edge(e));
      // Remove parent's child edge
      edge = None;
      g.borrow().edges_directed(parent, Direction::Outgoing)
	.for_each(|e| {
	  match e.weight() {
	    EdgeType::FirstChild => {
	      if e.target() == n {
	        edge = Some(e.id());
	      }
	    }
	    _ => {}
	  }
	});
      edge.map(|e| g.borrow_mut().remove_edge(e));
      // Repoint parent's child to sibling
      g.borrow_mut().add_edge(parent, next, EdgeType::FirstChild);
    }
    (None, None) => {
      // Remove parent's child edge
      let mut edge: Option<EdgeIndex> = None;
      g.borrow().edges_directed(parent, Direction::Outgoing)
	.for_each(|e| {
	  match e.weight() {
	    EdgeType::FirstChild => {
	      if e.target() == n {
	        edge = Some(e.id());
	      }
	    }
	    _ => {}
	  }
	});
      edge.map(|e| g.borrow_mut().remove_edge(e));
    }
  }
}

pub struct Attributes {
  attrs: Vec<XDMTreeNode>,
}

impl Attributes {
  pub fn new(parent: XDMTreeNode) -> Attributes {
    // Find all of the attributes and store their indices
    let mut a: Vec<XDMTreeNode> = vec![];
    let h = parent.g.borrow();
    h.edges_directed(parent.n, Direction::Outgoing)
      .filter(|e| match e.weight() {
        EdgeType::Attribute(_) => true,
	_ => false,
      })
      .for_each(|e| match h.edge_endpoints(e.id()) {
        Some((_, t)) => a.push(XDMTreeNode{g: parent.g.clone(), n: t}),
	None => {}
      });

    Attributes{attrs: a}
  }
}

impl Iterator for Attributes {
  type Item = XDMTreeNode;

  fn next(&mut self) -> Option<Self::Item> {
    match self.attrs.pop() {
      Some(a) => Some(a),
      None => None,
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
    let h = self.node.g.borrow();
    let v: Vec<Option<NodeIndex>> = h
      .edges_directed(self.node.n, Direction::Outgoing)
      .filter(|e| match e.weight() {
	    EdgeType::Parent => true,
	    _ => false,
      })
      .map(|e| match h.edge_endpoints(e.id()) {
        Some((_, t)) => Some(t),
	_ => None,
      })
      .collect();
    if v.len() == 1 {
      match v[0] {
        Some(m) => {
	  // Don't include the Document node
	  match h[m] {
	    NodeType::Document => None,
	    _ => {
	      self.node.n = m;
	      Some(self.node.clone())
	    }
	  }
	}
	None => None,
      }
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
    let h = self.node.g.borrow();
    let v: Vec<Option<NodeIndex>> = h
      .edges_directed(self.node.n, Direction::Outgoing)
      .filter(|e| match e.weight() {
	    EdgeType::NextSibling => true,
	    _ => false,
      })
      .map(|e| match h.edge_endpoints(e.id()) {
        Some((_, t)) => Some(t),
	_ => None,
      })
      .collect();
    if v.len() == 1 {
      match v[0] {
        Some(m) => {
	  self.node.n = m;
	  Some(self.node.clone())
	}
	None => None,
      }
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
    let h = self.node.g.borrow();
    let v: Vec<Option<NodeIndex>> = h
      .edges_directed(self.node.n, Direction::Outgoing)
      .filter(|e| match e.weight() {
	    EdgeType::PrecedingSibling => true,
	    _ => false,
      })
      .map(|e| match h.edge_endpoints(e.id()) {
        Some((_, t)) => Some(t),
	_ => None,
      })
      .collect();
    if v.len() == 1 {
      match v[0] {
        Some(m) => {
	  self.node.n = m;
	  Some(self.node.clone())
	}
	None => None,
      }
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
    let t = Rc::new(RefCell::new(StableGraph::new()));
    let r = XDMTreeNode::new(t.clone());
    parse_node(&d.content[0], r.clone(), &mut ns)?;
    Ok(r)
  }
}
fn parse_node(
  e: &XMLNode,
  parent: XDMTreeNode,
  ns: &mut HashMap<String, String>
) -> Result<(), Error> {
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
      let new = parent.new_element_node(
        QualifiedName::new(
	  newns.map(|m| m.clone()),
	  n.get_prefix(),
	  n.get_localname()
	)
      );
      match parent.append_child_node(new.clone()) {
        Ok(_) => {}
	Err(_) => return Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("unable to append child node")})
      };

      let mut status: Option<String> = None;
      a.iter()
        .for_each(|b| {
	  match b {
	    XMLNode::Attribute(qn, v) => {
	      match qn.get_prefix() {
	        Some(p) => {
		  if p == "xmlns" {
		    // Don't add this: it is a namespace declaration
	      	  } else {
	            match new.add_attribute(parent.new_attribute_node(qn.clone(), v.clone())) {
        	      Ok(_) => {}
		      Err(_) => {
		        status = Some(String::from("unable to add attribute"))
		      }
      		    };
	      	  }
		}
		_ => {
		  match new.add_attribute(parent.new_attribute_node(qn.clone(), v.clone())) {
        	      Ok(_) => {}
		      Err(_) => {
		        status = Some(String::from("unable to add attribute"))
		      }
		  };
		}
	      }
	    }
	    _ => {}, // shouldn't happen
	  }
	});

      if status.is_some() {
        return Result::Err(Error{kind: ErrorKind::Unknown, message: status.unwrap()})
      }

      c.iter().cloned().for_each(|f| {
        match parse_node(&f, new.clone(), ns) {
	  Ok(_) => {}
	  Err(e) => {
	    status = Some(e.to_string())
	  }
	}
      });

      if status.is_some() {
        return Result::Err(Error{kind: ErrorKind::Unknown, message: status.unwrap()})
      }
    }
    XMLNode::Text(v) => {
      let u = parent.new_value_node(v.clone());
      match parent.append_child_node(u) {
        Ok(_) => {}
	Err(_) => return Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("unable to append child node")})
      }
    }
    _ => {
      // TODO: Not yet implemented
    }
  }
  Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_doc() {
        let t = Rc::new(RefCell::new(StableGraph::new()));
	XDMTreeNode::new(t.clone());
	assert_eq!(t.borrow().node_count(), 1);
    }

    #[test]
    fn new_element() {
        let t = Rc::new(RefCell::new(StableGraph::new()));
	let d = XDMTreeNode::new(t.clone());
	let r = d.new_element_node(QualifiedName::new(None, None, "Test".to_string()));
	d.append_child_node(r).expect("unable to append child node");
	assert_eq!(d.get_doc().borrow().node_count(), 2);
	assert_eq!(d.to_xml_int(None, 0), "<Test></Test>");
    }

    #[test]
    fn new_value() {
        let t = Rc::new(RefCell::new(StableGraph::new()));
	let d = XDMTreeNode::new(t.clone());
	let r = d.new_element_node(QualifiedName::new(None, None, "Test".to_string()));
	d.append_child_node(r.clone()).expect("unable to append child node");
	let u = d.new_value_node(Value::String("this is a test".to_string()));
	r.append_child_node(u).expect("unable to append child node");
	assert_eq!(t.borrow().node_count(), 3);
	assert_eq!(d.to_xml_int(None, 0), "<Test>this is a test</Test>");
    }

    #[test]
    fn multi_elements() {
        let t = Rc::new(RefCell::new(StableGraph::new()));
	let d = XDMTreeNode::new(t.clone());
	let r = d.new_element_node(QualifiedName::new(None, None, "Test".to_string()));
	d.append_child_node(r.clone()).expect("unable to append child node");
	let c1 = d.new_element_node(QualifiedName::new(None, None, "Data".to_string()));
	let u1 = d.new_value_node(Value::String("one".to_string()));
	c1.append_child_node(u1).expect("unable to append child node");
	r.append_child_node(c1.clone()).expect("unable to append child node");
	let c2 = d.new_element_node(QualifiedName::new(None, None, "Data".to_string()));
	let u2 = d.new_value_node(Value::String("two".to_string()));
	c2.append_child_node(u2).expect("unable to append child node");
	r.append_child_node(c2.clone()).expect("unable to append child node");
	assert_eq!(t.borrow().node_count(), 6);
	assert_eq!(d.to_xml_int(None, 0), "<Test><Data>one</Data><Data>two</Data></Test>");
    }

    #[test]
    fn children() {
        let t = Rc::new(RefCell::new(StableGraph::new()));
	let d = XDMTreeNode::new(t.clone());
	let r = d.new_element_node(QualifiedName::new(None, None, "Test".to_string()));
	d.append_child_node(r.clone()).expect("unable to append child node");
	let c1 = d.new_element_node(QualifiedName::new(None, None, "Data".to_string()));
	let u1 = d.new_value_node(Value::String("one".to_string()));
	c1.append_child_node(u1).expect("unable to append child node");
	r.append_child_node(c1.clone()).expect("unable to append child node");
	let c2 = d.new_element_node(QualifiedName::new(None, None, "Data".to_string()));
	let u2 = d.new_value_node(Value::String("two".to_string()));
	c2.append_child_node(u2).expect("unable to append child node");
	r.append_child_node(c2.clone()).expect("unable to append child node");

	assert_eq!(r.child_iter().collect::<Vec<XDMTreeNode>>().len(), 2);
    }
    #[test]
    fn descend() {
        let t = Rc::new(RefCell::new(StableGraph::new()));
	let d = XDMTreeNode::new(t.clone());
	let r = d.new_element_node(QualifiedName::new(None, None, "Test".to_string()));
	d.append_child_node(r.clone()).expect("unable to append child node");
	let c1 = d.new_element_node(QualifiedName::new(None, None, "Data".to_string()));
	r.append_child_node(c1.clone()).expect("unable to append child node");
	let c2 = d.new_element_node(QualifiedName::new(None, None, "Data".to_string()));
	c1.append_child_node(c2.clone()).expect("unable to append child node");
	let c3 = d.new_element_node(QualifiedName::new(None, None, "Data".to_string()));
	c2.append_child_node(c3.clone()).expect("unable to append child node");

	assert_eq!(r.to_xml_int(None, 0), "<Test><Data><Data><Data></Data></Data></Data></Test>");
    }

    #[test]
    fn siblings() {
        let t = Rc::new(RefCell::new(StableGraph::new()));
	let d = XDMTreeNode::new(t.clone());
	let r = d.new_element_node(QualifiedName::new(None, None, "Test".to_string()));
	d.append_child_node(r.clone()).expect("unable to append child node");
	let c1 = d.new_element_node(QualifiedName::new(None, None, "Data".to_string()));
	let u1 = d.new_value_node(Value::String("one".to_string()));
	c1.append_child_node(u1).expect("unable to append child node");
	r.append_child_node(c1.clone()).expect("unable to append child node");
	let c2 = d.new_element_node(QualifiedName::new(None, None, "Data".to_string()));
	let u2 = d.new_value_node(Value::String("two".to_string()));
	c2.append_child_node(u2).expect("unable to append child node");
	r.append_child_node(c2.clone()).expect("unable to append child node");

	assert_eq!(c1.sibling_iter().collect::<Vec<XDMTreeNode>>().len(), 1);
    }

    #[test]
    fn preceding_siblings() {
        let t = Rc::new(RefCell::new(StableGraph::new()));
	let d = XDMTreeNode::new(t.clone());
	let r = d.new_element_node(QualifiedName::new(None, None, "Test".to_string()));
	d.append_child_node(r.clone()).expect("unable to append child node");
	let c1 = d.new_element_node(QualifiedName::new(None, None, "Data".to_string()));
	let u1 = d.new_value_node(Value::String("one".to_string()));
	c1.append_child_node(u1).expect("unable to append child node");
	r.append_child_node(c1.clone()).expect("unable to append child node");
	let c2 = d.new_element_node(QualifiedName::new(None, None, "Data".to_string()));
	let u2 = d.new_value_node(Value::String("two".to_string()));
	c2.append_child_node(u2).expect("unable to append child node");
	r.append_child_node(c2.clone()).expect("unable to append child node");

	assert_eq!(c2.preceding_sibling_iter().collect::<Vec<XDMTreeNode>>().len(), 1);
    }

    #[test]
    fn ancestors() {
        let t = Rc::new(RefCell::new(StableGraph::new()));
	let d = XDMTreeNode::new(t.clone());
	let r = d.new_element_node(QualifiedName::new(None, None, "Test".to_string()));
	d.append_child_node(r.clone()).expect("unable to append child node");
	let c1 = d.new_element_node(QualifiedName::new(None, None, "Data".to_string()));
	let u1 = d.new_value_node(Value::String("one".to_string()));
	c1.append_child_node(u1).expect("unable to append child node");
	r.append_child_node(c1.clone()).expect("unable to append child node");
	let c2 = d.new_element_node(QualifiedName::new(None, None, "Data".to_string()));
	let u2 = d.new_value_node(Value::String("two".to_string()));
	c2.append_child_node(u2.clone()).expect("unable to append child node");
	r.append_child_node(c2.clone()).expect("unable to append child node");

	assert_eq!(u2.ancestor_iter().collect::<Vec<XDMTreeNode>>().len(), 2);
    }

    #[test]
    fn attribute() {
        let t = Rc::new(RefCell::new(StableGraph::new()));
	let d = XDMTreeNode::new(t.clone());
	let r = d.new_element_node(QualifiedName::new(None, None, "Test".to_string()));
	d.append_child_node(r.clone()).expect("unable to append child node");
	r.add_attribute(
	  r.new_attribute_node(
	    QualifiedName::new(None, None, "status".to_string()),
	    Value::String("testing".to_string())
	  )
	).expect("unable to add attribute");

	assert_eq!(d.to_xml_int(None, 0), "<Test status='testing'></Test>");
    }

    #[test]
    fn get_attribute() {
        let t = Rc::new(RefCell::new(StableGraph::new()));
	let d = XDMTreeNode::new(t.clone());
	let r = d.new_element_node(QualifiedName::new(None, None, "Test".to_string()));
	d.append_child_node(r.clone()).expect("unable to append child node");
	r.add_attribute(
	  r.new_attribute_node(
	    QualifiedName::new(None, None, "status".to_string()),
	    Value::String("testing".to_string())
	  )
	).expect("unable to add attribute");

	assert_eq!(d.get_first_child().unwrap().get_attribute(&QualifiedName::new(None, None, "status".to_string())).unwrap().to_string(), "testing");
    }

    #[test]
    fn attribute_iter() {
        let t = Rc::new(RefCell::new(StableGraph::new()));
	let d = XDMTreeNode::new(t.clone());
	let r = d.new_element_node(QualifiedName::new(None, None, "Test".to_string()));
	d.append_child_node(r.clone()).expect("unable to append child node");
	r.add_attribute(r.new_attribute_node(
	  QualifiedName::new(None, None, "status".to_string()),
	  Value::String("testing".to_string())
	)).expect("unable to add attribute");
	r.add_attribute(r.new_attribute_node(
	  QualifiedName::new(None, None, "mode".to_string()),
	  Value::String("test".to_string())
	)).expect("unable to add attribute");
	let mut atts = r.attr_node_iter();
	let at1 = atts.next().unwrap();
	assert!(
	  at1.get_name().get_localname() == "status" ||
	  at1.get_name().get_localname() == "mode"
	);
	let at2 = atts.next().unwrap();
	assert!(
	  at2.get_name().get_localname() == "status" ||
	  at2.get_name().get_localname() == "mode"
	);

	match atts.next() {
	  None => {}
	  Some(_) => panic!("unexpected result")
	}
    }

    // Parsing XML

    #[test]
    fn parse_empty() {
      let r = from("<Test/>").expect("unable to parse \"<Test/>\"");

      let c = r.child_iter().collect::<Vec<XDMTreeNode>>();
      assert_eq!(c.len(), 1);
      assert_eq!(c[0].to_xml_int(None, 0), "<Test></Test>");
    }

    #[test]
    fn parse_empty_qualified() {
      let r = from("<x:Test xmlns:x='urn:my-test'/>").expect("unable to parse \"<x:Test xlmns:x='urn:my-test'/>\"");

      let c = r.child_iter().collect::<Vec<XDMTreeNode>>();
      assert_eq!(c.len(), 1);
      assert_eq!(c[0].to_xml_int(None, 0), "<x:Test xmlns:x='urn:my-test'></x:Test>");
    }

    #[test]
    fn parse_text() {
      let r = from("<Test>foobar</Test>").expect("unable to parse \"<Test><foobar</Test>\"");

      let c = r.child_iter().collect::<Vec<XDMTreeNode>>();
      assert_eq!(c.len(), 1);
      assert_eq!(c[0].to_xml_int(None, 0), "<Test>foobar</Test>");
    }

    #[test]
    fn parse_element_children() {
      let r = from("<Test><a/><b/><c/></Test>").expect("unable to parse \"<Test><a/><b/><c/></Test>\"");

      let c = r.child_iter().collect::<Vec<XDMTreeNode>>();
      assert_eq!(c.len(), 1);
      assert!(c[0].to_xml_int(None, 0) == "<Test><a/><b/><c/></Test>" ||
        c[0].to_xml_int(None, 0) == "<Test><a></a><b></b><c></c></Test>"
      );
    }

    #[test]
    fn parse_mixed() {
      let r = from("<Test>i1<child>one</child>i2<child>two</child>i3</Test>").expect("unable to parse \"<Test>i1<child>one</child>i2<child>two</child>i3</Test>\"");

      let c = r.child_iter().collect::<Vec<XDMTreeNode>>();
      assert_eq!(c.len(), 1);
      assert_eq!(c[0].to_xml_int(None, 0), "<Test>i1<child>one</child>i2<child>two</child>i3</Test>");
    }

    // Change structure

    #[test]
    fn remove_1() {
      let r = from("<Test><a><b/></a></Test>").expect("unable to parse XML");
      let c = r.get_first_child().unwrap().get_first_child().unwrap().get_first_child().unwrap();
      c.remove_node();
      assert_eq!(r.to_xml_int(None, 0), "<Test><a></a></Test>");
    }

    #[test]
    fn remove_2() {
      let r = from("<Test><a><b att1='val1'/></a></Test>").expect("unable to parse XML");
      let c = r.get_first_child().unwrap().get_first_child().unwrap().get_first_child().unwrap();
      c.remove_node();
      assert_eq!(r.to_xml_int(None, 0), "<Test><a></a></Test>");
    }

    #[test]
    fn remove_3() {
      let r = from("<Test><a><b><c/></b></a></Test>").expect("unable to parse XML");
      let c = r.get_first_child().unwrap().get_first_child().unwrap().get_first_child().unwrap();
      c.remove_node();
      assert_eq!(r.to_xml_int(None, 0), "<Test><a></a></Test>");
    }

    #[test]
    fn remove_4() {
      let r = from("<Test><a><b/><c/></a></Test>").expect("unable to parse XML");
      let c = r.get_first_child().unwrap().get_first_child().unwrap().get_first_child().unwrap();
      c.remove_node();
      assert_eq!(r.to_xml_int(None, 0), "<Test><a><c></c></a></Test>");
    }

    #[test]
    fn remove_5() {
      let r = from("<Test><a><p/><b/></a></Test>").expect("unable to parse XML");
      let c = r.get_first_child().unwrap().get_first_child().unwrap().child_iter().nth(1).unwrap();
      c.remove_node();
      assert_eq!(r.to_xml_int(None, 0), "<Test><a><p></p></a></Test>");
    }

    #[test]
    fn remove_6() {
      let r = from("<Test><a><p/><b/><c/></a></Test>").expect("unable to parse XML");
      let c = r.get_first_child().unwrap().get_first_child().unwrap().child_iter().nth(1).unwrap();
      c.remove_node();
      assert_eq!(r.to_xml_int(None, 0), "<Test><a><p></p><c></c></a></Test>");
    }
}
