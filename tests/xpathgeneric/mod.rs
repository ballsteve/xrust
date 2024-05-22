//! Tests for XPath defined generically

use std::collections::HashMap;
use xrust::xdmerror::{Error, ErrorKind};
use xrust::item::{Item, Node, NodeType, Sequence, SequenceTrait};
use xrust::parser::xpath::parse;
use xrust::transform::context::{Context, ContextBuilder, StaticContext};

type F = Box<dyn FnMut(&str) -> Result<(), Error>>;

fn no_src_no_result<N: Node>(e: impl AsRef<str>) -> Result<Sequence<N>, Error> {
    Context::new().dispatch(&mut StaticContext::<F>::new(), &parse(e.as_ref())?)
}

fn dispatch_rig<N: Node, G, H>(
    e: impl AsRef<str>,
    make_empty_doc: G,
    make_doc: H,
) -> Result<Sequence<N>, Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let rd = make_empty_doc();
    ContextBuilder::new()
        .context(vec![make_doc()])
        .result_document(rd)
        .build()
        .dispatch(&mut StaticContext::<F>::new(), &parse(e.as_ref())?)
}

pub fn generic_empty<N: Node>() -> Result<(), Error>
{
    let result: Sequence<N> = no_src_no_result("")?;
    if result.len() == 0 {
        Ok(())
    } else { Err(Error::new(ErrorKind::Unknown, format!("got result \"{}\", expected \"\"", result.to_string()))) }
}

pub fn generic_step_1_pos<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let result: Sequence<N> = dispatch_rig("child::a", make_empty_doc, make_doc)?;
    if result.len() == 1 {
        match &result[0] {
            Item::Node(n) => {
                match (n.node_type(), n.name().to_string().as_str()) {
                    (NodeType::Element, "a") => Ok(()),
                    (NodeType::Element, _) => Err(Error::new(ErrorKind::Unknown, format!("got element named \"{}\", expected \"a\"", result[0].name().to_string()))),
                    _ => Err(Error::new(ErrorKind::Unknown, "not an element type node")),
                }
            }
            _ => Err(Error::new(ErrorKind::Unknown, "not a node"))
        }
    } else { Err(Error::new(ErrorKind::Unknown, format!("got result \"{}\", expected \"\"", result.to_string()))) }
}

pub fn generic_path_1_pos<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let result: Sequence<N> = dispatch_rig("/child::a", make_empty_doc, make_doc)?;
    if result.len() == 1 {
        match &result[0] {
            Item::Node(n) => {
                match (n.node_type(), n.name().to_string().as_str()) {
                    (NodeType::Element, "a") => Ok(()),
                    (NodeType::Element, _) => Err(Error::new(ErrorKind::Unknown, format!("got element named \"{}\", expected \"a\"", result[0].name().to_string()))),
                    _ => Err(Error::new(ErrorKind::Unknown, "not an element type node")),
                }
            }
            _ => Err(Error::new(ErrorKind::Unknown, "not a node"))
        }
    } else { Err(Error::new(ErrorKind::Unknown, format!("got result \"{}\", expected \"\"", result.to_string()))) }
}

pub fn generic_path_1_neg<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let result: Sequence<N> = dispatch_rig("/child::b", make_empty_doc, make_doc)?;
    if result.len() == 0 {
        Ok(())
    } else { Err(Error::new(ErrorKind::Unknown, "found node, expected nothing")) }
}

pub fn generic_path_2<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let result: Sequence<N> = dispatch_rig("/child::a/child::b", make_empty_doc, make_doc)?;
    if result.len() == 2 {
        match &result[0] {
            Item::Node(n) => {
                match (n.node_type(), n.name().to_string().as_str()) {
                    (NodeType::Element, "b") => Ok(()),
                    (NodeType::Element, _) => Err(Error::new(ErrorKind::Unknown, format!("got element named \"{}\", expected \"a\"", result[0].name().to_string()))),
                    _ => Err(Error::new(ErrorKind::Unknown, "not an element type node")),
                }
            }
            _ => Err(Error::new(ErrorKind::Unknown, "not a node"))
        }
    } else { Err(Error::new(ErrorKind::Unknown, format!("got {} results, expected 0", result.len()))) }
}

pub fn generic_generate_id<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let result: Sequence<N> = dispatch_rig("generate-id()", make_empty_doc, make_doc)?;
    if result.len() == 1 {
        match &result[0] {
            Item::Value(v) => {
                if v.to_string().len() > 0 {
                    Ok(())
                } else {
                    Err(Error::new(ErrorKind::Unknown, "expected non-empty string"))
                }
            }
            _ => Err(Error::new(ErrorKind::Unknown, "not a value"))
        }
    } else { Err(Error::new(ErrorKind::Unknown, format!("got {} results, expected 1", result.len()))) }
}

fn unimplemented_rig<N: Node, G, H>(
    e: impl AsRef<str>,
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    match dispatch_rig(e, make_empty_doc, make_doc) {
        Err(e) => {
            if e.kind == ErrorKind::NotImplemented {
                Ok(())
            } else {
                Err(Error::new(ErrorKind::Unknown, "unexpected error code"))
            }
        }
        _ => Err(Error::new(ErrorKind::Unknown, "expected to fail")),
    }
}

pub fn generic_union<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    unimplemented_rig("'a' | 'b'", make_empty_doc, make_doc)
}
pub fn generic_intersectexcept<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    unimplemented_rig("'a' intersect 'b' except 'c'", make_empty_doc, make_doc)
}
pub fn generic_instanceof<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    unimplemented_rig("'a' instance of empty-sequence()", make_empty_doc, make_doc)
}
pub fn generic_treat<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    unimplemented_rig("'a' treat as empty-sequence()", make_empty_doc, make_doc)
}
pub fn generic_castable<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    unimplemented_rig("'a' castable as type?", make_empty_doc, make_doc)
}
pub fn generic_cast<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    unimplemented_rig("'a' cast as type?", make_empty_doc, make_doc)
}
pub fn generic_arrow<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    unimplemented_rig("'a' => spec()", make_empty_doc, make_doc)
}
pub fn generic_unary<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    unimplemented_rig("+'a'", make_empty_doc, make_doc)
}
pub fn generic_simplemap<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    unimplemented_rig("'a'!'b'", make_empty_doc, make_doc)
}
