//! These functions construct nodes, possibly destined for the result document.

use crate::item::{Node, NodeType, Sequence, SequenceTrait};
use crate::qname::QualifiedName;
use crate::transform::context::{Context, StaticContext};
use crate::transform::Transform;
use crate::value::Value;
use crate::xdmerror::{Error, ErrorKind};
use crate::Item;
use std::rc::Rc;

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
pub(crate) fn literal_element<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
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
    ctxt.dispatch(stctxt, c)?.iter().try_for_each(|i| {
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

/// Creates a singleton sequence with a new element node.
/// The name is interpreted as an AVT to determine the element name.
/// The transform is evaluated to create the content of the element.
pub(crate) fn element<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    qn: &Transform<N>,
    c: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    if ctxt.rd.is_none() {
        return Err(Error::new(
            ErrorKind::Unknown,
            String::from("context has no result document"),
        ));
    }
    let r = ctxt.rd.clone().unwrap();

    let qnavt = QualifiedName::try_from(ctxt.dispatch(stctxt, qn)?.to_string().as_str())?;
    let mut e = r.new_element(qnavt)?;
    ctxt.dispatch(stctxt, c)?.iter().try_for_each(|i| {
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
pub(crate) fn literal_attribute<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    qn: &QualifiedName,
    t: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    if ctxt.rd.is_none() {
        return Err(Error::new(
            ErrorKind::Unknown,
            String::from("context has no result document"),
        ));
    }

    let a = ctxt.rd.clone().unwrap().new_attribute(
        qn.clone(),
        Value::from(ctxt.dispatch(stctxt, t)?.to_string()),
    )?;
    Ok(vec![Rc::new(Item::Node(a))])
}

/// Creates a singleton sequence with a new comment node.
/// The transform is evaluated to create the value of the comment.
pub(crate) fn literal_comment<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    t: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    if ctxt.rd.is_none() {
        return Err(Error::new(
            ErrorKind::Unknown,
            String::from("context has no result document"),
        ));
    }

    let a = ctxt
        .rd
        .clone()
        .unwrap()
        .new_comment(Value::from(ctxt.dispatch(stctxt, t)?.to_string()))?;
    Ok(vec![Rc::new(Item::Node(a))])
}

/// Creates a singleton sequence with a new processing instruction node.
/// The transform is evaluated to create the value of the PI.
pub(crate) fn literal_processing_instruction<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    name: &Transform<N>,
    t: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    if ctxt.rd.is_none() {
        return Err(Error::new(
            ErrorKind::Unknown,
            String::from("context has no result document"),
        ));
    }

    let pi = ctxt.rd.clone().unwrap().new_processing_instruction(
        QualifiedName::new(None, None, ctxt.dispatch(stctxt, name)?.to_string()),
        Value::from(ctxt.dispatch(stctxt, t)?.to_string()),
    )?;
    Ok(vec![Rc::new(Item::Node(pi))])
}

/// Set an attribute on the context item, which must be an element-type node.
/// (TODO: use an expression to select the element)
/// If the element does not have an attribute with the given name, create it.
/// Otherwise replace the attribute's value with the supplied value.
/// Returns an empty sequence.
pub(crate) fn set_attribute<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    atname: &QualifiedName,
    v: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    if ctxt.rd.is_none() {
        return Err(Error::new(
            ErrorKind::Unknown,
            String::from("context has no result document"),
        ));
    }
    match &*ctxt.cur[ctxt.i] {
        Item::Node(n) => match n.node_type() {
            NodeType::Element => {
                let od = n.owner_document();
                let attval = ctxt.dispatch(stctxt, v)?;
                if attval.len() == 1 {
                    match &*attval[0] {
                        Item::Value(av) => {
                            n.add_attribute(od.new_attribute(atname.clone(), av.clone())?)?;
                        }
                        _ => {
                            n.add_attribute(
                                od.new_attribute(atname.clone(), Value::from(attval.to_string()))?,
                            )?;
                        }
                    }
                } else {
                    n.add_attribute(
                        od.new_attribute(atname.clone(), Value::from(attval.to_string()))?,
                    )?;
                }
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::Unknown,
                    String::from("context item is not an element-type node"),
                ))
            }
        },
        _ => {
            return Err(Error::new(
                ErrorKind::Unknown,
                String::from("context item is not a node"),
            ))
        }
    }
    Ok(vec![])
}

/// Construct a [Sequence] of items
pub(crate) fn make_sequence<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    items: &Vec<Transform<N>>,
) -> Result<Sequence<N>, Error> {
    items.iter().try_fold(vec![], |mut acc, i| {
        let mut r = ctxt.dispatch(stctxt, i)?;
        acc.append(&mut r);
        Ok(acc)
    })
}
/// Shallow copy of an item.
/// The first argument selects the items to be copied.
/// The second argument creates the content of the target item.
pub(crate) fn copy<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    s: &Transform<N>,
    c: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    let sel = ctxt.dispatch(stctxt, s)?;
    let mut result: Sequence<N> = Vec::new();
    for k in sel {
        let cp = k.shallow_copy()?;
        result.push(Rc::new(cp.clone()));
        match cp {
            Item::Node(mut im) => {
                for j in ctxt.dispatch(stctxt, c)? {
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
pub(crate) fn deep_copy<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    s: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    let sel = ctxt.dispatch(stctxt, s)?;
    let mut result: Sequence<N> = Vec::new();
    for k in sel {
        result.push(Rc::new(k.deep_copy()?));
    }
    Ok(result)
}
