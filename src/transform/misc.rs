//! Miscellaneous support functions.

use crate::ErrorKind;
use crate::item::{Node, Sequence, SequenceTrait};
use crate::transform::Transform;
use crate::transform::context::{Context, StaticContext};
use crate::xdmerror::Error;
use qualname::{NamespaceUri, NcName, QName};
use url::Url;

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
pub(crate) fn message<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
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
                code: Some(QName::new_from_parts(
                    NcName::try_from("XTMM9000").unwrap(),
                    Some(NamespaceUri::try_from("http://www.w3.org/2005/xqt-errors").unwrap()),
                )),
            })
        }
        _ => Ok(vec![]),
    }
}
