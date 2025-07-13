//! Support for variables.

use crate::item::{Node, Sequence};
use crate::qname::Interner;
use crate::transform::context::{Context, ContextBuilder, StaticContext};
use crate::transform::Transform;
use crate::xdmerror::{Error, ErrorKind};
use url::Url;

/// Declare a variable in a new scope and then evaluate the given transformation.
/// Returns the result of the transformation.
pub fn declare_variable<
    'i,
    I: Interner,
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<'i, I, N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    name: String,
    value: &Transform<'i, I, N>,
    f: &Transform<'i, I, N>,
) -> Result<Sequence<N>, Error> {
    ContextBuilder::from(ctxt)
        .variable(name, ctxt.dispatch(stctxt, value)?)
        .build()
        .dispatch(stctxt, f)
}
pub fn reference_variable<'i, I: Interner, N: Node>(
    ctxt: &Context<'i, I, N>,
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
