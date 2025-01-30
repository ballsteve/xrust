//! Support for Qualified Names.
//! Names are interned, using slotmap, for speedy equality checks (compare keys, rather than characters).
//! This also applies to local names, prefixes, and XML Namespace URIs.
//! The slotmap key for a QualifiedName is its URI Qualified Name.
//! A SecondaryMap is maintained, keyed by the URI Qualified Name key and mapped to the [QualifiedNameData] struct that contains the local name, Namespace URI, and prefix (if any).

//use crate::item::Node;
use crate::namespace::NamespaceMap;
use crate::parser::xml::qname::eqname;
use crate::parser::ParserState;
use crate::trees::nullo::Nullo;
//use crate::value::Value;
use crate::xdmerror::{Error, ErrorKind};
//use core::hash::{Hash, Hasher};
use slotmap::{DefaultKey, SecondaryMap, SlotMap};
//use std::cmp::Ordering;
//use std::collections::HashMap;
//use std::fmt;
//use std::fmt::Debug;
//use std::fmt::Formatter;

/// A QualifiedName is a slotmap Key
pub type QualifiedName = DefaultKey;

/// An Internment is a SlotMap for the base strings plus a SecondaryMap to map to a [QualifiedNameData] struct.
pub type Internment = (
    SlotMap<DefaultKey, String>,
    SecondaryMap<DefaultKey, QualifiedNameData>,
);

/// Initialise both the slotmap internment and a corresponding secondary mapping from keys to qualified names.
/// An application should only create one of these.
pub fn new_map() -> Internment {
    let mut sm = SlotMap::new();
    let mut sec = SecondaryMap::new();

    // Prime with the empty string and XML namespace
    let xml_uri = "http://www.w3.org/XML/1998/namespace";
    let prefix = sm.insert("xml".to_string());
    let uri = sm.insert(xml_uri.to_string());
    let xml = sm.insert("^".to_string());
    let uriqualified = sm.insert(uri_qualifiedname(xml_uri, "^"));

    let _ = sec.insert(
        uriqualified,
        QualifiedNameData::new(Some(uri), Some(prefix), xml),
    );

    (sm, sec)
}

pub fn uri_qualifiedname(uri: &str, name: &str) -> String {
    format!("Q{}{}{}{}", "{", uri, "}", name)
}

/// Create a QualifiedName (QN). A QN consists of a Namespace URI and a local name.
/// QNs may optionally have a prefix.
/// It is not valid for a QN to have a prefix but no Namespace URI.
/// Both prefix and Namespace URI may not be empty strings.
pub fn new(
    nsuri: Option<String>,
    prefix: Option<String>,
    localname: impl Into<String>,
    intern: &mut Internment,
) -> Result<QualifiedName, Error> {
    match (nsuri, prefix) {
        (None, Some(_)) => Err(Error::new(
            ErrorKind::DynamicAbsent,
            "missing Namespace URI",
        )),
        (Some(n), Some(p)) => {
            let my_localname = localname.into();
            let uriqualified = uri_qualifiedname(n.as_str(), my_localname.as_str());
            let localname_key = intern.0.insert(my_localname);
            let nsuri_key = intern.0.insert(n);
            let prefix_key = intern.0.insert(p);
            let k = intern.0.insert(uriqualified);

            let _ = intern.1.insert(
                k,
                QualifiedNameData::new(Some(nsuri_key), Some(prefix_key), localname_key),
            );

            Ok(k)
        }
        (Some(n), None) => {
            let my_localname = localname.into();
            let uriqualified = uri_qualifiedname(n.as_str(), my_localname.as_str());
            let localname_key = intern.0.insert(my_localname);
            let nsuri_key = intern.0.insert(n);
            let k = intern.0.insert(uriqualified);

            let _ = intern.1.insert(
                k,
                QualifiedNameData::new(Some(nsuri_key), None, localname_key),
            );

            Ok(k)
        }
        (None, None) => {
            // An unprefixed QName
            let my_localname = localname.into();
            let k = intern.0.insert(my_localname.clone());
            let qn = QualifiedNameData::new(None, None, k);
            intern.1.insert(k, qn);
            Ok(k)
        }
    }
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

#[derive(Clone)]
pub struct QualifiedNameData {
    nsuri: Option<DefaultKey>,
    prefix: Option<DefaultKey>,
    localname: DefaultKey,
}
// TODO: we may need methods that return a string slice, rather than a copy of the string
impl QualifiedNameData {
    /// Builds a QualifiedName from String parts
    pub fn new(
        nsuri: Option<DefaultKey>,
        prefix: Option<DefaultKey>,
        localname: DefaultKey,
    ) -> Self {
        QualifiedNameData {
            nsuri,
            prefix,
            localname,
        }
    }
    pub fn as_ref(&self) -> &Self {
        self
    }
    pub fn namespace_uri(&self) -> Option<DefaultKey> {
        self.nsuri
    }
    //    pub fn namespace_uri_to_string(&self) -> Option<String> {
    //        self.nsuri.as_ref().map(|x| x.to_string())
    //    }
    pub fn prefix(&self) -> Option<DefaultKey> {
        self.prefix
    }
    //    pub fn prefix_to_string(&self) -> Option<String> {
    //        self.prefix.as_ref().map(|x| x.to_string())
    //    }
    pub fn localname(&self) -> DefaultKey {
        self.localname.clone()
    }
    //    pub fn localname_to_string(&self) -> String {
    //        self.localname.to_string()
    //    }
    /*     /// Fully resolve a qualified name. If the qualified name has a prefix but no namespace URI,
    /// then find the prefix in the supplied namespaces and use the corresponding URI.
    /// If the qualified name already has a namespace URI, then this method has no effect.
    /// If the qualified name has no prefix, then this method has no effect.
    pub fn resolve<F>(&mut self, mapper: F) -> Result<(), Error>
    where
        F: Fn(Option<Rc<Value>>) -> Result<Rc<Value>, Error>,
    {
        match (&self.prefix, &self.nsuri) {
            (Some(p), None) => {
                self.nsuri = Some(mapper(Some(p.clone()))?.clone());
                Ok(())
            }
            _ => Ok(()),
        }
    }*/
}
/*
impl fmt::Display for QualifiedNameData {
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
*/
/*
impl Debug for QualifiedNameData {
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
*/
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
/*
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
*/
/*
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
}
*/
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
    fn unqualified_raw() {
        let mut intern = new_map();
        let qn = new(None, None, "foo", &mut intern).expect("unable to create qualified name");
        assert_eq!(intern.0[qn], "foo")
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
        let mut intern = new_map();
        let qn = new(
            Some("http://example.org/whatsinaname/".to_string()),
            Some("x".to_string()),
            "foo",
            &mut intern,
        )
        .expect("unable to create qualified name");
        assert_eq!(intern.0[qn], "Q{http://example.org/whatsinaname/}foo")
    }
    #[test]
    fn in_eq() {
        let mut intern = new_map();
        let qn1 = new(
            Some("http://example.org/whatsinaname/".to_string()),
            Some("x".to_string()),
            "foo",
            &mut intern,
        )
        .expect("unable to create first qualified name");
        let qn2 = new(
            Some("http://example.org/whatsinaname/".to_string()),
            Some("x".to_string()),
            "foo",
            &mut intern,
        )
        .expect("unable to create second qualified name");
        assert!(qn1 == qn2)
    }
    #[test]
    fn in_ne() {
        let mut intern = new_map();
        let qn1 = new(
            Some("http://example.org/whatsinaname/".to_string()),
            Some("x".to_string()),
            "foo",
            &mut intern,
        )
        .expect("unable to create first qualified name");
        let qn2 = new(
            Some("http://example.org/whatsinaname/".to_string()),
            Some("y".to_string()),
            "foo",
            &mut intern,
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
