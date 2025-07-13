//! These functions are for features defined in XPath Functions 1.0 and 2.0.

use crate::item::{Item, Node, Sequence};
use crate::qname::Interner;
use crate::transform::context::Context;
use crate::xdmerror::{Error, ErrorKind};

/// XSLT current-group function.
pub fn current_group<'i, I: Interner, N: Node>(
    ctxt: &Context<'i, I, N>,
) -> Result<Sequence<N>, Error> {
    Ok(ctxt.current_group.clone())
}

/// XSLT current-grouping-key function.
pub fn current_grouping_key<'i, I: Interner, N: Node>(
    ctxt: &Context<'i, I, N>,
) -> Result<Sequence<N>, Error> {
    ctxt.current_grouping_key.clone().map_or_else(
        || {
            Err(Error::new(
                ErrorKind::TypeError,
                String::from("no current grouping key"),
            ))
        },
        |k| Ok(vec![Item::Value(k)]),
    )
}
