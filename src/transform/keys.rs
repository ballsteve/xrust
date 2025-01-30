//! Support for keys.

use crate::item::{Node, Sequence};
use crate::transform::context::{Context, ContextBuilder, StaticContext};
use crate::transform::Transform;
use crate::xdmerror::Error;
use crate::{Item, SequenceTrait};
use std::collections::HashMap;
use url::Url;

/// For each key declaration:
/// 1. find the nodes in the document that match the pattern
/// 2. Evaluate the expression to calculate the key value
/// 3. Store the key value -> Node mapping
/// NB. an optimisation is to calculate a key's value the first time that key is accessed
/// TODO: support composite keys
pub(crate) fn populate_key_values<
    'i,
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &mut Context<N>,
    stctxt: &mut StaticContext<'i, N, F, G, H>,
    sd: N,
) -> Result<(), Error> {
    // We have to visit N nodes to compute K keys.
    // In a typical scenario, N >> K so we want to perform a single pass over the nodes.
    for n in sd.owner_document().descend_iter() {
        // Descend visits all nodes except attributes
        // TODO: support attributes
        for (name, d) in &ctxt.keys {
            for (m, u) in d {
                if m.matches(ctxt, stctxt, &Item::Node(n.clone())) {
                    let newctxt = ContextBuilder::from(&*ctxt)
                        .context(vec![Item::Node(n.clone())])
                        .build();
                    let values = newctxt.dispatch(stctxt, u)?;
                    // Each item in values is a value for this key
                    values.iter().for_each(|v| {
                        if let Some(kv) = ctxt.key_values.get_mut(name) {
                            // We've already seen this value, so append to existing mapping
                            if let Some(vv) = kv.get_mut(&v.to_string()) {
                                // This value for this key already has a mapping, so append this node
                                vv.push(n.clone());
                            } else {
                                // This value for this ley has not been seen before, so create new mapping
                                kv.insert(v.to_string(), vec![n.clone()]);
                            }
                        } else {
                            // Haven't seen this key before, so create new mapping
                            let mut new = HashMap::new();
                            new.insert(v.to_string(), vec![n.clone()]);
                            ctxt.key_values.insert(name.clone(), new);
                        }
                    })
                }
            }
        }
    }
    Ok(())
}

/// Look up the value of a key. The value is evaluated to a Sequence. The interpretation of the sequence depends on the key's composite setting.
/// TODO: support composite keys
pub fn key<
    'i,
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<'i, N, F, G, H>,
    name: &Box<Transform<N>>,
    v: &Box<Transform<N>>,
) -> Result<Sequence<N>, Error> {
    let keyname = ctxt.dispatch(stctxt, name)?.to_string();
    Ok(ctxt.dispatch(stctxt, v)?.iter().fold(vec![], |mut acc, s| {
        if let Some(u) = ctxt.key_values.get(&keyname) {
            if let Some(a) = u.get(&s.to_string()) {
                let mut b: Sequence<N> = a.iter().map(|n| Item::Node(n.clone())).collect();
                acc.append(&mut b);
                acc
            } else {
                acc
            }
        } else {
            acc
        }
    }))
}
