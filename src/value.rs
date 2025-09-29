//! An atomic value.
//!
//! An atomic value that is an item in a sequence.

use qualname::{NamespaceUri, NcName, QName};

use crate::output::OutputSpec;
use crate::xdmerror::{Error, ErrorKind};
use chrono::{DateTime, Local, NaiveDate};
use core::fmt;
use core::hash::{Hash, Hasher};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
#[cfg(test)]
use rust_decimal_macros::dec;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

/// Comparison operators for values
#[derive(Copy, Clone, Debug)]
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

impl From<String> for Operator {
    fn from(s: String) -> Self {
        Operator::from(s.as_str())
    }
}
impl From<&str> for Operator {
    fn from(s: &str) -> Self {
        match s {
            "=" | "eq" => Operator::Equal,
            "!=" | "ne" => Operator::NotEqual,
            "<" | "lt" => Operator::LessThan,
            "<=" | "le" => Operator::LessThanEqual,
            ">" | "gt" => Operator::GreaterThan,
            ">=" | "ge" => Operator::GreaterThanEqual,
            "is" => Operator::Is,
            "<<" => Operator::Before,
            ">>" => Operator::After,
            _ => Operator::After, // TODO: add error value
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// A concrete type that implements atomic values.
/// These are the 19 predefined types in XSD Schema Part 2, plus five additional types.
/// Also included is a hint for serialisation for if the value should be escaped.
#[derive(Clone, Debug)]
pub struct Value {
    pub value: ValueData,
    pub output: OutputSpec,
}

impl Value {
    pub fn value_ref(&self) -> &ValueData {
        &self.value
    }
    pub fn output_ref(&self) -> &OutputSpec {
        &self.output
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

pub struct ValueBuilder {
    value: Option<ValueData>,
    output: OutputSpec,
}

impl ValueBuilder {
    pub fn new() -> Self {
        ValueBuilder {
            value: None,
            output: OutputSpec::Normal,
        }
    }
    pub fn value(mut self, v: ValueData) -> Self {
        self.value = Some(v);
        self
    }
    pub fn output(mut self, o: OutputSpec) -> Self {
        self.output = o;
        self
    }
    /// Produce the Value. This will panic if a value has not been specified.
    pub fn build(self) -> Value {
        Value {
            value: self.value.unwrap(),
            output: self.output,
        }
    }
}
/// Derive a new [ValueBuilder] from an existing [Value]. The value data in the old Value will be copied to the builder.
impl From<&Value> for ValueBuilder {
    fn from(v: &Value) -> Self {
        ValueBuilder {
            value: Some(v.value.clone()),
            output: OutputSpec::Normal,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ValueData {
    /// node or simple type
    AnyType,
    /// a not-yet-validated anyType
    Untyped,
    /// base type of all simple types. i.e. not a node
    AnySimpleType,
    /// a list of IDREF
    IDREFS(Vec<IDREF>),
    /// a list of NMTOKEN
    NMTOKENS(Vec<NMTOKEN>),
    /// a list of ENTITY
    ENTITIES(Vec<ENTITY>),
    /// Any numeric type
    Numeric,
    /// all atomic values (no lists or unions)
    AnyAtomicType,
    /// untyped atomic value
    UntypedAtomic,
    Duration,
    Time(DateTime<Local>), // Ignore the date part. Perhaps use Instant instead?
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
    Date(NaiveDate),
    // gYearMonth
    // gYear
    // gMonthDay
    // gMonth
    // gDay
    String(String),
    NormalizedString(NormalizedString),
    /// Like normalizedString, but without leading, trailing and consecutive whitespace
    Token,
    /// language identifiers [a-zA-Z]{1,8}(-[a-zA-Z0-9]{1,8})*
    Language,
    /// NameChar+
    NMTOKEN(NMTOKEN),
    /// NameStartChar NameChar+
    Name(Name),
    /// (Letter | '_') NCNameChar+ (i.e. a Name without the colon)
    NCName(NcName),
    /// Same format as NCName
    ID(ID),
    /// Same format as NCName
    IDREF(IDREF),
    /// Same format as NCName
    ENTITY(ENTITY),
    Boolean(bool),
    //base64binary,
    //hexBinary,
    //anyURI,
    /// Qualified Name
    QName(QName),
    NamespaceUri(NamespaceUri),
    //NOTATION
}

impl fmt::Display for ValueData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let result = match self {
            ValueData::String(s) => s.to_string(),
            ValueData::NormalizedString(s) => s.0.to_string(),
            ValueData::Decimal(d) => d.to_string(),
            ValueData::Float(f) => f.to_string(),
            ValueData::Double(d) => d.to_string(),
            ValueData::Integer(i) => i.to_string(),
            ValueData::Long(l) => l.to_string(),
            ValueData::Short(s) => s.to_string(),
            ValueData::Int(i) => i.to_string(),
            ValueData::Byte(b) => b.to_string(),
            ValueData::UnsignedLong(l) => l.to_string(),
            ValueData::UnsignedShort(s) => s.to_string(),
            ValueData::UnsignedInt(i) => i.to_string(),
            ValueData::UnsignedByte(b) => b.to_string(),
            ValueData::NonPositiveInteger(i) => i.0.to_string(),
            ValueData::NonNegativeInteger(i) => i.0.to_string(),
            ValueData::PositiveInteger(i) => i.0.to_string(),
            ValueData::NegativeInteger(i) => i.0.to_string(),
            ValueData::Time(t) => t.format("%H:%M:%S.%f").to_string(),
            ValueData::DateTime(dt) => dt.format("%Y-%m-%dT%H:%M:%S%z").to_string(),
            ValueData::Date(d) => d.format("%Y-%m-%d").to_string(),
            ValueData::QName(q) => q.to_string(),
            ValueData::NCName(n) => n.to_string(),
            ValueData::NamespaceUri(n) => n.to_string(),
            ValueData::ID(s) => s.to_string(),
            ValueData::IDREF(s) => s.to_string(),
            ValueData::IDREFS(s) => s.join(" ").to_string(),
            _ => "".to_string(),
        };
        f.write_str(result.as_str())
    }
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{:?}", self.value).hash(state)
    }
}
impl Eq for Value {}

impl Value {
    /// Create a Time, ignoring the date part
    pub fn new_time(t: DateTime<Local>) -> Self {
        Value {
            value: ValueData::Time(t),
            output: OutputSpec::Normal,
        }
    }
    /// Create a date
    pub fn new_date(d: NaiveDate) -> Self {
        Value {
            value: ValueData::Date(d),
            output: OutputSpec::Normal,
        }
    }
    /// Give the effective boolean value.
    pub fn to_bool(&self) -> bool {
        match &self.value {
            ValueData::Boolean(b) => *b,
            ValueData::String(t) => {
                //t.is_empty()
                !t.is_empty()
            }
            ValueData::NormalizedString(s) => !s.0.is_empty(),
            ValueData::Double(n) => *n != 0.0,
            ValueData::Integer(i) => *i != 0,
            ValueData::Int(i) => *i != 0,

            // These are non-empty strings by definition, so must be true
            ValueData::NCName(_) | ValueData::NamespaceUri(_) | ValueData::QName(_) => true,

            _ => false,
        }
    }

    /// Convert the value to an integer, if possible.
    pub fn to_int(&self) -> Result<i64, Error> {
        match &self.value {
            ValueData::Int(i) => Ok(*i as i64),
            ValueData::Integer(i) => Ok(*i),
            _ => match self.to_string().parse::<i64>() {
                Ok(i) => Ok(i),
                Err(e) => Result::Err(Error::new(
                    ErrorKind::Unknown,
                    format!("type conversion error: {}", e),
                )),
            },
        }
    }
    /// Convert the value to a double. If the value cannot be converted, returns Nan.
    pub fn to_double(&self) -> f64 {
        match &self.value {
            ValueData::String(s) => s.parse::<f64>().unwrap_or(f64::NAN),
            ValueData::Integer(i) => (*i) as f64,
            ValueData::Int(i) => (*i) as f64,
            ValueData::Double(d) => *d,
            _ => f64::NAN,
        }
    }
    pub fn value_type(&self) -> &'static str {
        match &self.value {
            ValueData::AnyType => "AnyType",
            ValueData::Untyped => "Untyped",
            ValueData::AnySimpleType => "AnySimpleType",
            ValueData::IDREFS(_) => "IDREFS",
            ValueData::NMTOKENS(_) => "NMTOKENS",
            ValueData::ENTITIES(_) => "ENTITIES",
            ValueData::Numeric => "Numeric",
            ValueData::AnyAtomicType => "AnyAtomicType",
            ValueData::UntypedAtomic => "UntypedAtomic",
            ValueData::Duration => "Duration",
            ValueData::Time(_) => "Time",
            ValueData::Decimal(_) => "Decimal",
            ValueData::Float(_) => "Float",
            ValueData::Double(_) => "Double",
            ValueData::Integer(_) => "Integer",
            ValueData::NonPositiveInteger(_) => "NonPositiveInteger",
            ValueData::NegativeInteger(_) => "NegativeInteger",
            ValueData::Long(_) => "Long",
            ValueData::Int(_) => "Int",
            ValueData::Short(_) => "Short",
            ValueData::Byte(_) => "Byte",
            ValueData::NonNegativeInteger(_) => "NonNegativeInteger",
            ValueData::UnsignedLong(_) => "UnsignedLong",
            ValueData::UnsignedInt(_) => "UnsignedInt",
            ValueData::UnsignedShort(_) => "UnsignedShort",
            ValueData::UnsignedByte(_) => "UnsignedByte",
            ValueData::PositiveInteger(_) => "PositiveInteger",
            ValueData::DateTime(_) => "DateTime",
            ValueData::DateTimeStamp => "DateTimeStamp",
            ValueData::Date(_) => "Date",
            ValueData::String(_) => "String",
            ValueData::NormalizedString(_) => "NormalizedString",
            ValueData::Token => "Token",
            ValueData::Language => "Language",
            ValueData::NMTOKEN(_) => "NMTOKEN",
            ValueData::Name(_) => "Name",
            ValueData::NamespaceUri(_) => "NamespaceUri",
            ValueData::NCName(_) => "NCName",
            ValueData::ID(_) => "ID",
            ValueData::IDREF(_) => "IDREF",
            ValueData::ENTITY(_) => "ENTITY",
            ValueData::Boolean(_) => "boolean",
            ValueData::QName(_) => "QName",
        }
    }
    pub fn compare(&self, other: &Value, op: Operator) -> Result<bool, Error> {
        match &self.value {
            ValueData::Boolean(b) => {
                let c = other.to_bool();
                match op {
                    Operator::Equal => Ok(*b == c),
                    Operator::NotEqual => Ok(*b != c),
                    Operator::LessThan => Ok(!(*b) & c),
                    Operator::LessThanEqual => Ok(*b <= c),
                    Operator::GreaterThan => Ok(*b & !c),
                    Operator::GreaterThanEqual => Ok(*b >= c),
                    Operator::Is | Operator::Before | Operator::After => {
                        Err(Error::new(ErrorKind::TypeError, String::from("type error")))
                    }
                }
            }
            ValueData::Integer(i) => {
                let c = other.to_int()?;
                match op {
                    Operator::Equal => Ok(*i == c),
                    Operator::NotEqual => Ok(*i != c),
                    Operator::LessThan => Ok(*i < c),
                    Operator::LessThanEqual => Ok(*i <= c),
                    Operator::GreaterThan => Ok(*i > c),
                    Operator::GreaterThanEqual => Ok(*i >= c),
                    Operator::Is | Operator::Before | Operator::After => {
                        Err(Error::new(ErrorKind::TypeError, String::from("type error")))
                    }
                }
            }
            ValueData::Int(i) => {
                let c = other.to_int()? as i32;
                match op {
                    Operator::Equal => Ok(*i == c),
                    Operator::NotEqual => Ok(*i != c),
                    Operator::LessThan => Ok(*i < c),
                    Operator::LessThanEqual => Ok(*i <= c),
                    Operator::GreaterThan => Ok(*i > c),
                    Operator::GreaterThanEqual => Ok(*i >= c),
                    Operator::Is | Operator::Before | Operator::After => {
                        Err(Error::new(ErrorKind::TypeError, String::from("type error")))
                    }
                }
            }
            ValueData::Double(i) => {
                let c = other.to_double();
                match op {
                    Operator::Equal => Ok(*i == c),
                    Operator::NotEqual => Ok(*i != c),
                    Operator::LessThan => Ok(*i < c),
                    Operator::LessThanEqual => Ok(*i <= c),
                    Operator::GreaterThan => Ok(*i > c),
                    Operator::GreaterThanEqual => Ok(*i >= c),
                    Operator::Is | Operator::Before | Operator::After => {
                        Err(Error::new(ErrorKind::TypeError, String::from("type error")))
                    }
                }
            }
            ValueData::String(i) => {
                let c = other.to_string();
                match op {
                    Operator::Equal => Ok(*i == c),
                    Operator::NotEqual => Ok(*i != c),
                    Operator::LessThan => Ok(*i < c),
                    Operator::LessThanEqual => Ok(*i <= c),
                    Operator::GreaterThan => Ok(*i > c),
                    Operator::GreaterThanEqual => Ok(*i >= c),
                    Operator::Is | Operator::Before | Operator::After => {
                        Err(Error::new(ErrorKind::TypeError, String::from("type error")))
                    }
                }
            }
            ValueData::QName(q) => match (op, &other.value) {
                (Operator::Equal, ValueData::QName(r)) => Ok(*q == *r),
                (Operator::NotEqual, ValueData::QName(r)) => Ok(*q != *r),
                _ => Err(Error::new(ErrorKind::TypeError, String::from("type error"))),
            },
            ValueData::NCName(q) => match (op, &other.value) {
                (Operator::Equal, ValueData::NCName(r)) => Ok(*q == *r),
                (Operator::NotEqual, ValueData::NCName(r)) => Ok(*q != *r),
                _ => Err(Error::new(ErrorKind::TypeError, String::from("type error"))),
            },
            ValueData::NamespaceUri(q) => match (op, &other.value) {
                (Operator::Equal, ValueData::NamespaceUri(r)) => Ok(*q == *r),
                (Operator::NotEqual, ValueData::NamespaceUri(r)) => Ok(*q != *r),
                _ => Err(Error::new(ErrorKind::TypeError, String::from("type error"))),
            },
            _ => Result::Err(Error::new(
                ErrorKind::Unknown,
                format!(
                    "comparing type \"{}\" is not yet implemented",
                    self.value_type()
                ),
            )),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match &self.value {
            ValueData::String(s) => s.eq(&other.to_string()),
            ValueData::Boolean(b) => match other.value {
                ValueData::Boolean(c) => *b == c,
                _ => false, // type error?
            },
            ValueData::Decimal(d) => match other.value {
                ValueData::Decimal(e) => *d == e,
                _ => false, // type error?
            },
            ValueData::Integer(i) => match other.value {
                ValueData::Integer(j) => *i == j,
                _ => false, // type error? coerce to integer?
            },
            ValueData::Double(d) => match other.value {
                ValueData::Double(e) => *d == e,
                _ => false, // type error? coerce to integer?
            },
            ValueData::NCName(n) => match &other.value {
                ValueData::NCName(o) => *n == *o,
                _ => false,
            },
            ValueData::QName(n) => match &other.value {
                ValueData::QName(o) => *n == *o,
                _ => false,
            },
            ValueData::NamespaceUri(n) => match &other.value {
                ValueData::NamespaceUri(o) => *n == *o,
                _ => false,
            },
            _ => false, // not yet implemented
        }
    }
}
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        match &self.value {
            ValueData::String(s) => {
                let o: String = other.to_string();
                s.partial_cmp(&o)
            }
            ValueData::Boolean(_) => None,
            ValueData::Decimal(d) => match other.value {
                ValueData::Decimal(e) => d.partial_cmp(&e),
                _ => None, // type error?
            },
            ValueData::Integer(d) => match other.value {
                ValueData::Integer(e) => d.partial_cmp(&e),
                _ => None, // type error?
            },
            ValueData::Double(d) => match other.value {
                ValueData::Double(e) => d.partial_cmp(&e),
                _ => None, // type error?
            },
            _ => None,
        }
    }
}

//This is ONLY being used for namespace node sorting for the purposes of serializing
//Feel free to change it.
//We can change between versions, so long as each execution on that version is consistent.
impl Ord for Value {
    fn cmp(&self, other: &Value) -> Ordering {
        match &self.value {
            ValueData::String(s) => {
                let o: String = other.to_string();
                s.cmp(&o)
            }
            ValueData::Boolean(_) => Ordering::Equal,
            ValueData::Decimal(d) => match other.value {
                ValueData::Decimal(e) => d.cmp(&e),
                _ => Ordering::Equal, // type error?
            },
            ValueData::Integer(d) => match other.value {
                ValueData::Integer(e) => d.cmp(&e),
                _ => Ordering::Equal, // type error?
            },
            ValueData::Double(d) => match other.value {
                ValueData::Double(e) => d.partial_cmp(&e).unwrap_or(Ordering::Equal),
                _ => Ordering::Equal, // type error?
            },
            _ => Ordering::Equal,
        }
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value {
            value: ValueData::String(s),
            output: OutputSpec::Normal,
        }
    }
}
impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value {
            value: ValueData::String(String::from(s)),
            output: OutputSpec::Normal,
        }
    }
}
impl From<Decimal> for Value {
    fn from(d: Decimal) -> Self {
        Value {
            value: ValueData::Decimal(d),
            output: OutputSpec::Normal,
        }
    }
}
impl From<PositiveInteger> for Value {
    fn from(p: PositiveInteger) -> Self {
        Value {
            value: ValueData::PositiveInteger(p),
            output: OutputSpec::Normal,
        }
    }
}
impl From<NonPositiveInteger> for Value {
    fn from(n: NonPositiveInteger) -> Self {
        Value {
            value: ValueData::NonPositiveInteger(n),
            output: OutputSpec::Normal,
        }
    }
}
impl From<NegativeInteger> for Value {
    fn from(n: NegativeInteger) -> Self {
        Value {
            value: ValueData::NegativeInteger(n),
            output: OutputSpec::Normal,
        }
    }
}
impl From<NonNegativeInteger> for Value {
    fn from(n: NonNegativeInteger) -> Self {
        Value {
            value: ValueData::NonNegativeInteger(n),
            output: OutputSpec::Normal,
        }
    }
}
impl From<f32> for Value {
    fn from(f: f32) -> Self {
        Value {
            value: ValueData::Float(f),
            output: OutputSpec::Normal,
        }
    }
}
impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value {
            value: ValueData::Double(f),
            output: OutputSpec::Normal,
        }
    }
}
impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value {
            value: ValueData::Integer(i),
            output: OutputSpec::Normal,
        }
    }
}
impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value {
            value: ValueData::Int(i),
            output: OutputSpec::Normal,
        }
    }
}
impl From<i16> for Value {
    fn from(i: i16) -> Self {
        Value {
            value: ValueData::Short(i),
            output: OutputSpec::Normal,
        }
    }
}
impl From<i8> for Value {
    fn from(i: i8) -> Self {
        Value {
            value: ValueData::Byte(i),
            output: OutputSpec::Normal,
        }
    }
}
impl From<u64> for Value {
    fn from(i: u64) -> Self {
        Value {
            value: ValueData::UnsignedLong(i),
            output: OutputSpec::Normal,
        }
    }
}
impl From<u32> for Value {
    fn from(i: u32) -> Self {
        Value {
            value: ValueData::UnsignedInt(i),
            output: OutputSpec::Normal,
        }
    }
}
impl From<u16> for Value {
    fn from(i: u16) -> Self {
        Value {
            value: ValueData::UnsignedShort(i),
            output: OutputSpec::Normal,
        }
    }
}
impl From<u8> for Value {
    fn from(i: u8) -> Self {
        Value {
            value: ValueData::UnsignedByte(i),
            output: OutputSpec::Normal,
        }
    }
}
impl From<usize> for Value {
    fn from(u: usize) -> Self {
        Value {
            value: ValueData::UnsignedLong(u.to_u64().unwrap()),
            output: OutputSpec::Normal,
        }
    }
}
impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value {
            value: ValueData::Boolean(b),
            output: OutputSpec::Normal,
        }
    }
}
impl From<NormalizedString> for Value {
    fn from(n: NormalizedString) -> Self {
        Value {
            value: ValueData::NormalizedString(n),
            output: OutputSpec::Normal,
        }
    }
}
impl From<ID> for Value {
    fn from(n: ID) -> Self {
        Value {
            value: ValueData::ID(n),
            output: OutputSpec::Normal,
        }
    }
}
impl From<IDREF> for Value {
    fn from(n: IDREF) -> Self {
        Value {
            value: ValueData::IDREF(n),
            output: OutputSpec::Normal,
        }
    }
}
impl From<Vec<IDREF>> for Value {
    fn from(v: Vec<IDREF>) -> Self {
        Value {
            value: ValueData::IDREFS(v),
            output: OutputSpec::Normal,
        }
    }
}
impl From<QName> for Value {
    fn from(q: QName) -> Self {
        Value {
            value: ValueData::QName(q),
            output: OutputSpec::Normal,
        }
    }
}
impl From<NcName> for Value {
    fn from(q: NcName) -> Self {
        Value {
            value: ValueData::NCName(q),
            output: OutputSpec::Normal,
        }
    }
}
impl From<NamespaceUri> for Value {
    fn from(q: NamespaceUri) -> Self {
        Value {
            value: ValueData::NamespaceUri(q),
            output: OutputSpec::Normal,
        }
    }
}

#[derive(Clone, Debug, Hash)]
pub struct NonPositiveInteger(i64);
impl TryFrom<i64> for NonPositiveInteger {
    type Error = Error;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        if v > 0 {
            Err(Error::new(
                ErrorKind::TypeError,
                String::from("NonPositiveInteger must be less than zero"),
            ))
        } else {
            Ok(NonPositiveInteger(v))
        }
    }
}
impl fmt::Display for NonPositiveInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

#[derive(Clone, Debug, Hash)]
pub struct PositiveInteger(i64);
impl TryFrom<i64> for PositiveInteger {
    type Error = Error;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        if v <= 0 {
            Err(Error::new(
                ErrorKind::TypeError,
                String::from("PositiveInteger must be greater than zero"),
            ))
        } else {
            Ok(PositiveInteger(v))
        }
    }
}
impl fmt::Display for PositiveInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

#[derive(Clone, Debug, Hash)]
pub struct NonNegativeInteger(i64);
impl TryFrom<i64> for NonNegativeInteger {
    type Error = Error;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        if v < 0 {
            Err(Error::new(
                ErrorKind::TypeError,
                String::from("NonNegativeInteger must be zero or greater"),
            ))
        } else {
            Ok(NonNegativeInteger(v))
        }
    }
}
impl fmt::Display for NonNegativeInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

#[derive(Clone, Debug, Hash)]
pub struct NegativeInteger(i64);
impl TryFrom<i64> for NegativeInteger {
    type Error = Error;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        if v >= 0 {
            Err(Error::new(
                ErrorKind::TypeError,
                String::from("NegativeInteger must be less than zero"),
            ))
        } else {
            Ok(NegativeInteger(v))
        }
    }
}
impl fmt::Display for NegativeInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

#[derive(Clone, Debug, Hash)]
pub struct NormalizedString(String);
impl TryFrom<&str> for NormalizedString {
    type Error = Error;
    fn try_from(v: &str) -> Result<Self, Self::Error> {
        let n: &[_] = &['\n', '\r', '\t'];
        if v.find(n).is_none() {
            Ok(NormalizedString(v.to_string()))
        } else {
            Err(Error::new(
                ErrorKind::TypeError,
                String::from("value is not a normalized string"),
            ))
        }
    }
}
impl fmt::Display for NormalizedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

/// An XML Name (XML production 5):
/// Name ::= NameStartChar NameChar*
#[derive(Clone, Debug, Hash)]
pub struct Name(String);
impl TryFrom<&str> for Name {
    type Error = Error;
    fn try_from(v: &str) -> Result<Self, Self::Error> {
        // TODO: do a proper check
        let n: &[_] = &['\n', '\r', '\t'];
        if v.find(n).is_none() {
            Ok(Name(v.to_string()))
        } else {
            Err(Error::new(
                ErrorKind::TypeError,
                String::from("value is not a Name"),
            ))
        }
    }
}
impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

/// An XML Entity (XML production 56) must match the Name production (5):
/// Name ::= NameStartChar NameChar*
#[derive(Clone, Debug, Hash)]
pub struct ENTITY(String);
impl TryFrom<&str> for ENTITY {
    type Error = Error;
    fn try_from(v: &str) -> Result<Self, Self::Error> {
        // TODO: do a proper check
        let n: &[_] = &['\n', '\r', '\t'];
        if v.find(n).is_none() {
            Ok(ENTITY(v.to_string()))
        } else {
            Err(Error::new(
                ErrorKind::TypeError,
                String::from("value is not an ENTITY"),
            ))
        }
    }
}
impl fmt::Display for ENTITY {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

/// An XML NMTOKEN (XML production 56) must match the Nmtoken production (7):
/// Nmtoken ::= NameChar+
#[derive(Clone, Debug, Hash)]
pub struct NMTOKEN(String);
impl TryFrom<&str> for NMTOKEN {
    type Error = Error;
    fn try_from(v: &str) -> Result<Self, Self::Error> {
        // TODO: do a proper check
        if !v.is_empty() {
            Ok(NMTOKEN(v.to_string()))
        } else {
            Err(Error::new(
                ErrorKind::TypeError,
                String::from("value is not a NMTOKEN"),
            ))
        }
    }
}
impl fmt::Display for NMTOKEN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

/// An XML ID (XML production 56) must match the Name production (5):
/// Name ::= NameStartChar NameChar*
/// An ID must be unique within a document. It is the responsibility of the document to check for compliance.
#[derive(Clone, Debug, Hash)]
pub struct ID(String);
impl TryFrom<&str> for ID {
    type Error = Error;
    fn try_from(v: &str) -> Result<Self, Self::Error> {
        // TODO: An XML ID must be a Name
        let n: &[_] = &['\n', '\r', '\t'];
        if v.find(n).is_none() {
            Ok(ID(v.to_string()))
        } else {
            Err(Error::new(
                ErrorKind::TypeError,
                String::from("value is not an ID"),
            ))
        }
    }
}
impl TryFrom<String> for ID {
    type Error = Error;
    fn try_from(v: String) -> Result<Self, Self::Error> {
        // TODO: An XML ID must be a Name
        let n: &[_] = &['\n', '\r', '\t'];
        if v.find(n).is_none() {
            Ok(ID(v))
        } else {
            Err(Error::new(
                ErrorKind::TypeError,
                String::from("value is not an ID"),
            ))
        }
    }
}
impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

/// An XML IDREF (XML production 56) must match the Name production (5):
/// Name ::= NameStartChar NameChar*
#[derive(Clone, Debug, Hash)]
pub struct IDREF(String);
impl TryFrom<&str> for IDREF {
    type Error = Error;
    fn try_from(v: &str) -> Result<Self, Self::Error> {
        // TODO: An XML IDREF must be a Name
        let n: &[_] = &['\n', '\r', '\t'];
        if v.find(n).is_none() {
            Ok(IDREF(v.to_string()))
        } else {
            Err(Error::new(
                ErrorKind::TypeError,
                String::from("value is not an IDREF"),
            ))
        }
    }
}
impl Display for IDREF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0.to_string())
    }
}
impl TryFrom<String> for IDREF {
    type Error = Error;
    fn try_from(v: String) -> Result<Self, Self::Error> {
        // TODO: An XML IDREF must be a Name
        let n: &[_] = &['\n', '\r', '\t'];
        if v.find(n).is_none() {
            Ok(IDREF(v))
        } else {
            Err(Error::new(
                ErrorKind::TypeError,
                String::from("value is not an IDREF"),
            ))
        }
    }
}
impl fmt::Display for IDREF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

/// An NCName for XML Namespaces.
#[derive(Clone, Debug, Hash)]
pub struct NCName(String);
impl TryFrom<&str> for NCName {
    type Error = Error;
    fn try_from(v: &str) -> Result<Self, Self::Error> {
        // TODO: do a proper check
        let n: &[_] = &['\n', '\r', '\t', ':'];
        if v.find(n).is_none() {
            Ok(NCName(v.to_string()))
        } else {
            Err(Error::new(
                ErrorKind::TypeError,
                String::from("value is not a NCName"),
            ))
        }
    }
}
impl fmt::Display for NCName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_string() {
        assert_eq!(Value::from(String::from("foobar")).to_string(), "foobar");
    }
    #[test]
    fn from_str() {
        assert_eq!(Value::from("foobar").to_string(), "foobar");
    }
    #[test]
    fn from_decimal() {
        assert_eq!(Value::from(dec!(001.23)).to_string(), "1.23");
    }

    #[test]
    fn normalizedstring_valid_empty() {
        assert_eq!(
            NormalizedString::try_from("")
                .expect("invalid NormalizedString")
                .0,
            ""
        );
    }
    #[test]
    fn normalizedstring_valid() {
        assert_eq!(
            NormalizedString::try_from("notinvalid")
                .expect("invalid NormalizedString")
                .0,
            "notinvalid"
        );
    }
    #[test]
    fn normalizedstring_valid_spaces() {
        assert_eq!(
            NormalizedString::try_from("not an invalid string")
                .expect("invalid NormalizedString")
                .0,
            "not an invalid string"
        );
    }
    #[test]
    fn normalizedstring_invalid_tab() {
        let r = NormalizedString::try_from("contains tab	character");
        assert!(match r {
            Ok(_) => panic!("string contains tab character"),
            Err(_) => true,
        })
    }
    #[test]
    fn normalizedstring_invalid_newline() {
        let r = NormalizedString::try_from("contains newline\ncharacter");
        assert!(match r {
            Ok(_) => panic!("string contains newline character"),
            Err(_) => true,
        })
    }
    #[test]
    fn normalizedstring_invalid_cr() {
        let r = NormalizedString::try_from("contains carriage return\rcharacter");
        assert!(match r {
            Ok(_) => panic!("string contains cr character"),
            Err(_) => true,
        })
    }
    #[test]
    fn normalizedstring_invalid_all() {
        let r = NormalizedString::try_from("contains	all\rforbidden\ncharacters");
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
        assert_eq!(
            NonPositiveInteger::try_from(-10)
                .expect("invalid NonPositiveInteger")
                .0,
            -10
        );
    }
    #[test]
    fn nonpositiveinteger_valid_zero() {
        assert_eq!(
            NonPositiveInteger::try_from(0)
                .expect("invalid NonPositiveInteger")
                .0,
            0
        );
    }
    #[test]
    fn nonpositiveinteger_invalid() {
        let r = NonPositiveInteger::try_from(10);
        assert!(match r {
            Ok(_) => panic!("10 is not a nonPositiveInteger"),
            Err(_) => true,
        })
    }

    #[test]
    fn positiveinteger_valid() {
        assert_eq!(
            PositiveInteger::try_from(10)
                .expect("invalid PositiveInteger")
                .0,
            10
        );
    }
    #[test]
    fn positiveinteger_invalid_zero() {
        let r = PositiveInteger::try_from(0);
        assert!(match r {
            Ok(_) => panic!("0 is not a PositiveInteger"),
            Err(_) => true,
        })
    }
    #[test]
    fn positiveinteger_invalid() {
        let r = PositiveInteger::try_from(-10);
        assert!(match r {
            Ok(_) => panic!("-10 is not a PositiveInteger"),
            Err(_) => true,
        })
    }

    #[test]
    fn nonnegativeinteger_valid() {
        assert_eq!(
            NonNegativeInteger::try_from(10)
                .expect("invalid NonNegativeInteger")
                .0,
            10
        );
    }
    #[test]
    fn nonnegativeinteger_valid_zero() {
        assert_eq!(
            NonNegativeInteger::try_from(0)
                .expect("invalid NonNegativeInteger")
                .0,
            0
        );
    }
    #[test]
    fn nonnegativeinteger_invalid() {
        let r = NonNegativeInteger::try_from(-10);
        assert!(match r {
            Ok(_) => panic!("-10 is not a NonNegativeInteger"),
            Err(_) => true,
        })
    }

    #[test]
    fn negativeinteger_valid() {
        assert_eq!(
            NegativeInteger::try_from(-10)
                .expect("invalid NegativeInteger")
                .0,
            -10
        );
    }
    #[test]
    fn negativeinteger_invalid_zero() {
        let r = NegativeInteger::try_from(0);
        assert!(match r {
            Ok(_) => panic!("0 is not a NegativeInteger"),
            Err(_) => true,
        })
    }
    #[test]
    fn negativeinteger_invalid() {
        let r = NegativeInteger::try_from(10);
        assert!(match r {
            Ok(_) => panic!("10 is not a NegativeInteger"),
            Err(_) => true,
        })
    }

    // String Values
    #[test]
    fn string_strvalue() {
        assert_eq!(Value::from("foobar").to_string(), "foobar")
    }
    #[test]
    fn string_stringvalue() {
        assert_eq!(Value::from("foobar".to_string()).to_string(), "foobar")
    }
    #[test]
    fn decimal_stringvalue() {
        assert_eq!(Value::from(dec!(001.23)).to_string(), "1.23")
    }
    #[test]
    fn float_stringvalue() {
        assert_eq!(Value::from(001.2300_f32).to_string(), "1.23")
    }
    #[test]
    fn nonpositiveinteger_stringvalue() {
        let npi = NonPositiveInteger::try_from(-00123).expect("invalid nonPositiveInteger");
        let i = Value::from(npi);
        assert_eq!(i.to_string(), "-123")
    }
    #[test]
    fn nonnegativeinteger_stringvalue() {
        let nni = NonNegativeInteger::try_from(00123).expect("invalid nonNegativeInteger");
        let i = Value::from(nni);
        assert_eq!(i.to_string(), "123")
    }
    #[test]
    fn normalizedstring_stringvalue() {
        let ns = NormalizedString::try_from("foobar").expect("invalid normalizedString");
        let i = Value::from(ns);
        assert_eq!(i.to_string(), "foobar")
    }

    // value to_bool

    #[test]
    fn value_to_bool_string() {
        assert!(Value::from("2").to_bool())
    }

    // value to_int

    #[test]
    fn value_to_int_string() {
        assert_eq!(
            Value::from("2")
                .to_int()
                .expect("cannot convert to integer"),
            2
        )
    }

    // value to_double

    #[test]
    fn value_to_double_string() {
        assert_eq!(Value::from("3.0").to_double(), 3.0)
    }

    // value compare

    #[test]
    fn value_compare_eq() {
        assert!(
            Value::from("3")
                .compare(&Value::Double(3.0), Operator::Equal)
                .expect("unable to compare")
        )
    }

    #[test]
    fn value_compare_ne() {
        assert!(
            !Value::from("3")
                .compare(&Value::Double(3.0), Operator::NotEqual)
                .expect("unable to compare")
        )
    }

    //#[test]
    //fn value_atomize() {
    //let i = Value::Int(123);
    //assert_eq!(i.atomize().stringvalue(), "123")
    //}

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

    // Building a value
    #[test]
    fn build_1() {
        let v = ValueBuilder::new()
            .value(ValueData::String(String::from("test value")))
            .build();
        assert_eq!(v.to_string(), "test value");
        assert_eq!(v.output_ref(), &OutputSpec::Normal)
    }
    #[test]
    fn build_2() {
        let v = ValueBuilder::new()
            .value(ValueData::String(String::from("test value")))
            .output(OutputSpec::Escaped)
            .build();
        assert_eq!(v.to_string(), "test value");
        assert_eq!(v.output_ref(), &OutputSpec::Escaped)
    }
}
