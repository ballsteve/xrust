mod derive;
mod pattern;

use crate::trees::smite::RNode;
use crate::validators::relaxng::derive::{derive, is_nullable};
use crate::validators::ValidationError;

pub fn validate_relaxng(doc: &RNode, schema: &RNode) -> Result<(), ValidationError> {
    let schemapattern = pattern::prepare(schema);

    match schemapattern {
        Err(_) => Err(ValidationError::SchemaError(
            "Pattern Prep Error".to_string(),
        )),
        Ok((pat, refs)) => {
            if is_nullable(derive(doc, pat, &refs)) {
                Ok(())
            } else {
                Err(ValidationError::DocumentError("Some Error".to_string()))
            }
        }
    }
}

//TODO:
//Patterns split in two
//Pattern and elementrefs hashmap
//adjust validator to do lookups against hashmap when it encounters refs
