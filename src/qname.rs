//! # xrust::qname
//!
//! Support for Qualified Names.

use core::hash::{Hash, Hasher};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct QualifiedName {
    nsuri: Option<String>,
    prefix: Option<String>,
    localname: String,
}

// TODO: we may need methods that return a string slice, rather than a copy of the string
impl QualifiedName {
    pub fn new(nsuri: Option<String>, prefix: Option<String>, localname: String) -> QualifiedName {
        QualifiedName {
            nsuri,
            prefix,
            localname,
        }
    }
    pub fn as_ref(&self) -> &Self {
        self
    }
    pub fn get_nsuri(&self) -> Option<String> {
        self.nsuri.clone()
    }
    pub fn get_nsuri_ref(&self) -> Option<&str> {
        match self.nsuri {
            Some(ref n) => Some(&n),
            None => None,
        }
    }
    pub fn get_prefix(&self) -> Option<String> {
        self.prefix.clone()
    }
    pub fn get_localname(&self) -> String {
        self.localname.clone()
    }
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        self.prefix.as_ref().map_or((), |p| {
            result.push_str(p.as_str());
            result.push(':');
        });
        result.push_str(self.localname.as_str());
        result
    }
}

pub type QHash<T> = HashMap<QualifiedName, T>;

impl PartialEq for QualifiedName {
    // Only the namespace URI and local name have to match
    fn eq(&self, other: &QualifiedName) -> bool {
        self.nsuri.as_ref().map_or_else(
            || {
                other
                    .nsuri
                    .as_ref()
                    .map_or_else(|| self.localname.eq(other.localname.as_str()), |_| false)
            },
            |ns| {
                other.nsuri.as_ref().map_or_else(
                    || false,
                    |ons| ns.eq(ons.as_str()) && self.localname.eq(other.localname.as_str()),
                )
            },
        )
    }
}
impl Eq for QualifiedName {}

impl Hash for QualifiedName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.nsuri.as_ref().map(|ns| ns.hash(state));
        self.localname.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unqualified() {
        assert_eq!(
            QualifiedName::new(None, None, "foo".to_string()).to_string(),
            "foo"
        )
    }
    #[test]
    fn qualified() {
        assert_eq!(
            QualifiedName::new(
                Some("http://example.org/whatsinaname/".to_string()),
                Some("x".to_string()),
                "foo".to_string()
            )
            .to_string(),
            "x:foo"
        )
    }
    #[test]
    fn hashmap() {
        let mut h = QHash::<String>::new();
        h.insert(
            QualifiedName::new(None, None, "foo".to_string()),
            String::from("this is unprefixed foo"),
        );
        h.insert(
            QualifiedName::new(
                Some("http://example.org/whatsinaname/".to_string()),
                Some("x".to_string()),
                "foo".to_string(),
            ),
            "this is x:foo".to_string(),
        );
        h.insert(
            QualifiedName::new(
                Some("http://example.org/whatsinaname/".to_string()),
                Some("y".to_string()),
                "bar".to_string(),
            ),
            "this is y:bar".to_string(),
        );

        assert_eq!(h.len(), 3);
        assert_eq!(
            h.get(&QualifiedName {
                nsuri: Some("http://example.org/whatsinaname/".to_string()),
                prefix: Some("x".to_string()),
                localname: "foo".to_string()
            }),
            Some(&"this is x:foo".to_string())
        );
        assert_eq!(
            h.get(&QualifiedName {
                nsuri: None,
                prefix: None,
                localname: "foo".to_string()
            }),
            Some(&"this is unprefixed foo".to_string())
        );
    }
}
