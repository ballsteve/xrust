//! # xdm::item
//!
//! Sequence Item module.
//! An Item is a Node, Function or Atomic Value.

use std::cmp::Ordering;
use std::rc::Rc;
use decimal;
use crate::xdmerror::{Error, ErrorKind};

pub type Sequence<'a> = Vec<Rc<Item<'a>>>;

pub trait SequenceTrait<'a> {
  fn to_string(&self) -> String;
  fn to_bool(&self) -> bool;
  fn new_node(&mut self, n: Node);
  fn new_value(&mut self, v: Value<'a>);
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
  fn new_node(&mut self, n: Node) {
    self.push(Rc::new(Item::Node(n)));
  }
  fn new_value(&mut self, v: Value<'a>) {
    self.push(Rc::new(Item::Value(v)));
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
}

#[derive(Clone)]
pub enum Item<'a> {
    Node(Node),
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

impl<'a> Item<'a> {
  // Gives the string value of an item. All items have a string value.
  pub fn to_string(&self) -> String {
    match self {
      Item::Node(n) => n.to_string(),
      Item::Function => "".to_string(),
      Item::Value(v) => v.to_string(),
    }
  }
  // Should there also be a string slice version?
  // fn to_str(&self) -> &str;

  // Determine the effective boolean value of a sequence.
  // See XPath 2.4.3.
  pub fn to_bool(&self) -> bool {
    match self {
      Item::Node(_) => true,
      Item::Function => false,
      Item::Value(v) => v.to_bool(),
    }
  }

  pub fn to_int(&self) -> Result<i64, Error> {
    match self {
      Item::Node(_n) => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("type error: item is a node")}),
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

  pub fn to_double(&self) -> f64 {
    match self {
      Item::Node(_) => f64::NAN,
      Item::Function => f64::NAN,
      Item::Value(v) => v.to_double(),
    }
  }

  // TODO: atomization
  // fn atomize(&self);

  pub fn compare(&self, other: &Item, op: Operator) -> Result<bool, Error> {
    match self {
      Item::Value(v) => {
        v.compare(other, op)
      }
      Item::Node(_n) => {
        //n.compare(other, op)
	Result::Err(Error{kind: ErrorKind::NotImplemented, message: String::from("not yet implemented")})
      }
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
pub struct Node {
  nodetype: NodeType,
  name: Option<String>, // TODO: make it a QName
  value: Option<String>,
  attributes: Option<Vec<Node>>,
  content: Option<Vec<Node>>,
}

impl Node {
  pub fn new(t: NodeType) -> Node {
    Node{nodetype: t, name: None, value: None, attributes: None, content: None}
  }
  pub fn nodetype(&self) -> NodeType {
    self.nodetype
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
  pub fn set_attributes(mut self, a: Vec<Node>) -> Self {
    if a.len() == 0 {
      self.attributes = None;
    } else {
      self.attributes.replace(a);
    }
    self
  }
  pub fn attributes(&self) -> Option<&Vec<Node>> {
    self.attributes.as_ref().map(|a| a)
  }
  pub fn set_content(mut self, c: Vec<Node>) -> Self {
    if c.len() == 0 {
      self.content = None;
    } else {
      self.content.replace(c);
    }
    self
  }
  pub fn content(&self) -> Option<&Vec<Node>> {
    self.content.as_ref().map(|c| c)
  }

  pub fn prepend_node(mut self, n: Node) {
    match self.content {
      Some(mut v) => {v.insert(0, n);},
      None => {
        self.content.replace(vec![n]);
      },
    }
  }
  pub fn prepend_seq(&mut self, s: Vec<Node>) {
    let mut new: Vec<Node>;

    if self.content.is_some() {
      new = self.content.take().unwrap();
      for i in s {
	new.insert(0, i);
      }
    } else {
      new = s;
    }
    self.content.replace(new);
  }
  pub fn append_node(mut self, n: Node) {
    match self.content {
      Some(mut v) => {v.push(n);},
      None => {
        self.content.replace(vec![n]);
      },
    }
  }
  pub fn append_seq(mut self, s: Vec<Node>) {
    match self.content {
      Some(mut v) => {
        for i in s {
	  v.push(i);
	}
      },
      None => {self.content.replace(s);},
    }
  }

  pub fn to_string(&self) -> String {
    match self.nodetype {
      NodeType::Document => {
        let mut r = String::new();

        for i in self.content.as_ref().unwrap_or(&vec![]) {
	  r.push_str(i.to_string().as_str())
	}
	r
      }
      NodeType::Element => {
        let mut r = String::from("<");
	r.push_str(self.name());
	for i in self.attributes.as_ref().unwrap_or(&vec![]) {
	  r.push_str(" ");
	  r.push_str(&i.to_string())
	}
	r.push_str(">");
	for i in self.content.as_ref().unwrap_or(&vec![]) {
	  r.push_str(&i.to_string())
	}
	r.push_str("</");
	r.push_str(self.name());
	r.push_str(">");
	r
      }
      NodeType::Text => {
        String::from(self.value())
      }
      NodeType::Attribute => {
        let mut r = String::new();
        r.push_str(self.name());
        r.push_str("='");
        r.push_str(self.value());
        r.push_str("'");
        // TODO: delimiters, escaping
	r
      }
      NodeType::Comment => {
        let mut r = String::new();
        r.push_str("<!--");
        r.push_str(self.value());
        r.push_str("-->");
	r
      }
      NodeType::ProcessingInstruction => {
        let mut r = String::new();
        r.push_str("<?");
        r.push_str(self.name());
        r.push_str(" ");
        r.push_str(self.value());
        r.push_str("?>");
	r
      }
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
    fn to_string(&self) -> String {
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
	  //println!("compare strings {} to {}", s, t);
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
	  println!("to_int() failed: {}", e.message);
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
        Node::new(NodeType::Document);
        assert!(true)
    }
    #[test]
    fn node_element() {
        let e = Node::new(NodeType::Element).set_name("Test".to_string());
        assert_eq!(e.to_string(), "<Test></Test>")
    }
    #[test]
    fn node_text() {
        let t = Node::new(NodeType::Text).set_value("Test text".to_string());
        assert_eq!(t.to_string(), "Test text")
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
}
