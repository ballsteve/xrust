/*! # A tree structure for XDM

This module implements the Item module's [Node] trait.

This implementation uses interior mutability to create and manage a tree structure that is both mutable and fully navigable.

To create a tree, use [NodeBuilder] to make a Document-type node. To add a node, first create it using [NodeBuilder], then use a trait method to attach it to the tree.

NB. The Item module's Node trait is implemented for Rc\<intmuttree::Node\>. For convenience, this is defined as the type [RNode].

```rust
use std::rc::Rc;
use xrust::trees::smite::{Node as SmiteNode, RNode};
use xrust::item::{Node as ItemNode, NodeType};
use xrust::qname::QualifiedName;
use xrust::value::Value;
use xrust::xdmerror::Error;

pub(crate) type ExtDTDresolver = fn(Option<String>, String) -> Result<String, Error>;


// A document always has a NodeType::Document node as the toplevel node.
let mut doc = Rc::new(SmiteNode::new());

let mut top = doc.new_element(
    QualifiedName::new(None, None, String::from("Top-Level"))
).expect("unable to create element node");
// Nodes are Rc-shared, so it is cheap to clone them
doc.push(top.clone())
    .expect("unable to append child node");

top.push(
    doc.new_text(Rc::new(Value::from("content of the element")))
        .expect("unable to create text node")
).expect("unable to append child node");

assert_eq!(doc.to_xml(), "<Top-Level>content of the element</Top-Level>")
*/

use crate::item::{Node as ItemNode, NodeType};
use crate::output::OutputDefinition;
use crate::qname::QualifiedName;
use crate::value::Value;
use crate::xdmerror::*;
use crate::xmldecl::{XMLDecl, XMLDeclBuilder};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
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
    ), // to be well-formed, only one of the child nodes can be an element-type node
    Element(
        RefCell<Weak<Node>>, // Parent: must be a Document or an Element
        Rc<QualifiedName>,   // name
        RefCell<HashMap<Rc<QualifiedName>, RNode>>, // attributes
        RefCell<Vec<RNode>>, // children
        RefCell<HashMap<Option<String>, RNode>> // namespaces
    ),
    Text(RefCell<Weak<Node>>, Rc<Value>),
    Attribute(RefCell<Weak<Node>>, Rc<QualifiedName>, Rc<Value>),
    Comment(RefCell<Weak<Node>>, Rc<Value>),
    ProcessingInstruction(RefCell<Weak<Node>>, Rc<QualifiedName>, Rc<Value>),
    Namespace(RefCell<Weak<Node>>, //Parent
              Option<String>, //Prefix
              String) //URI
}
pub struct Node(NodeInner);

impl Node {
    /// Only documents are created new. All other types of nodes are created using new_* methods.
    pub fn new() -> Self {
        Node(NodeInner::Document(
            RefCell::new(None),
            RefCell::new(vec![]),
            RefCell::new(vec![]),
        ))
    }
    pub fn set_nsuri(&mut self, uri: String) -> Result<(), Error> {
        match &self.0 {
            NodeInner::Element(p, qn, att, c, ns) => {
                self.0 = NodeInner::Element(
                    p.clone(),
                    Rc::new(QualifiedName::new(
                        Some(uri),
                        qn.get_prefix(),
                        qn.get_localname(),
                    )),
                    att.clone(),
                    c.clone(),
                    ns.clone()
                );
                Ok(())
            }
            _ => Err(Error::new(
                ErrorKind::TypeError,
                String::from("not an Element node"),
            )),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (NodeInner::Document(_, c, _), NodeInner::Document(_, d, _)) => {
                c.borrow().iter().zip(d.borrow().iter())
                    .fold(true,|mut acc, (c, d)| if acc {acc = c == d; acc} else {acc})
                // TODO: use a method that terminates early on non-equality
            }
            (NodeInner::Element(_, name, atts, c, _), NodeInner::Element(_, o_name, o_atts, d, _)) => {
                if name == o_name {
                    // Attributes must match
                    let b_atts = atts.borrow();
                    let b_o_atts = o_atts.borrow();
                    if b_atts.len() == b_o_atts.len() {
                        let mut at_names: Vec<Rc<QualifiedName>> = b_atts.keys().cloned().collect();
                        at_names.sort();
                        if at_names.iter().fold(true, |mut acc, qn| {
                            if acc {
                                acc = b_atts.get(qn) == b_o_atts.get(qn);
                                acc
                            } else { acc }
                        }) {
                            // Content
                            c.borrow().iter()
                                .zip(d.borrow().iter())
                                .fold(true,|mut acc, (c, d)| if acc {acc = c == d; acc} else {acc})
                            // TODO: use a method that terminates early on non-equality
                        } else { false }
                    } else { false }
                    // Content must match
                } else { false }
            }
            (NodeInner::Text(_, v), NodeInner::Text(_, u)) => {
                v == u
            }
            (NodeInner::Attribute(_, name, v), NodeInner::Attribute(_, o_name, o_v)) => {
                if name == o_name {
                    v == o_v
                } else { false }
            }
            _ => { false }
        }
    }
}

impl ItemNode for RNode {
    type NodeIterator = Box<dyn Iterator<Item = RNode>>;

    fn node_type(&self) -> NodeType {
        match &self.0 {
            NodeInner::Document(_, _, _) => NodeType::Document,
            NodeInner::Element(_, _, _, _, _) => NodeType::Element,
            NodeInner::Attribute(_, _, _) => NodeType::Attribute,
            NodeInner::Text(_, _) => NodeType::Text,
            NodeInner::Comment(_, _) => NodeType::Comment,
            NodeInner::ProcessingInstruction(_, _, _) => NodeType::ProcessingInstruction,
            NodeInner::Namespace(_,_,_) => NodeType::Namespace
        }
    }
    fn name(&self) -> QualifiedName {
        match &self.0 {
            NodeInner::Element(_, qn, _, _, _)
            | NodeInner::ProcessingInstruction(_, qn, _)
            | NodeInner::Attribute(_, qn, _) => {
                let r: QualifiedName = (*qn.clone()).clone();
                r
            }
            _ => QualifiedName::new(None, None, String::from("")),
        }
    }
    fn value(&self) -> Rc<Value> {
        match &self.0 {
            NodeInner::Text(_, v)
            | NodeInner::Comment(_, v)
            | NodeInner::ProcessingInstruction(_, _, v)
            | NodeInner::Attribute(_, _, v) => v.clone(),
            _ => Rc::new(Value::from(String::from(""))),
        }
    }

    fn get_id(&self) -> String {
        format!("{:#p}", &(**self).0 as *const NodeInner)
    }

    fn to_string(&self) -> String {
        match &self.0 {
            NodeInner::Document(_, c, _) | NodeInner::Element(_, _, _, c, _) => {
                c.borrow().iter().fold(String::new(), |mut acc, n| {
                    acc.push_str(n.to_string().as_str());
                    acc
                })
            }
            NodeInner::Attribute(_, _, v)
            | NodeInner::Text(_, v)
            | NodeInner::Comment(_, v)
            | NodeInner::ProcessingInstruction(_, _, v) => v.to_string(),
            | NodeInner::Namespace(_,_,uri) => uri.to_string()

        }
    }
    fn to_xml(&self) -> String {
        to_xml_int(self, &OutputDefinition::new(), vec![], 0)
    }
    fn to_xml_with_options(&self, od: &OutputDefinition) -> std::string::String {
        to_xml_int(self, od, vec![], 0)
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
            NodeInner::Document(_, _, _) => self.clone(),
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
    fn get_attribute(&self, a: &QualifiedName) -> Rc<Value> {
        match &self.0 {
            NodeInner::Element(_, _, att, _, _) => att
                .borrow()
                .get(a)
                .map_or(Rc::new(Value::from(String::new())), |v| v.value()),
            _ => Rc::new(Value::from(String::new())),
        }
    }
    fn new_element(&self, qn: QualifiedName) -> Result<Self, Error> {
        let child = Rc::new(Node(NodeInner::Element(
            RefCell::new(Rc::downgrade(&self.owner_document())),
            Rc::new(qn),
            RefCell::new(HashMap::new()),
            RefCell::new(vec![]),
            RefCell::new(HashMap::new())
        )));
        unattached(self, child.clone());
        Ok(child)
    }
    fn new_text(&self, v: Rc<Value>) -> Result<Self, Error> {
        let child = Rc::new(Node(NodeInner::Text(
            RefCell::new(Rc::downgrade(&self.owner_document())),
            v,
        )));
        unattached(self, child.clone());
        Ok(child)
    }
    fn new_attribute(&self, qn: QualifiedName, v: Rc<Value>) -> Result<Self, Error> {
        let att = Rc::new(Node(NodeInner::Attribute(
            RefCell::new(Rc::downgrade(self)),
            Rc::new(qn.clone()),
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
    fn new_processing_instruction(&self, qn: QualifiedName, v: Rc<Value>) -> Result<Self, Error> {
        let child = Rc::new(Node(NodeInner::ProcessingInstruction(
            RefCell::new(Rc::downgrade(&self.owner_document())),
            Rc::new(qn.clone()),
            v,
        )));
        unattached(self, child.clone());
        Ok(child)
    }
    // Append a node to the child list of the new parent.
    // Must first detach the node from its current position in the tree.
    fn push(&mut self, n: Self) -> Result<(), Error> {
        if n.node_type() == NodeType::Document || n.node_type() == NodeType::Attribute || n.node_type() ==  NodeType::Namespace {
            return Err(Error::new(
                ErrorKind::TypeError,
                String::from("document, namespace or attribute type nodes cannot be inserted as a child"),
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
            NodeInner::Document(_, _, _) => {
                return Err(Error::new(
                    ErrorKind::TypeError,
                    String::from("cannot remove document node"),
                ))
            }
            NodeInner::Attribute(parent, qn, _) => {
                // Remove this node from the attribute hashmap
                match Weak::upgrade(&parent.borrow()) {
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
                            NodeInner::Document(_, _, _) => {} // attr was in the unattached list
                            _ => {
                                return Err(Error::new(
                                    ErrorKind::TypeError,
                                    String::from("parent is not an element"),
                                ))
                            }
                        }
                    }
                    None => {
                        return Err(Error::new(
                            ErrorKind::Unknown,
                            String::from("unable to find parent"),
                        ))
                    }
                }
            }
            NodeInner::Namespace(parent, prefix, _) => {
                // Remove this node from the attribute hashmap
                match Weak::upgrade(&parent.borrow()) {
                    Some(p) => {
                        match &p.0 {
                            NodeInner::Element(_, _, _, _, namespaces) => {
                                namespaces.borrow_mut().remove(prefix).ok_or(Error::new(
                                    ErrorKind::DynamicAbsent,
                                    String::from("unable to find namespace"),
                                ))?;
                                let doc = self.owner_document();
                                unattached(&doc, self.clone());
                            }
                            NodeInner::Document(_, _, _) => {} // attr was in the unattached list
                            _ => {
                                return Err(Error::new(
                                    ErrorKind::TypeError,
                                    String::from("parent is not an element"),
                                ))
                            }
                        }
                    }
                    None => {
                        return Err(Error::new(
                            ErrorKind::Unknown,
                            String::from("unable to find parent"),
                        ))
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
                    NodeInner::Document(_, _, _) => {} // node was in the unattached list
                    _ => {
                        return Err(Error::new(
                            ErrorKind::TypeError,
                            String::from("parent is not an element"),
                        ))
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
                // Firstly, make sure the node is removed from its old parent
                let mut m = att.clone();
                m.pop()?;
                // Popping will put the node in the unattached list,
                // so remove it from there
                detach(att.clone());
                // Now add to this parent
                // TODO: deal with same name being redefined
                if let NodeInner::Attribute(_, qn, _) = &att.0 {
                    let _ = patt.borrow_mut().insert(qn.clone(), att.clone());
                }
                make_parent(att, self.clone());
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
    //TODO confirm what the behaviour of this should be.
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
                if let NodeInner::Namespace(_, _, _) = &ns.0 {
                    let _ = n.borrow_mut().insert(ns.name().get_prefix(), ns.clone());
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
                    NodeInner::Document(_, children, _) | NodeInner::Element(_, _, _, children, _) => {
                        children.borrow_mut().insert(idx, n.clone());
                        make_parent(n, parent.clone())
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::TypeError,
                            String::from("parent is not an element"),
                        ))
                    }
                }
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::TypeError,
                    String::from("unable to find parent"),
                ))
            }
        }
        Ok(())
    }
    fn shallow_copy(&self) -> Result<Self, Error> {
        // All new nodes are parentless, i.e. they are unattached to the tree
        match &self.0 {
            NodeInner::Document(x, _, _) => Ok(Rc::new(Node(NodeInner::Document(
                x.clone(),
                RefCell::new(vec![]),
                RefCell::new(vec![]),
            )))),
            NodeInner::Element(p, qn, _, _, _) => {
                let new = Rc::new(Node(NodeInner::Element(
                    p.clone(),
                    qn.clone(),
                    RefCell::new(HashMap::new()),
                    RefCell::new(vec![]),
                    RefCell::new(HashMap::new()),
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
    fn get_canonical(&self) -> Result<Self, Error> {
        match &self.0 {
            NodeInner::Document(_, _, _)
            | NodeInner::Comment(_, _)
            | NodeInner::ProcessingInstruction(_, _, _)
            | NodeInner::Namespace(_,_,_) => Err(Error::new(
                ErrorKind::TypeError,
                "invalid node type".to_string(),
            )),
            NodeInner::Text(_, v) => {
                let mut w = v.clone();
                if let Value::String(s) = (*v.clone()).clone() {
                    w = Rc::new(Value::String(s.replace("\r\n", "\n").replace("\n\n", "\n")))
                }
                Ok(self.new_text(w)?)
            }
            NodeInner::Attribute(_, _, _) => self.shallow_copy(),
            NodeInner::Element(_, _, _, _, _) => {
                let mut result = self.shallow_copy()?;

                self.attribute_iter().try_for_each(|a| {
                    result.add_attribute(a.deep_copy()?)?;
                    Ok::<(), Error>(())
                })?;

                self.child_iter().try_for_each(|c| {
                    result.push(c.get_canonical()?)?;
                    Ok::<(), Error>(())
                })?;

                Ok(result)
            }
        }
    }
    fn set_xmldecl(&mut self, decl: XMLDecl) -> Result<(), Error> {
        match &self.0 {
            NodeInner::Document(x, _, _) => {
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
            NodeInner::Document(d, _, _) => d
                .borrow()
                .clone()
                .map_or_else(|| XMLDeclBuilder::new().build(), |x| x.clone()),
            _ => self.owner_document().xmldecl(),
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.0 {
            NodeInner::Document(_, _, _) => write!(f, "document"),
            NodeInner::Element(_, qn, ats, _, _) => {
                let attrs = ats.borrow();
                write!(
                    f,
                    "element-type node \"{}\"@[{}]",
                    qn.to_string(),
                    format_attrs(&attrs.clone())
                )
            }
            NodeInner::Attribute(_, qn, _) => {
                write!(f, "attribute-type node \"{}\"", qn.to_string())
            }
            NodeInner::Text(_, v) => write!(f, "text-type node \"{}\"", v.to_string()),
            NodeInner::Comment(_, v) => write!(f, "comment-type node \"{}\"", v.to_string()),
            NodeInner::ProcessingInstruction(_, qn, _) => {
                write!(f, "PI-type node \"{}\"", qn.to_string())
            }
            NodeInner::Namespace(_,pre,uri) => {
                write!(f, "namespace-type node \"{}:{}\"", pre.clone().unwrap_or("".to_string()), uri)
            }
        }
    }
}

fn format_attrs(ats: &HashMap<Rc<QualifiedName>, RNode>) -> String {
    let mut result = String::new();
    ats.iter().for_each(|(k, v)| {
        result.push_str(format!(" {}='{}'", k.to_string(), v.to_string()).as_str())
    });
    result
}

// Put the given node in the unattached list for the document "d".
// This is for use when the node is newly created.
fn unattached(d: &RNode, n: RNode) {
    match &d.0 {
        NodeInner::Document(_, _, u) => {
            u.borrow_mut().push(n.clone());
            make_parent(n, d.clone())
        }
        NodeInner::Element(_, _, _, _, _) => {
            let doc = d.owner_document();
            if let NodeInner::Document(_, _, u) = &doc.0 {
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
        | NodeInner::ProcessingInstruction(p, _, _) => {
            let doc = Weak::upgrade(&p.borrow()).unwrap();
            match &doc.0 {
                NodeInner::Document(_, _, u) => {
                    let i = u.borrow().iter().position(|x| Rc::ptr_eq(x, &n));
                    match i {
                        Some(i) => {
                            u.borrow_mut().remove(i);
                        }
                        None => {} // nothing to do. should this be an error?
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
        NodeInner::Document(_, c, _) => {
            c.borrow_mut().push(child.clone());
        }
        NodeInner::Element(_, _, _, c, _) => {
            c.borrow_mut().push(child.clone());
        }
        _ => {
            return Err(Error::new(
                ErrorKind::TypeError,
                String::from("unable to add child node"),
            ))
        }
    }
    make_parent(child, parent.clone());
    Ok(())
}

// Find the document order of ancestors
fn doc_order(n: &RNode) -> Vec<usize> {
    match &n.0 {
        NodeInner::Document(_, _, _) => vec![1 as usize],
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
            None => vec![1 as usize],
        },
    }
}

// Find the position of this node in the parent's child list.
fn find_index(parent: &RNode, child: &RNode) -> Result<usize, Error> {
    let idx = match &parent.0 {
        NodeInner::Document(_, c, _) | NodeInner::Element(_, _, _, c, _) => {
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
            ))
        }
    };
    idx.map_or(
        Err(Error::new(
            ErrorKind::Unknown,
            std::string::String::from("unable to find child"),
        )),
        Ok,
    )
}

// This handles the XML serialisation of the document.
// "ns" is the list of XML Namespaces that have been declared in an ancestor: (URI, prefix).
// "indent" is the current level of identation.
fn to_xml_int(
    node: &RNode,
    od: &OutputDefinition,
    ns: Vec<(String, Option<String>)>,
    indent: usize,
) -> String {
    match &node.0 {
        NodeInner::Document(_, _, _) => node.child_iter().fold(String::new(), |mut result, c| {
            result.push_str(to_xml_int(&c, od, ns.clone(), indent + 2).as_str());
            result
        }),
        NodeInner::Element(_, qn, _, _, _) => {
            let mut result = String::from("<");
            result.push_str(qn.to_string().as_str());

            // Check if any XML Namespaces need to be declared
            // newns is a vector of (prefix, namespace URI) pairs
            let mut declared = ns.clone();
            let mut newns: Vec<(String, Option<String>)> = vec![];
            // First, the element itself
            namespace_check(&qn, &declared).iter().for_each(|m| {
                newns.push(m.clone());
                declared.push(m.clone())
            });
            // Next, it's attributes
            node.attribute_iter().for_each(|a| {
                namespace_check(&a.name(), &declared).iter().for_each(|m| {
                    newns.push(m.clone());
                    declared.push(m.clone())
                })
            });
            // Finally, it's child elements
            node.child_iter()
                .filter(|c| c.node_type() == NodeType::Element)
                .for_each(|c| {
                    namespace_check(&c.name(), &declared).iter().for_each(|m| {
                        newns.push(m.clone());
                        declared.push(m.clone())
                    })
                });
            newns.iter().for_each(|(u, p)| {
                result.push_str(" xmlns");
                if let Some(q) = p {
                    result.push(':');
                    result.push_str(q.as_str());
                }
                result.push_str("='");
                result.push_str(u);
                result.push('\'');
            });

            node.attribute_iter().for_each(|a| {
                result.push_str(
                    format!(" {}='{}'", a.name().to_string().as_str(), a.value()).as_str(),
                )
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
                result.push_str(to_xml_int(&c, od, newns.clone(), indent + 2).as_str())
            });
            if do_indent && indent > 1 {
                result.push('\n');
                (0..(indent - 2)).for_each(|_| result.push(' '))
            }
            result.push_str("</");
            result.push_str(qn.to_string().as_str());
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

// Checks if this node's name is in a namespace that has already been declared.
// Returns a namespace to be declared if required, (URI, prefix).
fn namespace_check(
    qn: &QualifiedName,
    ns: &Vec<(String, Option<String>)>,
) -> Option<(String, Option<String>)> {
    let mut result = None;
    if let Some(qnuri) = qn.get_nsuri_ref() {
        // Has this namespace already been declared?
        if ns.iter().find(|(u, _)| u == qnuri).is_some() {
            // Namespace has been declared, but with the same prefix?
            // TODO: see forest.rs for example implementation
        } else {
            // Namespace has not been declared, so this element must declare it
            result = Some((qnuri.to_string(), qn.get_prefix()))
        }
    }
    result
}

pub struct Children {
    v: Vec<RNode>,
    i: usize,
}
impl Children {
    fn new(n: &RNode) -> Self {
        match &n.0 {
            NodeInner::Document(_, c, _) | NodeInner::Element(_, _, _, c, _) => Children {
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
            NodeInner::Document(_, _, _) => None,
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
            if let NodeInner::Element(_, _, _, children, _) = &self.0 .0 {
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
    it: Option<IntoIter<Rc<QualifiedName>, RNode>>,
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
        self.it.as_mut().map_or(None, |i| i.next().map(|(_, n)| n))
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
            .new_element(QualifiedName::new(None, None, String::from("Test")))
            .expect("unable to create element node");
        root.push(c).expect("unable to add node");
        assert_eq!(root.to_xml(), "<Test></Test>")
    }
    #[test]
    fn smite_element_2() {
        let mut root = Rc::new(Node::new());
        let mut child1 = root
            .new_element(QualifiedName::new(None, None, String::from("Test")))
            .expect("unable to create element node");
        root.push(child1.clone()).expect("unable to add node");
        let child2 = child1
            .new_element(QualifiedName::new(None, None, String::from("MoreTest")))
            .expect("unable to create child element");
        child1.push(child2).expect("unable to add node");
        assert_eq!(root.to_xml(), "<Test><MoreTest></MoreTest></Test>")
    }

    #[test]
    fn smite_generate_id_1() {
        let mut root = Rc::new(Node::new());
        let mut child1 = root
            .new_element(QualifiedName::new(None, None, String::from("Test")))
            .expect("unable to create element node");
        root.push(child1.clone()).expect("unable to add node");
        let child2 = child1
            .new_element(QualifiedName::new(None, None, String::from("MoreTest")))
            .expect("unable to create child element");
        child1.push(child2.clone()).expect("unable to add node");
        assert_ne!(child1.get_id(), child2.get_id())
    }
}
