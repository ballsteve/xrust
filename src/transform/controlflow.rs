//! These functions are for features that control program flow.

use std::collections::HashMap;
use std::rc::Rc;

use crate::item::{Node, Sequence, SequenceTrait};
use crate::transform::context::{Context, ContextBuilder, StaticContext};
use crate::transform::{Grouping, Transform};
use crate::value::{Operator, Value};
use crate::xdmerror::{Error, ErrorKind};

/// Iterate over the items in a sequence.
// TODO: Allow multiple variables
pub(crate) fn tr_loop<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    v: &Vec<(String, Transform<N>)>,
    b: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    if v.is_empty() {
        return Ok(vec![]);
    }
    // This implementation only supports one variable

    let mut result = vec![];

    for i in ctxt.dispatch(stctxt, &v[0].1)? {
        // Define a new context with all of the variables declared
        let lctxt = ContextBuilder::from(ctxt)
            .variable(v[0].0.clone(), vec![i.clone()])
            .build();
        let mut t = lctxt.dispatch(stctxt, b)?;
        result.append(&mut t);
    }
    Ok(result)
}

/// Choose a sequence to return.
pub(crate) fn switch<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    v: &Vec<(Transform<N>, Transform<N>)>,
    o: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    let mut candidate = ctxt.dispatch(stctxt, o)?;
    for (t, w) in v {
        let r = ctxt.dispatch(stctxt, t)?;
        if r.to_bool() {
            candidate = ctxt.dispatch(stctxt, w)?;
            break;
        }
    }
    Ok(candidate)
}

/// Evaluate a combinator for each item.
pub fn for_each<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    g: &Option<Grouping<N>>,
    s: &Transform<N>,
    body: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    match g {
        None => {
            let mut result: Sequence<N> = Vec::new();
            for i in ctxt.dispatch(stctxt, s)? {
                let mut v = ContextBuilder::from(ctxt)
                    .context(vec![i.clone()])
                    .previous_context(i)
                    .build()
                    .dispatch(stctxt, body)?;
                result.append(&mut v);
            }
            Ok(result)
        }
        Some(Grouping::By(b)) => group_by(ctxt, stctxt, &b, s, body),
        Some(Grouping::Adjacent(a)) => group_adjacent(ctxt, stctxt, &a, s, body),
        Some(Grouping::StartingWith(v)) => group_starting_with(ctxt, stctxt, &v, s, body),
        Some(Grouping::EndingWith(v)) => group_ending_with(ctxt, stctxt, &v, s, body),
    }
}

/// Evaluate a combinator for each group of items.
fn group_by<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    by: &Vec<Transform<N>>,
    s: &Transform<N>,
    body: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    // Each 'by' expression is evaluated to a string key and stored in the hashmap
    // TODO: this implementation is only supporting a single key
    let t = by[0].clone();
    let mut groups = HashMap::new();
    ctxt.dispatch(stctxt, s)?.iter().try_for_each(|i| {
        // There may be multiple keys returned.
        // For each one, add this item into the group for that key
        ContextBuilder::from(ctxt)
            .context(vec![i.clone()])
            .previous_context(i.clone())
            .build()
            .dispatch(stctxt, &t)?
            .iter()
            .for_each(|k| {
                let e: &mut Sequence<N> = groups.entry(k.to_string()).or_default();
                e.push(i.clone());
            });
        Ok(())
    })?;

    // Now evaluate the body for each group
    groups.iter().try_fold(vec![], |mut result, (k, v)| {
        // Set current-group and current-grouping-key
        let mut r = ContextBuilder::from(ctxt)
            .current_grouping_key(Rc::new(Value::from(k.clone())))
            .current_group(v.clone())
            .build()
            .dispatch(stctxt, body)?;
        result.append(&mut r);
        Ok(result)
    })
}

/// Evaluate a combinator for each group of items. 'adj' is an expression that is evaluated for each selected item. It must resolve to a singleton item. The first item starts the first group. For the second and subsequent items, if the 'adj' item is the same as the previous item then the item is added to the same group. Otherwise a new group is started.
fn group_adjacent<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    adj: &Vec<Transform<N>>,
    s: &Transform<N>,
    body: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    // TODO: this implementation is only supporting a single key
    let t = adj[0].clone();
    let mut groups = Vec::new();
    let sel = ctxt.dispatch(stctxt, s)?;
    if sel.is_empty() {
        return Ok(vec![]);
    } else {
        let mut curgrp = vec![sel[0].clone()];
        let mut curkey = ContextBuilder::from(ctxt)
            .context(vec![sel[1].clone()])
            .build()
            .dispatch(stctxt, &t)?;
        if curkey.len() != 1 {
            return Err(Error::new(
                ErrorKind::Unknown,
                String::from("group-adjacent attribute must evaluate to a single item"),
            ));
        }
        sel.iter().skip(1).try_for_each(|i| {
            let thiskey = ContextBuilder::from(ctxt)
                .context(vec![i.clone()])
                .previous_context(i.clone())
                .build()
                .dispatch(stctxt, &t)?;
            if thiskey.len() == 1 {
                if curkey[0].compare(&thiskey[0], Operator::Equal)? {
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
            .current_grouping_key(Rc::new(Value::from(k.clone())))
            .current_group(v.clone())
            .build()
            .dispatch(stctxt, body)?;
        result.append(&mut r);
        Ok(result)
    })
}

/// Evaluate a combinator for each group of items.
fn group_starting_with<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    _ctxt: &Context<N>,
    _stctxt: &mut StaticContext<F>,
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
pub fn group_ending_with<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    _ctxt: &Context<N>,
    _stctxt: &mut StaticContext<F>,
    _pat: &Vec<Transform<N>>,
    _s: &Transform<N>,
    _body: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    Err(Error::new(
        ErrorKind::NotImplemented,
        String::from("not implemented"),
    ))
}
