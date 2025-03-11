//! Support for XML Namespaces
//!
//! The [NamespaceMap] object represents a static mapping of prefix to namespace URI.
//! Since namespaces don't change once they are declared, this object is usually Rc-shared.
//! Namespace prefixes and URIs are interned in a Slotmap.

use crate::qname_in::{Internment, QualifiedName};
use lasso::{Interner, LargeSpur};
use std::collections::hash_map::Iter;
use std::collections::HashMap;

/// In some circumstances, a transformation must resolve a qualified name.
/// To do this, it must have a copy of the in-scope namespaces.
/// This type represents a mapping from prefix to Namespace URI.
/// The "None" prefix is for the default namespace.
pub struct NamespaceMap(HashMap<Option<LargeSpur>, LargeSpur>);
// TODO: should the default namespace be represented by the empty string prefix?

impl NamespaceMap {
    /// Create a new namespace mapping.
    pub fn new<I: Interner<LargeSpur>>(intern: &Internment<I>) -> Self {
        let mut map = HashMap::new();
        map.insert(
            Some(intern.borrow().get("xml").unwrap()),
            intern
                .borrow()
                .get("http://www.w3.org/XML/1998/namespace")
                .unwrap(),
        );
        NamespaceMap(map)
    }
    /// Insert a mapping into the map.
    pub fn insert<I: Interner<LargeSpur>>(
        &mut self,
        intern: &Internment<I>,
        prefix: Option<String>,
        uri: String,
    ) -> Option<LargeSpur> {
        let mut interner = intern.borrow_mut();
        let prefix_key = prefix.map(|p| interner.get_or_intern(p.as_str()));
        let uri_key = interner.get_or_intern(uri.as_str());
        self.0.insert(prefix_key, uri_key)
    }
    /// Lookup a prefix in the map, returning the namespace URI.
    pub fn get(&self, prefix: &Option<LargeSpur>) -> Option<LargeSpur> {
        self.0.get(prefix).cloned()
    }
    /// Iterate over mappings. Each item is a (prefix,namespace URI) pair.
    pub fn iter(&self) -> Iter<Option<LargeSpur>, LargeSpur> {
        self.0.iter()
    }
}
