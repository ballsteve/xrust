pub mod dtd;

use crate::item::{Node, NodeType};
use crate::validators::dtd::validate_dtd;

#[derive(Clone)]
pub enum Schema {
    DTD, //Will add the rest as they become available.
}

#[derive(Debug)]
pub enum ValidationError {
    DocumentError(String),
    SchemaError(String),
}

pub(crate) fn validate(doc: &impl Node, schema: Schema) -> Result<(), ValidationError> {
    match doc.node_type() {
        NodeType::Document => match schema {
            Schema::DTD => validate_dtd(doc.clone()),
        },
        _ => Err(ValidationError::DocumentError(
            "Node provided was not a document".to_string(),
        )),
    }
}
