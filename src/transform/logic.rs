//! These functions are for features defined in XPath Functions 1.0 and 2.0.

use std::rc::Rc;

use crate::item::{Item, Node, Sequence, SequenceTrait};
use crate::transform::context::{Context, StaticContext};
use crate::transform::Transform;
use crate::value::{Operator, Value};
use crate::xdmerror::{Error, ErrorKind};

/// Return the disjunction of all of the given functions.
pub(crate) fn tr_or<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    v: &Vec<Transform<N>>,
) -> Result<Sequence<N>, Error> {
    // Future: Evaluate every operand to check for dynamic errors
    let mut b = false;
    let mut i = 0;
    loop {
        match v.get(i) {
            Some(a) => {
                if ctxt.dispatch(stctxt, a)?.to_bool() {
                    b = true;
                    break;
                }
                i += 1;
            }
            None => break,
        }
    }
    Ok(vec![Rc::new(Item::Value(Value::from(b)))])
}

/// Return the conjunction of all of the given functions.
pub(crate) fn tr_and<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    v: &Vec<Transform<N>>,
) -> Result<Sequence<N>, Error> {
    // Future: Evaluate every operand to check for dynamic errors
    let mut b = true;
    let mut i = 0;
    loop {
        match v.get(i) {
            Some(a) => {
                if !ctxt.dispatch(stctxt, a)?.to_bool() {
                    b = false;
                    break;
                }
                i += 1;
            }
            None => break,
        }
    }
    Ok(vec![Rc::new(Item::Value(Value::from(b)))])
}

/// General comparison of two sequences.
pub(crate) fn general_comparison<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    o: &Operator,
    l: &Transform<N>,
    r: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    let left = ctxt.dispatch(stctxt, l)?;
    let right = ctxt.dispatch(stctxt, r)?;

    let mut b = false;
    for i in left {
        for j in &right {
            b = i.compare(&*j, *o).unwrap();
            if b {
                break;
            }
        }
        if b {
            break;
        }
    }

    Ok(vec![Rc::new(Item::Value(Value::from(b)))])
}

/// Value comparison of two singelton sequences.
pub(crate) fn value_comparison<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    o: &Operator,
    l: &Transform<N>,
    r: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    let left = ctxt.dispatch(stctxt, l)?;
    if left.len() != 1 {
        return Err(Error::new(
            ErrorKind::TypeError,
            String::from("left-hand sequence is not a singleton sequence"),
        ));
    }
    let right = ctxt.dispatch(stctxt, r)?;
    if right.len() != 1 {
        return Err(Error::new(
            ErrorKind::TypeError,
            String::from("right-hand sequence is not a singleton sequence"),
        ));
    }

    Ok(vec![Rc::new(Item::Value(Value::from(
        left[0].compare(&*right[0], *o)?,
    )))])
}

/// Each function in the supplied vector is evaluated, and the resulting sequences are combined into a single sequence.
/// TODO: eliminate duplicates
pub(crate) fn union<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    branches: &Vec<Transform<N>>,
) -> Result<Sequence<N>, Error> {
    let mut result = vec![];
    for b in branches {
        let mut c = ctxt.dispatch(stctxt, b)?;
        result.append(&mut c)
    }
    Ok(result)
}
