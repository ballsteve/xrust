//! # xrust::item
//!
//! Sequence Item module.
//! An Item is a Node, Function or Atomic Value.
//!
//! Nodes are implemented as a trait.

use core::fmt;
use std::any::Any;
use std::cmp::Ordering;
use std::rc::Rc;
use crate::qname::QualifiedName;
use crate::xdmerror::{Error, ErrorKind};

/// In XPath, the Sequence is the fundamental data structure.
/// It is an ordered collection of [Item]s.
/// The Rust impementation is a Vector of reference counted [Item]s.
///
/// See [SequenceTrait] for methods.
pub type Sequence = Vec<Rc<Item>>;

pub trait SequenceTrait {
  /// Return the string value of the [Sequence].
  fn to_string(&self) -> String;
  /// Return a XML formatted representation of the [Sequence].
  fn to_xml(&self) -> String;
  /// Return a XML formatted representation of the [Sequence], controlled by the supplied output definition.
  fn to_xml_with_options(&self, od: &OutputDefinition) -> String;
  /// Return a JSON formatted representation of the [Sequence].
  fn to_json(&self) -> String;
  /// Return the Effective Boolean Value of the [Sequence].
  fn to_bool(&self) -> bool;
  /// Convert the [Sequence] to an integer. The [Sequence] must be a singleton value.
  fn to_int(&self) -> Result<i64, Error>;
  /// Push a [Document] to the [Sequence]
  fn new_document(&mut self, d: Box<dyn Document>);
  /// Push a [Node] to the [Sequence]
  fn new_node(&mut self, d: Box<dyn Document>, n: Box<dyn Node>);
  /// Push a [Value] to the [Sequence]
  fn new_value(&mut self, v: Value);
  /// Push an [Item] to the [Sequence]
  fn add(&mut self, i: &Rc<Item>);
}

impl SequenceTrait for Sequence {
  /// Returns the string value of the Sequence.
  fn to_string(&self) -> String {
    let mut r = String::new();
    for i in self {
      r.push_str(i.to_string().as_str())
    }
    r
  }
  /// Renders the Sequence as XML
  fn to_xml(&self) -> String {
    let mut r = String::new();
    for i in self {
      r.push_str(i.to_xml().as_str())
    }
    r
  }
  /// Renders the Sequence as XML
  fn to_xml_with_options(&self, od: &OutputDefinition) -> String {
    let mut r = String::new();
    for i in self {
      r.push_str(i.to_xml_with_options(od).as_str())
    }
    r
  }
  /// Renders the Sequence as JSON
  fn to_json(&self) -> String {
    let mut r = String::new();
    for i in self {
      r.push_str(i.to_json().as_str())
    }
    r
  }
  /// Push a [Document] on to the [Sequence]
  fn new_document(&mut self, d: Box<dyn Document>) {
    self.push(Rc::new(Item::Document(d)));
  }
  /// Push a [Document]'s [Node] on to the [Sequence]
  fn new_node(&mut self, d: Box<dyn Document>, n: Box<dyn Node>) {
    self.push(Rc::new(Item::Node(d, n)));
  }
  /// Push a [Value] on to the [Sequence]
  fn new_value(&mut self, v: Value) {
    self.push(Rc::new(Item::Value(v)));
  }
  //fn new_function(&self, f: Function) -> Sequence {
  //}
  /// Push an [Item] on to the [Sequence]. This clones the Item.
  fn add(&mut self, i: &Rc<Item>) {
    self.push(Rc::clone(i));
  }

  /// Calculate the effective boolean value of the Sequence
  fn to_bool(&self) -> bool {
    if self.len() == 0 {
      false
    } else {
      match *self[0] {
        Item::Document(_) |
        Item::Node(..) => true,
	_ => {
	  if self.len() == 1 {
	    (*self[0]).to_bool()
	  } else {
	    false // should be a type error
	  }
	}
      }
    }
  }

  /// Convenience routine for integer value of the [Sequence]. The Sequence must be a singleton; i.e. be a single item.
  fn to_int(&self) -> Result<i64, Error> {
    if self.len() == 1 {
      self[0].to_int()
    } else {
      Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("type error: sequence is not a singleton")})
    }
  }
}

/// An Item in a [Sequence]. Can be a [Node], Function or [Value].
///
/// [Document]s and [Node]s are dynamic trait objects. [Node]s can only exist in the context of a [Document].
///
/// Functions are not yet implemented.
//#[derive(Clone)]
pub enum Item {
    /// A [Document]
    Document(Box<dyn Document>),
    /// A [Node]
    Node(Box<dyn Document>, Box<dyn Node>),

    /// Functions are not yet supported
    Function,

    /// A scalar value
    Value(Value),
}

impl Item {
  /// Gives the string value of an item. All items have a string value.
  pub fn to_string(&self) -> String {
    match self {
      Item::Document(d) => d.to_string(None),
      Item::Node(d, n) => d.to_string(Some(n)),
      Item::Function => "".to_string(),
      Item::Value(v) => v.to_string(),
    }
  }
  /// Serialize as XML
  pub fn to_xml(&self) -> String {
    match self {
      Item::Document(d) => d.to_xml(None),
      Item::Node(d, n) => d.to_xml(Some(*n)),
      Item::Function => "".to_string(),
      Item::Value(v) => v.to_string(),
    }
  }
  /// Serialize as XML, with options
  pub fn to_xml_with_options(&self, od: &OutputDefinition) -> String {
    match self {
      Item::Document(d) => d.to_xml_with_options(None, od),
      Item::Node(d, n) => d.to_xml_with_options(Some(*n), od),
      Item::Function => "".to_string(),
      Item::Value(v) => v.to_string(),
    }
  }
  /// Serialize as JSON
  pub fn to_json(&self) -> String {
    match self {
      Item::Document(d) => d.to_json(None),
      Item::Node(d, n) => d.to_json(Some(*n)),
      Item::Function => "".to_string(),
      Item::Value(v) => v.to_string(),
    }
  }

  /// Determine the effective boolean value of the item.
  /// See XPath 2.4.3.
  pub fn to_bool(&self) -> bool {
    match self {
      Item::Document(_) |
      Item::Node(..) => true,
      Item::Function => false,
      Item::Value(v) => v.to_bool(),
    }
  }

  /// Gives the integer value of the item, if possible.
  pub fn to_int(&self) -> Result<i64, Error> {
    match self {
      Item::Document(_) |
      Item::Node(..) => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("type error: item is a node")}),
      Item::Function => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("type error: item is a function")}),
      Item::Value(v) => {
        match v.to_int() {
	  Ok(i) => {
	    Ok(i)
	  }
	  Err(e) => {
	    Result::Err(e)
	  }
	}
      },
    }
  }

  /// Gives the double value of the item. Returns NaN if the value cannot be converted to a double.
  pub fn to_double(&self) -> f64 {
    match self {
      Item::Document(_) |
      Item::Node(..) => f64::NAN,
      Item::Function => f64::NAN,
      Item::Value(v) => v.to_double(),
    }
  }

  /// Gives the name of the item. Certain types of Nodes have names, such as element-type nodes. If the item does not have a name returns an empty string.
  pub fn to_name(&self) -> QualifiedName {
    match self {
      Item::Node(d, i) => d.to_name(Some(*i)),
      _ => QualifiedName::new(None, None, "".to_string())
    }
  }

  // TODO: atomization
  // fn atomize(&self);

  /// Compare two items.
  pub fn compare(&self, other: &Item, op: Operator) -> Result<bool, Error> {
    match self {
      Item::Value(v) => {
        v.compare(other, op)
      }
      Item::Node(..) => {
        other.compare(&Item::Value(Value::String(self.to_string())), op)
      }
      _ => {
        Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("type error")})
      }
    }
  }

  /// Is this item an element-type node?
  pub fn is_element_node(&self) -> bool {
    match self {
      Item::Node(d, n) => {
        match d.node_type(*n) {
	  NodeType::Element => true,
	  _ => false,
	}
      }
      _ => false,
    }
  }
  /// If the Item is a Document type, then returns the root element of the document, otherwise returns None
  pub fn get_root_element(&self) -> Option<Box<dyn Node>> {
    match self {
      Item::Document(d) => {
        d.get_root_element()
      }
      _ => None,
    }
  }

  /// Gives the type of the item.
  pub fn item_type(&self) -> &'static str {
    match self {
      Item::Document(_) => "Document",
      Item::Node(..) => "Node",
      Item::Function => "Function",
      Item::Value(v) => v.value_type(),
    }
  }
}

/// Document tree.
///
/// A Document contains [Node] objects.
///
///Because [Node]s cannot exist outside of a Document, their methods are accessed via the Document trait.
pub trait Document: Any {
    /// Upcast to Any
    fn as_any(&self) -> &dyn Any;
    /// Upcast to Any, mutable
    fn as_any_mut(&mut self) -> &mut dyn Any;
    /// Return the string value of the [Node], or the Document if None
    fn to_string(&self, n: Option<Box<dyn Node>>) -> String;
    /// Serialize the given [Node] as XML, or the Document if None
    fn to_xml(&self, n: Option<Box<dyn Node>>) -> String;
    /// Serialize as XML, with options
    fn to_xml_with_options(&self, n: Option<Box<dyn Node>>, od: &OutputDefinition) -> String;
    /// Serialize as JSON
    fn to_json(&self, n: Option<Box<dyn Node>>) -> String;
    /// Determine the effective boolean value. See XPath 2.4.3.
    /// A Document or Node always returns true.
    fn to_bool(&self, _n: Option<Box<dyn Node>>) -> bool {
	true
    }
    /// Return the integer value. For a Document, this is a type error.
    fn to_int(&self, n: Option<Box<dyn Node>>) -> Result<i64, Error>;
    /// Return the double value. For a Document, this is a type error, i.e. NaN.
    fn to_double(&self, n: Option<Box<dyn Node>>) -> f64;
    /// Gives the name of the [Node]. Documents do not have a name, so the implementation must return an empty string.
    fn to_name(&self, n: Option<Box<dyn Node>>) -> QualifiedName;

    /// Return the type of a Node
    fn node_type(&self, n: Box<dyn Node>) -> NodeType;

    /// Callback for logging/debugging, particularly in a web_sys environment
    fn log(&self, _m: &str) {
	// Noop
    }

    /// Return the root node of the Document.
    fn get_root_element(&self) -> Option<Box<dyn Node>>;
    /// Set the root element for the Document. If the Document already has a root element then it will be removed. The node must be an element. If the node supplied is of a different concrete type to the Document then an error is returned. If the element is from a different Document, then the function performs a deep copy.
    fn set_root_element(&mut self, r: &dyn Any) -> Result<(), Error>;

    /// An iterator over ancestors of a [Node].
    fn ancestor_iter(&self, n: Box<dyn Node>) -> Box<dyn AncestorIterator<Item = Box<dyn Node>>>;
    /// Navigate to the parent of a [Node]. Documents, and the root element, don't have a parent, so the default implementation returns None. This is a convenience function for ancestor_iter.
    fn parent(&self, _n: Box<dyn Node>) -> Option<Box<dyn Node>> {
	None
    }
    /// An iterator for the child nodes of a [Node]. Non-element type nodes will immediately return None.
    fn child_iter(&self, n: Box<dyn Node>) -> Box<dyn ChildIterator<Item = Box<dyn Node>>>;
    /// An iterator for the child nodes of the Document. This may include the prologue, root element, and epilogue.
    fn doc_child_iter(&self) -> Box<dyn DocChildIterator<Item = Box<dyn Node>>>;
    /// An iterator for descendants of a [Node]. Does not include the [Node] itself.
    // fn descend_iter(&self, n: Box<dyn Node>) -> Box<dyn Iterator<Item = Box<dyn Node>>>;
    /// An iterator for following siblings of a [Node]. Does not include the [Node] itself.
    // fn following_sibling_iter(&self, n: Box<dyn Node>) -> Box<dyn Iterator<Item = Box<dyn Node>>>;
    /// An iterator for preceding siblings of a [Node]. Does not include the [Node] itself.
    // fn preceding_sibling_iter(&self, n: Box<dyn Node>) -> Box<dyn Iterator<Item = Box<dyn Node>>>;

    /// Create an element [Node] in the Document.
    fn new_element(&mut self, name: QualifiedName) -> Result<Box<dyn Node>, Error>;
    /// Create a text [Node] in the Document.
    fn new_text(&mut self, c: Value) -> Result<Box<dyn Node>, Error>;
    /// Create an attribute [Node] in the Document.
    fn new_attribute(&mut self, name: QualifiedName, v: Value) -> Result<Box<dyn Node>, Error>;
    /// Create a comment [Node] in the Document.
    fn new_comment(&mut self, v: Value) -> Result<Box<dyn Node>, Error>;
    /// Create a processing instruction [Node] in the Document.
    fn new_processing_instruction(&mut self, name: QualifiedName, v: Value) -> Result<Box<dyn Node>, Error>;

    /// Append a [Node] to the children of a [Node]. If the [Node] to be appended is from a different Document then this function performs a deep copy.
    fn append_child(&mut self, parent: Box<dyn Node>, child: Box<dyn Node>) -> Result<(), Error>;
    /// Inserts a [Node] (insert) before another [Node] (child) in the children of it's parent element [Node]. If the [Node] to be inserted is from a different Document then this function performs a deep copy.
    fn insert_before(&mut self, child: Box<dyn Node>, insert: Box<dyn Node>) -> Result<(), Error>;
    // TODO: replace_child

    /// Add an attribute [Node] to an element type [Node]. If the attribute [Node] is from a different Document then this function adds a copy of the attribute [Node].
    fn add_attribute_node(&mut self, _parent: Box<dyn Node>, _a: &dyn Any) -> Result<(), Error> {
	Result::Err(Error::new(ErrorKind::NotImplemented, String::from("not implemented")))
    }

    /// Remove a node from its parent
    fn remove(&mut self, _n: Box<dyn Node>) -> Result<(), Error> {
	Result::Err(Error::new(ErrorKind::NotImplemented, String::from("not implemented")))
    }
}

/// An iterator over ancestor nodes
pub trait AncestorIterator {
    type Item = Box<dyn Node>;
    fn next(&mut self, d: &dyn Document) -> Option<Self::Item>;
}

/// An iterator over child nodes
pub trait ChildIterator {
    type Item = Box<dyn Node>;
    fn next(&mut self, d: &dyn Document) -> Option<Self::Item>;
}

/// An iterator over child nodes of a [Document]
pub trait DocChildIterator {
    type Item = Box<dyn Node>;
    fn next(&mut self, d: &dyn Document) -> Option<Self::Item>;
}

/// Node in a tree.
///
/// This trait defines how to navigate a tree-like structure.
pub trait Node: Any {
    /// Upcast to Any
    fn as_any(&self) -> &dyn Any;
    /// Upcast to Any, mutable
    fn as_any_mut(&mut self) -> &mut dyn Any;
    /// Return the string value of the Node
    fn to_string(&self) -> String;
    /// Serialize as XML
    fn to_xml(&self) -> String;
    /// Serialize as XML, with options
    fn to_xml_with_options(&self, od: &OutputDefinition) -> String;
    /// Serialize as JSON
    fn to_json(&self) -> String;
    /// Determine the effective boolean value. See XPath 2.4.3.
    /// A (non-empty) Node always returns true.
    fn to_bool(&self) -> bool {
	true
    }
    /// Return the integer value.
    fn to_int(&self) -> Result<i64, Error>;
    /// Return the double value.
    fn to_double(&self) -> f64;
    /// Gives the name of the node. Certain types of Nodes have names, such as element-type nodes. If the item does not have a name returns an empty string.
    fn to_name(&self) -> QualifiedName;
    /// Navigate to the [Document]. Not all implementations are able to do this, so if this is the case the option can be set to None.
    fn owner_document(&self) -> Option<Box<dyn Document>> {
	None
    }
    /// Return the type of the Node
    fn node_type(&self) -> NodeType;

    /// Return the value of an attribute. Returns None if the node is not an element, or the element has no such attribute.
    fn get_attribute(&self, _name: &QualifiedName) -> Option<Value> {
	None
    }
    /// Add an attribute to an element node. If the attribute already exists, it's value is overwritten. If the Node is not an element then this operation has no effect.
    fn set_attribute(&self, name: QualifiedName, val: Value);
    /// Iterator for attributes
    //type AttributeIterator: Iterator<Item=dyn Node>;
    //fn attribute_iter(&self) -> Self::AttributeIterator;
    //fn attribute_iter(&self) -> Box<dyn Iterator<Item=dyn Node>>;
    fn attributes(&self) -> Vec<Box<dyn Node>>;
    /// Set the value for nodes that have values (text, attribute, comment, PI). No effect on other nodes.
    fn set_value(&self, v: Value);
    /// Returns if the node is an element-type node
    fn is_element(&self) -> bool {
	false
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

/// An output definition. See XSLT v3.0 26 Serialization
#[derive(Clone, Debug)]
pub struct OutputDefinition {
  name: Option<QualifiedName>,	// TODO: EQName
  indent: bool,
  // TODO: all the other myriad output parameters
}

impl OutputDefinition {
  pub fn new() -> OutputDefinition {
    OutputDefinition{name: None, indent: false}
  }
  pub fn get_name(&self) -> Option<QualifiedName> {
    self.name.clone()
  }
  pub fn set_name(&mut self, name: Option<QualifiedName>) {
    match name {
      Some(n) => {self.name.replace(n);},
      None => {self.name = None;},
    }
  }
  pub fn get_indent(&self) -> bool {
    self.indent
  }
  pub fn set_indent(&mut self, ind: bool) {
    self.indent = ind;
  }
}
impl fmt::Display for OutputDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.indent {
	  f.write_str("indent output")
	} else {
	  f.write_str("do not indent output")
	}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // to_bool

    #[test]
    fn bool_value_string_empty() {
      assert_eq!(Item::Value(Value::from("")).to_bool(), false)
    }
    #[test]
    fn bool_value_string_nonempty() {
      assert_eq!(Item::Value(Value::from("false")).to_bool(), true)
    }
    #[test]
    fn bool_value_int_zero() {
      assert_eq!(Item::Value(Value::from(0)).to_bool(), false)
    }
    #[test]
    fn bool_value_int_nonzero() {
      assert_eq!(Item::Value(Value::from(42)).to_bool(), true)
    }

    // to_int

    #[test]
    fn int_value_string() {
      match Item::Value(Value::from("1")).to_int() {
        Ok(i) => assert_eq!(i, 1),
	Err(_) => {
	  panic!("to_int() failed")
	}
      }
    }

    // to_double

    #[test]
    fn double_value_string() {
      assert_eq!(Item::Value(Value::from("2.0")).to_double(), 2.0)
    }

    // Sequences

    #[test]
    fn sequence() {
        let _s = Sequence::new();
        assert!(true)
    }
    #[test]
    fn sequence_one() {
        let mut s = Sequence::new();
	s.new_value(Value::from("one"));
	let mut t = Sequence::new();
	t.add(&s[0]);
	assert!(Rc::ptr_eq(&s[0], &t[0]))
    }
}
