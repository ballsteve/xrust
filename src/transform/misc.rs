//! Miscellaneous support functions.

use crate::item::{Node, Sequence, SequenceTrait};
use crate::qname::QualifiedName;
use crate::transform::context::{Context, StaticContext};
use crate::transform::Transform;
use crate::xdmerror::Error;
use crate::ErrorKind;

/// XSLT current function.
pub fn current<N: Node>(ctxt: &Context<N>) -> Result<Sequence<N>, Error> {
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
pub(crate) fn message<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    body: &Transform<N>,
    _sel: &Option<Box<Transform<N>>>, // select expression, an alternative to body
    _e: &Transform<N>,                // error code
    t: &Transform<N>,                 // terminate
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
                code: Some(QualifiedName::new(
                    Some(String::from("http://www.w3.org/2005/xqt-errors")),
                    None,
                    String::from("XTMM9000"),
                )),
            })
        }
        _ => Ok(vec![]),
    }
}
