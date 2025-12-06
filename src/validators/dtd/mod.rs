mod derive;

use crate::item::NodeType;
use crate::validators::dtd::derive::{child_deriv, is_nullable};
use crate::validators::ValidationError;
use crate::Node;

pub(crate) fn validate_dtd(doc: impl Node) -> Result<(), ValidationError> {
    match doc.node_type() {
        NodeType::Document => {
            match doc.get_dtd() {
                None => Err(ValidationError::DocumentError(
                    "No DTD Information on the document".to_string(),
                )),
                Some(dtd) => {
                    match &dtd.name {
                        None => Err(ValidationError::DocumentError(
                            "Document name not found in DTD".to_string(),
                        )),
                        Some(n) => {
                            match dtd.patterns.get(n) {
                                None => Err(ValidationError::DocumentError(
                                    "Element Declaration not found.".to_string(),
                                )),
                                Some(pat) => {
                                    //println!("pat-{:?}", pat);
                                    //for pt in &dtd.patterns {
                                    //    println!("{:?}", pt)
                                    //}
                                    match is_nullable(child_deriv(
                                        pat.clone(),
                                        doc.child_iter()
                                            .find(|node| {
                                                node.node_type() != NodeType::ProcessingInstruction
                                                    && node.node_type() != NodeType::Comment
                                                    && !(node.node_type() == NodeType::Text
                                                        && node.value().to_string() == *"")
                                            })
                                            .unwrap(),
                                        dtd,
                                    )) {
                                        true => Ok(()),
                                        false => {
                                            Err(ValidationError::SchemaError("Invalid".to_string()))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => Err(ValidationError::DocumentError(
            "Node provided was not a document".to_string(),
        )),
    }
}
