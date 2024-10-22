//! These functions are for features defined in XPath Functions 1.0 and 2.0.

use pkg_version::*;
use std::rc::Rc;
use url::Url;

use crate::item::{Item, Node, Sequence};
use crate::qname::QualifiedName;
use crate::transform::context::{Context, StaticContext};
use crate::transform::{NamespaceMap, Transform};
use crate::value::Value;
use crate::xdmerror::{Error, ErrorKind};
use crate::SequenceTrait;

/// XPath position function.
pub fn position<N: Node>(ctxt: &Context<N>) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::from(ctxt.i as i64 + 1)))])
}

/// XPath last function.
pub fn last<N: Node>(ctxt: &Context<N>) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::from(
        ctxt.cur.len() as i64
    )))])
}

/// XPath count function.
pub fn tr_count<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
    J: FnMut(&Context<N>) -> Result<Sequence<N>, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H, J>,
    s: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::from(
        ctxt.dispatch(stctxt, s)?.len() as i64,
    )))])
}

/// XPath generate-id function.
pub fn generate_id<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
    J: FnMut(&Context<N>) -> Result<Sequence<N>, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H, J>,
    s: &Option<Box<Transform<N>>>,
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
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
    J: FnMut(&Context<N>) -> Result<Sequence<N>, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H, J>,
    s: &Box<Transform<N>>,
    ns: &Rc<NamespaceMap>,
) -> Result<Sequence<N>, Error> {
    let prop = ctxt.dispatch(stctxt, s)?;
    if prop.len() == 1 {
        let qn = QualifiedName::try_from((prop.to_string().as_str(), ns.clone()))?;
        match (
            qn.namespace_uri_to_string().as_deref(),
            qn.localname_to_string().as_str(),
        ) {
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
pub fn available_system_properties<N: Node>() -> Result<Sequence<N>, Error> {
    Ok(vec![
        Item::Value(Rc::new(Value::from(QualifiedName::new(
            Some(XSLTNS.to_string()),
            None,
            String::from("version"),
        )))),
        Item::Value(Rc::new(Value::from(QualifiedName::new(
            Some(XSLTNS.to_string()),
            None,
            String::from("vendor"),
        )))),
        Item::Value(Rc::new(Value::from(QualifiedName::new(
            Some(XSLTNS.to_string()),
            None,
            String::from("vendor-url"),
        )))),
        Item::Value(Rc::new(Value::from(QualifiedName::new(
            Some(XSLTNS.to_string()),
            None,
            String::from("product-name"),
        )))),
        Item::Value(Rc::new(Value::from(QualifiedName::new(
            Some(XSLTNS.to_string()),
            None,
            String::from("product-version"),
        )))),
        Item::Value(Rc::new(Value::from(QualifiedName::new(
            Some(XSLTNS.to_string()),
            None,
            String::from("is-schema-aware"),
        )))),
        Item::Value(Rc::new(Value::from(QualifiedName::new(
            Some(XSLTNS.to_string()),
            None,
            String::from("supports-serialization"),
        )))),
        Item::Value(Rc::new(Value::from(QualifiedName::new(
            Some(XSLTNS.to_string()),
            None,
            String::from("supports-backward-compatibility"),
        )))),
        Item::Value(Rc::new(Value::from(QualifiedName::new(
            Some(XSLTNS.to_string()),
            None,
            String::from("supports-namspace-axis"),
        )))),
        Item::Value(Rc::new(Value::from(QualifiedName::new(
            Some(XSLTNS.to_string()),
            None,
            String::from("supports-streaming"),
        )))),
        Item::Value(Rc::new(Value::from(QualifiedName::new(
            Some(XSLTNS.to_string()),
            None,
            String::from("supports-dynamic-evaluation"),
        )))),
        Item::Value(Rc::new(Value::from(QualifiedName::new(
            Some(XSLTNS.to_string()),
            None,
            String::from("xpath-version"),
        )))),
        Item::Value(Rc::new(Value::from(QualifiedName::new(
            Some(XSLTNS.to_string()),
            None,
            String::from("xsd-version"),
        )))),
    ])
}

/// XSLT document function.
/// The first argument is a sequence of URI references. Each reference is cast to xs:anyURI.
/// Relative URIs are resolved against the base URI of the second argument. The default is to use the baseURI of the context (i.e. the XSL stylesheet).
pub fn document<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
    J: FnMut(&Context<N>) -> Result<Sequence<N>, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H, J>,
    uris: &Box<Transform<N>>,
    _base: &Option<Box<Transform<N>>>,
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

pub(crate) fn tr_error<N: Node>(
    _ctxt: &Context<N>,
    kind: &ErrorKind,
    msg: &String,
) -> Result<Sequence<N>, Error> {
    Err(Error::new(*kind, msg.clone()))
}

pub(crate) fn not_implemented<N: Node>(
    _ctxt: &Context<N>,
    msg: &String,
) -> Result<Sequence<N>, Error> {
    Err(Error::new(ErrorKind::NotImplemented, msg.clone()))
}
