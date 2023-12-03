use std::rc::Rc;
use crate::Item;
#[macro_export]
macro_rules! xslt_tests (
    ( $x:expr , $y:expr ) => {
	use url::Url;
	use xrust::xslt::from_document;

	#[test]
	fn xslt_literal_text() {
	    let src = Rc::new(Item::Node(
		$x("<Test><Level1>one</Level1><Level1>two</Level1></Test>")
		    .expect("unable to parse source document")
	    ));

	    let style = $x("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'>Found the document</xsl:template>
</xsl:stylesheet>").expect("unable to parse stylesheet");

	    // Setup context
	    let mut ctxt = from_document(
			style,
			None,
			|s| $x(s),
			|url| Ok(String::new()),
	    ).expect("failed to compile stylesheet");

		// Add the source document to the context
		ctxt.context(vec![src], 0);

		// Let 'er rip
	    let seq = ctxt.evaluate().expect("evaluation failed");

	    assert_eq!(seq.to_string(), "Found the document")
	}

	#[test]
	fn xslt_literal_element() {
	    let src = Rc::new(Item::Node(
		$x("<Test><Level1>one</Level1><Level1>two</Level1></Test>")
		    .expect("unable to parse source document")
	    ));

	    let style = $x("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><answer>Made an element</answer></xsl:template>
</xsl:stylesheet>").expect("unable to parse stylesheet");

	    // Setup dynamic context with result document
	    let mut ctxt = from_document(
			style,
			None,
			|s| $x(s),
			|url| Ok(String::new()),
	    ).expect("failed to compile stylesheet");

	    ctxt.context(vec![src], 0);
		ctxt.result_document($y());

	    let seq = ctxt.evaluate().expect("evaluation failed");

	    assert_eq!(seq.to_xml(), "<answer>Made an element</answer>")
	}

	#[test]
	fn xslt_apply_templates_1() {
	    let src = Rc::new(Item::Node(
		$x("<Test><Level1>one</Level1><Level1>two</Level1></Test>")
		    .expect("unable to parse source document")
	    ));

	    let style = $x("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::*'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::text()'>found text</xsl:template>
</xsl:stylesheet>").expect("unable to parse stylesheet");

	    // Setup dynamic context
	    let mut ctxt = from_document(
			style,
			None,
			|s| $x(s),
			|url| Ok(String::new()),
	    ).expect("failed to compile stylesheet");

	    ctxt.context(vec![src], 0);

	    let seq = ctxt.evaluate().expect("evaluation failed");

	    assert_eq!(seq.to_xml(), "found textfound text")
	}

	#[test]
	fn xslt_apply_templates_2() {
	    let src = Rc::new(Item::Node(
		$x("<Test>one<Level1/>two<Level1/>three<Level1/>four<Level1/></Test>")
		    .expect("unable to parse source document")
	    ));

	    let style = $x("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:apply-templates select='child::text()'/></xsl:template>
  <xsl:template match='child::Level1'>found Level1 element</xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>").expect("unable to parse stylesheet");

	    // Setup dynamic context with result document
	    let mut ctxt = from_document(
			style,
			None,
			|s| $x(s),
			|url| Ok(String::new()),
	    ).expect("failed to compile stylesheet");

	    ctxt.context(vec![src], 0);

	    let seq = ctxt.evaluate().expect("evaluation failed");

	    assert_eq!(seq.to_xml(), "onetwothreefour")
	}

	// Although we have the source and stylesheet in files,
	// they are inlined here to avoid dependency on I/O libraries
	#[test]
	fn xslt_issue_58() {
		let srcxml = r#"<Example>
    <Title>XSLT in Rust</Title>
    <Paragraph>A simple document.</Paragraph>
</Example>
"#;
		let src = Rc::new(Item::Node($x(srcxml).expect("unable to parse source document")));
		let stylexml = r#"<xsl:stylesheet
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
"#;
		let style = $x(stylexml).expect("unable to parse stylesheet document");

	    // Setup dynamic context with result document
	    let mut ctxt = from_document(
			style,
			None,
			|s| $x(s),
			|url| Ok(String::new()),
	    ).expect("failed to compile stylesheet");

	    ctxt.context(vec![src], 0);
		ctxt.result_document($y());

	    let seq = ctxt.evaluate().expect("evaluation failed");

	    assert_eq!(seq.to_xml(), r#"<dat:dataPack xmlns:dat='http://www.stormware.cz/schema/version_2/data.xsd' xmlns:int='http://www.stormware.cz/schema/version_2/intDoc.xsd'>
    <int:head>XSLT in Rust</int:head>
    <int:body>A simple document.</int:body>
</dat:dataPack>"#)
	}

	#[test]
	#[should_panic]
	fn xslt_include() {
	    let src = Rc::new(Item::Node(
		$x("<Test>one<Level1/>two<Level2/>three<Level3/>four<Level4/></Test>")
		    .expect("unable to parse source document")
	    ));

	    let style = $x("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:include href='included.xsl'/>
  <xsl:template match='child::Test'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Level1'>found Level1 element</xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>").expect("unable to parse stylesheet");

	    // Setup dynamic context with result document
	    let pwd = std::env::current_dir().expect("unable to get current directory");
	    let pwds = pwd.into_os_string().into_string().expect("unable to convert pwd");
	    let mut ctxt = from_document(
			style,
			Some(Url::parse(format!("file://{}/tests/xsl/including.xsl", pwds.as_str()).as_str()).expect("unable to parse URL")),
			|s| $x(s),
			|_| Ok(String::new()),
	    ).expect("failed to compile stylesheet");

	    ctxt.context(vec![src], 0);

	    let seq = ctxt.evaluate().expect("evaluation failed");

	    assert_eq!(seq.to_xml(), "onefound Level1 elementtwofound Level2 elementthreefound Level3 elementfour")
	}

/*
	#[test]
	fn import_1() {
	    let mut sc = StaticContext::new_with_xslt_builtins();

	    let src = Rc::new(Item::Node($x("<Test><Level1>one</Level1><Level2>two</Level2><Level3>three</Level3><Level4>four</Level4></Test>")));

	    let style = $x("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:import href='imported.xsl'/>
  <xsl:template match='child::Test'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Level1'>shallower import level</xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>");

	    // Setup dynamic context with result document
	    let pwd = std::env::current_dir().expect("unable to get current directory");
	    let pwds = pwd.into_os_string().into_string().expect("unable to convert pwd");
	    let ev = from_document(
		style,
		&mut sc,
		Some(Url::parse(format!("file://{}/tests/xsl/importing.xsl", pwds.as_str()).as_str()).expect("unable to parse URL")),
	    )
		.expect("failed to compile stylesheet");

	    let rd = $y();

	    // Prime the stylesheet evaluation by finding the template for the document root
	    // and making the document root the initial context
	    let t = ev.find_match(&src, rd, None)
		.expect("unable to find match");
	    assert!(t.len() >= 1);

	    let seq = ev.evaluate(Some(vec![Rc::clone(&src)]), Some(0), &t, rd)
		.expect("evaluation failed");

	    assert_eq!(seq.to_xml(), "shallower import leveltwothreefour")
	}

	#[test]
	fn apply_import() {
	    let mut sc = StaticContext::new_with_xslt_builtins();

	    let src = Rc::new(Item::Node($x("<Test><Level1>one</Level1><Level2>two</Level2><Level3>three</Level3><Level4>four</Level4></Test>")));

	    let style = $x("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:import href='imported.xsl'/>
  <xsl:template match='child::Test'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Level1'>shallow1 <xsl:apply-imports/> shallow2</xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>");

	    // Setup dynamic context with result document
	    let pwd = std::env::current_dir().expect("unable to get current directory");
	    let pwds = pwd.into_os_string().into_string().expect("unable to convert pwd");
	    let ev = from_document(
		style,
		&mut sc,
		Some(Url::parse(format!("file://{}/tests/xsl/importing.xsl", pwds.as_str()).as_str()).expect("unable to parse URL")),
	    )
		.expect("failed to compile stylesheet");

	    let rd = $y();

	    // Prime the stylesheet evaluation by finding the template for the document root
	    // and making the document root the initial context
	    let t = ev.find_match(&src, rd, None)
		.expect("unable to find match");
	    assert!(t.len() >= 1);

	    let seq = ev.evaluate(Some(vec![Rc::clone(&src)]), Some(0), &t, rd)
		.expect("evaluation failed");

	    assert_eq!(seq.to_xml(), "shallow1 deeper import level shallow2twothreefour")
	}*/
    }
);
