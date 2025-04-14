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

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

/// A Qualified Name is a triple of (QNKey, PrefixKey, &Internment)
pub struct QualifiedName<'i, I: Interner>(QNKey, PrefixKey, &'i I);

impl<'i, I: Interner> QualifiedName<'i, I> {
    pub fn new(
        namespace_uri: Option<String>,
        prefix: Option<String>,
        local_part: impl Into<String>,
        interner: &'i I,
    ) -> Self {
        let (pre_key, qn_key) = interner.get_or_intern(local_part.into(), prefix, namespace_uri);
        QualifiedName(qn_key, pre_key, interner)
    }
    pub fn to_string(&self) -> String {
        self.2.to_string(self.0, self.1)
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

/// An identifier for a Qualified Name.
pub type QNKey = usize;

/// An identifier for a prefixed QN.
pub type PrefixKey = usize;

/// An Interner provides internment of Qualified Names.
pub trait Interner {
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
struct QN_Details {
    local_part: String,
    namespace_uri: Option<String>,
    count: usize,
}
impl QN_Details {
    pub fn new(local_part: String, namespace_uri: Option<String>) -> Self {
        QN_Details {
            local_part,
            namespace_uri,
            count: 1,
        }
    }
}

struct Prefixed_QN {
    prefix: Option<String>,
    qn: QNKey,
    count: usize,
}
impl Prefixed_QN {
    fn new(prefix: Option<String>, qn: QNKey) -> Self {
        Prefixed_QN {
            prefix,
            qn,
            count: 1,
        }
    }
}

/// A simple [Interner] for local use.
pub struct LocalInternment {
    qn_keys: RefCell<Vec<QNKey>>,
    prefix_keys: RefCell<Vec<PrefixKey>>,

    // (local-part,namespace-uri,prefix) -> LIKey
    pqn_to_key: RefCell<HashMap<(String, Option<String>, Option<String>), PrefixKey>>,
    // (local-part,namespace-uri) -> QNKey
    qn_to_key: RefCell<HashMap<(String, Option<String>), QNKey>>,

    prefixes: RefCell<HashMap<PrefixKey, Prefixed_QN>>,
    details: RefCell<HashMap<QNKey, QN_Details>>,
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
                            QN_Details::new(local_part.clone(), namespace_uri.clone()),
                        );
                        self.prefixes
                            .borrow_mut()
                            .insert(new_pre_key, Prefixed_QN::new(prefix.clone(), new_qn_key));
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
                            .insert(new_pre_key, Prefixed_QN::new(prefix.clone(), qn_key));
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

/*
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
*/
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
/*
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
*/
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
    use super::*;

    #[test]
    fn unqualified() {
        let intern = LocalInternment::new();
        let foo = QualifiedName::new(None, None, "foo", &intern);
        assert_eq!(foo.to_string(), "foo")
    }
    #[test]
    fn unqualified_eq() {
        let intern = LocalInternment::new();
        let foo = QualifiedName::new(None, None, "foo", &intern);
        let bar = QualifiedName::new(None, None, "foo", &intern);
        assert_eq!(foo, bar)
    }
    #[test]
    fn unqualified_ne() {
        let intern = LocalInternment::new();
        let foo = QualifiedName::new(None, None, "foo", &intern);
        let bar = QualifiedName::new(None, None, "bar", &intern);
        assert_ne!(foo, bar)
    }
    #[test]
    fn qualified() {
        let intern = LocalInternment::new();
        let foo = QualifiedName::new(
            Some("foo".to_string()),
            Some("http://example.org/whatsinaname/".to_string()),
            "foo",
            &intern,
        );
        assert_eq!(foo.to_string(), "foo:foo")
    }
    #[test]
    fn qualified_eq() {
        let intern = LocalInternment::new();
        let foo = QualifiedName::new(
            Some("foo".to_string()),
            Some("http://example.org/whatsinaname/".to_string()),
            "foo",
            &intern,
        );
        let bar = QualifiedName::new(
            Some("bar".to_string()),
            Some("http://example.org/whatsinaname/".to_string()),
            "foo",
            &intern,
        );
        assert_eq!(foo, bar)
    }
    #[test]
    fn qualified_ne() {
        let intern = LocalInternment::new();
        let foo = QualifiedName::new(
            Some("foo".to_string()),
            Some("http://example.org/foo/".to_string()),
            "foo",
            &intern,
        );
        let bar = QualifiedName::new(
            Some("bar".to_string()),
            Some("http://example.org/bar/".to_string()),
            "foo",
            &intern,
        );
        assert_ne!(foo, bar)
    }
    // TODO: test dropping QNs
}
