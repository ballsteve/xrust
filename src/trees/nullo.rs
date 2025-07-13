use crate::item::{Node, NodeType};
use crate::output::OutputDefinition;
use crate::qname::{Interner, LocalInternment, QualifiedName};
use crate::validators::{Schema, ValidationError};
use crate::value::Value;
use crate::xdmerror::{Error, ErrorKind};
use crate::xmldecl::{XMLDecl, XMLDeclBuilder, DTD};
/// A null tree implementation
///
/// This tree implementation implements nothing.
/// The parser combinator is generic in [Node].
/// Occasionally, a module using the parser, but not needing a [Node],
/// nevertheless requires a concrete type that has the [Node] trait.
use std::cmp::Ordering;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub struct Nullo();

impl Node for Nullo {
    type NodeIterator = Box<dyn Iterator<Item = Nullo>>;

    fn new_document() -> Self {
        Nullo()
    }
    fn node_type(&self) -> NodeType {
        NodeType::Unknown
    }
    fn name<'i, I: Interner>(&self) -> Option<QualifiedName<'i, I>> {
        None
    }
    fn value(&self) -> Rc<Value> {
        Rc::new(Value::from(""))
    }
    fn get_id(&self) -> String {
        String::from("")
    }
    fn to_string(&self) -> String {
        String::new()
    }
    fn to_xml(&self) -> String {
        String::new()
    }
    fn to_xml_with_options(&self, _: &OutputDefinition) -> String {
        String::new()
    }
    fn to_json(&self) -> String {
        String::new()
    }
    fn is_same(&self, _: &Self) -> bool {
        false
    }
    fn document_order(&self) -> Vec<usize> {
        vec![]
    }
    fn cmp_document_order(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
    fn is_element(&self) -> bool {
        false
    }
    fn child_iter(&self) -> Self::NodeIterator {
        Box::new(NulloIter::new())
    }
    fn namespace_iter(&self) -> Self::NodeIterator {
        Box::new(NulloIter::new())
    }
    fn ancestor_iter(&self) -> Self::NodeIterator {
        Box::new(NulloIter::new())
    }
    fn descend_iter(&self) -> Self::NodeIterator {
        Box::new(NulloIter::new())
    }
    fn next_iter(&self) -> Self::NodeIterator {
        Box::new(NulloIter::new())
    }
    fn prev_iter(&self) -> Self::NodeIterator {
        Box::new(NulloIter::new())
    }
    fn attribute_iter(&self) -> Self::NodeIterator {
        Box::new(NulloIter::new())
    }
    fn get_attribute<'i, I: Interner>(&self, _: &QualifiedName<'i, I>) -> Rc<Value> {
        Rc::new(Value::from(""))
    }
    fn get_attribute_node<'i, I: Interner>(&self, _: &QualifiedName<'i, I>) -> Option<Self> {
        None
    }
    fn owner_document(&self) -> Self {
        self.clone()
    }
    fn get_canonical(&self) -> Result<Self, Error> {
        Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not implemented"),
        ))
    }
    fn new_element<'i, I: Interner>(&self, _: QualifiedName<'i, I>) -> Result<Self, Error> {
        Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not implemented"),
        ))
    }
    fn new_text(&self, _: Rc<Value>) -> Result<Self, Error> {
        Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not implemented"),
        ))
    }
    fn new_attribute<'i, I: Interner>(
        &self,
        _: QualifiedName<'i, I>,
        _: Rc<Value>,
    ) -> Result<Self, Error> {
        Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not implemented"),
        ))
    }
    fn new_comment(&self, _: Rc<Value>) -> Result<Self, Error> {
        Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not implemented"),
        ))
    }
    fn new_processing_instruction<'i, I: Interner>(
        &self,
        _: QualifiedName<'i, I>,
        _: Rc<Value>,
    ) -> Result<Self, Error> {
        Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not implemented"),
        ))
    }
    fn new_namespace(&self, _ns: String, _prefix: Option<String>) -> Result<Self, Error> {
        Err(Error::new(ErrorKind::NotImplemented, "not implemented"))
    }
    fn push(&mut self, _: Self) -> Result<(), Error> {
        Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not implemented"),
        ))
    }
    fn pop(&mut self) -> Result<(), Error> {
        Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not implemented"),
        ))
    }
    fn insert_before(&mut self, _: Self) -> Result<(), Error> {
        Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not implemented"),
        ))
    }
    fn add_attribute(&self, _: Self) -> Result<(), Error> {
        Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not implemented"),
        ))
    }
    fn shallow_copy(&self) -> Result<Self, Error> {
        Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not implemented"),
        ))
    }
    fn deep_copy(&self) -> Result<Self, Error> {
        Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not implemented"),
        ))
    }
    fn xmldecl(&self) -> XMLDecl {
        XMLDeclBuilder::new().build()
    }
    fn set_xmldecl(&mut self, _: XMLDecl) -> Result<(), Error> {
        Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not implemented"),
        ))
    }

    fn add_namespace(&self, _: Self) -> Result<(), Error> {
        Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not implemented"),
        ))
    }

    fn is_id(&self) -> bool {
        false
    }

    fn is_idrefs(&self) -> bool {
        false
    }
    fn get_dtd<'i, I: Interner>(&self) -> Option<DTD<'i, I>> {
        None
    }
    fn set_dtd<'i, I: Interner>(&self, _dtd: DTD<'i, I>) -> Result<(), Error> {
        Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not implemented"),
        ))
    }

    fn validate(&self, _sch: Schema) -> Result<(), ValidationError> {
        Err(ValidationError::SchemaError("Not Implemented".to_string()))
    }
}

pub struct NulloIter();
impl NulloIter {
    fn new() -> Self {
        NulloIter()
    }
}
impl Iterator for NulloIter {
    type Item = Nullo;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
impl fmt::Debug for Nullo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Nullo node")
    }
}

impl PartialEq for Nullo {
    fn eq(&self, other: &Self) -> bool {
        Node::eq::<LocalInternment>(self, other)
    }
}
