//! Support for XML Namespaces
//!
//! The [NamespaceMap] object represents a static mapping of prefix to namespace URI.
//! Since namespaces don't change once they are declared, this object is usually Rc-shared.
//! Namespace prefixes and URIs are interned in a Slotmap.

use crate::qname_in::{Internment, QualifiedName};
use lasso::Interner;
use std::collections::hash_map::Iter;
use std::collections::HashMap;

/// In some circumstances, a transformation must resolve a qualified name.
/// To do this, it must have a copy of the in-scope namespaces.
/// This type represents a mapping from prefix to Namespace URI.
/// The "None" prefix is for the default namespace.
pub struct NamespaceMap(HashMap<Option<QualifiedName>, QualifiedName>);
// TODO: should the default namespace be represented by the empty string prefix?

impl NamespaceMap {
    /// Create a new namespace mapping.
    pub fn new<'i, I: Interner<QualifiedName>>(intern: &'i mut Internment<'i, I>) -> Self {
        let mut map = HashMap::new();
        map.insert(
            Some(intern.get_or_intern("xml")),
            intern.get_or_intern("http://www.w3.org/XML/1998/namespace"),
        );
        NamespaceMap(map)
    }
    /// Insert a mapping into the map.
    pub fn insert(
        &mut self,
        prefix: Option<QualifiedName>,
        uri: QualifiedName,
    ) -> Option<QualifiedName> {
        self.0.insert(prefix, uri)
    }
    /// Lookup a prefix in the map, returning the namespace URI.
    pub fn get(&self, prefix: &Option<QualifiedName>) -> Option<QualifiedName> {
        self.0.get(prefix).cloned()
    }
    /// Iterate over mappings. Each item is a (prefix,namespace URI) pair.
    pub fn iter(&self) -> Iter<Option<QualifiedName>, QualifiedName> {
        self.0.iter()
    }
}
