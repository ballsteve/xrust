//! # xrust::item
//!
//! Sequence Item module.
//! An Item is a Node, Function or Atomic Value.
//!
//! Nodes are implemented as a trait.

//use core::fmt;
use std::any::Any;
use std::cmp::Ordering;
use std::rc::Rc;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use chrono::{Date, DateTime, Local};
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
  fn new_document(&mut self, d: Rc<dyn Document>);
  /// Push a [Node] to the [Sequence]
  fn new_node(&mut self, d: Rc<dyn Node>);
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
  fn new_document(&mut self, d: Rc<dyn Document>) {
    self.push(Rc::new(Item::Document(d)));
  }
  /// Push a [Node] on to the [Sequence]
  fn new_node(&mut self, n: Rc<dyn Node>) {
    self.push(Rc::new(Item::Node(n)));
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
        Item::Node(_) => true,
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
/// A [Document] is a special [Node].
///
/// Nodes are a trait.
///
/// Functions are not yet implemented.
#[derive(Clone)]
pub enum Item {
    /// A [Document]
    Document(Rc<dyn Document>),
    /// A [Node]
    Node(Rc<dyn Node>),

    /// Functions are not yet supported
    Function,

    /// A scalar value
    Value(Value),
}

// Comparison operators
#[derive(Copy, Clone)]
pub enum Operator {
  Equal,
  NotEqual,
  LessThan,
  LessThanEqual,
  GreaterThan,
  GreaterThanEqual,
  Is,
  Before,
  After,
}

impl Operator {
  pub fn to_string(&self) -> &str {
    match self {
      Operator::Equal => "=",
      Operator::NotEqual => "!=",
      Operator::LessThan => "<",
      Operator::LessThanEqual => "<=",
      Operator::GreaterThan => ">",
      Operator::GreaterThanEqual => ">=",
      Operator::Is => "is",
      Operator::Before => "<<",
      Operator::After => ">>",
    }
  }
}

impl Item {
  /// Gives the string value of an item. All items have a string value.
  pub fn to_string(&self) -> String {
    match self {
      Item::Document(d) => d.to_string(),
      Item::Node(n) => n.to_string(),
      Item::Function => "".to_string(),
      Item::Value(v) => v.to_string(),
    }
  }
  /// Serialize as XML
  pub fn to_xml(&self) -> String {
    match self {
      Item::Document(d) => d.to_xml(),
      Item::Node(n) => n.to_xml(),
      Item::Function => "".to_string(),
      Item::Value(v) => v.to_string(),
    }
  }
  /// Serialize as XML, with options
  pub fn to_xml_with_options(&self, od: &OutputDefinition) -> String {
    match self {
      Item::Document(d) => d.to_xml_with_options(od),
      Item::Node(n) => n.to_xml_with_options(od),
      Item::Function => "".to_string(),
      Item::Value(v) => v.to_string(),
    }
  }
  /// Serialize as JSON
  pub fn to_json(&self) -> String {
    match self {
      Item::Document(d) => d.to_json(),
      Item::Node(n) => n.to_json(),
      Item::Function => "".to_string(),
      Item::Value(v) => v.to_string(),
    }
  }

  /// Determine the effective boolean value of the item.
  /// See XPath 2.4.3.
  pub fn to_bool(&self) -> bool {
    match self {
      Item::Document(_) |
      Item::Node(_) => true,
      Item::Function => false,
      Item::Value(v) => v.to_bool(),
    }
  }

  /// Gives the integer value of the item, if possible.
  pub fn to_int(&self) -> Result<i64, Error> {
    match self {
      Item::Document(_) |
      Item::Node(_) => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("type error: item is a node")}),
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
      Item::Node(_) => f64::NAN,
      Item::Function => f64::NAN,
      Item::Value(v) => v.to_double(),
    }
  }

  /// Gives the name of the item. Certain types of Nodes have names, such as element-type nodes. If the item does not have a name returns an empty string.
  pub fn to_name(&self) -> QualifiedName {
    match self {
      Item::Node(i) => i.to_name(),
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
      Item::Node(_) => {
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
      Item::Node(n) => {
        match n.node_type() {
	  NodeType::Element => true,
	  _ => false,
	}
      }
      _ => false,
    }
  }
  /// If the Item is a Document type, then returns the root element of the document, otherwise returns None
  pub fn get_root_element(&self) -> Option<Rc<dyn Node>> {
    match self {
      Item::Document(d) => {
        match d.get_root_element() {
	  Some(n) => Some(n),
	  _ => None,
	}
      }
      _ => None,
    }
  }

  /// Gives the type of the item.
  pub fn item_type(&self) -> &'static str {
    match self {
      Item::Document(_) => "Document",
      Item::Node(_) => "Node",
      Item::Function => "Function",
      Item::Value(v) => v.value_type(),
    }
  }
}

/// Document tree.
///
/// A Document contains [Node] objects.
pub trait Document {
  /// Return the string value of the Document
  fn to_string(&self) -> String;
  /// Serialize as XML
  fn to_xml(&self) -> String;
  /// Serialize as XML, with options
  fn to_xml_with_options(&self, od: &OutputDefinition) -> String;
  /// Serialize as JSON
  fn to_json(&self) -> String;
  /// Determine the effective boolean value. See XPath 2.4.3.
  /// A (non-empty) Document always returns true.
  fn to_bool(&self) -> bool {
    true
  }
  /// Return the integer value. For a Document, this is a type error.
  fn to_int(&self) -> Result<i64, Error> {
    Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("type error: document")})
  }
  /// Return the double value. For a Document, this is a type error, i.e. NaN.
  fn to_double(&self) -> f64 {
    f64::NAN
  }
  /// Gives the name of the document. Documents do not have a name, so the default implementation returns an empty string.
  fn to_name(&self) -> QualifiedName {
    QualifiedName::new(None, None, String::new())
  }

  /// Callback for logging/debugging, particularly in a web_sys environment
  fn log(&self, _m: &str) {
    // Noop
  }

  /// Navigate to the parent of the Document. Documents don't have a parent, so the default implementation returns None.
  fn parent(&self) -> Option<Rc<dyn Node>> {
    None
  }
  /// Return the children of the Document. This may include the prologue, root element and epilogue. If the Document has no children then returns an empty vector.
  fn children(&self) -> Vec<Rc<dyn Node>>;
  /// Return the root node of the Document.
  fn get_root_element(&self) -> Option<Rc<dyn Node>>;

  /// Create an element Node in the Document.
  fn new_element(&self, name: QualifiedName) -> Result<Rc<dyn Node>, Error>;
  /// Create a text Node in the Document.
  fn new_text(&self, c: Value) -> Result<Rc<dyn Node>, Error>;
  /// Create an attribute Node in the Document.
  fn new_attribute(&self, name: QualifiedName, v: Value) -> Result<Rc<dyn Node>, Error>;
  // TODO: new_comment, new_PI
  /// Insert the root element in the Document. NB. If the element supplied is of a different concrete type to the Document then this will likely result in an error.
  fn set_root_element(&mut self, r: &dyn Any) -> Result<(), Error>;
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
  fn owner_document(&self) -> Option<Rc<dyn Document>> {
    None
  }
  /// Return the type of the Node
  fn node_type(&self) -> NodeType;
  /// Navigate to the parent of the node.
  fn parent(&self) -> Option<Rc<dyn Node>>;
  /// An iterator over ancestors of the Node.
  // TODO: rust complains about borrowing data in an 'Rc' as mutable
  //fn ancestor_iter(self) -> Rc<dyn Iterator<Item=Rc<dyn Node>>> where Self: Sized {
    //Rc::new(Ancestor::new(Rc::new(self)))
  //}
  /// Return all of the ancestors of the Node.
  // TODO: this gives the error "error[E0277]: the size for values of type `Self` cannot be known at compilation time"
  fn ancestors(&self) -> Vec<Rc<dyn Node>>;
  //{
    //self.ancestor_iter().collect()
  //}
  /// Return the children of the node. If the node has no children then returns an empty vector.
  fn children(&self) -> Vec<Rc<dyn Node>>;
  /// Return descendants of the Node, but not including the Node itself.
  fn descendants(&self) -> Vec<Rc<dyn Node>>;
  /// Return the next following sibling.
  fn get_following_sibling(&self) -> Option<Rc<dyn Node>>;
  /// An iterator over following siblings.
  // TODO: rust complains about borrowing data in an 'Rc' as mutable
  //fn following_sibling_iter(self) -> Rc<dyn Iterator<Item=Rc<dyn Node>>> where Self: Sized {
    //Rc::new(FollowingSibling::new(Rc::new(self)))
  //}
  /// Return all of the following siblings of the Node.
  // TODO: see above
  fn following_siblings(&self) -> Vec<Rc<dyn Node>>;
  //{
    //self.following_sibling_iter().collect()
  //}
  /// Return the next preceding sibling.
  fn get_preceding_sibling(&self) -> Option<Rc<dyn Node>>;
  /// An iterator over preceding siblings.
  // TODO: rust complains about borrowing data in an 'Rc' as mutable
  //fn preceding_sibling_iter(self) -> Rc<dyn Iterator<Item=Rc<dyn Node>>> where Self: Sized {
    //Rc::new(PrecedingSibling::new(Rc::new(self)))
  //}
  /// Return all of the preceding siblings of the Node.
  // TODO: see above
  fn preceding_siblings(&self) -> Vec<Rc<dyn Node>>;
  //{
    //self.preceding_sibling_iter().collect()
  //}

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
  fn attributes(&self) -> Vec<Rc<dyn Node>>;
  /// Set the value for nodes that have values (text, attribute, comment, PI). No effect on other nodes.
  fn set_value(&self, v: Value);
  /// Returns if the node is an element-type node
  fn is_element(&self) -> bool {
    false
  }

  /// Insert a Node as a child. The node is appended to the list of children. NB. If the element supplied is of a different concrete type to the Node then this will likely result in an error.
  fn append_child(&self, c: &dyn Any) -> Result<(), Error>;
  /// Add a text node as a child.
  fn append_text_child(&self, t: Value) -> Result<(), Error>;
  // TODO: insert_before, replace_child
  fn add_attribute_node(&self, a: &dyn Any) -> Result<(), Error>;

  /// Remove a node from its parent
  fn remove(&self) -> Result<(), Error>;
  // TODO: remove_child
}

pub struct Ancestor {
  node: Rc<dyn Node>,
}

//impl Ancestor {
//  fn new(n: Rc<dyn Node>) -> Ancestor {
//    Ancestor{node: n}
//  }
//}
impl Iterator for Ancestor {
  type Item = Rc<dyn Node>;

  fn next(&mut self) -> Option<Self::Item> {
    match self.node.parent() {
      Some(n) => {
        // The dynamic trait object is not clonable, so make a second call to the method to get an object to store in the iterator
	self.node = self.node.parent().unwrap();
	Some(n)
      }
      None => None,
    }
  }
}

struct FollowingSibling {
  node: Rc<dyn Node>,
}

//impl FollowingSibling {
//  fn new(n: Rc<dyn Node>) -> FollowingSibling {
//    FollowingSibling{node: n}
//  }
//}
impl Iterator for FollowingSibling {
  type Item = Rc<dyn Node>;

  fn next(&mut self) -> Option<Self::Item> {
    match self.node.get_following_sibling() {
      Some(n) => {
        // The dynamic trait object is not clonable, so make a second call to the method to get an object to store in the iterator
	self.node = self.node.get_following_sibling().unwrap();
	Some(n)
      }
      None => None,
    }
  }
}

struct PrecedingSibling {
  node: Rc<dyn Node>,
}

//impl PrecedingSibling {
//  fn new(n: Rc<dyn Node>) -> PrecedingSibling {
//    PrecedingSibling{node: n}
//  }
//}
impl Iterator for PrecedingSibling {
  type Item = Rc<dyn Node>;

  fn next(&mut self) -> Option<Self::Item> {
    match self.node.get_preceding_sibling() {
      Some(n) => {
        // The dynamic trait object is not clonable, so make a second call to the method to get an object to store in the iterator
	self.node = self.node.get_preceding_sibling().unwrap();
	Some(n)
      }
      None => None,
    }
  }
}

#[derive(Copy, Clone, Debug)]
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

// A concrete type that implements atomic values

impl PartialEq for Value {
  fn eq(&self, other: &Value) -> bool {
    match self {
        Value::String(s) => s.eq(&other.to_string()),
	Value::Boolean(b) => match other {
	  Value::Boolean(c) => b == c,
	  _ => false, // type error?
	},
	Value::Decimal(d) => match other {
	  Value::Decimal(e) => d == e,
	  _ => false, // type error?
	},
	Value::Integer(i) => match other {
	  Value::Integer(j) => i == j,
	  _ => false, // type error? coerce to integer?
	},
	Value::Double(d) => match other {
	  Value::Double(e) => d == e,
	  _ => false, // type error? coerce to integer?
	},
        _ => false, // not yet implemented
    }
  }
}
impl PartialOrd for Value {
  fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
    match self {
        Value::String(s) => {
	  let o: String = other.to_string();
	  s.partial_cmp(&o)
	},
	Value::Boolean(_) => None,
	Value::Decimal(d) => match other {
	  Value::Decimal(e) => d.partial_cmp(e),
	  _ => None, // type error?
	}
	Value::Integer(d) => match other {
	  Value::Integer(e) => d.partial_cmp(e),
	  _ => None, // type error?
	}
	Value::Double(d) => match other {
	  Value::Double(e) => d.partial_cmp(e),
	  _ => None, // type error?
	}
	_ => None,
    }
  }
}

/// An atomic value. These are the 19 predefined types in XSD Schema Part 2, plus five additional types.
#[derive(Clone)]
pub enum Value {
    /// node or simple type
    AnyType,
    /// a not-yet-validated anyType
    Untyped,
    /// base type of all simple types. i.e. not a node
    AnySimpleType,
    /// a list of IDREF
    IDREFS,
    /// a list of NMTOKEN
    NMTOKENS,
    /// a list of ENTITY
    ENTITIES,
    /// Any numeric type
    Numeric,
    /// all atomic values (no lists or unions)
    AnyAtomicType,
    /// untyped atomic value
    UntypedAtomic,
    Duration,
    Time(DateTime<Local>),	// Ignore the date part. Perhaps use Instant instead?
    Decimal(Decimal),
    Float(f32),
    Double(f64),
    Integer(i64),
    NonPositiveInteger(NonPositiveInteger),
    NegativeInteger(NegativeInteger),
    Long(i64),
    Int(i32),
    Short(i16),
    Byte(i8),
    NonNegativeInteger(NonNegativeInteger),
    UnsignedLong(u64),
    UnsignedInt(u32),
    UnsignedShort(u16),
    UnsignedByte(u8),
    PositiveInteger(PositiveInteger),
    DateTime(DateTime<Local>),
    DateTimeStamp,
    Date(Date<Local>),
    String(String),
    NormalizedString(NormalizedString),
    /// Like normalizedString, but without leading, trailing and consecutive whitespace
    Token,
    /// language identifiers [a-zA-Z]{1,8}(-[a-zA-Z0-9]{1,8})*
    Language,
    /// NameChar+
    NMTOKEN,
    /// NameStartChar NameChar+
    Name,
    /// (Letter | '_') NCNameChar+ (i.e. a Name without the colon)
    NCName,
    /// Same format as NCName
    ID,
    /// Same format as NCName
    IDREF,
    /// Same format as NCName
    ENTITY,
    Boolean(bool),
}

impl Value {
    /// Give the string value.
    pub fn to_string(&self) -> String {
	match self {
	    Value::String(s) => s.to_string(),
	    Value::NormalizedString(s) => s.value.to_string(),
	    Value::Decimal(d) => d.to_string(),
	    Value::Float(f) => f.to_string(),
	    Value::Double(d) => d.to_string(),
	    Value::Integer(i) => i.to_string(),
	    Value::Long(l) => l.to_string(),
	    Value::Short(s) => s.to_string(),
	    Value::Int(i) => i.to_string(),
	    Value::Byte(b) => b.to_string(),
	    Value::UnsignedLong(l) => l.to_string(),
	    Value::UnsignedShort(s) => s.to_string(),
	    Value::UnsignedInt(i) => i.to_string(),
	    Value::UnsignedByte(b) => b.to_string(),
	    Value::NonPositiveInteger(i) => i.value().to_string(),
	    Value::NonNegativeInteger(i) => i.value().to_string(),
	    Value::PositiveInteger(i) => i.value().to_string(),
	    Value::NegativeInteger(i) => i.value().to_string(),
	    Value::Time(t) => t.to_string(),
	    Value::DateTime(dt) => dt.to_string(),
	    Value::Date(d) => d.to_string(),
 	    _ => "".to_string(),
	}
    }

    /// Give the effective boolean value.
    pub fn to_bool(&self) -> bool {
        match &self {
            Value::Boolean(b) => *b == true,
            Value::String(t) => {
                //t.is_empty()
	        t.len() != 0
            },
	    Value::NormalizedString(s) => s.value.len() != 0,
            Value::Double(n) => *n != 0.0,
            Value::Integer(i) => *i != 0,
            _ => false
	}
    }

    /// Convert the value to an integer, if possible.
    pub fn to_int(&self) -> Result<i64, Error> {
        match &self {
	    Value::String(s) => {
	      match s.parse::<i64>() {
	        Ok(i) => Ok(i),
		Err(e) => Result::Err(Error{kind: ErrorKind::Unknown, message: format!("type conversion error: {}", e)}),
	      }
	    }
            Value::Integer(i) => Ok(*i),
            _ => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("type error (conversion not implemented)")})
	}
    }
    /// Convert the value to a double. If the value cannot be converted, returns Nan.
    pub fn to_double(&self) -> f64 {
        match &self {
	    Value::String(s) => {
	      match s.parse::<f64>() {
	        Ok(i) => i,
		Err(_) => f64::NAN,
	      }
	    }
            Value::Integer(i) => (*i) as f64,
            Value::Double(d) => *d,
            _ => f64::NAN,
	}
    }

    // TODO: type coersion
    // TODO: will probably have to implement comparison in the item module (as a trait?)
    /// Compare two items
    pub fn compare(&self, other: &Item, op: Operator) -> Result<bool, Error> {
      match &self {
        Value::Boolean(b) => {
	  let c = other.to_bool();
      	  match op {
                Operator::Equal => Ok(*b == c),
    		Operator::NotEqual => Ok(*b != c),
    		Operator::LessThan => Ok(*b < c),
    		Operator::LessThanEqual => Ok(*b <= c),
    		Operator::GreaterThan => Ok(*b > c),
    		Operator::GreaterThanEqual => Ok(*b >= c),
    		Operator::Is |
    		Operator::Before |
    		Operator::After => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("type error")})
	  }
	}
        Value::Integer(i) => {
	  match other.to_int() {
	    Ok(j) => {
      	      match op {
                Operator::Equal => Ok(*i == j),
    		Operator::NotEqual => Ok(*i != j),
    		Operator::LessThan => Ok(*i < j),
    		Operator::LessThanEqual => Ok(*i <= j),
    		Operator::GreaterThan => Ok(*i > j),
    		Operator::GreaterThanEqual => Ok(*i >= j),
    		Operator::Is |
    		Operator::Before |
    		Operator::After => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("type error")})
      	      }
	    }
	    Result::Err(e) => Result::Err(e)
	  }
	}
        Value::Double(d) => {
	  let e = other.to_double();
      	      match op {
                Operator::Equal => Ok(*d == e),
    		Operator::NotEqual => Ok(*d != e),
    		Operator::LessThan => Ok(*d < e),
    		Operator::LessThanEqual => Ok(*d <= e),
    		Operator::GreaterThan => Ok(*d > e),
    		Operator::GreaterThanEqual => Ok(*d >= e),
    		Operator::Is |
    		Operator::Before |
    		Operator::After => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("type error")})
      	      }
	}
        Value::String(s) => {
	  let t = other.to_string();
      	  match op {
                Operator::Equal => Ok(s.to_string() == t),
    		Operator::NotEqual => Ok(s.to_string() != t),
    		Operator::LessThan => Ok(s.to_string() < t),
    		Operator::LessThanEqual => Ok(s.to_string() <= t),
    		Operator::GreaterThan => Ok(s.to_string() > t),
    		Operator::GreaterThanEqual => Ok(s.to_string() >= t),
    		Operator::Is |
    		Operator::Before |
    		Operator::After => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("type error")})
	  }
	}
	_ => Result::Err(Error{kind: ErrorKind::Unknown, message: format!("comparing type \"{}\" is not yet implemented", self.value_type())})
      }
    }
    pub fn value_type(&self) -> &'static str {
      match &self {
        Value::AnyType => "AnyType",
        Value::Untyped => "Untyped",
        Value::AnySimpleType => "AnySimpleType",
        Value::IDREFS => "IDREFS",
        Value::NMTOKENS => "NMTOKENS",
        Value::ENTITIES => "ENTITIES",
        Value::Numeric => "Numeric",
        Value::AnyAtomicType => "AnyAtomicType",
        Value::UntypedAtomic => "UntypedAtomic",
        Value::Duration => "Duration",
        Value::Time(_) => "Time",
        Value::Decimal(_) => "Decimal",
        Value::Float(_) => "Float",
        Value::Double(_) => "Double",
        Value::Integer(_) => "Integer",
        Value::NonPositiveInteger(_) => "NonPositiveInteger",
        Value::NegativeInteger(_) => "NegativeInteger",
        Value::Long(_) => "Long",
        Value::Int(_) => "Int",
        Value::Short(_) => "Short",
        Value::Byte(_) => "Byte",
        Value::NonNegativeInteger(_) => "NonNegativeInteger",
        Value::UnsignedLong(_) => "UnsignedLong",
        Value::UnsignedInt(_) => "UnsignedInt",
        Value::UnsignedShort(_) => "UnsignedShort",
        Value::UnsignedByte(_) => "UnsignedByte",
        Value::PositiveInteger(_) => "PositiveInteger",
        Value::DateTime(_) => "DateTime",
        Value::DateTimeStamp => "DateTimeStamp",
        Value::Date(_) => "Date",
        Value::String(_) => "String",
        Value::NormalizedString(_) => "NormalizedString",
        Value::Token => "Token",
        Value::Language => "Language",
        Value::NMTOKEN => "NMTOKEN",
        Value::Name => "Name",
        Value::NCName => "NCName",
        Value::ID => "ID",
        Value::IDREF => "IDREF",
        Value::ENTITY => "ENTITY",
	Value::Boolean(_) => "boolean",
      }
    }
}

pub struct NonPositiveInteger {
    value: i64,
}
impl NonPositiveInteger {
    pub fn new(v: i64) -> Result<Self, Error> {
        if v > 0 {
	    let e = Error {
	    	kind: ErrorKind::TypeError,
		message: String::from("nonPositiveInteger must be zero or less"),
	    };
	    Err(e)
	} else {
	    let n = NonPositiveInteger {
	        value: v,
	    };
	    Ok(n)
	}
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}
impl Clone for NonPositiveInteger {
    fn clone(&self) -> Self {
        NonPositiveInteger::new(self.value).expect("unable to clone NonPositiveInteger")
    }
}

pub struct PositiveInteger {
    value: i64,
}
impl PositiveInteger {
    pub fn new(v: i64) -> Result<Self, Error> {
        if v <= 0 {
	    let e = Error {
	    	kind: ErrorKind::TypeError,
		message: String::from("PositiveInteger must be greater than zero"),
	    };
	    Err(e)
	} else {
	    let n = PositiveInteger {
	        value: v,
	    };
	    Ok(n)
	}
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}
impl Clone for PositiveInteger {
    fn clone(&self) -> Self {
        PositiveInteger::new(self.value).expect("unable to clone PositiveInteger")
    }
}

pub struct NonNegativeInteger {
    value: i64,
}
impl NonNegativeInteger {
    pub fn new(v: i64) -> Result<Self, Error> {
        if v < 0 {
	    let e = Error {
	    	kind: ErrorKind::TypeError,
		message: String::from("nonNegativeInteger must be zero or greater"),
	    };
	    Err(e)
	} else {
	    let n = NonNegativeInteger {
	        value: v,
	    };
	    Ok(n)
	}
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}
impl Clone for NonNegativeInteger {
    fn clone(&self) -> Self {
        NonNegativeInteger::new(self.value).expect("unable to clone NonNegativeInteger")
    }
}

pub struct NegativeInteger {
    value: i64,
}
impl NegativeInteger {
    pub fn new(v: i64) -> Result<Self, Error> {
        if v >= 0 {
	    let e = Error {
	    	kind: ErrorKind::TypeError,
		message: String::from("NegativeInteger must be less than zero"),
	    };
	    Err(e)
	} else {
	    let n = NegativeInteger {
	        value: v,
	    };
	    Ok(n)
	}
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}
impl Clone for NegativeInteger {
    fn clone(&self) -> Self {
        NegativeInteger::new(self.value).expect("unable to clone NegativeInteger")
    }
}

pub struct NormalizedString {
    value: String,
}
impl NormalizedString {
    pub fn new(v: &str) -> Result<Self, Error> {
        let n: &[_] = &['\n', '\r', '\t'];
        match v.find(n) {
	    None => Ok(NormalizedString{value: v.to_string()}),
	    _ => Err(Error {
	        kind: ErrorKind::TypeError,
		message: String::from("value is not a normalized string"),
	  	})
	}
    }
    pub fn value(self) -> String {
        self.value.to_string()
    }
}
impl Clone for NormalizedString {
    fn clone(&self) -> Self {
        NormalizedString::new(&self.value.clone()).expect("unable to clone NormalizedString")
    }
}

/// An output definition. See XSLT v3.0 26 Serialization
#[derive(Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizedstring_valid_empty() {
        assert_eq!(NormalizedString::new("").expect("invalid NormalizedString").value, "");
    }
    #[test]
    fn normalizedstring_valid() {
        assert_eq!(NormalizedString::new("notinvalid").expect("invalid NormalizedString").value, "notinvalid");
    }
    #[test]
    fn normalizedstring_valid_spaces() {
        assert_eq!(NormalizedString::new("not an invalid string").expect("invalid NormalizedString").value, "not an invalid string");
    }
    #[test]
    fn normalizedstring_invalid_tab() {
        let r = NormalizedString::new("contains tab	character");
	assert!(match r {
	    Ok(_) => panic!("string contains tab character"),
	    Err(_) => true,
	})
    }
    #[test]
    fn normalizedstring_invalid_newline() {
        let r = NormalizedString::new("contains newline\ncharacter");
	assert!(match r {
	    Ok(_) => panic!("string contains newline character"),
	    Err(_) => true,
	})
    }
    #[test]
    fn normalizedstring_invalid_cr() {
        let r = NormalizedString::new("contains carriage return\rcharacter");
	assert!(match r {
	    Ok(_) => panic!("string contains cr character"),
	    Err(_) => true,
	})
    }
    #[test]
    fn normalizedstring_invalid_all() {
        let r = NormalizedString::new("contains	all\rforbidden\ncharacters");
	assert!(match r {
	    Ok(_) => panic!("string contains at least one forbidden character"),
	    Err(_) => true,
	})
    }

// Numeric is in the too hard basket for now
//    #[test]
//    fn numeric_float() {
//        assert_eq!(Numeric::new(f32::0.123).value, 0.123);
//    }
//    #[test]
//    fn numeric_double() {
//        assert_eq!(Numeric::new(f64::0.456).value, 0.456);
//    }
//    #[test]
//    fn numeric_decimal() {
//        assert_eq!(Numeric::new(dec!(123.456)), 123.456);
//    }

    #[test]
    fn nonpositiveinteger_valid() {
        assert_eq!(NonPositiveInteger::new(-10).expect("invalid NonPositiveInteger").value, -10);
    }
    #[test]
    fn nonpositiveinteger_valid_zero() {
        assert_eq!(NonPositiveInteger::new(0).expect("invalid NonPositiveInteger").value, 0);
    }
    #[test]
    fn nonpositiveinteger_invalid() {
        let r = NonPositiveInteger::new(10);
	assert!(match r {
	    Ok(_) => panic!("10 is not a nonPositiveInteger"),
	    Err(_) => true,
	})
    }

    #[test]
    fn positiveinteger_valid() {
        assert_eq!(PositiveInteger::new(10).expect("invalid PositiveInteger").value, 10);
    }
    #[test]
    fn positiveinteger_invalid_zero() {
        let r = PositiveInteger::new(0);
	assert!(match r {
	    Ok(_) => panic!("0 is not a PositiveInteger"),
	    Err(_) => true,
	})
    }
    #[test]
    fn positiveinteger_invalid() {
        let r = PositiveInteger::new(-10);
	assert!(match r {
	    Ok(_) => panic!("-10 is not a PositiveInteger"),
	    Err(_) => true,
	})
    }

    #[test]
    fn nonnegativeinteger_valid() {
        assert_eq!(NonNegativeInteger::new(10).expect("invalid NonNegativeInteger").value, 10);
    }
    #[test]
    fn nonnegativeinteger_valid_zero() {
        assert_eq!(NonNegativeInteger::new(0).expect("invalid NonNegativeInteger").value, 0);
    }
    #[test]
    fn nonnegativeinteger_invalid() {
        let r = NonNegativeInteger::new(-10);
	assert!(match r {
	    Ok(_) => panic!("-10 is not a NonNegativeInteger"),
	    Err(_) => true,
	})
    }

    #[test]
    fn negativeinteger_valid() {
        assert_eq!(NegativeInteger::new(-10).expect("invalid NegativeInteger").value, -10);
    }
    #[test]
    fn negativeinteger_invalid_zero() {
        let r = NegativeInteger::new(0);
	assert!(match r {
	    Ok(_) => panic!("0 is not a NegativeInteger"),
	    Err(_) => true,
	})
    }
    #[test]
    fn negativeinteger_invalid() {
        let r = NegativeInteger::new(10);
	assert!(match r {
	    Ok(_) => panic!("10 is not a NegativeInteger"),
	    Err(_) => true,
	})
    }

    // String Values
    #[test]
    fn string_stringvalue() {
        assert_eq!(Value::String("foobar".to_string()).to_string(), "foobar")
    }
    #[test]
    fn decimal_stringvalue() {
        assert_eq!(Value::Decimal(dec!(001.23)).to_string(), "1.23")
    }
    #[test]
    fn float_stringvalue() {
        assert_eq!(Value::Float(001.2300_f32).to_string(), "1.23")
    }
    #[test]
    fn nonpositiveinteger_stringvalue() {
        let npi = NonPositiveInteger::new(-00123).expect("invalid nonPositiveInteger");
	let i = Value::NonPositiveInteger(npi);
        assert_eq!(i.to_string(), "-123")
    }
    #[test]
    fn nonnegativeinteger_stringvalue() {
        let nni = NonNegativeInteger::new(00123).expect("invalid nonNegativeInteger");
	let i = Value::NonNegativeInteger(nni);
        assert_eq!(i.to_string(), "123")
    }
    #[test]
    fn normalizedstring_stringvalue() {
        let ns = NormalizedString::new("foobar").expect("invalid normalizedString");
	let i = Value::NormalizedString(ns);
        assert_eq!(i.to_string(), "foobar")
    }

    // to_bool

    #[test]
    fn bool_value_string_empty() {
      assert_eq!(Item::Value(Value::String("".to_string())).to_bool(), false)
    }
    #[test]
    fn bool_value_string_nonempty() {
      assert_eq!(Item::Value(Value::String("false".to_string())).to_bool(), true)
    }
    #[test]
    fn bool_value_int_zero() {
      assert_eq!(Item::Value(Value::Integer(0)).to_bool(), false)
    }
    #[test]
    fn bool_value_int_nonzero() {
      assert_eq!(Item::Value(Value::Integer(42)).to_bool(), true)
    }

    // to_int

    #[test]
    fn int_value_string() {
      match Item::Value(Value::String("1".to_string())).to_int() {
        Ok(i) => assert_eq!(i, 1),
	Err(_) => {
	  panic!("to_int() failed")
	}
      }
    }

    // to_double

    #[test]
    fn double_value_string() {
      assert_eq!(Item::Value(Value::String("2.0".to_string())).to_double(), 2.0)
    }

    // value to_bool

    #[test]
    fn value_to_bool_string() {
      assert_eq!(Value::String("2".to_string()).to_bool(), true)
    }

    // value to_int

    #[test]
    fn value_to_int_string() {
      assert_eq!(Value::String("2".to_string()).to_int().expect("cannot convert to integer"), 2)
    }

    // value to_double

    #[test]
    fn value_to_double_string() {
      assert_eq!(Value::String("3.0".to_string()).to_double(), 3.0)
    }

    // value compare

    #[test]
    fn value_compare_eq() {
      assert_eq!(Value::String("3".to_string()).compare(&Item::Value(Value::Double(3.0)), Operator::Equal).expect("unable to compare"), true)
    }

    #[test]
    fn value_compare_ne() {
      assert_eq!(Value::String("3".to_string()).compare(&Item::Value(Value::Double(3.0)), Operator::NotEqual).expect("unable to compare"), false)
    }

    //#[test]
    //fn value_atomize() {
	//let i = Value::Int(123);
        //assert_eq!(i.atomize().stringvalue(), "123")
    //}

    // Sequences

    #[test]
    fn sequence() {
        let _s = Sequence::new();
        assert!(true)
    }
    #[test]
    fn sequence_one() {
        let mut s = Sequence::new();
	s.new_value(Value::String("one".to_string()));
	let mut t = Sequence::new();
	t.add(&s[0]);
	assert!(Rc::ptr_eq(&s[0], &t[0]))
    }

    // Operators
    #[test]
    fn op_equal() {
      assert_eq!(Operator::Equal.to_string(), "=")
    }
    #[test]
    fn op_notequal() {
      assert_eq!(Operator::NotEqual.to_string(), "!=")
    }
    #[test]
    fn op_lt() {
      assert_eq!(Operator::LessThan.to_string(), "<")
    }
    #[test]
    fn op_ltequal() {
      assert_eq!(Operator::LessThanEqual.to_string(), "<=")
    }
    #[test]
    fn op_gt() {
      assert_eq!(Operator::GreaterThan.to_string(), ">")
    }
    #[test]
    fn op_gtequal() {
      assert_eq!(Operator::GreaterThanEqual.to_string(), ">=")
    }
    #[test]
    fn op_is() {
      assert_eq!(Operator::Is.to_string(), "is")
    }
    #[test]
    fn op_before() {
      assert_eq!(Operator::Before.to_string(), "<<")
    }
    #[test]
    fn op_after() {
      assert_eq!(Operator::After.to_string(), ">>")
    }
}
