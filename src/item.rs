//! # xdm::item
//!
//! Sequence Item module.
//! An Item is a Node, Function or Atomic Value.

use std::cmp::Ordering;
use decimal;
use crate::xdmerror::{Error, ErrorKind};

//#[derive(Clone)]
//pub enum Item<N, F> {
//    Node(N),
//    Function(F),
//    Value(Value),
//}

pub trait Item<'a> {
  // All items have a string value
  fn stringvalue(&self) -> String;

  // Determine the effective boolean value of a sequence.
  // See XPath 2.4.3.
  fn to_bool(&self) -> bool;

  // TODO: atomization
  // fn atomize(&self);

  // Functions for Node items
  fn doc(&self) -> Result<Box<dyn Item<'a> + 'a>, Error>;
  fn children(&self) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error>;
  fn parent(&self) -> Result<Box<dyn Item<'a> + 'a>, Error>;
}

// A concrete type that implements atomic values

impl PartialEq for Value {
  fn eq(&self, other: &Value) -> bool {
    match self {
        Value::String(s) => s.eq(&other.stringvalue()),
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
        Value::String(s) => s.partial_cmp(&other.stringvalue()),
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
pub enum Value {
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
    String(String), // TODO: consider using a string slice instead
    NormalizedString(NormalizedString),
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

impl<'a> Item<'a> for Value {
    fn stringvalue(&self) -> String {
	match self {
	    Value::String(s) => s.to_string(),
	    Value::NormalizedString(s) => String::from(s.value.as_str()), // TODO: this copies the value, so no good for large strings
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
 	    _ => String::from(""),
	}
    }

    fn to_bool(&self) -> bool {
        match &self {
            Value::Boolean(b) => *b == true,
            Value::String(t) => {
                //t.is_empty()
	        t.as_str().len() != 0
            },
            Value::Double(n) => *n != 0.0,
            Value::Integer(i) => *i != 0,
            _ => false
	}
    }

    fn doc(&self) -> Result<Box<dyn Item<'a> + 'a>, Error> {
      Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("atomic values don't have a document")})
    }
    fn children(&self) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
      Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("atomic values don't have children")})
    }
    fn parent(&self) -> Result<Box<dyn Item<'a> + 'a>, Error> {
      Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("atomic values don't have a parent")})
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
    pub fn new(v: String) -> Result<Self, Error> {
        let n: &[_] = &['\n', '\r', '\t'];
        match v.find(n) {
	    None => Ok(NormalizedString{value: v}),
	    _ => Err(Error {
	        kind: ErrorKind::TypeError,
		message: String::from("value is not a normalized string"),
	  	})
	}
    }
    pub fn value(self) -> String {
        self.value
    }
}
impl Clone for NormalizedString {
    fn clone(&self) -> Self {
        NormalizedString::new(self.value.clone()).expect("unable to clone NormalizedString")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizedstring_valid_empty() {
        assert_eq!(NormalizedString::new(String::from("")).expect("invalid NormalizedString").value, "");
    }
    #[test]
    fn normalizedstring_valid() {
        assert_eq!(NormalizedString::new(String::from("notinvalid")).expect("invalid NormalizedString").value, "notinvalid");
    }
    #[test]
    fn normalizedstring_valid_spaces() {
        assert_eq!(NormalizedString::new(String::from("not an invalid string")).expect("invalid NormalizedString").value, "not an invalid string");
    }
    #[test]
    fn normalizedstring_invalid_tab() {
        let r = NormalizedString::new(String::from("contains tab	character"));
	assert!(match r {
	    Ok(_) => panic!("string contains tab character"),
	    Err(_) => true,
	})
    }
    #[test]
    fn normalizedstring_invalid_newline() {
        let r = NormalizedString::new(String::from("contains newline\ncharacter"));
	assert!(match r {
	    Ok(_) => panic!("string contains newline character"),
	    Err(_) => true,
	})
    }
    #[test]
    fn normalizedstring_invalid_cr() {
        let r = NormalizedString::new(String::from("contains carriage return\rcharacter"));
	assert!(match r {
	    Ok(_) => panic!("string contains cr character"),
	    Err(_) => true,
	})
    }
    #[test]
    fn normalizedstring_invalid_all() {
        let r = NormalizedString::new(String::from("contains	all\rforbidden\ncharacters"));
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
        assert_eq!(Value::String(String::from("foobar")).stringvalue(), "foobar")
    }
    #[test]
    fn decimal_stringvalue() {
        assert_eq!(Value::Decimal(decimal::d128!(001.23)).stringvalue(), "1.23")
    }
    #[test]
    fn float_stringvalue() {
        assert_eq!(Value::Float(001.2300_f32).stringvalue(), "1.23")
    }
    #[test]
    fn nonpositiveinteger_stringvalue() {
        let npi = NonPositiveInteger::new(-00123).expect("invalid nonPositiveInteger");
	let i = Value::NonPositiveInteger(npi);
        assert_eq!(i.stringvalue(), "-123")
    }
    #[test]
    fn nonnegativeinteger_stringvalue() {
        let nni = NonNegativeInteger::new(00123).expect("invalid nonNegativeInteger");
	let i = Value::NonNegativeInteger(nni);
        assert_eq!(i.stringvalue(), "123")
    }
    #[test]
    fn normalizedstring_stringvalue() {
        let ns = NormalizedString::new(String::from("foobar")).expect("invalid normalizedString");
	let i = Value::NormalizedString(ns);
        assert_eq!(i.stringvalue(), "foobar")
    }

    //#[test]
    //fn value_atomize() {
	//let i = Value::Int(123);
        //assert_eq!(i.atomize().stringvalue(), "123")
    //}
}
