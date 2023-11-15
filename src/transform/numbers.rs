//! These functions are for features defined in XPath Functions 1.0 and 2.0.

use std::rc::Rc;

use crate::item::{Item, Node, Sequence, SequenceTrait};
use crate::transform::context::Context;
use crate::transform::{ArithmeticOperand, ArithmeticOperator, Transform};
use crate::value::Value;
use crate::xdmerror::{Error, ErrorKind};

/// XPath number function.
pub fn number<N: Node>(ctxt: &Context<N>, num: &Transform<N>) -> Result<Sequence<N>, Error> {
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
pub fn sum<N: Node>(ctxt: &Context<N>, s: &Transform<N>) -> Result<Sequence<N>, Error> {
    Ok(vec![Rc::new(Item::Value(Value::Double(
        ctxt.dispatch(s)?.iter().fold(0.0, |mut acc, i| {
            acc += i.to_double();
            acc
        }),
    )))])
}

/// XPath floor function.
pub fn floor<N: Node>(ctxt: &Context<N>, f: &Transform<N>) -> Result<Sequence<N>, Error> {
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
pub fn ceiling<N: Node>(ctxt: &Context<N>, c: &Transform<N>) -> Result<Sequence<N>, Error> {
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
    pr: &Option<Box<Transform<N>>>,
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
                    ((n[0].to_double() * (10.0_f64).powi(m[0].to_int().unwrap() as i32)).round())
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

/// Generate a sequence with a range of integers.
pub(crate) fn tr_range<N: Node>(
    ctxt: &Context<N>,
    start: &Transform<N>,
    end: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    let s = ctxt.dispatch(start)?;
    let e = ctxt.dispatch(end)?;
    if s.len() == 0 || e.len() == 0 {
        // Empty sequence is the result
        return Ok(vec![]);
    }
    if s.len() != 1 || e.len() != 1 {
        return Err(Error::new(
            ErrorKind::TypeError,
            String::from("operands must be singleton sequence"),
        ));
    }
    let i = s[0].to_int()?;
    let j = e[0].to_int()?;
    if i > j {
        // empty sequence result
        Ok(vec![])
    } else if i == j {
        let mut seq = Sequence::new();
        seq.push_value(Value::Integer(i));
        Ok(seq)
    } else {
        let mut result = Sequence::new();
        for k in i..=j {
            result.push_value(Value::from(k))
        }
        Ok(result)
    }
}

/// Perform an arithmetic operation.
pub(crate) fn arithmetic<N: Node>(
    ctxt: &Context<N>,
    ops: &Vec<ArithmeticOperand<N>>,
) -> Result<Sequence<N>, Error> {
    // Type: the result will be a number, but integer or double?
    // If all of the operands are integers, then the result is integer otherwise double
    // TODO: check the type of all operands to determine type of result (can probably do this in static analysis phase)
    // In the meantime, let's assume the result will be double and convert any integers
    let mut acc = 0.0;
    for o in ops {
        let j = match ctxt.dispatch(&o.operand) {
            Ok(s) => s,
            Err(_) => {
                acc = f64::NAN;
                break;
            }
        };
        if j.len() != 1 {
            acc = f64::NAN;
            break;
        }
        let u = j[0].to_double();
        match o.op {
            ArithmeticOperator::Noop => acc = u,
            ArithmeticOperator::Add => acc += u,
            ArithmeticOperator::Subtract => acc -= u,
            ArithmeticOperator::Multiply => acc *= u,
            ArithmeticOperator::Divide => acc /= u,
            ArithmeticOperator::IntegerDivide => acc /= u, // TODO: convert to integer
            ArithmeticOperator::Modulo => acc = acc % u,
        }
    }
    Ok(vec![Rc::new(Item::Value(Value::from(acc)))])
}
