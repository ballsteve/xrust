//! Support for Qualified Names.
//! TODO: Intern names for speedy equality checks (compare pointers, rather than characters).

use crate::value::Value;
use crate::item::Node;
use crate::parser::xml::qname::eqname;
use crate::parser::ParserState;
use crate::trees::nullo::Nullo;
use crate::xdmerror::{Error, ErrorKind};
use core::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct QualifiedName {
    nsuri: Option<Rc<Value>>,
    prefix: Option<Rc<Value>>,
    localname: Rc<Value>,
}

// TODO: we may need methods that return a string slice, rather than a copy of the string
impl QualifiedName {
    /// Builds a QualifiedName from String parts
    pub fn new(
        nsuri: Option<String>,
        prefix: Option<String>,
        localname: impl Into<String>,
    ) -> QualifiedName {
        QualifiedName {
            nsuri: nsuri.map(|s| Rc::new(Value::from(s))),
            prefix: prefix.map(|s| Rc::new(Value::from(s))),
            localname: Rc::new(Value::from(localname.into())),
        }
    }
    /// Builds a QualifiedName from shared components
    pub fn new_from_values(
        nsuri: Option<Rc<Value>>,
        prefix: Option<Rc<Value>>,
        localname: Rc<Value>,
    ) -> QualifiedName {
        QualifiedName {
            nsuri,
            prefix,
            localname,
        }
    }
    pub fn as_ref(&self) -> &Self {
        self
    }
    pub fn namespace_uri(&self) -> Option<Rc<Value>> {
        self.nsuri.clone()
    }
    pub fn namespace_uri_to_string(&self) -> Option<String> {
        self.nsuri.as_ref().map(|x| x.to_string())
    }
    pub fn prefix(&self) -> Option<Rc<Value>> {
        self.prefix.clone()
    }
    pub fn prefix_to_string(&self) -> Option<String> {
        self.prefix.as_ref().map(|x| x.to_string())
    }
    pub fn localname(&self) -> Rc<Value> {
        self.localname.clone()
    }
    pub fn localname_to_string(&self) -> String {
        self.localname.to_string()
    }
    /// Fully resolve a qualified name. If the qualified name has a prefix but no namespace URI,
    /// then find the prefix in the supplied namespaces and use the corresponding URI.
    /// If the qualified name already has a namespace URI, then this method has no effect.
    /// If the qualified name has no prefix, then this method has no effect.
    pub fn resolve<N: Node>(
        &mut self,
        namespaces: &Rc<HashMap<Option<Rc<Value>>, N>>,
    ) -> Result<(), Error> {
        match (&self.prefix, &self.nsuri) {
            (Some(p), None) => namespaces.get(&Some(p.clone())).map_or(
                Err(Error::new(
                    ErrorKind::DynamicAbsent,
                    format!("no namespace corresponding to prefix \"{}\"", p),
                )),
                |u| {
                    self.nsuri = Some(u.value().clone());
                    Ok(())
                }
            ),
            _ => Ok(()),
        }
    }
}

impl fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        let _ = self.prefix.as_ref().map_or((), |p| {
            result.push_str(p.to_string().as_str());
            result.push(':');
        });
        result.push_str(self.localname.to_string().as_str());
        f.write_str(result.as_str())
    }
}

impl Debug for QualifiedName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let _ = f.write_str("namespace ");
        let _ = f.write_str(self.nsuri.as_ref().map_or("--none--".to_string(), |ns| ns.to_string()).as_str());
        let _ = f.write_str(" prefix ");
        let _ = f.write_str(self.prefix.as_ref().map_or("--none--".to_string(), |p| p.to_string()).as_str());
        let _ = f.write_str(" local part \"");
        let _ = f.write_str(self.localname.to_string().as_str());
        f.write_str("\"")
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
                    .map_or_else(|| self.localname.eq(&other.localname), |_| false)
            },
            |ns| {
                other.nsuri.as_ref().map_or_else(
                    || false,
                    |ons| ns.eq(ons) && self.localname.eq(&other.localname),
                )
            },
        )
    }
}

/// A partial ordering for QualifiedNames. Unprefixed names are considered to come before prefixed names.
impl PartialOrd for QualifiedName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self.nsuri, &other.nsuri) {
            (None, None) => self.localname.partial_cmp(&other.localname),
            (Some(_), None) => Some(Ordering::Greater),
            (None, Some(_)) => Some(Ordering::Less),
            (Some(n), Some(m)) => {
                if n == m {
                    self.localname.partial_cmp(&other.localname)
                } else {
                    n.partial_cmp(m)
                }
            }
        }
    }
}
impl Ord for QualifiedName {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl Eq for QualifiedName {}

impl Hash for QualifiedName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(ns) = self.nsuri.as_ref() {
            ns.hash(state)
        }
        self.localname.hash(state);
    }
}

/// Parse a string to create a [QualifiedName].
/// QualifiedName ::= (prefix ":")? local-name
impl TryFrom<&str> for QualifiedName {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let state: ParserState<Nullo> = ParserState::new(None, None);
        match eqname()((s, state)) {
            Ok((_, qn)) => Ok(qn),
            Err(_) => Err(Error::new(
                ErrorKind::ParseError,
                String::from("unable to parse qualified name"),
            )),
        }
    }
}

/// Parse a string to create a [QualifiedName].
/// Resolve prefix against a set of XML Namespace declarations
/// QualifiedName ::= (prefix ":")? local-name
impl<N: Node> TryFrom<(&str, &Rc<HashMap<Rc<Value>, N>>)> for QualifiedName {
    type Error = Error;
    fn try_from(s: (&str, &Rc<HashMap<Rc<Value>, N>>)) -> Result<Self, Self::Error> {
        let state: ParserState<Nullo> = ParserState::new(None, None);
        match eqname()((s.0, state)) {
            Ok((_, qn)) => {
                if qn.prefix().is_some() && qn.namespace_uri().is_none() {
                    match s.1.get(&qn.prefix().unwrap()) {
                        Some(ns) => Ok(QualifiedName::new_from_values(
                            Some(ns.value().clone()),
                            qn.prefix(),
                            qn.localname().clone(),
                        )),
                        _ => Err(Error::new(
                            ErrorKind::Unknown,
                            format!("unable to match prefix \"{}\"", qn.prefix_to_string().unwrap()),
                        )),
                    }
                } else {
                    Ok(qn)
                }
            }
            Err(_) => Err(Error::new(
                ErrorKind::ParseError,
                String::from("unable to parse qualified name"),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unqualified_raw() {
        assert_eq!(
            QualifiedName::new(None, None, "foo").to_string(),
            "foo"
        )
    }
    #[test]
    fn unqualified_rc() {
        assert_eq!(
            QualifiedName::new_from_values(None, None, Rc::new(Value::from("foo"))).to_string(),
            "foo"
        )
    }
    #[test]
    fn qualified_raw() {
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
    fn qualified_rc() {
        assert_eq!(
            QualifiedName::new_from_values(
                Some(Rc::new(Value::from("http://example.org/whatsinaname/"))),
                Some(Rc::new(Value::from("x"))),
                Rc::new(Value::from("foo"))
            )
                .to_string(),
            "x:foo"
        )
    }
    #[test]
    fn eqname() {
        let e = QualifiedName::try_from("Q{http://example.org/bar}foo")
            .expect("unable to parse EQName");
        assert_eq!(e.localname_to_string(), "foo");
        assert_eq!(e.namespace_uri_to_string(), Some(String::from("http://example.org/bar")));
        assert_eq!(e.prefix_to_string(), None)
    }
    #[test]
    fn hashmap() {
        let mut h = QHash::<String>::new();
        h.insert(
            QualifiedName::new(None, None, "foo"),
            String::from("this is unprefixed foo"),
        );
        h.insert(
            QualifiedName::new(
                Some("http://example.org/whatsinaname/".to_string()),
                Some("x".to_string()),
                "foo",
            ),
            "this is x:foo".to_string(),
        );
        h.insert(
            QualifiedName::new(
                Some("http://example.org/whatsinaname/".to_string()),
                Some("y".to_string()),
                "bar",
            ),
            "this is y:bar".to_string(),
        );

        assert_eq!(h.len(), 3);
        assert_eq!(
            h.get(&QualifiedName {
                nsuri: Some(Rc::new(Value::from("http://example.org/whatsinaname/"))),
                prefix: Some(Rc::new(Value::from("x"))),
                localname: Rc::new(Value::from("foo")),
            }),
            Some(&"this is x:foo".to_string())
        );
        assert_eq!(
            h.get(&QualifiedName {
                nsuri: None,
                prefix: None,
                localname: Rc::new(Value::from("foo")),
            }),
            Some(&"this is unprefixed foo".to_string())
        );
    }
}
