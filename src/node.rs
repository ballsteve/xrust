//! # xrust::node
//!
//! A node in a document tree.

use generational_arena::{Arena, Index};
use std::collections::HashMap;
use crate::qname::QualifiedName;
use crate::output::OutputDefinition;
use crate::xdmerror::{Error, ErrorKind};
use crate::value::Value;
use crate::parsexml::*;

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

impl TryFrom<&str> for Tree {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
	let d = parse(s)?;
	if d.content.len() == 0 {
	    Result::Err(Error::new(ErrorKind::Unknown, String::from("unable to parse XML")))
	} else {
	    let mut ns: HashMap<String, String> = HashMap::new();
	    let mut t = Tree::new();
	    for c in d.content {
		t.doc_push_node(make_node(c, &mut t, &mut ns)?)?;
	    }
	    Ok(t)
	}
    }
}

fn make_node(
    n: XMLNode,
    t: &mut Tree,
    ns: &mut HashMap<String, String>,
) -> Result<Node, Error> {
    match n {
	XMLNode::Element(m, a, c) -> {
	    a.iter()
		.filter(|b| {
		    match b {
			XMLNode::Attribute(qn, _) => {
			    match qn.get_prefix() {
				Some("xmlns") => true,
				_ => false,
			    }
			}
			_ => false,
		    }
		})
		.for_each(|b| {
		    if let XMLNode::Attribute(qn, v) = b {
			// add map from prefix to uri in hashmap
			ns.insert(qn.get_localname(), v.to_string()).map(|| {})
		    }
		});
	    // Add element to the tree
	    let newns = n.get_prefix()
		.map(|p| ns.get(&p).clone());
	    let new = t.new_element(
		QualifiedName::new(
		    newns,
		    n.get_prefix(),
		    n.get_localname(),
		)
	    )?;
	    
	}
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
#[derive(Copy, Clone, PartialEq, Debug)]
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

    pub fn to_string(&self, d: &Tree) -> String {
	match self.node_type(d) {
	    NodeType::Element => {
		// TODO: string value of all descendant text nodes
		String::new()
	    }
	    NodeType::Text |
	    NodeType::Attribute |
	    NodeType::Comment => {
		self.get(d).unwrap().value().as_ref().map_or(
		    String::new(),
		    |v| v.to_string()
		)
	    }
	    _ => String::new(),
	}
    }
    pub fn to_xml(&self, d: &Tree) -> String {
	let nc = d.get(self.0).unwrap();	// TODO: Don't Panic
	match nc.node_type() {
	    NodeType::Element => {
		let mut result = String::from("<");
		let name = nc.name().as_ref().unwrap();
		result.push_str(name.to_string().as_str());
		nc.attributes.iter().for_each(|(k, v)| {
		    result.push(' ');
		    result.push_str(k.to_string().as_str());
		    result.push_str("='");
		    result.push_str(v.to_string(d).as_str());
		    result.push('\'');
		});
		result.push_str(">");
		let mut children = self.child_iter();
		loop {
		    match children.next(d) {
			Some(c) => result.push_str(c.to_xml(d).as_str()),
			None => break,
		    }
		};
		result.push_str("</");
		result.push_str(name.to_string().as_str());
		result.push_str(">");
		result
	    }
	    NodeType::Text => {
		nc.value().as_ref().unwrap().to_string()
	    }
	    NodeType::Comment => {
		let mut result = String::from("<!--");
		result.push_str(nc.value().as_ref().unwrap().to_string().as_str());
		result.push_str("-->");
		result
	    }
	    NodeType::ProcessingInstruction => {
		let mut result = String::from("<?");
		result.push_str(nc.name().as_ref().unwrap().to_string().as_str());
		result.push(' ');
		result.push_str(nc.value().as_ref().unwrap().to_string().as_str());
		result.push_str("?>");
		result
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

	// TODO: detach c from wherever it is currently located

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

    pub fn add_attribute(&self, d: &mut Tree, a: Node) -> Result<(), Error> {
        if self.node_type(d) != NodeType::Element {
            return Result::Err(Error::new(
                ErrorKind::Unknown,
                String::from("must be an element"),
            ));
        }
        if a.node_type(d) != NodeType::Attribute {
            return Result::Err(Error::new(
                ErrorKind::Unknown,
                String::from("argument must be an attribute"),
            ));
        }

	// TODO: detach a from wherever it is currently located

	// self will now be a's parent
	d.get_mut(a.0).unwrap().parent = Some(self.clone());
	// Add a to self's attribute hashmap
	let qn = d.get(a.0).unwrap().name().as_ref().unwrap().clone();
	d.get_mut(self.0).unwrap().attributes.insert(qn, a);
	Ok(())
    }

    pub fn ancestor_iter(&self) -> Ancestors {
	Ancestors::new(self.0)
    }
    pub fn parent(&self, d: &Tree) -> Option<Node> {
	self.ancestor_iter().next(d).map(|p| p)
    }
    pub fn child_iter(&self) -> Children {
	Children::new(self.0)
    }
}

pub struct Ancestors {
    cur: Index,
}

impl Ancestors {
    fn new(cur: Index) -> Ancestors {
	Ancestors{cur}
    }
    pub fn next(&mut self, d: &Tree) -> Option<Node> {
	if let Some(c) = d.get(self.cur) {
	    if let Some(p) = c.parent {
		if p.node_type(d) == NodeType::Document {
		    None
		} else {
		    self.cur = p.0;
		    Some(p)
		}
	    } else {
		None
	    }
	} else {
	    None
	}
    }
}

pub struct Children {
    parent: Index,
    cur: usize,
}

impl Children {
    fn new(parent: Index) -> Children {
	Children{parent, cur: 0}
    }
    pub fn next(&mut self, d: &Tree) -> Option<Node> {
	if let Some(n) = d.get(self.parent) {
	    if n.children.len() > self.cur {
		self.cur += 1;
		Some(n.children[self.cur - 1])
	    } else {
		None
	    }
	} else {
	    None
	}
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
	t.push_doc_node(e).expect("unable to add node to doc");
	assert_eq!(e.to_xml(&t), "<Test></Test>")
    }

    #[test]
    fn add_element() {
	let mut t = Tree::new();
	let e = t.new_element(QualifiedName::new(None, None, String::from("Test"))).expect("unable to create element node");
	t.push_doc_node(e).expect("unable to add node to doc");
	let l1 = t.new_element(QualifiedName::new(None, None, String::from("Level-1"))).expect("unable to create element node");
	e.append_child(&mut t, l1).expect("unable to append node");
	assert_eq!(e.to_xml(&t), "<Test><Level-1></Level-1></Test>")
    }

    #[test]
    fn add_text() {
	let mut t = Tree::new();
	let e = t.new_element(QualifiedName::new(None, None, String::from("Test"))).expect("unable to create element node");
	t.push_doc_node(e).expect("unable to add node to doc");
	let l1 = t.new_element(QualifiedName::new(None, None, String::from("Level-1"))).expect("unable to create element node");
	e.append_child(&mut t, l1).expect("unable to append node");
	let txt = t.new_text(Value::from("this is a test")).expect("unable to create text node");
	l1.append_child(&mut t, txt).expect("unable to append node");
	assert_eq!(e.to_xml(&t), "<Test><Level-1>this is a test</Level-1></Test>")
    }

    #[test]
    fn add_attribute() {
	let mut t = Tree::new();
	let e = t.new_element(QualifiedName::new(None, None, String::from("Test"))).expect("unable to create element node");
	t.push_doc_node(e).expect("unable to add node to doc");
	let l1 = t.new_element(QualifiedName::new(None, None, String::from("Level-1"))).expect("unable to create element node");
	e.append_child(&mut t, l1).expect("unable to append node");
	let txt = t.new_attribute(QualifiedName::new(None, None, String::from("data")), Value::from("this is a test")).expect("unable to create text node");
	l1.add_attribute(&mut t, txt).expect("unable to add attribute");
	assert_eq!(e.to_xml(&t), "<Test><Level-1 data='this is a test'></Level-1></Test>")
    }

    #[test]
    fn add_comment() {
	let mut t = Tree::new();
	let e = t.new_element(QualifiedName::new(None, None, String::from("Test"))).expect("unable to create element node");
	t.push_doc_node(e).expect("unable to add node to doc");
	let l1 = t.new_element(QualifiedName::new(None, None, String::from("Level-1"))).expect("unable to create element node");
	e.append_child(&mut t, l1).expect("unable to append node");
	let c = t.new_comment(Value::from("this is a comment")).expect("unable to create comment node");
	l1.append_child(&mut t, c).expect("unable to append node");
	assert_eq!(e.to_xml(&t), "<Test><Level-1><!--this is a comment--></Level-1></Test>")
    }

    #[test]
    fn add_pi() {
	let mut t = Tree::new();
	let e = t.new_element(QualifiedName::new(None, None, String::from("Test"))).expect("unable to create element node");
	t.push_doc_node(e).expect("unable to add node to doc");
	let l1 = t.new_element(QualifiedName::new(None, None, String::from("Level-1"))).expect("unable to create element node");
	e.append_child(&mut t, l1).expect("unable to append node");
	let pi = t.new_processing_instruction(QualifiedName::new(None, None, String::from("testPI")), Value::from("this is a PI")).expect("unable to create processing instruction node");
	l1.append_child(&mut t, pi).expect("unable to append node");
	assert_eq!(e.to_xml(&t), "<Test><Level-1><?testPI this is a PI?></Level-1></Test>")
    }

    #[test]
    fn children() {
	let mut t = Tree::new();
	let e = t.new_element(QualifiedName::new(None, None, String::from("Test"))).expect("unable to create element node");
	t.push_doc_node(e).expect("unable to add node to doc");
	let l1 = t.new_element(QualifiedName::new(None, None, String::from("Level-1"))).expect("unable to create element node");
	e.append_child(&mut t, l1).expect("unable to append node");
	let t1 = t.new_text(Value::from("one")).expect("unable to create text node");
	l1.append_child(&mut t, t1).expect("unable to append node");
	let l2 = t.new_element(QualifiedName::new(None, None, String::from("Level-1"))).expect("unable to create element node");
	e.append_child(&mut t, l2).expect("unable to append node");
	let t2 = t.new_text(Value::from("two")).expect("unable to create text node");
	l2.append_child(&mut t, t2).expect("unable to append node");

	assert_eq!(e.to_xml(&t), "<Test><Level-1>one</Level-1><Level-1>two</Level-1></Test>");

	let mut children = e.child_iter();
	assert_eq!(children.next(&t), Some(l1));
	assert_eq!(children.next(&t), Some(l2));
	assert_eq!(children.next(&t), None)
    }

    #[test]
    fn ancestors() {
	let mut t = Tree::new();
	let e = t.new_element(QualifiedName::new(None, None, String::from("Test"))).expect("unable to create element node");
	t.push_doc_node(e).expect("unable to add node to doc");
	let l1 = t.new_element(QualifiedName::new(None, None, String::from("Level-1"))).expect("unable to create element node");
	e.append_child(&mut t, l1).expect("unable to append node");
	let t1 = t.new_text(Value::from("one")).expect("unable to create text node");
	l1.append_child(&mut t, t1).expect("unable to append node");
	let l2 = t.new_element(QualifiedName::new(None, None, String::from("Level-1"))).expect("unable to create element node");
	e.append_child(&mut t, l2).expect("unable to append node");
	let t2 = t.new_text(Value::from("two")).expect("unable to create text node");
	l2.append_child(&mut t, t2).expect("unable to append node");

	assert_eq!(e.to_xml(&t), "<Test><Level-1>one</Level-1><Level-1>two</Level-1></Test>");

	let mut ancestors = t2.ancestor_iter();
	assert_eq!(ancestors.next(&t), Some(l2));
	assert_eq!(ancestors.next(&t), Some(e));
	assert_eq!(ancestors.next(&t), None)
    }

    #[test]
    fn parent() {
	let mut t = Tree::new();
	let e = t.new_element(QualifiedName::new(None, None, String::from("Test"))).expect("unable to create element node");
	t.push_doc_node(e).expect("unable to add node to doc");
	let l1 = t.new_element(QualifiedName::new(None, None, String::from("Level-1"))).expect("unable to create element node");
	e.append_child(&mut t, l1).expect("unable to append node");
	let t1 = t.new_text(Value::from("one")).expect("unable to create text node");
	l1.append_child(&mut t, t1).expect("unable to append node");
	let l2 = t.new_element(QualifiedName::new(None, None, String::from("Level-1"))).expect("unable to create element node");
	e.append_child(&mut t, l2).expect("unable to append node");
	let t2 = t.new_text(Value::from("two")).expect("unable to create text node");
	l2.append_child(&mut t, t2).expect("unable to append node");

	assert_eq!(e.to_xml(&t), "<Test><Level-1>one</Level-1><Level-1>two</Level-1></Test>");

	assert_eq!(t2.parent(&t), Some(l2));
    }
}
