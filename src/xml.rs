use crate::parser;
use crate::qname::QualifiedName;
use crate::xdmerror::Error;
use crate::Value;
use std::collections::HashMap;
use std::convert::TryFrom;

// This structure allows multiple root elements.
// An XML document will only be well-formed if there is exactly one element.
// However, external general entities may have more than one element.
#[derive(PartialEq)]
pub struct XMLDocument {
    pub prologue: Vec<XMLNode>,
    pub content: Vec<XMLNode>,
    pub epilogue: Vec<XMLNode>,
    pub xmldecl: Option<XMLdecl>,
}
impl TryFrom<&str> for XMLDocument {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        XMLDocument::try_from(s.to_string())
    }
}
impl TryFrom<String> for XMLDocument {
    type Error = Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        parser::xml::parse(s)
    }
}

#[derive(Clone, PartialEq)]
pub enum XMLNode {
    Element(QualifiedName, Vec<XMLNode>, Vec<XMLNode>), // Element name, attributes, content
    Attribute(QualifiedName, Value),
    Text(Value),
    PI(String, Value),
    Comment(Value),           // Comment value is a string
    DTD(DTDDecl),             // These only occur in the prologue
    Reference(QualifiedName), // General entity reference. These need to be resolved before presentation to the application
}

/// DTD declarations.
/// Only general entities are supported, so far.
/// TODO: element, attribute declarations
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DTDDecl {
    Element(QualifiedName, String),
    Attlist(QualifiedName, String),
    Notation(QualifiedName, String),
    GeneralEntity(QualifiedName, String),
    ParamEntity(QualifiedName, String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DTD {
    pub(crate) elements: HashMap<String, DTDDecl>,
    pub(crate) attlists: HashMap<String, DTDDecl>,
    pub(crate) notations: HashMap<String, DTDDecl>,
    pub(crate) generalentities: HashMap<String, DTDDecl>,
    pub(crate) paramentities: HashMap<String, DTDDecl>,
    publicid: Option<String>,
    systemid: Option<String>,
    name: Option<String>,
}

impl DTD {
    pub fn new() -> DTD {
        DTD {
            elements: Default::default(),
            attlists: Default::default(),
            notations: Default::default(),
            generalentities: Default::default(),
            paramentities: Default::default(),
            publicid: None,
            systemid: None,
            name: None,
        }
    }
}

impl Default for DTD {
    fn default() -> Self {
        Self::new()
    }
}
