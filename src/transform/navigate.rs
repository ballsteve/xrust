//! Navigation routines

use crate::item::{Node, NodeType, Sequence, SequenceTrait};
use crate::transform::context::{Context, ContextBuilder, StaticContext};
use crate::transform::{Axis, NodeMatch, Transform};
use crate::xdmerror::{Error, ErrorKind};
use crate::Item;
use std::rc::Rc;

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
        match &*ctxt.cur[0] {
            Item::Node(n) => match n.node_type() {
                NodeType::Document => Ok(vec![Rc::new(Item::Node(n.clone()))]),
                _ => n
                    .ancestor_iter()
                    .last()
                    .map_or(Ok(vec![]), |m| Ok(vec![Rc::new(Item::Node(m))])),
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
pub(crate) fn compose<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    steps: &Vec<Transform<N>>,
) -> Result<Sequence<N>, Error> {
    steps.iter().try_fold(ctxt.cur.clone(), |seq, t| {
        ContextBuilder::from(ctxt).current(seq).build().dispatch(stctxt, t)
    })
}

/// For each item in the current context, evaluate the given node matching operation.
pub(crate) fn step<N: Node>(ctxt: &Context<N>, nm: &NodeMatch) -> Result<Sequence<N>, Error> {
    match ctxt.cur.iter().try_fold(vec![], |mut acc, i| {
        match &**i {
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
                                c.push_node(a.clone());
                                c
                            },
                        );
                        acc.append(&mut s);
                        Ok(acc)
                    }
                    Axis::Parent => match n.parent() {
                        Some(p) => {
                            acc.push_node(p.clone());
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
                            .for_each(|c| acc.push_node(c.clone()));

                        Ok(acc)
                    }
                    Axis::DescendantOrSelf => {
                        if nm.matches(n) {
                            acc.push(i.clone())
                        }
                        n.descend_iter()
                            .filter(|c| nm.matches(c))
                            .for_each(|c| acc.push_node(c.clone()));
                        Ok(acc)
                    }
                    Axis::DescendantOrSelfOrRoot => {
                        acc.push_node(n.owner_document().clone());
                        if nm.matches(n) {
                            acc.push(i.clone())
                        }
                        n.descend_iter()
                            .filter(|c| nm.matches(c))
                            .for_each(|c| acc.push_node(c.clone()));
                        Ok(acc)
                    }
                    Axis::Ancestor => {
                        n.ancestor_iter()
                            .filter(|c| nm.matches(c))
                            .for_each(|c| acc.push_node(c.clone()));

                        Ok(acc)
                    }
                    Axis::AncestorOrSelf => {
                        n.ancestor_iter()
                            .filter(|c| nm.matches(c))
                            .for_each(|c| acc.push_node(c.clone()));
                        if nm.matches(n) {
                            acc.push(i.clone())
                        }
                        Ok(acc)
                    }
                    Axis::FollowingSibling => {
                        n.next_iter()
                            .filter(|c| nm.matches(c))
                            .for_each(|c| acc.push_node(c.clone()));

                        Ok(acc)
                    }
                    Axis::PrecedingSibling => {
                        n.prev_iter()
                            .filter(|c| nm.matches(c))
                            .for_each(|c| acc.push_node(c.clone()));

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
                            acc.push_node(g.clone());
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
                            acc.push_node(g.clone());
                        });
                        Ok(acc)
                    }
                    Axis::Attribute => {
                        n.attribute_iter()
                            .filter(|a| nm.matches(a))
                            .for_each(|a| acc.push_node(a.clone()));
                        Ok(acc)
                    }
                    Axis::SelfAttribute => {
                        if n.node_type() == NodeType::Attribute {
                            acc.push_node(n.clone())
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
            // Eliminate duplicates
            r.dedup_by(|a, b| {
                get_node(a).map_or(false, |aa| get_node(b).map_or(false, |bb| aa.is_same(bb)))
            });
            Ok(r)
        }
        Err(err) => Err(err),
    }
}

fn get_node<N: Node>(i: &Rc<Item<N>>) -> Result<&N, Error> {
    match &**i {
        Item::Node(n) => Ok(n),
        _ => Err(Error::new(ErrorKind::Unknown, String::from("not a node"))),
    }
}

/// Remove items that don't match the predicate.
pub(crate) fn filter<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    predicate: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    ctxt.cur.iter().try_fold(vec![], |mut acc, i| {
        if ContextBuilder::from(ctxt)
            .current(vec![i.clone()])
            .build()
            .dispatch(stctxt, predicate)?
            .to_bool()
        {
            acc.push(i.clone())
        }
        Ok(acc)
    })
}
