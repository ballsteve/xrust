//! These functions are for features that control program flow.

use std::rc::Rc;
use std::collections::HashMap;

use crate::xdmerror::{Error, ErrorKind};
use crate::value::{Value, Operator};
use crate::item::{Item, Node, NodeType, Sequence, SequenceTrait};
use crate::transform::{Grouping, Transform};
use crate::transform::context::{Context, ContextBuilder};

/// Iterate over the items in a sequence.
// TODO: Allow multiple variables
pub(crate) fn tr_loop<N: Node>(
    ctxt: &Context<N>,
    v: &Vec<(String, Transform<N>)>,
    b: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    // Define a new context with all of the variables declared
    let mut lctxt = ctxt.clone();
    v.iter()
        .try_for_each(|(n, d)| {
            lctxt.var_push(n.clone(), ctxt.dispatch(d)?);
            Ok::<(), Error>(())
        });
    // Now dispatch the body of the loop with the new context
    lctxt.dispatch(b)
}

/// Choose a sequence to return.
pub(crate) fn switch<N: Node>(
    ctxt: &Context<N>,
    v: &Vec<(Transform<N>, Transform<N>)>,
    o: &Transform<N>,
) -> Result<Sequence<N>, Error> {
        let mut candidate = ctxt.dispatch(o)?;
        for (t, w) in v {
            let r = ctxt.dispatch(t)?;
            if r.to_bool() {
                candidate = ctxt.dispatch(w)?;
                break;
            }
        }
        Ok(candidate)
}

/// Evaluate a combinator for each item.
pub fn for_each<N: Node>(
    ctxt: &Context<N>,
    g: &Option<Grouping<N>>,
    s: &Transform<N>,
    body: &Transform<N>,
) -> Result<Sequence<N>, Error> {

    match g {
        None => {
            let mut result: Sequence<N> = Vec::new();
            for i in ctxt.dispatch(s)? {
                let mut v = ContextBuilder::from(ctxt).current(vec![i]).build().dispatch(body)?;
                result.append(&mut v);
            }
            Ok(result)
        }
        Some(Grouping::By(b)) => group_by(ctxt, &b, s, body),
        Some(Grouping::Adjacent(a)) => group_adjacent(ctxt, &a, s, body),
        Some(Grouping::StartingWith(v)) => group_starting_with(ctxt, &v, s, body),
        Some(Grouping::EndingWith(v)) => group_ending_with(ctxt, &v, s, body),
    }
}

/// Evaluate a combinator for each group of items.
fn group_by<N: Node>(
    ctxt: &Context<N>,
    by: &Vec<Transform<N>>,
    s: &Transform<N>,
    body: &Transform<N>,
) -> Result<Sequence<N>, Error> {
        // Each 'by' expression is evaluated to a string key and stored in the hashmap
    // TODO: this implementation is only supporting a single key
    let t = by[0].clone();
        let mut groups = HashMap::new();
        ctxt.dispatch(s)?.iter().try_for_each(|i| {
            // There may be multiple keys returned.
            // For each one, add this item into the group for that key
            ContextBuilder::from(ctxt)
                .current(vec![i.clone()])
                .build()
                .dispatch(&t)?
                .iter()
                .for_each(|k| {
                    let e: &mut Vec<Rc<Item<N>>> = groups.entry(k.to_string()).or_default();
                    e.push(i.clone());
                });
            Ok(())
        })?;

        // Now evaluate the body for each group
        groups.iter().try_fold(vec![], |mut result, (k, v)| {
            // Set current-group and current-grouping-key
            let mut r = ContextBuilder::from(ctxt)
                    .current_grouping_key(Value::from(k.clone()))
                    .current_group(v.clone())
                    .build()
                    .dispatch(body)?;
            result.append(&mut r);
            Ok(result)
        })
}

/// Evaluate a combinator for each group of items. 'adj' is an expression that is evaluated for each selected item. It must resolve to a singleton item. The first item starts the first group. For the second and subsequent items, if the 'adj' item is the same as the previous item then the item is added to the same group. Otherwise a new group is started.
fn group_adjacent<N: Node>(
    ctxt: &Context<N>,
    adj: &Vec<Transform<N>>,
    s: &Transform<N>,
    body: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    // TODO: this implementation is only supporting a single key
    let t = adj[0].clone();
        let mut groups = Vec::new();
        let sel = ctxt.dispatch(s)?;
        if sel.is_empty() {
            return Ok(vec![]);
        } else {
            let mut curgrp = vec![sel[0].clone()];
            let mut curkey = ContextBuilder::from(ctxt)
                .current(vec![sel[1].clone()])
                .build()
                .dispatch(&t)?;
            if curkey.len() != 1 {
                return Err(Error::new(
                    ErrorKind::Unknown,
                    String::from("group-adjacent attribute must evaluate to a single item"),
                ));
            }
            sel.iter().skip(1).try_for_each(|i| {
                let thiskey = ContextBuilder::from(ctxt)
                    .current(vec![i.clone()])
                    .build()
                    .dispatch(&t)?;
                if thiskey.len() == 1 {
                    if curkey[0].compare(&*thiskey[0], Operator::Equal)? {
                        // Append to the current group
                        curgrp.push(i.clone())
                    } else {
                        // Close the previous group, start a new group with this item as its first member
                        groups.push((curkey.to_string(), curgrp.clone()));
                        curgrp = vec![i.clone()];
                        curkey = thiskey;
                    }
                    Ok(())
                } else {
                    Err(Error::new(
                        ErrorKind::Unknown,
                        String::from("group-adjacent attribute must evaluate to a single item"),
                    ))
                }
            })?;
            // Close the last group
            groups.push((curkey.to_string(), curgrp))
        }

        // Now evaluate the body for each group
        groups.iter().try_fold(vec![], |mut result, (k, v)| {
            // Set current-group and current-grouping-key
            let mut r = ContextBuilder::from(ctxt)
                    .current_grouping_key(Value::from(k.clone()))
                    .current_group(v.clone())
                    .build()
                    .dispatch(body)?;
            result.append(&mut r);
            Ok(result)
        })
}

/// Evaluate a combinator for each group of items.
fn group_starting_with<N: Node>(
    _ctxt: &Context<N>,
    _pat: &Vec<Transform<N>>,
    _s: &Transform<N>,
    _body: &Transform<N>,
) -> Result<Sequence<N>, Error> {
        Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not implemented"),
        ))
}

/// Evaluate a combinator for each group of items.
pub fn group_ending_with<N: Node>(
    _ctxt: &Context<N>,
    _pat: &Vec<Transform<N>>,
    _s: &Transform<N>,
    _body: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    Err(Error::new(
        ErrorKind::NotImplemented,
        String::from("not implemented"),
    ))
}
