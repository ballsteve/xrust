//! Navigation routines

use crate::item::{Node, NodeType, Sequence, SequenceTrait};
use crate::transform::context::{Context, ContextBuilder, StaticContext};
use crate::transform::{Axis, NodeMatch, Transform};
use crate::xdmerror::{Error, ErrorKind};
use crate::Item;
use url::Url;

/// The root node of the context item.
pub(crate) fn root<N: Node>(ctxt: &Context<N>) -> Result<Sequence<N>, Error> {
    if ctxt.cur.is_empty() {
        Err(Error::new(
            ErrorKind::ContextNotNode,
            String::from("no context"),
        ))
    } else {
        // TODO: check all context items.
        // If any of them is not a Node then error.
        match &ctxt.cur[0] {
            Item::Node(n) => match n.node_type() {
                NodeType::Document => Ok(vec![Item::Node(n.clone())]),
                _ => n
                    .ancestor_iter()
                    .last()
                    .map_or(Ok(vec![]), |m| Ok(vec![Item::Node(m)])),
            },
            _ => Err(Error::new(
                ErrorKind::ContextNotNode,
                String::from("context item is not a node"),
            )),
        }
    }
}

/// The context item.
pub(crate) fn context<N: Node>(ctxt: &Context<N>) -> Result<Sequence<N>, Error> {
    ctxt.cur.get(ctxt.i).map_or(
        Err(Error::new(
            ErrorKind::DynamicAbsent,
            String::from("no context"),
        )),
        |i| Ok(vec![i.clone()]),
    )
}

/// Each transform in the supplied vector is evaluated.
/// The sequence returned by a transform is used as the context for the next transform.
/// See also XSLT 20.4.1 for how the current item is set.
pub(crate) fn compose<
    'i,
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<'i, N, F, G, H>,
    steps: &Vec<Transform<N>>,
) -> Result<Sequence<N>, Error> {
    let mut context = ctxt.cur.clone();
    let mut current;
    if ctxt.previous_context.is_none() {
        if ctxt.cur.is_empty() {
            current = None
        } else {
            current = Some(context[ctxt.i].clone())
        }
    } else {
        current = ctxt.previous_context.clone()
    }
    let mut it = steps.iter();
    loop {
        if let Some(t) = it.next() {
            // previous context is the last step's context.
            // If the initial previous context is None, then the current context is also the previous context (XSLT 20.4.1)
            let new = ContextBuilder::from(ctxt)
                .context(context.clone())
                .previous_context(current)
                .build()
                .dispatch(stctxt, t)?;
            if context.len() > ctxt.i {
                current = Some(context[ctxt.i].clone());
            } else {
                current = None
            }
            context = new;
        } else {
            break;
        }
    }
    Ok(context)
    //    steps.iter().try_fold(ctxt.cur.clone(), |seq, t| {
    //        ContextBuilder::from(ctxt)
    //            .current(seq)
    //            .build()
    //            .dispatch(stctxt, t)
    //    })
}

/// For each item in the current context, evaluate the given node matching operation.
pub(crate) fn step<N: Node>(ctxt: &Context<N>, nm: &NodeMatch) -> Result<Sequence<N>, Error> {
    match ctxt.cur.iter().try_fold(vec![], |mut acc, i| {
        match i {
            Item::Node(n) => {
                match nm.axis {
                    Axis::SelfAxis => {
                        if nm.matches(n) {
                            acc.push(i.clone());
                            Ok(acc)
                        } else {
                            Ok(acc)
                        }
                    }
                    Axis::SelfDocument => {
                        if n.node_type() == NodeType::Document {
                            acc.push(i.clone());
                            Ok(acc)
                        } else {
                            Ok(acc)
                        }
                    }
                    Axis::Child => {
                        let mut s = n.child_iter().filter(|c| nm.matches(c)).fold(
                            Sequence::new(),
                            |mut c, a| {
                                c.push_node(&a);
                                c
                            },
                        );
                        acc.append(&mut s);
                        Ok(acc)
                    }
                    Axis::Parent => match n.parent() {
                        Some(p) => {
                            acc.push_node(&p);
                            Ok(acc)
                        }
                        None => Ok(acc),
                    },
                    Axis::ParentDocument => {
                        // Only matches the Document.
                        // If no parent then return the Document
                        // NB. Document is a special kind of Node
                        match n.node_type() {
                            NodeType::Document => {
                                // The context is the document
                                acc.push(i.clone());
                                Ok(acc)
                            }
                            _ => Ok(acc),
                        }
                    }
                    Axis::Descendant => {
                        n.descend_iter()
                            .filter(|c| nm.matches(c))
                            .for_each(|c| acc.push_node(&c));

                        Ok(acc)
                    }
                    Axis::DescendantOrSelf => {
                        if nm.matches(n) {
                            acc.push(i.clone())
                        }
                        n.descend_iter()
                            .filter(|c| nm.matches(c))
                            .for_each(|c| acc.push_node(&c));
                        Ok(acc)
                    }
                    Axis::DescendantOrSelfOrRoot => {
                        acc.push_node(&n.owner_document());
                        if nm.matches(n) {
                            acc.push(i.clone())
                        }
                        n.descend_iter()
                            .filter(|c| nm.matches(c))
                            .for_each(|c| acc.push_node(&c));
                        Ok(acc)
                    }
                    Axis::Ancestor => {
                        n.ancestor_iter()
                            .filter(|c| nm.matches(c))
                            .for_each(|c| acc.push_node(&c));

                        Ok(acc)
                    }
                    Axis::AncestorOrSelf => {
                        n.ancestor_iter()
                            .filter(|c| nm.matches(c))
                            .for_each(|c| acc.push_node(&c));
                        if nm.matches(n) {
                            acc.push(i.clone())
                        }
                        Ok(acc)
                    }
                    Axis::FollowingSibling => {
                        n.next_iter()
                            .filter(|c| nm.matches(c))
                            .for_each(|c| acc.push_node(&c));

                        Ok(acc)
                    }
                    Axis::PrecedingSibling => {
                        n.prev_iter()
                            .filter(|c| nm.matches(c))
                            .for_each(|c| acc.push_node(&c));

                        Ok(acc)
                    }
                    Axis::Following => {
                        // XPath 3.3.2.1: the following axis contains all nodes that are descendants of the root of the tree in which the context node is found, are not descendants of the context node, and occur after the context node in document order.
                        // iow, for each ancestor-or-self node, include every next sibling and its descendants

                        let mut bcc = vec![];

                        // Start with following siblings of self
                        n.next_iter().for_each(|a| {
                            bcc.push(a.clone());
                            a.descend_iter().for_each(|b| bcc.push(b.clone()));
                        });

                        // Now traverse ancestors
                        n.ancestor_iter().for_each(|a| {
                            a.next_iter().for_each(|b| {
                                bcc.push(b.clone());
                                b.descend_iter().for_each(|c| bcc.push(c.clone()));
                            })
                        });
                        bcc.iter().filter(|e| nm.matches(*e)).for_each(|g| {
                            acc.push_node(g);
                        });
                        Ok(acc)
                    }
                    Axis::Preceding => {
                        // XPath 3.3.2.1: the preceding axis contains all nodes that are descendants of the root of the tree in which the context node is found, are not ancestors of the context node, and occur before the context node in document order.
                        // iow, for each ancestor-or-self node, include every previous sibling and its descendants

                        let mut bcc = vec![];

                        // Start with preceding siblings of self
                        n.prev_iter().for_each(|a| {
                            bcc.push(a.clone());
                            a.descend_iter().for_each(|b| bcc.push(b.clone()));
                        });

                        // Now traverse ancestors
                        n.ancestor_iter().for_each(|a| {
                            a.prev_iter().for_each(|b| {
                                bcc.push(b.clone());
                                b.descend_iter().for_each(|c| bcc.push(c.clone()));
                            })
                        });
                        bcc.iter().filter(|e| nm.matches(*e)).for_each(|g| {
                            acc.push_node(g);
                        });
                        Ok(acc)
                    }
                    Axis::Attribute => {
                        n.attribute_iter()
                            .filter(|a| nm.matches(a))
                            .for_each(|a| acc.push_node(&a));
                        Ok(acc)
                    }
                    Axis::SelfAttribute => {
                        if n.node_type() == NodeType::Attribute {
                            acc.push_node(n)
                        }
                        Ok(acc)
                    }
                    _ => Err(Error::new(
                        ErrorKind::NotImplemented,
                        String::from("coming soon"),
                    )),
                }
            }
            _ => Err(Error::new(
                ErrorKind::Unknown,
                String::from("context item is not a node"),
            )),
        }
    }) {
        Ok(mut r) => {
            // Sort in document order
            r.sort_unstable_by(|a, b| {
                get_node_unchecked(a).cmp_document_order(get_node_unchecked(b))
            });
            // Eliminate duplicates
            r.dedup_by(|a, b| {
                get_node(a).map_or(false, |aa| get_node(b).map_or(false, |bb| aa.is_same(bb)))
            });
            Ok(r)
        }
        Err(err) => Err(err),
    }
}

fn get_node_unchecked<N: Node>(i: &Item<N>) -> &N {
    match i {
        Item::Node(n) => n,
        _ => panic!("not a node"),
    }
}
fn get_node<N: Node>(i: &Item<N>) -> Result<&N, Error> {
    match i {
        Item::Node(n) => Ok(n),
        _ => Err(Error::new(ErrorKind::Unknown, String::from("not a node"))),
    }
}

/// Remove items that don't match the predicate.
pub(crate) fn filter<
    'i,
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<'i, N, F, G, H>,
    predicate: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    ctxt.cur.iter().try_fold(vec![], |mut acc, i| {
        if ContextBuilder::from(ctxt)
            .context(vec![i.clone()])
            .previous_context(ctxt.previous_context.clone())
            .build()
            .dispatch(stctxt, predicate)?
            .to_bool()
        {
            acc.push(i.clone())
        }
        Ok(acc)
    })
}
