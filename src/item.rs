//! # xdm::item
//!
//! Sequence Item module.
//! An Item is a Node, Function or Atomic Value.

use std::cmp::Ordering;
use std::rc::Rc;
//use std::cell::RefCell;
use decimal;
use crate::xdmerror::{Error, ErrorKind};
use trees::{Tree, RcNode};
use roxmltree::Node;
use json::JsonValue;

pub type Sequence<'a> = Vec<Rc<Item<'a>>>;

pub trait SequenceTrait<'a> {
  //fn clone(&self) -> Sequence;
  fn to_string(&self) -> String;
  fn to_bool(&self) -> bool;
  fn to_int(&self) -> Result<i64, Error>;
  fn new_node(&mut self, n: RcNode<NodeDefn>);
  //fn new_xdoc(&mut self, d: Document<'a>);
  fn new_xnode(&mut self, n: Node<'a, 'a>);
  fn new_value(&mut self, v: Value<'a>);
  fn new_jvalue(&mut self, j: JsonValue);
  fn add(&mut self, i: &Rc<Item<'a>>);
}

impl<'a> SequenceTrait<'a> for Sequence<'a> {
  fn to_string(&self) -> String {
    let mut r = String::new();
    for i in self {
      r.push_str(i.to_string().as_str())
    }
    r
  }
  fn new_node(&mut self, n: RcNode<NodeDefn>) {
    self.push(Rc::new(Item::Node(n)));
  }
//  fn new_xdoc(&mut self, d: Document<'a>) {
//    self.push(Rc::new(Item::XDoc(d)));
//  }
  fn new_xnode(&mut self, n: Node<'a, 'a>) {
    self.push(Rc::new(Item::XNode(n)));
  }
  fn new_value(&mut self, v: Value<'a>) {
    self.push(Rc::new(Item::Value(v)));
  }
  fn new_jvalue(&mut self, j: JsonValue) {
    self.push(Rc::new(Item::JsonValue(j)));
  }
  //fn new_function(&self, f: Function) -> Sequence {
  //}
  fn add(&mut self, i: &Rc<Item<'a>>) {
    self.push(Rc::clone(i));
  }

  // Calculate the effective boolean value
  fn to_bool(&self) -> bool {
    if self.len() == 0 {
      false
    } else {
      match *self[0] {
        Item::Node(_) |
	Item::XNode(_) => true,
	Item::JsonValue(_) => true,
	//Item::XDoc(_) => true,
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

  // Convenience routine for integer
  fn to_int(&self) -> Result<i64, Error> {
    if self.len() == 1 {
      self[0].to_int()
    } else {
      Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("type error: sequence is not a singleton")})
    }
  }
}

#[derive(Clone)]
pub enum Item<'a> {
    Node(RcNode<NodeDefn>),
    XNode(Node<'a, 'a>),
    JsonValue(JsonValue),
    //XDoc(Document<'a>), cannot be cloned
    Function,
    Value(Value<'a>),
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

impl<'a> Item<'a> {
  // Gives the string value of an item. All items have a string value.
  // TODO: properly implement string value.
  // At the moment this pretty-prints the item, which includes markup then it shouldn't
  pub fn to_string(&self) -> String {
    match self {
      Item::Node(n) => node_to_string(n),
      Item::XNode(n) => xnode_to_string(*n),
      //Item::XDoc(d) => d.to_string(),
      Item::Function => "".to_string(),
      Item::Value(v) => v.to_string(),
      Item::JsonValue(j) => j.pretty(0),
    }
  }
  // Should there also be a string slice version?
  // fn to_str(&self) -> &str;

  // Determine the effective boolean value of a sequence.
  // See XPath 2.4.3.
  pub fn to_bool(&self) -> bool {
    match self {
      Item::Node(_) |
      Item::XNode(_) => true,
      //Item::XDoc(_) => true,
      Item::Function => false,
      Item::Value(v) => v.to_bool(),
      Item::JsonValue(j) => true,
    }
  }

  pub fn to_int(&self) -> Result<i64, Error> {
    match self {
      Item::Node(_) |
      Item::XNode(_) => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("type error: item is a node")}),
      //Item::XDoc(_) => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("type error: item is a node")}),
      Item::Function => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("type error: item is a function")}),
      Item::JsonValue(_) => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("type error: item is a json value")}),
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

  pub fn to_double(&self) -> f64 {
    match self {
      Item::Node(_) |
      Item::XNode(_) => f64::NAN,
      //Item::XDoc(_) => f64::NAN,
      Item::Function => f64::NAN,
      Item::JsonValue(_) => f64::NAN,
      Item::Value(v) => v.to_double(),
    }
  }

  pub fn to_name(&self) -> &str {
    match self {
      Item::XNode(i) => {
        match i.node_type() {
	  roxmltree::NodeType::Root => "",
	  roxmltree::NodeType::Element |
	  roxmltree::NodeType::PI => i.tag_name().name(),
	  roxmltree::NodeType::Text |
	  roxmltree::NodeType::Comment => "",
	}
      }
      _ => ""
    }
  }

  // TODO: atomization
  // fn atomize(&self);

  pub fn compare(&self, other: &Item, op: Operator) -> Result<bool, Error> {
    match self {
      Item::Value(v) => {
        v.compare(other, op)
      }
      Item::Node(_) |
      Item::XNode(_) |
      Item::JsonValue(_) => {
        //n.compare(other, op)
	Result::Err(Error{kind: ErrorKind::NotImplemented, message: String::from("not yet implemented")})
      }
      //Item::XDoc(_) => {
        //n.compare(other, op)
	//Result::Err(Error{kind: ErrorKind::NotImplemented, message: String::from("not yet implemented")})
      //}
      _ => {
        Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("type error")})
      }
    }
  }
}

// Node in a tree

#[derive(Copy, Clone)]
pub enum NodeType {
  Document,
  Element,
  Text,
  Attribute,
  Comment,
  ProcessingInstruction,
}

#[derive(Clone)]
pub struct NodeDefn {
  nodetype: NodeType,
  name: Option<String>, // TODO: make it a QName
  value: Option<String>,
}

impl NodeDefn {
  pub fn new(t: NodeType) -> Self {
    NodeDefn {
      nodetype: t,
      name: None,
      value: None,
    }
  }
  pub fn nodetype(&self) -> &NodeType {
    &self.nodetype
  }
  pub fn set_name(mut self, n: String) -> Self {
    // TODO: restrict which types can have a name
    self.name.replace(n);
    self
  }
  pub fn name(&self) -> &str {
    // None => empty string
    if self.name.is_some() {
      self.name.as_ref().unwrap()
    } else {
      ""
    }
  }
  pub fn set_value(mut self, v: String) -> Self {
    self.value.replace(v);
    self
  }
  pub fn value(&self) -> &str {
    if self.value.is_some() {
      self.value.as_ref().unwrap()
    } else {
      ""
    }
  }
}

pub fn node_to_string(node: &RcNode<NodeDefn>) -> String {
  let d = node.data();

  match d.nodetype {
      NodeType::Document => {
        if node.has_no_child() {
	  String::new()
	} else {
	  node.iter_rc().fold(String::new(), |s,c| s + &node_to_string(&c))
	}
      }
      NodeType::Element => {
        if node.has_no_child() {
	  format!("<{}/>", d.name.as_ref().unwrap())
	} else {
	  // TODO: attributes
	  format!("<{}>{}</{}>", d.name.as_ref().unwrap(), node.iter_rc().fold(String::new(), |s,c| s + &node_to_string(&c)), d.name.as_ref().unwrap())
	}
      }
      NodeType::Text => {
        String::from(d.value.as_ref().unwrap())
      }
      NodeType::Attribute => {
        let mut r = String::new();
        r.push_str(d.name.as_ref().unwrap().as_str());
        r.push_str("='");
        r.push_str(d.value.as_ref().unwrap().as_str());
        r.push_str("'");
        // TODO: delimiters, escaping
	r
      }
      NodeType::Comment => {
        let mut r = String::new();
        r.push_str("<!--");
        r.push_str(d.value.as_ref().unwrap().as_str());
        r.push_str("-->");
	r
      }
      NodeType::ProcessingInstruction => {
        let mut r = String::new();
        r.push_str("<?");
        r.push_str(d.name.as_ref().unwrap().as_str());
        r.push_str(" ");
        r.push_str(d.value.as_ref().unwrap().as_str());
        r.push_str("?>");
	r
      }
  }
}

pub fn xnode_to_string(node: Node) -> String {
  match node.node_type() {
      roxmltree::NodeType::Root => {
        if node.has_children() {
	  xnode_to_string(node.first_child().unwrap())
	} else {
	  String::new()
	}
      }
      roxmltree::NodeType::Element => {
        if node.has_children() {
	  // TODO: attributes
	  format!("<{}>{}</{}>", node.tag_name().name(), node.children().fold(String::new(), |s,c| s + &xnode_to_string(c)), node.tag_name().name())
	} else {
	  format!("<{}/>", node.tag_name().name())
	}
      }
      roxmltree::NodeType::Text => {
        String::from(node.text().unwrap_or(""))
      }
      roxmltree::NodeType::Comment => {
        let mut r = String::new();
        r.push_str("<!--");
        r.push_str(node.text().unwrap_or(""));
        r.push_str("-->");
	r
      }
      roxmltree::NodeType::PI => {
        let mut r = String::new();
        r.push_str("<?");
        r.push_str(node.tag_name().name());
        r.push_str(" ");
        r.push_str(node.text().unwrap_or(""));
        r.push_str("?>");
	r
      }
  }
}

// A concrete type that implements atomic values

impl<'a> PartialEq for Value<'a> {
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
impl<'a> PartialOrd for Value<'a> {
  fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
    match self {
        Value::String(s) => {
	  let o: String = other.to_string();
	  s.partial_cmp(&o.as_str())
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

#[derive(Clone)]
pub enum Value<'a> {
    AnyType, // node or simple type
    Untyped, // a not-yet-valildated anyType
    AnySimpleType, // base type of all simple types. i.e. not a node
    IDREFS, // a list of IDREF
    NMTOKENS, // a list of NMTOKEN
    ENTITIES, // a list of ENTITY
    Numeric, // (Numeric)
    AnyAtomicType, // all atomic values (no lists or unions)
    UntypedAtomic, // untyped atomic value
    Duration,
    Time,
    Decimal(decimal::d128),
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
    DateTime,
    DateTimeStamp,
    Date,
    String(&'a str), // Items never change, so no need for a String
    StringOwned(String),	// Except that ownership should be with the Value
    				// TODO: resolve this
    NormalizedString(NormalizedString<'a>),
    Token, // TODO like normalizedString, but without leading, trailing and consecutive whitespace
    Language, // language identifiers [a-zA-Z]{1,8}(-[a-zA-Z0-9]{1,8})*
    NMTOKEN, // NameChar+
    Name, // NameStartChar NameChar+
    NCName, // (Letter | '_') NCNameChar+ (i.e. a Name without the colon)
    ID, // NCName
    IDREF, // NCName
    ENTITY, // NCName
    Boolean(bool),
}

impl<'a> Value<'a> {
    pub fn to_string(&self) -> String {
	match self {
	    Value::String(s) => s.to_string(),
	    Value::StringOwned(s) => s.to_string(),
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
 	    _ => "".to_string(),
	}
    }

    fn to_bool(&self) -> bool {
        match &self {
            Value::Boolean(b) => *b == true,
            Value::String(t) => {
                //t.is_empty()
	        t.len() != 0
            },
            Value::StringOwned(t) => {
                //t.is_empty()
	        t.len() != 0
            },
	    Value::NormalizedString(s) => s.value.len() != 0,
            Value::Double(n) => *n != 0.0,
            Value::Integer(i) => *i != 0,
            _ => false
	}
    }

    fn to_int(&self) -> Result<i64, Error> {
        match &self {
	    Value::String(s) => {
	      match s.parse::<i64>() {
	        Ok(i) => Ok(i),
		Err(e) => Result::Err(Error{kind: ErrorKind::Unknown, message: format!("type conversion error: {}", e)}),
	      }
	    }
	    Value::StringOwned(s) => {
	      match s.parse::<i64>() {
	        Ok(i) => Ok(i),
		Err(e) => Result::Err(Error{kind: ErrorKind::Unknown, message: format!("type conversion error: {}", e)}),
	      }
	    }
            Value::Integer(i) => Ok(*i),
            _ => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("type error (conversion not implemented)")})
	}
    }
    fn to_double(&self) -> f64 {
        match &self {
	    Value::String(s) => {
	      match s.parse::<f64>() {
	        Ok(i) => i,
		Err(_) => f64::NAN,
	      }
	    }
	    Value::StringOwned(s) => {
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
    fn compare(&self, other: &Item, op: Operator) -> Result<bool, Error> {
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
	_ => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not yet implemented")})
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

pub struct NormalizedString<'a> {
    value: &'a str,
}
impl<'a> NormalizedString<'a> {
    pub fn new(v: &'a str) -> Result<Self, Error> {
        let n: &[_] = &['\n', '\r', '\t'];
        match v.find(n) {
	    None => Ok(NormalizedString{value: v}),
	    _ => Err(Error {
	        kind: ErrorKind::TypeError,
		message: String::from("value is not a normalized string"),
	  	})
	}
    }
    pub fn value(self) -> &'a str {
        self.value
    }
}
impl<'a> Clone for NormalizedString<'a> {
    fn clone(&self) -> Self {
        NormalizedString::new(self.value.clone()).expect("unable to clone NormalizedString")
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
//        assert_eq!(Numeric::new(decimal::d128!(123.456)).value, 123.456);
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
        assert_eq!(Value::String("foobar").to_string(), "foobar")
    }
    #[test]
    fn decimal_stringvalue() {
        assert_eq!(Value::Decimal(decimal::d128!(001.23)).to_string(), "1.23")
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
      assert_eq!(Item::Value(Value::String("")).to_bool(), false)
    }
    #[test]
    fn bool_value_string_nonempty() {
      assert_eq!(Item::Value(Value::String("false")).to_bool(), true)
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
      match Item::Value(Value::String("1")).to_int() {
        Ok(i) => assert_eq!(i, 1),
	Err(e) => {
	  panic!("to_int() failed")
	}
      }
    }

    // to_double

    #[test]
    fn double_value_string() {
      assert_eq!(Item::Value(Value::String("2.0")).to_double(), 2.0)
    }

    // value to_bool

    #[test]
    fn value_to_bool_string() {
      assert_eq!(Value::String("2").to_bool(), true)
    }

    // value to_int

    #[test]
    fn value_to_int_string() {
      assert_eq!(Value::String("2").to_int().expect("cannot convert to integer"), 2)
    }

    // value to_double

    #[test]
    fn value_to_double_string() {
      assert_eq!(Value::String("3.0").to_double(), 3.0)
    }

    // value compare

    #[test]
    fn value_compare_eq() {
      assert_eq!(Value::String("3").compare(&Item::Value(Value::Double(3.0)), Operator::Equal).expect("unable to compare"), true)
    }

    #[test]
    fn value_compare_ne() {
      assert_eq!(Value::String("3").compare(&Item::Value(Value::Double(3.0)), Operator::NotEqual).expect("unable to compare"), false)
    }

    //#[test]
    //fn value_atomize() {
	//let i = Value::Int(123);
        //assert_eq!(i.atomize().stringvalue(), "123")
    //}

    // Nodes

    #[test]
    fn node_document() {
        RcNode::from(Tree::new(NodeDefn::new(NodeType::Document)));
        assert!(true)
    }
    #[test]
    fn node_element() {
        let d = RcNode::from(Tree::new(NodeDefn::new(NodeType::Document)));
        let e = Tree::new(NodeDefn::new(NodeType::Element).set_name("Test".to_string()));
	d.push_front(e);
        assert_eq!(node_to_string(&d), "<Test/>")
    }
    #[test]
    fn node_text() {
        let d = RcNode::from(Tree::new(NodeDefn::new(NodeType::Document)));
        let mut e = Tree::new(NodeDefn::new(NodeType::Element).set_name("Test".to_string()));
        let t = Tree::new(NodeDefn::new(NodeType::Text).set_value("Test text".to_string()));
	e.push_front(t);
	d.push_front(e);
        assert_eq!(node_to_string(&d), "<Test>Test text</Test>")
    }
    #[test]
    fn item_node_to_string() {
        let d = RcNode::from(Tree::new(NodeDefn::new(NodeType::Document)));
        let mut e = Tree::new(NodeDefn::new(NodeType::Element).set_name("Test".to_string()));
        let t = Tree::new(NodeDefn::new(NodeType::Text).set_value("Test text".to_string()));
	e.push_front(t);
	d.push_front(e);
	let i = Item::Node(d);
        assert_eq!(i.to_string(), "<Test>Test text</Test>")
    }

    // Documents and Nodes using roxmltree
//    #[test]
//    fn xnode_doc() {
//      let d = roxmltree::Document::parse("<Test/>").expect("unable to parse XML");
//      let i = Item::XDoc(d);
//      assert_eq!(i.to_string(), "<Test/>")
//    }
    #[test]
    fn xnode_node() {
      let d = roxmltree::Document::parse("<Test><Level2>test text</Level2></Test>").expect("unable to parse XML");
      let i = Item::XNode(d.root().first_child().unwrap().first_child().unwrap());
      assert_eq!(i.to_string(), "<Level2>test text</Level2>")
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
	s.new_value(Value::String("one"));
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

    // Json

    #[test]
    fn json_value() {
      let i = Item::JsonValue(JsonValue::String("this is json".to_string()));
      assert_eq!(i.to_string(), "\"this is json\"")
    }
}
