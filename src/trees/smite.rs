/*! # A tree structure for XDM

This module implements the Item module's [Node] trait.

This implementation uses interior mutability to create and manage a tree structure that is both mutable and fully navigable.

To create a tree, use [NodeBuilder] to make a Document-type node. To add a node, first create it using [NodeBuilder], then use a trait method to attach it to the tree.

NB. The Item module's Node trait is implemented for Rc\<intmuttree::Node\>. For convenience, this is defined as the type [RNode].

```rust
use std::rc::Rc;
use xrust::trees::intmuttree::{Document, NodeBuilder, RNode};
use xrust::item::{Node, NodeType};
use xrust::qname::QualifiedName;
use xrust::value::Value;
use xrust::xdmerror::Error;

pub(crate) type ExtDTDresolver = fn(Option<String>, String) -> Result<String, Error>;


// A document always has a NodeType::Document node as the toplevel node.
let mut doc = NodeBuilder::new(NodeType::Document).build();

let mut top = NodeBuilder::new(NodeType::Element)
    .name(QualifiedName::new(None, None, String::from("Top-Level")))
    .build();
// Nodes are Rc-shared, so it is cheap to clone them
doc.push(top.clone())
    .expect("unable to append child node");

top.push(
    NodeBuilder::new(NodeType::Text)
    .value(Rc::new(Value::from("content of the element")))
    .build()
).expect("unable to append child node");

assert_eq!(doc.to_xml(), "<Top-Level>content of the element</Top-Level>")
*/

use crate::item::{Node as ItemNode, NodeType};
use crate::output::OutputDefinition;
use crate::xmldecl::{XMLDecl, XMLDeclBuilder};
use crate::parser;
use crate::parser::xml::XMLDocument;
use crate::qname::QualifiedName;
use crate::value::Value;
use crate::xdmerror::*;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::{Rc, Weak};
use crate::trees::intmuttree::NodeBuilder;

/// A node in a tree.
pub type RNode = Rc<Node>;

enum NodeInner {
    Document(Option<XMLDecl>, RefCell<Vec<RNode>>), // only one of these can be an element-type node
    Element(
        RefCell<Weak<Node>>, // Parent: must be a Document or an Element
        Rc<QualifiedName>, // name
        RefCell<HashMap<Rc<QualifiedName>, RNode>>, // attributes
        RefCell<Vec<RNode>>, // children
    ),
    Text(RefCell<Weak<Node>>, Rc<Value>),
    Attribute(RefCell<Weak<Node>>, Rc<QualifiedName>, Rc<Value>),
    Comment(RefCell<Weak<Node>>, Rc<Value>),
    ProcessingInstruction(RefCell<Weak<Node>>, Rc<QualifiedName>, Rc<Value>),
}
pub struct Node(NodeInner);

impl Node {
    /// Only documents are created new. All other types of nodes are created using new_* methods.
    pub fn new() -> Self {
        Node(NodeInner::Document(None, RefCell::new(vec![])))
    }
    pub fn set_xmldecl(&mut self, decl: XMLDecl) -> Result<(), Error> {
        match &self.0 {
            NodeInner::Document(_, c) => {
                self.0 = NodeInner::Document(Some(decl), c.clone());
                Ok(())
            }
            _ => Err(Error::new(ErrorKind::TypeError, String::from("not a Document node"))),
        }
    }
    pub fn xmldecl(&self) -> Result<Option<XMLDecl>, Error> {
        match &self.0 {
            NodeInner::Document(d, _) => Ok(d.clone()),
            _ => Err(Error::new(ErrorKind::TypeError, String::from("not a Document node"))),
        }
    }
    pub fn set_nsuri(&mut self, uri: String) -> Result<(), Error>{
        match &self.0 {
            NodeInner::Element(p, qn, att, c) => {
                self.0 = NodeInner::Element(
                    p.clone(),
                    Rc::new(QualifiedName::new(Some(uri), qn.get_prefix(), qn.get_localname())),
                    att.clone(),
                    c.clone()
                );
                Ok(())
            }
            _ => Err(Error::new(ErrorKind::TypeError, String::from("not an Element node"))),
        }
    }
}


impl ItemNode for RNode {
    type NodeIterator = Box<dyn Iterator<Item = RNode>>;

    fn node_type(&self) -> NodeType {
        match &self.0 {
            NodeInner::Document(_, _) => NodeType::Document,
            NodeInner::Element(_, _, _, _) => NodeType::Element,
            NodeInner::Attribute(_, _, _) => NodeType::Attribute,
            NodeInner::Text(_, _) => NodeType::Text,
            NodeInner::Comment(_, _) => NodeType::Comment,
            NodeInner::ProcessingInstruction(_, _, _) => NodeType::ProcessingInstruction,
        }
    }
    fn name(&self) -> QualifiedName {
        match &self.0 {
            NodeInner::Element(_, qn, _, _) |
            NodeInner::ProcessingInstruction(_, qn, _) |
            NodeInner::Attribute(_, qn, _) => {
                let r: QualifiedName = (*qn.clone()).clone();
                r
            }
            _ => QualifiedName::new(None, None, String::from("")),
        }
    }
    fn value(&self) -> Rc<Value> {
        match &self.0 {
            NodeInner::Text(_, v) |
            NodeInner::Comment(_, v) |
            NodeInner::ProcessingInstruction(_, _, v) |
            NodeInner::Attribute(_, _, v) => v.clone(),
            _ => Rc::new(Value::from(String::from(""))),
        }
    }
    fn to_string(&self) -> String {
        match &self.0 {
            NodeInner::Document(_, c) |
            NodeInner::Element(_, _, _, c) => c.borrow().iter()
                .fold(String::new(), |mut acc, n| {acc.push_str(n.to_string().as_str()); acc}),
            NodeInner::Attribute(_, _, v) |
            NodeInner::Text(_, v) |
            NodeInner::Comment(_, v) |
            NodeInner::ProcessingInstruction(_, _, v) => v.to_string(),
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
            NodeInner::Document(_, _) => self.clone(),
            _ => self.ancestor_iter().last().unwrap()
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
            NodeInner::Element(_, _, att, _) => att.borrow().get(a)
                .map_or(Rc::new(Value::from(String::new())), |v| v.value()),
            _ => Rc::new(Value::from(String::new()))
        }
    }
    fn new_element(&self, qn: QualifiedName) -> Result<Self, Error> {
        let child = Rc::new(Node ( NodeInner::Element (RefCell::new(Rc::downgrade(self)), Rc::new(qn), RefCell::new(HashMap::new()), RefCell::new(vec![]))));
        push_node(self, child.clone())?;
        Ok(child)
    }
    fn new_text(&self, v: Rc<Value>) -> Result<Self, Error> {
        let child = Rc::new(Node ( NodeInner::Text (RefCell::new(Rc::downgrade(self)), v)));
        push_node(self, child.clone())?;
        Ok(child)
    }
    fn new_attribute(&self, qn: QualifiedName, v: Rc<Value>) -> Result<Self, Error> {
        let att = Rc::new(Node ( NodeInner::Attribute (RefCell::new(Rc::downgrade(self)), Rc::new(qn.clone()), v)));
        match &self.0 {
            NodeInner::Element(_, _, patt, _) => patt.borrow_mut().insert(Rc::new(qn), att.clone()),
            _ => return Err(Error::new(ErrorKind::TypeError, String::from("unable to add attribute node"))),
        };
        Ok(att)
    }
    fn new_comment(&self, v: Rc<Value>) -> Result<Self, Error> {
        let child = Rc::new(Node ( NodeInner::Comment (RefCell::new(Rc::downgrade(self)), v)));
        push_node(self, child.clone())?;
        Ok(child)
    }
    fn new_processing_instruction(&self, qn: QualifiedName, v: Rc<Value>) -> Result<Self, Error> {
        let child = Rc::new(Node ( NodeInner::ProcessingInstruction (RefCell::new(Rc::downgrade(self)), Rc::new(qn.clone()), v)));
        push_node(self, child.clone())?;
        Ok(child)
    }
    // Append a node to the child list of the new parent.
    // Must first detach the node from its current position in the tree.
    fn push(&mut self, n: Self) -> Result<(), Error> {
        let mut m = n.clone();
        m.pop()?;
        push_node(self, n)?;
        Ok(())
    }
    // Remove a node from the tree. If the node is unattached (i.e. ?), then this has no effect.
    // In this implementation, nodes always have a parent, so create a temporary tree. But where does the tmp tree go? i.e. who owns it?
    // Leave the parent field in the child unchanged, which is invalid. But the node is no longer accessible from the tree.
    fn pop(&mut self) -> Result<(), Error> {
        match &self.0 {
            NodeInner::Document(_, _) => return Err(Error::new(ErrorKind::TypeError, String::from("cannot remove document node"))),
            NodeInner::Attribute(parent, qn, _) => {
                // Remove this node from the attribute hashmap
                match Weak::upgrade(&parent.borrow()) {
                    Some(p) => {
                        match &p.0 {
                            NodeInner::Element(_, _, att, _) => att.borrow_mut().remove(qn).ok_or(Error::new(ErrorKind::DynamicAbsent, String::from("unable to find attribute")))?,
                            _ => return Err(Error::new(ErrorKind::TypeError, String::from("parent is not an element")))
                        }
                    }
                    None => return Err(Error::new(ErrorKind::Unknown, String::from("unable to find parent")))
                }
            }
            NodeInner::Element(parent, _, _, _) |
            NodeInner::Text(parent, _) |
            NodeInner::Comment(parent, _) |
            NodeInner::ProcessingInstruction(parent, _, _) => {
                // Remove this node from the old parent's child list
                match Weak::upgrade(&parent.borrow()) {
                    Some(p) => {
                        match &p.0 {
                            NodeInner::Element(_, _, _, c) => {
                                let idx = find_index(&p, self)?;
                                c.borrow_mut().remove(idx)
                            }
                            _ => return Err(Error::new(ErrorKind::TypeError, String::from("parent is not an element")))
                        }
                    }
                    None => return Err(Error::new(ErrorKind::Unknown, String::from("unable to find parent")))
                }
            }
        };
        Ok(())
    }
    fn add_attribute(&self, att: Self) -> Result<(), Error> {
        if att.node_type() != NodeType::Attribute { return Err(Error::new(ErrorKind::TypeError, String::from("node is not an attribute")))}

        match &self.0 {
            NodeInner::Element(_, _, patt, _) => {
                // Firstly, make sure the node is removed from its old parent
                let mut m = att.clone();
                m.pop()?;
                // Now add to this parent
                // TODO: deal with same name being redefined
                if let NodeInner::Attribute(_, qn, _) = &att.0 {
                    let _ = patt.borrow_mut().insert(qn.clone(), att);
                }
                Ok(())
            }
            _ => Err(Error::new(ErrorKind::TypeError, String::from("cannot add an attribute to this type of node"))),
        }
    }
    fn insert_before(&mut self, n: Self) -> Result<(), Error> {
        if n.node_type() == NodeType::Document || n.node_type() == NodeType::Attribute { return Err(Error::new(ErrorKind::TypeError, String::from("cannot insert document or attribute node")))}

        // Detach from current location
        let mut m = n.clone();
        m.pop()?;
        // Now insert into parent's child list
        match &self.0 {
            NodeInner::Element(p, _, _, _) |
            NodeInner::Text(p, _) |
            NodeInner::Comment(p, _) |
            NodeInner::ProcessingInstruction(p, _, _) => {
                let parent = Weak::upgrade(&p.borrow()).unwrap();
                let idx = find_index(&parent, self)?;
                match &parent.0 {
                    NodeInner::Document(_, children) |
                    NodeInner::Element(_, _, _, children) => {
                        children.borrow_mut().insert(idx, n)
                    }
                    _ => return Err(Error::new(ErrorKind::TypeError, String::from("parent is not an element")))
                }
            }
            _ => return Err(Error::new(ErrorKind::TypeError, String::from("unable to find parent")))
        }
        Ok(())
    }
    fn shallow_copy(&self) -> Result<Self, Error> {
        match &self.0 {
            NodeInner::Document(x, _) => Ok(Rc::new(Node(NodeInner::Document(x.clone(), RefCell::new(vec![]))))),
            NodeInner::Element(p, qn, _, _) => Ok(Rc::new(Node(NodeInner::Element(p.clone(), qn.clone(), RefCell::new(HashMap::new()), RefCell::new(vec![]))))),
            NodeInner::Attribute(p, qn, v) => Ok(Rc::new(Node(NodeInner::Attribute(p.clone(), qn.clone(), v.clone())))),
            NodeInner::Text(p, v) => Ok(Rc::new(Node(NodeInner::Text(p.clone(), v.clone())))),
            NodeInner::Comment(p, v) => Ok(Rc::new(Node(NodeInner::Comment(p.clone(), v.clone())))),
            NodeInner::ProcessingInstruction(p, qn, v) => Ok(Rc::new(Node(NodeInner::ProcessingInstruction(p.clone(), qn.clone(), v.clone())))),
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
            NodeInner::Document(_, _) |
            NodeInner::Comment(_, _) |
            NodeInner::ProcessingInstruction(_, _, _) => Err(Error::new(ErrorKind::TypeError, "invalid node type".to_string())),
            NodeInner::Text(_, v) => {
                let mut w = v.clone();
                if let Value::String(s) = (*v.clone()).clone() {
                    w = Rc::new(Value::String(s.replace("\r\n", "\n").replace("\n\n", "\n")))
                }
                Ok(self.new_text(w)?)
            }
            NodeInner::Attribute(_, _, _) => self.shallow_copy(),
            NodeInner::Element(_, _, _, _) => {
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
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.0 {
            NodeInner::Document(_, _) => write!(f, "document"),
            NodeInner::Element(_, qn, _, _) => write!(f, "element-type node \"{}\"", qn.to_string()),
            NodeInner::Attribute(_, qn, _) => write!(f, "attribute-type node \"{}\"", qn.to_string()),
            NodeInner::Text(_, v) => write!(f, "text-type node \"{}\"", v.to_string()),
            NodeInner::Comment(_, v) => write!(f, "comment-type node \"{}\"", v.to_string()),
            NodeInner::ProcessingInstruction(_, qn, _) => write!(f, "PI-type node \"{}\"", qn.to_string()),
        }
    }
}

fn push_node(parent: &RNode, child: RNode) -> Result<(), Error> {
    if child.node_type() == NodeType::Attribute || child.node_type() == NodeType::Document {
        return Err(Error::new(ErrorKind::TypeError, String::from("cannot append an attribute or document node as a child node")))
    }
    match &parent.0 {
        NodeInner::Document(_, c) => {
            c.borrow_mut().push(child);
        }
        NodeInner::Element(_, _, _, c) => {
            c.borrow_mut().push(child);
        }
        _ => return Err(Error::new(ErrorKind::TypeError, String::from("unable to add child node"))),
    }
    Ok(())
}

// Find the document order of ancestors
fn doc_order(n: &RNode) -> Vec<usize> {
    match &n.0 {
        NodeInner::Document(_, _) => vec![1 as usize],
        NodeInner::Attribute(_, _, _) => {
            let mut a = doc_order(&n.parent().unwrap());
            a.push(2);
            a
        }
        NodeInner::Element(p, _, _, _) |
        NodeInner::Text(p, _) |
        NodeInner::Comment(p, _) |
        NodeInner::ProcessingInstruction(p, _, _) => {
            match Weak::upgrade(&p.borrow()) {
                Some(q) => {
                    let idx = find_index(&q, n).expect("unable to locate node in parent");
                    let mut a = doc_order(&q);
                    a.push(idx + 2);
                    a
                }
                None => vec![1 as usize]
            }
        }
    }
}

// Find the position of this node in the parent's child list.
fn find_index(parent: &RNode, child: &RNode) -> Result<usize, Error> {
    let idx = match &parent.0 {
        NodeInner::Document(_, c) |
        NodeInner::Element(_, _, _, c) => {
            c.borrow().iter()
                .enumerate()
                .fold(None, |mut acc, (i, v)| {
                    if Rc::ptr_eq(child, v) {
                        acc = Some(i)
                        // TODO: stop here
                    }
                    acc
                })
        }
        _ => return Err(Error::new(ErrorKind::TypeError, String::from("parent is not an element")))
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
        NodeInner::Document(_, _) => node
            .child_iter()
            .fold(String::new(), |mut result, c| {
                result.push_str(to_xml_int(&c, od, ns.clone(), indent + 2).as_str());
                result
            }),
        NodeInner::Element(_, qn, _, _) => {
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

            node.attribute_iter()
                .for_each(|a| result.push_str(format!(" {}='{}'", a.name().to_string().as_str(), a.value()).as_str()));
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
            NodeInner::Document(_, c) |
            NodeInner::Element(_, _, _, c) => Children {
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
            NodeInner::Document(_, _) => None,
            NodeInner::Element(p, _, _, _) |
            NodeInner::Attribute(p, _, _) |
            NodeInner::Text(p, _) |
            NodeInner::Comment(p, _) |
            NodeInner::ProcessingInstruction(p, _, _) => {
                Weak::upgrade(&p.borrow())
            }
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
            Some(p) => {
                Siblings(p.clone(), find_index(&p, n).expect("unable to find node within parent"), dir)
            }
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
            if let NodeInner::Element(_, _, _, children) = &self.0.0 {
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
        if let NodeInner::Element(_, _, attributes, _) = &n.0 {
            let b = attributes.borrow();
            Attributes {
                it: Some(b.clone().into_iter()),
            }
        } else {
            // Other types of nodes don't have attributes, so always return None
            Attributes{ it: None }
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

    #[test]
    fn newnode_new() {
        let _ = Node::new();
        assert!(true)
    }

    #[test]
    fn newnode_xmldecl() {
        let mut d = Node::new();
        let x = XMLDeclBuilder::new()
            .version(String::from("1.1"))
            .build();
        d.set_xmldecl(x).expect("unable to set XML Declaration");
        assert!(true)
    }
    #[test]
    fn newnode_element_1() {
        let root = Rc::new(Node::new());
        root.new_element(QualifiedName::new(None, None, String::from("Test")))
            .expect("unable to create element node");
        assert_eq!(root.to_xml(), "<Test></Test>")
    }
    #[test]
    fn newnode_element_2() {
        let root = Rc::new(Node::new());
        let child1 = root.new_element(QualifiedName::new(None, None, String::from("Test")))
            .expect("unable to create element node");
        child1.new_element(QualifiedName::new(None, None, String::from("MoreTest")))
            .expect("unable to create child element");
        assert_eq!(root.to_xml(), "<Test><MoreTest></MoreTest></Test>")
    }

}
