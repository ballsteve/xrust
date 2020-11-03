//! # xdm::item
//!
//! Sequence Item module.
//! An Item is a Node, Function or Atomic Value.

use decimal;

use crate::xdmerror::{Error, ErrorKind};

#[derive(Clone)]
pub enum Item {
    Node,
    Function,
    Value(Value),
}

pub trait StringValue {
    fn stringvalue(&self) -> String;
}

impl StringValue for Item {
    fn stringvalue(&self) -> String {
        match self {
	    Item::Value(v) => v.stringvalue(),
	    _ => String::from(""),
	}
    }
}

// TODO: atomize is low priority for now
//pub trait Atomize {
//    fn atomize(&self) -> Result<Item, Error>;
//}

//impl Atomize for Item {
//    fn atomize(&self) -> Result<Item, Error> {
//        match self {
//	    Item::Value(_) => Ok(self.clone()),
//	    Item::Function => Err(Error{kind: ErrorKind::TypeError, message: String::from("cannot atomize a function")}),
//	    Item::Node => Ok(self.clone()), // TODO return typed value
//	}
//    }
//}

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

impl StringValue for Value {
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
}

// TODO defining a union is in the too hard basket for now
// Numeric is a union of double, float and decimal
//pub enum NumericType {
//    Double,
//    Float,
//    Decimal,
//}
//pub struct Numeric {
//    numerictype: NumericType,
//    double: f64,
//    float: f32,
//    decimal: decimal::d128,
//}

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
        assert_eq!(Item::Value(Value::String(String::from("foobar"))).stringvalue(), "foobar")
    }
    #[test]
    fn decimal_stringvalue() {
        assert_eq!(Item::Value(Value::Decimal(decimal::d128!(001.23))).stringvalue(), "1.23")
    }
    #[test]
    fn float_stringvalue() {
        assert_eq!(Item::Value(Value::Float(001.2300_f32)).stringvalue(), "1.23")
    }
    #[test]
    fn nonpositiveinteger_stringvalue() {
        let npi = NonPositiveInteger::new(-00123).expect("invalid nonPositiveInteger");
	let i = Item::Value(Value::NonPositiveInteger(npi));
        assert_eq!(i.stringvalue(), "-123")
    }
    #[test]
    fn nonnegativeinteger_stringvalue() {
        let nni = NonNegativeInteger::new(00123).expect("invalid nonNegativeInteger");
	let i = Item::Value(Value::NonNegativeInteger(nni));
        assert_eq!(i.stringvalue(), "123")
    }
    #[test]
    fn normalizedstring_stringvalue() {
        let ns = NormalizedString::new(String::from("foobar")).expect("invalid normalizedString");
	let i = Item::Value(Value::NormalizedString(ns));
        assert_eq!(i.stringvalue(), "foobar")
    }

    //#[test]
    //fn value_atomize() {
	//let i = Item::Value(Value::Int(123));
        //assert_eq!(i.atomize().stringvalue(), "123")
    //}
}
