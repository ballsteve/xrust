pub mod relaxng;

use std::rc::Rc;
use crate::trees::smite::{RNode, Node as SmiteNode};
use crate::parser::{ParserConfig, xml};
use crate::validators::relaxng::validate_relaxng;


pub(crate) enum Schema{
    //Schematron(String), //Schema File
    //XMLSchema(schemafile)
    RelaxNG(String) //Schema File
    //DTD //How do we pull the DTD? Store on doc while parsing?
}

pub enum ValidationError{
    DocumentError(String),
    SchemaError(String)
}


pub(crate) fn validate(doc: &RNode, s: Schema) -> Result<(), ValidationError>  {
    match s {
        Schema::RelaxNG(schema) => {
            let schemadoc = Rc::new(SmiteNode::new());
            let _ = xml::parse(schemadoc.clone(), schema.as_str(), ParserConfig::new());
            validate_relaxng(doc, &schemadoc)
        }
    }
}
