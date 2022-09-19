//! # A tree structure for XDM
//!
//! Uses interior mutability to create and manage a tree structure that is both mutable and fully navigable.

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::borrow::BorrowMut;
//use std::collections::HashMap;
use crate::xdmerror::*;
use crate::qname::*;
use crate::output::OutputDefinition;
use crate::value::Value;
use crate::item::{NodeType, INode, MNode};
use crate::parsexml::content;

/// A node in a tree.
#[derive(Clone, Default)]
pub struct Node {
    node_type: NodeType,
    parent: RefCell<Option<Weak<Node>>>,
    children: RefCell<Vec<RNode>>,
    // TODO: attributes
    name: Option<QualifiedName>,
    value: Option<Value>,
}

impl Node {
    /// Create an empty, unattached node
    fn new(n: NodeType) -> Self {
	Node{
	    node_type: n,
	    parent: RefCell::new(None),
	    children: RefCell::new(vec![]),
	    ..Default::default()
	}
    }
}

pub type RNode = Rc<Node>;

//pub trait NodeTrait {
//    fn push(&self, n: RNode) -> Result<(), Error>;
//    fn to_xml(&self) -> String;
//}

impl INode for RNode {
    type NodeIterator = Box<dyn Iterator<Item = RNode>>;
    type Mutable = RNode;

    fn node_type(&self) -> NodeType {
	self.node_type.clone()
    }
    fn name(&self) -> QualifiedName {
	self.name.as_ref().map_or(
	    QualifiedName::new(None, None, String::new()),
	    |n| n.clone()
	)
    }
    fn value(&self) -> Value {
	self.value.as_ref().map_or(
	    Value::from(""),
	    |v| v.clone(),
	)
    }
    fn to_string(&self) -> String {
	String::from("not yet implemented")
    }
    /// Serialise as XML
    fn to_xml(&self) -> String {
	make_xml(self)
    }
    /// Serialise the node as XML, with options such as indentation.
    fn to_xml_with_options(&self, _od: &OutputDefinition) -> String {
	String::from("not implemented")
    }

    fn to_mnode(&self) -> Self::Mutable {
	self.clone()
    }

    fn child_iter(&self) -> Self::NodeIterator {
	Box::new(Children::new(self))
    }
    fn ancestor_iter(&self) -> Self::NodeIterator {
	Box::new(Ancestors::new(self))
    }
    fn descend_iter(&self) -> Self::NodeIterator {
	Box::new(Descendants::new(self))
    }
    fn next_iter(&self) -> Self::NodeIterator {
	Box::new(Siblings::new(self, 1))
    }
    fn prev_iter(&self) -> Self::NodeIterator {
	Box::new(Siblings::new(self, -1))
    }
    fn attribute_iter(&self) -> Self::NodeIterator {
	Box::new(Attributes::new(self))
    }
}

fn make_xml(s: &RNode) -> String {
    match s.node_type {
	NodeType::Document => {
	    s.children.borrow().iter()
		.fold(
		    String::new(),
		    |mut result, c| {
			result.push_str(make_xml(c).as_str());
			result
		    }
		)
	}
	NodeType::Element => {
	    let mut result = String::from("<");
	    result.push_str(
		s.name.as_ref().map_or(
		    String::new(),
		    |n| n.to_string()
		).as_str()
	    );
	    result.push_str(">");
	    s.children.borrow().iter()
		.for_each(|c| {
		    result.push_str(make_xml(c).as_str())
		});
	    result.push_str("</");
	    result.push_str(
		s.name.as_ref().map_or(
		    String::new(),
		    |n| n.to_string()
		).as_str()
	    );
	    result.push_str(">");
	    result
	}
	NodeType::Text => {
	    INode::value(s).to_string()
	}
	_ => String::new()
    }
}

impl MNode for RNode {
    type NodeIterator = Box<dyn Iterator<Item = RNode>>;
    type Immutable = RNode;

    /// Append a node to the child list
    fn push(&mut self, n: RNode) -> Result<(), Error> {
	*n.parent.borrow_mut() = Some(Rc::downgrade(self));
	self.children.borrow_mut().push(n);
	Ok(())
    }
    /// Add an attribute to this element-type node
    fn add_attribute(&mut self, att: Self) -> Result<(), Error> {
	Result::Err(Error::new(ErrorKind::NotImplemented, String::from("not yet implemented")))
    }

    /// An iterator over the children of the node
    fn child_iter(&self) -> Self::NodeIterator {
	Box::new(Children::new(self))
    }

    fn new_element(&self, qn: QualifiedName) -> Result<Self, Error> {
	Ok(NodeBuilder::new(NodeType::Element)
	   .name(qn)
	   .build()
	)
    }
    fn new_text(&self, v: Value) -> Result<Self, Error> {
	Ok(NodeBuilder::new(NodeType::Text)
	   .value(v)
	   .build()
	)
    }
    fn new_attribute(&self, qn: QualifiedName, v: Value) -> Result<Self, Error> {
	Ok(NodeBuilder::new(NodeType::Attribute)
	   .name(qn)
	   .value(v)
	   .build()
	)
    }

    fn node_type(&self) -> NodeType {
	self.node_type.clone()
    }
    fn name(&self) -> QualifiedName {
	self.name.as_ref().map_or(
	    QualifiedName::new(None, None, String::new()),
	    |n| n.clone()
	)
    }
    fn value(&self) -> Value {
	self.value.as_ref().map_or(
	    Value::from(""),
	    |v| v.clone(),
	)
    }
    fn to_string(&self) -> String {
	String::from("not yet implemented")
    }
    fn to_xml(&self) -> String {
	make_xml(self)
    }
    fn to_xml_with_options(&self, _od: &OutputDefinition) -> String {
	String::from("not yet implemented")
    }
}

pub struct Children {
    v: Vec<RNode>,
    i: usize,
}
impl Children {
    fn new(n: &RNode) -> Self {
	match INode::node_type(n) {
	    NodeType::Element => {
		Children{v: n.children.borrow().clone(), i: 0}
	    }
	    _ => {
		Children{v: vec![], i: 0}
	    }
	}
    }
}
impl Iterator for Children {
    type Item = RNode;

    fn next(&mut self) -> Option<RNode> {
	match self.v.get(self.i) {
	    Some(c) => {
		self.i += 1;
		Some(c.clone())
	    }
	    None => None,
	}
    }
}

pub struct Ancestors {
    cur: RNode,
}

impl Ancestors {
    fn new(n: &RNode) -> Self {
	Ancestors{cur: n.clone()}
    }
}

impl Iterator for Ancestors {
    type Item = RNode;

    fn next(&mut self) -> Option<RNode> {
	let s = self.cur.clone();
	let p = s.parent.borrow();
	match &*p {
	    None => None,
	    Some(q) => {
		match Weak::upgrade(&q) {
		    None => None,
		    Some(r) => {
			self.cur = r.clone();
			Some(r)
		    }
		}
	    }
	}
    }
}

// This implementation eagerly constructs a list of nodes
// to traverse.
// An alternative would be to lazily traverse the descendants.
pub struct Descendants{
    v: Vec<RNode>,
    cur: usize,
}
impl Descendants {
    fn new(n: &RNode) -> Self {
	Descendants{
	    v: n.children.borrow().iter()
		.fold(
		    vec![],
		    |mut acc, c| {
			let mut d = descendant_add(c);
			acc.append(&mut d);
			acc
		    }
		),
	    cur: 0,
	}
    }
}
fn descendant_add(n: &RNode) -> Vec<RNode> {
    let mut result = vec![n.clone()];
    n.children.borrow().iter()
	.for_each(|c| {
	    let mut l = descendant_add(c);
	    result.append(&mut l);
	});
    result
}
impl Iterator for Descendants {
    type Item = RNode;

    fn next(&mut self) -> Option<RNode> {
	match self.v.get(self.cur) {
	    Some(n) => {
		self.cur += 1;
		Some(n.clone())
	    }
	    None => None,
	}
    }
}

pub struct Siblings(RNode);
impl Siblings {
    fn new(n: &RNode, _dir: i32) -> Self {
	Siblings(n.clone())
    }
}
impl Iterator for Siblings {
    type Item = RNode;

    // TODO
    fn next(&mut self) -> Option<RNode> {
	None
    }
}

pub struct Attributes(RNode);
impl Attributes {
    fn new(n: &RNode) -> Self {
	Attributes(n.clone())
    }
}
impl Iterator for Attributes {
    type Item = RNode;

    // TODO
    fn next(&mut self) -> Option<RNode> {
	None
    }
}

pub struct NodeBuilder(Node);

impl NodeBuilder {
    pub fn new(n: NodeType) -> Self {
	NodeBuilder(Node::new(n))
    }
    pub fn name(mut self, qn: QualifiedName) -> Self {
	self.0.name = Some(qn);
	self
    }
    pub fn value(mut self, v: Value) -> Self {
	self.0.value = Some(v);
	self
    }
    pub fn build(self) -> Rc<Node> {
	Rc::new(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_push() {
	let mut root = NodeBuilder::new(NodeType::Document)
	    .build();
	let child = NodeBuilder::new(NodeType::Element)
	    .name(QualifiedName::new(None, None, String::from("Test")))
	    .build();
	root.push(child);
	assert_eq!(INode::to_xml(&root), "<Test></Test>")
    }

    #[test]
    fn child_iter() {
	let mut root = NodeBuilder::new(NodeType::Document)
	    .build();
	let mut child = NodeBuilder::new(NodeType::Element)
	    .name(QualifiedName::new(None, None, String::from("Test")))
	    .build();
	root.push(child.clone());
	(1..=5).for_each(|i| {
	    let mut l1 = NodeBuilder::new(NodeType::Element)
		.name(QualifiedName::new(None, None, String::from("Level1")))
		.build();
	    child.push(l1.clone());
	    l1.push(
		NodeBuilder::new(NodeType::Text)
		    .value(Value::from(i))
		    .build()
	    );
	});
	assert_eq!(INode::to_xml(&root), "<Test><Level1>1</Level1><Level1>2</Level1><Level1>3</Level1><Level1>4</Level1><Level1>5</Level1></Test>")
    }
}
