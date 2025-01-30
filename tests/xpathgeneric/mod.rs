//! Tests for XPath defined generically

use pkg_version::{pkg_version_major, pkg_version_minor, pkg_version_patch};
use std::rc::Rc;
use xrust::item::{Item, Node, NodeType, Sequence, SequenceTrait};
use xrust::parser::xpath::parse;
use xrust::pattern::Pattern;
use xrust::qname::QualifiedName;
use xrust::qname_in::{new, new_map, Internment, QualifiedName as InQualifiedName};
use xrust::transform::callable::ActualParameters;
use xrust::transform::context::{Context, ContextBuilder, StaticContextBuilder};
use xrust::transform::{Axis, KindTest, NodeMatch, NodeTest, Transform};
use xrust::value::Value;
use xrust::xdmerror::{Error, ErrorKind};

fn no_src_no_result<N: Node>(e: impl AsRef<str>) -> Result<Sequence<N>, Error> {
    let mut intern = new_map();
    let mut stctxt = StaticContextBuilder::new(&mut intern)
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    Context::new().dispatch(&mut stctxt, &parse(e.as_ref(), None)?)
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
    let mut intern = new_map();
    let mut stctxt = StaticContextBuilder::new(&mut intern)
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let rd = make_empty_doc();
    ContextBuilder::new()
        .context(vec![make_doc()])
        .result_document(rd)
        .build()
        .dispatch(&mut stctxt, &parse(e.as_ref(), None)?)
}

pub fn generic_empty<N: Node>() -> Result<(), Error> {
    let result: Sequence<N> = no_src_no_result("")?;
    if result.len() == 0 {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!("got result \"{}\", expected \"\"", result.to_string()),
        ))
    }
}

pub fn generic_step_1_pos<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let result: Sequence<N> = dispatch_rig("child::a", make_empty_doc, make_doc)?;
    if result.len() == 1 {
        match &result[0] {
            Item::Node(n) => match (n.node_type(), n.name().to_string().as_str()) {
                (NodeType::Element, "a") => Ok(()),
                (NodeType::Element, _) => Err(Error::new(
                    ErrorKind::Unknown,
                    format!(
                        "got element named \"{}\", expected \"a\"",
                        result[0].name().to_string()
                    ),
                )),
                _ => Err(Error::new(ErrorKind::Unknown, "not an element type node")),
            },
            _ => Err(Error::new(ErrorKind::Unknown, "not a node")),
        }
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!("got result \"{}\", expected \"\"", result.to_string()),
        ))
    }
}
pub fn generic_step_2_pos<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // Abbreviated from child::a
    let result: Sequence<N> = dispatch_rig("a", make_empty_doc, make_doc)?;
    if result.len() == 1 {
        match &result[0] {
            Item::Node(n) => match (n.node_type(), n.name().to_string().as_str()) {
                (NodeType::Element, "a") => Ok(()),
                (NodeType::Element, _) => Err(Error::new(
                    ErrorKind::Unknown,
                    format!(
                        "got element named \"{}\", expected \"a\"",
                        result[0].name().to_string()
                    ),
                )),
                _ => Err(Error::new(ErrorKind::Unknown, "not an element type node")),
            },
            _ => Err(Error::new(ErrorKind::Unknown, "not a node")),
        }
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!("got result \"{}\", expected \"\"", result.to_string()),
        ))
    }
}

pub fn generic_path_1_pos<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let result: Sequence<N> = dispatch_rig("/child::a", make_empty_doc, make_doc)?;
    if result.len() == 1 {
        match &result[0] {
            Item::Node(n) => match (n.node_type(), n.name().to_string().as_str()) {
                (NodeType::Element, "a") => Ok(()),
                (NodeType::Element, _) => Err(Error::new(
                    ErrorKind::Unknown,
                    format!(
                        "got element named \"{}\", expected \"a\"",
                        result[0].name().to_string()
                    ),
                )),
                _ => Err(Error::new(ErrorKind::Unknown, "not an element type node")),
            },
            _ => Err(Error::new(ErrorKind::Unknown, "not a node")),
        }
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!("got result \"{}\", expected \"\"", result.to_string()),
        ))
    }
}
pub fn generic_path_2_pos<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let result: Sequence<N> = dispatch_rig("/a", make_empty_doc, make_doc)?;
    if result.len() == 1 {
        match &result[0] {
            Item::Node(n) => match (n.node_type(), n.name().to_string().as_str()) {
                (NodeType::Element, "a") => Ok(()),
                (NodeType::Element, _) => Err(Error::new(
                    ErrorKind::Unknown,
                    format!(
                        "got element named \"{}\", expected \"a\"",
                        result[0].name().to_string()
                    ),
                )),
                _ => Err(Error::new(ErrorKind::Unknown, "not an element type node")),
            },
            _ => Err(Error::new(ErrorKind::Unknown, "not a node")),
        }
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!("got result \"{}\", expected \"\"", result.to_string()),
        ))
    }
}

pub fn generic_path_1_neg<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let result: Sequence<N> = dispatch_rig("/child::b", make_empty_doc, make_doc)?;
    if result.len() == 0 {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            "found node, expected nothing",
        ))
    }
}

pub fn generic_path_3<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let result: Sequence<N> = dispatch_rig("/child::a/child::b", make_empty_doc, make_doc)?;
    if result.len() == 2 {
        match &result[0] {
            Item::Node(n) => match (n.node_type(), n.name().to_string().as_str()) {
                (NodeType::Element, "b") => Ok(()),
                (NodeType::Element, _) => Err(Error::new(
                    ErrorKind::Unknown,
                    format!(
                        "got element named \"{}\", expected \"a\"",
                        result[0].name().to_string()
                    ),
                )),
                _ => Err(Error::new(ErrorKind::Unknown, "not an element type node")),
            },
            _ => Err(Error::new(ErrorKind::Unknown, "not a node")),
        }
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!("got {} results, expected 0", result.len()),
        ))
    }
}
pub fn generic_path_4<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let result: Sequence<N> = dispatch_rig("/a/b", make_empty_doc, make_doc)?;
    if result.len() == 2 {
        match &result[0] {
            Item::Node(n) => match (n.node_type(), n.name().to_string().as_str()) {
                (NodeType::Element, "b") => Ok(()),
                (NodeType::Element, _) => Err(Error::new(
                    ErrorKind::Unknown,
                    format!(
                        "got element named \"{}\", expected \"a\"",
                        result[0].name().to_string()
                    ),
                )),
                _ => Err(Error::new(ErrorKind::Unknown, "not an element type node")),
            },
            _ => Err(Error::new(ErrorKind::Unknown, "not a node")),
        }
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!("got {} results, expected 0", result.len()),
        ))
    }
}

pub fn generic_root_desc_or_self_1<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = dispatch_rig("//child::a", make_empty_doc, make_doc)?;
    assert_eq!(s.len(), 5);
    for t in s {
        match &t {
            Item::Node(n) => {
                assert_eq!(n.node_type(), NodeType::Element);
                assert_eq!(n.name().to_string(), "a")
            }
            _ => panic!("not a node"),
        }
    }
    Ok(())
}

pub fn generic_root_desc_or_self_2<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = dispatch_rig("//child::a/child::b", make_empty_doc, make_doc)?;
    assert_eq!(s.len(), 10);
    for t in s {
        match &t {
            Item::Node(n) => {
                assert_eq!(n.node_type(), NodeType::Element);
                assert_eq!(n.name().to_string(), "b")
            }
            _ => panic!("not a node"),
        }
    }
    Ok(())
}

pub fn generic_root_desc_or_self_3<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = dispatch_rig("//child::a//child::b", make_empty_doc, make_doc)?;
    assert_eq!(s.len(), 10);
    for t in s {
        match &t {
            Item::Node(n) => {
                assert_eq!(n.node_type(), NodeType::Element);
                assert_eq!(n.name().to_string(), "b")
            }
            _ => panic!("not a node"),
        }
    }
    Ok(())
}

pub fn generic_rel_path_1<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = dispatch_rig("child::a/child::b", make_empty_doc, make_doc)?;
    assert_eq!(s.len(), 2);
    for t in s {
        match &t {
            Item::Node(n) => {
                assert_eq!(n.node_type(), NodeType::Element);
                assert_eq!(n.name().to_string(), "b")
            }
            _ => panic!("not a node"),
        }
    }
    Ok(())
}

pub fn generic_rel_path_2<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = dispatch_rig("child::a//child::b", make_empty_doc, make_doc)?;
    assert_eq!(s.len(), 10);
    for t in s {
        match &t {
            Item::Node(n) => {
                assert_eq!(n.node_type(), NodeType::Element);
                assert_eq!(n.name().to_string(), "b")
            }
            _ => panic!("not a node"),
        }
    }
    Ok(())
}

pub fn generic_step_2<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = dispatch_rig("child::bc", make_empty_doc, make_doc)?;
    assert_eq!(s.len(), 0);
    Ok(())
}

pub fn generic_step_wild_1<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = dispatch_rig("child::*", make_empty_doc, make_doc)?;
    assert_eq!(s.len(), 1);
    for t in s {
        match &t {
            Item::Node(n) => {
                assert_eq!(n.node_type(), NodeType::Element);
                assert_eq!(n.name().to_string(), "a")
            }
            _ => panic!("not a node"),
        }
    }
    Ok(())
}

pub fn generic_step_attribute_1<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = dispatch_rig("/child::*/attribute::id", make_empty_doc, make_doc)?;
    assert_eq!(s.len(), 1);
    for t in s {
        match &t {
            Item::Node(n) => {
                assert_eq!(n.node_type(), NodeType::Attribute);
                assert_eq!(n.name().to_string(), "id");
                assert_eq!(n.value().to_string(), "a1")
            }
            _ => panic!("not a node"),
        }
    }
    Ok(())
}
pub fn generic_step_attribute_2<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = dispatch_rig("/child::*/@id", make_empty_doc, make_doc)?;
    assert_eq!(s.len(), 1);
    for t in s {
        match &t {
            Item::Node(n) => {
                assert_eq!(n.node_type(), NodeType::Attribute);
                assert_eq!(n.name().to_string(), "id");
                assert_eq!(n.value().to_string(), "a1")
            }
            _ => panic!("not a node"),
        }
    }
    Ok(())
}
/*
pub fn generic_step_parent_1<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let rd = make_empty_doc();
    let sd = make_doc();
    match &sd {
        Item::Node(c) => {
            let l = c.descend_iter().last().unwrap();
            let mut stctxt = StaticContextBuilder::new()
                .message(|_| Ok(()))
                .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
                .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
                .build();
            let s = ContextBuilder::new()
                .context(vec![Item::Node(l)])
                .result_document(rd)
                .build()
                .dispatch(&mut stctxt, &parse("parent::a", None)?)?;
            assert_eq!(s.len(), 1);
            assert_eq!(s[0].name().to_string(), "a")
        }
        _ => panic!("unable to unpack node"),
    }
    Ok(())
}
pub fn generic_step_parent_2<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let rd = make_empty_doc();
    let sd = make_doc();
    match &sd {
        Item::Node(c) => {
            let l = c.descend_iter().last().unwrap();
            let mut stctxt = StaticContextBuilder::new()
                .message(|_| Ok(()))
                .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
                .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
                .build();
            let s = ContextBuilder::new()
                .context(vec![Item::Node(l)])
                .result_document(rd)
                .build()
                .dispatch(&mut stctxt, &parse("..", None)?)?;
            assert_eq!(s.len(), 1);
            assert_eq!(s[0].name().to_string(), "a")
        }
        _ => panic!("unable to unpack node"),
    }
    Ok(())
}

pub fn generic_step_wild_2<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let rd = make_empty_doc();
    let sd = make_doc();
    match &sd {
        Item::Node(c) => {
            let l = c.descend_iter().last().unwrap();
            let mut stctxt = StaticContextBuilder::new()
                .message(|_| Ok(()))
                .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
                .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
                .build();
            let s = ContextBuilder::new()
                .context(vec![Item::Node(l)])
                .result_document(rd)
                .build()
                .dispatch(&mut stctxt, &parse("ancestor::*", None)?)?;
            assert_eq!(s.len(), 3);
        }
        _ => panic!("unable to unpack node"),
    }
    Ok(())
}

pub fn generic_generate_id<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
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
            _ => Err(Error::new(ErrorKind::Unknown, "not a value")),
        }
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!("got {} results, expected 1", result.len()),
        ))
    }
}

pub fn generic_xpath_context_item<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let rd = make_empty_doc();
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let s = ContextBuilder::new()
        .context(vec![Item::Value(Rc::new(Value::from("foobar")))])
        .result_document(rd)
        .build()
        .dispatch(&mut stctxt, &parse(".", None)?)?;
    assert_eq!(s.len(), 1);
    assert_eq!(s[0].to_string(), "foobar");
    Ok(())
}

pub fn generic_parens_singleton<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let rd = make_empty_doc();
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let s = ContextBuilder::new()
        .context(vec![Item::Value(Rc::new(Value::from("foobar")))])
        .result_document(rd)
        .build()
        .dispatch(&mut stctxt, &parse("(1)", None)?)?;
    assert_eq!(s.len(), 1);
    assert_eq!(s[0].to_int().unwrap(), 1);
    Ok(())
}

pub fn generic_int<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let result: Sequence<N> = no_src_no_result("1")?;
    if result.len() == 1 {
        match &result[0] {
            Item::Value(v) => {
                if v.to_int().unwrap() == 1 {
                    Ok(())
                } else {
                    Err(Error::new(ErrorKind::Unknown, "expected integer value"))
                }
            }
            _ => Err(Error::new(ErrorKind::Unknown, "not a value")),
        }
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!("got {} results, expected 1", result.len()),
        ))
    }
}

pub fn generic_decimal<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let result: Sequence<N> = no_src_no_result("1.2")?;
    if result.len() == 1 {
        match &result[0] {
            Item::Value(v) => {
                if v.to_double() == 1.2 {
                    Ok(())
                } else {
                    Err(Error::new(ErrorKind::Unknown, "expected double value"))
                }
            }
            _ => Err(Error::new(ErrorKind::Unknown, "not a value")),
        }
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!("got {} results, expected 1", result.len()),
        ))
    }
}

pub fn generic_exponent<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let result: Sequence<N> = no_src_no_result("1.2e2")?;
    if result.len() == 1 {
        match &result[0] {
            Item::Value(v) => {
                if v.to_double() == 120.0 {
                    Ok(())
                } else {
                    Err(Error::new(ErrorKind::Unknown, "expected double value"))
                }
            }
            _ => Err(Error::new(ErrorKind::Unknown, "not a value")),
        }
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!("got {} results, expected 1", result.len()),
        ))
    }
}

pub fn generic_string_apos<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let result: Sequence<N> = no_src_no_result("'abc'")?;
    if result.len() == 1 {
        match &result[0] {
            Item::Value(v) => {
                if v.to_string() == "abc" {
                    Ok(())
                } else {
                    Err(Error::new(ErrorKind::Unknown, "expected string value"))
                }
            }
            _ => Err(Error::new(ErrorKind::Unknown, "not a value")),
        }
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!("got {} results, expected 1", result.len()),
        ))
    }
}

pub fn generic_string_apos_esc<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let result: Sequence<N> = no_src_no_result("'abc''def'")?;
    if result.len() == 1 {
        match &result[0] {
            Item::Value(v) => {
                if v.to_string() == "abc'def" {
                    Ok(())
                } else {
                    Err(Error::new(ErrorKind::Unknown, "expected string value"))
                }
            }
            _ => Err(Error::new(ErrorKind::Unknown, "not a value")),
        }
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!("got {} results, expected 1", result.len()),
        ))
    }
}

pub fn generic_string_quot<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let result: Sequence<N> = no_src_no_result(r#""abc""#)?;
    if result.len() == 1 {
        match &result[0] {
            Item::Value(v) => {
                if v.to_string() == "abc" {
                    Ok(())
                } else {
                    Err(Error::new(ErrorKind::Unknown, "expected string value"))
                }
            }
            _ => Err(Error::new(ErrorKind::Unknown, "not a value")),
        }
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!("got {} results, expected 1", result.len()),
        ))
    }
}

pub fn generic_string_quot_esc<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let result: Sequence<N> = no_src_no_result(r#""abc""def""#)?;
    if result.len() == 1 {
        match &result[0] {
            Item::Value(v) => {
                if v.to_string() == r#"abc"def"# {
                    Ok(())
                } else {
                    Err(Error::new(ErrorKind::Unknown, "expected string value"))
                }
            }
            _ => Err(Error::new(ErrorKind::Unknown, "not a value")),
        }
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!("got {} results, expected 1", result.len()),
        ))
    }
}

pub fn generic_literal_sequence<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let result: Sequence<N> = no_src_no_result("1,'abc',2")?;
    if result.len() == 3 {
        match &result[0] {
            Item::Value(v) => {
                if v.to_int().unwrap() == 1 {
                    assert_eq!(result[1].to_string(), "abc");
                    assert_eq!(result[2].to_int().unwrap(), 2);
                    Ok(())
                } else {
                    Err(Error::new(ErrorKind::Unknown, "expected integer value"))
                }
            }
            _ => Err(Error::new(ErrorKind::Unknown, "not a value")),
        }
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!("got {} results, expected 1", result.len()),
        ))
    }
}

pub fn generic_literal_sequence_ws<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("1 , 'abc', 2")?;
    assert_eq!(s.len(), 3);
    assert_eq!(s[0].to_int().unwrap(), 1);
    assert_eq!(s[1].to_string(), "abc");
    assert_eq!(s[2].to_int().unwrap(), 2);
    Ok(())
}

pub fn generic_xpath_comment<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> =
        no_src_no_result("1(::),(: a comment :)'abc', (: outer (: inner :) outer :) 2")?;
    assert_eq!(s.len(), 3);
    assert_eq!(s[0].to_int().unwrap(), 1);
    assert_eq!(s[1].to_string(), "abc");
    assert_eq!(s[2].to_int().unwrap(), 2);
    Ok(())
}

pub fn generic_kindtest_text_abbrev<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("text()")?;
    assert_eq!(s.len(), 0);
    Ok(())
}
pub fn generic_kindtest_text_full<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("child::text()")?;
    assert_eq!(s.len(), 0);
    Ok(())
}

pub fn generic_fncall_string<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("string(('a', 'b', 'c'))")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_string(), "abc");
    Ok(())
}

pub fn generic_fncall_current_1<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("current()")?;
    assert_eq!(s.len(), 0);
    assert_eq!(s.to_string(), "");
    Ok(())
}

pub fn generic_fncall_current_2<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = dispatch_rig("current()/child::a", make_empty_doc, make_doc)?;
    assert_eq!(s.len(), 1);
    Ok(())
}

pub fn generic_fncall_current_3<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let rd = make_empty_doc();
    let sd = make_doc();
    if let Item::Node(ref doc) = sd {
        let top = doc.child_iter().nth(0).unwrap();
        let mut stctxt = StaticContextBuilder::new()
            .message(|_| Ok(()))
            .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
            .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
            .build();
        let s = ContextBuilder::new()
            .result_document(rd)
            .context(vec![Item::Node(top)])
            .previous_context(Some(sd))
            .build()
            .dispatch(&mut stctxt, &parse("current()/child::a", None)?)
            .expect("evaluation failed");
        assert_eq!(s.len(), 1)
    } else {
        panic!("not a node")
    }
    Ok(())
}

pub fn generic_fncall_concat<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("concat('a', 'b', 'c')")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_string(), "abc");
    Ok(())
}

pub fn generic_fncall_startswith_pos<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("starts-with('abc', 'a')")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_bool(), true);
    Ok(())
}
pub fn generic_fncall_startswith_neg<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("starts-with('abc', 'b')")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_bool(), false);
    Ok(())
}

pub fn generic_fncall_contains_pos<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("contains('abc', 'b')")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_bool(), true);
    Ok(())
}
pub fn generic_fncall_contains_neg<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("contains('abc', 'd')")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_bool(), false);
    Ok(())
}

pub fn generic_fncall_substring_2arg<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("substring('abcdefg', 4)")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_string(), "defg");
    Ok(())
}
pub fn generic_fncall_substring_3arg<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("substring('abcdefg', 4, 2)")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_string(), "de");
    Ok(())
}

pub fn generic_fncall_substringbefore_pos<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("substring-before('abc', 'b')")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_string(), "a");
    Ok(())
}
pub fn generic_fncall_substringbefore_neg<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("substring-before('abc', 'd')")?;
    assert_eq!(s.to_string(), "");
    Ok(())
}

pub fn generic_fncall_substringafter_pos_1<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("substring-after('abc', 'b')")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_string(), "c");
    Ok(())
}
pub fn generic_fncall_substringafter_pos_2<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("substring-after('abc', 'c')")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_string(), "");
    Ok(())
}
pub fn generic_fncall_substringafter_neg<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("substring-after('abc', 'd')")?;
    assert_eq!(s.to_string(), "");
    Ok(())
}

pub fn generic_fncall_normalizespace<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("normalize-space('	a  b\nc 	')")?;
    assert_eq!(s.to_string(), "a b c");
    Ok(())
}

pub fn generic_fncall_translate<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("translate('abcdeabcde', 'ade', 'XY')")?;
    assert_eq!(s.to_string(), "XbcYXbcY");
    Ok(())
}

pub fn generic_fncall_boolean_true<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("boolean('abcdeabcde')")?;
    assert_eq!(s.len(), 1);
    match &s[0] {
        Item::Value(v) => match **v {
            Value::Boolean(b) => assert_eq!(b, true),
            _ => panic!("not a singleton boolean true value"),
        },
        _ => panic!("not a value"),
    }
    Ok(())
}
pub fn generic_fncall_boolean_false<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("boolean('')")?;
    assert_eq!(s.len(), 1);
    match &s[0] {
        Item::Value(v) => match **v {
            Value::Boolean(b) => assert_eq!(b, false),
            _ => panic!("not a singleton boolean true value"),
        },
        _ => panic!("not a value"),
    }
    Ok(())
}

pub fn generic_fncall_not_true<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("not('')")?;
    assert_eq!(s.len(), 1);
    match &s[0] {
        Item::Value(v) => match **v {
            Value::Boolean(b) => assert_eq!(b, true),
            _ => panic!("not a singleton boolean true value"),
        },
        _ => panic!("not a value"),
    }
    Ok(())
}
pub fn generic_fncall_not_false<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("not('abc')")?;
    assert_eq!(s.len(), 1);
    match &s[0] {
        Item::Value(v) => match **v {
            Value::Boolean(b) => assert_eq!(b, false),
            _ => panic!("not a singleton boolean true value"),
        },
        _ => panic!("not a value"),
    }
    Ok(())
}

pub fn generic_fncall_true<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("true()")?;
    assert_eq!(s.len(), 1);
    match &s[0] {
        Item::Value(v) => match **v {
            Value::Boolean(b) => assert_eq!(b, true),
            _ => panic!("not a singleton boolean true value"),
        },
        _ => panic!("not a value"),
    }
    Ok(())
}
pub fn generic_fncall_false<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("false()")?;
    assert_eq!(s.len(), 1);
    match &s[0] {
        Item::Value(v) => match **v {
            Value::Boolean(b) => assert_eq!(b, false),
            _ => panic!("not a singleton boolean true value"),
        },
        _ => panic!("not a value"),
    }
    Ok(())
}

pub fn generic_fncall_number_int<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("number('123')")?;
    assert_eq!(s.len(), 1);
    match &s[0] {
        Item::Value(v) => match **v {
            Value::Integer(i) => assert_eq!(i, 123),
            _ => panic!("not a singleton integer value, got \"{}\"", s.to_string()),
        },
        _ => panic!("not a value"),
    }
    Ok(())
}
pub fn generic_fncall_number_double<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("number('123.456')")?;
    assert_eq!(s.len(), 1);
    match &s[0] {
        Item::Value(v) => match **v {
            Value::Double(d) => assert_eq!(d, 123.456),
            _ => panic!("not a singleton double value, got \"{}\"", s.to_string()),
        },
        _ => panic!("not a value"),
    }
    Ok(())
}

pub fn generic_fncall_sum<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("sum(('123.456', 10, 20, '0'))")?;
    assert_eq!(s.len(), 1);
    match &s[0] {
        Item::Value(v) => match **v {
            Value::Double(d) => assert_eq!(d, 123.456 + 10.0 + 20.0),
            _ => panic!("not a singleton double value"),
        },
        _ => panic!("not a value"),
    }
    Ok(())
}

pub fn generic_fncall_avg<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("avg(('123.456', 10, 20, '0'))")?;
    assert_eq!(s.len(), 1);
    match &s[0] {
        Item::Value(v) => match **v {
            Value::Double(d) => assert!(d - 38.364 < 0.01),
            _ => panic!("not a singleton double value"),
        },
        _ => panic!("not a value"),
    }
    Ok(())
}

pub fn generic_fncall_min<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("min(('123.456', 10, 20, '0'))")?;
    assert_eq!(s.len(), 1);
    match &s[0] {
        Item::Value(v) => match **v {
            Value::Double(d) => assert_eq!(d, 0.0),
            _ => panic!("not a singleton double value"),
        },
        _ => panic!("not a value"),
    }
    Ok(())
}

pub fn generic_fncall_max<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("max(('123.456', 10, 20, '0'))")?;
    assert_eq!(s.len(), 1);
    match &s[0] {
        Item::Value(v) => match **v {
            Value::Double(d) => assert_eq!(d, 123.456),
            _ => panic!("not a singleton double value"),
        },
        _ => panic!("not a value"),
    }
    Ok(())
}

pub fn generic_fncall_floor<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("floor(123.456)")?;
    assert_eq!(s.len(), 1);
    match &s[0] {
        Item::Value(v) => match **v {
            Value::Double(d) => assert_eq!(d, 123.0),
            _ => panic!("not a singleton double value"),
        },
        _ => panic!("not a value"),
    }
    Ok(())
}

pub fn generic_fncall_ceiling<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("ceiling(123.456)")?;
    assert_eq!(s.len(), 1);
    match &s[0] {
        Item::Value(v) => match **v {
            Value::Double(d) => assert_eq!(d, 124.0),
            _ => panic!("not a singleton double value"),
        },
        _ => panic!("not a value"),
    }
    Ok(())
}

pub fn generic_fncall_round_down<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("round(123.456)")?;
    assert_eq!(s.len(), 1);
    match &s[0] {
        Item::Value(v) => match **v {
            Value::Double(d) => assert_eq!(d, 123.0),
            _ => panic!("not a singleton double value"),
        },
        _ => panic!("not a value"),
    }
    Ok(())
}
pub fn generic_fncall_round_up<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("round(123.654)")?;
    assert_eq!(s.len(), 1);
    match &s[0] {
        Item::Value(v) => match **v {
            Value::Double(d) => assert_eq!(d, 124.0),
            _ => panic!("not a singleton double value"),
        },
        _ => panic!("not a value"),
    }
    Ok(())
}

pub fn generic_fncall_count_1<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("count((1, 2, 3, 4))")?;
    assert_eq!(s.len(), 1);
    match &s[0] {
        Item::Value(v) => match **v {
            Value::Integer(d) => assert_eq!(d, 4),
            _ => panic!("not a singleton integer value"),
        },
        _ => panic!("not a value"),
    }
    Ok(())
}
pub fn generic_fncall_count_2<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let rd = make_empty_doc();
    let sd = make_doc();
    if let Item::Node(ref doc) = sd {
        let l = doc.descend_iter().last().unwrap();
        let mut stctxt = StaticContextBuilder::new()
            .message(|_| Ok(()))
            .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
            .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
            .build();
        let s = ContextBuilder::new()
            .result_document(rd)
            .context(vec![Item::Node(l)])
            .previous_context(Some(sd))
            .build()
            .dispatch(&mut stctxt, &parse("count(ancestor::*)", None)?)
            .expect("evaluation failed");
        assert_eq!(s.len(), 1);
        assert_eq!(s.to_int().expect("unable to get int from sequence"), 3)
    } else {
        panic!("not a node")
    }
    Ok(())
}

pub fn generic_format_number_1<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("format-number(456.789, '#.##')")?;
    assert_eq!(s.len(), 1);
    match &s[0] {
        Item::Value(v) => match &**v {
            Value::String(d) => assert_eq!(d, "456.79"),
            _ => panic!("not a singleton double value"),
        },
        _ => panic!("not a value"),
    }
    Ok(())
}

pub fn generic_fncall_user_defined<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let e: Transform<N> =
        parse("test:my_func(123)", None).expect("failed to parse expression \"test:my_func(123)\"");
    match e {
        Transform::Invoke(qn, ap, _) => {
            assert_eq!(
                qn,
                Rc::new(QualifiedName::new(
                    None,
                    Some("test".to_string()),
                    "my_func".to_string()
                ))
            );
            match ap {
                ActualParameters::Positional(v) => {
                    assert_eq!(v.len(), 1);
                    match &v[0] {
                        Transform::Literal(Item::Value(u)) => {
                            assert_eq!(u.to_int().expect("not an integer"), 123)
                        }
                        _ => panic!("not a literal integer"),
                    }
                }
                _ => panic!("Not positional parameters"),
            }
        }
        _ => panic!("Not an invocation"),
    }
    Ok(())
}

// Variables

pub fn generic_let_1<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("let $x := 'a' return ($x, $x)")?;
    assert_eq!(s.len(), 2);
    assert_eq!(s.to_string(), "aa");
    Ok(())
}
pub fn generic_let_2<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("let $x := 'a', $y := 'b' return ($x, $y)")?;
    assert_eq!(s.len(), 2);
    assert_eq!(s.to_string(), "ab");
    Ok(())
}

// Loops

pub fn generic_for_1<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("for $x in ('a', 'b', 'c') return ($x, $x)")?;
    assert_eq!(s.len(), 6);
    assert_eq!(s.to_string(), "aabbcc");
    Ok(())
}
pub fn generic_for_2<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("for $x in (1, 2, 3) return $x * 2")?;
    assert_eq!(s.len(), 3);
    assert_eq!(s.to_string(), "246");
    Ok(())
}

// Conditionals

pub fn generic_if_1<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("if (1) then 'one' else 'not one'")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_string(), "one");
    Ok(())
}
pub fn generic_if_2<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("if (0) then 'one' else 'not one'")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_string(), "not one");
    Ok(())
}
pub fn generic_issue_95<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let result: Sequence<N> = dispatch_rig("@*|node()", make_empty_doc, make_doc)?;
    if result.len() == 1 {
        match &result[0] {
            Item::Node(n) => match (n.node_type(), n.name().to_string().as_str()) {
                (NodeType::Element, "a") => Ok(()),
                (NodeType::Element, _) => Err(Error::new(
                    ErrorKind::Unknown,
                    format!(
                        "got element named \"{}\", expected \"a\"",
                        result[0].name().to_string()
                    ),
                )),
                _ => Err(Error::new(ErrorKind::Unknown, "not an element type node")),
            },
            _ => Err(Error::new(ErrorKind::Unknown, "not a node")),
        }
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!("got {} results, expected 0", result.len()),
        ))
    }
}

pub fn generic_navigate_predicate_1<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let rd = make_empty_doc();
    let sd = make_doc();
    if let Item::Node(d) = sd {
        let xform = parse("../ *[@id eq 'b6']", None).expect("parsing failed");
        let mut stctxt = StaticContextBuilder::new()
            .message(|_| Ok(()))
            .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
            .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
            .build();
        let s = ContextBuilder::new()
            .context(vec![Item::Node(
                d.first_child().unwrap().first_child().unwrap(),
            )])
            .result_document(rd)
            .build()
            .dispatch(&mut stctxt, &xform)
            .expect("transform failed");
        s.iter().for_each(|x| eprintln!("got item {:?}", x));
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].name().to_string(), "b");
        Ok(())
    } else {
        panic!("unable to unpack node")
    }
}

// System properties

pub fn generic_sys_prop_vers_qual<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> =
        no_src_no_result("system-property('Q{http://www.w3.org/1999/XSL/Transform}version')")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_string(), "0.9");
    Ok(())
}
pub fn generic_sys_prop_product_vers<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result(
        "system-property('Q{http://www.w3.org/1999/XSL/Transform}product-version')",
    )?;
    assert_eq!(s.len(), 1);
    assert_eq!(
        s.to_string(),
        format!(
            "{}.{}.{}",
            pkg_version_major!(),
            pkg_version_minor!(),
            pkg_version_patch!()
        )
    );
    Ok(())
}

pub fn generic_document_1<N: Node, G, H, J>(
    make_empty_doc: G,
    make_doc: H,
    make_from_str: J,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
    J: Fn(&str) -> Result<N, Error>,
{
    let mut msgs: Vec<String> = vec![];
    let mut stctxt = StaticContextBuilder::new()
        .message(|m| {
            msgs.push(m.to_string());
            Ok(())
        })
        .fetcher(|_| Ok(String::from("<Test>external document</Test>")))
        .parser(|s| make_from_str(s))
        .build();
    let rd = make_empty_doc();
    let seq: Sequence<N> = ContextBuilder::new()
        .context(vec![make_doc()])
        .result_document(rd)
        .build()
        .dispatch(
            &mut stctxt,
            &parse("document('urn:example.org/test')", None)
                .expect("unable to parse XPath expression"),
        )
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "external document");
    Ok(())
}

// Keys

pub fn generic_key_1<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let e: Transform<N> = parse("key('mykey', 'blue')", None)
        .expect("failed to parse expression \"key('mykey', 'blue'))\"");
    let mut sd = make_empty_doc();
    let mut top = sd
        .new_element(Rc::new(QualifiedName::new(None, None, String::from("Top"))))
        .expect("unable to create element");
    sd.push(top.clone()).expect("unable to add node");
    let mut red1 = sd
        .new_element(Rc::new(QualifiedName::new(None, None, String::from("one"))))
        .expect("unable to create element");
    red1.push(
        sd.new_text(Rc::new(Value::from("red")))
            .expect("unable to create text"),
    )
    .expect("unable to create element");
    top.push(red1).expect("unable to add node");
    let mut blue1 = sd
        .new_element(Rc::new(QualifiedName::new(None, None, String::from("two"))))
        .expect("unable to create element");
    blue1
        .push(
            sd.new_text(Rc::new(Value::from("blue")))
                .expect("unable to create text"),
        )
        .expect("unable to create element");
    top.push(blue1).expect("unable to add node");
    let mut yellow1 = sd
        .new_element(Rc::new(QualifiedName::new(
            None,
            None,
            String::from("three"),
        )))
        .expect("unable to create element");
    yellow1
        .push(
            sd.new_text(Rc::new(Value::from("yellow")))
                .expect("unable to create text"),
        )
        .expect("unable to create element");
    top.push(yellow1).expect("unable to add node");

    let mut ctxt = ContextBuilder::new()
        .context(vec![Item::Node(sd.clone())])
        .build();
    ctxt.declare_key(
        String::from("mykey"),
        Pattern::try_from("child::*").expect("unable to parse pattern"), // Top/ *
        Transform::Step(NodeMatch {
            axis: Axis::Child,
            nodetest: NodeTest::Kind(KindTest::Text),
        }),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    ctxt.populate_key_values(&mut stctxt, sd.clone())
        .expect("unable to populate key values");
    let seq = ctxt.dispatch(&mut stctxt, &e).expect("evaluation failed");

    assert_eq!(seq.len(), 1);
    assert_eq!(seq[0].name().to_string(), "two");
    Ok(())
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

pub fn generic_union<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    dispatch_rig("'a' | 'b'", make_empty_doc, make_doc).map_or(Ok(()), |v| {
        Err(Error::new(
            ErrorKind::TypeError,
            format!("expected type error, got \"{}\"", v.to_string()),
        ))
    })
}
pub fn generic_intersectexcept<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    unimplemented_rig("'a' intersect 'b' except 'c'", make_empty_doc, make_doc)
}
pub fn generic_instanceof<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    unimplemented_rig("'a' instance of empty-sequence()", make_empty_doc, make_doc)
}
pub fn generic_treat<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    unimplemented_rig("'a' treat as empty-sequence()", make_empty_doc, make_doc)
}
pub fn generic_castable<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    unimplemented_rig("'a' castable as type?", make_empty_doc, make_doc)
}
pub fn generic_cast<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    unimplemented_rig("'a' cast as type?", make_empty_doc, make_doc)
}
pub fn generic_arrow<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    unimplemented_rig("'a' => spec()", make_empty_doc, make_doc)
}
pub fn generic_unary<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    unimplemented_rig("+'a'", make_empty_doc, make_doc)
}
pub fn generic_simplemap<N: Node, G, H>(make_empty_doc: G, make_doc: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    unimplemented_rig("'a'!'b'", make_empty_doc, make_doc)
}
*/
