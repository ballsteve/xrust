use std::rc::Rc;
use xrust::Node;
use xrust::parser::xml;
use xrust::trees::smite::{Node as SmiteNode};
use xrust::validators::relaxng::validate_relaxng;

mod conformance;

//use std::convert::TryFrom;
//use std::fs;
//use xrust::{Document, Error};

/*
#[test]
#[ignore]
fn bigfile() {
    /* A million elements, each with an attribute and value */

    let testxml =
        Document::try_from((fs::read_to_string("tests/xml/45M.xml").unwrap(), None, None));

    assert!(testxml.is_ok());
}
 */

/*
#[test]
fn rngtest() {

    let docfile = r#"<?xml version="1.0" encoding="utf-8"?>
<foo/>"#.to_string();

    /*
    let schemafile = r#"<?xml version="1.0" encoding="utf-8"?>
<rng:grammar>
<rng:start>
<rng:ref name="foo"/>
</rng:start>
<rng:define name="foo">
<element xmlns="http://relaxng.org/ns/structure/1.0" ns="">
<rng:name ns="">foo</rng:name>
<empty/>
</element>
</rng:define>
</rng:grammar>"#.to_string();
     */

    let schemafile = r#"<?xml version="1.0" encoding="utf-8"?>
<element  xmlns:rng="http://relaxng.org/ns/structure/1.0" xmlns="http://relaxng.org/ns/structure/1.0" ns=""><rng:name ns="">foo</rng:name><empty/></element>"#.to_string();



    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None, None);

    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None, None);

    println!("doc-{:?}", doc.to_xml());
    println!("pat-{:?}", sch.to_xml());
    let result = validate_relaxng(&doc, &sch);
    println!("res-{:?}", result.is_ok());
    //assert!(result.is_ok());
        }

 */