/*! # A tree structure for XDM

This module implements the Item module's [ItemNode] trait.

This implementation uses interior mutability to create and manage a tree structure that is both mutable and fully navigable.

To create a tree, use [NodeBuilder] to make a Document-type node. To add a node, first create it using [NodeBuilder], then use a trait method to attach it to the tree.

NB. The Item module's Node trait is implemented for Rc\<intmuttree::Node\>. For convenience, this is defined as the type [RNode].

```rust
use xrust::intmuttree::{Document, NodeBuilder, RNode};
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
    .value(Value::from("content of the element"))
    .build()
).expect("unable to append child node");

assert_eq!(doc.to_xml(), "<Top-Level>content of the element</Top-Level>")
*/

use crate::item::{Node as ItemNode, NodeType};
use crate::output::OutputDefinition;
use crate::parser;
use crate::parser::xml::XMLDocument;
use crate::qname::*;
use crate::value::Value;
use crate::xdmerror::*;
use std::cell::RefCell;
use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::rc::{Rc, Weak};

pub(crate) type ExtDTDresolver = fn(Option<String>, String) -> Result<String, Error>;

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

        XMLDocument {
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

impl
    TryFrom<(
        String,
        Option<ExtDTDresolver>,
        Option<String>,
    )> for Document
{
    type Error = Error;
    fn try_from(
        s: (
            String,
            Option<ExtDTDresolver>,
            Option<String>,
        ),
    ) -> Result<Self, Self::Error> {
        parser::xml::parse(s.0.as_str(), s.1, s.2)
    }
}
impl
    TryFrom<(
        &str,
        Option<ExtDTDresolver>,
        Option<String>,
    )> for Document
{
    type Error = Error;
    fn try_from(
        s: (
            &str,
            Option<ExtDTDresolver>,
            Option<String>,
        ),
    ) -> Result<Self, Self::Error> {
        parser::xml::parse(s.0, s.1, s.2)
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
    value: Option<Value>,
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
    fn value(&self) -> Value {
        self.value.as_ref().map_or(Value::from(""), |v| v.clone())
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
        match self.node_type {
            NodeType::Document => {
                self.children
                    .borrow()
                    .iter()
                    .fold(String::new(), |mut result, c| {
                        result.push_str(c.to_xml().as_str());
                        result
                    })
            }
            NodeType::Element => {
                let mut result = String::from("<");
                result.push_str(
                    self.name
                        .borrow()
                        .as_ref()
                        .map_or(String::new(), |n| n.to_string())
                        .as_str(),
                );
                self.attributes
                    .borrow()
                    .iter()
                    .for_each(|(k, v)| result.push_str(format!(" {}='{}'", k, v.value()).as_str()));
                result.push('>');
                self.children
                    .borrow()
                    .iter()
                    .for_each(|c| result.push_str(c.to_xml().as_str()));
                result.push_str("</");
                result.push_str(
                    self.name
                        .borrow()
                        .as_ref()
                        .map_or(String::new(), |n| n.to_string())
                        .as_str(),
                );
                result.push('>');
                result
            }
            NodeType::Text => self.value().to_string(),
            _ => String::new(),
        }
    }
    /// Serialise the node as XML, with options such as indentation.
    fn to_xml_with_options(&self, _od: &OutputDefinition) -> String {
        String::from("not implemented")
    }

    fn is_same(&self, other: &Self) -> bool {
        Rc::ptr_eq(self, other)
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
    fn get_attribute(&self, a: &QualifiedName) -> Value {
        self.attributes
            .borrow()
            .get(a)
            .map_or(Value::from(""), |v| v.value.as_ref().unwrap().clone())
    }

    fn new_element(&self, qn: QualifiedName) -> Result<Self, Error> {
        Ok(NodeBuilder::new(NodeType::Element).name(qn).build())
    }
    fn new_text(&self, v: Value) -> Result<Self, Error> {
        Ok(NodeBuilder::new(NodeType::Text).value(v).build())
    }
    fn new_attribute(&self, qn: QualifiedName, v: Value) -> Result<Self, Error> {
        Ok(NodeBuilder::new(NodeType::Attribute)
            .name(qn)
            .value(v)
            .build())
    }

    /// Append a node to the child list
    fn push(&mut self, n: RNode) -> Result<(), Error> {
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
        self.attributes.borrow_mut().insert(att.name(), att);
        Ok(())
    }
    /// Insert a node into the child list immediately before this node.
    fn insert_before(&mut self, mut insert: Self) -> Result<(), Error> {
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
            NodeType::Comment => Err(Error {
                kind: ErrorKind::TypeError,
                message: "".to_string(),
            }),
            NodeType::Text => {
                let v = match self.value() {
                    Value::String(s) => {
                        Value::String(s.replace("\r\n", "\n").replace("\n\n", "\n"))
                    }
                    e => e,
                };
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
    pub fn value(mut self, v: Value) -> Self {
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
    fn get_attr() {
        let mut root = NodeBuilder::new(NodeType::Document).build();
        let child = NodeBuilder::new(NodeType::Element)
            .name(QualifiedName::new(None, None, String::from("Test")))
            .build();
        root.push(child.clone()).expect("unable to append child");
        let at = root
            .new_attribute(
                QualifiedName::new(None, None, String::from("mode")),
                Value::from("testing"),
            )
            .expect("unable to create attribute node");
        child.add_attribute(at).expect("unable to add attribute");

        assert_eq!(
            child.get_attribute(&QualifiedName::new(None, None, String::from("mode"))),
            Value::from("testing")
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
                Value::from("testing"),
            )
            .expect("unable to create attribute node");
        child.add_attribute(at).expect("unable to add attribute");

        assert_eq!(
            child.get_attribute(&QualifiedName::new(None, None, String::from("foo"))),
            Value::from("")
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
                    .value(Value::from(i))
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
                    .value(Value::from(i))
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
                    .value(Value::from("foo"))
                    .build(),
            )
            .expect("unable to add attribute");
        child
            .push(
                NodeBuilder::new(NodeType::Text)
                    .value(Value::from("1234"))
                    .build(),
            )
            .expect("unable to add text node");

        assert_eq!(
            root.deep_copy().expect("unable to copy").to_xml(),
            "<Test id='foo'>1234</Test>"
        )
    }
}
