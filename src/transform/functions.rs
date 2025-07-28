//! These functions are for features defined in XPath Functions 1.0 and 2.0.

use pkg_version::*;
use std::rc::Rc;
use std::sync::LazyLock;
use url::Url;

use crate::SequenceTrait;
use crate::item::{Item, Node, Sequence};
use crate::parser::xml::qname::eqname_to_qname;
use crate::parser::{ParseError, ParserState, StaticStateBuilder};
use crate::transform::context::{Context, StaticContext};
use crate::transform::{NamespaceMap, Transform};
use crate::value::Value;
use crate::xdmerror::{Error, ErrorKind};
use qualname::{NamespaceUri, NcName, QName};

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
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
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
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
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
                    ));
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
//const XSLTNS: &str = "http://www.w3.org/1999/XSL/Transform";
static XSLTNS: LazyLock<Option<NamespaceUri>> =
    LazyLock::new(|| Some(NamespaceUri::try_from("http://www.w3.org/1999/XSL/Transform").unwrap()));
static XSLVERSION: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("version").unwrap(), XSLTNS.clone()));
static XSLVENDOR: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("vendor").unwrap(), XSLTNS.clone()));
static XSLVENDORURL: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(NcName::try_from("vendor-url").unwrap(), XSLTNS.clone())
});
static XSLPRODUCTNAME: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(NcName::try_from("product-name").unwrap(), XSLTNS.clone())
});
static XSLPRODUCTVERSION: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(NcName::try_from("product-version").unwrap(), XSLTNS.clone())
});
static XSLISSCHEMAAWARE: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(NcName::try_from("is-schema-aware").unwrap(), XSLTNS.clone())
});
static XSLSUPPORTSSERIALIZATION: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(
        NcName::try_from("supports-serialization").unwrap(),
        XSLTNS.clone(),
    )
});
static XSLSUPPORTSBACKWARDCOMPATIBILITY: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(
        NcName::try_from("supports-backward-campatibility").unwrap(),
        XSLTNS.clone(),
    )
});
static XSLSUPPORTSNAMESPACEAXIS: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(
        NcName::try_from("supports-namespace-axis").unwrap(),
        XSLTNS.clone(),
    )
});
static XSLSUPPORTSSTREAMING: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(
        NcName::try_from("supports-streaming").unwrap(),
        XSLTNS.clone(),
    )
});
static XSLSUPPORTSDYNAMICEVALUATION: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(
        NcName::try_from("supports-dynamic-evaluation").unwrap(),
        XSLTNS.clone(),
    )
});
static XSLSUPPORTSHOF: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(
        NcName::try_from("supports-higher-order-functions").unwrap(),
        XSLTNS.clone(),
    )
});
static XSLXPATHVERSION: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(NcName::try_from("xpath-version").unwrap(), XSLTNS.clone())
});
static XSLXSDVERSION: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(NcName::try_from("xsd-version").unwrap(), XSLTNS.clone())
});

/// XSLT system-property function.
pub fn system_property<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    s: &Box<Transform<N>>,
    ns: &Rc<NamespaceMap>,
) -> Result<Sequence<N>, Error> {
    eprintln!("system_property: in-scope-namespaces: {:?}", ns);
    let prop = ctxt.dispatch(stctxt, s)?;
    if prop.len() == 1 {
        let ps: ParserState<N> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|prefix| {
                ns.namespace_uri(&Some(prefix.clone()))
                    .ok_or(ParseError::MissingNameSpace)
            })
            .build();
        let propstr = prop.to_string();
        let qn = eqname_to_qname()((propstr.as_str(), ps), &mut static_state)
            .map_err(|_| Error::new(ErrorKind::DynamicAbsent, "unable to resolve QName"))?;
        //let qn = QName::try_from((prop.to_string().as_str(), ns.clone()))?;
        if qn.1 == *XSLVERSION {
            Ok(vec![Item::Value(Rc::new(Value::from("0.9")))])
        } else if qn.1 == *XSLVENDOR {
            Ok(vec![Item::Value(Rc::new(Value::from(
                "Steve Ball, Daniel Murphy",
            )))])
        } else if qn.1 == *XSLVENDORURL {
            Ok(vec![Item::Value(Rc::new(Value::from(
                "https://github.com/ballsteve/xrust",
            )))])
        } else if qn.1 == *XSLPRODUCTNAME {
            Ok(vec![Item::Value(Rc::new(Value::from("\u{03A7}rust")))])
        } else if qn.1 == *XSLPRODUCTVERSION {
            Ok(vec![Item::Value(Rc::new(Value::from(format!(
                "{}.{}.{}",
                pkg_version_major!(),
                pkg_version_minor!(),
                pkg_version_patch!()
            ))))])
        } else if qn.1 == *XSLISSCHEMAAWARE {
            Ok(vec![Item::Value(Rc::new(Value::from("no")))])
        } else if qn.1 == *XSLSUPPORTSSERIALIZATION {
            Ok(vec![Item::Value(Rc::new(Value::from("no")))])
        } else if qn.1 == *XSLSUPPORTSBACKWARDCOMPATIBILITY {
            Ok(vec![Item::Value(Rc::new(Value::from("no")))])
        } else if qn.1 == *XSLSUPPORTSNAMESPACEAXIS {
            Ok(vec![Item::Value(Rc::new(Value::from("no")))])
        } else if qn.1 == *XSLSUPPORTSSTREAMING {
            Ok(vec![Item::Value(Rc::new(Value::from("no")))])
        } else if qn.1 == *XSLSUPPORTSDYNAMICEVALUATION {
            Ok(vec![Item::Value(Rc::new(Value::from("no")))])
        } else if qn.1 == *XSLSUPPORTSHOF {
            Ok(vec![Item::Value(Rc::new(Value::from("no")))])
        } else if qn.1 == *XSLXPATHVERSION {
            Ok(vec![Item::Value(Rc::new(Value::from(2.9)))])
        } else if qn.1 == *XSLXSDVERSION {
            Ok(vec![Item::Value(Rc::new(Value::from(1.1)))])
        } else {
            Err(Error::new(
                ErrorKind::Unknown,
                format!("unknown property \"{}\"", qn.1),
            ))
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
        Item::Value(Rc::new(Value::from(QName::new_from_parts(
            NcName::try_from("version").unwrap(),
            XSLTNS.clone(),
        )))),
        Item::Value(Rc::new(Value::from(QName::new_from_parts(
            NcName::try_from("vendor").unwrap(),
            XSLTNS.clone(),
        )))),
        Item::Value(Rc::new(Value::from(QName::new_from_parts(
            NcName::try_from("vendor-url").unwrap(),
            XSLTNS.clone(),
        )))),
        Item::Value(Rc::new(Value::from(QName::new_from_parts(
            NcName::try_from("product-name").unwrap(),
            XSLTNS.clone(),
        )))),
        Item::Value(Rc::new(Value::from(QName::new_from_parts(
            NcName::try_from("product-version").unwrap(),
            XSLTNS.clone(),
        )))),
        Item::Value(Rc::new(Value::from(QName::new_from_parts(
            NcName::try_from("is-schema-aware").unwrap(),
            XSLTNS.clone(),
        )))),
        Item::Value(Rc::new(Value::from(QName::new_from_parts(
            NcName::try_from("supports-serialization").unwrap(),
            XSLTNS.clone(),
        )))),
        Item::Value(Rc::new(Value::from(QName::new_from_parts(
            NcName::try_from("supports-backward-compatibility").unwrap(),
            XSLTNS.clone(),
        )))),
        Item::Value(Rc::new(Value::from(QName::new_from_parts(
            NcName::try_from("supports-namespace-axis").unwrap(),
            XSLTNS.clone(),
        )))),
        Item::Value(Rc::new(Value::from(QName::new_from_parts(
            NcName::try_from("supports-streaming").unwrap(),
            XSLTNS.clone(),
        )))),
        Item::Value(Rc::new(Value::from(QName::new_from_parts(
            NcName::try_from("supports-dynamic-evaluation").unwrap(),
            XSLTNS.clone(),
        )))),
        Item::Value(Rc::new(Value::from(QName::new_from_parts(
            NcName::try_from("xpath-version").unwrap(),
            XSLTNS.clone(),
        )))),
        Item::Value(Rc::new(Value::from(QName::new_from_parts(
            NcName::try_from("xsd-version").unwrap(),
            XSLTNS.clone(),
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
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
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
