mod derive;
mod pattern;

use crate::item::Node;
use crate::trees::smite::RNode;
use crate::validators::ValidationError;

pub fn validate_relaxng(doc: &RNode, schema: &RNode) -> Result<(), ValidationError> {
    //let schemapattern = Ok(Pattern::Empty);
    let schemapattern = pattern::patternmaker(schema.clone());
    println!("schemapattern-{:?}", schemapattern);
    let d = derive::derive(doc, schemapattern.unwrap());
    println!("d-{:?}", d);
    println!("d2-{:?}", d.is_nullable());

    if d.is_nullable() {
        Ok(())
    } else {
        Err(ValidationError::DocumentError("Some Error".to_string()))
    }
}

/*
fn is_nullable(pat: pattern::Pattern) -> bool {
    match (pat.name().get_nsuri().as_deref(), pat.name().get_localname().as_str()){
        (Some("http://relaxng.org/ns/structure/1.0"), "element") => {
            println!("boom")
        }
        _ => {println!("boomboom")}
    }
    println!("{:?}",pat.name());
    true
}
 */
