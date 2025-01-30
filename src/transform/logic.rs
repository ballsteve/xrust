//! These functions are for features defined in XPath Functions 1.0 and 2.0.

use std::rc::Rc;
use url::Url;

use crate::item::{Item, Node, Sequence, SequenceTrait};
use crate::transform::context::{Context, StaticContext};
use crate::transform::Transform;
use crate::value::{Operator, Value};
use crate::xdmerror::{Error, ErrorKind};

/// Return the disjunction of all of the given functions.
pub(crate) fn tr_or<
    'i,
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<'i, N, F, G, H>,
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
    Ok(vec![Item::Value(Rc::new(Value::from(b)))])
}

/// Return the conjunction of all of the given functions.
pub(crate) fn tr_and<
    'i,
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<'i, N, F, G, H>,
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
    Ok(vec![Item::Value(Rc::new(Value::from(b)))])
}

/// General comparison of two sequences.
pub(crate) fn general_comparison<
    'i,
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<'i, N, F, G, H>,
    o: &Operator,
    l: &Transform<N>,
    r: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    let left = ctxt.dispatch(stctxt, l)?;
    let right = ctxt.dispatch(stctxt, r)?;

    let mut b = false;
    for i in left {
        for j in &right {
            b = i.compare(j, *o).unwrap();
            if b {
                break;
            }
        }
        if b {
            break;
        }
    }

    Ok(vec![Item::Value(Rc::new(Value::from(b)))])
}

/// Value comparison of two singleton sequences.
pub(crate) fn value_comparison<
    'i,
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<'i, N, F, G, H>,
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

    Ok(vec![Item::Value(Rc::new(Value::from(
        left[0].compare(&right[0], *o)?,
    )))])
}

/// Each function in the supplied vector is evaluated, and the resulting sequences are combined into a single sequence.
/// All items must be nodes.
/// TODO: eliminate duplicates, sort by document order (XPath 3.1 3.4.2).
pub(crate) fn union<
    'i,
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<'i, N, F, G, H>,
    branches: &Vec<Transform<N>>,
) -> Result<Sequence<N>, Error> {
    let mut result = vec![];
    for b in branches {
        let mut c = ctxt.dispatch(stctxt, b)?;
        if c.iter().any(|f| !f.is_node()) {
            return Err(Error::new(
                ErrorKind::TypeError,
                "all operands must be nodes",
            ));
        }
        result.append(&mut c)
    }
    // TODO: eliminate duplicates and sort into document order
    Ok(result)
}
