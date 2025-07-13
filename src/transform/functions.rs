//! These functions are for features defined in XPath Functions 1.0 and 2.0.

use pkg_version::*;
use std::rc::Rc;
use url::Url;

use crate::item::{Item, Node, Sequence};
use crate::qname::{Interner, QualifiedName, UriQualifiedName};
use crate::transform::context::{Context, StaticContext};
use crate::transform::{NamespaceMap, Transform};
use crate::value::Value;
use crate::xdmerror::{Error, ErrorKind};
use crate::SequenceTrait;

/// XPath position function.
pub fn position<'i, I: Interner, N: Node>(ctxt: &Context<'i, I, N>) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::from(ctxt.i as i64 + 1)))])
}

/// XPath last function.
pub fn last<'i, I: Interner, N: Node>(ctxt: &Context<'i, I, N>) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::from(
        ctxt.cur.len() as i64
    )))])
}

/// XPath count function.
pub fn tr_count<
    'i,
    I: Interner,
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<'i, I, N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    s: &Transform<'i, I, N>,
) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::from(
        ctxt.dispatch(stctxt, s)?.len() as i64,
    )))])
}

/// XPath generate-id function.
pub fn generate_id<
    'i,
    I: Interner,
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<'i, I, N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    s: &Option<Box<Transform<'i, I, N>>>,
) -> Result<Sequence<N>, Error> {
    let i = match s {
        None => ctxt.cur[ctxt.i].clone(),
        Some(t) => {
            let seq = ctxt.dispatch(stctxt, t)?;
            match seq.len() {
                0 => return Ok(vec![Item::Value(Rc::new(Value::from("")))]),
                1 => seq[0].clone(),
                _ => {
                    return Err(Error::new(
                        ErrorKind::TypeError,
                        String::from("not a singleton sequence"),
                    ))
                }
            }
        }
    };
    match i {
        Item::Node(n) => Ok(vec![Item::Value(Rc::new(Value::from(n.get_id())))]),
        _ => Err(Error::new(ErrorKind::TypeError, String::from("not a node"))),
    }
}

// TODO: this is copied from the xslt module. Move to a common definitions module.
const XSLTNS: &str = "http://www.w3.org/1999/XSL/Transform";

/// XSLT system-property function.
pub fn system_property<
    'i,
    I: Interner,
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<'i, I, N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    s: &Box<Transform<'i, I, N>>,
    ns: &Rc<NamespaceMap>,
) -> Result<Sequence<N>, Error> {
    let prop = ctxt.dispatch(stctxt, s)?;
    if prop.len() == 1 {
        let qn = QualifiedName::try_from((prop.to_string().as_str(), ns.clone(), ctxt.intern))?;
        match (qn.namespace_uri().as_deref(), qn.local_part().as_str()) {
            (Some(XSLTNS), "version") => Ok(vec![Item::Value(Rc::new(Value::from("0.9")))]),
            (Some(XSLTNS), "vendor") => Ok(vec![Item::Value(Rc::new(Value::from(
                "Steve Ball, Daniel Murphy",
            )))]),
            (Some(XSLTNS), "vendor-url") => Ok(vec![Item::Value(Rc::new(Value::from(
                "https://github.com/ballsteve/xrust",
            )))]),
            (Some(XSLTNS), "product-name") => {
                Ok(vec![Item::Value(Rc::new(Value::from("\u{03A7}rust")))])
            }
            (Some(XSLTNS), "product-version") => {
                Ok(vec![Item::Value(Rc::new(Value::from(format!(
                    "{}.{}.{}",
                    pkg_version_major!(),
                    pkg_version_minor!(),
                    pkg_version_patch!()
                ))))])
            }
            (Some(XSLTNS), "is-schema-aware") => Ok(vec![Item::Value(Rc::new(Value::from("no")))]),
            (Some(XSLTNS), "supports-serialization") => {
                Ok(vec![Item::Value(Rc::new(Value::from("no")))])
            }
            (Some(XSLTNS), "supports-backwards-compatibility") => {
                Ok(vec![Item::Value(Rc::new(Value::from("no")))])
            }
            (Some(XSLTNS), "supports-namespace-axis") => {
                Ok(vec![Item::Value(Rc::new(Value::from("no")))])
            }
            (Some(XSLTNS), "supports-streaming") => {
                Ok(vec![Item::Value(Rc::new(Value::from("no")))])
            }
            (Some(XSLTNS), "supports-dynamic-evaluation") => {
                Ok(vec![Item::Value(Rc::new(Value::from("no")))])
            }
            (Some(XSLTNS), "supports-higher-order-functions") => {
                Ok(vec![Item::Value(Rc::new(Value::from("no")))])
            }
            (Some(XSLTNS), "xpath-version") => Ok(vec![Item::Value(Rc::new(Value::from(2.9)))]),
            (Some(XSLTNS), "xsd-version") => Ok(vec![Item::Value(Rc::new(Value::from(1.1)))]),
            _ => Err(Error::new(
                ErrorKind::Unknown,
                format!("unknown property \"{}\"", qn),
            )),
        }
    } else {
        Err(Error::new(
            ErrorKind::TypeError,
            String::from("not a singleton sequence"),
        ))
    }
}

/// XSLT available-system-property function.
pub fn available_system_properties<'i, I: Interner, N: Node>() -> Result<Sequence<N>, Error> {
    Ok(vec![
        Item::Value(Rc::new(Value::from(UriQualifiedName::new(
            String::from("version"),
            Some(XSLTNS.to_string()),
        )))),
        Item::Value(Rc::new(Value::from(UriQualifiedName::new(
            String::from("vendor"),
            Some(XSLTNS.to_string()),
        )))),
        Item::Value(Rc::new(Value::from(UriQualifiedName::new(
            String::from("vendor-url"),
            Some(XSLTNS.to_string()),
        )))),
        Item::Value(Rc::new(Value::from(UriQualifiedName::new(
            String::from("product-name"),
            Some(XSLTNS.to_string()),
        )))),
        Item::Value(Rc::new(Value::from(UriQualifiedName::new(
            String::from("product-version"),
            Some(XSLTNS.to_string()),
        )))),
        Item::Value(Rc::new(Value::from(UriQualifiedName::new(
            String::from("is-schema-aware"),
            Some(XSLTNS.to_string()),
        )))),
        Item::Value(Rc::new(Value::from(UriQualifiedName::new(
            String::from("supports-serialization"),
            Some(XSLTNS.to_string()),
        )))),
        Item::Value(Rc::new(Value::from(UriQualifiedName::new(
            String::from("supports-backward-compatibility"),
            Some(XSLTNS.to_string()),
        )))),
        Item::Value(Rc::new(Value::from(UriQualifiedName::new(
            String::from("supports-namspace-axis"),
            Some(XSLTNS.to_string()),
        )))),
        Item::Value(Rc::new(Value::from(UriQualifiedName::new(
            String::from("supports-streaming"),
            Some(XSLTNS.to_string()),
        )))),
        Item::Value(Rc::new(Value::from(UriQualifiedName::new(
            String::from("supports-dynamic-evaluation"),
            Some(XSLTNS.to_string()),
        )))),
        Item::Value(Rc::new(Value::from(UriQualifiedName::new(
            String::from("xpath-version"),
            Some(XSLTNS.to_string()),
        )))),
        Item::Value(Rc::new(Value::from(UriQualifiedName::new(
            String::from("xsd-version"),
            Some(XSLTNS.to_string()),
        )))),
    ])
}

/// XSLT document function.
/// The first argument is a sequence of URI references. Each reference is cast to xs:anyURI.
/// Relative URIs are resolved against the base URI of the second argument. The default is to use the baseURI of the context (i.e. the XSL stylesheet).
pub fn document<
    'i,
    I: Interner,
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<'i, I, N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    uris: &Box<Transform<'i, I, N>>,
    _base: &Option<Box<Transform<'i, I, N>>>,
) -> Result<Sequence<N>, Error> {
    let u_list = ctxt.dispatch(stctxt, uris)?;
    if let Some(h) = &mut stctxt.fetcher {
        if let Some(g) = &mut stctxt.parser {
            u_list.iter().try_fold(vec![], |mut acc, u| {
                // TODO: resolve relative URI against base URI
                let url = Url::parse(u.to_string().as_str())
                    .map_err(|_| Error::new(ErrorKind::TypeError, "unable to parse URL"))?;
                let docdata = h(&url)?;
                //let x = g(docdata.as_str())?;
                //acc.push(Item::Node(x));
                acc.push(Item::Node(g(docdata.as_str())?));
                Ok(acc)
            })
        } else {
            Err(Error::new(
                ErrorKind::StaticAbsent,
                "function to parse document not supplied",
            ))
        }
    } else {
        Err(Error::new(
            ErrorKind::StaticAbsent,
            "function to resolve URI not supplied",
        ))
    }
}

pub(crate) fn tr_error<'i, I: Interner, N: Node>(
    _ctxt: &Context<'i, I, N>,
    kind: &ErrorKind,
    msg: &String,
) -> Result<Sequence<N>, Error> {
    Err(Error::new(*kind, msg.clone()))
}

pub(crate) fn not_implemented<'i, I: Interner, N: Node>(
    _ctxt: &Context<'i, I, N>,
    msg: &String,
) -> Result<Sequence<N>, Error> {
    Err(Error::new(ErrorKind::NotImplemented, msg.clone()))
}
