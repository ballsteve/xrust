//! Tests for XSLT defined generically

use pkg_version::{pkg_version_major, pkg_version_minor, pkg_version_patch};
use qualname::NamespaceMap;
use url::Url;
use xrust::item::{Item, Node, Sequence, SequenceTrait};
use xrust::transform::context::StaticContextBuilder;
use xrust::xdmerror::{Error, ErrorKind};
use xrust::xslt::from_document;

fn test_rig<N: Node, G, H, J>(
    src: impl AsRef<str>,
    style: impl AsRef<str>,
    parse_from_str: G,
    _parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(Sequence<N>, N), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let srcdoc = parse_from_str(src.as_ref()).map_err(|e| {
        Error::new(
            e.kind,
            format!("error parsing source document: {}", e.message),
        )
    })?;
    let styledoc = parse_from_str(style.as_ref())
        .map_err(|e| Error::new(e.kind, format!("error parsing stylesheet: {}", e.message)))?;
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let mut ctxt = from_document(styledoc, None, |s| parse_from_str(s), |_| Ok(String::new()))?;
    ctxt.context(vec![Item::Node(srcdoc.clone())], 0);
    // Make sure the document lives until the end of the function's scope
    let rd = make_doc()?;
    ctxt.result_document(rd.clone());
    ctxt.populate_key_values(&mut stctxt, srcdoc.clone())?;
    let result = ctxt.evaluate(&mut stctxt);
    result.map(|r| (r, rd))
}

fn test_msg_rig<N: Node, G, H, J>(
    src: impl AsRef<str>,
    style: impl AsRef<str>,
    parse_from_str: G,
    _parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(Sequence<N>, Vec<String>, N), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let srcdoc = parse_from_str(src.as_ref())?;
    let styledoc = parse_from_str(style.as_ref())?;
    let mut msgs: Vec<String> = vec![];
    let mut stctxt = StaticContextBuilder::new()
        .message(|m| {
            msgs.push(String::from(m));
            Ok(())
        })
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let mut ctxt = from_document(styledoc, None, |s| parse_from_str(s), |_| Ok(String::new()))?;
    ctxt.context(vec![Item::Node(srcdoc.clone())], 0);
    let rd = make_doc()?;
    ctxt.result_document(rd.clone());
    let seq = ctxt.evaluate(&mut stctxt)?;
    Ok((seq, msgs, rd))
}

pub fn generic_literal_text<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Test><Level1>one</Level1><Level1>two</Level1></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'>Found the document</xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    if result.0.to_string() == "Found the document" {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!(
                "got result \"{}\", expected \"Found the document\"",
                result.0.to_string()
            ),
        ))
    }
}

pub fn generic_sys_prop<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Test><Level1>one</Level1><Level1>two</Level1></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:sequence select='system-property("xsl:version")'/>-<xsl:sequence select='system-property("xsl:product-version")'/></xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    if result.0.to_string()
        == format!(
            "0.9-{}.{}.{}",
            pkg_version_major!(),
            pkg_version_minor!(),
            pkg_version_patch!()
        )
    {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!(
                "got result \"{}\", expected \"{}\"",
                result.0.to_string(),
                format!(
                    "0.9-{}.{}.{}",
                    pkg_version_major!(),
                    pkg_version_minor!(),
                    pkg_version_patch!()
                )
            ),
        ))
    }
}

pub fn generic_value_of_1<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Test>special &lt; less than</Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::*'><xsl:value-of select='.'/></xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    if result.0.to_string() == "special < less than" {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!(
                "got result \"{}\", expected \"special &lt; less than\"",
                result.0.to_string()
            ),
        ))
    }
}

pub fn generic_value_of_2<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Test>special &lt; less than</Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::*'><xsl:value-of select='.' disable-output-escaping='yes'/></xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    if result.0.to_string() == "special < less than" {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!(
                "got result \"{}\", expected \"special < less than\"",
                result.0.to_string()
            ),
        ))
    }
}

pub fn generic_literal_element<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Test><Level1>one</Level1><Level1>two</Level1></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><answer>Made an element</answer></xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    if result.0.to_xml() == "<answer>Made an element</answer>" {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!(
                "got result \"{}\", expected \"<answer>Made an element</answer>\"",
                result.0.to_string()
            ),
        ))
    }
}

pub fn generic_element<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Test><Level1>one</Level1><Level1>two</Level1></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:element name='answer{count(ancestor::*)}'>Made an element</xsl:element></xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    if result.0.to_xml() == "<answer0>Made an element</answer0>" {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!(
                "got result \"{}\", expected \"<answer0>Made an element</answer0>\"",
                result.0.to_string()
            ),
        ))
    }
}

pub fn generic_apply_templates_1<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Test><Level1>one</Level1><Level1>two</Level1></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::*'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::text()'>found text</xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    if result.0.to_xml() == "found textfound text" {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!(
                "got result \"{}\", expected \"found textfound text\"",
                result.0.to_string()
            ),
        ))
    }
}

pub fn generic_apply_templates_2<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Test>one<Level1/>two<Level1/>three<Level1/>four<Level1/></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:apply-templates select='child::text()'/></xsl:template>
  <xsl:template match='child::Level1'>found Level1 element</xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    if result.0.to_xml() == "onetwothreefour" {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!(
                "got result \"{}\", expected \"onetwothreefour\"",
                result.0.to_string()
            ),
        ))
    }
}

pub fn generic_apply_templates_mode_bad_qname<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    // Specify a QName for a mode that has not been declared as an XML Namespace
    let result = test_rig(
        "<Test>one<Level1>a</Level1>two<Level1>b</Level1>three<Level1>c</Level1>four<Level1>d</Level1></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><HEAD><xsl:apply-templates select='child::Level1' mode='test:head'/></HEAD><BODY><xsl:apply-templates select='child::Level1' mode='body'/></BODY></xsl:template>
  <xsl:template match='child::Level1' mode='test:head'><h1><xsl:apply-templates/></h1></xsl:template>
  <xsl:template match='child::Level1' mode='test:body'><p><xsl:apply-templates/></p></xsl:template>
  <xsl:template match='child::Level1'>should not see this</xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    );
    if result.is_err() {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            "stylesheet succeeded, expected error",
        ))
    }
}

pub fn generic_apply_templates_mode<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Test>one<Level1>a</Level1>two<Level1>b</Level1>three<Level1>c</Level1>four<Level1>d</Level1></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><HEAD><xsl:apply-templates select='child::Level1' mode='head'/></HEAD><BODY><xsl:apply-templates select='child::Level1' mode='body'/></BODY></xsl:template>
  <xsl:template match='child::Level1' mode='head'><h1><xsl:apply-templates/></h1></xsl:template>
  <xsl:template match='child::Level1' mode='body'><p><xsl:apply-templates/></p></xsl:template>
  <xsl:template match='child::Level1'>should not see this</xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    if result.0.to_xml()
        == "<HEAD><h1>a</h1><h1>b</h1><h1>c</h1><h1>d</h1></HEAD><BODY><p>a</p><p>b</p><p>c</p><p>d</p></BODY>"
    {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!(
                "got result \"{}\", expected \"<HEAD><h1>a</h1><h1>b</h1><h1>c</h1><h1>d</h1></HEAD><BODY><p>a</p><p>b</p><p>c</p><p>d</p></BODY>\"",
                result.0.to_xml()
            ),
        ))
    }
}

pub fn generic_apply_templates_sort<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Test>one<Level1>a</Level1>two<Level1>b</Level1>three<Level1>c</Level1>four<Level1>d</Level1></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:apply-templates><xsl:sort select='.'/></xsl:apply-templates></xsl:template>
  <xsl:template match='child::Level1'><L><xsl:apply-templates/></L></xsl:template>
  <xsl:template match='child::Test/child::text()'><p><xsl:sequence select='.'/></p></xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    if result.0.to_xml()
        == "<L>a</L><L>b</L><L>c</L><L>d</L><p>four</p><p>one</p><p>three</p><p>two</p>"
    {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!(
                "got result \"{}\", expected \"<L>a</L><L>b</L><L>c</L><L>d</L><p>four</p><p>one</p><p>three</p><p>two</p>\"",
                result.0.to_xml()
            ),
        ))
    }
}

pub fn generic_comment<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Test>one<Level1/>two<Level1/>three<Level1/>four<Level1/></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Level1'><xsl:comment> this is a level 1 element </xsl:comment></xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    if result.0.to_xml()
        == "one<!-- this is a level 1 element -->two<!-- this is a level 1 element -->three<!-- this is a level 1 element -->four<!-- this is a level 1 element -->"
    {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!(
                "got result \"{}\", expected \"one<!-- this is a level 1 element -->two<!-- this is a level 1 element -->three<!-- this is a level 1 element -->four<!-- this is a level 1 element -->\"",
                result.0.to_string()
            ),
        ))
    }
}

pub fn generic_pi<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Test>one<Level1/>two<Level1/>three<Level1/>four<Level1/></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Level1'><xsl:processing-instruction name='piL1'>this is a level 1 element</xsl:processing-instruction></xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    if result.0.to_xml()
        == "one<?piL1 this is a level 1 element?>two<?piL1 this is a level 1 element?>three<?piL1 this is a level 1 element?>four<?piL1 this is a level 1 element?>"
    {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!(
                "got result \"{}\", expected \"one<?piL1 this is a level 1 element?>two<?piL1 this is a level 1 element?>three<?piL1 this is a level 1 element?>four<?piL1 this is a level 1 element?>\"",
                result.0.to_string()
            ),
        ))
    }
}

pub fn generic_current<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Test ref='one'><second name='foo'>I am foo</second><second name='one'>I am one</second></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'>
    <xsl:sequence select='child::*/child::second[attribute::name eq current()/attribute::ref]'/>
  </xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;

    if result.0.to_xml() == "<second name='one'>I am one</second>" {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!(
                "got result \"{}\", expected \"<second name='one'>I am one</second>\"",
                result.0.to_string()
            ),
        ))
    }
}

pub fn generic_key_1<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Test><one>blue</one><two>yellow</two><three>green</three><four>blue</four></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:key name='mykey' match='child::*' use='child::text()'/>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'>#blue = <xsl:sequence select='count(key("mykey", "blue"))'/></xsl:template>
  <xsl:template match='child::Test/child::*'>shouldn't see this</xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    if result.0.to_xml() == "#blue = 2" {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!(
                "got result \"{}\", expected \"#blue = 2\"",
                result.0.to_string()
            ),
        ))
    }
}

// Although we have the source and stylesheet in files,
// they are inlined here to avoid dependency on I/O libraries
pub fn generic_issue_58<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        r#"<Example>
    <Title>XSLT in Rust</Title>
    <Paragraph>A simple document.</Paragraph>
</Example>
"#,
        r#"<xsl:stylesheet
	version="1.0"
	xmlns:dat="http://www.stormware.cz/schema/version_2/data.xsd"
	xmlns:int="http://www.stormware.cz/schema/version_2/intDoc.xsd"
	xmlns:xsl="http://www.w3.org/1999/XSL/Transform">

	<xsl:output method="xml" encoding="utf-8" indent="yes"/>

    <xsl:template match="child::Example">
        <dat:dataPack>
            <xsl:apply-templates/>
        </dat:dataPack>
    </xsl:template>
    <xsl:template match="child::Title">
        <int:head>
            <xsl:apply-templates/>
        </int:head>
    </xsl:template>
    <xsl:template match="child::Paragraph">
        <int:body>
            <xsl:apply-templates/>
        </int:body>
    </xsl:template>
</xsl:stylesheet>
"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    if result.0.to_xml()
        == r#"<dat:dataPack xmlns:dat='http://www.stormware.cz/schema/version_2/data.xsd'>
    <int:head xmlns:int='http://www.stormware.cz/schema/version_2/intDoc.xsd'>XSLT in Rust</int:head>
    <int:body xmlns:int='http://www.stormware.cz/schema/version_2/intDoc.xsd'>A simple document.</int:body>
</dat:dataPack>"# {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!("not expected result"),
        ))
    }
}

pub fn generic_issue_95<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Test><Level1>one</Level1><Level1>two</Level1></Test>",
        r#"<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
          <xsl:template match="@*|node()">
            <xsl:copy>
              <xsl:apply-templates select="@*|node()"/>
            </xsl:copy>
          </xsl:template>
        </xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    if result.0.to_xml() == "<Test><Level1>one</Level1><Level1>two</Level1></Test>" {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!(
                "got result \"{}\", expected \"Found the document\"",
                result.0.to_xml()
            ),
        ))
    }
}

pub fn generic_message_1<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let (result, msgs, _rd) = test_msg_rig(
        "<Test>one<Level1/>two<Level1/>three<Level1/>four<Level1/></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Level1'><xsl:message>here is a level 1 element</xsl:message><L/></xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    if result.to_xml() == "one<L/>two<L/>three<L/>four<L/>" {
        if msgs.len() == 4 {
            if msgs[0] == "here is a level 1 element" {
                Ok(())
            } else {
                Err(Error::new(
                    ErrorKind::Unknown,
                    format!(
                        "got message \"{}\", expected \"here is a level 1 element\"",
                        msgs[0]
                    ),
                ))
            }
        } else {
            Err(Error::new(
                ErrorKind::Unknown,
                format!("got {} messages, expected 4", msgs.len()),
            ))
        }
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!(
                "got result \"{}\", expected \"one<L></L>two<L></L>three<L></L>four<L></L>\"",
                result.to_string()
            ),
        ))
    }
}

pub fn generic_message_term<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    match test_msg_rig(
        "<Test>one<Level1/>two<Level1/>three<Level1/>four<Level1/></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Level1'><xsl:message terminate='yes'>here is a level 1 element</xsl:message><L/></xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    ) {
        Err(e) => {
            if e.kind == ErrorKind::Terminated
                && e.message == "here is a level 1 element"
                && e.code.is_some_and(|f| f.local_name() == *"XTMM9000")
            {
                Ok(())
            } else {
                Err(Error::new(ErrorKind::Unknown, "incorrect error"))
            }
        }
        Ok(_) => Err(Error::new(
            ErrorKind::Unknown,
            "evaluation succeeded when it should have failed",
        )),
    }
}
pub fn generic_callable_named_1<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Test><one>blue</one><two>yellow</two><three>green</three><four>blue</four></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'>
    <xsl:call-template name='my_template'>
      <xsl:with-param name='my_param' select='count(child::*)'/>
    </xsl:call-template>
  </xsl:template>
  <xsl:template name='my_template'>
    <xsl:param name='my_param'>default value</xsl:param>
    <xsl:text>There are </xsl:text>
    <xsl:sequence select='$my_param'/>
    <xsl:text> child elements</xsl:text>
  </xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    if result.0.to_string() == "There are 4 child elements" {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!(
                "got result \"{}\", expected \"There are 4 child elements\"",
                result.0.to_string()
            ),
        ))
    }
}
pub fn generic_callable_posn_1<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
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
    if result.0.to_string() == "There are 4 child elements" {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!(
                "got result \"{}\", expected \"There are 4 child elements\"",
                result.0.to_string()
            ),
        ))
    }
}

pub fn generic_include<N: Node, G, H, J>(
    parse_from_str: G,
    _parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let srcdoc =
        parse_from_str("<Test>one<Level1/>two<Level2/>three<Level3/>four<Level4/></Test>")?;
    let styledoc = parse_from_str(
        "<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:include href='included.xsl'/>
  <xsl:template match='child::Test'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Level1'>found Level1 element</xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>",
    )?;
    let pwd = std::env::current_dir().expect("unable to get current directory");
    let pwds = pwd
        .into_os_string()
        .into_string()
        .expect("unable to convert pwd");
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let mut ctxt = from_document(
        styledoc,
        Some(
            Url::parse(format!("file://{}/tests/xsl/including.xsl", pwds.as_str()).as_str())
                .expect("unable to parse URL"),
        ),
        |s| parse_from_str(s),
        |_| Ok(String::new()),
    )?;
    ctxt.context(vec![Item::Node(srcdoc.clone())], 0);
    ctxt.result_document(make_doc()?);
    let result = ctxt.evaluate(&mut stctxt)?;
    if result.to_string()
        == "onefound Level1 elementtwofound Level2 elementthreefound Level3 elementfour"
    {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!(
                "got result \"{}\", expected \"onefound Level1 elementtwofound Level2 elementthreefound Level3 elementfour\"",
                result.to_string()
            ),
        ))
    }
}

pub fn generic_document_1<N: Node, G, H, J>(
    parse_from_str: G,
    _parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let srcdoc = parse_from_str("<Test><internal>on the inside</internal></Test>")?;
    let styledoc = parse_from_str(
        r##"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::Test'><xsl:apply-templates/>|<xsl:apply-templates select='document("urn::test.org/test")'/></xsl:template>
  <xsl:template match='child::internal'>found internal element</xsl:template>
  <xsl:template match='child::external'>found external element</xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>"##,
    )?;
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_url| {
            Ok(String::from(
                "<Outside><external>from outside</external></Outside>",
            ))
        })
        .parser(|s| parse_from_str(s))
        .build();
    let mut ctxt = from_document(styledoc, None, |s| parse_from_str(s), |_| Ok(String::new()))?;
    ctxt.context(vec![Item::Node(srcdoc.clone())], 0);
    ctxt.result_document(make_doc()?);
    let result = ctxt.evaluate(&mut stctxt)?;
    if result.to_string() == "found internal element|found external element" {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            format!(
                "got result \"{}\", expected \"onefound Level1 elementtwofound Level2 elementthreefound Level3 elementfour\"",
                result.to_string()
            ),
        ))
    }
}

pub fn generic_number_1<N: Node, G, H, J>(
    parse_from_str: G,
    _parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let srcdoc = parse_from_str("<Test><t>one</t><t>two</t><t>three</t></Test>")?;
    let styledoc = parse_from_str(
        r##"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::Test'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::t'>t element <xsl:number/></xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>"##,
    )?;
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_url| Ok(String::new()))
        .parser(|s| parse_from_str(s))
        .build();
    let mut ctxt = from_document(styledoc, None, |s| parse_from_str(s), |_| Ok(String::new()))?;
    ctxt.context(vec![Item::Node(srcdoc.clone())], 0);
    ctxt.result_document(make_doc()?);
    let result = ctxt.evaluate(&mut stctxt)?;
    assert_eq!(result.to_string(), "t element 1t element 2t element 3");
    Ok(())
}

pub fn attr_set_1<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Test><Level1>one</Level1><Level1>two</Level1></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:attribute-set name='foo'>
    <xsl:attribute name='bar'>from set foo</xsl:attribute>
  </xsl:attribute-set>
  <xsl:template match='child::Level1'><xsl:copy xsl:use-attribute-sets='foo'/></xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    assert_eq!(
        result.0.to_xml(),
        "<Level1 bar='from set foo'/><Level1 bar='from set foo'/>"
    );
    Ok(())
}

pub fn attr_set_2<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Test><Level1>one</Level1><Level1>two</Level1></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:attribute-set name='foo'>
    <xsl:attribute name='bar'>from set foo</xsl:attribute>
  </xsl:attribute-set>
  <xsl:template match='child::Level1'><MyElement xsl:use-attribute-sets='foo'><xsl:apply-templates/></MyElement></xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    assert_eq!(
        result.0.to_xml(),
        "<MyElement bar='from set foo'>one</MyElement><MyElement bar='from set foo'>two</MyElement>"
    );
    Ok(())
}

pub fn attr_set_3<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Test><Level1>one</Level1><Level1>two</Level1></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:attribute-set name='foo'>
    <xsl:attribute name='bar'>from set foo</xsl:attribute>
  </xsl:attribute-set>
  <xsl:template match='child::Level1'><xsl:element name='Element' xsl:use-attribute-sets='foo'><xsl:apply-templates/></xsl:element></xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    assert_eq!(
        result.0.to_xml(),
        "<Element bar='from set foo'>one</Element><Element bar='from set foo'>two</Element>"
    );
    Ok(())
}

pub fn feg_starting_with_1<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let (result, _rd) = test_rig(
        "<Test><a>one</a><b>two</b><c>three</c><a>four</a><b>five</b><c>six</c><a>seven</a><b>eight</b><c>nine</c></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='Test'>
    <result>
      <xsl:for-each-group select='*' group-starting-with='a'>
        <group>
          <xsl:apply-templates select='current-group()'/>
        </group>
      </xsl:for-each-group>
    </result>
  </xsl:template>
  <xsl:template match='*'>
    <xsl:copy>
      <xsl:apply-templates/>
    </xsl:copy>
  </xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    assert_eq!(
        result.to_xml(),
        "<result><group><a>one</a><b>two</b><c>three</c></group><group><a>four</a><b>five</b><c>six</c></group><group><a>seven</a><b>eight</b><c>nine</c></group></result>"
    );
    Ok(())
}

pub fn feg_starting_with_2<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let (result, _rd) = test_rig(
        "<Test><a>one</a><b>two</b><c>three</c><a>four</a><b>five</b><c>six</c><a>seven</a><b>eight</b><c>nine</c></Test>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='Test'>
    <result>
      <xsl:for-each-group select='*' group-starting-with='a'>
        <group>
          <first>
            <xsl:apply-templates select='current-group()[position() eq 1]'/>
          </first>
          <rest>
            <xsl:apply-templates select='current-group()[position() ne 1]'/>
          </rest>
        </group>
      </xsl:for-each-group>
    </result>
  </xsl:template>
  <xsl:template match='*'>
    <xsl:copy>
      <xsl:apply-templates/>
    </xsl:copy>
  </xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    assert_eq!(
        result.to_xml(),
        "<result><group><first><a>one</a></first><rest><b>two</b><c>three</c></rest></group><group><first><a>four</a></first><rest><b>five</b><c>six</c></rest></group><group><first><a>seven</a></first><rest><b>eight</b><c>nine</c></rest></group></result>"
    );
    Ok(())
}

pub fn issue_96_abs<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Example><Level1>one</Level1><Level1>two</Level1></Example>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/Example'>found an Example element</xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    assert_eq!(result.0.to_xml(), "found an Example element");
    Ok(())
}

pub fn issue_96_rel<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Example><Level1>one</Level1><Level1>two</Level1></Example>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='Example'>found an Example element</xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    assert_eq!(result.0.to_xml(), "found an Example element");
    Ok(())
}

pub fn issue_96_mixed<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<Example><Level1>one</Level1><Level1>two</Level1></Example>",
        r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::Example/Level1'>found a Level1 element</xsl:template>
</xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    assert_eq!(
        result.0.to_xml(),
        "found a Level1 elementfound a Level1 element"
    );
    Ok(())
}

pub fn issue_126<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<catalog>
          <cd>
            <title>Empire Burlesque</title>
            <artist>Bob Dylan</artist>
            <country>USA</country>
            <company>Columbia</company>
            <price>10.90</price>
            <year>1985</year>
          </cd>
          <cd>
            <title>Hide your heart</title>
            <artist>Bonnie Tyler</artist>
            <country>UK</country>
            <company>CBS Records</company>
            <price>9.90</price>
            <year>1988</year>
          </cd>
          <cd>
            <title>Greatest Hits</title>
            <artist>Dolly Parton</artist>
            <country>USA</country>
            <company>RCA</company>
            <price>9.90</price>
            <year>1982</year>
          </cd>
          <cd>
            <title>Still got the blues</title>
            <artist>Gary Moore</artist>
            <country>UK</country>
            <company>Virgin records</company>
            <price>10.20</price>
            <year>1990</year>
          </cd>
          <cd>
            <title>Eros</title>
            <artist>Eros Ramazzotti</artist>
            <country>EU</country>
            <company>BMG</company>
            <price>9.90</price>
            <year>1997</year>
          </cd>
          <cd>
            <title>One night only</title>
            <artist>Bee Gees</artist>
            <country>UK</country>
            <company>Polydor</company>
            <price>10.90</price>
            <year>1998</year>
          </cd>
          <cd>
            <title>Sylvias Mother</title>
            <artist>Dr.Hook</artist>
            <country>UK</country>
            <company>CBS</company>
            <price>8.10</price>
            <year>1973</year>
          </cd>
          <cd>
            <title>Maggie May</title>
            <artist>Rod Stewart</artist>
            <country>UK</country>
            <company>Pickwick</company>
            <price>8.50</price>
            <year>1990</year>
          </cd>
          <cd>
            <title>Romanza</title>
            <artist>Andrea Bocelli</artist>
            <country>EU</country>
            <company>Polydor</company>
            <price>10.80</price>
            <year>1996</year>
          </cd>
          <cd>
            <title>When a man loves a woman</title>
            <artist>Percy Sledge</artist>
            <country>USA</country>
            <company>Atlantic</company>
            <price>8.70</price>
            <year>1987</year>
          </cd>
          <cd>
            <title>Black angel</title>
            <artist>Savage Rose</artist>
            <country>EU</country>
            <company>Mega</company>
            <price>10.90</price>
            <year>1995</year>
          </cd>
          <cd>
            <title>1999 Grammy Nominees</title>
            <artist>Many</artist>
            <country>USA</country>
            <company>Grammy</company>
            <price>10.20</price>
            <year>1999</year>
          </cd>
          <cd>
            <title>For the good times</title>
            <artist>Kenny Rogers</artist>
            <country>UK</country>
            <company>Mucik Master</company>
            <price>8.70</price>
            <year>1995</year>
          </cd>
          <cd>
            <title>Big Willie style</title>
            <artist>Will Smith</artist>
            <country>USA</country>
            <company>Columbia</company>
            <price>9.90</price>
            <year>1997</year>
          </cd>
          <cd>
            <title>Tupelo Honey</title>
            <artist>Van Morrison</artist>
            <country>UK</country>
            <company>Polydor</company>
            <price>8.20</price>
            <year>1971</year>
          </cd>
          <cd>
            <title>Soulsville</title>
            <artist>Jorn Hoel</artist>
            <country>Norway</country>
            <company>WEA</company>
            <price>7.90</price>
            <year>1996</year>
          </cd>
          <cd>
            <title>The very best of</title>
            <artist>Cat Stevens</artist>
            <country>UK</country>
            <company>Island</company>
            <price>8.90</price>
            <year>1990</year>
          </cd>
          <cd>
            <title>Stop</title>
            <artist>Sam Brown</artist>
            <country>UK</country>
            <company>A and M</company>
            <price>8.90</price>
            <year>1988</year>
          </cd>
          <cd>
            <title>Bridge of Spies</title>
            <artist>T`Pau</artist>
            <country>UK</country>
            <company>Siren</company>
            <price>7.90</price>
            <year>1987</year>
          </cd>
          <cd>
            <title>Private Dancer</title>
            <artist>Tina Turner</artist>
            <country>UK</country>
            <company>Capitol</company>
            <price>8.90</price>
            <year>1983</year>
          </cd>
          <cd>
            <title>Midt om natten</title>
            <artist>Kim Larsen</artist>
            <country>EU</country>
            <company>Medley</company>
            <price>7.80</price>
            <year>1983</year>
          </cd>
          <cd>
            <title>Pavarotti Gala Concert</title>
            <artist>Luciano Pavarotti</artist>
            <country>UK</country>
            <company>DECCA</company>
            <price>9.90</price>
            <year>1991</year>
          </cd>
          <cd>
            <title>The dock of the bay</title>
            <artist>Otis Redding</artist>
            <country>USA</country>
            <COMPANY>Stax Records</COMPANY>
            <PRICE>7.90</PRICE>
            <YEAR>1968</YEAR>
          </cd>
          <cd>
            <title>Picture book</title>
            <artist>Simply Red</artist>
            <country>EU</country>
            <company>Elektra</company>
            <price>7.20</price>
            <year>1985</year>
          </cd>
          <cd>
            <title>Red</title>
            <artist>The Communards</artist>
            <country>UK</country>
            <company>London</company>
            <price>7.80</price>
            <year>1987</year>
          </cd>
          <cd>
            <title>Unchain my heart</title>
            <artist>Joe Cocker</artist>
            <country>USA</country>
            <company>EMI</company>
            <price>8.20</price>
            <year>1987</year>
          </cd>
        </catalog>",
        r###"<xsl:stylesheet version="1.0"
        xmlns:xsl="http://www.w3.org/1999/XSL/Transform">

        <xsl:template match="/">
          <html>
          <body>
          <h2>My CD Collection</h2>
          <table border="1">
            <ts bgcolor="#9acd32">
              <th>Title</th>
              <th>Artist</th>
            </ts>
            <xsl:for-each select="catalog/cd">
            <tr>
              <td><xsl:value-of select="title"/></td>
              <td><xsl:value-of select="artist"/></td>
            </tr>
            </xsl:for-each>
          </table>
          </body>
          </html>
        </xsl:template>

        </xsl:stylesheet>"###,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;
    assert_eq!(
        result.0.to_xml(),
        r###"<html><body><h2>My CD Collection</h2><table border='1'><ts bgcolor='#9acd32'><th>Title</th><th>Artist</th></ts><tr><td>Empire Burlesque</td><td>Bob Dylan</td></tr><tr><td>Hide your heart</td><td>Bonnie Tyler</td></tr><tr><td>Greatest Hits</td><td>Dolly Parton</td></tr><tr><td>Still got the blues</td><td>Gary Moore</td></tr><tr><td>Eros</td><td>Eros Ramazzotti</td></tr><tr><td>One night only</td><td>Bee Gees</td></tr><tr><td>Sylvias Mother</td><td>Dr.Hook</td></tr><tr><td>Maggie May</td><td>Rod Stewart</td></tr><tr><td>Romanza</td><td>Andrea Bocelli</td></tr><tr><td>When a man loves a woman</td><td>Percy Sledge</td></tr><tr><td>Black angel</td><td>Savage Rose</td></tr><tr><td>1999 Grammy Nominees</td><td>Many</td></tr><tr><td>For the good times</td><td>Kenny Rogers</td></tr><tr><td>Big Willie style</td><td>Will Smith</td></tr><tr><td>Tupelo Honey</td><td>Van Morrison</td></tr><tr><td>Soulsville</td><td>Jorn Hoel</td></tr><tr><td>The very best of</td><td>Cat Stevens</td></tr><tr><td>Stop</td><td>Sam Brown</td></tr><tr><td>Bridge of Spies</td><td>T`Pau</td></tr><tr><td>Private Dancer</td><td>Tina Turner</td></tr><tr><td>Midt om natten</td><td>Kim Larsen</td></tr><tr><td>Pavarotti Gala Concert</td><td>Luciano Pavarotti</td></tr><tr><td>The dock of the bay</td><td>Otis Redding</td></tr><tr><td>Picture book</td><td>Simply Red</td></tr><tr><td>Red</td><td>The Communards</td></tr><tr><td>Unchain my heart</td><td>Joe Cocker</td></tr></table></body></html>"###
    );
    Ok(())
}

pub fn issue_137_1<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<dummy/>",
        r#"<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
        <xsl:variable name="title">A really exciting document</xsl:variable>
        <xsl:variable name="backcolor" select="'#FFFFCC'"/>
        <xsl:template match="/*">
            <HTML><TITLE><xsl:value-of select="$title"/></TITLE>
            <BODY BGCOLOR='{$backcolor}'>
                <!-- ... -->
            </BODY></HTML>
        </xsl:template>
        </xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;

    assert_eq!(
        result.0.to_xml(),
        "<HTML><TITLE>A really exciting document</TITLE><BODY BGCOLOR='#FFFFCC'/></HTML>"
    );
    Ok(())
}

pub fn issue_137_2<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<dummy/>",
        r#"<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
        <xsl:variable name="title">A really exciting document</xsl:variable>
        <xsl:variable name="body-title" select="concat('Title: ', $title)"/>
        <xsl:variable name="backcolor" select="'#FFFFCC'"/>
        <xsl:template match="/*">
            <HTML><TITLE><xsl:value-of select="$title"/></TITLE>
            <BODY BGCOLOR='{$backcolor}'>
                <H1><xsl:value-of select="$body-title"/></H1>
            </BODY></HTML>
        </xsl:template>
        </xsl:stylesheet>"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;

    assert_eq!(
        result.0.to_xml(),
        "<HTML><TITLE>A really exciting document</TITLE><BODY BGCOLOR='#FFFFCC'><H1>Title: A really exciting document</H1></BODY></HTML>"
    );
    Ok(())
}

pub fn dbk_1<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let (result, _rd) = test_rig(
        "<db:article xmlns:db='http://docbook.org/ns/docbook'>
        <db:sect1 xmlns:db='http://docbook.org/ns/docbook'>
          <db:title xmlns:db='http://docbook.org/ns/docbook'>Level 1 Heading</db:title>
          <db:para xmlns:db='http://docbook.org/ns/docbook'>First paragraph</db:para>
          <db:sect2 xmlns:db='http://docbook.org/ns/docbook'>
            <db:title xmlns:db='http://docbook.org/ns/docbook'>Level 2 Heading</db:title>
            <db:para xmlns:db='http://docbook.org/ns/docbook'>Second paragraph</db:para>
          </db:sect2>
          <db:sect2 xmlns:db='http://docbook.org/ns/docbook'>
            <db:title xmlns:db='http://docbook.org/ns/docbook'>Second Level 2 Heading</db:title>
            <db:para xmlns:db='http://docbook.org/ns/docbook'>Third paragraph</db:para>
          </db:sect2>
        </db:sect1>
        <db:sect1 xmlns:db='http://docbook.org/ns/docbook'>
          <db:title xmlns:db='http://docbook.org/ns/docbook'>Second Level 1 Heading</db:title>
          <db:para xmlns:db='http://docbook.org/ns/docbook'>Fourth paragraph</db:para>
          <db:sect2 xmlns:db='http://docbook.org/ns/docbook'>
            <db:title xmlns:db='http://docbook.org/ns/docbook'>Third Level 2 Heading</db:title>
            <db:para xmlns:db='http://docbook.org/ns/docbook'>Fifth paragraph</db:para>
          </db:sect2>
        </db:sect1></db:article>
",
        r#"<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
          xmlns:db="http://docbook.org/ns/docbook"
          version="3.0">

          <xsl:output indent="yes"/>
          <xsl:strip-space elements="*"/>

          <xsl:template match="db:article">
            <HTML>
            <BODY>
            <xsl:apply-templates/>
            </BODY>
            </HTML>
          </xsl:template>
          <xsl:template match="*">
            <DIV>matched element <I><xsl:sequence select="name()"/></I></DIV>
          </xsl:template>
          <xsl:template match="db:sect1">
            <DIV CLASS="sect1"><xsl:apply-templates/></DIV>
          </xsl:template>
          <xsl:template match="db:sect2">
            <DIV CLASS="sect2"><xsl:apply-templates/></DIV>
          </xsl:template>
          <xsl:template match="db:title">
            <H1><xsl:apply-templates/></H1>
          </xsl:template>
          <xsl:template match="db:para">
            <P><xsl:apply-templates/></P>
          </xsl:template>
          <xsl:template match="db:emphasis">
            <I><xsl:apply-templates/></I>
          </xsl:template>
        </xsl:stylesheet>
"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;

    assert_eq!(
        result.to_xml(),
        "<HTML><BODY>
        <DIV CLASS='sect1'>
          <H1>Level 1 Heading</H1>
          <P>First paragraph</P>
          <DIV CLASS='sect2'>
            <H1>Level 2 Heading</H1>
            <P>Second paragraph</P>
          </DIV>
          <DIV CLASS='sect2'>
            <H1>Second Level 2 Heading</H1>
            <P>Third paragraph</P>
          </DIV>
        </DIV>
        <DIV CLASS='sect1'>
          <H1>Second Level 1 Heading</H1>
          <P>Fourth paragraph</P>
          <DIV CLASS='sect2'>
            <H1>Third Level 2 Heading</H1>
            <P>Fifth paragraph</P>
          </DIV>
        </DIV></BODY></HTML>"
    );
    Ok(())
}

pub fn md_1<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let (result, _rd) = test_rig(
        "<article>
          <heading1>Level 1 Heading</heading1>
          <para>First paragraph with <emph>emphasised</emph> text, <emph role='strong'>bold</emph> text, and <emph role='underline'>underlined</emph> text</para>
        </article>
",
        r#"<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
          xmlns:db="http://docbook.org/ns/docbook"
          version="3.0">

          <xsl:output indent="yes"/>
          <xsl:strip-space elements="*"/>

          <xsl:template match="article">
            <db:article>
              <xsl:for-each-group select="*" group-starting-with="heading1">
                <xsl:choose>
                  <xsl:when test="current-group()[position() eq 1]/self::heading1">
                    <db:sect1>
                      <xsl:apply-templates select="current-group()[position() eq 1]"/>
                      <xsl:for-each-group select="current-group()[position() ne 1]"
                        group-starting-with="heading2">
                        <xsl:choose>
                          <xsl:when test="current-group()[position() eq 1]/self::heading2">
                            <db:sect2>
                              <xsl:apply-templates select="current-group()"/>
                            </db:sect2>
                          </xsl:when>
                          <xsl:otherwise>
                            <xsl:apply-templates select="current-group()"/>
                          </xsl:otherwise>
                        </xsl:choose>
                      </xsl:for-each-group>
                    </db:sect1>
                  </xsl:when>
                  <xsl:otherwise>
                    <xsl:apply-templates select="current-group()"/>
                  </xsl:otherwise>
                </xsl:choose>
              </xsl:for-each-group>
            </db:article>
          </xsl:template>
          <xsl:template match="heading1|heading2">
            <db:title>
              <xsl:apply-templates/>
            </db:title>
          </xsl:template>
          <xsl:template match="para">
            <db:para>
              <xsl:apply-templates/>
            </db:para>
          </xsl:template>
          <xsl:template match="emph">
            <db:emphasis>
              <xsl:apply-templates select="@*|node()"/>
            </db:emphasis>
          </xsl:template>
          <xsl:template match="attribute::role|attribute::id">
            <xsl:copy/>
          </xsl:template>
          <!--xsl:template match="emph[@role eq 'strong']">
            <db:emphasis role="strong">
              <xsl:apply-templates select="@*|node()"/>
            </db:emphasis>
          </xsl:template>
          <xsl:template match="emph[@role eq 'underline']">
            <db:emphasis role="underline">
              <xsl:apply-templates select="@*|node()"/>
            </db:emphasis>
          </xsl:template-->
        </xsl:stylesheet>
"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    )?;

    assert_eq!(
        result.to_xml(),
        "<db:article xmlns:db='http://docbook.org/ns/docbook'><db:sect1><db:title>Level 1 Heading</db:title><db:para>First paragraph with <db:emphasis>emphasised</db:emphasis> text, <db:emphasis role='strong'>bold</db:emphasis> text, and <db:emphasis role='underline'>underlined</db:emphasis> text</db:para></db:sect1></db:article>"
    );
    Ok(())
}

pub fn conform_1<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<article>
          <heading1>Level 1 Heading</heading1>
          <para>First paragraph with <emph>emphasised</emph> text, <emph role='strong'>bold</emph> text, and <emph role='underline'>underlined</emph> text</para>
        </article>
",
        r#"<?xml version="1.0" encoding="utf-8"?>
        <xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
		xmlns="http://www.w3.org/1999/xhtml"
		xmlns:h="http://www.w3.org/1999/xhtml"
		xmlns:f="http://docbook.org/xslt/ns/extension"
		xmlns:m="http://docbook.org/xslt/ns/mode"
		xmlns:fn="http://www.w3.org/2003/11/xpath-functions"
		xmlns:db="http://docbook.org/docbook-ng"
		exclude-result-prefixes="h f m fn db"
                        version="2.0">

        <xsl:variable name="dummy">
          <db:book>
              <db:info>
	<db:title>Book Title</db:title>
              </db:info>
            <db:chapter>
              <db:info>
	<db:title>ChapterTitle</db:title>
              </db:info>
              <db:para/>
            </db:chapter>
          </db:book>
        </xsl:variable>

        <xsl:template match="/">
          <xsl:apply-templates select="$dummy/db:book/db:chapter/db:info/db:title"
		       mode="m:titlepage-mode"/>
        </xsl:template>

        <xsl:template match="db:chapter/db:info/db:title
		     |db:appendix/db:info/db:title
		     |db:preface/db:info/db:title
		     |db:bibliography/db:info/db:title"
	      mode="m:titlepage-mode"
	      priority="100">

          <h2>
            <xsl:next-match/>
          </h2>
        </xsl:template>

        <xsl:template match="db:title" mode="m:titlepage-mode">
          <xsl:apply-templates/>
        </xsl:template>

        <xsl:template match="*" mode="m:titlepage-mode">
          <xsl:apply-templates select="."/>
        </xsl:template>

        </xsl:stylesheet>

"#,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    );

    assert_eq!(result.is_ok(), true);
    Ok(())
}

pub fn conform_2<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<article>
          <heading1>Level 1 Heading</heading1>
          <para>First paragraph with <emph>emphasised</emph> text, <emph role='strong'>bold</emph> text, and <emph role='underline'>underlined</emph> text</para>
        </article>
",
        r###"<?xml version="1.0" encoding="utf-8"?>
        <xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
		xmlns="http://www.w3.org/1999/xhtml"
		xmlns:h="http://www.w3.org/1999/xhtml"
		xmlns:f="http://docbook.org/xslt/ns/extension"
		xmlns:m="http://docbook.org/xslt/ns/mode"
		xmlns:fn="http://www.w3.org/2003/11/xpath-functions"
		xmlns:db="http://docbook.org/docbook-ng"
		exclude-result-prefixes="h f m fn db"
                        version="2.0">

        <xsl:variable name="dummy">
          <db:book>
              <db:info>
	<db:title>Book Title</db:title>
              </db:info>
            <db:chapter>
              <db:info>
	<db:title>ChapterTitle</db:title>
              </db:info>
              <db:para/>
            </db:chapter>
          </db:book>
        </xsl:variable>

        <xsl:template match="/">
          <xsl:apply-templates select="$dummy/db:book/db:chapter/db:info/db:title"
		       mode="m:titlepage-mode"/>
        </xsl:template>

        <xsl:template match="db:chapter/db:info/db:title
		     |db:appendix/db:info/db:title
		     |db:preface/db:info/db:title
		     |db:bibliography/db:info/db:title"
	      mode="m:titlepage-mode"
	      priority="100">

          <h2>
            <xsl:next-match/>
          </h2>
        </xsl:template>

        <xsl:template match="db:title" mode="#all">
          <xsl:apply-templates/>
        </xsl:template>

        <xsl:template match="*" mode="m:titlepage-mode">
          <xsl:apply-templates select="."/>
        </xsl:template>

        </xsl:stylesheet>

"###,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    );

    assert!(result.is_err());
    Ok(())
}

pub fn conform_3<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<article>
          <heading1>Level 1 Heading</heading1>
          <para>First paragraph with <emph>emphasised</emph> text, <emph role='strong'>bold</emph> text, and <emph role='underline'>underlined</emph> text</para>
        </article>
",
        r###"<?xml version="1.0" encoding="UTF-8"?>
        <t:transform xmlns:t="http://www.w3.org/1999/XSL/Transform" version="2.0">
        <!-- Purpose: Test of select in xsl:value-of with current function -->

           <t:variable name="var">
		    <doc xmlns:xsl="http://www.w3.org/1999/XSL/Transform">6<num1>1<num2>2<num3>3</num3>
                    </num2>
                 </num1>
                 <num4>4<num6>at3</num6>
                 </num4>
                 <num5>5</num5>
              </doc>
	  </t:variable>

           <t:template match="doc">
		    <out>
                 <t1>
                    <t:value-of select="name($var//*[string() = current()/@*])" separator="|"/>
                 </t1>
              </out>
	  </t:template>

           <t:template match="text()"/>
        </t:transform>

"###,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    );

    assert!(result.is_ok());
    Ok(())
}

pub fn conform_4<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<article>
          <heading1>Level 1 Heading</heading1>
          <para>First paragraph with <emph>emphasised</emph> text, <emph role='strong'>bold</emph> text, and <emph role='underline'>underlined</emph> text</para>
        </article>
",
        r###"<?xml version="1.0" encoding="UTF-8"?>
        <xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform" version="2.0">

          <!-- Purpose: When no stylesheets are imported, an xsl:apply-imports should
               select the built-in templates. -->

        <xsl:template match="/">
          <result>
            Before apply-imports
              <xsl:apply-imports/>
            After apply-imports
          </result>
        </xsl:template>

        </xsl:stylesheet>

"###,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    );

    assert!(result.is_ok());
    Ok(())
}

pub fn conform_5<N: Node, G, H, J>(
    parse_from_str: G,
    parse_from_str_with_ns: J,
    make_doc: H,
) -> Result<(), Error>
where
    G: Fn(&str) -> Result<N, Error>,
    H: Fn() -> Result<N, Error>,
    J: Fn(&str) -> Result<(N, Option<NamespaceMap>), Error>,
{
    let result = test_rig(
        "<doc>
          <c x='attribute'>test</c>
          <heading1>Level 1 Heading</heading1>
          <para>First paragraph with <emph>emphasised</emph> text, <emph role='strong'>bold</emph> text, and <emph role='underline'>underlined</emph> text</para>
        </doc>
",
        r###"<?xml version="1.0"?>
        <xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform" version="2.0">

        <?spec xpath#id-path-expressions?>
          <!-- Purpose: Tests following axis starting from an attribute. -->
          <!-- Author: Scott Boag -->

        <xsl:template match="/">
          <out>
            <xsl:for-each select="//c/@x">
              <xsl:apply-templates select="following::*"/>
            </xsl:for-each>
          </out>
        </xsl:template>

        <xsl:template match="*">
          <xsl:text> </xsl:text><xsl:value-of select="name()"/>
        </xsl:template>

        </xsl:stylesheet>
"###,
        parse_from_str,
        parse_from_str_with_ns,
        make_doc,
    );

    assert!(result.is_ok());
    Ok(())
}
