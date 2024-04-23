#[macro_export]
macro_rules! xslt_tests (
    ( $x:expr , $y:expr ) => {
	use url::Url;
	use xrust::xslt::from_document;

	#[test]
	fn xslt_literal_text() {
	    let src = Item::Node(
		$x("<Test><Level1>one</Level1><Level1>two</Level1></Test>")
		    .expect("unable to parse source document")
	    );

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
	    let seq = ctxt.evaluate(&mut StaticContext::<F>::new()).expect("evaluation failed");

	    assert_eq!(seq.to_string(), "Found the document")
	}

	#[test]
	fn xslt_value_of_1() {
	    let src = Item::Node(
		$x("<Test>special &lt; less than</Test>")
		    .expect("unable to parse source document")
	    );

	    let style = $x("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::*'><xsl:value-of select='.'/></xsl:template>
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
		ctxt.result_document($y());

		// Let 'er rip
	    let seq = ctxt.evaluate(&mut StaticContext::<F>::new()).expect("evaluation failed");

	    assert_eq!(seq.to_string(), "special &lt; less than")
	}
	#[test]
	fn xslt_value_of_2() {
	    let src = Item::Node(
		$x("<Test>special &lt; less than</Test>")
		    .expect("unable to parse source document")
	    );

	    let style = $x("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::*'><xsl:value-of select='.' disable-output-escaping='yes'/></xsl:template>
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
		ctxt.result_document($y());

		// Let 'er rip
	    let seq = ctxt.evaluate(&mut StaticContext::<F>::new()).expect("evaluation failed");

	    assert_eq!(seq.to_string(), "special < less than")
	}

	#[test]
	fn xslt_literal_element() {
	    let src = Item::Node(
		$x("<Test><Level1>one</Level1><Level1>two</Level1></Test>")
		    .expect("unable to parse source document")
	    );

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

	    let seq = ctxt.evaluate(&mut StaticContext::<F>::new()).expect("evaluation failed");

	    assert_eq!(seq.to_xml(), "<answer>Made an element</answer>")
	}
	#[test]
	fn xslt_element() {
	    let src = Item::Node(
		$x("<Test><Level1>one</Level1><Level1>two</Level1></Test>")
		    .expect("unable to parse source document")
	    );

	    let style = $x("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:element name='answer{count(ancestor::*)}'>Made an element</xsl:element></xsl:template>
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

	    let seq = ctxt.evaluate(&mut StaticContext::<F>::new()).expect("evaluation failed");

	    assert_eq!(seq.to_xml(), "<answer0>Made an element</answer0>")
	}

	#[test]
	fn xslt_apply_templates_1() {
	    let src = Item::Node(
		$x("<Test><Level1>one</Level1><Level1>two</Level1></Test>")
		    .expect("unable to parse source document")
	    );

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

	    let seq = ctxt.evaluate(&mut StaticContext::<F>::new()).expect("evaluation failed");

	    assert_eq!(seq.to_xml(), "found textfound text")
	}

	#[test]
	fn xslt_apply_templates_2() {
	    let src = Item::Node(
		$x("<Test>one<Level1/>two<Level1/>three<Level1/>four<Level1/></Test>")
		    .expect("unable to parse source document")
	    );

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

	    let seq = ctxt.evaluate(&mut StaticContext::<F>::new()).expect("evaluation failed");

	    assert_eq!(seq.to_xml(), "onetwothreefour")
	}

	#[test]
	fn xslt_comment() {
	    let src = Item::Node(
		$x("<Test>one<Level1/>two<Level1/>three<Level1/>four<Level1/></Test>")
		    .expect("unable to parse source document")
	    );

	    let style = $x("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Level1'><xsl:comment> this is a level 1 element </xsl:comment></xsl:template>
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
		ctxt.result_document($y());

	    let seq = ctxt.evaluate(&mut StaticContext::<F>::new()).expect("evaluation failed");

	    assert_eq!(seq.to_xml(), "one<!-- this is a level 1 element -->two<!-- this is a level 1 element -->three<!-- this is a level 1 element -->four<!-- this is a level 1 element -->")
	}

	#[test]
	fn xslt_pi() {
	    let src = Item::Node(
		$x("<Test>one<Level1/>two<Level1/>three<Level1/>four<Level1/></Test>")
		    .expect("unable to parse source document")
	    );

	    let style = $x("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Level1'><xsl:processing-instruction name='piL1'>this is a level 1 element</xsl:processing-instruction></xsl:template>
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
		ctxt.result_document($y());

	    let seq = ctxt.evaluate(&mut StaticContext::<F>::new()).expect("evaluation failed");

	    assert_eq!(seq.to_xml(), "one<?piL1 this is a level 1 element?>two<?piL1 this is a level 1 element?>three<?piL1 this is a level 1 element?>four<?piL1 this is a level 1 element?>")
	}

	#[test]
	fn xslt_message_1() {
	    let src = Item::Node(
		$x("<Test>one<Level1/>two<Level1/>three<Level1/>four<Level1/></Test>")
		    .expect("unable to parse source document")
	    );

	    let style = $x("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Level1'><xsl:message>here is a level 1 element</xsl:message><L/></xsl:template>
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
		ctxt.result_document($y());

		let mut msgs: Vec<String> = vec![];
		let mut stctxt = StaticContextBuilder::new()
			.message(|m| {msgs.push(String::from(m)); Ok(())})
			.build();
	    let seq = ctxt.evaluate(&mut stctxt).expect("evaluation failed");

	    assert_eq!(seq.to_xml(), "one<L></L>two<L></L>three<L></L>four<L></L>");
		assert_eq!(msgs.len(), 4);
		assert_eq!(msgs[0], "here is a level 1 element")
	}

	#[test]
	fn xslt_message_term() {
	    let src = Item::Node(
		$x("<Test>one<Level1/>two<Level1/>three<Level1/>four<Level1/></Test>")
		    .expect("unable to parse source document")
	    );

	    let style = $x("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Level1'><xsl:message terminate='yes'>here is a level 1 element</xsl:message><L/></xsl:template>
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
		ctxt.result_document($y());

		let mut msgs: Vec<String> = vec![];
		let mut stctxt = StaticContextBuilder::new()
			.message(|m| {msgs.push(String::from(m)); Ok(())})
			.build();
	    match ctxt.evaluate(&mut stctxt) {
			Err(e) => {
				assert_eq!(e.kind, ErrorKind::Terminated);
				assert_eq!(e.message, "here is a level 1 element");
				assert_eq!(e.code.unwrap().to_string(), "XTMM9000")
			}
			Ok(_) => panic!("evaluation succeeded when it should have failed")
		}
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
		let src = Item::Node($x(srcxml).expect("unable to parse source document"));
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

	    let seq = ctxt.evaluate(&mut StaticContext::<F>::new()).expect("evaluation failed");

	    assert_eq!(seq.to_xml(), r#"<dat:dataPack xmlns:dat='http://www.stormware.cz/schema/version_2/data.xsd' xmlns:int='http://www.stormware.cz/schema/version_2/intDoc.xsd'>
    <int:head>XSLT in Rust</int:head>
    <int:body>A simple document.</int:body>
</dat:dataPack>"#)
	}

	#[test]
	#[should_panic]
	fn xslt_include() {
	    let src = Item::Node(
		$x("<Test>one<Level1/>two<Level2/>three<Level3/>four<Level4/></Test>")
		    .expect("unable to parse source document")
	    );

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

	    let seq = ctxt.evaluate(&mut StaticContext::<F>::new()).expect("evaluation failed");

	    assert_eq!(seq.to_xml(), "onefound Level1 elementtwofound Level2 elementthreefound Level3 elementfour")
	}

	#[test]
	fn xslt_fn_current() {
	    let src = Item::Node(
		$x("<Test ref='one'><second name='foo'>I am foo</second><second name='one'>I am one</second></Test>")
		    .expect("unable to parse source document")
	    );

	    let style = $x("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'>
    <xsl:sequence select='child::*/child::second[attribute::name eq current()/attribute::ref]'/>
  </xsl:template>
</xsl:stylesheet>").expect("unable to parse stylesheet");

	    // Setup dynamic context with result document
	    let mut ctxt = from_document(
			style,
			None,
			|s| $x(s),
			|url| Ok(String::new()),
	    ).expect("failed to compile stylesheet");

	    ctxt.context(vec![src.clone()], 0);
		ctxt.previous_context(src);
		ctxt.result_document($y());

	    let seq = ctxt.evaluate(&mut StaticContext::<F>::new()).expect("evaluation failed");

	    assert_eq!(seq.to_xml(), "<second name='one'>I am one</second>")
	}

	#[test]
	fn xslt_key_1() {
	    let src = $x("<Test><one>blue</one><two>yellow</two><three>green</three><four>blue</four></Test>")
		    .expect("unable to parse source document");

	    let style = $x(r#"<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:key name='mykey' match='child::*' use='child::text()'/>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'>#blue = <xsl:sequence select='count(key("mykey", "blue"))'/></xsl:template>
  <xsl:template match='child::Test/child::*'>shouldn't see this</xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>"#).expect("unable to parse stylesheet");

		// A static context is needed for all evaluations
		let mut stctxt = StaticContext::<F>::new();

	    // Setup dynamic context with result document
	    let mut ctxt = from_document(
			style,
			None,
			|s| $x(s),
			|url| Ok(String::new()),
	    ).expect("failed to compile stylesheet");

	    ctxt.context(vec![Item::Node(src.clone())], 0);
		ctxt.result_document($y());
		ctxt.populate_key_values(&mut stctxt, src.clone());

	    let seq = ctxt.evaluate(&mut stctxt).expect("evaluation failed");

	    assert_eq!(seq.to_xml(), "#blue = 2")
	}

/*
	#[test]
	fn import_1() {
	    let mut sc = StaticContext::new_with_xslt_builtins();

	    let src = Item::Node($x("<Test><Level1>one</Level1><Level2>two</Level2><Level3>three</Level3><Level4>four</Level4></Test>"));

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

	    let src = Item::Node($x("<Test><Level1>one</Level1><Level2>two</Level2><Level3>three</Level3><Level4>four</Level4></Test>"));

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
