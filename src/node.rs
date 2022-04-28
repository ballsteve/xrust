//! # xrust::node
//!
//! A node in a document tree.

use generational_arena::{Arena, Index};
use std::collections::HashMap;
use crate::qname::QualifiedName;
use crate::output::OutputDefinition;
use crate::xdmerror::{Error, ErrorKind};
use crate::value::Value;

/// A Forest. Forests contain [Tree]s.
//pub struct Forest {
//    a: Arena<NodeContent>,
//    f: Vec<Index>,	// The document node for each Tree
//}

/// A Tree, using an Arena Allocator.
/// Nodes can be detached, but not deleted
#[derive(Clone)]
pub struct Tree {
    a: Arena<NodeContent>,
    d: Index,	// The document node
}

impl Tree
{
    pub fn new() -> Self {
        let mut a = Arena::new();
	let d = a.insert(
	    NodeBuilder::new(NodeType::Document).build()
	);
	Tree {
            a: a,
	    d: d,
        }
    }
    fn get(&self, i: Index) -> Option<&NodeContent> {
	self.a.get(i)
    }
    fn get_mut(&mut self, i: Index) -> Option<&mut NodeContent> {
	self.a.get_mut(i)
    }
    fn get_doc_node(&self) -> Index {
	self.d
    }
    fn push_doc_node(&mut self, n: Node) -> Result<(), Error> {
	// Set the parent to the document node
	self.get_mut(n.0).unwrap().parent = Some(Node::from(self.d));
	// Push the node onto the doc node's children
	self.get_mut(self.d)
	    .map_or_else(
		|| Result::Err(Error::new(ErrorKind::Unknown, String::from("no document node"))),
		|e| {
		    e.children.push(n);
		    Ok(())
		}
	    )
    }
    fn insert(&mut self, nc: NodeContent) -> Index {
	self.a.insert(nc)
    }

    pub fn new_element(&mut self, name: QualifiedName) -> Result<Node, Error> {
	Ok(
	    Node::from(self.a
		 .insert(NodeBuilder::new(NodeType::Element).name(name).build())
	    )
	)
    }
    pub fn new_text(&mut self, c: Value) -> Result<Node, Error> {
	Ok(
	    Node::from(self.a
		 .insert(NodeBuilder::new(NodeType::Text).value(c).build())
	    )
	)
    }
    pub fn new_attribute(&mut self, name: QualifiedName, v: Value) -> Result<Node, Error> {
	Ok(
	    Node::from(self.a
		       .insert(
			   NodeBuilder::new(NodeType::Attribute)
			       .name(name)
			       .value(v)
			       .build()
		       )
	    )
	)
    }
    pub fn new_comment(&mut self, v: Value) -> Result<Node, Error> {
        Ok(
	    Node::from(self.a
		       .insert(NodeBuilder::new(NodeType::Comment).value(v).build())
	    )
	)
    }
    pub fn new_processing_instruction(&mut self, name: QualifiedName, v: Value) -> Result<Node, Error> {
        Ok(
	    Node::from(self.a
		       .insert(
			   NodeBuilder::new(NodeType::ProcessingInstruction)
			       .name(name)
			       .value(v)
			       .build()
		       )
	    )
	)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum NodeType {
  Document,
  Element,
  Text,
  Attribute,
  Comment,
  ProcessingInstruction,
  Unknown,
}

impl NodeType {
  pub fn to_string(&self) -> &'static str {
    match self {
      NodeType::Document => "Document",
      NodeType::Element => "Element",
      NodeType::Attribute => "Attribute",
      NodeType::Text => "Text",
      NodeType::ProcessingInstruction => "Processing-Instruction",
      NodeType::Comment => "Comment",
      NodeType::Unknown => "--None--",
    }
  }
}

impl Default for NodeType {
  fn default() -> Self {
    NodeType::Unknown
  }
}

/// Node
#[derive(Copy, Clone, Debug)]
pub struct Node(Index);

impl From<Index> for Node {
    fn from(i: Index) -> Self {
	Node(i)
    }
}

impl Node {
    fn get<'a>(&self, d: &'a Tree) -> Option<&'a NodeContent> {
	d.get(self.0)
    }
    fn get_mut<'a>(&self, d: &'a mut Tree) -> Option<&'a mut NodeContent> {
	d.get_mut(self.0)
    }

    pub fn to_string(&self, _d: &Tree) -> String {
	String::from("not implemented yet (document)")
    }
    pub fn to_xml(&self, d: &Tree) -> String {
	let nc = d.get(self.0).unwrap();	// TODO: Don't Panic
	match nc.node_type() {
	    NodeType::Element => {
		let mut result = String::from("<");
		let name = nc.name().as_ref().unwrap();
		result.push_str(name.to_string().as_str());
		result.push_str(">");
		// TODO: iterate over children
		result.push_str("</");
		result.push_str(name.to_string().as_str());
		result.push_str(">");
		result
	    }
	    NodeType::Text => {
		nc.value().as_ref().unwrap().to_string()
	    }
	    _ => {
		// TODO
		String::from("-- not implemented --")
	    }
	}
    }
    pub fn to_xml_with_options(&self, _d: &Tree, _od: &OutputDefinition) -> String {
	String::from("not implemented yet")
    }
    pub fn to_json(&self, _d: &Tree) -> String {
	String::from("not implemented yet")
    }

    pub fn to_int(&self, d: &Tree) -> Result<i64, Error> {
	// Convert to a string, then try parsing that as an integer
	self.to_string(d).parse::<i64>()
	    .map_err(|e| Error::new(ErrorKind::Unknown, e.to_string()))
    }
    pub fn to_double(&self, d: &Tree) -> f64 {
	// Convert to a string, then try parsing that as a double
	match self.to_string(d).parse::<f64>() {
	    Ok(f) => f,
	    Err(_) => f64::NAN,
	}
    }
    pub fn to_name(&self, d: &Tree) -> QualifiedName {
	d.get(self.0)
	    .map_or(
		QualifiedName::new(None, None, String::from("")),
		|o| o.name().as_ref().map_or(
		    QualifiedName::new(None, None, String::from("")),
		    |p| p.clone(),
		)
	    )
    }

    pub fn node_type(&self, d: &Tree) -> NodeType {
	d.get(self.0)
	    .map_or(
		NodeType::Unknown,
		|m| m.node_type(),
	    )
    }

    pub fn append_child(&self, d: &mut Tree, c: Node) -> Result<(), Error> {
	// TODO: Don't Panic

	// Check that self is an element and that c is not an attribute
        if self.node_type(d) != NodeType::Element {
            return Result::Err(Error::new(
                ErrorKind::Unknown,
                String::from("must be an element"),
            ));
        }
        if c.node_type(d) == NodeType::Attribute {
            return Result::Err(Error::new(
                ErrorKind::Unknown,
                String::from("cannot append an attribute as a child"),
            ));
        }

	// TODO: detach c from wherever it currently is located

	// self will now be c's parent
	d.get_mut(c.0).unwrap().parent = Some(self.clone());

	// Push c onto self's child list
        d.get_mut(self.0).unwrap().children.push(c);

        Ok(())
    }
    pub fn insert_before(&mut self, _d: &mut Tree, _insert: Node) -> Result<(), Error> {
        return Result::Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not yet implemented"),
        ));
    }

    pub fn parent(&self, _d: &mut Tree) -> Option<Node> {
	None
//	self.ancestor_iter(n).next(self).map(|p| p)
    }
}

#[derive(Clone, Default)]
pub struct NodeContent {
    t: NodeType,
    name: Option<QualifiedName>,
    v: Option<Value>,
    parent: Option<Node>, // The document node has no parent
    attributes: HashMap<QualifiedName, Node>, // for non-elements nodes this is always. Should this be an Option?
    children: Vec<Node>, // for non-element nodes this is always empty. Should this be an Option?
}

impl NodeContent {
    pub fn new(t: NodeType) -> Self {
        NodeContent {
	    t,
            ..Default::default()
        }
    }
    pub fn node_type(&self) -> NodeType {
	self.t
    }
    pub fn name(&self) -> &Option<QualifiedName> {
        &self.name
    }
    pub fn value(&self) -> &Option<Value> {
	&self.v
    }
}

struct NodeBuilder(NodeContent);

impl NodeBuilder {
    pub fn new(t: NodeType) -> Self {
        NodeBuilder(NodeContent::new(t))
    }
    pub fn name(mut self, qn: QualifiedName) -> Self {
        self.0.name = Some(qn);
        self
    }
    // Q: what to do if the node already has a value?
    // This implementation drops the previous value
    pub fn value(mut self, v: Value) -> Self {
        self.0.v = Some(v);
        self
    }
    pub fn build(self) -> NodeContent {
        self.0
    }
}

/// Nodes
///
/// A document contains [Node] objects.
pub trait NodeTrait {
    /// Return the string value of the [Node]
    fn to_string(&self) -> String;
    /// Serialize the given [Node] as XML
    fn to_xml(&self) -> String;
    /// Serialize as XML, with options
    fn to_xml_with_options(&self, od: &OutputDefinition) -> String;
    /// Serialize as JSON
    fn to_json(&self) -> String;
    /// Determine the effective boolean value. See XPath 2.4.3.
    /// A Document or Node always returns true.
    fn to_bool(&self) -> bool {
	true
    }
    /// Return the integer value. For a Document, this is a type error.
    fn to_int(&self) -> Result<i64, Error>;
    /// Return the double value. For a Document, this is a type error, i.e. NaN.
    fn to_double(&self) -> f64;
    /// Gives the name of the [Node]. Documents do not have a name, so the implementation must return an empty string.
    fn to_name(&self) -> QualifiedName;

    /// Return the type of a Node
    fn node_type(&self) -> NodeType;

    /// Callback for logging/debugging, particularly in a web_sys environment
    fn log(&self, _m: &str) {
	// Noop
    }

    /// Return the root node of the Document.
    //fn get_root_element(&self) -> Option<N>;
    /// Set the root element for the Document. If the Document already has a root element then it will be removed. The node must be an element. If the node supplied is of a different concrete type to the Document then an error is returned. If the element is from a different Document, then the function performs a deep copy.
    //fn set_root_element(&mut self, r: Self::Node) -> Result<(), Error>;

    /// An iterator over ancestors of a [Node].
    //fn ancestor_iter<D: Document<N>>(&self, n: N) -> Box<dyn AncestorIterator<D, N, Item = N>>;
    /// Navigate to the parent of a [Node]. Documents, and the root element, don't have a parent, so the default implementation returns None. This is a convenience function for ancestor_iter.
    fn parent(&self) -> Option<Node> {
	None
    }
    /// An iterator for the child nodes of a [Node]. Non-element type nodes will immediately return None.
    //fn child_iter<D: Document<N>>(&self, n: N) -> Box<dyn ChildIterator<D, N, Item = N>>;
    /// An iterator for the child nodes of the Document. This may include the prologue, root element, and epilogue.
    //fn doc_child_iter<D: Document<N>>(&self) -> Box<dyn DocChildIterator<D, N, Item = N>>;
    /// An iterator for descendants of a [Node]. Does not include the [Node] itself.
    // fn descend_iter(&self, n: Box<dyn Node>) -> Box<dyn Iterator<Item = Box<dyn Node>>>;
    /// An iterator for following siblings of a [Node]. Does not include the [Node] itself.
    // fn following_sibling_iter(&self, n: Box<dyn Node>) -> Box<dyn Iterator<Item = Box<dyn Node>>>;
    /// An iterator for preceding siblings of a [Node]. Does not include the [Node] itself.
    // fn preceding_sibling_iter(&self, n: Box<dyn Node>) -> Box<dyn Iterator<Item = Box<dyn Node>>>;

    /// Create an element [Node] in the Document.
    fn new_element(&mut self, name: QualifiedName) -> Result<Node, Error>;
    /// Create a text [Node] in the Document.
    fn new_text(&mut self, c: Value) -> Result<Node, Error>;
    /// Create an attribute [Node] in the Document.
    fn new_attribute(&mut self, name: QualifiedName, v: Value) -> Result<Node, Error>;
    /// Create a comment [Node] in the Document.
    fn new_comment(&mut self, v: Value) -> Result<Node, Error>;
    /// Create a processing instruction [Node] in the Document.
    fn new_processing_instruction(&mut self, name: QualifiedName, v: Value) -> Result<Node, Error>;

    /// Append a [Node] to the children of a [Node]. If the [Node] to be appended is from a different Document then this function performs a deep copy.
    fn append_child(&mut self, parent: Node, child: Node) -> Result<(), Error>;
    /// Inserts a [Node] (insert) before another [Node] (child) in the children of it's parent element [Node]. If the [Node] to be inserted is from a different Document then this function performs a deep copy.
    fn insert_before(&mut self, child: Node, insert: Node) -> Result<(), Error>;
    // TODO: replace_child

    /// Add an attribute [Node] to an element type [Node]. If the attribute [Node] is from a different Document then this function adds a copy of the attribute [Node].
    fn add_attribute_node(&mut self, _parent: Node, _a: Node) -> Result<(), Error> {
	Result::Err(Error::new(ErrorKind::NotImplemented, String::from("not implemented")))
    }

    /// Remove a node from its parent
    fn remove(&mut self, _n: Node) -> Result<(), Error> {
	Result::Err(Error::new(ErrorKind::NotImplemented, String::from("not implemented")))
    }
}

/// An iterator over ancestor nodes
pub trait AncestorIterator {
    type Node;
    fn next(&mut self, t: Tree) -> Option<Self::Node>;
}

/// An iterator over child nodes
pub trait ChildIterator {
    type Node;
    fn next(&mut self, t: Tree) -> Option<Self::Node>;
}

/// An iterator over child nodes of a [Document]
pub trait DocChildIterator {
    type Node;
    fn next(&mut self, t: Tree) -> Option<Self::Node>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emptydoc() {
	Tree::new();
	assert!(true)
    }

    #[test]
    fn root_element() {
	let mut t = Tree::new();
	let e = t.new_element(QualifiedName::new(None, None, String::from("Test"))).expect("unable to create element node");
	t.push_doc_node(e);
	assert_eq!(e.to_xml(&t), "<Test></Test>")
    }

    #[test]
    fn add_element() {
	let mut t = Tree::new();
	let e = t.new_element(QualifiedName::new(None, None, String::from("Test"))).expect("unable to create element node");
	t.push_doc_node(e);
	let l1 = t.new_element(QualifiedName::new(None, None, String::from("Level-1"))).expect("unable to create element node");
	e.append_child(&mut t, l1);
	assert_eq!(e.to_xml(&t), "<Test><Level-1></Level-1></Test>")
    }
}
