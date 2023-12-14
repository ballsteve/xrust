//! These functions are for features defined in XPath Functions 1.0 and 2.0.

use crate::item::{Item, Node, Sequence};
use crate::transform::context::Context;
use crate::xdmerror::{Error, ErrorKind};

/// XSLT current-group function.
pub fn current_group<N: Node>(ctxt: &Context<N>) -> Result<Sequence<N>, Error> {
    Ok(ctxt.current_group.clone())
}

/// XSLT current-grouping-key function.
pub fn current_grouping_key<N: Node>(ctxt: &Context<N>) -> Result<Sequence<N>, Error> {
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
