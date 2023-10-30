//! These functions are for features defined in XPath Functions 1.0 and 2.0.

use std::rc::Rc;

use crate::xdmerror::{Error, ErrorKind};
use crate::value::Value;
use crate::item::{Item, Node, NodeType, Sequence, SequenceTrait};
use crate::transform::Transform;
use crate::transform::context::{Context, ContextBuilder};

/// XPath number function.
pub fn number<N: Node>(
    ctxt: &Context<N>,
    num: &Transform<N>,
) -> Result<Sequence<N>, Error> {
        let n = ctxt.dispatch(num)?;
        match n.len() {
            1 => {
                // First try converting to an integer
                match n[0].to_int() {
                    Ok(i) => Ok(vec![Rc::new(Item::Value(Value::Integer(i)))]),
                    _ => {
                        // Otherwise convert to double.
                        // NB. This can't fail. At worst it returns NaN.
                        Ok(vec![Rc::new(Item::Value(Value::Double(n[0].to_double())))])
                    }
                }
            }
            _ => Err(Error::new(
                ErrorKind::TypeError,
                String::from("not a singleton sequence"),
            )),
        }
}

/// XPath sum function.
pub fn sum<N: Node>(
    ctxt: &Context<N>,
    s: &Transform<N>,
) -> Result<Sequence<N>, Error> {
        Ok(vec![Rc::new(Item::Value(Value::Double(
            ctxt.dispatch(s)?.iter().fold(0.0, |mut acc, i| {
                acc += i.to_double();
                acc
            }),
        )))])
}

/// XPath floor function.
pub fn floor<N: Node>(
    ctxt: &Context<N>,
    f: &Transform<N>,
) -> Result<Sequence<N>, Error> {
        let n = ctxt.dispatch(f)?;
        match n.len() {
            1 => Ok(vec![Rc::new(Item::Value(Value::Double(
                n[0].to_double().floor(),
            )))]),
            _ => Err(Error::new(
                ErrorKind::TypeError,
                String::from("not a singleton sequence"),
            )),
        }
}

/// XPath ceiling function.
pub fn ceiling<N: Node>(
    ctxt: &Context<N>,
    c: &Transform<N>,
) -> Result<Sequence<N>, Error> {
        let n = ctxt.dispatch(c)?;
        match n.len() {
            1 => Ok(vec![Rc::new(Item::Value(Value::Double(
                n[0].to_double().ceil(),
            )))]),
            _ => Err(Error::new(
                ErrorKind::TypeError,
                String::from("not a singleton sequence"),
            )),
        }
}

/// XPath round function.
pub fn round<N: Node>(
    ctxt: &Context<N>,
    r: &Transform<N>,
    pr: Option<&Transform<N>>,
) -> Result<Sequence<N>, Error> {
        pr.as_ref().map_or_else(
            || {
                // precision is 0, i.e. round to nearest whole number
                let n = ctxt.dispatch(r)?;
                match n.len() {
                    1 => Ok(vec![Rc::new(Item::Value(Value::Double(
                        n[0].to_double().round(),
                    )))]),
                    _ => Err(Error::new(
                        ErrorKind::TypeError,
                        String::from("not a singleton sequence"),
                    )),
                }
            },
            |p| {
                let n = ctxt.dispatch(r)?;
                let m = ctxt.dispatch(p)?;
                match (n.len(), m.len()) {
                    (1, 1) => Ok(vec![Rc::new(Item::Value(Value::Double(
                        ((n[0].to_double() * (10.0_f64).powi(m[0].to_int().unwrap() as i32))
                            .round())
                            * (10.0_f64).powi(-m[0].to_int().unwrap() as i32),
                    )))]),
                    _ => Err(Error::new(
                        ErrorKind::TypeError,
                        String::from("not a singleton sequence"),
                    )),
                }
            },
        )
}
