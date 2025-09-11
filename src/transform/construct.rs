//! These functions construct nodes, possibly destined for the result document.

use crate::item::{Node, NodeType, Sequence, SequenceTrait};
use crate::output::OutputSpec;
use crate::qname::QualifiedName;
use crate::transform::context::{Context, StaticContext};
use crate::transform::Transform;
use crate::value::{Value, ValueBuilder, ValueData};
use crate::xdmerror::{Error, ErrorKind};
use crate::Item;
use std::rc::Rc;
use url::Url;

/// An empty sequence.
pub(crate) fn empty<N: Node>(_ctxt: &Context<N>) -> Result<Sequence<N>, Error> {
    Ok(Sequence::new())
}

/// Creates a singleton sequence with the given value
pub(crate) fn literal<N: Node>(_ctxt: &Context<N>, val: &Item<N>) -> Result<Sequence<N>, Error> {
    Ok(vec![val.clone()])
}

/// Creates a singleton sequence with a new element node.
/// Also create a Namespace node, if required.
/// The transform is evaluated to create the content of the element.
pub(crate) fn literal_element<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    qn: &Rc<QualifiedName>,
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

    // If the element is in a namespace, check if the namespace is in scope.
    // If not, create and add a Namespace node for that namespace.
    // Issue: the tree is being created from the bottom up, so we can't know if an ancestor will declare the namespace.
    // This will result in lots of redundant Namespace nodes.
    if let Some(ns) = qn.namespace_uri() {
        e.add_namespace(r.new_namespace(ns, qn.prefix())?)?;
    }

    // Create the content of the new element
    ctxt.dispatch(stctxt, c)?.iter().try_for_each(|i| {
        // Item could be a Node or text
        match i {
            Item::Node(t) => {
                match t.node_type() {
                    NodeType::Attribute => e.add_attribute(t.clone()), // TODO: Also check namespace of attribute
                    NodeType::Namespace => e.add_namespace(t.clone()),
                    _ => e.push(t.deep_copy()?),
                }
            }
            _ => {
                // Add the Value as a text node
                let n = r.new_text(Rc::new(Value::from(i.to_string())))?;
                e.push(n)
            }
        }
    })?;

    // TODO: remove redundant namespace declarations from the newly added child elements

    Ok(vec![Item::Node(e)])
}

/// Creates a singleton sequence with a new element node.
/// The name is interpreted as an AVT to determine the element name.
/// The transform is evaluated to create the content of the element.
pub(crate) fn element<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
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
    let mut e = r.new_element(Rc::new(qnavt))?;
    ctxt.dispatch(stctxt, c)?.iter().try_for_each(|i| {
        // Item could be a Node or text
        match i {
            Item::Node(t) => match t.node_type() {
                NodeType::Attribute => e.add_attribute(t.clone()),
                _ => e.push(t.deep_copy()?),
            },
            _ => {
                // Add the Value as a text node
                let n = r.new_text(Rc::new(Value::from(i.to_string())))?;
                e.push(n)
            }
        }
    })?;
    Ok(vec![Item::Node(e)])
}

/// Creates a new text node.
/// The transform is evaluated to create the value of the text node.
/// Special characters are escaped, unless disabled as per output specification.
pub(crate) fn literal_text<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    t: &Transform<N>,
    o: &OutputSpec,
) -> Result<Sequence<N>, Error> {
    if ctxt.rd.is_none() {
        return Err(Error::new(
            ErrorKind::Unknown,
            String::from("context has no result document"),
        ));
    }

    let v = ctxt.dispatch(stctxt, t)?.to_string();
    Ok(vec![Item::Node(
        ctxt.rd.clone().unwrap().new_text(Rc::new(
            ValueBuilder::new()
                .value(ValueData::String(v))
                .output(o.clone())
                .build(),
        ))?,
    )])
}

/// Creates a singleton sequence with a new attribute node.
/// The transform is evaluated to create the value of the attribute.
/// TODO: AVT for attribute name
pub(crate) fn literal_attribute<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    qn: &Rc<QualifiedName>,
    t: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    if ctxt.rd.is_none() {
        return Err(Error::new(
            ErrorKind::Unknown,
            String::from("context has no result document"),
        ));
    }

    let v = ctxt.dispatch(stctxt, t)?;
    let a = ctxt
        .rd
        .clone()
        .unwrap()
        .new_attribute(qn.clone(), Rc::new(Value::from(v.to_string())))?;
    Ok(vec![Item::Node(a)])
}

/// Creates a singleton sequence with a new comment node.
/// The transform is evaluated to create the value of the comment.
pub(crate) fn literal_comment<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
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
        .new_comment(Rc::new(Value::from(ctxt.dispatch(stctxt, t)?.to_string())))?;
    Ok(vec![Item::Node(a)])
}

/// Creates a singleton sequence with a new processing instruction node.
/// The transform is evaluated to create the value of the PI.
pub(crate) fn literal_processing_instruction<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
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
        Rc::new(QualifiedName::new(
            None,
            None,
            ctxt.dispatch(stctxt, name)?.to_string(),
        )),
        Rc::new(Value::from(ctxt.dispatch(stctxt, t)?.to_string())),
    )?;
    Ok(vec![Item::Node(pi)])
}

/// Set an attribute on the context item, which must be an element-type node.
/// (TODO: use an expression to select the element)
/// If the element does not have an attribute with the given name, create it.
/// Otherwise replace the attribute's value with the supplied value.
/// Returns an empty sequence.
pub(crate) fn set_attribute<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    atname: &Rc<QualifiedName>,
    v: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    if ctxt.rd.is_none() {
        return Err(Error::new(
            ErrorKind::Unknown,
            String::from("context has no result document"),
        ));
    }
    if ctxt.context_item.is_none() {
        return Err(Error::new(ErrorKind::DynamicAbsent, "no context item"));
    }
    match &ctxt.context_item.as_ref().unwrap() {
        Item::Node(n) => match n.node_type() {
            NodeType::Element => {
                let od = n.owner_document();
                let attval = ctxt.dispatch(stctxt, v)?;
                if attval.len() == 1 {
                    match attval.first() {
                        Some(Item::Value(av)) => {
                            n.add_attribute(od.new_attribute(atname.clone(), av.clone())?)?;
                        }
                        _ => {
                            n.add_attribute(od.new_attribute(
                                atname.clone(),
                                Rc::new(Value::from(attval.to_string())),
                            )?)?;
                        }
                    }
                } else {
                    n.add_attribute(od.new_attribute(
                        atname.clone(),
                        Rc::new(Value::from(attval.to_string())),
                    )?)?;
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
pub(crate) fn make_sequence<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
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
pub(crate) fn copy<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    s: &Transform<N>,
    c: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    let sel = ctxt.dispatch(stctxt, s)?;
    let mut result: Sequence<N> = Vec::new();
    for k in sel {
        let cp = k.shallow_copy()?;
        result.push(cp.clone());
        match cp {
            Item::Node(mut im) => {
                for j in ctxt.dispatch(stctxt, c)? {
                    match &j {
                        Item::Value(v) => im.push(im.new_text(v.clone())?)?,
                        Item::Node(n) => match n.node_type() {
                            NodeType::Attribute => im.add_attribute(n.clone())?,
                            _ => im.push(n.clone())?,
                        },
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
pub(crate) fn deep_copy<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    s: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    let sel = ctxt.dispatch(stctxt, s)?;
    let mut result: Sequence<N> = Vec::new();
    for k in sel {
        result.push(k.deep_copy()?);
    }
    Ok(result)
}
