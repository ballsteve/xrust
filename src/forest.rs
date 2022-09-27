//! # xrust::forest
//!
//! A forest is a collection of [Tree]s. A [Tree] is a collection of [Node]s. A [Node] is an index into the [Tree].
//!
//! Both [Forest]s and [Tree]s use an arena allocator, so the object itself is simply an index that may be copied and cloned. However, in order to dererence the [Tree] or [Node] the [Forest] must be passed as an argument. This also makes deallocating memory difficult; the objects will persist until the entire [Forest] is freed.

use crate::output::OutputDefinition;
use crate::parsexml::{XMLDocument, XMLNode};
use crate::qname::QualifiedName;
use crate::value::Value;
use crate::xdmerror::{Error, ErrorKind};
use generational_arena::{Arena, Index};
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::convert::TryFrom;

/// A Forest. Forests contain [Tree]s. Each [Tree] is identified by a copyable value, similar to a Node value, that can be easily stored and passed as a parameter.
#[derive(Clone)]
pub struct Forest {
    a: Vec<Tree>,
}

pub type TreeIndex = usize;

impl Forest {
    /// Create a new, empty forest.
    pub fn new() -> Forest {
        Forest { a: vec![] }
    }
    /// Start a [Tree] in the forest. The [Tree] will have a single node, which is a Document type [Node].
    pub fn plant_tree(&mut self) -> TreeIndex {
        let i = self.a.len();
        self.a.push(Tree::new(i));
        i
    }

    /// Borrow a [Tree], given a [TreeIndex]. Return None if no suh [Tree] exists.
    pub fn get_ref(&self, i: TreeIndex) -> Option<&Tree> {
        self.a.get(i)
    }
    /// Mutably borrow a [Tree], given a [TreeIndex]. Return None if no suh [Tree] exists.
    pub fn get_ref_mut(&mut self, i: TreeIndex) -> Option<&mut Tree> {
        self.a.get_mut(i)
    }

    /// Parse a string as XML to create a [Tree].
    ///
    ///```rust
    ///use xrust::forest::Forest;
    ///let mut f = Forest::new();
    ///let src = f.grow_tree("<Example>document</Example>")
    ///    .expect("unable to parse XML");
    pub fn grow_tree(&mut self, s: &str) -> Result<TreeIndex, Error> {
        let d = XMLDocument::try_from(s)?;
        if d.content.len() == 0 {
            Result::Err(Error::new(
                ErrorKind::Unknown,
                String::from("unable to parse XML"),
            ))
        } else {
            let mut ns: HashMap<String, String> = HashMap::new();
            let ti = self.plant_tree();
            for c in d.content {
                let e = make_node(c, self, ti, &mut ns)?;
                self.get_ref_mut(ti).unwrap().push_doc_node(e)?;
            }
            Ok(ti)
        }
    }
}

/// A Tree, using an Arena Allocator.
/// Nodes can be detached, but not deleted
#[derive(Clone)]
pub struct Tree {
    i: TreeIndex, // The index in the Forest
    a: Arena<NodeContent>,
    d: Index, // The document node
}

impl Tree {
    /// Create a tree with the given [TreeIndex].
    ///
    /// The newly created tree has a single [Node], of type Document.
    pub fn new(i: TreeIndex) -> Self {
        let mut a = Arena::new();
        let d = a.insert(NodeBuilder::new(NodeType::Document).build());
        Tree { i: i, a: a, d: d }
    }

    fn get(&self, i: Index) -> Option<&NodeContent> {
        self.a.get(i)
    }
    fn get_mut(&mut self, i: Index) -> Option<&mut NodeContent> {
        self.a.get_mut(i)
    }
    /// Return the Document-type [Node].
    pub fn get_doc_node(&self) -> Node {
        Node::new(self.d, self.i)
    }
    /// Append a [Node] as a child of the Document-type [Node].
    pub fn push_doc_node(&mut self, n: Node) -> Result<(), Error> {
        // Set the parent to the document node
        self.get_mut(n.0).unwrap().parent = Some(Node::new(self.d, self.i));
        // Push the node onto the doc node's children
        self.get_mut(self.d).map_or_else(
            || {
                Result::Err(Error::new(
                    ErrorKind::Unknown,
                    String::from("no document node"),
                ))
            },
            |e| {
                e.children.push(n);
                Ok(())
            },
        )
    }

    /// Create a new Element-type [Node] in this tree. The newly created [Node] is not attached to the tree, i.e. it has no parent.
    pub fn new_element(&mut self, name: QualifiedName) -> Result<Node, Error> {
        Ok(Node::new(
            self.a
                .insert(NodeBuilder::new(NodeType::Element).name(name).build()),
            self.i,
        ))
    }
    /// Create a new Text-type [Node] in this tree. The newly created [Node] is not attached to the tree, i.e. it has no parent.
    pub fn new_text(&mut self, c: Value) -> Result<Node, Error> {
        Ok(Node::new(
            self.a
                .insert(NodeBuilder::new(NodeType::Text).value(c).build()),
            self.i,
        ))
    }
    /// Create a new Attribute-type [Node] in this tree. The newly created [Node] is not attached to the tree, i.e. it has no parent.
    pub fn new_attribute(&mut self, name: QualifiedName, v: Value) -> Result<Node, Error> {
        Ok(Node::new(
            self.a.insert(
                NodeBuilder::new(NodeType::Attribute)
                    .name(name)
                    .value(v)
                    .build(),
            ),
            self.i,
        ))
    }
    /// Create a new Comment-type [Node] in this tree. The newly created [Node] is not attached to the tree, i.e. it has no parent.
    pub fn new_comment(&mut self, v: Value) -> Result<Node, Error> {
        Ok(Node::new(
            self.a
                .insert(NodeBuilder::new(NodeType::Comment).value(v).build()),
            self.i,
        ))
    }
    /// Create a new ProcessingInstruction-type [Node] in this tree. The newly created [Node] is not attached to the tree, i.e. it has no parent.
    pub fn new_processing_instruction(
        &mut self,
        name: QualifiedName,
        v: Value,
    ) -> Result<Node, Error> {
        Ok(Node::new(
            self.a.insert(
                NodeBuilder::new(NodeType::ProcessingInstruction)
                    .name(name)
                    .value(v)
                    .build(),
            ),
            self.i,
        ))
    }
}

fn make_node(
    n: XMLNode,
    f: &mut Forest,
    ti: TreeIndex,
    ns: &mut HashMap<String, String>,
) -> Result<Node, Error> {
    match n {
        XMLNode::Element(m, a, c) => {
            a.iter()
                .filter(|b| match b {
                    XMLNode::Attribute(qn, _) => match qn.get_prefix() {
                        Some(p) => p == "xmlns",
                        _ => false,
                    },
                    _ => false,
                })
                .for_each(|b| {
                    if let XMLNode::Attribute(qn, v) = b {
                        // add map from prefix to uri in hashmap
                        ns.insert(qn.get_localname(), v.to_string()).map(|_| {});
                    }
                });
            // Add element to the tree
            let newns = match m.get_prefix() {
                Some(p) => match ns.get(&p) {
                    Some(q) => Some(q.clone()),
                    None => {
                        return Result::Err(Error::new(
                            ErrorKind::Unknown,
                            String::from("namespace URI not found for prefix"),
                        ))
                    }
                },
                None => None,
            };
            let new = f.get_ref_mut(ti).unwrap().new_element(QualifiedName::new(
                newns,
                m.get_prefix(),
                m.get_localname(),
            ))?;

            // Attributes
            a.iter().for_each(|b| {
                if let XMLNode::Attribute(qn, v) = b {
                    match qn.get_prefix() {
                        Some(p) => {
                            if p != "xmlns" {
                                let ans = ns.get(&p).unwrap_or(&"".to_string()).clone();
                                match f.get_ref_mut(ti).unwrap().new_attribute(
                                    QualifiedName::new(Some(ans), Some(p), qn.get_localname()),
                                    v.clone(),
                                ) {
                                    Ok(c) => {
                                        new.add_attribute(f, c).expect("unable to add attribute");
                                        // TODO: Don't Panic
                                    }
                                    Err(_) => {
                                        //return Result::Err(e);
                                    }
                                };
                            }
                            // otherwise it is a namespace declaration, see above
                        }
                        _ => {
                            // Unqualified name
                            match f
                                .get_ref_mut(ti)
                                .unwrap()
                                .new_attribute(qn.clone(), v.clone())
                            {
                                Ok(c) => {
                                    new.add_attribute(f, c).expect("unable to add attribute");
                                    // TODO: Don't Panic
                                }
                                Err(_) => {
                                    //return Result::Err(e);
                                }
                            }
                        }
                    }
                }
            });

            // Element content
            for h in c.iter().cloned() {
                let g = make_node(h, f, ti, ns)?;
                new.append_child(f, g)?
            }

            Ok(new)
        }
        XMLNode::Attribute(_qn, _v) => {
            // Handled in element arm
            Result::Err(Error::new(
                ErrorKind::NotImplemented,
                String::from("not implemented"),
            ))
        }
        XMLNode::Text(v) => Ok(f.get_ref_mut(ti).unwrap().new_text(v)?),
        XMLNode::Comment(v) => Ok(f.get_ref_mut(ti).unwrap().new_comment(v)?),
        XMLNode::PI(m, v) => Ok(f
            .get_ref_mut(ti)
            .unwrap()
            .new_processing_instruction(QualifiedName::new(None, None, m), v)?),
        XMLNode::Reference(_) | XMLNode::DTD(_) => Result::Err(Error::new(
            ErrorKind::TypeError,
            String::from("not expected"),
        )),
    }
}

/// All [Node]s have a type. The type of the [Node] determines what components are meaningful, such as name and content.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum NodeType {
    Document,
    Element,
    Text,
    Attribute,
    Comment,
    ProcessingInstruction,
    Unknown,
}

impl NodeType {
    /// Return a string representation of the node type.
    pub fn to_string(&self) -> &'static str {
        match self {
            NodeType::Document => "Document",
            NodeType::Element => "Element",
            NodeType::Attribute => "Attribute",
            NodeType::Text => "Text",
            NodeType::ProcessingInstruction => "Processing-Instruction",
            NodeType::Comment => "Comment",
            NodeType::Unknown => "--None--",
        }
    }
}

impl Default for NodeType {
    fn default() -> Self {
        NodeType::Unknown
    }
}

/// A node in the [Tree]. Depending on the type of the node, it may have a name, value, content, or attributes.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Node(Index, TreeIndex);

impl Node {
    /// Wrap the given Arena Index and [TreeIndex] to create a new node. NB. this does not create a node in the [Tree] (i.e. [Forest] or arena allocator)
    fn new(i: Index, t: TreeIndex) -> Self {
        Node(i, t)
    }

    fn get<'a>(&self, f: &'a Forest) -> Option<&'a NodeContent> {
        f.get_ref(self.1).unwrap().get(self.0)
    }

    /// Programmer view of the Node. This is needed since the std::fmt::Debug trait cannot be implemented (because the Forest is required to gather the necessary data).
    pub fn fmt_debug(&self, f: &Forest) -> String {
        let mut result = String::from(self.node_type(f).to_string());
        result.push_str("-type node");
        match self.node_type(f) {
            NodeType::Element => {
                result.push_str(" \"");
                result.push_str(self.to_name(f).to_string().as_str());
                result.push_str("\"");
            }
            NodeType::Text => {
                result.push_str(" \"");
                result.push_str(self.to_value(f).to_string().as_str()); // TODO: limit to at most, say, 10 chars
                result.push_str("\"");
            }
            _ => {} // TODO: attribute, comment, PI, etc
        }
        result
    }

    /// Return the string representation of the node.
    pub fn to_string(&self, f: &Forest) -> String {
        match f.get_ref(self.1) {
            Some(e) => e,
            None => return String::from(""),
        };
        match self.node_type(f) {
            NodeType::Element => {
                // TODO: string value of all descendant text nodes
                String::new()
            }
            NodeType::Text | NodeType::Attribute | NodeType::Comment => self
                .get(f)
                .unwrap()
                .value()
                .as_ref()
                .map_or(String::new(), |v| v.to_string()),
            _ => String::new(),
        }
    }
    /// Serialise the node as XML.
    pub fn to_xml(&self, f: &Forest) -> String {
        let mut ns: HashMap<String, Option<String>> = HashMap::new();
        self.to_xml_int(f, &OutputDefinition::new(), 0, &mut ns)
    }
    fn to_xml_int(
        &self,
        f: &Forest,
        od: &OutputDefinition,
        indent: usize,
        ns: &mut HashMap<String, Option<String>>,
    ) -> String {
        let d = match f.get_ref(self.1) {
            Some(e) => e,
            None => return String::from(""),
        };
        let nc = match d.get(self.0) {
            Some(e) => e,
            None => return String::from(""),
        };
        match nc.node_type() {
            NodeType::Element => {
                let mut result = String::from("<");

                let name = nc.name().as_ref().unwrap();

                // Check if any XML Namespaces need to be declared,
                // Either for the element for any of its attributes.
                let mut newns: Vec<(Option<String>, String)> = vec![];
                if let Some(uri) = name.get_nsuri() {
                    match name.get_prefix() {
                        Some(p) => {
                            match ns.get(uri.as_str()) {
                                Some(op) => {
                                    // Already declared, but with the same prefix?
                                    match op {
                                        Some(q) => {
                                            if p != *q {
                                                ns.insert(uri.clone(), Some(p.clone()));
                                                newns.push((Some(p), uri));
                                            } // else already declared
                                        }
                                        None => {
                                            // Was declared with default namespace, now has a prefix
                                            ns.insert(uri.clone(), Some(p.clone()));
                                            newns.push((Some(p), uri));
                                        }
                                    }
                                }
                                None => {
                                    ns.insert(uri.clone(), Some(p.clone()));
                                    newns.push((Some(p), uri));
                                }
                            }
                        }
                        None => {
                            // Default namespace
                            match ns.get(uri.as_str()) {
                                Some(_) => {
                                    ns.insert(uri.clone(), None);
                                    newns.push((None, uri));
                                }
                                None => {
                                    // Already declared
                                }
                            }
                        }
                    }
                }

                result.push_str(name.to_string().as_str());
                newns.iter().for_each(|(p, u)| {
                    result.push_str(" xmlns");
                    if let Some(q) = p {
                        result.push(':');
                        result.push_str(q.as_str());
                    }
                    result.push_str("='");
                    result.push_str(u);
                    result.push('\'');
                });
                nc.attributes.iter().for_each(|(k, v)| {
                    // Declare namespace for attribute, if not already declared
                    if let Some(uri) = k.get_nsuri() {
                        if ns.get(uri.as_str()).is_none() {
                            ns.insert(uri.clone(), k.get_prefix());
                            result.push_str(" xmlns:");
                            result.push_str(k.get_prefix().unwrap().as_str());
                            result.push_str("='");
                            result.push_str(uri.as_str());
                            result.push('\'');
                        }
                    }
                    result.push(' ');
                    result.push_str(k.to_string().as_str());
                    result.push_str("='");
                    result.push_str(v.to_string(f).as_str());
                    result.push('\'');
                });
                result.push_str(">");

                // Content of the element.
                // If the indent option is enabled, then if no child is a text node then add spacing
                let do_indent: bool = if od.get_indent() {
                    let mut acc = true;
                    let mut children = self.child_iter();
                    loop {
                        match children.next(f) {
                            Some(c) => {
                                if c.node_type(f) == NodeType::Text {
                                    acc = false
                                }
                            }
                            None => break,
                        }
                    }
                    acc
                } else {
                    false
                };
                let mut children = self.child_iter();
                loop {
                    match children.next(f) {
                        Some(c) => {
                            if do_indent {
                                result.push('\n');
                                (0..indent).for_each(|_| result.push(' '));
                            };
                            result.push_str(c.to_xml_int(f, od, indent, ns).as_str());
                        }
                        None => break,
                    }
                }
                if do_indent {
                    result.push('\n');
                    (0..(indent - 2)).for_each(|_| result.push(' '));
                };

                result.push_str("</");
                result.push_str(name.to_string().as_str());
                result.push_str(">");
                result
            }
            NodeType::Text => nc.value().as_ref().unwrap().to_string(),
            NodeType::Comment => {
                let mut result = String::from("<!--");
                result.push_str(nc.value().as_ref().unwrap().to_string().as_str());
                result.push_str("-->");
                result
            }
            NodeType::ProcessingInstruction => {
                let mut result = String::from("<?");
                result.push_str(nc.name().as_ref().unwrap().to_string().as_str());
                result.push(' ');
                result.push_str(nc.value().as_ref().unwrap().to_string().as_str());
                result.push_str("?>");
                result
            }
            _ => {
                // TODO
                String::from("-- not implemented --")
            }
        }
    }
    /// Serialise the node as XML, under the control of the given OutputDefinition. The usual use is to perform indenting, i.e. "pretty-printing".
    pub fn to_xml_with_options(&self, f: &Forest, od: &OutputDefinition) -> String {
        let mut ns: HashMap<String, Option<String>> = HashMap::new();
        self.to_xml_int(f, od, 2, &mut ns)
    }
    /// Serialise the node as JSON.
    pub fn to_json(&self, _f: &Forest) -> String {
        String::from("not implemented yet")
    }

    /// A convenience method that converts the value to a string and then converts the string to an integer.
    pub fn to_int(&self, f: &Forest) -> Result<i64, Error> {
        // Convert to a string, then try parsing that as an integer
        self.to_string(f)
            .parse::<i64>()
            .map_err(|e| Error::new(ErrorKind::Unknown, e.to_string()))
    }
    /// A convenience method that converts the value to a string and then converts the string to a double.
    pub fn to_double(&self, f: &Forest) -> f64 {
        // Convert to a string, then try parsing that as a double
        match self.to_string(f).parse::<f64>() {
            Ok(g) => g,
            Err(_) => f64::NAN,
        }
    }
    /// Get the name of the node. If the node is of a type that doesn't have a name, returns a name with an empty local name, URI, and prefix.
    pub fn to_name(&self, f: &Forest) -> QualifiedName {
        f.get_ref(self.1)
            .map_or(QualifiedName::new(None, None, String::from("")), |d| {
                d.get(self.0)
                    .map_or(QualifiedName::new(None, None, String::from("")), |o| {
                        o.name()
                            .as_ref()
                            .map_or(QualifiedName::new(None, None, String::from("")), |p| {
                                p.clone()
                            })
                    })
            })
    }
    /// Get the value of the node. If the node is of a type that doesn't have a value, returns an empty string value.
    pub fn to_value(&self, f: &Forest) -> Value {
        f.get_ref(self.1).map_or(Value::from(""), |d| {
            d.get(self.0).map_or(Value::from(""), |o| {
                o.value().as_ref().map_or(Value::from(""), |p| p.clone())
            })
        })
    }

    /// Returns the node's type.
    pub fn node_type(&self, f: &Forest) -> NodeType {
        f.get_ref(self.1).map_or(NodeType::Unknown, |d| {
            d.get(self.0).map_or(NodeType::Unknown, |m| m.node_type())
        })
    }

    /// Append the given node to this node's child list. This node must be an element-type node. The node to be appended must not be an attribute-type node.
    ///
    /// If the given node is not in the same [Tree] as this node, makes a deep copy of the given node and appends that to this node's child list.
    pub fn append_child(&self, f: &mut Forest, c: Node) -> Result<(), Error> {
        // Check that self is an element and that c is not an attribute
        if self.node_type(f) != NodeType::Element {
            return Result::Err(Error::new(
                ErrorKind::Unknown,
                String::from("must be an element"),
            ));
        }
        if c.node_type(f) == NodeType::Attribute {
            return Result::Err(Error::new(
                ErrorKind::Unknown,
                String::from("cannot append an attribute as a child"),
            ));
        }

        // Is c in a different Tree?
        if self.1 == c.1 {
            // Detach from its current position, then append to self's child list
            c.remove(f)?;

            // self will now be c's parent
            f.get_ref_mut(self.1)
                .ok_or(Error::new(
                    ErrorKind::Unknown,
                    String::from("unable to find tree"),
                ))?
                .get_mut(c.0)
                .ok_or(Error::new(
                    ErrorKind::Unknown,
                    String::from("unable to find node"),
                ))?
                .parent = Some(self.clone());

            // Push c onto self's child list
            f.get_ref_mut(self.1)
                .ok_or(Error::new(
                    ErrorKind::Unknown,
                    String::from("unable to find tree"),
                ))?
                .get_mut(self.0)
                .ok_or(Error::new(
                    ErrorKind::Unknown,
                    String::from("unable to find node"),
                ))?
                .children
                .push(c);
        } else {
            // c is in a different Tree, so deep copy
            let cp = c.deep_copy(f, Some(self.1))?;

            // self will now be cp's parent
            f.get_ref_mut(self.1)
                .ok_or(Error::new(
                    ErrorKind::Unknown,
                    String::from("unable to find tree"),
                ))?
                .get_mut(cp.0)
                .ok_or(Error::new(
                    ErrorKind::Unknown,
                    String::from("unable to find node"),
                ))?
                .parent = Some(self.clone());

            // Push cp onto self's child list
            f.get_ref_mut(self.1)
                .ok_or(Error::new(
                    ErrorKind::Unknown,
                    String::from("unable to find tree"),
                ))?
                .get_mut(self.0)
                .ok_or(Error::new(
                    ErrorKind::Unknown,
                    String::from("unable to find node"),
                ))?
                .children
                .push(cp);
        }

        Ok(())
    }
    /// Insert the given node before this node in the parent's child list. This node must be an element-type node. The given node must not be an attribute-type node.
    /// If the given node is in the same tree, then it is removed from the tree and then inserted so that it becomes the first preceding of this node.
    /// If the given node is in a different tree, then it is deep copied. The copied node will then become the first preceding sibling of this node.
    pub fn insert_before(&self, f: &mut Forest, insert: Node) -> Result<(), Error> {
        let p = self.parent(f).ok_or(Error::new(
            ErrorKind::Unknown,
            String::from("unable to insert before document node"),
        ))?;

        if self.1 == insert.1 {
            // Given node is in the same tree. Detach from it's current position, and then insert before this node.
            insert.remove(f)?;
            let d = f.get_ref_mut(self.1).ok_or(Error::new(
                ErrorKind::Unknown,
                String::from("unable to find tree"),
            ))?;
            let cl = &mut d.get_mut(p.0).unwrap().children;
            let i = cl
                .iter()
                .enumerate()
                .skip_while(|(_, x)| x.0 != self.0)
                .nth(0)
                .map(|(e, _)| e)
                .unwrap();
            cl.insert(i, insert);
            d.get_mut(insert.0).unwrap().parent = Some(p);
        } else {
            // Given node is in a different tree. Deep copy the node.
            // First find where to insert the copied node
            let d = f.get_ref(self.1).ok_or(Error::new(
                ErrorKind::Unknown,
                String::from("unable to find tree"),
            ))?;
            let i = d
                .get(p.0)
                .unwrap()
                .children
                .iter()
                .enumerate()
                .skip_while(|(_, x)| x.0 != self.0)
                .nth(0)
                .map(|(e, _)| e)
                .unwrap();

            // Then do the copy and insert it
            let cp = insert.deep_copy(f, Some(self.1))?;
            let clm = &mut f
                .get_ref_mut(self.1)
                .ok_or(Error::new(
                    ErrorKind::Unknown,
                    String::from("unable to find tree"),
                ))?
                .get_mut(p.0)
                .unwrap()
                .children;
            clm.insert(i, cp);

            // Update the copied node with it's new parent
            f.get_ref_mut(self.1)
                .ok_or(Error::new(
                    ErrorKind::Unknown,
                    String::from("unable to find tree"),
                ))?
                .get_mut(cp.0)
                .unwrap()
                .parent = Some(p);
        }

        Ok(())
    }

    /// Detach the node from the tree
    pub fn remove(&self, f: &mut Forest) -> Result<(), Error> {
        let d = f.get_ref_mut(self.1).ok_or(Error::new(
            ErrorKind::Unknown,
            String::from("unable to find tree"),
        ))?;

        // Is this node in the tree? If not, then do nothing
        let p = match d.get(self.0).unwrap().parent {
            Some(q) => q.0,
            None => return Ok(()),
        };

        // Remove from parent's child list
        let cl = &mut d.get_mut(p).unwrap().children;
        let i = cl
            .iter()
            .enumerate()
            .skip_while(|(_, x)| x.0 != self.0)
            .nth(0)
            .map(|(e, _)| e)
            .unwrap();
        cl.remove(i);

        // This node now has no parent
        d.get_mut(self.0).unwrap().parent = None;

        Ok(())
    }

    /// Add the given node as an attribute of this node. This node must be an element-type node. The given node must be an attribute-type node. The given node is detached from it's current parent and then attached as an attribute of this node. If the given node is in a different [Tree] to this node, then it is deep-copied and the given node remains untouched.
    pub fn add_attribute(&self, f: &mut Forest, a: Node) -> Result<(), Error> {
        if self.node_type(f) != NodeType::Element {
            return Result::Err(Error::new(
                ErrorKind::Unknown,
                String::from("must be an element"),
            ));
        }
        if a.node_type(f) != NodeType::Attribute {
            return Result::Err(Error::new(
                ErrorKind::Unknown,
                String::from("argument must be an attribute"),
            ));
        }

        let d = match f.get_ref_mut(self.1) {
            Some(e) => e,
            None => {
                return Result::Err(Error::new(
                    ErrorKind::Unknown,
                    String::from("unable to find tree"),
                ))
            }
        };

        // TODO: detach a from wherever it is currently located

        // self will now be a's parent
        d.get_mut(a.0).unwrap().parent = Some(self.clone());
        // Add a to self's attribute hashmap
        let qn = d.get(a.0).unwrap().name().as_ref().unwrap().clone();
        d.get_mut(self.0).unwrap().attributes.insert(qn, a);
        Ok(())
    }

    /// Creates an interator for the ancestors of this node.
    pub fn ancestor_iter(&self) -> Ancestors {
        Ancestors::new(self.0, self.1)
    }
    /// Returns the parent node.
    ///
    /// The Document-type node of the [Tree] does not have a parent. If the node is not attached to the [Tree], it will not have a parent.
    pub fn parent(&self, f: &Forest) -> Option<Node> {
        self.ancestor_iter().next(f).map(|p| p)
    }
    /// Creates an iterator over the children of this node.
    pub fn child_iter(&self) -> Children {
        Children::new(self.0, self.1)
    }
    /// Returns the first child node of this node.
    pub fn get_first_element(&self, f: &Forest) -> Option<Node> {
        let mut cit = self.child_iter();
        let mut ret = None;
        loop {
            match cit.next(f) {
                Some(n) => match n.node_type(f) {
                    NodeType::Element => {
                        ret = Some(n);
                        break;
                    }
                    _ => {}
                },
                None => break,
            }
        }
        ret
    }
    /// Creates an iterator over the following siblings of this node.
    pub fn next_iter(&self, f: &Forest) -> Siblings {
        Siblings::new(self.0, self.1, 1, f)
    }
    /// Creates an iterator over the preceding siblings of this node.
    pub fn prev_iter(&self, f: &Forest) -> Siblings {
        Siblings::new(self.0, self.1, -1, f)
    }
    /// Creates an iterator over the descendants of this node.
    pub fn descend_iter(&self, f: &Forest) -> Descendants {
        Descendants::new(self.0, self.1, f)
    }
    /// Creates an iterator over the attributes of this node.
    pub fn attribute_iter<'a>(&self, f: &'a Forest) -> Attributes<'a> {
        Attributes::new(self.0, f.get_ref(self.1).unwrap())
    }
    /// Returns an attribute.
    pub fn get_attribute(&self, f: &Forest, qn: &QualifiedName) -> Option<Node> {
        match f.get_ref(self.1) {
            Some(d) => match d.get(self.0) {
                Some(nc) => match nc.attributes.get(qn) {
                    Some(m) => Some(m.clone()),
                    None => None,
                },
                None => None,
            },
            None => None,
        }
    }

    /// Convenience method that returns if this node is an element-type node
    pub fn is_element(&self, f: &Forest) -> bool {
        match f.get_ref(self.1) {
            Some(d) => match d.get(self.0) {
                Some(nc) => nc.t == NodeType::Element,
                None => false,
            },
            None => false,
        }
    }
    /// Make a recursive copy of the node, i.e. a "deep" copy.
    ///
    /// The new node will be created in a different tree if one is supplied.
    pub fn deep_copy(&self, f: &mut Forest, t: Option<TreeIndex>) -> Result<Node, Error> {
        let cptreeidx = t.map_or_else(|| self.1, |u| u);
        // TODO: check that this is a valid tree index

        match self.node_type(f) {
            NodeType::Element => {
                let nm = self.to_name(f);
                let new = f.get_ref_mut(cptreeidx).unwrap().new_element(nm)?;
                let mut attrs = vec![];
                let mut ait = self.attribute_iter(f);
                loop {
                    match ait.next() {
                        Some(a) => attrs.push(a),
                        None => break,
                    }
                }
                attrs.iter().for_each(|a| {
                    let cp = a.deep_copy(f, t).expect("unable to copy attribute");
                    new.add_attribute(f, cp).expect("unable to add attribute");
                });
                let mut cit = self.child_iter();
                loop {
                    match cit.next(f) {
                        Some(d) => {
                            let cp = d.deep_copy(f, t)?;
                            new.append_child(f, cp)?;
                        }
                        None => break,
                    }
                }
                Ok(new)
            }
            NodeType::Attribute => {
                let nm = self.to_name(f);
                let v = self.to_value(f);
                f.get_ref_mut(cptreeidx).unwrap().new_attribute(nm, v)
            }
            NodeType::Text => {
                let v = self.to_value(f);
                f.get_ref_mut(cptreeidx).unwrap().new_text(v)
            }
            NodeType::Comment => {
                let v = self.to_value(f);
                f.get_ref_mut(cptreeidx).unwrap().new_comment(v)
            }
            NodeType::ProcessingInstruction => {
                let nm = self.to_name(f);
                let v = self.to_value(f);
                f.get_ref_mut(cptreeidx)
                    .unwrap()
                    .new_processing_instruction(nm, v)
            }
            _ => Result::Err(Error::new(
                ErrorKind::Unknown,
                String::from("unable to copy node"),
            )),
        }
    }
}

/// Navigate the ancestors of a [Node].
pub struct Ancestors {
    t: TreeIndex,
    cur: Index,
}

impl Ancestors {
    fn new(cur: Index, t: TreeIndex) -> Ancestors {
        Ancestors { t, cur }
    }
    pub fn next(&mut self, f: &Forest) -> Option<Node> {
        if let Some(d) = f.get_ref(self.t) {
            if let Some(c) = d.get(self.cur) {
                if let Some(p) = c.parent {
                    if p.node_type(f) == NodeType::Document {
                        None
                    } else {
                        self.cur = p.0;
                        Some(p)
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// Navigate the descendants of a [Node].
pub struct Descendants {
    t: TreeIndex,
    start: Index,
    cur: Index,
    stack: Vec<(Index, usize)>,
}

impl Descendants {
    fn new(cur: Index, t: TreeIndex, f: &Forest) -> Descendants {
        // Find cur in the parent's child list
        let d = f.get_ref(t).unwrap();
        let pi = d.get(cur).unwrap().parent.unwrap().0;
        let p = d.get(pi).unwrap();
        let q = p
            .children
            .iter()
            .enumerate()
            .skip_while(|(_, i)| i.0 != cur)
            .nth(0)
            .map(|(e, _)| e)
            .unwrap();
        Descendants {
            t,
            start: cur,
            cur: cur,
            stack: vec![(pi, q)],
        }
    }
    pub fn next(&mut self, f: &Forest) -> Option<Node> {
        if self.stack.is_empty() {
            None
        } else {
            // Return the first child,
            // otherwise return the next sibling
            // otherwise return an ancestor's next sibling
            // (don't go past start)
            match Node::new(self.cur, self.t).child_iter().next(f) {
                Some(n) => {
                    self.stack.push((self.cur, 0));
                    self.cur = n.0;
                    Some(n)
                }
                None => {
                    let d = f.get_ref(self.t).unwrap();
                    let (i, mut s) = self.stack.last_mut().unwrap();
                    let pnc = d.get(*i).unwrap();
                    if pnc.children.len() < s {
                        // have a next sibling
                        s += 1;
                        self.cur = pnc.children.get(s).unwrap().0;
                        Some(Node::new(self.cur, self.t))
                    } else {
                        // ancestor next sibling
                        let result: Option<Node>;
                        loop {
                            self.stack.pop();
                            if self.stack.is_empty() {
                                result = None;
                                break;
                            } else {
                                let l = self.stack.last_mut().unwrap();
                                let (j, mut t) = l;
                                let qnc = d.get(*j).unwrap();
                                if qnc.children.len() > t + 1 {
                                    t += 1;
                                    *l = (*j, t);
                                    self.cur = qnc.children.get(t).unwrap().0;
                                    result = Some(Node::new(self.cur, self.t));
                                    break;
                                } else {
                                    if *j == self.start {
                                        result = None;
                                        break;
                                    }
                                }
                            }
                        }
                        result
                    }
                }
            }
        }
    }
}

/// Navigate the children of a [Node].
pub struct Children {
    t: TreeIndex,
    parent: Index,
    cur: usize,
}

impl Children {
    fn new(parent: Index, t: TreeIndex) -> Children {
        Children { t, parent, cur: 0 }
    }
    pub fn next(&mut self, f: &Forest) -> Option<Node> {
        if let Some(d) = f.get_ref(self.t) {
            if let Some(n) = d.get(self.parent) {
                if n.children.len() > self.cur {
                    self.cur += 1;
                    Some(n.children[self.cur - 1])
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// Navigate the siblings of a [Node]. Nodes may be navigated before (preceding) or after (following) the current [Node].
pub struct Siblings {
    t: TreeIndex,
    parent: Index,
    cur: usize,
    dir: i16,
}

impl Siblings {
    fn new(n: Index, t: TreeIndex, dir: i16, f: &Forest) -> Siblings {
        let d = f.get_ref(t).unwrap();
        let nc = d.get(n).unwrap();
        let pnc = d.get(nc.parent.unwrap().0).unwrap();
        let cur = pnc
            .children
            .iter()
            .enumerate()
            .skip_while(|(_, i)| i.0 != n)
            .nth(0)
            .map(|(e, _)| e)
            .unwrap();
        Siblings {
            t,
            parent: nc.parent.unwrap().0,
            dir,
            cur: cur,
        }
    }
    pub fn next(&mut self, f: &Forest) -> Option<Node> {
        if let Some(d) = f.get_ref(self.t) {
            if let Some(n) = d.get(self.parent) {
                if self.dir > 0 && n.children.len() > self.cur + 1 {
                    self.cur += 1;
                    Some(n.children[self.cur])
                } else if self.dir < 0 && self.cur > 0 {
                    self.cur -= 1;
                    Some(n.children[self.cur])
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// Navigate the attributes of a [Node]. The order in which the attributes are visited is undefined.
pub struct Attributes<'a> {
    it: Iter<'a, QualifiedName, Node>,
}

impl<'a> Attributes<'a> {
    fn new(i: Index, d: &'a Tree) -> Attributes {
        Attributes {
            it: d.get(i).unwrap().attributes.iter(),
        }
    }
    pub fn next(&mut self) -> Option<Node> {
        self.it.next().map(|(_, n)| *n)
    }
}

/// The content of a [Node].
#[derive(Clone, Default)]
pub struct NodeContent {
    t: NodeType,
    name: Option<QualifiedName>,
    v: Option<Value>,
    parent: Option<Node>,                     // The document node has no parent
    attributes: HashMap<QualifiedName, Node>, // for non-elements nodes this is always. Should this be an Option?
    children: Vec<Node>, // for non-element nodes this is always empty. Should this be an Option?
}

impl NodeContent {
    /// Create a NodeContent of the given type
    pub fn new(t: NodeType) -> Self {
        NodeContent {
            t,
            ..Default::default()
        }
    }
    /// Return the type of the node
    pub fn node_type(&self) -> NodeType {
        self.t
    }
    /// Return the name of the node, if it has a name
    pub fn name(&self) -> &Option<QualifiedName> {
        &self.name
    }
    /// Return the value of the node, if it has a value
    pub fn value(&self) -> &Option<Value> {
        &self.v
    }
}

/// A builder for a [Node].
struct NodeBuilder(NodeContent);

impl NodeBuilder {
    /// Start building a [Node]
    pub fn new(t: NodeType) -> Self {
        NodeBuilder(NodeContent::new(t))
    }
    /// Set the name of the [Node]
    pub fn name(mut self, qn: QualifiedName) -> Self {
        self.0.name = Some(qn);
        self
    }
    /// Set the value of the [Node]. Replaces the previous value, if the node had one.
    pub fn value(mut self, v: Value) -> Self {
        self.0.v = Some(v);
        self
    }
    /// Complete building the [Node]
    pub fn build(self) -> NodeContent {
        self.0
    }
}

/// An iterator over ancestor nodes (for future use)
pub trait AncestorIterator {
    type Node;
    fn next(&mut self, t: Tree) -> Option<Self::Node>;
}

/// An iterator over child nodes (for future use)
pub trait ChildIterator {
    type Node;
    fn next(&mut self, t: Tree) -> Option<Self::Node>;
}

/// An iterator over child nodes of a [Tree] (for future use)
pub trait DocChildIterator {
    type Node;
    fn next(&mut self, t: Tree) -> Option<Self::Node>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emptydoc() {
        let mut f = Forest::new();
        f.plant_tree();
        assert!(true)
    }

    #[test]
    fn root_element() {
        let mut f = Forest::new();
        let ti = f.plant_tree();
        let e = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Test")))
            .expect("unable to create element node");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(e)
            .expect("unable to add node to doc");
        assert_eq!(e.to_xml(&f), "<Test></Test>")
    }

    #[test]
    fn add_element() {
        let mut f = Forest::new();
        let ti = f.plant_tree();
        let e = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Test")))
            .expect("unable to create element node");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(e)
            .expect("unable to add node to doc");
        let l1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l1).expect("unable to append node");
        assert_eq!(e.to_xml(&f), "<Test><Level-1></Level-1></Test>")
    }

    #[test]
    fn add_text() {
        let mut f = Forest::new();
        let ti = f.plant_tree();
        let e = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Test")))
            .expect("unable to create element node");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(e)
            .expect("unable to add node to doc");
        let l1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l1).expect("unable to append node");
        let txt = f
            .get_ref_mut(ti)
            .unwrap()
            .new_text(Value::from("this is a test"))
            .expect("unable to create text node");
        l1.append_child(&mut f, txt).expect("unable to append node");
        assert_eq!(
            e.to_xml(&f),
            "<Test><Level-1>this is a test</Level-1></Test>"
        )
    }

    #[test]
    fn add_attribute() {
        let mut f = Forest::new();
        let ti = f.plant_tree();
        let e = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Test")))
            .expect("unable to create element node");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(e)
            .expect("unable to add node to doc");
        let l1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l1).expect("unable to append node");
        let txt = f
            .get_ref_mut(ti)
            .unwrap()
            .new_attribute(
                QualifiedName::new(None, None, String::from("data")),
                Value::from("this is a test"),
            )
            .expect("unable to create text node");
        l1.add_attribute(&mut f, txt)
            .expect("unable to add attribute");
        assert_eq!(
            e.to_xml(&f),
            "<Test><Level-1 data='this is a test'></Level-1></Test>"
        )
    }

    #[test]
    fn add_comment() {
        let mut f = Forest::new();
        let ti = f.plant_tree();
        let e = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Test")))
            .expect("unable to create element node");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(e)
            .expect("unable to add node to doc");
        let l1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l1).expect("unable to append node");
        let c = f
            .get_ref_mut(ti)
            .unwrap()
            .new_comment(Value::from("this is a comment"))
            .expect("unable to create comment node");
        l1.append_child(&mut f, c).expect("unable to append node");
        assert_eq!(
            e.to_xml(&f),
            "<Test><Level-1><!--this is a comment--></Level-1></Test>"
        )
    }

    #[test]
    fn add_pi() {
        let mut f = Forest::new();
        let ti = f.plant_tree();
        let e = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Test")))
            .expect("unable to create element node");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(e)
            .expect("unable to add node to doc");
        let l1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l1).expect("unable to append node");
        let pi = f
            .get_ref_mut(ti)
            .unwrap()
            .new_processing_instruction(
                QualifiedName::new(None, None, String::from("testPI")),
                Value::from("this is a PI"),
            )
            .expect("unable to create processing instruction node");
        l1.append_child(&mut f, pi).expect("unable to append node");
        assert_eq!(
            e.to_xml(&f),
            "<Test><Level-1><?testPI this is a PI?></Level-1></Test>"
        )
    }

    #[test]
    fn remove() {
        let mut f = Forest::new();
        let ti = f.plant_tree();
        let e = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Test")))
            .expect("unable to create element node");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(e)
            .expect("unable to add node to doc");
        let l1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l1).expect("unable to append node");
        let t1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_text(Value::from("one"))
            .expect("unable to create text node");
        l1.append_child(&mut f, t1).expect("unable to append node");
        let l2 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l2).expect("unable to append node");
        let t2 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_text(Value::from("two"))
            .expect("unable to create text node");
        l2.append_child(&mut f, t2).expect("unable to append node");

        assert_eq!(
            e.to_xml(&f),
            "<Test><Level-1>one</Level-1><Level-1>two</Level-1></Test>"
        );
        l1.remove(&mut f).expect("unable to remove node");
        assert_eq!(e.to_xml(&f), "<Test><Level-1>two</Level-1></Test>");
    }

    #[test]
    fn children() {
        let mut f = Forest::new();
        let ti = f.plant_tree();
        let e = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Test")))
            .expect("unable to create element node");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(e)
            .expect("unable to add node to doc");
        let l1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l1).expect("unable to append node");
        let t1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_text(Value::from("one"))
            .expect("unable to create text node");
        l1.append_child(&mut f, t1).expect("unable to append node");
        let l2 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l2).expect("unable to append node");
        let t2 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_text(Value::from("two"))
            .expect("unable to create text node");
        l2.append_child(&mut f, t2).expect("unable to append node");

        assert_eq!(
            e.to_xml(&f),
            "<Test><Level-1>one</Level-1><Level-1>two</Level-1></Test>"
        );

        let mut children = e.child_iter();
        assert_eq!(children.next(&f), Some(l1));
        assert_eq!(children.next(&f), Some(l2));
        assert_eq!(children.next(&f), None)
    }

    #[test]
    fn ancestors() {
        let mut f = Forest::new();
        let ti = f.plant_tree();
        let e = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Test")))
            .expect("unable to create element node");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(e)
            .expect("unable to add node to doc");
        let l1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l1).expect("unable to append node");
        let t1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_text(Value::from("one"))
            .expect("unable to create text node");
        l1.append_child(&mut f, t1).expect("unable to append node");
        let l2 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l2).expect("unable to append node");
        let t2 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_text(Value::from("two"))
            .expect("unable to create text node");
        l2.append_child(&mut f, t2).expect("unable to append node");

        assert_eq!(
            e.to_xml(&f),
            "<Test><Level-1>one</Level-1><Level-1>two</Level-1></Test>"
        );

        let mut ancestors = t2.ancestor_iter();
        assert_eq!(ancestors.next(&f), Some(l2));
        assert_eq!(ancestors.next(&f), Some(e));
        assert_eq!(ancestors.next(&f), None)
    }

    #[test]
    fn parent() {
        let mut f = Forest::new();
        let ti = f.plant_tree();
        let e = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Test")))
            .expect("unable to create element node");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(e)
            .expect("unable to add node to doc");
        let l1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l1).expect("unable to append node");
        let t1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_text(Value::from("one"))
            .expect("unable to create text node");
        l1.append_child(&mut f, t1).expect("unable to append node");
        let l2 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l2).expect("unable to append node");
        let t2 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_text(Value::from("two"))
            .expect("unable to create text node");
        l2.append_child(&mut f, t2).expect("unable to append node");

        assert_eq!(
            e.to_xml(&f),
            "<Test><Level-1>one</Level-1><Level-1>two</Level-1></Test>"
        );

        assert_eq!(t2.parent(&f), Some(l2));
    }

    #[test]
    fn following_sibling() {
        let mut f = Forest::new();
        let ti = f.plant_tree();
        let e = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Test")))
            .expect("unable to create element node");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(e)
            .expect("unable to add node to doc");
        let l1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l1).expect("unable to append node");
        let t1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_text(Value::from("one"))
            .expect("unable to create text node");
        l1.append_child(&mut f, t1).expect("unable to append node");
        let l2 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l2).expect("unable to append node");
        let t2 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_text(Value::from("two"))
            .expect("unable to create text node");
        l2.append_child(&mut f, t2).expect("unable to append node");

        assert_eq!(
            e.to_xml(&f),
            "<Test><Level-1>one</Level-1><Level-1>two</Level-1></Test>"
        );

        let mut follow = l1.next_iter(&f);
        assert_eq!(follow.next(&f), Some(l2));
        assert_eq!(follow.next(&f), None)
    }

    #[test]
    fn preceding_sibling() {
        let mut f = Forest::new();
        let ti = f.plant_tree();
        let e = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Test")))
            .expect("unable to create element node");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(e)
            .expect("unable to add node to doc");
        let l1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l1).expect("unable to append node");
        let t1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_text(Value::from("one"))
            .expect("unable to create text node");
        l1.append_child(&mut f, t1).expect("unable to append node");
        let l2 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l2).expect("unable to append node");
        let t2 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_text(Value::from("two"))
            .expect("unable to create text node");
        l2.append_child(&mut f, t2).expect("unable to append node");

        assert_eq!(
            e.to_xml(&f),
            "<Test><Level-1>one</Level-1><Level-1>two</Level-1></Test>"
        );

        let mut pre = l2.prev_iter(&f);
        assert_eq!(pre.next(&f), Some(l1));
        assert_eq!(pre.next(&f), None)
    }

    #[test]
    fn descendants() {
        let mut f = Forest::new();
        let ti = f.plant_tree();
        let e = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Test")))
            .expect("unable to create element node");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(e)
            .expect("unable to add node to doc");
        let g = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Another")))
            .expect("unable to create element node");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(g)
            .expect("unable to add node to doc");
        let l1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l1).expect("unable to append node");
        let t1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_text(Value::from("one"))
            .expect("unable to create text node");
        l1.append_child(&mut f, t1).expect("unable to append node");
        let l2 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l2).expect("unable to append node");
        let t2 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_text(Value::from("two"))
            .expect("unable to create text node");
        l2.append_child(&mut f, t2).expect("unable to append node");

        assert_eq!(
            e.to_xml(&f),
            "<Test><Level-1>one</Level-1><Level-1>two</Level-1></Test>"
        );

        let mut desc = e.descend_iter(&f);
        assert_eq!(desc.next(&f), Some(l1));
        assert_eq!(desc.next(&f), Some(t1));
        assert_eq!(desc.next(&f), Some(l2));
        assert_eq!(desc.next(&f), Some(t2));
        assert_eq!(desc.next(&f), None)
    }

    #[test]
    fn get_first_element() {
        let mut f = Forest::new();
        let ti = f.plant_tree();
        let cm = f
            .get_ref_mut(ti)
            .unwrap()
            .new_comment(Value::from(" not an element "))
            .expect("unable to create comment");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(cm)
            .expect("unable to add comment node to doc");
        let e = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Test")))
            .expect("unable to create element node");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(e)
            .expect("unable to add node to doc");
        let g = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Another")))
            .expect("unable to create element node");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(g)
            .expect("unable to add node to doc");
        assert_eq!(
            f.get_ref(ti)
                .unwrap()
                .get_doc_node()
                .get_first_element(&f)
                .unwrap(),
            e
        )
    }

    #[test]
    fn serialise_1() {
        let mut f = Forest::new();
        let ti = f.plant_tree();
        let e = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Test")))
            .expect("unable to create element node");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(e)
            .expect("unable to add node to doc");
        let g = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Another")))
            .expect("unable to create element node");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(g)
            .expect("unable to add node to doc");
        let l1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l1).expect("unable to append node");
        let t1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_text(Value::from("one"))
            .expect("unable to create text node");
        l1.append_child(&mut f, t1).expect("unable to append node");
        let l2 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element node");
        e.append_child(&mut f, l2).expect("unable to append node");
        let t2 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_text(Value::from("two"))
            .expect("unable to create text node");
        l2.append_child(&mut f, t2).expect("unable to append node");

        assert_eq!(
            e.to_xml(&f),
            "<Test><Level-1>one</Level-1><Level-1>two</Level-1></Test>"
        );
        let mut od = OutputDefinition::new();
        od.set_indent(true);
        assert_eq!(
            e.to_xml_with_options(&f, &od),
            "<Test>
  <Level-1>one</Level-1>
  <Level-1>two</Level-1>
</Test>"
        );
    }

    #[test]
    fn serialise_2() {
        let mut f = Forest::new();
        let ti = f.plant_tree();
        let e = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(
                Some(String::from("http://testing.org/ns")),
                Some(String::from("tst")),
                String::from("Test"),
            ))
            .expect("unable to create element node");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(e)
            .expect("unable to add node to doc");
        let g = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(
                Some(String::from("http://testing.org/ns")),
                Some(String::from("tst")),
                String::from("Another"),
            ))
            .expect("unable to create element node");
        f.get_ref_mut(ti)
            .unwrap()
            .push_doc_node(g)
            .expect("unable to add node to doc");
        let l1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(
                Some(String::from("http://testing.org/ns")),
                Some(String::from("tst")),
                String::from("Level-1"),
            ))
            .expect("unable to create element node");
        e.append_child(&mut f, l1).expect("unable to append node");
        let t1 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_text(Value::from("one"))
            .expect("unable to create text node");
        l1.append_child(&mut f, t1).expect("unable to append node");
        let l2 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_element(QualifiedName::new(
                Some(String::from("http://testing.org/ns")),
                Some(String::from("tst")),
                String::from("Level-1"),
            ))
            .expect("unable to create element node");
        e.append_child(&mut f, l2).expect("unable to append node");
        let t2 = f
            .get_ref_mut(ti)
            .unwrap()
            .new_text(Value::from("two"))
            .expect("unable to create text node");
        l2.append_child(&mut f, t2).expect("unable to append node");

        assert_eq!(e.to_xml(&f), "<tst:Test xmlns:tst='http://testing.org/ns'><tst:Level-1>one</tst:Level-1><tst:Level-1>two</tst:Level-1></tst:Test>");
        let mut od = OutputDefinition::new();
        od.set_indent(true);
        assert_eq!(
            e.to_xml_with_options(&f, &od),
            "<tst:Test xmlns:tst='http://testing.org/ns'>
  <tst:Level-1>one</tst:Level-1>
  <tst:Level-1>two</tst:Level-1>
</tst:Test>"
        );
    }

    #[test]
    fn parse() {
        let mut f = Forest::new();
        let ti = f
            .grow_tree(
                "<Test><empty/>
<data mode='mixed'>This contains <i>mixed</i> content.</data>
<special>Some escaped chars &lt;&amp;&gt;</special>
</Test>",
            )
            .expect("unable to parse");
        assert_eq!(
            f.get_ref(ti)
                .unwrap()
                .get_doc_node()
                .child_iter()
                .next(&f)
                .unwrap()
                .to_xml(&f),
            "<Test><empty></empty>
<data mode='mixed'>This contains <i>mixed</i> content.</data>
<special>Some escaped chars <&></special>
</Test>"
        )
    }

    #[test]
    fn deep_copy_1() {
        let mut f = Forest::new();
        let t1 = f
            .grow_tree("<Test><one/><two/><three/></Test>")
            .expect("unable to parse document 1");
        let t1root = f
            .get_ref(t1)
            .unwrap()
            .get_doc_node()
            .child_iter()
            .next(&f)
            .unwrap();
        let mut cit = t1root.child_iter();
        let _t1one = cit.next(&f).unwrap();
        let t1two = cit.next(&f).unwrap();
        let t1three = cit.next(&f).unwrap();

        t1two
            .insert_before(&mut f, t1three)
            .expect("unable to insert node");
        assert_eq!(
            t1root.to_xml(&f),
            "<Test><one></one><three></three><two></two></Test>"
        );
    }

    #[test]
    fn deep_copy_2() {
        let mut f = Forest::new();
        let t1 = f
            .grow_tree("<Test><one/><two/><three/></Test>")
            .expect("unable to parse document 1");
        let t2 = f
            .grow_tree("<Another><test>document</test></Another>")
            .expect("unable to parse document 1");
        let t1root = f
            .get_ref(t1)
            .unwrap()
            .get_doc_node()
            .child_iter()
            .next(&f)
            .unwrap();
        let mut t1it = t1root.child_iter();
        let _t1one = t1it.next(&f).unwrap();
        let t1two = t1it.next(&f).unwrap();
        let t2root = f
            .get_ref(t2)
            .unwrap()
            .get_doc_node()
            .child_iter()
            .next(&f)
            .unwrap();
        t1two
            .insert_before(&mut f, t2root)
            .expect("unable to insert node");
        assert_eq!(t1root.to_xml(&f), "<Test><one></one><Another><test>document</test></Another><two></two><three></three></Test>");
    }

    #[test]
    fn deep_copy_3() {
        let mut f = Forest::new();
        let t1 = f
            .grow_tree("<Test><one/><two/><three/></Test>")
            .expect("unable to parse document 1");
        let t2 = f
            .grow_tree("<Another><test>document</test></Another>")
            .expect("unable to parse document 1");
        let t1root = f
            .get_ref(t1)
            .unwrap()
            .get_doc_node()
            .child_iter()
            .next(&f)
            .unwrap();
        let mut t1it = t1root.child_iter();
        let _t1one = t1it.next(&f).unwrap();
        let t1two = t1it.next(&f).unwrap();
        let t2root = f
            .get_ref(t2)
            .unwrap()
            .get_doc_node()
            .child_iter()
            .next(&f)
            .unwrap();
        t1two
            .append_child(&mut f, t2root)
            .expect("unable to append node");
        assert_eq!(t1root.to_xml(&f), "<Test><one></one><two><Another><test>document</test></Another></two><three></three></Test>");
    }
}
