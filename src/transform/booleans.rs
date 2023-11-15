//! These functions are for features defined in XPath Functions 1.0 and 2.0.

use std::rc::Rc;

use crate::item::{Item, Node, Sequence, SequenceTrait};
use crate::transform::context::Context;
use crate::transform::Transform;
use crate::value::Value;
use crate::xdmerror::Error;

/// XPath boolean function.
pub fn boolean<N: Node>(ctxt: &Context<N>, b: &Transform<N>) -> Result<Sequence<N>, Error> {
    Ok(vec![Rc::new(Item::Value(Value::Boolean(
        ctxt.dispatch(b)?.to_bool(),
    )))])
}

/// XPath not function.
pub fn not<N: Node>(ctxt: &Context<N>, n: &Transform<N>) -> Result<Sequence<N>, Error> {
    Ok(vec![Rc::new(Item::Value(Value::Boolean(
        !ctxt.dispatch(n)?.to_bool(),
    )))])
}

/// XPath true function.
pub fn tr_true<N: Node>(_ctxt: &Context<N>) -> Result<Sequence<N>, Error> {
    Ok(vec![Rc::new(Item::Value(Value::Boolean(true)))])
}

/// XPath false function.
pub fn tr_false<N: Node>(_ctxt: &Context<N>) -> Result<Sequence<N>, Error> {
    Ok(vec![Rc::new(Item::Value(Value::Boolean(false)))])
}
