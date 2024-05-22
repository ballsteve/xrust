//! Tests for XPath defined generically

use xrust::xdmerror::{Error, ErrorKind};
use xrust::item::{Item, Node, Sequence, SequenceTrait};
use xrust::parser::xpath::parse;
use xrust::transform::context::{Context, StaticContext};

type F = Box<dyn FnMut(&str) -> Result<(), Error>>;

fn no_src_no_result<N: Node>(e: impl AsRef<str>) -> Result<Sequence<N>, Error> {
    Context::new().dispatch(&mut StaticContext::<F>::new(), &parse(e.as_ref())?)
}

pub fn generic_empty<N: Node>() -> Result<(), Error>
{
    let result: Sequence<N> = no_src_no_result("")?;
    if result.len() == 0 {
        Ok(())
    } else { Err(Error::new(ErrorKind::Unknown, format!("got result \"{}\", expected \"\"", result.to_string()))) }
}
