/*! Defines common features of XML documents.
 */

use crate::qname::QualifiedName;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, PartialEq)]
pub struct XMLDecl {
    pub(crate) version: String,
    pub(crate) encoding: Option<String>,
    pub(crate) standalone: Option<String>,
}

impl XMLDecl {
    pub fn new(version: String, encoding: Option<String>, standalone: Option<String>) -> Self {
        XMLDecl {
            version,
            encoding,
            standalone,
        }
    }
    pub fn version(&self) -> String {
        self.version.clone()
    }
    pub fn set_encoding(&mut self, e: String) {
        self.encoding = Some(e)
    }
    pub fn encoding(&self) -> String {
        self.encoding.as_ref().map_or(String::new(), |e| e.clone())
    }
    pub fn set_standalone(&mut self, s: String) {
        self.standalone = Some(s)
    }
    pub fn standalone(&self) -> String {
        self.standalone
            .as_ref()
            .map_or(String::new(), |e| e.clone())
    }
}

impl fmt::Display for XMLDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::from("<?xml version=\"");
        result.push_str(self.version.as_str());
        result.push('"');
        if let Some(e) = self.encoding.as_ref() {
            result.push_str(" encoding=\"");
            result.push_str(e.as_str());
            result.push('"');
        };
        if let Some(e) = self.standalone.as_ref() {
            result.push_str(" standalone=\"");
            result.push_str(e.as_str());
            result.push('"');
        };
        f.write_str(result.as_str())
    }
}

pub struct XMLDeclBuilder(XMLDecl);

impl Default for XMLDeclBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl XMLDeclBuilder {
    pub fn new() -> Self {
        XMLDeclBuilder(XMLDecl {
            version: String::new(),
            encoding: None,
            standalone: None,
        })
    }
    pub fn version(mut self, v: String) -> Self {
        self.0.version = v;
        self
    }
    pub fn encoding(mut self, v: String) -> Self {
        self.0.encoding = Some(v);
        self
    }
    pub fn standalone(mut self, v: String) -> Self {
        self.0.standalone = Some(v);
        self
    }
    pub fn build(self) -> XMLDecl {
        self.0
    }
}

/// DTD declarations.
/// Only general entities are supported, so far.
/// TODO: element, attribute declarations

#[derive(Clone, PartialEq)]
pub struct DTD {
    pub(crate) elements: HashMap<String, DTDDecl>,
    pub(crate) attlists: HashMap<QualifiedName, HashMap<QualifiedName, (AttType, DefaultDecl, bool)>>,// Boolean for is_editable;
    pub(crate) notations: HashMap<String, DTDDecl>,
    pub(crate) generalentities: HashMap<String, (String, bool)>, // Boolean for is_editable;
    pub(crate) paramentities: HashMap<String, (String, bool)>,// Boolean for is_editable;
    publicid: Option<String>,
    systemid: Option<String>,
    name: Option<String>,
}

impl DTD {
    pub fn new() -> DTD {
        let default_entities = vec![
            ("amp".to_string(), ("&".to_string(), false)),
            ("gt".to_string(), (">".to_string(), false)),
            ("lt".to_string(), ("<".to_string(), false)),
            ("apos".to_string(), ("'".to_string(), false)),
            ("quot".to_string(), ("\"".to_string(), false)),
        ];
        DTD {
            elements: Default::default(),
            attlists: Default::default(),
            notations: Default::default(),
            generalentities: default_entities.into_iter().collect(),
            paramentities: HashMap::new(),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DTDDecl {
    Element(QualifiedName, String),
    Notation(QualifiedName, String),
    GeneralEntity(QualifiedName, String),
    ParamEntity(QualifiedName, String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum AttType {
    CDATA,
    ID,
    IDREF,
    IDREFS,
    ENTITY,
    ENTITIES,
    NMTOKEN,
    NMTOKENS,
    NOTATION(Vec<String>),
    ENUMERATION(Vec<String>)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum DefaultDecl {
    Required,
    Implied,
    FIXED(String),
    Default(String)
}
