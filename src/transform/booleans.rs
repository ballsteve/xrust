//! These functions are for features defined in XPath Functions 1.0 and 2.0.

use std::rc::Rc;
use url::Url;

use crate::item::{Item, Node, Sequence, SequenceTrait};
use crate::qname::Interner;
use crate::transform::context::{Context, StaticContext};
use crate::transform::Transform;
use crate::value::Value;
use crate::xdmerror::Error;

/// XPath boolean function.
pub fn boolean<
    'i,
    I: Interner,
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<'i, I, N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    b: &Transform<'i, I, N>,
) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::Boolean(
        ctxt.dispatch(stctxt, b)?.to_bool(),
    )))])
}

/// XPath not function.
pub fn not<
    'i,
    I: Interner,
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<'i, I, N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    n: &Transform<'i, I, N>,
) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::Boolean(
        !ctxt.dispatch(stctxt, n)?.to_bool(),
    )))])
}

/// XPath true function.
pub fn tr_true<'i, I: Interner, N: Node>(_ctxt: &Context<'i, I, N>) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::Boolean(true)))])
}

/// XPath false function.
pub fn tr_false<'i, I: Interner, N: Node>(_ctxt: &Context<'i, I, N>) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::Boolean(false)))])
}
