//! Tests for XSLT defined generically

use std::collections::HashMap;
use crate::{ErrorKind, SequenceTrait};
use crate::item::{Item, Node, Sequence};
use crate::xdmerror::Error;
use crate::transform::context::StaticContext;
use crate::xslt::from_document;

type F = Box<dyn FnMut(&str) -> Result<(), Error>>;

fn test_rig<N: Node, G, H, J>(
    src: impl AsRef<str>,
    style: impl AsRef<str>,
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<Sequence<N>, Error>
    where
        G: Fn(&str) -> Result<N, Error>,
        H: Fn() -> Result<N, Error>,
        J: Fn(&str) -> Result<(N, Vec<HashMap<String, String>>), Error>,
{
    let srcdoc = parse_from_str(src.as_ref())?;
    let (styledoc, stylens) = parse_from_str_with_ns(style.as_ref())?;
    let mut stctxt = StaticContext::<F>::new();
    let mut ctxt = from_document(
        styledoc,
        stylens,
        None,
        |s| parse_from_str(s),
        |_| Ok(String::new()),
    )?;
    ctxt.context(vec![Item::Node(srcdoc.clone())], 0);
    ctxt.result_document(make_doc()?);
    ctxt.evaluate(&mut stctxt)
}

pub fn generic_callable_posn_2<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
    where
        G: Fn(&str) -> Result<N, Error>,
        H: Fn() -> Result<N, Error>,
        J: Fn(&str) -> Result<(N, Vec<HashMap<String, String>>), Error>,
{
    let result = test_rig(
        "<Test><one>blue</one><two>yellow</two><three>green</three><four>blue</four></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform' xmlns:eg='http://example.org/'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'>
    <xsl:sequence select='eg:my_func(count(child::*))'/>
  </xsl:template>
  <xsl:function name='eg:my_func'>
    <xsl:param name='my_param'/>
    <xsl:text>There are </xsl:text>
    <xsl:sequence select='$my_param'/>
    <xsl:text> child elements</xsl:text>
  </xsl:function>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    if result.to_string() == "There are 4 child elements" {
        Ok(())
    } else { Err(Error::new(ErrorKind::Unknown, format!("got result \"{}\", expected \"There are 4 child elements\"", result.to_string()))) }
}
