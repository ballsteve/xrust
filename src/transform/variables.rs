//! Support for variables.

use std::rc::Rc;

use crate::xdmerror::{Error, ErrorKind};
use crate::value::{Value, Operator};
use crate::item::{Item, Node, NodeType, Sequence, SequenceTrait};
use crate::transform::Transform;
use crate::transform::context::{Context, ContextBuilder};

/// Declare a variable in a new scope and then evaluate the given transformation.
/// Returns the result of the transformation.
pub fn declare_variable<N: Node>(
    ctxt: &Context<N>,
    name: String,
    value: &Transform<N>,
    f: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    ContextBuilder::from(ctxt)
        .variable(name, ctxt.dispatch(value)?)
        .build()
        .dispatch(f)
}
pub fn reference_variable<N: Node>(
    ctxt: &Context<N>,
    name: &String,
) -> Result<Sequence<N>, Error> {
    match ctxt.vars.get(name) {
        Some(u) => match u.last() {
            Some(t) => Ok(t.clone()),
            None => Err(Error::new(
                ErrorKind::Unknown,
                format!("variable \"{}\" is no longer in scope", name),
            )),
        },
        None => Err(Error::new(
            ErrorKind::Unknown,
            format!("unknown variable \"{}\"", name),
        )),
    }
}
