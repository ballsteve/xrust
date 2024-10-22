//! These functions are for features defined in XPath Functions 1.0 and 2.0.

use std::rc::Rc;

use unicode_segmentation::UnicodeSegmentation;
use url::Url;

use crate::item::{Item, Node, Sequence, SequenceTrait};
use crate::transform::context::{Context, StaticContext};
use crate::transform::Transform;
use crate::value::Value;
use crate::xdmerror::{Error, ErrorKind};

/// XPath local-name function.
pub fn local_name<
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
    s.as_ref().map_or_else(
        || {
            // Get the name of the context item
            // TODO: handle the case of there not being a context item
            match ctxt.cur[ctxt.i] {
                Item::Node(ref m) => Ok(vec![Item::Value(m.name().localname().clone())]),
                _ => Err(Error::new(
                    ErrorKind::TypeError,
                    String::from("type error: not a node"),
                )),
            }
        },
        |t| {
            // Get the name of the singleton node
            let n = ctxt.dispatch(stctxt, t)?;
            match n.len() {
                0 => Ok(vec![Item::Value(Rc::new(Value::from("")))]),
                1 => match n[0] {
                    Item::Node(ref m) => Ok(vec![Item::Value(m.name().localname())]),
                    _ => Err(Error::new(
                        ErrorKind::TypeError,
                        String::from("type error: not a node"),
                    )),
                },
                _ => Err(Error::new(
                    ErrorKind::TypeError,
                    String::from("type error: not a singleton node"),
                )),
            }
        },
    )
}

/// XPath name function.
pub fn name<
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
    s.as_ref().map_or_else(
        || {
            // Get the name of the context item
            // TODO: handle the case of there being no context item
            match ctxt.cur[ctxt.i] {
                Item::Node(ref m) => Ok(vec![Item::Value(Rc::new(Value::from(m.name().clone())))]),
                _ => Err(Error::new(
                    ErrorKind::TypeError,
                    String::from("type error: not a node"),
                )),
            }
        },
        |t| {
            // Get the name of the singleton node
            let n = ctxt.dispatch(stctxt, t)?;
            match n.len() {
                0 => Ok(vec![Item::Value(Rc::new(Value::from("")))]),
                1 => match n[0] {
                    Item::Node(ref m) => Ok(vec![Item::Value(Rc::new(Value::from(
                        m.name().to_string(),
                    )))]),
                    _ => Err(Error::new(
                        ErrorKind::TypeError,
                        String::from("type error: not a node"),
                    )),
                },
                _ => Err(Error::new(
                    ErrorKind::TypeError,
                    String::from("type error: not a singleton node"),
                )),
            }
        },
    )
}

/// XPath string function.
pub fn string<
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
        ctxt.dispatch(stctxt, s)?.to_string(),
    )))])
}

/// XPath starts-with function.
pub fn starts_with<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
    J: FnMut(&Context<N>) -> Result<Sequence<N>, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H, J>,
    s: &Transform<N>,
    t: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    // s is the string to search, t is what to search for
    Ok(vec![Item::Value(Rc::new(Value::from(
        ctxt.dispatch(stctxt, s)?
            .to_string()
            .starts_with(ctxt.dispatch(stctxt, t)?.to_string().as_str()),
    )))])
}

/// XPath ends-with function.
pub fn ends_with<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
    J: FnMut(&Context<N>) -> Result<Sequence<N>, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H, J>,
    s: &Transform<N>,
    t: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    // s is the string to search, t is what to search for
    Ok(vec![Item::Value(Rc::new(Value::from(
        ctxt.dispatch(stctxt, s)?
            .to_string()
            .ends_with(ctxt.dispatch(stctxt, t)?.to_string().as_str()),
    )))])
}

/// XPath contains function.
pub fn contains<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
    J: FnMut(&Context<N>) -> Result<Sequence<N>, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H, J>,
    s: &Transform<N>,
    t: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    // s is the string to search, t is what to search for
    Ok(vec![Item::Value(Rc::new(Value::from(
        ctxt.dispatch(stctxt, s)?
            .to_string()
            .contains(ctxt.dispatch(stctxt, t)?.to_string().as_str()),
    )))])
}

/// XPath substring function.
pub fn substring<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
    J: FnMut(&Context<N>) -> Result<Sequence<N>, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H, J>,
    s: &Transform<N>,
    t: &Transform<N>,
    l: &Option<Box<Transform<N>>>,
) -> Result<Sequence<N>, Error> {
    // must have two or three arguments.
    // s is the string to search,
    // t is the index to start at,
    // l is the length of the substring at extract (or the rest of the string if missing)
    match l {
        Some(m) => Ok(vec![Item::Value(Rc::new(Value::from(
            ctxt.dispatch(stctxt, s)?
                .to_string()
                .graphemes(true)
                .skip(ctxt.dispatch(stctxt, t)?.to_int()? as usize - 1)
                .take(ctxt.dispatch(stctxt, m)?.to_int()? as usize)
                .collect::<String>(),
        )))]),
        None => Ok(vec![Item::Value(Rc::new(Value::from(
            ctxt.dispatch(stctxt, s)?
                .to_string()
                .graphemes(true)
                .skip(ctxt.dispatch(stctxt, t)?.to_int()? as usize - 1)
                .collect::<String>(),
        )))]),
    }
}

/// XPath substring-before function.
pub fn substring_before<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
    J: FnMut(&Context<N>) -> Result<Sequence<N>, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H, J>,
    s: &Transform<N>,
    t: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    // s is the string to search,
    // t is the string to find.
    let u = ctxt.dispatch(stctxt, s)?.to_string();
    match u.find(ctxt.dispatch(stctxt, t)?.to_string().as_str()) {
        Some(i) => {
            match u.get(0..i) {
                Some(v) => Ok(vec![Item::Value(Rc::new(Value::from(v)))]),
                None => {
                    // This shouldn't happen!
                    Err(Error::new(
                        ErrorKind::Unknown,
                        String::from("unable to extract substring"),
                    ))
                }
            }
        }
        None => Ok(vec![]),
    }
}

/// XPath substring-after function.
pub fn substring_after<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
    J: FnMut(&Context<N>) -> Result<Sequence<N>, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H, J>,
    s: &Transform<N>,
    t: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    // s is the string to search,
    // t is the string to find.
    let u = ctxt.dispatch(stctxt, s)?.to_string();
    let v = ctxt.dispatch(stctxt, t)?.to_string();
    match u.find(v.as_str()) {
        Some(i) => {
            match u.get(i + v.len()..u.len()) {
                Some(w) => Ok(vec![Item::Value(Rc::new(Value::from(w)))]),
                None => {
                    // This shouldn't happen!
                    Err(Error::new(
                        ErrorKind::Unknown,
                        String::from("unable to extract substring"),
                    ))
                }
            }
        }
        None => Ok(vec![]),
    }
}

/// XPath normalize-space function.
pub fn normalize_space<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
    J: FnMut(&Context<N>) -> Result<Sequence<N>, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H, J>,
    n: &Option<Box<Transform<N>>>,
) -> Result<Sequence<N>, Error> {
    let s: Result<String, Error> = n.as_ref().map_or_else(
        || {
            // Use the current item
            Ok(ctxt.cur[ctxt.i].to_string())
        },
        |m| {
            let t = ctxt.dispatch(stctxt, m)?;
            Ok(t.to_string())
        },
    );
    // intersperse is the right iterator to use, but it is only available in nightly at the moment
    s.map(|u| {
        vec![Item::Value(Rc::new(Value::from(
            u.split_whitespace().collect::<Vec<&str>>().join(" "),
        )))]
    })
}

/// XPath translate function.
pub fn translate<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
    J: FnMut(&Context<N>) -> Result<Sequence<N>, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H, J>,
    s: &Transform<N>,
    map: &Transform<N>,
    trn: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    // s is the string to search
    // map are the map chars
    // trn are the translate chars
    let o = ctxt.dispatch(stctxt, map)?.to_string();
    let m: Vec<&str> = o.graphemes(true).collect();
    let u = ctxt.dispatch(stctxt, trn)?.to_string();
    let t: Vec<&str> = u.graphemes(true).collect();
    let mut result: String = String::new();

    for c in ctxt.dispatch(stctxt, s)?.to_string().graphemes(true) {
        let mut a: Option<Option<usize>> = Some(None);
        for (i, _item) in m.iter().enumerate() {
            if c == m[i] {
                if i < t.len() {
                    a = Some(Some(i));
                    break;
                } else {
                    // omit this character
                    a = None
                }
            } else {
                // keep looking for a match
            }
        }
        match a {
            Some(None) => {
                result.push_str(c);
            }
            Some(Some(j)) => result.push_str(t[j]),
            None => {
                // omit char
            }
        }
    }
    Ok(vec![Item::Value(Rc::new(Value::from(result)))])
}

/// XPath concat function. All arguments are concatenated into a single string value.
pub(crate) fn tr_concat<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
    J: FnMut(&Context<N>) -> Result<Sequence<N>, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H, J>,
    arguments: &Vec<Transform<N>>,
) -> Result<Sequence<N>, Error> {
    match arguments
        .iter()
        .try_fold(String::new(), |mut acc, a| match ctxt.dispatch(stctxt, a) {
            Ok(b) => {
                acc.push_str(b.to_string().as_str());
                Ok(acc)
            }
            Err(err) => Err(err),
        }) {
        Ok(r) => Ok(vec![Item::Value(Rc::new(Value::from(r)))]),
        Err(err) => Err(err),
    }
}
