use crate::Error;
use crate::qname::QualifiedName;
use crate::trees::smite::RNode;

pub(crate) type DataType = (QualifiedName, String);
type Attributenode = (QualifiedName, String);
pub(crate) type Param = (String, String);
type Context = (QualifiedName, Vec<(String, QualifiedName)>);

#[derive(Clone, Debug)]
pub(crate) enum NameClass {
    AnyName,
    AnyNameExcept(Box<NameClass>),
    Name(String, String),
    NSName(String),
    NSNameExcept(String, Box<NameClass>),
    NameClassChoice(Box<NameClass>, Box<NameClass>)
}

#[derive(Clone, Debug)]
pub(crate) enum Pattern {
    Empty,
    NotAllowed,
    Text,
    Choice(Box<Pattern>, Box<Pattern>),
    Interleave(Box<Pattern>, Box<Pattern>),
    Group(Box<Pattern>, Box<Pattern>),
    OneOrMore(Box<Pattern>),
    List(Box<Pattern>),
    Data(DataType, Vec<Param>),
    DataExcept(DataType, Vec<Param>, Box<Pattern>),
    Value(DataType, String, Context),
    Attribute(NameClass, Box<Pattern>),
    Element(NameClass, Box<Pattern>),
    After(Box<Pattern>, Box<Pattern>)
}

impl Pattern{
    pub(crate) fn new(schema: &RNode) -> Pattern {
        Pattern::Empty
    }

    pub(crate) fn is_nullable(&self) -> bool {
        match self {
            Pattern::Group(p1, p2) => {p1.is_nullable() && p2.is_nullable()},
            Pattern::Interleave(p1, p2) => {p1.is_nullable() && p2.is_nullable()},
            Pattern::Choice(p1, p2) => {p1.is_nullable() || p2.is_nullable()},
            Pattern::OneOrMore(p) => { p.is_nullable()},
            Pattern::Empty => true,
            Pattern::Text => true,
            Pattern::Element(_, _) => false,
            Pattern::Attribute(_, _) => false,
            Pattern::List(_) => false,
            Pattern::Value(_, _, _) => false,
            Pattern::Data(_, _) => false,
            Pattern::DataExcept(_, _, _) => false,
            Pattern::NotAllowed => false,
            Pattern::After(_, _) => false
        }
    }
}

