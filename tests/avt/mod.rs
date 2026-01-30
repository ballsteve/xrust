//! Tests for avt module defined generically

use xrust::{ErrorKind, Sequence, SequenceTrait, Transform};

use std::rc::Rc;
use xrust::item::{Item, Node};
use xrust::parser::avt::parse;
use xrust::transform::context::{Context, StaticContextBuilder};
use xrust::value::Value;
use xrust::xdmerror::Error;

fn test_rig<N: Node>(x: &Transform<N>) -> Result<Sequence<N>, Error> {
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let mut ctxt = Context::new();
    ctxt.var_push(
        String::from("y"),
        vec![Item::Value(Rc::new(Value::from("Y")))],
    );
    ctxt.dispatch(&mut stctxt, x)
}

pub fn avt_empty<N: Node>() -> Result<(), Error> {
    let x: Transform<N> = parse("", None).expect("unable to parse empty string");
    assert_eq!(test_rig(&x).expect("transformation failed").to_string(), "");
    Ok(())
}

pub fn content<N: Node, G>(_: G) -> Result<(), Error>
where
    G: Fn() -> N,
{
    let x: Transform<N> = parse("plain content", None).expect("unable to parse avt");
    assert_eq!(
        test_rig(&x).expect("transformation failed").to_string(),
        "plain content"
    );
    Ok(())
}
pub fn var_ref<N: Node, G>(_: G) -> Result<(), Error>
where
    G: Fn() -> N,
{
    let x: Transform<N> = parse("before {$y} after", None).expect("unable to parse avt");
    assert_eq!(
        test_rig(&x).expect("transformation failed").to_string(),
        "before Y after"
    );
    Ok(())
}
