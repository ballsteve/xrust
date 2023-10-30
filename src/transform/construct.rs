//! These functions construct nodes, possibly destined for the result document.

use std::rc::Rc;
use crate::qname::QualifiedName;
use crate::value::Value;
use crate::Item;
use crate::xdmerror::{Error, ErrorKind};
use crate::transform::context::Context;
use crate::item::{Sequence, SequenceTrait, Node, NodeType};
use crate::transform::Transform;

/// An empty sequence.
pub(crate) fn empty<N: Node>(_ctxt: &Context<N>) -> Result<Sequence<N>, Error> {
    Ok(Sequence::new())
}

/// Creates a singleton sequence with the given value
pub(crate) fn literal<N: Node>(
    _ctxt: &Context<N>,
    val: &Rc<Item<N>>,
) -> Result<Sequence<N>, Error> {
    Ok(vec![val.clone()])
}

/// Creates a singleton sequence with a new element node.
/// The transform is evaluated to create the content of the element.
pub(crate) fn literal_element<N: Node>(
    ctxt: &Context<N>,
    qn: &QualifiedName,
    c: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    if ctxt.rd.is_none() {
        return Err(Error::new(
            ErrorKind::Unknown,
            String::from("context has no result document"),
        ));
    }
    let r = ctxt.rd.clone().unwrap();

    let mut e = r.new_element(qn.clone())?;
    ctxt.dispatch(c)?.iter()
        .try_for_each(|i| {
            // Item could be a Node or text
            match &**i {
                Item::Node(t) => match t.node_type() {
                    NodeType::Attribute => e.add_attribute(t.clone()),
                    _ => e.push(t.clone()),
                },
                _ => {
                    // Add the Value as a text node
                    let n = r.new_text(Value::from(i.to_string()))?;
                    e.push(n)
                }
            }
        })?;
    Ok(vec![Rc::new(Item::Node(e))])
}

/// Creates a singleton sequence with a new attribute node.
/// The transform is evaluated to create the value of the attribute.
/// TODO: AVT for attribute name
pub(crate) fn literal_attribute<N: Node>(
    ctxt: &Context<N>,
    qn: &QualifiedName,
    t: &Transform<N>,
) -> Result<Sequence<N>, Error> {
        if ctxt.rd.is_none() {
            return Err(Error::new(
                ErrorKind::Unknown,
                String::from("context has no result document"),
            ))
        }

        let a = ctxt
            .rd
            .clone()
            .unwrap()
            .new_attribute(qn.clone(), Value::from(ctxt.dispatch(t)?.to_string()))?;
        Ok(vec![Rc::new(Item::Node(a))])
}

/// Construct a [Sequence] of items
pub(crate) fn make_sequence<N: Node>(
    ctxt: &Context<N>,
    items: &Vec<Transform<N>>,
) -> Result<Sequence<N>, Error> {
    items.iter().try_fold(
        vec![],
        |mut acc, i| {
            let mut r = ctxt.dispatch(i)?;
            acc.append(&mut r);
            Ok(acc)
        }
    )
}
/// Shallow copy of an item.
/// The first argument selects the items to be copied.
/// The second argument creates the content of the target item.
pub(crate) fn copy<N: Node>(
    ctxt: &Context<N>,
    s: &Transform<N>,
    c: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    let sel = ctxt.dispatch(s)?;
    let mut result: Sequence<N> = Vec::new();
    for k in sel {
        let cp = k.shallow_copy()?;
        result.push(Rc::new(cp.clone()));
        match cp {
            Item::Node(mut im) => {
                for j in ctxt.dispatch(c)? {
                    match &*j {
                        Item::Value(v) => im.push(im.new_text(v.clone())?)?,
                        Item::Node(n) => im.push(n.clone())?,
                        _ => {
                            return Err(Error::new(
                                        ErrorKind::NotImplemented,
                                        String::from("not yet implemented"),
                                    ))
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Ok(result)
}

/// Deep copy of an item.
/// The first argument selects the items to be copied. If not specified then the context item is copied.
pub(crate) fn deep_copy<N: Node>(
    ctxt: &Context<N>,
    s: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    let sel = ctxt.dispatch(s)?;
    let mut result: Sequence<N> = Vec::new();
    for k in sel {
        result.push(Rc::new(k.deep_copy()?));
    }
    Ok(result)
}
