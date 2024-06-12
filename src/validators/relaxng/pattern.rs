use std::collections::HashMap;
use std::rc::Rc;
use crate::{Error, Node, Value};
use crate::item::NodeType;
use crate::qname::QualifiedName;
use crate::trees::smite::RNode;
use crate::validators::ValidationError;

pub(crate) type DataType = (QualifiedName, String);
type Attributenode = (QualifiedName, String);
pub(crate) type Param = (String, String);
type Context = (QualifiedName, Vec<(String, QualifiedName)>);


#[derive(Debug)]
pub(crate) enum PatternError<'a>{
    NotRelaxNG,
    MissingName,
    Other(&'a str)
}

pub(super) fn prepare(schemadoc: &RNode) -> Result<(RNode, HashMap<String,RNode>), PatternError> {
    //TODO implement

    let mut ci = schemadoc.child_iter();
    let pat = ci.next().unwrap();
    let mut refs = HashMap::new();
    for r in ci {
        refs.insert(r.name().get_localname(), r);
    }
    Ok((pat, refs))
}