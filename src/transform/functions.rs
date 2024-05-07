//! These functions are for features defined in XPath Functions 1.0 and 2.0.

use pkg_version::*;
use std::rc::Rc;

use crate::item::{Item, Node, Sequence};
use crate::qname::QualifiedName;
use crate::transform::context::{Context, StaticContext};
use crate::transform::Transform;
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
pub fn tr_count<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    s: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::from(
        ctxt.dispatch(stctxt, s)?.len() as i64,
    )))])
}

/// XPath generate-id function.
pub fn generate_id<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
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
pub fn system_property<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    s: &Box<Transform<N>>,
) -> Result<Sequence<N>, Error> {
    let prop = ctxt.dispatch(stctxt, s)?;
    if prop.len() == 1 {
        eprintln!("sys_prop: {} namespaces:", ctxt.namespaces_ref().len());
        ctxt.namespaces_ref().iter().for_each(|ns| {
            ns.iter()
                .for_each(|(k, v)| eprintln!("prefix {} nsuri {}", k, v))
        });
        let qn = QualifiedName::try_from((prop.to_string().as_str(), ctxt.namespaces_ref()))?;
        match (qn.get_nsuri_ref(), qn.get_localname().as_str()) {
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
                format!("unknown property \"{}\"", qn.to_string()),
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

/// A user defined function.
/// Each argument is declared as a variable in the [Context].
/// The body of the function is then evaluated and it's result is returned.
pub fn user_defined<N: Node, F: FnMut(&str) -> Result<(), Error>>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<F>,
    arguments: &Vec<(String, Transform<N>)>,
    body: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    let mut new_ctxt = ctxt.clone();
    arguments
        .iter()
        .try_for_each(|(n, a)| match ctxt.dispatch(stctxt, a) {
            Ok(b) => {
                new_ctxt.var_push(n.clone(), b);
                Ok(())
            }
            Err(err) => Err(err),
        })?;
    new_ctxt.dispatch(stctxt, body)
}

pub(crate) fn tr_error<N: Node>(
    _ctxt: &Context<N>,
    kind: &ErrorKind,
    msg: &String,
) -> Result<Sequence<N>, Error> {
    Err(Error::new(kind.clone(), msg.clone()))
}

pub(crate) fn not_implemented<N: Node>(
    _ctxt: &Context<N>,
    msg: &String,
) -> Result<Sequence<N>, Error> {
    Err(Error::new(ErrorKind::NotImplemented, msg.clone()))
}
