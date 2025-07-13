//! Support for Qualified Names.
//! Names are interned for speedy equality checks (compare keys, rather than characters).
//! This also applies to local names, prefixes, and XML Namespace URIs.
//! Qualified Names are equal if their local-part and namespace URI both match.
//! However, to serialise a QN it is necessary to know it's prefix (if it has one).
//! NB. A requirement for interning Qualified Names is to not leak memory.
//! To achieve this, entries in the internment must be reference counted.
//! NB. Another requirement is that QNs must be Send+Sync. This means avoiding Rc.
//! NB. Another requirement is that the internment is reentrant, which means using interior mutability.
//! The internment must be Send+Sync, so define a trait to allow the application to provide an appropriate object.

use crate::item::Node;
use crate::namespace::NamespaceMap;
use crate::parser::xml::qname::eqname;
use crate::parser::ParserState;
use crate::trees::nullo::Nullo;
use crate::xdmerror::{Error, ErrorKind};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// A Qualified Name is a triple of (QNKey, PrefixKey, &Internment)
#[derive(Clone)]
pub struct QualifiedName<'i, I: Interner>(QNKey, PrefixKey, &'i I);

impl<'i, I: Interner> QualifiedName<'i, I> {
    /// Create a [QualifiedName] and intern it in the supplied [Interner]
    pub fn new(
        local_part: impl Into<String>,
        namespace_uri: Option<String>,
        prefix: Option<String>,
        interner: &'i I,
    ) -> Self {
        let (pre_key, qn_key) = interner.get_or_intern(local_part.into(), prefix, namespace_uri);
        QualifiedName(qn_key, pre_key, interner)
    }
    pub fn local_part(&self) -> String {
        self.2.local_part(&self.0)
    }
    pub fn namespace_uri(&self) -> Option<String> {
        self.2.namespace_uri(&self.0)
    }
    pub fn prefix(&self) -> Option<String> {
        self.2.prefix(&self.1)
    }
    pub fn interner(&self) -> &'i I {
        self.2
    }
    /// Create a [QualifiedName] by parsing a string.
    /// To resolve XML Namespaces a [NamespaceMap] may be given.
    /// It is an error if the name is prefixed and the prefix cannot be resolved to a namespace declaration.
    pub fn parse(
        source: impl Into<String>,
        ns_map: Option<&NamespaceMap>,
        intern: &'i I,
    ) -> Result<QualifiedName<'i, I>, Error> {
        let state: ParserState<I, Nullo> = ParserState::new(None, None, None, intern);
        let src = source.into();
        let x = match eqname()((src.as_str(), state)) {
            Ok((_, qn)) => {
                if ns_map.is_some() && qn.prefix().is_some() && qn.namespace_uri().is_none() {
                    match ns_map.unwrap().get(&qn.prefix()) {
                        Some(ns) => Ok(QualifiedName::new(
                            qn.local_part(),
                            Some(ns),
                            qn.prefix(),
                            intern,
                        )),
                        _ => Err(Error::new(
                            ErrorKind::Unknown,
                            format!("unable to match prefix \"{}\"", qn.prefix().unwrap()),
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
        };
        x
        // Now resolve the prefix
    }
    /// Display the QN as a URI Qualified string
    pub fn to_uri_qualified(&self) -> UriQualifiedName {
        UriQualifiedName::new(self.2.local_part(&self.0), self.2.namespace_uri(&self.0))
    }
    /// Serialise the [QualifiedName], as (prefix ':')? local-part
    pub fn to_string(&self) -> String {
        self.2.to_string(self.0, self.1)
    }
    /// Fully resolve a qualified name. If the qualified name has a prefix but no namespace URI,
    /// then find the prefix in the supplied namespaces and use the corresponding URI.
    /// If the qualified name already has a non-null namespace URI, then this method has no effect.
    /// If the qualified name has no prefix, then this method has no effect.
    pub fn resolve<F>(&mut self, mapper: F) -> Result<(), Error>
    where
        F: Fn(Option<String>) -> Result<String, Error>,
    {
        self.2.prefix(&self.1).map_or(Ok(()), |p| {
            self.2.namespace_uri(&self.0).map_or_else(
                || {
                    let new_nsuri = mapper(Some(p.clone()))?;
                    // Old configuration of QN is now invalid
                    // So grab a copy of the local-part
                    let local_part = self.2.local_part(&self.0);
                    self.2.decr_ref_count(self.0, self.1);
                    let (a, b) = self.2.get_or_intern(local_part, Some(p), Some(new_nsuri));
                    self.1 = a;
                    self.0 = b;
                    Ok(())
                },
                |_| Ok(()),
            )
        })
    }
}

impl<'i, I: Interner> Drop for QualifiedName<'i, I> {
    fn drop(&mut self) {
        self.2.decr_ref_count(self.0, self.1);
    }
}

impl<'i, I: Interner> fmt::Display for QualifiedName<'i, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        let _ = self.2.prefix(&self.1).as_ref().map_or((), |p| {
            result.push_str(p.to_string().as_str());
            result.push(':');
        });
        result.push_str(self.2.local_part(&self.0).to_string().as_str());
        f.write_str(result.as_str())
    }
}

impl<'i, I: Interner> Debug for QualifiedName<'i, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let _ = f.write_str("namespace ");
        let _ = f.write_str(
            self.2
                .namespace_uri(&self.0)
                .as_ref()
                .map_or("--none--".to_string(), |ns| ns.to_string())
                .as_str(),
        );
        let _ = f.write_str(" prefix ");
        let _ = f.write_str(
            self.2
                .prefix(&self.1)
                .as_ref()
                .map_or("--none--".to_string(), |p| p.to_string())
                .as_str(),
        );
        let _ = f.write_str(" local part \"");
        let _ = f.write_str(self.2.local_part(&self.0).as_str());
        f.write_str("\"")
    }
}

impl<'i, I: Interner> PartialEq for QualifiedName<'i, I> {
    // Only the namespace URI and local name have to match
    fn eq(&self, other: &QualifiedName<'i, I>) -> bool {
        self.0 == other.0
    }
}
impl<'i, I: Interner> Eq for QualifiedName<'i, I> {}

#[derive(Clone, Debug, PartialEq)]
pub struct UriQualifiedName(String);
impl UriQualifiedName {
    pub fn new(local_part: impl Into<String>, ns_uri: Option<impl Into<String>>) -> Self {
        let mut braced = String::new();
        if ns_uri.is_some() {
            braced.push_str("Q{");
            braced.push_str(ns_uri.unwrap().into().as_ref());
            braced.push('}');
        }
        braced.push_str(local_part.into().as_ref());
        UriQualifiedName(braced)
    }
}
impl fmt::Display for UriQualifiedName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}
impl<'i, I: Interner> From<QualifiedName<'i, I>> for UriQualifiedName {
    fn from(qn: QualifiedName<'i, I>) -> Self {
        UriQualifiedName::new(qn.local_part(), qn.namespace_uri())
    }
}

/// An identifier for a Qualified Name.
pub type QNKey = usize;

/// An identifier for a prefixed QN.
pub type PrefixKey = usize;

/// An Interner provides internment of Qualified Names.
pub trait Interner: Clone {
    /// Retrieve the key for a Qualified Name. If the QN was previously unknown, then it is added to the internment. Increments the ref count.
    fn get_or_intern(
        &self,
        local_part: String,
        prefix: Option<String>,
        namespace_uri: Option<String>,
    ) -> (PrefixKey, QNKey);
    /// Decrement the reference count for a QN and prefix.
    /// If the new reference count is zero, then remove the QN from the internment.
    fn decr_ref_count(&self, qn: QNKey, p: PrefixKey);
    /// Serialise the QN: (prefix ':')? local-part
    fn to_string(&self, qn: QNKey, p: PrefixKey) -> String;
    // TODO: URI Qualified serialisation - ('{' namespace-uri '}')? local-part
    /// Get the prefix
    fn prefix(&self, p: &PrefixKey) -> Option<String>;
    /// Get the namespace-URI
    fn namespace_uri(&self, qn: &QNKey) -> Option<String>;
    /// Get the local-part
    fn local_part(&self, qn: &QNKey) -> String;
}

pub struct Internment;

/// A Qualified Name, without prefix.
#[derive(Clone, Debug)]
struct QnDetails {
    local_part: String,
    namespace_uri: Option<String>,
    count: usize,
}
impl QnDetails {
    pub fn new(local_part: String, namespace_uri: Option<String>) -> Self {
        QnDetails {
            local_part,
            namespace_uri,
            count: 1,
        }
    }
}

#[derive(Clone, Debug)]
struct PrefixedQn {
    prefix: Option<String>,
    qn: QNKey,
    count: usize,
}
impl PrefixedQn {
    fn new(prefix: Option<String>, qn: QNKey) -> Self {
        PrefixedQn {
            prefix,
            qn,
            count: 1,
        }
    }
}

/// A simple [Interner] for local use.
#[derive(Clone)]
pub struct LocalInternment {
    qn_keys: RefCell<Vec<QNKey>>,
    prefix_keys: RefCell<Vec<PrefixKey>>,

    // (local-part,namespace-uri,prefix) -> LIKey
    pqn_to_key: RefCell<HashMap<(String, Option<String>, Option<String>), PrefixKey>>,
    // (local-part,namespace-uri) -> QNKey
    qn_to_key: RefCell<HashMap<(String, Option<String>), QNKey>>,

    prefixes: RefCell<HashMap<PrefixKey, PrefixedQn>>,
    details: RefCell<HashMap<QNKey, QnDetails>>,
}

impl LocalInternment {
    pub fn new() -> Self {
        LocalInternment {
            qn_keys: RefCell::new(vec![]),
            prefix_keys: RefCell::new(vec![]),
            pqn_to_key: RefCell::new(HashMap::new()),
            qn_to_key: RefCell::new(HashMap::new()),
            prefixes: RefCell::new(HashMap::new()),
            details: RefCell::new(HashMap::new()),
        }
        // TODO: add pre-defined namespaces, such as xml:
    }
    fn prefix_key(
        &self,
        local_part: String,
        nsuri: Option<String>,
        prefix: Option<String>,
    ) -> Option<PrefixKey> {
        let pqn2keyb = self.pqn_to_key.borrow();
        let pre_key_o = pqn2keyb.get(&(local_part, nsuri, prefix));
        pre_key_o.map(|p| p.clone())
    }
    fn qname_key(&self, local_part: String, nsuri: Option<String>) -> Option<QNKey> {
        let qn2keyb = self.qn_to_key.borrow();
        let qn_key_o = qn2keyb.get(&(local_part, nsuri));
        qn_key_o.map(|qn| qn.clone())
    }
    fn qn_key_from_pre_key(&self, pre_key: &PrefixKey) -> QNKey {
        let prefixb = self.prefixes.borrow();
        let pdata = prefixb.get(pre_key).unwrap();
        pdata.qn
    }
}

impl Interner for LocalInternment {
    fn get_or_intern(
        &self,
        local_part: String,
        namespace_uri: Option<String>,
        prefix: Option<String>,
    ) -> (PrefixKey, QNKey) {
        // Does this QN already exist?
        // If so then return QN tuple-struct
        // If not insert into hashmaps and return QN tuple-struct
        let pre_key_o = self.prefix_key(local_part.clone(), namespace_uri.clone(), prefix.clone());
        pre_key_o.map_or_else(
            || {
                // This combo of (local-part, ns-uri, prefix) hasn't been seen before,
                // So intern it.
                // NB. we may already have a key for (local-part, ns-uri)
                let new_pre_key = self.prefix_keys.borrow().len();

                let qn_key_o = self.qname_key(local_part.clone(), namespace_uri.clone());
                qn_key_o.map_or_else(
                    || {
                        // No (local-part, ns-uri) found either, so have to intern everything
                        let new_qn_key = self.qn_keys.borrow().len();
                        self.qn_keys.borrow_mut().push(new_qn_key);
                        self.prefix_keys.borrow_mut().push(new_pre_key);
                        self.details.borrow_mut().insert(
                            new_qn_key,
                            QnDetails::new(local_part.clone(), namespace_uri.clone()),
                        );
                        self.prefixes
                            .borrow_mut()
                            .insert(new_pre_key, PrefixedQn::new(prefix.clone(), new_qn_key));
                        self.qn_to_key
                            .borrow_mut()
                            .insert((local_part.clone(), namespace_uri.clone()), new_qn_key);
                        self.pqn_to_key.borrow_mut().insert(
                            (local_part.clone(), namespace_uri.clone(), prefix.clone()),
                            new_pre_key,
                        );
                        (new_pre_key, new_qn_key)
                    },
                    |qn_key| {
                        // This is a new prefix for an existing QN
                        // Increment ref count for QN
                        self.prefix_keys.borrow_mut().push(new_pre_key);
                        self.prefixes
                            .borrow_mut()
                            .insert(new_pre_key, PrefixedQn::new(prefix.clone(), qn_key));
                        self.pqn_to_key.borrow_mut().insert(
                            (local_part.clone(), namespace_uri.clone(), prefix.clone()),
                            new_pre_key,
                        );
                        self.details.borrow_mut().get_mut(&qn_key).unwrap().count += 1;
                        (new_pre_key, qn_key)
                    },
                )
            },
            |pre_key| {
                // We have seen this combo before.
                // Increment ref count
                // Construct QN triple
                let qn_key = self.qn_key_from_pre_key(&pre_key);
                self.prefixes.borrow_mut().get_mut(&pre_key).unwrap().count += 1;
                self.details.borrow_mut().get_mut(&qn_key).unwrap().count += 1;
                (pre_key, qn_key)
            },
        )
    }
    fn decr_ref_count(&self, qn: QNKey, p: PrefixKey) {
        // TODO: do this more efficiently
        self.prefixes.borrow_mut().get_mut(&p).unwrap().count -= 1;
        self.details.borrow_mut().get_mut(&qn).unwrap().count -= 1;
        let count = self.details.borrow().get(&qn).unwrap().count;
        if count == 0 {
            self.prefixes.borrow_mut().remove(&p);
            self.details.borrow_mut().remove(&qn);
        }
    }
    fn to_string(&self, qn: QNKey, p: PrefixKey) -> String {
        let mut result = String::new();
        let _ = self
            .prefixes
            .borrow()
            .get(&p)
            .unwrap()
            .prefix
            .as_ref()
            .map_or((), |p| {
                result.push_str(p.as_str());
                result.push(':');
            });
        result.push_str(self.details.borrow().get(&qn).unwrap().local_part.as_str());
        result
    }
    fn prefix(&self, p: &PrefixKey) -> Option<String> {
        self.prefixes.borrow().get(p).unwrap().prefix.clone()
    }
    fn namespace_uri(&self, qn: &QNKey) -> Option<String> {
        self.details.borrow().get(qn).unwrap().namespace_uri.clone()
    }
    fn local_part(&self, qn: &QNKey) -> String {
        self.details.borrow().get(qn).unwrap().local_part.clone()
    }
}

//fn uri_qualifiedname(uri: &str, name: &str) -> String {
//    format!("Q{}{}{}{}", "{", uri, "}", name)
//}

/// A partial ordering for QualifiedNames.
/// Unprefixed names are considered to come before prefixed names.
impl<'i, I: Interner> PartialOrd for QualifiedName<'i, I> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 == other.0 {
            Some(Ordering::Equal)
        } else {
            let lhs_local_part = self.2.local_part(&self.0);
            let rhs_local_part = self.2.local_part(&other.0);
            let lpcmp = lhs_local_part.partial_cmp(&rhs_local_part);
            if Some(Ordering::Equal) == lpcmp {
                match (self.2.prefix(&self.1), other.2.prefix(&other.1)) {
                    (None, None) => Some(Ordering::Equal),
                    (None, Some(_)) => Some(Ordering::Greater),
                    (Some(_), None) => Some(Ordering::Less),
                    (Some(_), Some(_)) => Some(
                        self.2
                            .namespace_uri(&self.0)
                            .unwrap()
                            .partial_cmp(&other.2.namespace_uri(&other.1).unwrap())?,
                    ),
                }
            } else {
                lpcmp
            }
        }
    }
}

impl<'i, I: Interner> Ord for QualifiedName<'i, I> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'i, I: Interner> Hash for QualifiedName<'i, I> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(ns) = self.2.namespace_uri(&self.0).as_ref() {
            ns.hash(state)
        }
        self.2.local_part(&self.0).hash(state);
    }
}

/// Parse a string to create a [QualifiedName].
/// Resolve prefix against a set of XML Namespace declarations.
/// This method can be used when there is no XSL stylesheet to derive the namespaces.
/// QualifiedName ::= (prefix ":")? local-name
impl<'i, I: Interner> TryFrom<(&str, Rc<NamespaceMap>, &'i I)> for QualifiedName<'i, I> {
    type Error = Error;
    fn try_from(s: (&str, Rc<NamespaceMap>, &'i I)) -> Result<Self, Self::Error> {
        let state: ParserState<I, Nullo> = ParserState::new(None, None, None, s.2);
        match eqname()((s.0, state)) {
            Ok((_, qn)) => {
                if qn.prefix().is_some() && qn.namespace_uri().is_none() {
                    match s.1.get(&qn.prefix()) {
                        Some(ns) => Ok(QualifiedName::new(
                            qn.local_part(),
                            Some(ns),
                            qn.prefix(),
                            s.2,
                        )),
                        _ => Err(Error::new(
                            ErrorKind::Unknown,
                            format!("unable to match prefix \"{}\"", qn.prefix().unwrap()),
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
impl<'i, I: Interner, N: Node> TryFrom<(&str, N, &'i I)> for QualifiedName<'i, I> {
    type Error = Error;
    fn try_from(s: (&str, N, &'i I)) -> Result<Self, Self::Error> {
        let state: ParserState<I, Nullo> = ParserState::new(None, None, None, s.2);
        match eqname()((s.0, state)) {
            Ok((_, qn)) => {
                if qn.prefix().is_some() && qn.namespace_uri().is_none() {
                    s.1.namespace_iter()
                        .find(|ns| ns.name::<I>().unwrap().local_part() == qn.prefix().unwrap())
                        .map_or(
                            Err(Error::new(
                                ErrorKind::DynamicAbsent,
                                format!(
                                    "no namespace matching prefix \"{}\"",
                                    qn.prefix().unwrap()
                                ),
                            )),
                            |ns| {
                                Ok(QualifiedName::new(
                                    qn.local_part(),
                                    Some(ns.value().to_string()),
                                    qn.prefix(),
                                    s.2,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unqualified() {
        let intern = LocalInternment::new();
        let foo = QualifiedName::new("foo", None, None, &intern);
        assert_eq!(foo.to_string(), "foo")
    }
    #[test]
    fn unqualified_eq() {
        let intern = LocalInternment::new();
        let foo = QualifiedName::new("foo", None, None, &intern);
        let bar = QualifiedName::new("foo", None, None, &intern);
        assert_eq!(foo, bar)
    }
    #[test]
    fn unqualified_ne() {
        let intern = LocalInternment::new();
        let foo = QualifiedName::new("foo", None, None, &intern);
        let bar = QualifiedName::new("bar", None, None, &intern);
        assert_ne!(foo, bar)
    }
    #[test]
    fn qualified() {
        let intern = LocalInternment::new();
        let foo = QualifiedName::new(
            "foo",
            Some("foo".to_string()),
            Some("http://example.org/whatsinaname/".to_string()),
            &intern,
        );
        assert_eq!(foo.to_string(), "foo:foo")
    }
    #[test]
    fn qualified_eq() {
        let intern = LocalInternment::new();
        let foo = QualifiedName::new(
            "foo",
            Some("foo".to_string()),
            Some("http://example.org/whatsinaname/".to_string()),
            &intern,
        );
        let bar = QualifiedName::new(
            "foo",
            Some("bar".to_string()),
            Some("http://example.org/whatsinaname/".to_string()),
            &intern,
        );
        assert_eq!(foo, bar)
    }
    #[test]
    fn qualified_ne() {
        let intern = LocalInternment::new();
        let foo = QualifiedName::new(
            "foo",
            Some("foo".to_string()),
            Some("http://example.org/foo/".to_string()),
            &intern,
        );
        let bar = QualifiedName::new(
            "foo",
            Some("bar".to_string()),
            Some("http://example.org/bar/".to_string()),
            &intern,
        );
        assert_ne!(foo, bar)
    }
    // TODO: test dropping QNs
    #[test]
    fn parse_unqualified() {
        let intern = LocalInternment::new();
        let foo = QualifiedName::new(
            "foo",
            Some("foo".to_string()),
            Some("http://example.org/foo/".to_string()),
            &intern,
        );
        let bar = QualifiedName::new(
            "foo",
            Some("bar".to_string()),
            Some("http://example.org/bar/".to_string()),
            &intern,
        );
        assert_ne!(foo, bar)
    }
}
