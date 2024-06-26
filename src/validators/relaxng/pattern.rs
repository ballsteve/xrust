use crate::trees::smite::RNode;
use crate::Node;
use std::collections::HashMap;

pub(crate) type Param = (String, String);

#[derive(Debug)]
pub(crate) enum PatternError<'a> {
    NotRelaxNG,
    MissingName,
    Other(&'a str),
}

pub(super) fn prepare(schemadoc: &RNode) -> Result<(RNode, HashMap<String, RNode>), PatternError> {
    //TODO implement

    let mut ci = schemadoc.child_iter();
    let pat = ci.next().unwrap();
    let mut refs = HashMap::new();
    for r in ci {
        refs.insert(r.name().get_localname(), r);
    }
    Ok((pat, refs))
}
