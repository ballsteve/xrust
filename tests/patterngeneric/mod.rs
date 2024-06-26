//! Tests for pattern module defined generically

use std::rc::Rc;
use url::Url;
use xrust::ErrorKind;

use xrust::item::{Item, Node};
use xrust::pattern::Pattern;
use xrust::qname::QualifiedName;
use xrust::transform::context::{Context, StaticContext, StaticContextBuilder};
use xrust::value::Value;
use xrust::xdmerror::Error;

pub fn pattern_empty<N: Node>() -> Result<(), Error> {
    let _: Pattern<N> = Pattern::try_from("").expect("unable to parse empty string");
    Ok(())
}

pub fn pattern_predicate_1_pos<N: Node, G>(make_empty_doc: G) -> Result<(), Error>
where
    G: Fn() -> N,
{
    let p: Pattern<N> = Pattern::try_from(".[self::a]").expect("unable to parse \".[self::a]\"");

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QualifiedName::new(None, None, String::from("Test")))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut a = sd
        .new_element(QualifiedName::new(None, None, String::from("a")))
        .expect("unable to create element");
    t.push(a.clone()).expect("unable to append child");
    let t_a = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    a.push(t_a).expect("unable to append text node");
    let mut b = sd
        .new_element(QualifiedName::new(None, None, String::from("b")))
        .expect("unable to create element");
    t.push(b.clone()).expect("unable to append child");
    let t_b = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    b.push(t_b).expect("unable to append text node");

    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();

    assert_eq!(
        p.matches(&Context::new(), &mut stctxt, &Rc::new(Item::Node(a))),
        true
    );
    Ok(())
}
pub fn pattern_predicate_1_neg<N: Node, G>(make_empty_doc: G) -> Result<(), Error>
where
    G: Fn() -> N,
{
    let p: Pattern<N> = Pattern::try_from(".[self::a]").expect("unable to parse \".[self::a]\"");

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QualifiedName::new(None, None, String::from("Test")))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut a = sd
        .new_element(QualifiedName::new(None, None, String::from("a")))
        .expect("unable to create element");
    t.push(a.clone()).expect("unable to append child");
    let t_a = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    a.push(t_a).expect("unable to append text node");
    let mut b = sd
        .new_element(QualifiedName::new(None, None, String::from("b")))
        .expect("unable to create element");
    t.push(b.clone()).expect("unable to append child");
    let t_b = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    b.push(t_b).expect("unable to append text node");

    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();

    assert_eq!(
        p.matches(&Context::new(), &mut stctxt, &Rc::new(Item::Node(b))),
        false
    );
    Ok(())
}

pub fn pattern_sel_root_pos<N: Node, G>(make_empty_doc: G) -> Result<(), Error>
where
    G: Fn() -> N,
{
    let p: Pattern<N> = Pattern::try_from("/").expect("unable to parse \"/\"");

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QualifiedName::new(None, None, String::from("Test")))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut a = sd
        .new_element(QualifiedName::new(None, None, String::from("a")))
        .expect("unable to create element");
    t.push(a.clone()).expect("unable to append child");
    let t_a = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    a.push(t_a).expect("unable to append text node");
    let mut b = sd
        .new_element(QualifiedName::new(None, None, String::from("b")))
        .expect("unable to create element");
    t.push(b.clone()).expect("unable to append child");
    let t_b = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    b.push(t_b).expect("unable to append text node");

    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();

    assert_eq!(
        p.matches(&Context::new(), &mut stctxt, &Rc::new(Item::Node(sd))),
        true
    );
    Ok(())
}

pub fn pattern_sel_root_neg<N: Node, G>(make_empty_doc: G) -> Result<(), Error>
where
    G: Fn() -> N,
{
    let p: Pattern<N> = Pattern::try_from("/").expect("unable to parse \"/\"");

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QualifiedName::new(None, None, String::from("Test")))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut a = sd
        .new_element(QualifiedName::new(None, None, String::from("a")))
        .expect("unable to create element");
    t.push(a.clone()).expect("unable to append child");
    let t_a = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    a.push(t_a).expect("unable to append text node");
    let mut b = sd
        .new_element(QualifiedName::new(None, None, String::from("b")))
        .expect("unable to create element");
    t.push(b.clone()).expect("unable to append child");
    let t_b = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    b.push(t_b).expect("unable to append text node");

    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();

    assert_eq!(
        p.matches(&Context::new(), &mut stctxt, &Rc::new(Item::Node(a))),
        false
    );
    Ok(())
}

pub fn pattern_sel_1_pos<N: Node, G>(make_empty_doc: G) -> Result<(), Error>
where
    G: Fn() -> N,
{
    let p: Pattern<N> = Pattern::try_from("child::a").expect("unable to parse \"child::a\"");

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QualifiedName::new(None, None, String::from("Test")))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut a = sd
        .new_element(QualifiedName::new(None, None, String::from("a")))
        .expect("unable to create element");
    t.push(a.clone()).expect("unable to append child");
    let t_a = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    a.push(t_a).expect("unable to append text node");
    let mut b = sd
        .new_element(QualifiedName::new(None, None, String::from("b")))
        .expect("unable to create element");
    t.push(b.clone()).expect("unable to append child");
    let t_b = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    b.push(t_b).expect("unable to append text node");

    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();

    assert_eq!(
        p.matches(&Context::new(), &mut stctxt, &Rc::new(Item::Node(a))),
        true
    );
    Ok(())
}

pub fn pattern_sel_1_neg<N: Node, G>(make_empty_doc: G) -> Result<(), Error>
where
    G: Fn() -> N,
{
    let p: Pattern<N> = Pattern::try_from("child::a").expect("unable to parse \"child::a\"");

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QualifiedName::new(None, None, String::from("Test")))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut a = sd
        .new_element(QualifiedName::new(None, None, String::from("a")))
        .expect("unable to create element");
    t.push(a.clone()).expect("unable to append child");
    let t_a = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    a.push(t_a).expect("unable to append text node");
    let mut b = sd
        .new_element(QualifiedName::new(None, None, String::from("b")))
        .expect("unable to create element");
    t.push(b.clone()).expect("unable to append child");
    let t_b = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    b.push(t_b).expect("unable to append text node");

    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();

    assert_eq!(
        p.matches(&Context::new(), &mut stctxt, &Rc::new(Item::Node(b))),
        false
    );
    Ok(())
}

pub fn pattern_sel_2_pos<N: Node, G>(make_empty_doc: G) -> Result<(), Error>
where
    G: Fn() -> N,
{
    let p: Pattern<N> = Pattern::try_from("child::Test/child::a")
        .expect("unable to parse \"child::Test/child::a\"");

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QualifiedName::new(None, None, String::from("Test")))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut a = sd
        .new_element(QualifiedName::new(None, None, String::from("a")))
        .expect("unable to create element");
    t.push(a.clone()).expect("unable to append child");
    let t_a = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    a.push(t_a).expect("unable to append text node");
    let mut b = sd
        .new_element(QualifiedName::new(None, None, String::from("b")))
        .expect("unable to create element");
    t.push(b.clone()).expect("unable to append child");
    let t_b = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    b.push(t_b).expect("unable to append text node");

    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();

    assert_eq!(
        p.matches(&Context::new(), &mut stctxt, &Rc::new(Item::Node(a))),
        true
    );
    Ok(())
}

pub fn pattern_sel_2_neg<N: Node, G>(make_empty_doc: G) -> Result<(), Error>
where
    G: Fn() -> N,
{
    let p: Pattern<N> = Pattern::try_from("child::Test/child::a")
        .expect("unable to parse \"child::Test/child::a\"");

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QualifiedName::new(None, None, String::from("NotATest")))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut a = sd
        .new_element(QualifiedName::new(None, None, String::from("a")))
        .expect("unable to create element");
    t.push(a.clone()).expect("unable to append child");
    let t_a = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    a.push(t_a).expect("unable to append text node");
    let mut b = sd
        .new_element(QualifiedName::new(None, None, String::from("b")))
        .expect("unable to create element");
    t.push(b.clone()).expect("unable to append child");
    let t_b = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    b.push(t_b).expect("unable to append text node");

    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();

    assert_eq!(
        p.matches(&Context::new(), &mut stctxt, &Rc::new(Item::Node(a))),
        false
    );
    Ok(())
}

pub fn pattern_sel_text_kind_1_pos<N: Node, G>(make_empty_doc: G) -> Result<(), Error>
where
    G: Fn() -> N,
{
    let p: Pattern<N> =
        Pattern::try_from("child::text()").expect("unable to parse \"child::text()\"");

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QualifiedName::new(None, None, String::from("Test")))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut a = sd
        .new_element(QualifiedName::new(None, None, String::from("a")))
        .expect("unable to create element");
    t.push(a.clone()).expect("unable to append child");
    let t_a = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    a.push(t_a.clone()).expect("unable to append text node");
    let mut b = sd
        .new_element(QualifiedName::new(None, None, String::from("b")))
        .expect("unable to create element");
    t.push(b.clone()).expect("unable to append child");
    let t_b = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    b.push(t_b).expect("unable to append text node");

    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();

    assert_eq!(
        p.matches(&Context::new(), &mut stctxt, &Rc::new(Item::Node(t_a))),
        true
    );
    Ok(())
}
