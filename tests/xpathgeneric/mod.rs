//! Tests for XPath defined generically

use std::rc::Rc;
use xrust::xdmerror::{Error, ErrorKind};
use xrust::value::Value;
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
            _ => panic!("not a node")
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
            _ => panic!("not a node")
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
            _ => panic!("not a node")
        }
    }
    Ok(())
}

pub fn generic_rel_path_1<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
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
            _ => panic!("not a node")
        }
    }
    Ok(())
}

pub fn generic_rel_path_2<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
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
            _ => panic!("not a node")
        }
    }
    Ok(())
}

pub fn generic_step_2<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let s: Sequence<N> = dispatch_rig("child::bc", make_empty_doc, make_doc)?;
    assert_eq!(s.len(), 0);
    Ok(())
}

pub fn generic_step_wild_1<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
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
            _ => panic!("not a node")
        }
    }
    Ok(())
}

pub fn generic_step_wild_2<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let rd = make_empty_doc();
    let sd = make_doc();
    match &sd {
        Item::Node(c) => {
            let l = c.descend_iter().last().unwrap();
            let s = ContextBuilder::new()
                .context(vec![Item::Node(l)])
                .result_document(rd)
                .build()
                .dispatch(&mut StaticContext::<F>::new(), &parse("ancestor::*")?)?;
                assert_eq!(s.len(), 3);
        }
        _ => panic!("unable to unpack node"),
    }
    Ok(())
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

pub fn generic_xpath_context_item<N: Node, G, H>(
    make_empty_doc: G,
    _: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let rd = make_empty_doc();
    let s = ContextBuilder::new()
        .context(vec![Item::Value(Rc::new(Value::from("foobar")))])
        .result_document(rd)
        .build()
        .dispatch(&mut StaticContext::<F>::new(), &parse(".")?)?;
    assert_eq!(s.len(), 1);
    assert_eq!(s[0].to_string(), "foobar");
    Ok(())
}

pub fn generic_parens_singleton<N: Node, G, H>(
    make_empty_doc: G,
    _: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let rd = make_empty_doc();
    let s = ContextBuilder::new()
        .context(vec![Item::Value(Rc::new(Value::from("foobar")))])
        .result_document(rd)
        .build()
        .dispatch(&mut StaticContext::<F>::new(), &parse("(1)")?)?;
    assert_eq!(s.len(), 1);
    assert_eq!(s[0].to_int().unwrap(), 1);
    Ok(())
}

pub fn generic_int<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
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
            _ => Err(Error::new(ErrorKind::Unknown, "not a value"))
        }
    } else { Err(Error::new(ErrorKind::Unknown, format!("got {} results, expected 1", result.len()))) }
}

pub fn generic_decimal<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
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
            _ => Err(Error::new(ErrorKind::Unknown, "not a value"))
        }
    } else { Err(Error::new(ErrorKind::Unknown, format!("got {} results, expected 1", result.len()))) }
}

pub fn generic_exponent<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
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
            _ => Err(Error::new(ErrorKind::Unknown, "not a value"))
        }
    } else { Err(Error::new(ErrorKind::Unknown, format!("got {} results, expected 1", result.len()))) }
}

pub fn generic_string_apos<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
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
            _ => Err(Error::new(ErrorKind::Unknown, "not a value"))
        }
    } else { Err(Error::new(ErrorKind::Unknown, format!("got {} results, expected 1", result.len()))) }
}

pub fn generic_string_apos_esc<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
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
            _ => Err(Error::new(ErrorKind::Unknown, "not a value"))
        }
    } else { Err(Error::new(ErrorKind::Unknown, format!("got {} results, expected 1", result.len()))) }
}

pub fn generic_string_quot<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
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
            _ => Err(Error::new(ErrorKind::Unknown, "not a value"))
        }
    } else { Err(Error::new(ErrorKind::Unknown, format!("got {} results, expected 1", result.len()))) }
}

pub fn generic_string_quot_esc<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
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
            _ => Err(Error::new(ErrorKind::Unknown, "not a value"))
        }
    } else { Err(Error::new(ErrorKind::Unknown, format!("got {} results, expected 1", result.len()))) }
}

pub fn generic_literal_sequence<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
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
            _ => Err(Error::new(ErrorKind::Unknown, "not a value"))
        }
    } else { Err(Error::new(ErrorKind::Unknown, format!("got {} results, expected 1", result.len()))) }
}

pub fn generic_literal_sequence_ws<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
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

pub fn generic_xpath_comment<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("1(::),(: a comment :)'abc', (: outer (: inner :) outer :) 2")?;
    assert_eq!(s.len(), 3);
    assert_eq!(s[0].to_int().unwrap(), 1);
    assert_eq!(s[1].to_string(), "abc");
    assert_eq!(s[2].to_int().unwrap(), 2);
    Ok(())
}

pub fn generic_fncall_string<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("string(('a', 'b', 'c'))")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_string(), "abc");
    Ok(())
}

pub fn generic_fncall_current_1<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("current()")?;
    assert_eq!(s.len(), 0);
    assert_eq!(s.to_string(), "");
    Ok(())
}

pub fn generic_fncall_current_2<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let s: Sequence<N> = dispatch_rig("current()/child::a", make_empty_doc, make_doc)?;
    assert_eq!(s.len(), 1);
    Ok(())
}

pub fn generic_fncall_current_3<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let rd = make_empty_doc();
    let sd = make_doc();
    if let Item::Node(ref doc) = sd {
        let top = doc.child_iter().nth(0).unwrap();
        let s = ContextBuilder::new()
            .result_document(rd)
            .context(vec![Item::Node(top)])
            .previous_context(Some(sd))
            .build()
            .dispatch(&mut StaticContext::<F>::new(), &parse("current()/child::a")?)
            .expect("evaluation failed");
        assert_eq!(s.len(), 1)
    } else {
        panic!("not a node")
    }
    Ok(())
}

pub fn generic_fncall_concat<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("concat('a', 'b', 'c')")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_string(), "abc");
    Ok(())
}

pub fn generic_fncall_startswith_pos<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("starts-with('abc', 'a')")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_bool(), true);
    Ok(())
}
pub fn generic_fncall_startswith_neg<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("starts-with('abc', 'b')")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_bool(), false);
    Ok(())
}

pub fn generic_fncall_contains_pos<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("contains('abc', 'b')")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_bool(), true);
    Ok(())
}
pub fn generic_fncall_contains_neg<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("contains('abc', 'd')")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_bool(), false);
    Ok(())
}

pub fn generic_fncall_substring_2arg<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("substring('abcdefg', 4)")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_string(), "defg");
    Ok(())
}
pub fn generic_fncall_substring_3arg<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("substring('abcdefg', 4, 2)")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_string(), "de");
    Ok(())
}

pub fn generic_fncall_substringbefore_pos<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("substring-before('abc', 'b')")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_string(), "a");
    Ok(())
}
pub fn generic_fncall_substringbefore_neg<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("substring-before('abc', 'd')")?;
    assert_eq!(s.to_string(), "");
    Ok(())
}

pub fn generic_fncall_substringafter_pos_1<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("substring-after('abc', 'b')")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_string(), "c");
    Ok(())
}
pub fn generic_fncall_substringafter_pos_2<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("substring-after('abc', 'c')")?;
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_string(), "");
    Ok(())
}
pub fn generic_fncall_substringafter_neg<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("substring-after('abc', 'd')")?;
    assert_eq!(s.to_string(), "");
    Ok(())
}

pub fn generic_fncall_normalizespace<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("normalize-space('	a  b\nc 	')")?;
    assert_eq!(s.to_string(), "a b c");
    Ok(())
}

pub fn generic_fncall_translate<N: Node, G, H>(
    _: G,
    _: H,
) -> Result<(), Error>
    where
        G: Fn() -> N,
        H: Fn() -> Item<N>,
{
    let s: Sequence<N> = no_src_no_result("translate('abcdeabcde', 'ade', 'XY')")?;
    assert_eq!(s.to_string(), "XbcYXbcY");
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
