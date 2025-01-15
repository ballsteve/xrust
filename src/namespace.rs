//! Support for XML Namespaces
//!
//! The [NamespaceMap] object represents a static mapping of prefix to namespace URI.
//! Since namespaces don't change once they are declared, this object is usually Rc-shared.
//! Namespace prefixes and URIs are interned in a Slotmap.

use crate::qname::Internment;
use crate::value::Value;
use slotmap::DefaultKey;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
//use std::rc::Rc;

/// In some circumstances, a transformation must resolve a qualified name.
/// To do this, it must have a copy of the in-scope namespaces.
/// This type represents a mapping from prefix to Namespace URI.
/// The "None" prefix is for the default namespace.
pub struct NamespaceMap(HashMap<Option<DefaultKey>, DefaultKey>);
// TODO: should the default namespace be represented by the empty string prefix?

impl NamespaceMap {
    /// Create a new namespace mapping.
    pub fn new(intern: &mut Internment) -> Self {
        let mut map = HashMap::new();
        map.insert(
            Some(intern.0.insert("xml".to_string())),
            intern
                .0
                .insert("http://www.w3.org/XML/1998/namespace".to_string()),
        );
        NamespaceMap(map)
    }
    /// Insert a mapping into the map.
    pub fn insert(&mut self, prefix: Option<DefaultKey>, uri: DefaultKey) -> Option<DefaultKey> {
        self.0.insert(prefix, uri)
    }
    /// Lookup a prefix in the map, returning the namespace URI.
    pub fn get(&self, prefix: &Option<DefaultKey>) -> Option<DefaultKey> {
        self.0.get(prefix).cloned()
    }
    /// Iterate over mappings. Each item is a (prefix,namespace URI) pair.
    pub fn iter(&self) -> Iter<Option<DefaultKey>, DefaultKey> {
        self.0.iter()
    }
}
