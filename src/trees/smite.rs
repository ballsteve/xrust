/*! # A tree structure for XDM

This module implements the Item module's [Node](crate::item::Node) trait.

This implementation uses interior mutability to create and manage a tree structure that is both mutable and fully navigable.

To create a tree, use [Node::new()](crate::trees::smite::Node) to make a Document-type node.
To add a node, first create it using a creation method, defined by the [Node](crate::item::Node) trait, such as new_element() or new_text(),
then use the push(), insert_before(), or add_attribute() method to attach it to a node in the tree.

NB. The Item module's Node trait is implemented for Rc\<smite::Node\>. For convenience, this is defined as the type [RNode](crate::trees::smite::RNode).

```rust
use std::rc::Rc;
use xrust::trees::smite::RNode;
use xrust::item::{Node as ItemNode, NodeType};
use xrust::qname::QualifiedName;
use xrust::value::Value;
use xrust::xdmerror::Error;

pub(crate) type ExtDTDresolver = fn(Option<String>, String) -> Result<String, Error>;

// A document always has a NodeType::Document node as the toplevel node.
let mut doc = RNode::new_document();

// Create an element-type node. Upon creation, it is *not* attached to the tree.
let mut top = doc.new_element(
    Rc::new(QualifiedName::new(None, None, "Top-Level"))
).expect("unable to create element node");

// Nodes are Rc-shared, so it is cheap to clone them.
// Now attach the element node to the tree.
// In this case, it is being attached to the document node, so it will become the root element.
doc.push(top.clone())
    .expect("unable to append child node");

// Now create a text node and attach it to the root element.
top.push(
    doc.new_text(Rc::new(Value::from("content of the element")))
        .expect("unable to create text node")
).expect("unable to append child node");

assert_eq!(doc.to_xml(), "<Top-Level>content of the element</Top-Level>")
*/

use crate::item::{Node as ItemNode, NodeType};
use crate::output::OutputDefinition;
use crate::parser::xml::qname::qualname_to_qname;
use crate::parser::{ParseError, ParserStateBuilder, StaticStateBuilder};
use crate::validators::{Schema, ValidationError};
use crate::value::Value;
use crate::xdmerror::*;
use crate::xmldecl::{DTD, XMLDecl, XMLDeclBuilder};
use qualname::{NamespacePrefix, NamespaceUri, NcName, QName};
use regex::Regex;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::btree_map::IntoIter;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::{Rc, Weak};

/// A node in a tree.
pub type RNode = Rc<Node>;

enum NodeInner {
    Document(
        RefCell<Option<XMLDecl>>,
        RefCell<Vec<RNode>>, // Child nodes
        RefCell<Vec<RNode>>, // Unattached nodes
        RefCell<Option<DTD>>,
    ), // to be well-formed, only one of the child nodes can be an element-type node
    Element(
        RefCell<Weak<Node>>,             // Parent: must be a Document or an Element
        QName,                           // name
        RefCell<BTreeMap<QName, RNode>>, // attributes
        RefCell<Vec<RNode>>,             // children
        Rc<RefCell<BTreeMap<Option<Rc<Value>>, RNode>>>, // namespace declarations
    ),
    Text(RefCell<Weak<Node>>, Rc<Value>),
    Attribute(RefCell<Weak<Node>>, QName, Rc<Value>),
    Comment(RefCell<Weak<Node>>, Rc<Value>),
    ProcessingInstruction(RefCell<Weak<Node>>, Rc<Value>, Rc<Value>),
    Namespace(
        RefCell<Weak<Node>>, // Parent
        Option<Rc<Value>>,   // Prefix
        Rc<Value>,           // URI
    ),
}
pub struct Node(NodeInner);

impl Node {
    /// Only documents are created new. All other types of nodes are created using new_* methods.
    fn new() -> Self {
        Node(NodeInner::Document(
            RefCell::new(None),
            RefCell::new(vec![]),
            RefCell::new(vec![]),
            None.into(),
        ))
    }
    /* pub fn set_nsuri(&mut self, uri: Rc<Value>) -> Result<(), Error> {
        match &self.0 {
            NodeInner::Element(p, qn, att, c, ns) => {
                qn.set_namespace_uri(uri.to_string());
                Ok(())
            }
            _ => Err(Error::new(
                ErrorKind::TypeError,
                String::from("not an Element node"),
            )),
        }
    }*/
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (NodeInner::Document(_, c, _, _), NodeInner::Document(_, d, _, _)) => {
                c.borrow()
                    .iter()
                    .zip(d.borrow().iter())
                    .fold(true, |mut acc, (c, d)| {
                        if acc {
                            acc = c == d;
                            acc
                        } else {
                            acc
                        }
                    })
                // TODO: use a method that terminates early on non-equality
            }
            (
                NodeInner::Element(_, name, atts, c, _),
                NodeInner::Element(_, o_name, o_atts, d, _),
            ) => {
                if name == o_name {
                    // Attributes must match
                    let b_atts = atts.borrow();
                    let b_o_atts = o_atts.borrow();
                    if b_atts.len() == b_o_atts.len() {
                        let mut at_names: Vec<QName> = b_atts.keys().cloned().collect();
                        at_names.sort();
                        if at_names.iter().fold(true, |mut acc, qn| {
                            if acc {
                                acc = b_atts.get(qn) == b_o_atts.get(qn);
                                acc
                            } else {
                                acc
                            }
                        }) {
                            // Content
                            c.borrow().iter().zip(d.borrow().iter()).fold(
                                true,
                                |mut acc, (c, d)| {
                                    if acc {
                                        acc = c == d;
                                        acc
                                    } else {
                                        acc
                                    }
                                },
                            )
                            // TODO: use a method that terminates early on non-equality
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                    // Content must match
                } else {
                    false
                }
            }
            (NodeInner::Text(_, v), NodeInner::Text(_, u)) => v == u,
            (NodeInner::Attribute(_, name, v), NodeInner::Attribute(_, o_name, o_v)) => {
                if name == o_name { v == o_v } else { false }
            }
            (
                NodeInner::ProcessingInstruction(_, name, v),
                NodeInner::ProcessingInstruction(_, o_name, o_v),
            ) => name == o_name && v == o_v,
            _ => false,
        }
    }
}

impl ItemNode for RNode {
    type NodeIterator = Box<dyn Iterator<Item = RNode>>;

    fn new_document() -> Self {
        Rc::new(Node::new())
    }

    fn node_type(&self) -> NodeType {
        match &self.0 {
            NodeInner::Document(_, _, _, _) => NodeType::Document,
            NodeInner::Element(_, _, _, _, _) => NodeType::Element,
            NodeInner::Attribute(_, _, _) => NodeType::Attribute,
            NodeInner::Text(_, _) => NodeType::Text,
            NodeInner::Comment(_, _) => NodeType::Comment,
            NodeInner::ProcessingInstruction(_, _, _) => NodeType::ProcessingInstruction,
            NodeInner::Namespace(_, _, _) => NodeType::Namespace,
        }
    }
    fn name(&self) -> Option<QName> {
        match &self.0 {
            NodeInner::Element(_, qn, _, _, _) | NodeInner::Attribute(_, qn, _) => Some(qn.clone()),
            NodeInner::ProcessingInstruction(_, nm, _) => {
                // A PI's target is a Name, which may not be a valid NcName
                // But it is also not a QName
                // Best we can do is treat it as an unprefixed name
                // If this fails then return None
                NcName::try_from(nm.to_string().as_str())
                    .map_or(None, |ncn| Some(QName::from_local_name(ncn)))
            }
            NodeInner::Namespace(_, p, _) => p.as_ref().map(|pf| {
                QName::from_local_name(
                    NcName::try_from(pf.to_string().as_str()).expect("not a valid QName"),
                )
            }),
            _ => None,
        }
    }
    fn to_qname(&self, name: impl AsRef<str>) -> Result<QName, Error> {
        // Parse the prefixed name
        // Use the namespace iterator to set up namespace declarations
        // First, make sure the supplied is valid
        let mut ss = StaticStateBuilder::new()
            .namespace(|prefix: &NamespacePrefix| {
                let nsdo = self.namespace_iter().find(|ns| {
                    // TODO: it's annoying to have to convert the namespace node name back to a prefix when we know it is a prefix
                    NamespacePrefix::try_from(ns.name().unwrap().local_name()).unwrap() == *prefix
                });
                nsdo.map_or(
                    Err(ParseError::MissingNameSpace),
                    // It's annoying to have to convert the namespace node value to a namespace URI when we already know it is a namespace URI
                    |nsd| Ok(NamespaceUri::try_from(nsd.value().to_string().as_str()).unwrap()),
                )
            })
            .build();
        let state = ParserStateBuilder::new().doc(self.owner_document()).build();
        match qualname_to_qname()((name.as_ref(), state), &mut ss) {
            Ok((_, qn)) => Ok(qn),
            Err(_) => Err(Error::new(
                ErrorKind::ParseError,
                "unable to resolve qualified name",
            )),
        }
    }
    fn value(&self) -> Rc<Value> {
        match &self.0 {
            NodeInner::Text(_, v)
            | NodeInner::Comment(_, v)
            | NodeInner::ProcessingInstruction(_, _, v)
            | NodeInner::Attribute(_, _, v) => v.clone(),
            NodeInner::Namespace(_, _, ns) => ns.clone(),
            _ => Rc::new(Value::from("")),
        }
    }

    fn get_id(&self) -> String {
        format!("{:#p}", &(self).0 as *const NodeInner)
    }

    fn to_string(&self) -> String {
        match &self.0 {
            NodeInner::Document(_, c, _, _) | NodeInner::Element(_, _, _, c, _) => {
                c.borrow().iter().fold(String::new(), |mut acc, n| {
                    acc.push_str(n.to_string().as_str());
                    acc
                })
            }
            NodeInner::Attribute(_, _, v)
            | NodeInner::Text(_, v)
            | NodeInner::Comment(_, v)
            | NodeInner::ProcessingInstruction(_, _, v) => v.to_string(),
            NodeInner::Namespace(_, _, uri) => uri.to_string(),
        }
    }
    fn to_xml(&self) -> String {
        to_xml_int(self, &OutputDefinition::new(), 0)
    }
    fn to_xml_with_options(&self, od: &OutputDefinition) -> std::string::String {
        to_xml_int(self, od, 0)
    }
    fn is_same(&self, other: &Self) -> bool {
        Rc::ptr_eq(self, other)
    }
    fn document_order(&self) -> Vec<usize> {
        doc_order(self)
    }
    // Find the document node, given an arbitrary node in the tree.
    // There is always a document node, so this will not panic.
    fn owner_document(&self) -> Self {
        match &self.0 {
            NodeInner::Document(_, _, _, _) => self.clone(),
            _ => self.ancestor_iter().last().unwrap(),
        }
    }
    fn cmp_document_order(&self, other: &Self) -> Ordering {
        let this_order = self.document_order();
        let other_order = other.document_order();
        let mut this_it = this_order.iter();
        let mut other_it = other_order.iter();
        for _i in 0.. {
            match (this_it.next(), other_it.next()) {
                (Some(t), Some(o)) => {
                    if t < o {
                        return Ordering::Less;
                    } else if t > o {
                        return Ordering::Greater;
                    }
                    // otherwise continue the loop
                }
                (Some(_), None) => return Ordering::Greater,
                (None, Some(_)) => return Ordering::Less,
                (None, None) => return Ordering::Equal,
            }
        }
        // Will never reach here
        Ordering::Equal
    }
    fn child_iter(&self) -> Self::NodeIterator {
        Box::new(Children::new(self))
    }
    fn ancestor_iter(&self) -> Self::NodeIterator {
        Box::new(Ancestors::new(self))
    }
    fn descend_iter(&self) -> Self::NodeIterator {
        Box::new(Descendants::new(self))
    }
    fn next_iter(&self) -> Self::NodeIterator {
        Box::new(Siblings::new(self, 1))
    }
    fn prev_iter(&self) -> Self::NodeIterator {
        Box::new(Siblings::new(self, -1))
    }
    fn attribute_iter(&self) -> Self::NodeIterator {
        Box::new(Attributes::new(self))
    }
    fn namespace_iter(&self) -> Self::NodeIterator {
        Box::new(NamespaceNodes::new(self.clone()))
    }
    fn get_attribute(&self, a: &QName) -> Rc<Value> {
        match &self.0 {
            NodeInner::Element(_, _, att, _, _) => att
                .borrow()
                .get(a)
                .map_or(Rc::new(Value::from(String::new())), |v| v.value()),
            _ => Rc::new(Value::from(String::new())),
        }
    }
    fn get_attribute_node(&self, a: &QName) -> Option<Self> {
        match &self.0 {
            NodeInner::Element(_, _, att, _, _) => att.borrow().get(a).cloned(),
            _ => None,
        }
    }
    fn new_element(&self, qn: QName) -> Result<Self, Error> {
        let child = Rc::new(Node(NodeInner::Element(
            RefCell::new(Rc::downgrade(&self.owner_document())),
            qn,
            RefCell::new(BTreeMap::new()),
            RefCell::new(vec![]),
            Rc::new(RefCell::new(BTreeMap::new())),
        )));
        unattached(self, child.clone());
        Ok(child)
    }
    fn new_namespace(&self, ns: Rc<Value>, prefix: Option<Rc<Value>>) -> Result<Self, Error> {
        let ns_node = Rc::new(Node(NodeInner::Namespace(
            RefCell::new(Rc::downgrade(&self.owner_document())),
            prefix,
            ns,
        )));
        unattached(self, ns_node.clone());
        Ok(ns_node)
    }
    fn new_text(&self, v: Rc<Value>) -> Result<Self, Error> {
        let child = Rc::new(Node(NodeInner::Text(
            RefCell::new(Rc::downgrade(&self.owner_document())),
            v,
        )));
        unattached(self, child.clone());
        Ok(child)
    }
    fn new_attribute(&self, qn: QName, v: Rc<Value>) -> Result<Self, Error> {
        //TODO if the attribute is xml:id then type needs to be set as ID, regardless of DTD.
        let att = Rc::new(Node(NodeInner::Attribute(
            RefCell::new(Rc::downgrade(self)),
            qn.clone(),
            v,
        )));
        unattached(self, att.clone());
        Ok(att)
    }
    fn new_comment(&self, v: Rc<Value>) -> Result<Self, Error> {
        let child = Rc::new(Node(NodeInner::Comment(
            RefCell::new(Rc::downgrade(&self.owner_document())),
            v,
        )));
        unattached(self, child.clone());
        Ok(child)
    }
    fn new_processing_instruction(&self, qn: Rc<Value>, v: Rc<Value>) -> Result<Self, Error> {
        let child = Rc::new(Node(NodeInner::ProcessingInstruction(
            RefCell::new(Rc::downgrade(&self.owner_document())),
            qn.clone(),
            v,
        )));
        unattached(self, child.clone());
        Ok(child)
    }
    // Append a node to the child list of the new parent.
    // Must first detach the node from its current position in the tree.
    fn push(&mut self, n: Self) -> Result<(), Error> {
        if n.node_type() == NodeType::Document
            || n.node_type() == NodeType::Attribute
            || n.node_type() == NodeType::Namespace
        {
            return Err(Error::new(
                ErrorKind::TypeError,
                String::from(
                    "document, namespace, or attribute type nodes cannot be inserted as a child",
                ),
            ));
        }

        let mut m = n.clone();
        m.pop()?;
        push_node(self, n)?;
        Ok(())
    }
    // Remove a node from the tree. If the node is unattached, then this has no effect.
    // The node is added to the unattached list of the owner document.
    fn pop(&mut self) -> Result<(), Error> {
        match &self.0 {
            NodeInner::Document(_, _, _, _) => {
                return Err(Error::new(
                    ErrorKind::TypeError,
                    String::from("cannot remove document node"),
                ));
            }
            NodeInner::Attribute(parent, qn, _) => {
                // Remove this node from the attribute hashmap
                let myp = Weak::upgrade(&parent.borrow()); // make borrow temporary
                match myp {
                    Some(p) => {
                        match &p.0 {
                            NodeInner::Element(_, _, att, _, _) => {
                                att.borrow_mut().remove(qn).ok_or(Error::new(
                                    ErrorKind::DynamicAbsent,
                                    String::from("unable to find attribute"),
                                ))?;
                                let doc = self.owner_document();
                                unattached(&doc, self.clone());
                            }
                            NodeInner::Document(_, _, _, _) => {} // attr was in the unattached list
                            _ => {
                                return Err(Error::new(
                                    ErrorKind::TypeError,
                                    String::from("parent is not an element"),
                                ));
                            }
                        }
                    }
                    None => {
                        return Err(Error::new(
                            ErrorKind::Unknown,
                            String::from("unable to find parent"),
                        ));
                    }
                }
            }
            NodeInner::Namespace(parent, prefix, _) => {
                // Remove this node from the attribute hashmap
                match Weak::upgrade(&parent.borrow()) {
                    Some(p) => {
                        match &p.0 {
                            NodeInner::Element(_, _, _, _, namespaces) => {
                                namespaces
                                    .borrow_mut()
                                    .remove_entry(prefix)
                                    .ok_or(Error::new(
                                        ErrorKind::DynamicAbsent,
                                        String::from("unable to find namespace"),
                                    ))?;
                                let doc = self.owner_document();
                                unattached(&doc, self.clone());
                            }
                            NodeInner::Document(_, _, _, _) => {} // attr was in the unattached list
                            _ => {
                                return Err(Error::new(
                                    ErrorKind::TypeError,
                                    String::from("parent is not an element"),
                                ));
                            }
                        }
                    }
                    None => {
                        return Err(Error::new(
                            ErrorKind::Unknown,
                            String::from("unable to find parent"),
                        ));
                    }
                }
            }
            NodeInner::Element(parent, _, _, _, _)
            | NodeInner::Text(parent, _)
            | NodeInner::Comment(parent, _)
            | NodeInner::ProcessingInstruction(parent, _, _) => {
                // Remove this node from the old parent's child list
                let p = if let Some(q) = Weak::upgrade(&parent.borrow()) {
                    q
                } else {
                    return Err(Error::new(
                        ErrorKind::Unknown,
                        String::from("unable to access parent"),
                    ));
                };
                match &p.0 {
                    NodeInner::Element(_, _, _, c, _) => {
                        let idx = find_index(&p, self)?;
                        c.borrow_mut().remove(idx);
                        let doc = self.owner_document();
                        unattached(&doc, self.clone())
                    }
                    NodeInner::Document(_, _, _, _) => {} // node was in the unattached list
                    _ => {
                        return Err(Error::new(
                            ErrorKind::TypeError,
                            String::from("parent is not an element"),
                        ));
                    }
                }
            }
        };
        Ok(())
    }
    fn add_attribute(&self, att: Self) -> Result<(), Error> {
        if att.node_type() != NodeType::Attribute {
            return Err(Error::new(
                ErrorKind::TypeError,
                String::from("node is not an attribute"),
            ));
        }

        match &self.0 {
            NodeInner::Element(_, _, patt, _, _) => {
                // Short-circuit: Is this attribute already attached to this element?
                if let Some(b) = patt.borrow().get(&self.name().unwrap()) {
                    if att.is_same(b) {
                        return Ok(());
                    }
                }
                // Firstly, make sure the node is removed from its old parent
                let mut m = att.clone();
                m.pop()?;
                // Popping will put the node in the unattached list,
                // so remove it from there
                detach(m.clone());
                // Now add to this parent
                // TODO: deal with same name being redefined
                if let NodeInner::Attribute(_, qn, _) = &m.0 {
                    let _ = patt.borrow_mut().insert(qn.clone(), m.clone());
                }
                make_parent(m, self.clone());
                Ok(())
            }
            _ => Err(Error::new(
                ErrorKind::TypeError,
                String::from("cannot add an attribute to this type of node"),
            )),
        }
    }
    /// Add a namespace to this element-type node.
    /// NOTE: does NOT update the namespace values of the element itself.
    // TODO: confirm what the behaviour of this should be.
    fn add_namespace(&self, ns: Self) -> Result<(), Error> {
        if ns.node_type() != NodeType::Namespace {
            return Err(Error::new(
                ErrorKind::TypeError,
                String::from("node is not a namespace"),
            ));
        }

        match &self.0 {
            NodeInner::Element(_, _, _, _, n) => {
                // Firstly, make sure the node is removed from its old parent
                let mut m = ns.clone();
                m.pop()?;
                // Popping will put the node in the unattached list,
                // so remove it from there
                detach(ns.clone());
                // Now add to this parent
                // TODO: deal with same name being redefined
                if let NodeInner::Namespace(_, alias, _) = &m.0 {
                    let _ = n.borrow_mut().insert(alias.clone(), ns.clone());
                }

                make_parent(ns, self.clone());
                Ok(())
            }
            _ => Err(Error::new(
                ErrorKind::TypeError,
                String::from("cannot add a namespace to this type of node"),
            )),
        }
    }
    fn insert_before(&mut self, n: Self) -> Result<(), Error> {
        if n.node_type() == NodeType::Document || n.node_type() == NodeType::Attribute {
            return Err(Error::new(
                ErrorKind::TypeError,
                String::from("cannot insert document or attribute node"),
            ));
        }

        // Detach from current location
        let mut m = n.clone();
        m.pop()?;
        detach(n.clone());
        // Now insert into parent's child list
        match &self.0 {
            NodeInner::Element(p, _, _, _, _)
            | NodeInner::Text(p, _)
            | NodeInner::Comment(p, _)
            | NodeInner::ProcessingInstruction(p, _, _) => {
                let parent = Weak::upgrade(&p.borrow()).unwrap();
                let idx = find_index(&parent, self)?;
                match &parent.0 {
                    NodeInner::Document(_, children, _, _)
                    | NodeInner::Element(_, _, _, children, _) => {
                        children.borrow_mut().insert(idx, n.clone());
                        make_parent(n, parent.clone())
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::TypeError,
                            String::from("parent is not an element"),
                        ));
                    }
                }
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::TypeError,
                    String::from("unable to find parent"),
                ));
            }
        }
        Ok(())
    }
    fn shallow_copy(&self) -> Result<Self, Error> {
        // All new nodes are parentless, i.e. they are unattached to the tree
        // The new element will have the same set of in-scope namespaces as the original element.
        match &self.0 {
            NodeInner::Document(x, _, _, _) => Ok(Rc::new(Node(NodeInner::Document(
                x.clone(),
                RefCell::new(vec![]),
                RefCell::new(vec![]),
                None.into(),
            )))),
            NodeInner::Element(p, qn, _, _, ns) => {
                let new = Rc::new(Node(NodeInner::Element(
                    p.clone(),
                    qn.clone(),
                    RefCell::new(BTreeMap::new()),
                    RefCell::new(vec![]),
                    ns.clone(),
                )));
                unattached(self, new.clone());
                Ok(new)
            }
            NodeInner::Attribute(p, qn, v) => Ok(Rc::new(Node(NodeInner::Attribute(
                p.clone(),
                qn.clone(),
                v.clone(),
            )))),
            NodeInner::Text(p, v) => {
                let new = Rc::new(Node(NodeInner::Text(p.clone(), v.clone())));
                unattached(&self.parent().unwrap(), new.clone());
                Ok(new)
            }
            NodeInner::Comment(p, v) => {
                let new = Rc::new(Node(NodeInner::Comment(p.clone(), v.clone())));
                unattached(&self.parent().unwrap(), new.clone());
                Ok(new)
            }
            NodeInner::ProcessingInstruction(p, qn, v) => {
                let new = Rc::new(Node(NodeInner::ProcessingInstruction(
                    p.clone(),
                    qn.clone(),
                    v.clone(),
                )));
                unattached(&self.parent().unwrap(), new.clone());
                Ok(new)
            }
            NodeInner::Namespace(p, pre, uri) => {
                let new = Rc::new(Node(NodeInner::Namespace(
                    p.clone(),
                    pre.clone(),
                    uri.clone(),
                )));
                unattached(&self.parent().unwrap(), new.clone());
                Ok(new)
            }
        }
    }
    fn deep_copy(&self) -> Result<Self, Error> {
        let mut new = self.shallow_copy()?;
        self.attribute_iter().try_for_each(|a| {
            new.add_attribute(a.deep_copy()?)?;
            Ok(())
        })?;
        self.child_iter().try_for_each(|c| {
            new.push(c.deep_copy()?)?;
            Ok(())
        })?;
        Ok(new)
    }
    // For special character escaping rules, see section 3.4.
    fn get_canonical(&self) -> Result<Self, Error> {
        match &self.0 {
            NodeInner::Document(_, e, _, _) => {
                let mut result = self.shallow_copy()?;
                for n in e.borrow_mut().iter() {
                    if let Ok(rn) = n.get_canonical() {
                        result.push(rn)?
                    }
                }
                Ok(result)
            }
            NodeInner::ProcessingInstruction(_, qn, v) => {
                let d = self.owner_document();
                let mut w = v.clone();
                if let Value::String(s) = (*v.clone()).clone() {
                    w = Rc::new(Value::String(
                        s.replace("&", "&amp;")
                            .replace("<", "&lt;")
                            .replace(">", "&gt;")
                            .replace("\r", "&#D;"),
                    ))
                }
                Ok(d.new_processing_instruction(qn.clone(), w)?)
            }
            NodeInner::Comment(_, _) | NodeInner::Namespace(_, _, _) => Err(Error::new(
                ErrorKind::TypeError,
                "invalid node type".to_string(),
            )),
            NodeInner::Text(_, v) => {
                let d = self.owner_document();
                let mut w = v.clone();
                if let Value::String(s) = (*v.clone()).clone() {
                    w = Rc::new(Value::String(
                        s.replace("&", "&amp;")
                            .replace("<", "&lt;")
                            .replace(">", "&gt;")
                            .replace("\r", "&#xD;"),
                    ))
                }
                Ok(d.new_text(w)?)
            }
            NodeInner::Attribute(_, qn, v) => {
                //self.shallow_copy()
                let d = self.owner_document();
                let w = v.to_string();
                Ok(d.new_attribute(
                    qn.clone(),
                    Rc::new(Value::String(
                        w.replace("&", "&amp;")
                            .replace("<", "&lt;")
                            .replace("\"", "&quot;")
                            .replace("\r", "&#xD;")
                            .replace("\t", "&#x9;")
                            .replace("\n", "&#xA;"),
                    )),
                )?)
            }
            NodeInner::Element(_, _, _, _, _) => {
                let mut result = self.shallow_copy()?;

                let d = result.owner_document();
                self.attribute_iter().try_for_each(|a| {
                    //Replace any number of spaces with a single space.
                    let re = Regex::new(r"\s+").unwrap();
                    result.add_attribute(
                        d.new_attribute(
                            a.name().unwrap(),
                            Rc::new(Value::String(
                                re.replace_all(a.clone().value().to_string().trim(), " ")
                                    .to_string(),
                            )),
                        )?,
                    )?;
                    //result.add_attribute(a.get_canonical()?)?;
                    Ok::<(), Error>(())
                })?;

                self.child_iter().try_for_each(|c| {
                    if let Ok(rn) = c.get_canonical() {
                        result.push(rn)?
                    }
                    Ok::<(), Error>(())
                })?;

                Ok(result)
            }
        }
    }
    fn set_xmldecl(&mut self, decl: XMLDecl) -> Result<(), Error> {
        match &self.0 {
            NodeInner::Document(x, _, _, _) => {
                *x.borrow_mut() = Some(decl);
                Ok(())
            }
            // TODO: traverse to the document node
            _ => Err(Error::new(
                ErrorKind::TypeError,
                String::from("not a Document node"),
            )),
        }
    }
    fn xmldecl(&self) -> XMLDecl {
        match &self.0 {
            NodeInner::Document(d, _, _, _) => d
                .borrow()
                .clone()
                .map_or_else(|| XMLDeclBuilder::new().build(), |x| x.clone()),
            _ => self.owner_document().xmldecl(),
        }
    }

    fn is_id(&self) -> bool {
        match &self.0 {
            //TODO Add Element XML ID support
            NodeInner::Attribute(_, _, v) => match v.as_ref() {
                Value::ID(_) => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn is_idrefs(&self) -> bool {
        match &self.0 {
            //TODO Add Element XML ID REF support
            NodeInner::Attribute(_, _, v) => match v.as_ref() {
                Value::IDREF(_) => true,
                Value::IDREFS(_) => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn get_dtd(&self) -> Option<DTD> {
        match &self.0 {
            NodeInner::Document(_, _, _, dtd) => dtd.borrow().clone(),
            _ => self.owner_document().get_dtd(),
        }
    }

    fn set_dtd(&self, dtd: DTD) -> Result<(), Error> {
        match &self.0 {
            NodeInner::Document(_, _, _, d) => {
                *d.borrow_mut() = Some(dtd);
                Ok(())
            }
            // TODO: traverse to the document node
            _ => Err(Error::new(
                ErrorKind::TypeError,
                String::from("not a Document node"),
            )),
        }
    }

    fn validate(&self, sch: Schema) -> Result<(), ValidationError> {
        crate::validators::validate(self, sch)
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.0 {
            NodeInner::Document(_, _, _, _) => write!(f, "document"),
            NodeInner::Element(_, qn, ats, _, _) => {
                let attrs = ats.borrow();
                write!(
                    f,
                    "element-type node \"{}\"@[{}]",
                    qn,
                    format_attrs(&attrs.clone())
                )
            }
            NodeInner::Attribute(_, qn, _) => {
                write!(f, "attribute-type node \"{}\"", qn)
            }
            NodeInner::Text(_, v) => write!(f, "text-type node \"{}\"", v),
            NodeInner::Comment(_, v) => write!(f, "comment-type node \"{}\"", v),
            NodeInner::ProcessingInstruction(_, qn, _) => {
                write!(f, "PI-type node \"{}\"", qn)
            }
            NodeInner::Namespace(_, pre, uri) => {
                write!(
                    f,
                    "namespace-type node \"{}:{}\"",
                    pre.clone().map_or("".to_string(), |v| v.to_string()),
                    uri
                )
            }
        }
    }
}

fn format_attrs(ats: &BTreeMap<QName, RNode>) -> String {
    let mut result = String::new();
    ats.iter()
        .for_each(|(k, v)| result.push_str(format!(" {}='{}'", k, v.to_string()).as_str()));
    result
}

// Put the given node in the unattached list for the document "d".
// This is for use when the node is newly created.
fn unattached(d: &RNode, n: RNode) {
    // Is it already in the unattached list? If so then do nothing
    match &d.0 {
        NodeInner::Document(_, _, u, _) => {
            if u.borrow().iter().any(|f| f.is_same(&n)) {
                return;
            }
            u.borrow_mut().push(n.clone());
            make_parent(n, d.clone())
        }
        NodeInner::Element(_, _, _, _, _) => {
            let doc = d.owner_document();
            if let NodeInner::Document(_, _, u, _) = &doc.0 {
                if u.borrow().iter().any(|f| f.is_same(&n)) {
                    return;
                }
                u.borrow_mut().push(n.clone());
                make_parent(n, doc.clone())
            } else {
                panic!("cannot find document node")
            }
        }
        _ => panic!("not a document node"),
    }
}
// Make the parent of the node be the given new parent
fn make_parent(n: RNode, b: RNode) {
    match &n.0 {
        NodeInner::Element(p, _, _, _, _)
        | NodeInner::Attribute(p, _, _)
        | NodeInner::Text(p, _)
        | NodeInner::Comment(p, _)
        | NodeInner::Namespace(p, _, _)
        | NodeInner::ProcessingInstruction(p, _, _) => *p.borrow_mut() = Rc::downgrade(&b),
        _ => panic!("unable to change parent"),
    }
}
// Remove an unattached node from the unattached list.
// This is in preparation for it being added to the tree.
fn detach(n: RNode) {
    match &n.0 {
        NodeInner::Element(p, _, _, _, _)
        | NodeInner::Attribute(p, _, _)
        | NodeInner::Text(p, _)
        | NodeInner::Comment(p, _)
        | NodeInner::Namespace(p, _, _)
        | NodeInner::ProcessingInstruction(p, _, _) => {
            let doc = Weak::upgrade(&p.borrow()).unwrap();
            match &doc.0 {
                NodeInner::Document(_, _, u, _) => {
                    let i = u.borrow().iter().position(|x| Rc::ptr_eq(x, &n));
                    if let Some(i) = i {
                        u.borrow_mut().remove(i);
                    }
                }
                _ => panic!("not a document"),
            }
        }
        _ => panic!("unable to change parent"),
    }
}

fn push_node(parent: &RNode, child: RNode) -> Result<(), Error> {
    if child.node_type() == NodeType::Attribute || child.node_type() == NodeType::Document {
        return Err(Error::new(
            ErrorKind::TypeError,
            String::from("cannot append an attribute or document node as a child node"),
        ));
    }
    match &parent.0 {
        NodeInner::Document(_, c, _, _) => {
            c.borrow_mut().push(child.clone());
        }
        NodeInner::Element(_, _, _, c, _) => {
            c.borrow_mut().push(child.clone());
        }
        _ => {
            return Err(Error::new(
                ErrorKind::TypeError,
                String::from("unable to add child node"),
            ));
        }
    }
    make_parent(child, parent.clone());
    Ok(())
}

// Find the document order of ancestors
fn doc_order(n: &RNode) -> Vec<usize> {
    match &n.0 {
        NodeInner::Document(_, _, _, _) => vec![1usize],
        NodeInner::Attribute(_, _, _) => {
            let mut a = doc_order(&n.parent().unwrap());
            a.push(2);
            a
        }
        NodeInner::Namespace(_, _, _) => {
            let mut a = doc_order(&n.parent().unwrap());
            a.push(2);
            a
        }
        NodeInner::Element(p, _, _, _, _)
        | NodeInner::Text(p, _)
        | NodeInner::Comment(p, _)
        | NodeInner::ProcessingInstruction(p, _, _) => match Weak::upgrade(&p.borrow()) {
            Some(q) => {
                let idx = find_index(&q, n).expect("unable to locate node in parent");
                let mut a = doc_order(&q);
                a.push(idx + 2);
                a
            }
            None => vec![1usize],
        },
    }
}

// Find the position of this node in the parent's child list.
fn find_index(parent: &RNode, child: &RNode) -> Result<usize, Error> {
    let idx = match &parent.0 {
        NodeInner::Document(_, c, _, _) | NodeInner::Element(_, _, _, c, _) => {
            c.borrow().iter().enumerate().fold(None, |mut acc, (i, v)| {
                if Rc::ptr_eq(child, v) {
                    acc = Some(i)
                    // TODO: stop here
                }
                acc
            })
        }
        _ => {
            return Err(Error::new(
                ErrorKind::TypeError,
                String::from("parent is not an element"),
            ));
        }
    };
    idx.ok_or(Error::new(
        ErrorKind::Unknown,
        std::string::String::from("unable to find child"),
    ))
}

/// Resolve the node's name (a [QName]) to a prefixed name.
/// If the QName has no Namespace URI then the returned string will be an unprefixed name.
/// Otherwise, the in-scope namespaces are used to find the prefix.
/// Nodes that don't have a name return an empty string.
fn to_prefixed_name(n: &RNode) -> String {
    match &n.0 {
        NodeInner::Element(_, qn, _, _, _) | NodeInner::Attribute(_, qn, _) => {
            let ns = qn.namespace_uri();
            if ns.is_none() {
                // Unprefixed name
                String::from(qn.local_name())
            } else {
                eprintln!(
                    "need prefix for {}, local-part {}",
                    qn.namespace_uri().unwrap().as_str(),
                    qn.local_name()
                );
                let uns = ns.unwrap();
                n.namespace_iter()
                    .inspect(|f| eprintln!("inspect: f \"{}\"", f.value().to_string()))
                    .find(|m| m.value().to_string() == uns.as_str())
                    .map_or_else(
                        || panic!("unable to find namespace"),
                        |p| {
                            format!(
                                "{}:{}",
                                String::from(p.name().unwrap().local_name()),
                                qn.local_name()
                            )
                        },
                    )
            }
        }
        _ => String::new(),
    }
}

// This handles the XML serialisation of the document.
// "indent" is the current level of indentation.
fn to_xml_int(node: &RNode, od: &OutputDefinition, indent: usize) -> String {
    match &node.0 {
        NodeInner::Document(_, _, _, _) => {
            node.child_iter().fold(String::new(), |mut result, c| {
                result.push_str(to_xml_int(&c, od, indent + 2).as_str());
                result
            })
        }
        NodeInner::Element(_, _qn, _, _, ns) => {
            let mut result = String::from("<");
            result.push_str(to_prefixed_name(node).as_str());

            // Namespace declarations
            ns.borrow().iter().for_each(|(pre, nsuri)| {
                let pre_str = pre.as_ref().map_or_else(
                    || format!(" xmlns='{}'", nsuri.to_string()),
                    |p| format!(" xmlns:{}='{}'", p, nsuri.to_string()),
                );
                result.push_str(pre_str.as_str());
            });

            // Attributes
            node.attribute_iter().for_each(|a| {
                result.push_str(format!(" {}='{}'", to_prefixed_name(&a), a.value()).as_str())
            });
            result.push('>');

            // Content of the element.
            // If the indent option is enabled, then if no child is a text node then add spacing.
            let do_indent: bool = od
                .get_indent()
                .then(|| {
                    node.child_iter().fold(true, |mut acc, c| {
                        if acc && c.node_type() == NodeType::Text {
                            acc = false
                        }
                        acc
                    })
                })
                .map_or(false, |b| b);

            node.child_iter().for_each(|c| {
                if do_indent {
                    result.push('\n');
                    (0..indent).for_each(|_| result.push(' '))
                }
                result.push_str(to_xml_int(&c, od, indent + 2).as_str())
            });
            if do_indent && indent > 1 {
                result.push('\n');
                (0..(indent - 2)).for_each(|_| result.push(' '))
            }
            result.push_str("</");
            result.push_str(to_prefixed_name(node).as_str());
            result.push('>');
            result
        }
        NodeInner::Text(_, v) => v.to_string(),
        NodeInner::Comment(_, v) => {
            let mut result = String::from("<!--");
            result.push_str(v.to_string().as_str());
            result.push_str("-->");
            result
        }
        NodeInner::ProcessingInstruction(_, qn, v) => {
            let mut result = String::from("<?");
            result.push_str(qn.to_string().as_str());
            result.push(' ');
            result.push_str(v.to_string().as_str());
            result.push_str("?>");
            result
        }
        _ => String::new(),
    }
}

pub struct Children {
    v: Vec<RNode>,
    i: usize,
}
impl Children {
    fn new(n: &RNode) -> Self {
        match &n.0 {
            NodeInner::Document(_, c, _, _) | NodeInner::Element(_, _, _, c, _) => Children {
                v: c.borrow().clone(),
                i: 0,
            },
            _ => Children { v: vec![], i: 0 },
        }
    }
}
impl Iterator for Children {
    type Item = RNode;

    fn next(&mut self) -> Option<RNode> {
        match self.v.get(self.i) {
            Some(c) => {
                self.i += 1;
                Some(c.clone())
            }
            None => None,
        }
    }
}

pub struct Ancestors {
    cur: RNode,
}

impl Ancestors {
    fn new(n: &RNode) -> Self {
        Ancestors { cur: n.clone() }
    }
}

impl Iterator for Ancestors {
    type Item = RNode;

    fn next(&mut self) -> Option<RNode> {
        let parent = match &self.cur.0 {
            NodeInner::Document(_, _, _, _) => None,
            NodeInner::Element(p, _, _, _, _)
            | NodeInner::Attribute(p, _, _)
            | NodeInner::Text(p, _)
            | NodeInner::Comment(p, _)
            | NodeInner::ProcessingInstruction(p, _, _)
            | NodeInner::Namespace(p, _, _) => Weak::upgrade(&p.borrow()),
        };
        parent.map(|q| {
            self.cur = q.clone();
            q
        })
    }
}

// This implementation eagerly constructs a list of nodes to traverse.
// A better approach would be to lazily traverse the descendants.
pub struct Descendants {
    v: Vec<RNode>,
    cur: usize,
}
impl Descendants {
    fn new(n: &RNode) -> Self {
        Descendants {
            v: n.child_iter().fold(vec![], |mut acc, c| {
                let mut d = descendant_add(&c);
                acc.append(&mut d);
                acc
            }),
            cur: 0,
        }
    }
}
fn descendant_add(n: &RNode) -> Vec<RNode> {
    let mut result = vec![n.clone()];
    n.child_iter().for_each(|c| {
        let mut l = descendant_add(&c);
        result.append(&mut l);
    });
    result
}
impl Iterator for Descendants {
    type Item = RNode;

    fn next(&mut self) -> Option<RNode> {
        match self.v.get(self.cur) {
            Some(n) => {
                self.cur += 1;
                Some(n.clone())
            }
            None => None,
        }
    }
}

// Store the parent node and the index of the child node that we want the sibling of.
// TODO: Don't Panic. If anything fails, then the iterator's next method should return None.
pub struct Siblings(RNode, usize, i32);
impl Siblings {
    fn new(n: &RNode, dir: i32) -> Self {
        match n.parent() {
            Some(p) => Siblings(
                p.clone(),
                find_index(&p, n).expect("unable to find node within parent"),
                dir,
            ),
            None => {
                // Document nodes don't have siblings
                Siblings(n.clone(), 0, -1)
            }
        }
    }
}
impl Iterator for Siblings {
    type Item = RNode;

    fn next(&mut self) -> Option<RNode> {
        if self.1 == 0 && self.2 < 0 {
            None
        } else {
            let newidx = if self.2 < 0 {
                self.1 - self.2.wrapping_abs() as usize
            } else {
                self.1 + self.2 as usize
            };
            if let NodeInner::Element(_, _, _, children, _) = &self.0.0 {
                match children.borrow().get(newidx) {
                    Some(n) => {
                        self.1 = newidx;
                        Some(n.clone())
                    }
                    None => None,
                }
            } else {
                None
            }
        }
    }
}

pub struct Attributes {
    it: Option<<BTreeMap<QName, RNode> as IntoIterator>::IntoIter>,
}
impl Attributes {
    fn new(n: &RNode) -> Self {
        if let NodeInner::Element(_, _, attributes, _, _) = &n.0 {
            let b = attributes.borrow();
            Attributes {
                it: Some(b.clone().into_iter()),
            }
        } else {
            // Other types of nodes don't have attributes, so always return None
            Attributes { it: None }
        }
    }
}
impl Iterator for Attributes {
    type Item = RNode;

    fn next(&mut self) -> Option<RNode> {
        self.it.as_mut().and_then(|i| i.next().map(|(_, n)| n))
    }
}

// Return the in-scope namespaces
// NB. Prefixed namespaces cannot be undeclared (XML Namespaces, 3rd Edition, section 5)
// TODO: handle undeclaring a default namespace. i.e. xmlns=""
pub struct NamespaceNodes {
    in_scope: Vec<Option<Rc<Value>>>, // namespaces that are already in scope, masking outer declarations
    cur_element: RNode,
    ancestor_it: Box<dyn Iterator<Item = RNode>>,
    ns_it: Option<IntoIter<Option<Rc<Value>>, RNode>>,
    xmlns: bool, // The undeclared, but always in-scope, "xml" namespace
}

impl NamespaceNodes {
    fn new(n: RNode) -> Self {
        match &n.0 {
            NodeInner::Element(_, _, _, _, ns) => {
                let nsit = ns.borrow().clone().into_iter();
                NamespaceNodes {
                    in_scope: vec![],
                    cur_element: n.clone(),
                    ancestor_it: n.clone().ancestor_iter(),
                    ns_it: Some(nsit),
                    xmlns: false,
                }
            }
            _ => NamespaceNodes {
                in_scope: vec![],
                cur_element: n.parent().unwrap(),
                ancestor_it: n.parent().unwrap().ancestor_iter(),
                ns_it: None,
                xmlns: false,
            },
        }
    }
}
impl Iterator for NamespaceNodes {
    type Item = RNode;

    fn next(&mut self) -> Option<RNode> {
        find_ns(self).or_else(|| {
            if self.xmlns {
                None
            } else {
                self.xmlns = true;
                Some(
                    self.cur_element
                        .owner_document()
                        .new_namespace(
                            Rc::new(Value::from("http://www.w3.org/XML/1998/namespace")),
                            Some(Rc::new(Value::from("xml"))),
                        )
                        .expect("unable to create namespace node"),
                )
            }
        })
    }
}
// Recursively ascend the ancestors looking for the first namespace node
fn find_ns(nn: &mut NamespaceNodes) -> Option<RNode> {
    if nn.cur_element.node_type() == NodeType::Element {
        if nn.ns_it.is_some() {
            // Iterating through the current element's namespace declarations
            let mut nsiter = nn.ns_it.take().unwrap();
            match nsiter.next() {
                Some((_, n)) => {
                    // Is there an inner scope?
                    eprintln!(
                        "find_ns: n type {:?} name {:?} value {:?}",
                        n.node_type(),
                        n.name(),
                        n.value()
                    );
                    // The default namespace should be represented by a None name.
                    // However, it might be represented by a prefix that is an empty string.
                    // Just in case, map an empty string to a None
                    let npo: Option<String> = if let Some(qn) = n.name() {
                        if qn.local_name().is_empty() {
                            None
                        } else {
                            Some(qn.local_name().to_string())
                        }
                    } else {
                        None
                    };
                    if let Some(_) = nn.in_scope.iter().find(|f| {
                        (f.is_none() && npo.is_none())
                            || f.as_ref().is_some_and(|g| {
                                g.to_string().as_str() == npo.as_ref().map_or("", |np| np.as_str())
                            })
                    }) {
                        // Yes, so don't include this outer scope declaration
                        nn.ns_it = Some(nsiter);
                        find_ns(nn)
                    } else {
                        // No, so this declaration is the inner scope
                        nn.in_scope.push(Some(Rc::new(Value::from(
                            npo.as_ref().map_or("", |np| np.as_str()),
                        ))));
                        nn.ns_it = Some(nsiter);
                        Some(n.clone())
                    }
                }
                None => {
                    // Move to the parent
                    nn.ns_it = None;
                    match nn.ancestor_it.next() {
                        Some(c) => {
                            nn.cur_element = c;
                            // nn.ns_it = None; take() has already done this
                            find_ns(nn)
                        }
                        None => None,
                    }
                }
            }
        } else {
            // Haven't looked at this element's namespaces yet
            if let NodeInner::Element(_, _, _, _, ns) = &nn.cur_element.0 {
                let mut nsiter = ns.borrow().clone().into_iter();
                match nsiter.next() {
                    Some((_, n)) => {
                        // Is there an inner scope?
                        let npo: Option<String> = if let Some(qn) = n.name() {
                            if qn.local_name().is_empty() {
                                None
                            } else {
                                Some(qn.local_name().to_string())
                            }
                        } else {
                            None
                        };
                        if let Some(_) = nn.in_scope.iter().find(|f| {
                            (f.is_none() && npo.is_none())
                                || f.as_ref().is_some_and(|g| {
                                    g.to_string().as_str()
                                        == npo.as_ref().map_or("", |np| np.as_str())
                                })
                        }) {
                            nn.ns_it = Some(nsiter);
                            find_ns(nn)
                        } else {
                            // No, so this declaration is the inner scope
                            nn.in_scope.push(Some(Rc::new(Value::from(
                                npo.as_ref().map_or("", |np| np.as_str()),
                            ))));
                            nn.ns_it = Some(nsiter);
                            Some(n.clone())
                        }
                    }
                    None => {
                        nn.ns_it = None;
                        match nn.ancestor_it.next() {
                            Some(b) => {
                                nn.cur_element = b;
                                find_ns(nn)
                            }
                            None => None,
                        }
                    }
                }
            } else {
                // can't happen
                None
            }
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xmldecl::XMLDeclBuilder;

    #[test]
    fn smite_new() {
        let _ = Node::new();
        assert!(true)
    }

    #[test]
    fn smite_xmldecl() {
        let mut d = Rc::new(Node::new());
        let x = XMLDeclBuilder::new().version(String::from("1.1")).build();
        d.set_xmldecl(x).expect("unable to set XML Declaration");
        assert!(true)
    }
    #[test]
    fn smite_element_1() {
        let mut root = Rc::new(Node::new());
        let c = root
            .new_element(QName::try_from("Test").expect("not a QName"))
            .expect("unable to create element node");
        root.push(c).expect("unable to add node");
        assert_eq!(root.to_xml(), "<Test></Test>")
    }
    #[test]
    fn smite_element_2() {
        let mut root = Rc::new(Node::new());
        let mut child1 = root
            .new_element(QName::try_from("Test").expect("not a QName"))
            .expect("unable to create element node");
        root.push(child1.clone()).expect("unable to add node");
        let child2 = child1
            .new_element(QName::try_from("MoreTest").expect("not a QName"))
            .expect("unable to create child element");
        child1.push(child2).expect("unable to add node");
        assert_eq!(root.to_xml(), "<Test><MoreTest></MoreTest></Test>")
    }

    #[test]
    fn smite_generate_id_1() {
        let mut root = Rc::new(Node::new());
        let mut child1 = root
            .new_element(QName::try_from("Test").expect("not a QName"))
            .expect("unable to create element node");
        root.push(child1.clone()).expect("unable to add node");
        let child2 = child1
            .new_element(QName::try_from("MoreTest").expect("not a QName"))
            .expect("unable to create child element");
        child1.push(child2.clone()).expect("unable to add node");
        assert_ne!(child1.get_id(), child2.get_id())
    }
}
