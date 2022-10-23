#[macro_export]
macro_rules! xslt_tests (
    ( $x:expr , $y:expr ) => {
	//use xrust::xpath::parse;
	//use xrust::evaluate::StaticContext;
	use xrust::xslt::from_document;

	#[test]
	fn xslt_literal_text() {
	    let mut sc = StaticContext::new_with_xslt_builtins();

	    let src = Rc::new(Item::Node(
		$x("<Test><Level1>one</Level1><Level1>two</Level1></Test>")
		    .expect("unable to parse source document")
	    ));

	    let style = $x("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'>Found the document</xsl:template>
</xsl:stylesheet>").expect("unable to parse stylesheet");

	    // Setup dynamic context with result document
	    let ev = from_document(
		style,
		&mut sc,
		None,
		$x,
	    )
		.expect("failed to compile stylesheet");

	    let rd = $y();

	    // Prime the stylesheet evaluation by finding the template for the document root
	    // and making the document root the initial context
	    let t = ev.find_match(&src, None, &rd)
		.expect("unable to find match");
	    assert!(t.len() >= 1);

	    let seq = ev.evaluate(Some(vec![Rc::clone(&src)]), Some(0), &t, &rd)
		.expect("evaluation failed");

	    assert_eq!(seq.to_string(), "Found the document")
	}

	#[test]
	fn xslt_literal_element() {
	    let mut sc = StaticContext::new_with_xslt_builtins();

	    let src = Rc::new(Item::Node(
		$x("<Test><Level1>one</Level1><Level1>two</Level1></Test>")
		    .expect("unable to parse source document")
	    ));

	    let style = $x("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><answer>Made an element</answer></xsl:template>
</xsl:stylesheet>").expect("unable to parse stylesheet");

	    // Setup dynamic context with result document
	    let ev = from_document(
		style,
		&mut sc,
		None,
		$x,
	    )
		.expect("failed to compile stylesheet");

	    let rd = $y();

	    // Prime the stylesheet evaluation by finding the template for the document root
	    // and making the document root the initial context
	    let t = ev.find_match(&src, None, &rd)
		.expect("unable to find match");
	    assert!(t.len() >= 1);

	    let seq = ev.evaluate(Some(vec![Rc::clone(&src)]), Some(0), &t, &rd)
		.expect("evaluation failed");

	    assert_eq!(seq.to_xml(), "<answer>Made an element</answer>")
	}

	#[test]
	fn xslt_apply_templates_1() {
	    let mut sc = StaticContext::new_with_xslt_builtins();

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
	    let ev = from_document(
		style,
		&mut sc,
		None,
		$x,
	    )
		.expect("failed to compile stylesheet");

	    let rd = $y();

	    // Prime the stylesheet evaluation by finding the template for the document root
	    // and making the document root the initial context
	    let t = ev.find_match(&src, None, &rd)
		.expect("unable to find match");
	    assert!(t.len() >= 1);

	    let seq = ev.evaluate(Some(vec![Rc::clone(&src)]), Some(0), &t, &rd)
		.expect("evaluation failed");

	    assert_eq!(seq.to_xml(), "found textfound text")
	}

	#[test]
	fn xslt_apply_templates_2() {
	    let mut sc = StaticContext::new_with_xslt_builtins();

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
	    let ev = from_document(
		style,
		&mut sc,
		None,
		$x,
	    )
		.expect("failed to compile stylesheet");

	    let rd = $y();

	    // Prime the stylesheet evaluation by finding the template for the document root
	    // and making the document root the initial context
	    let t = ev.find_match(&src, None, &rd)
		.expect("unable to find match");
	    assert!(t.len() >= 1);

	    let seq = ev.evaluate(Some(vec![Rc::clone(&src)]), Some(0), &t, &rd)
		.expect("evaluation failed");

	    assert_eq!(seq.to_xml(), "onetwothreefour")
	}
/*
	#[test]
	fn include() {
	    let mut sc = StaticContext::new_with_xslt_builtins();

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
	    let ev = from_document(
		style,
		&mut sc,
		Some(Url::parse(format!("file://{}/tests/xsl/including.xsl", pwds.as_str()).as_str()).expect("unable to parse URL")),
		$x,
	    )
		.expect("failed to compile stylesheet");

	    let rd = $y();

	    // Prime the stylesheet evaluation by finding the template for the document root
	    // and making the document root the initial context
	    let t = ev.find_match(&src, None, &rd)
		.expect("unable to find match");
	    assert!(t.len() >= 1);

	    let seq = ev.evaluate(Some(vec![Rc::clone(&src)]), Some(0), &t, &rd)
		.expect("evaluation failed");

	    assert_eq!(seq.to_xml(), "onefound Level1 elementtwofound Level2 elementthreefound Level3 elementfour")
	}

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
