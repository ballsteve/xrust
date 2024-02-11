//! These functions are for features defined in XPath Functions 1.0 and 2.0.

use std::rc::Rc;

use crate::item::{Item, Node, Sequence};
use crate::transform::context::{Context, StaticContext};
use crate::transform::Transform;
use crate::value::Value;
use crate::xdmerror::{Error, ErrorKind};

/// XPath position function.
pub fn position<N: Node>(ctxt: &Context<N>) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::from(ctxt.i as i64 + 1)))])
}

/// XPath last function.
pub fn last<N: Node>(ctxt: &Context<N>) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::from(
        ctxt.cur.len() as i64
    )))])
}

/// XPath count function.
pub fn tr_count<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    s: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::from(
        ctxt.dispatch(stctxt, s)?.len() as i64,
    )))])
}

/// XPath generate-id function.
pub fn generate_id<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    s: &Option<Box<Transform<N>>>,
) -> Result<Sequence<N>, Error> {
    let i = match s {
        None => {
            ctxt.cur[ctxt.i].clone()
        }
        Some(t) => {
            let seq = ctxt.dispatch(stctxt, t)?;
            match seq.len() {
                0 => return Ok(vec![Item::Value(Rc::new(Value::from("")))]),
                1 => seq[0].clone(),
                _ => return Err(Error::new(ErrorKind::TypeError, String::from("not a singleton sequence"))),
            }
        }
    };
    match i {
        Item::Node(n) => Ok(vec![Item::Value(Rc::new(Value::from(n.get_id())))]),
        _ => Err(Error::new(ErrorKind::TypeError, String::from("not a node")))
    }
}

/// A user defined function.
/// Each argument is declared as a variable in the [Context].
/// The body of the function is then evaluated and it's result is returned.
pub fn user_defined<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    arguments: &Vec<(String, Transform<N>)>,
    body: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    let mut new_ctxt = ctxt.clone();
    arguments
        .iter()
        .try_for_each(|(n, a)| match ctxt.dispatch(stctxt, a) {
            Ok(b) => {
                new_ctxt.var_push(n.clone(), b);
                Ok(())
            }
            Err(err) => Err(err),
        })?;
    new_ctxt.dispatch(stctxt, body)
}

pub(crate) fn tr_error<N: Node>(
    _ctxt: &Context<N>,
    kind: &ErrorKind,
    msg: &String,
) -> Result<Sequence<N>, Error> {
    Err(Error::new(kind.clone(), msg.clone()))
}

pub(crate) fn not_implemented<N: Node>(
    _ctxt: &Context<N>,
    msg: &String,
) -> Result<Sequence<N>, Error> {
    Err(Error::new(ErrorKind::NotImplemented, msg.clone()))
}
