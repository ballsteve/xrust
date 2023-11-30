//! Miscellaneous support functions.

use crate::item::{Node, Sequence, SequenceTrait};
use crate::transform::context::{Context, StaticContext};
use crate::transform::Transform;
use crate::xdmerror::Error;

/// Emits a message from the stylesheet.
/// The transform is evaluated to create the content of the message.
pub(crate) fn message<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    t: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    let msg = ctxt.dispatch(stctxt, t)?.to_string();
    if let Some(f) =  &mut stctxt.message {
        f(msg.as_str())?
    }
    Ok(vec![])
}
