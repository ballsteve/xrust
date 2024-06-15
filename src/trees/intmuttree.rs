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

use crate::externals::URLResolver;
use crate::item::{Node as ItemNode, NodeType};
use crate::output::OutputDefinition;
use crate::parser::xml::parse;
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

//pub(crate) type ExtDTDresolver = fn(Option<String>, String) -> Result<String, Error>;

/// An XML document.
#[derive(Clone, Default)]
pub struct Document {
    pub xmldecl: Option<XMLDecl>,
    pub prologue: Vec<RNode>,
    pub content: Vec<RNode>,
    pub epilogue: Vec<RNode>,
}

impl Document {
    fn new() -> Self {
        Document {
            ..Default::default()
        }
    }
    pub fn set_xmldecl(&mut self, x: XMLDecl) {
        self.xmldecl = Some(x)
    }
    pub fn get_xmldecl(&self) -> &Option<XMLDecl> {
        &self.xmldecl
    }

    pub fn push_content(&mut self, n: RNode) {
        self.content.push(n)
    }

    pub fn to_xml(&self) -> String {
        // TODO: XML Decl, prologue, epilogue
        let mut result = String::new();
        self.content
            .iter()
            .for_each(|c| result.push_str(c.to_xml().as_str()));
        result
    }
    pub fn canonical(self) -> Document {
        let d = match self.xmldecl {
            None => XMLDecl {
                version: "1.0".to_string(),
                encoding: Some("UTF-8".to_string()),
                standalone: None,
            },
            Some(x) => XMLDecl {
                version: x.version,
                encoding: Some("UTF-8".to_string()),
                standalone: None,
            },
        };
        let mut p = vec![];
        for pn in self.prologue {
            if let Ok(pcn) = pn.get_canonical() {
                p.push(pcn)
            }
        }
        let mut c = vec![];
        for cn in self.content {
            if let Ok(ccn) = cn.get_canonical() {
                c.push(ccn)
            }
        }
        let mut e = vec![];
        for en in self.epilogue {
            if let Ok(ecn) = en.get_canonical() {
                e.push(ecn)
            }
        }

        Document {
            xmldecl: Some(d),
            prologue: p,
            content: c,
            epilogue: e,
        }
    }
    /*
    /// Expand the general entities in the document content.
    pub fn expand(&self) -> Result<(), Error> {
        let mut ent: HashMap<QualifiedName, Vec<RNode>> = HashMap::new();

        // Process general entity declarations and store the result in the HashMap.
        for p in &self.prologue {
            if p.node_type() == NodeType::Unknown {
                let DTDDecl::GeneralEntity(n, c) = p.dtd.as_ref().unwrap();
                let (rest, e) = content(c.as_str())
                    .map_err(|e| Error::new(ErrorKind::Unknown, e.to_string()))?;
                if rest.len() != 0 {
                    return Result::Err(Error::new(
                        ErrorKind::Unknown,
                        format!("unable to parse general entity \"{}\"", n.to_string()),
                    ));
                }
                match ent.insert(n.clone(), e) {
                    Some(_) => {
                        return Result::Err(Error::new(
                            ErrorKind::Unknown,
                            format!("general entity \"{}\" already defined", n.to_string()),
                        ))
                    }
                    None => {}
                }
            }
        }
        // Descend the tree, replacing reference nodes with their content
        // TODO: Don't Panic
        self.content
            .iter()
            .for_each(|c| expand_node(c.clone(), &ent).expect("unable to expand node"));

        Ok(())
    }

     */
}

impl TryFrom<(String, Option<URLResolver>, Option<String>)> for Document {
    type Error = Error;
    fn try_from(s: (String, Option<URLResolver>, Option<String>)) -> Result<Self, Self::Error> {
        let doc = NodeBuilder::new(NodeType::Document).build();
        parse(doc.clone(), s.0.as_str(), s.1, s.2)?;
        let result = DocumentBuilder::new().content(vec![doc]).build();
        Ok(result)
    }
}
impl TryFrom<(&str, Option<URLResolver>, Option<String>)> for Document {
    type Error = Error;
    fn try_from(s: (&str, Option<URLResolver>, Option<String>)) -> Result<Self, Self::Error> {
        let doc = NodeBuilder::new(NodeType::Document).build();
        parse(doc.clone(), s.0, s.1, s.2)?;
        let result = DocumentBuilder::new().content(vec![doc]).build();
        Ok(result)
    }
}

impl PartialEq for Document {
    fn eq(&self, other: &Document) -> bool {
        self.xmldecl == other.xmldecl
            && self
                .content
                .iter()
                .zip(other.content.iter())
                .fold(true, |acc, (a, b)| acc && a == b)
    }
}

impl PartialEq for Node {
    // TODO: attributes
    fn eq(&self, other: &Node) -> bool {
        self.node_type == other.node_type
            && self.name == other.name
            && self.value == other.value
            && self
                .children
                .borrow()
                .iter()
                .zip(other.children.borrow().iter())
                .fold(true, |acc, (a, b)| acc && a == b)
    }
}

/*
fn expand_node(mut n: RNode, ent: &HashMap<QualifiedName, Vec<RNode>>) -> Result<(), Error> {
    // TODO: Don't Panic
    match n.node_type() {
        NodeType::Reference => ent
            .get(&n.name())
            .map(|d| {
                for e in d {
                    n.insert_before(e.clone()).expect("unable to insert node")
                }
                n.pop().expect("unable to remove node")
            })
            .ok_or(Error::new(
                ErrorKind::Unknown,
                String::from("reference to unknown entity"),
            )),
        _ => Ok(()),
    }
}
 */

pub struct DocumentBuilder(Document);

impl Default for DocumentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentBuilder {
    pub fn new() -> Self {
        DocumentBuilder(Document::new())
    }
    pub fn xmldecl(mut self, x: XMLDecl) -> Self {
        self.0.xmldecl = Some(x);
        self
    }
    pub fn prologue(mut self, p: Vec<RNode>) -> Self {
        self.0.prologue = p;
        self
    }
    pub fn content(mut self, p: Vec<RNode>) -> Self {
        self.0.content = p;
        self
    }
    pub fn epilogue(mut self, p: Vec<RNode>) -> Self {
        self.0.epilogue = p;
        self
    }
    pub fn build(self) -> Document {
        self.0
    }
}

/// A node in a tree.
#[derive(Clone, Default)]
pub struct Node {
    node_type: NodeType,
    parent: RefCell<Option<Weak<Node>>>,
    children: RefCell<Vec<RNode>>,
    attributes: RefCell<HashMap<QualifiedName, RNode>>,
    // name is mutable only so that the namespace URI can be set once the document is parsed.
    // If we can build a better parser then the RefCell can be removed.
    name: RefCell<Option<QualifiedName>>,
    value: Option<Rc<Value>>,
    pi_name: Option<String>,
    dtd: Option<DTD>,
    reference: Option<QualifiedName>,
}

impl Node {
    /// Create an empty, unattached node
    pub fn new(n: NodeType) -> Self {
        Node {
            node_type: n,
            parent: RefCell::new(None),
            children: RefCell::new(vec![]),
            attributes: RefCell::new(HashMap::new()),
            ..Default::default()
        }
    }
    pub fn pi_name(&self) -> Option<String> {
        self.pi_name.clone()
    }
    pub fn reference(&self) -> Option<QualifiedName> {
        self.reference.clone()
    }
    pub fn set_nsuri(&self, uri: String) {
        let new = match &*self.name.borrow() {
            Some(old) => QualifiedName::new(Some(uri), old.get_prefix(), old.get_localname()),
            None => panic!("no node name"),
        };
        let _ = self.name.borrow_mut().insert(new);
    }
}

pub type RNode = Rc<Node>;

impl ItemNode for RNode {
    type NodeIterator = Box<dyn Iterator<Item = RNode>>;

    fn node_type(&self) -> NodeType {
        self.node_type
    }
    fn name(&self) -> QualifiedName {
        self.name
            .borrow()
            .as_ref()
            .map_or(QualifiedName::new(None, None, String::new()), |n| n.clone())
    }
    fn value(&self) -> Rc<Value> {
        self.value
            .as_ref()
            .map_or(Rc::new(Value::from("")), |v| v.clone())
    }

    fn get_id(&self) -> String {
        format!("{:p}", &**self as *const Node)
    }

    fn to_string(&self) -> String {
        match self.node_type() {
            NodeType::Document | NodeType::Element => self
                .descend_iter()
                .filter(|c| c.node_type() == NodeType::Text)
                .fold(String::new(), |mut acc, c| {
                    acc.push_str(c.to_string().as_str());
                    acc
                }),
            NodeType::Text
            | NodeType::Attribute
            | NodeType::Comment
            | NodeType::ProcessingInstruction => self.value().to_string(),
            _ => String::new(),
        }
    }
    /// Serialise as XML
    fn to_xml(&self) -> String {
        to_xml_int(self, &OutputDefinition::new(), vec![], 0)
    }
    /// Serialise the node as XML, with options such as indentation.
    fn to_xml_with_options(&self, od: &OutputDefinition) -> String {
        to_xml_int(self, od, vec![], 0)
    }

    fn is_same(&self, other: &Self) -> bool {
        Rc::ptr_eq(self, other)
    }
    fn document_order(&self) -> Vec<usize> {
        doc_order(self)
    }
    fn cmp_document_order(&self, other: &Self) -> Ordering {
        let this_order = self.document_order();
        let other_order = other.document_order();
        // zip the two iterators and compare usizes
        //        let mut it = this_order.iter().zip(other_order.iter());
        // Implementation note: fold seems to be consuming all of the items, so try an explicit loop instead
        //        let m = (&mut it).fold(
        //            Ordering::Equal,
        //            |acc, (t, o)| {
        //                if acc == Ordering::Equal { t.cmp(o) } else { acc }
        //            }
        //        );
        // and then unzip and compare the remaining vectors, at least one of which should be empty
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
    fn owner_document(&self) -> Self {
        if self.node_type() == NodeType::Document {
            self.clone()
        } else {
            self.ancestor_iter().last().unwrap()
        }
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
        self.attributes
            .borrow()
            .get(a)
            .map_or(Rc::new(Value::from("")), |v| {
                v.value.as_ref().unwrap().clone()
            })
    }

    fn new_element(&self, qn: QualifiedName) -> Result<Self, Error> {
        Ok(NodeBuilder::new(NodeType::Element).name(qn).build())
    }
    fn new_text(&self, v: Rc<Value>) -> Result<Self, Error> {
        Ok(NodeBuilder::new(NodeType::Text).value(v).build())
    }
    fn new_attribute(&self, qn: QualifiedName, v: Rc<Value>) -> Result<Self, Error> {
        Ok(NodeBuilder::new(NodeType::Attribute)
            .name(qn)
            .value(v)
            .build())
    }
    fn new_comment(&self, v: Rc<Value>) -> Result<Self, Error> {
        Ok(NodeBuilder::new(NodeType::Comment).value(v).build())
    }
    fn new_processing_instruction(&self, qn: QualifiedName, v: Rc<Value>) -> Result<Self, Error> {
        Ok(NodeBuilder::new(NodeType::ProcessingInstruction)
            .name(qn)
            .value(v)
            .build())
    }

    /// Append a node to the child list
    fn push(&mut self, n: RNode) -> Result<(), Error> {
        if n.node_type() == NodeType::Document {
            return Err(Error::new(
                ErrorKind::TypeError,
                String::from("document type nodes cannot be inserted into a tree"),
            ));
        }
        *n.parent.borrow_mut() = Some(Rc::downgrade(self));
        self.children.borrow_mut().push(n);
        Ok(())
    }
    /// Remove a node from the tree. If the node is unattached (i.e. does not have a parent), then this has no effect.
    fn pop(&mut self) -> Result<(), Error> {
        // Find this node in the parent's node list
        let parent = self.parent().ok_or_else(|| {
            Error::new(
                ErrorKind::Unknown,
                String::from("unable to insert before: node is an orphan"),
            )
        })?;
        let idx = find_index(&parent, self)?;
        parent.children.borrow_mut().remove(idx);
        Ok(())
    }
    /// Add an attribute to this element-type node
    fn add_attribute(&self, att: Self) -> Result<(), Error> {
        if self.node_type() != NodeType::Element {
            return Result::Err(Error::new(
                ErrorKind::Unknown,
                String::from("must be an element node"),
            ));
        }
        if att.node_type() != NodeType::Attribute {
            return Result::Err(Error::new(
                ErrorKind::Unknown,
                String::from("must be an attribute node"),
            ));
        }
        self.attributes.borrow_mut().insert(att.name(), att.clone());
        *att.parent.borrow_mut() = Some(Rc::downgrade(self));
        Ok(())
    }
    /// Insert a node into the child list immediately before this node.
    fn insert_before(&mut self, mut insert: Self) -> Result<(), Error> {
        if insert.node_type() == NodeType::Document {
            return Err(Error::new(
                ErrorKind::TypeError,
                String::from("document type nodes cannot be inserted into a tree"),
            ));
        }

        // Detach the node first. Ignore any error, it's OK if the node is not attached anywhere.
        _ = insert.pop();

        // Get the parent of this node. It is an error if there is no parent.
        let parent = self.parent().ok_or_else(|| {
            Error::new(
                ErrorKind::Unknown,
                String::from("unable to insert before: node is an orphan"),
            )
        })?;

        // Find the child node's index in the parent's child list
        let idx = find_index(&parent, self)?;
        // Insert the node at position of self, shifting insert right
        parent.children.borrow_mut().insert(idx, insert);
        // All done
        Ok(())
    }

    /// Shallow copy the node. Returned node is unattached.
    fn shallow_copy(&self) -> Result<Self, Error> {
        Ok(NodeBuilder::new(self.node_type())
            .name(self.name())
            .value(self.value())
            .build())
    }

    /// Deep copy the node. Returned node is unattached.
    fn deep_copy(&self) -> Result<Self, Error> {
        let mut result = NodeBuilder::new(self.node_type())
            .name(self.name())
            .value(self.value())
            .build();

        self.attribute_iter().try_for_each(|a| {
            result.add_attribute(a.deep_copy()?)?;
            Ok::<(), Error>(())
        })?;

        self.child_iter().try_for_each(|c| {
            result.push(c.deep_copy()?)?;
            Ok::<(), Error>(())
        })?;

        Ok(result)
    }

    fn get_canonical(&self) -> Result<Self, Error> {
        match self.node_type() {
            NodeType::Comment => Err(Error::new(ErrorKind::TypeError, "".to_string())),
            NodeType::Text => {
                let mut v: Rc<Value> = self.value();
                if let Value::String(s) = &*v {
                    v = Rc::new(Value::String(s.replace("\r\n", "\n").replace("\n\n", "\n")))
                }
                let result = NodeBuilder::new(self.node_type())
                    .name(self.name())
                    .value(v)
                    .build();
                Ok(result)
            }
            _ => {
                let mut result = NodeBuilder::new(self.node_type())
                    .name(self.name())
                    .value(self.value())
                    .build();

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
    fn xmldecl(&self) -> crate::xmldecl::XMLDecl {
        crate::xmldecl::XMLDeclBuilder::new().build()
    }
    fn set_xmldecl(&mut self, _: crate::xmldecl::XMLDecl) -> Result<(), Error> {
        Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not implemented"),
        ))
    }

    fn add_namespace(&self, _ns: Self) -> Result<(), Error> {
        todo!()
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "type {} name {:?} value {:?}",
            self.node_type, self.name, self.value
        )
    }
}

// Find the document order of ancestors
fn doc_order(n: &RNode) -> Vec<usize> {
    match n.node_type {
        NodeType::Document => vec![1 as usize],
        NodeType::Element
        | NodeType::Text
        | NodeType::Comment
        | NodeType::ProcessingInstruction => {
            let p = n.parent.borrow();
            match &*p {
                Some(q) => match Weak::upgrade(&q) {
                    Some(r) => {
                        let idx = find_index(&r, &n).expect("unable to locate node in parent");
                        let mut a = doc_order(&r);
                        a.push(idx + 2);
                        a
                    }
                    None => vec![1 as usize],
                },
                None => vec![1 as usize],
            }
        }
        NodeType::Attribute => {
            // Namespace nodes are first, then attributes
            let mut a = doc_order(&n.parent().unwrap());
            a.push(2);
            a
        }
        _ => vec![0],
    }
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
    match node.node_type {
        NodeType::Document => node
            .children
            .borrow()
            .iter()
            .fold(String::new(), |mut result, c| {
                result.push_str(to_xml_int(c, od, ns.clone(), indent + 2).as_str());
                result
            }),
        NodeType::Element => {
            let mut result = String::from("<");
            // Elements must have a name, so unpack it
            let qn = node.name.borrow().as_ref().unwrap().clone();
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
            node.attributes.borrow().iter().for_each(|(k, _)| {
                namespace_check(k, &declared).iter().for_each(|m| {
                    newns.push(m.clone());
                    declared.push(m.clone())
                })
            });
            // Finally, it's child elements
            node.child_iter()
                .filter(|c| c.node_type == NodeType::Element)
                .for_each(|c| {
                    c.name.borrow().as_ref().map(|d| {
                        namespace_check(d, &declared).iter().for_each(|m| {
                            newns.push(m.clone());
                            declared.push(m.clone())
                        })
                    });
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

            node.attributes
                .borrow()
                .iter()
                .for_each(|(k, v)| result.push_str(format!(" {}='{}'", k, v.value()).as_str()));
            result.push('>');

            // Content of the element.
            // If the indent option is enabled, then if no child is a text node then add spacing.
            let do_indent: bool = od
                .get_indent()
                .then(|| {
                    node.child_iter().fold(true, |mut acc, c| {
                        if acc && c.node_type == NodeType::Text {
                            acc = false
                        }
                        acc
                    })
                })
                .map_or(false, |b| b);

            node.children.borrow().iter().for_each(|c| {
                if do_indent {
                    result.push('\n');
                    (0..indent).for_each(|_| result.push(' '))
                }
                result.push_str(to_xml_int(c, od, newns.clone(), indent + 2).as_str())
            });
            if do_indent && indent > 1 {
                result.push('\n');
                (0..(indent - 2)).for_each(|_| result.push(' '))
            }
            result.push_str("</");
            result.push_str(
                node.name
                    .borrow()
                    .as_ref()
                    .map_or(String::new(), |n| n.to_string())
                    .as_str(),
            );
            result.push('>');
            result
        }
        NodeType::Text => node.value().to_string(),
        NodeType::Comment => {
            let mut result = String::from("<!--");
            let s = node
                .value
                .as_ref()
                .map_or("".to_string(), |n| n.to_string());
            result.push_str(s.as_str());
            result.push_str("-->");
            result
        }
        NodeType::ProcessingInstruction => {
            let mut result = String::from("<?");
            let s = node
                .name
                .borrow()
                .as_ref()
                .map_or("".to_string(), |n| n.to_string());
            result.push_str(s.as_str());
            result.push(' ');
            let t = node.value.clone().map_or("".to_string(), |n| n.to_string());
            result.push_str(t.as_str());
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

// Find the position of this node in the parent's child list.
fn find_index(p: &RNode, c: &RNode) -> Result<usize, Error> {
    let idx = p
        .children
        .borrow()
        .iter()
        .enumerate()
        .fold(None, |mut acc, (i, v)| {
            if Rc::ptr_eq(c, v) {
                acc = Some(i);
                // TODO: stop here
            }
            acc
        });
    idx.map_or(
        Err(Error::new(
            ErrorKind::Unknown,
            String::from("unable to find child"),
        )),
        Ok,
    )
}

pub struct Children {
    v: Vec<RNode>,
    i: usize,
}
impl Children {
    fn new(n: &RNode) -> Self {
        match n.node_type() {
            NodeType::Document | NodeType::Element => Children {
                v: n.children.borrow().clone(),
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
        let s = self.cur.clone();
        let p = s.parent.borrow();
        match &*p {
            None => None,
            Some(q) => match Weak::upgrade(q) {
                None => None,
                Some(r) => {
                    self.cur = r.clone();
                    Some(r)
                }
            },
        }
    }
}

// This implementation eagerly constructs a list of nodes
// to traverse.
// An alternative would be to lazily traverse the descendants.
// Also, rewrite this iterator in terms of child_iter.
pub struct Descendants {
    v: Vec<RNode>,
    cur: usize,
}
impl Descendants {
    fn new(n: &RNode) -> Self {
        Descendants {
            v: n.children.borrow().iter().fold(vec![], |mut acc, c| {
                let mut d = descendant_add(c);
                acc.append(&mut d);
                acc
            }),
            cur: 0,
        }
    }
}
fn descendant_add(n: &RNode) -> Vec<RNode> {
    let mut result = vec![n.clone()];
    n.children.borrow().iter().for_each(|c| {
        let mut l = descendant_add(c);
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
                let (j, _) = p
                    .children
                    .borrow()
                    .iter()
                    .enumerate()
                    .find(|&(_, j)| Rc::ptr_eq(j, n))
                    .unwrap();
                Siblings(p.clone(), j, dir)
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
            match self.0.children.borrow().get(newidx) {
                Some(n) => {
                    self.1 = newidx;
                    Some(n.clone())
                }
                None => None,
            }
        }
    }
}

pub struct Attributes {
    it: IntoIter<QualifiedName, RNode>,
}
impl Attributes {
    fn new(n: &RNode) -> Self {
        let b = n.attributes.borrow();
        Attributes {
            it: b.clone().into_iter(),
        }
    }
}
impl Iterator for Attributes {
    type Item = RNode;

    fn next(&mut self) -> Option<RNode> {
        self.it.next().map(|(_, n)| n)
    }
}

pub struct NodeBuilder(Node);

impl NodeBuilder {
    pub fn new(n: NodeType) -> Self {
        NodeBuilder(Node::new(n))
    }
    pub fn name(self, qn: QualifiedName) -> Self {
        *self.0.name.borrow_mut() = Some(qn);
        self
    }
    pub fn value(mut self, v: Rc<Value>) -> Self {
        self.0.value = Some(v);
        self
    }
    pub fn pi_name(mut self, pi: String) -> Self {
        self.0.pi_name = Some(pi);
        self
    }
    pub fn dtd(mut self, d: DTD) -> Self {
        self.0.dtd = Some(d);
        self
    }
    pub fn reference(mut self, qn: QualifiedName) -> Self {
        self.0.reference = Some(qn);
        self
    }
    pub fn build(self) -> Rc<Node> {
        Rc::new(self.0)
    }
}

#[derive(Clone, PartialEq)]
pub struct XMLDecl {
    pub(crate) version: String,
    pub(crate) encoding: Option<String>,
    pub(crate) standalone: Option<String>,
}

impl XMLDecl {
    pub fn new(version: String, encoding: Option<String>, standalone: Option<String>) -> Self {
        XMLDecl {
            version,
            encoding,
            standalone,
        }
    }
    pub fn version(&self) -> String {
        self.version.clone()
    }
    pub fn set_encoding(&mut self, e: String) {
        self.encoding = Some(e)
    }
    pub fn encoding(&self) -> String {
        self.encoding.as_ref().map_or(String::new(), |e| e.clone())
    }
    pub fn set_standalone(&mut self, s: String) {
        self.standalone = Some(s)
    }
    pub fn standalone(&self) -> String {
        self.standalone
            .as_ref()
            .map_or(String::new(), |e| e.clone())
    }
}

impl fmt::Display for XMLDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::from("<?xml version=\"");
        result.push_str(self.version.as_str());
        result.push('"');
        if let Some(e) = self.encoding.as_ref() {
            result.push_str(" encoding=\"");
            result.push_str(e.as_str());
            result.push('"');
        };
        if let Some(e) = self.standalone.as_ref() {
            result.push_str(" standalone=\"");
            result.push_str(e.as_str());
            result.push('"');
        };
        f.write_str(result.as_str())
    }
}

pub struct XMLDeclBuilder(XMLDecl);

impl Default for XMLDeclBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl XMLDeclBuilder {
    pub fn new() -> Self {
        XMLDeclBuilder(XMLDecl {
            version: String::new(),
            encoding: None,
            standalone: None,
        })
    }
    pub fn version(mut self, v: String) -> Self {
        self.0.version = v;
        self
    }
    pub fn encoding(mut self, v: String) -> Self {
        self.0.encoding = Some(v);
        self
    }
    pub fn standalone(mut self, v: String) -> Self {
        self.0.standalone = Some(v);
        self
    }
    pub fn build(self) -> XMLDecl {
        self.0
    }
}

/// DTD declarations.
/// Only general entities are supported, so far.
/// TODO: element, attribute declarations

#[derive(Clone, PartialEq)]
pub struct DTD {
    pub(crate) elements: HashMap<String, DTDDecl>,
    pub(crate) attlists: HashMap<String, DTDDecl>,
    pub(crate) notations: HashMap<String, DTDDecl>,
    pub(crate) generalentities: HashMap<String, (String, bool)>, // Boolean for is_editable;
    pub(crate) paramentities: HashMap<String, (String, bool)>,
    publicid: Option<String>,
    systemid: Option<String>,
    name: Option<String>,
}

impl DTD {
    pub fn new() -> DTD {
        let default_entities = vec![
            ("amp".to_string(), ("&".to_string(), false)),
            ("gt".to_string(), (">".to_string(), false)),
            ("lt".to_string(), ("<".to_string(), false)),
            ("apos".to_string(), ("'".to_string(), false)),
            ("quot".to_string(), ("\"".to_string(), false)),
        ];
        DTD {
            elements: Default::default(),
            attlists: Default::default(),
            notations: Default::default(),
            generalentities: default_entities.into_iter().collect(),
            paramentities: HashMap::new(),
            publicid: None,
            systemid: None,
            name: None,
        }
    }
}

impl Default for DTD {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DTDDecl {
    Element(QualifiedName, String),
    Attlist(QualifiedName, String),
    Notation(QualifiedName, String),
    GeneralEntity(QualifiedName, String),
    ParamEntity(QualifiedName, String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_push() {
        let mut root = NodeBuilder::new(NodeType::Document).build();
        let child = NodeBuilder::new(NodeType::Element)
            .name(QualifiedName::new(None, None, String::from("Test")))
            .build();
        root.push(child).expect("unable to append child");
        assert_eq!(root.to_xml(), "<Test></Test>")
    }

    #[test]
    fn doc_order() {
        let mut root = NodeBuilder::new(NodeType::Document).build();
        let child = NodeBuilder::new(NodeType::Element)
            .name(QualifiedName::new(None, None, String::from("Test")))
            .build();
        root.push(child.clone()).expect("unable to append child");
        assert_eq!(child.document_order(), vec![1, 2])
    }

    #[test]
    fn cmp_doc_order_1() {
        let mut root = NodeBuilder::new(NodeType::Document).build();
        let child1 = NodeBuilder::new(NodeType::Element)
            .name(QualifiedName::new(None, None, String::from("Before")))
            .build();
        root.push(child1.clone()).expect("unable to append child");
        let child2 = NodeBuilder::new(NodeType::Element)
            .name(QualifiedName::new(None, None, String::from("After")))
            .build();
        root.push(child2.clone()).expect("unable to append child");
        assert_eq!(child1.cmp_document_order(&child2), Ordering::Less);
        assert_eq!(child2.cmp_document_order(&child1), Ordering::Greater);
    }

    #[test]
    fn cmp_doc_order_2() {
        let mut root = NodeBuilder::new(NodeType::Document).build();
        let child1 = NodeBuilder::new(NodeType::Element)
            .name(QualifiedName::new(None, None, String::from("Before")))
            .build();
        root.push(child1.clone()).expect("unable to append child");
        let child2 = NodeBuilder::new(NodeType::Element)
            .name(QualifiedName::new(None, None, String::from("After")))
            .build();
        root.push(child2.clone()).expect("unable to append child");
        assert_eq!(child1.cmp_document_order(&child1), Ordering::Equal)
    }

    #[test]
    fn get_attr() {
        let mut root = NodeBuilder::new(NodeType::Document).build();
        let child = NodeBuilder::new(NodeType::Element)
            .name(QualifiedName::new(None, None, String::from("Test")))
            .build();
        root.push(child.clone()).expect("unable to append child");
        let at = root
            .new_attribute(
                QualifiedName::new(None, None, String::from("mode")),
                Rc::new(Value::from("testing")),
            )
            .expect("unable to create attribute node");
        child.add_attribute(at).expect("unable to add attribute");

        assert_eq!(
            child.get_attribute(&QualifiedName::new(None, None, String::from("mode"))),
            Value::from("testing").into()
        )
    }
    #[test]
    fn get_attr_neg() {
        let mut root = NodeBuilder::new(NodeType::Document).build();
        let child = NodeBuilder::new(NodeType::Element)
            .name(QualifiedName::new(None, None, String::from("Test")))
            .build();
        root.push(child.clone()).expect("unable to append child");
        let at = root
            .new_attribute(
                QualifiedName::new(None, None, String::from("mode")),
                Rc::new(Value::from("testing")),
            )
            .expect("unable to create attribute node");
        child.add_attribute(at).expect("unable to add attribute");

        assert_eq!(
            child.get_attribute(&QualifiedName::new(None, None, String::from("foo"))),
            Value::from("").into()
        )
    }

    #[test]
    fn child_iter() {
        let mut root = NodeBuilder::new(NodeType::Document).build();
        let mut child = NodeBuilder::new(NodeType::Element)
            .name(QualifiedName::new(None, None, String::from("Test")))
            .build();
        root.push(child.clone()).expect("unable to append child");
        (1..=5).for_each(|i| {
            let mut l1 = NodeBuilder::new(NodeType::Element)
                .name(QualifiedName::new(None, None, String::from("Level1")))
                .build();
            child.push(l1.clone()).expect("unable to append child");
            l1.push(
                NodeBuilder::new(NodeType::Text)
                    .value(Rc::new(Value::from(i)))
                    .build(),
            )
            .expect("unable to append child");
        });
        assert_eq!(root.to_xml(), "<Test><Level1>1</Level1><Level1>2</Level1><Level1>3</Level1><Level1>4</Level1><Level1>5</Level1></Test>")
    }

    #[test]
    fn pop() {
        let mut root = NodeBuilder::new(NodeType::Document).build();
        let mut child = NodeBuilder::new(NodeType::Element)
            .name(QualifiedName::new(None, None, String::from("Test")))
            .build();
        root.push(child.clone()).expect("unable to append child");
        (1..=5).for_each(|i| {
            let mut l1 = NodeBuilder::new(NodeType::Element)
                .name(QualifiedName::new(None, None, String::from("Level1")))
                .build();
            child.push(l1.clone()).expect("unable to append child");
            l1.push(
                NodeBuilder::new(NodeType::Text)
                    .value(Rc::new(Value::from(i)))
                    .build(),
            )
            .expect("unable to append child");
        });
        child
            .child_iter()
            .nth(2)
            .unwrap()
            .pop()
            .expect("unable to remove node");
        assert_eq!(
            root.to_xml(),
            "<Test><Level1>1</Level1><Level1>2</Level1><Level1>4</Level1><Level1>5</Level1></Test>"
        )
    }

    #[test]
    fn deep_copy() {
        let mut root = NodeBuilder::new(NodeType::Document).build();
        let mut child = NodeBuilder::new(NodeType::Element)
            .name(QualifiedName::new(None, None, String::from("Test")))
            .build();
        root.push(child.clone()).expect("unable to append child");
        child
            .add_attribute(
                NodeBuilder::new(NodeType::Attribute)
                    .name(QualifiedName::new(None, None, String::from("id")))
                    .value(Rc::new(Value::from("foo")))
                    .build(),
            )
            .expect("unable to add attribute");
        child
            .push(
                NodeBuilder::new(NodeType::Text)
                    .value(Rc::new(Value::from("1234")))
                    .build(),
            )
            .expect("unable to add text node");

        assert_eq!(
            root.deep_copy().expect("unable to copy").to_xml(),
            "<Test id='foo'>1234</Test>"
        )
    }

    #[test]
    fn to_xml() {
        let mut root = NodeBuilder::new(NodeType::Document).build();
        let mut child = NodeBuilder::new(NodeType::Element)
            .name(QualifiedName::new(
                Some(String::from("http://test.org/")),
                Some(String::from("eg")),
                String::from("Test"),
            ))
            .build();
        root.push(child.clone()).expect("unable to append child");
        child
            .add_attribute(
                NodeBuilder::new(NodeType::Attribute)
                    .name(QualifiedName::new(None, None, String::from("id")))
                    .value(Rc::new(Value::from("foo")))
                    .build(),
            )
            .expect("unable to add attribute");
        child
            .push(
                NodeBuilder::new(NodeType::Text)
                    .value(Rc::new(Value::from("1234")))
                    .build(),
            )
            .expect("unable to add text node");

        assert_eq!(
            root.to_xml(),
            "<eg:Test xmlns:eg='http://test.org/' id='foo'>1234</eg:Test>"
        )
    }

    #[test]
    fn to_xml_with_options() {
        let mut root = NodeBuilder::new(NodeType::Document).build();
        let mut child = NodeBuilder::new(NodeType::Element)
            .name(QualifiedName::new(
                Some(String::from("http://test.org/")),
                Some(String::from("eg")),
                String::from("Test"),
            ))
            .build();
        root.push(child.clone()).expect("unable to append child");
        child
            .add_attribute(
                NodeBuilder::new(NodeType::Attribute)
                    .name(QualifiedName::new(None, None, String::from("id")))
                    .value(Rc::new(Value::from("foo")))
                    .build(),
            )
            .expect("unable to add attribute");
        let mut l1 = root
            .new_element(QualifiedName::new(
                Some(String::from("http://test.org/")),
                Some(String::from("eg")),
                String::from("Level-1"),
            ))
            .expect("unable to create element");
        child.push(l1.clone()).expect("unable to add node");
        l1.push(
            NodeBuilder::new(NodeType::Text)
                .value(Rc::new(Value::from("1234")))
                .build(),
        )
        .expect("unable to add text node");

        let mut od = OutputDefinition::new();
        od.set_indent(true);
        assert_eq!(
            root.to_xml_with_options(&od),
            r#"<eg:Test xmlns:eg='http://test.org/' id='foo'>
  <eg:Level-1>1234</eg:Level-1>
</eg:Test>"#
        )
    }
}
