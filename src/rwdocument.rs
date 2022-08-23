//! # A mutable tree structure
//!
//! This module provides traits for a mutable tree structure. The nodes of the tree will be Rc-controlled, so that the get_mut method can be used to mutate a node. Therefore, the tree cannot be fully navigable since there can be only one reference to the node for the get_mut call to succeed.
//!
//! This tree structure will be used to create a fully navigable tree - see the [Node] trait in the [Item] module.

use crate::xdmerror::Error;
use crate::qname::QualifiedName;
use crate::value::Value;
use crate::item::NodeType;

/// A mutable document. A document contains [RWNode]s.
pub trait RWDocument {
    type Docitem: RWNode;
    type RWNodeIterator: Iterator<Item=Self::Docitem>;

    fn push_content(&mut self, n: Self::Docitem) -> Result<(), Error>;
    //fn push_prologue(&mut self, n: N) -> Result<(), Error>;
    //fn push_epilogue(&mut self, n: N) -> Result<(), Error>;

    fn content_iter(&self) -> Self::RWNodeIterator;
    //fn prologue_iter(&self) -> Self::RWNodeIterator;
    //fn epilogue_iter(&self) -> Self::RWNodeIterator;

    /// Create an element-type [RWNode] in this document. The new node is not attached to the tree.
    fn new_element(&mut self, qn: QualifiedName) -> Result<Self::Docitem, Error>;
    /// Create a text-type [RWNode] in this document. The new node is not attached to the tree.
    fn new_text(&mut self, v: Value) -> Result<Self::Docitem, Error>;

    fn to_xml(&self) -> String;
}

pub trait RWNode {
    type RWNodeIterator: Iterator<Item=Self>;

    fn node_type(&self) -> NodeType;
    /// Get the name of the node. If the node doesn't have a name, then returns a [QualifiedName] with an empty string for it's localname.
    fn name(&self) -> QualifiedName;
    /// Get the value of the node. If the node doesn't have a value, then returns a [Value] that is an empty string.
    fn value(&self) -> Value;
    /// Get the string value of the node. See XPath ???
    fn to_string(&self) -> String;
    /// Serialise the node as XML
    fn to_xml(&self) -> String;
    // fn to_xml_with_options(&self, od: OutputDefinition) -> String;

    /// An iterator over the children of the node
    fn child_iter(&self) -> Self::RWNodeIterator;
    /// Get the first child of the node, if there is one
    fn first_child(&self) -> Option<Self> where Self: Sized {
	self.child_iter().next()
    }
    /// An iterator over the descendants of the node
    // TODO fn descend_iter(&self) -> Self::NodeIterator;

    /// Append a node to the child list
    fn push(&mut self, n: Self) -> Result<(), Error>;
    // fn add_attribute(&mut self, n: N) -> Result<(), Error>;
}


