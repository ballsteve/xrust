//! Miscellaneous support functions.

use crate::item::{Node, Sequence, SequenceTrait};
use crate::qname::{Interner, QualifiedName};
use crate::transform::context::{Context, StaticContext};
use crate::transform::Transform;
use crate::xdmerror::Error;
use crate::ErrorKind;
use url::Url;

/// XSLT current function.
pub fn current<'i, I: Interner, N: Node>(ctxt: &Context<'i, I, N>) -> Result<Sequence<N>, Error> {
    if ctxt.previous_context.is_some() {
        Ok(vec![ctxt.previous_context.as_ref().unwrap().clone()])
    } else {
        Err(Error::new(
            ErrorKind::DynamicAbsent,
            String::from("current item missing"),
        ))
    }
}

/// Emits a message from the stylesheet.
/// The transform is evaluated to create the content of the message.
pub(crate) fn message<
    'i,
    I: Interner,
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<'i, I, N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    body: &Transform<'i, I, N>,
    _sel: &Option<Box<Transform<'i, I, N>>>, // select expression, an alternative to body
    _e: &Transform<'i, I, N>,                // error code
    t: &Transform<'i, I, N>,                 // terminate
) -> Result<Sequence<N>, Error> {
    let msg = ctxt.dispatch(stctxt, body)?.to_string();
    if let Some(f) = &mut stctxt.message {
        f(msg.as_str())?
    }
    match ctxt.dispatch(stctxt, t)?.to_string().trim() {
        "yes" => {
            // TODO: return error code
            Err(Error {
                kind: ErrorKind::Terminated,
                message: msg,
                code: Some(String::from("XTMM9000")),
            })
        }
        _ => Ok(vec![]),
    }
}
