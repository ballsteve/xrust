//! Support for Qualified Names.
//! Names are interned for speedy equality checks (compare keys, rather than characters).
//! This also applies to local names, prefixes, and XML Namespace URIs.
//! The intern key for a QualifiedName is its URI Qualified Name.

//use crate::item::Node;
use crate::namespace::NamespaceMap;
use crate::parser::xml::qname::eqname;
use crate::parser::ParserState;
use crate::trees::nullo::Nullo;
//use crate::value::Value;
use crate::xdmerror::{Error, ErrorKind};
use core::hash::{Hash, Hasher};
use lasso::{Interner, Key, LargeSpur, Resolver};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::rc::Rc;

pub type Internment<I: Interner<LargeSpur>> = Rc<RefCell<I>>;

/// Create an internment. This is simply an Interner in an interior mutability setup.
pub fn new_internment<I: Interner<LargeSpur>>(mut intern: I) -> Internment<I> {
    let _ = intern.get_or_intern_static("");
    let xml_uri = "http://www.w3.org/XML/1998/namespace";
    let _ = intern.get_or_intern_static(xml_uri);
    let uriqualified = uri_qualifiedname(xml_uri, "^");
    let _ = intern.get_or_intern(uriqualified.as_str());
    let _ = intern.get_or_intern_static("xml");
    let _ = intern.get_or_intern_static("^");
    Rc::new(RefCell::new(intern))
}

fn uri_qualifiedname(uri: &str, name: &str) -> String {
    format!("Q{}{}{}{}", "{", uri, "}", name)
}

/*/// Create a [QualifiedName] by parsing a string.
/// To resolve XML Namespaces a [NamespaceMap] may be given.
/// It is an error if the name is prefixed and the prefix cannot be resolved to a namespace declaration.
pub fn parse(
    source: impl Into<String>,
    ns_map: Option<&NamespaceMap>,
    intern: &mut Internment,
) -> Result<QualifiedName, Error> {
    let state: ParserState<Nullo> = ParserState::new(None, None, None);
    match eqname()((source.into().as_str(), state)) {
        Ok((_, qn)) => Ok(qn),
        Err(_) => Err(Error::new(
            ErrorKind::ParseError,
            String::from("unable to parse qualified name"),
        )),
    }
}*/

/// An XML Namespaces Qualified Name. An unprefixed Name has no prefix or namespace URI.
/// A prefixed Qualified Name has all three components.
/// An unprefixed Name in the presence of a default namespace will have a namespace URI and local part, but no prefix.
/// Applications may use string interning. In this case, the QN will store a key for the interned name. The QN's URI Qualified Name will be used as the string.
/// When comparing two QNs, using an intern key will be faster.
#[derive(Clone)]
pub struct QualifiedName {
    nsuri: Option<String>,
    prefix: Option<String>,
    localname: String,
    key: Option<LargeSpur>,
}
// TODO: we may need methods that return a string slice, rather than a copy of the string
impl QualifiedName {
    /// Create a QualifiedName (QN), adding it to the internment.
    /// A QN consists of a Namespace URI and a local name.
    /// QNs may optionally have a prefix.
    /// It is not valid for a QN to have a prefix but no Namespace URI.
    /// Both prefix and Namespace URI may not be empty strings.
    pub fn new_interned<I: Interner<LargeSpur>>(
        intern: &Internment<I>,
        nsuri: Option<impl Into<String>>,
        prefix: Option<impl Into<String>>,
        localname: impl Into<String>,
    ) -> Result<Self, Error> {
        match (nsuri, prefix) {
            (None, Some(_)) => Err(Error::new(
                ErrorKind::DynamicAbsent,
                "missing Namespace URI",
            )),
            (Some(n), Some(p)) => {
                let inlocalname = localname.into();
                let inn = n.into();
                let uriqualified =
                    uri_qualifiedname(inn.clone().as_ref(), inlocalname.clone().as_ref());
                let k = intern.borrow_mut().get_or_intern(uriqualified.as_str());

                Ok(QualifiedName {
                    nsuri: Some(inn),
                    prefix: Some(p.into()),
                    localname: inlocalname,
                    key: Some(k),
                })
            }
            (Some(n), None) => {
                let inlocalname = localname.into();
                let inn = n.into();
                let uriqualified =
                    uri_qualifiedname(inn.clone().as_ref(), inlocalname.clone().as_ref());
                let k = intern.borrow_mut().get_or_intern(uriqualified.as_str());

                Ok(QualifiedName {
                    nsuri: Some(inn),
                    prefix: None,
                    localname: inlocalname,
                    key: Some(k),
                })
            }
            (None, None) => {
                // An unprefixed QName
                let the_localname = localname.into();
                let k = intern.borrow_mut().get_or_intern(the_localname.as_ref());
                Ok(QualifiedName {
                    nsuri: None,
                    prefix: None,
                    localname: the_localname,
                    key: Some(k),
                })
            }
        }
    }
    /// Builds a QualifiedName from String parts
    pub fn new(
        nsuri: Option<impl Into<String>>,
        prefix: Option<impl Into<String>>,
        localname: impl Into<String>,
    ) -> Result<Self, Error> {
        if nsuri.is_none() && prefix.is_some() {
            Err(Error::new(
                ErrorKind::DynamicAbsent,
                "missing Namespace URI",
            ))
        } else {
            Ok(QualifiedName {
                nsuri: nsuri.map(|n| n.into()),
                prefix: prefix.map(|p| p.into()),
                localname: localname.into(),
                key: None,
            })
        }
    }
    pub fn as_ref(&self) -> &Self {
        self
    }
    pub fn namespace_uri(&self) -> Option<String> {
        self.nsuri.clone()
    }
    //    pub fn namespace_uri_to_string(&self) -> Option<String> {
    //        self.nsuri.as_ref().map(|x| x.to_string())
    //    }
    pub fn prefix(&self) -> Option<String> {
        self.prefix.clone()
    }
    //    pub fn prefix_to_string(&self) -> Option<String> {
    //        self.prefix.as_ref().map(|x| x.to_string())
    //    }
    pub fn localname(&self) -> String {
        self.localname.clone()
    }
    pub fn key(self) -> Option<LargeSpur> {
        self.key.clone()
    }
    pub fn localname_to_string(&self) -> String {
        self.localname.to_string()
    }
    /// Fully resolve a qualified name. If the qualified name has a prefix but no namespace URI,
    /// then find the prefix in the supplied namespaces and use the corresponding URI.
    /// If the qualified name already has a namespace URI, then this method has no effect.
    /// If the qualified name has no prefix, then this method has no effect.
    /// If an Interner is supplied then the QN will be interned.
    pub fn resolve<F, I: Interner<LargeSpur>>(
        &mut self,
        mapper: F,
        intern_o: Option<&Internment<I>>,
    ) -> Result<(), Error>
    where
        F: Fn(Option<String>) -> Result<String, Error>,
    {
        match (&self.prefix, &self.nsuri) {
            (Some(p), None) => {
                self.nsuri = Some(mapper(Some(p.clone()))?.clone());
                // Now intern
                if let Some(intern) = intern_o {
                    let uriqualified =
                        uri_qualifiedname(self.nsuri.as_ref().unwrap(), self.localname.as_ref());
                    self.key = Some(intern.borrow_mut().get_or_intern(uriqualified.as_str()));
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
    /// Display the QN as a prefixed string
    pub fn to_string(&self) -> String {
        self.prefix
            .as_ref()
            .map_or(String::new(), |s| s.clone() + ":");
        self.localname.clone()
    }
    /// Display the QN as a URI Qualified string
    pub fn to_uri_qualified(&self) -> String {
        self.nsuri.as_ref().map_or(String::new(), |s| {
            "{".to_string() + s.clone().as_ref() + "}"
        });
        self.localname.clone()
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
        let _ = f.write_str(
            self.nsuri
                .as_ref()
                .map_or("--none--".to_string(), |ns| ns.to_string())
                .as_str(),
        );
        let _ = f.write_str(" prefix ");
        let _ = f.write_str(
            self.prefix
                .as_ref()
                .map_or("--none--".to_string(), |p| p.to_string())
                .as_str(),
        );
        let _ = f.write_str(" local part \"");
        let _ = f.write_str(self.localname.to_string().as_str());
        f.write_str("\"")
    }
}

//pub type QHash<T> = HashMap<QualifiedName, T>;

/*
impl PartialEq for QualifiedNameData {
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
*/

/// A partial ordering for QualifiedNames. Unprefixed names are considered to come before prefixed names.
/// If the QNs have keys, then use them for comparison.
impl PartialOrd for QualifiedName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self.key, &other.key, &self.nsuri, &other.nsuri) {
            (Some(s), Some(o), _, _) => {
                if s == o {
                    Some(Ordering::Equal)
                } else {
                    Some(Ordering::Less)
                }
            }
            (_, _, None, None) => self.localname.partial_cmp(&other.localname),
            (_, _, Some(_), None) => Some(Ordering::Greater),
            (_, _, None, Some(_)) => Some(Ordering::Less),
            (_, _, Some(n), Some(m)) => {
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
impl PartialEq for QualifiedName {
    fn eq(&self, other: &QualifiedName) -> bool {
        match (self.key, other.key) {
            (Some(s), Some(o)) => s == o,
            _ => false,
        }
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

/*
/// Parse a string to create a [QualifiedName].
/// QualifiedName ::= (prefix ":")? local-name
impl TryFrom<&str> for QualifiedName {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let state: ParserState<Nullo> = ParserState::new(None, None, None);
        match eqname()((s, state)) {
            Ok((_, qn)) => Ok(qn),
            Err(_) => Err(Error::new(
                ErrorKind::ParseError,
                String::from("unable to parse qualified name"),
            )),
        }
    }
}*/

/*
/// Parse a string to create a [QualifiedName].
/// Resolve prefix against a set of XML Namespace declarations.
/// This method can be used when there is no XSL stylesheet to derive the namespaces.
/// QualifiedName ::= (prefix ":")? local-name
impl TryFrom<(&str, &NamespaceMap, Internment)> for QualifiedName {
    type Error = Error;
    fn try_from(s: (&str, Rc<NamespaceMap>)) -> Result<Self, Self::Error> {
        let state: ParserState<Nullo> = ParserState::new(None, None, None);
        match eqname()((s.0, state)) {
            Ok((_, qn)) => {
                if qn.prefix().is_some() && qn.namespace_uri().is_none() {
                    match s.1.get(&qn.prefix()) {
                        Some(ns) => Ok(QualifiedName::new_from_values(
                            Some(ns),
                            qn.prefix(),
                            qn.localname().clone(),
                        )),
                        _ => Err(Error::new(
                            ErrorKind::Unknown,
                            format!(
                                "unable to match prefix \"{}\"",
                                qn.prefix_to_string().unwrap()
                            ),
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

/// Parse a string to create a [QualifiedName].
/// Resolve prefix against a set of XML Namespace declarations.
/// This method can be used when there is an XSL stylesheet to derive the namespaces.
/// QualifiedName ::= (prefix ":")? local-name
impl<N: Node> TryFrom<(&str, N)> for QualifiedName {
    type Error = Error;
    fn try_from(s: (&str, N)) -> Result<Self, Self::Error> {
        let state: ParserState<Nullo> = ParserState::new(None, None, None);
        match eqname()((s.0, state)) {
            Ok((_, qn)) => {
                if qn.prefix().is_some() && qn.namespace_uri().is_none() {
                    s.1.namespace_iter()
                        .find(|ns| ns.name().localname() == qn.prefix().unwrap())
                        .map_or(
                            Err(Error::new(
                                ErrorKind::DynamicAbsent,
                                format!(
                                    "no namespace matching prefix \"{}\"",
                                    qn.prefix_to_string().unwrap()
                                ),
                            )),
                            |ns| {
                                Ok(QualifiedName::new_from_values(
                                    Some(ns.value()),
                                    qn.prefix(),
                                    qn.localname(),
                                ))
                            },
                        )
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
*/

#[cfg(test)]
mod tests {
    use lasso::Rodeo;

    use super::*;

    #[test]
    fn unqualified_raw() {
        let mut intern = new_internment(Rodeo::new());
        let qn = QualifiedName::new_interned(&intern, None, None, "foo")
            .expect("unable to create qualified name");
        assert_eq!(intern.borrow().resolve(&qn.key().unwrap()), "foo")
    }
    /*#[test]
    fn unqualified_rc() {
        assert_eq!(
            QualifiedName::new_from_values(None, None, Rc::new(Value::from("foo"))).to_string(),
            "foo"
        )
    }*/
    #[test]
    fn qualified_raw() {
        let intern = new_internment(Rodeo::new());
        let qn = QualifiedName::new_interned(
            &intern,
            Some("http://example.org/whatsinaname/"),
            Some("x"),
            "foo",
        )
        .expect("unable to create qualified name");
        let bi = intern.borrow();
        let uriqn = bi.resolve(&qn.key().unwrap());
        assert_eq!(uriqn, "Q{http://example.org/whatsinaname/}foo")
    }
    #[test]
    fn in_eq() {
        let intern = new_internment(Rodeo::new());
        let qn1 = QualifiedName::new_interned(
            &intern,
            Some("http://example.org/whatsinaname/"),
            Some("x"),
            "foo",
        )
        .expect("unable to create first qualified name");
        let qn2 = QualifiedName::new_interned(
            &intern,
            Some("http://example.org/whatsinaname/".to_string()),
            Some("x".to_string()),
            "foo",
        )
        .expect("unable to create second qualified name");
        assert!(qn1 == qn2)
    }
    #[test]
    fn in_ne() {
        let intern = new_internment(Rodeo::new());
        let qn1 = QualifiedName::new_interned(
            &intern,
            Some("http://example.org/whatsinaname/".to_string()),
            Some("x".to_string()),
            "foo",
        )
        .expect("unable to create first qualified name");
        let qn2 = QualifiedName::new_interned(
            &intern,
            Some("http://example.org/whatsinaname/".to_string()),
            Some("y".to_string()),
            "foo",
        )
        .expect("unable to create second qualified name");
        assert_ne!(qn1, qn2)
    }
    /*#[test]
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
    }*/
    /*#[test]
    fn eqname() {
        let mut intern = new_map();
        let qn = parse("Q{http://example.org/bar}foo", None, None, &mut intern)
            .expect("unable to parse qualified name");
        let e = QualifiedName::try_from().expect("unable to parse EQName");
        assert_eq!(e.localname_to_string(), "foo");
        assert_eq!(
            e.namespace_uri_to_string(),
            Some(String::from("http://example.org/bar"))
        );
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
    }*/
}
