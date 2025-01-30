//! These functions are for features defined in XPath Functions 1.0 and 2.0.

use std::rc::Rc;
use url::Url;

use crate::item::{Item, Node, Sequence, SequenceTrait};
use crate::transform::context::{Context, StaticContext};
use crate::transform::Transform;
use crate::value::Value;
use crate::xdmerror::Error;

/// XPath boolean function.
pub fn boolean<
    'i,
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<'i, N, F, G, H>,
    b: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::Boolean(
        ctxt.dispatch(stctxt, b)?.to_bool(),
    )))])
}

/// XPath not function.
pub fn not<
    'i,
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<'i, N, F, G, H>,
    n: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::Boolean(
        !ctxt.dispatch(stctxt, n)?.to_bool(),
    )))])
}

/// XPath true function.
pub fn tr_true<N: Node>(_ctxt: &Context<N>) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::Boolean(true)))])
}

/// XPath false function.
pub fn tr_false<N: Node>(_ctxt: &Context<N>) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::Boolean(false)))])
}
