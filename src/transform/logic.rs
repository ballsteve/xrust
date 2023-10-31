//! These functions are for features defined in XPath Functions 1.0 and 2.0.

use std::rc::Rc;

use crate::xdmerror::{Error, ErrorKind};
use crate::value::{Value, Operator};
use crate::item::{Item, Node, NodeType, Sequence, SequenceTrait};
use crate::transform::Transform;
use crate::transform::context::{Context, ContextBuilder};

/// Return the disjunction of all of the given functions.
pub(crate) fn tr_or<N: Node>(
    ctxt: &Context<N>,
    v: &Vec<Transform<N>>,
) -> Result<Sequence<N>, Error> {
        // Future: Evaluate every operand to check for dynamic errors
        let mut b = false;
        let mut i = 0;
        loop {
            match v.get(i) {
                Some(a) => {
                    if ctxt.dispatch(a)?.to_bool() {
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
pub(crate) fn tr_and<N: Node>(
    ctxt: &Context<N>,
    v: &Vec<Transform<N>>,
) -> Result<Sequence<N>, Error> {
        // Future: Evaluate every operand to check for dynamic errors
        let mut b = true;
        let mut i = 0;
        loop {
            match v.get(i) {
                Some(a) => {
                    if !ctxt.dispatch(a)?.to_bool() {
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
pub(crate) fn general_comparison<N: Node>(
    ctxt: &Context<N>,
    o: &Operator,
    l: &Transform<N>,
    r: &Transform<N>,
) -> Result<Sequence<N>, Error> {
        let left = ctxt.dispatch(l)?;
        let right = ctxt.dispatch(r)?;

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
pub(crate) fn value_comparison<N: Node>(
    ctxt: &Context<N>,
    o: &Operator,
    l: &Transform<N>,
    r: &Transform<N>,
) -> Result<Sequence<N>, Error> {
        let left = ctxt.dispatch(l)?;
        if left.len() != 1 {
            return Err(Error::new(
                ErrorKind::TypeError,
                String::from("left-hand sequence is not a singleton sequence"),
            ));
        }
        let right = ctxt.dispatch(r)?;
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
