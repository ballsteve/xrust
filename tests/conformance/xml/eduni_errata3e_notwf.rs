/*
Richard Tobin's XML 1.0 3rd edition errata test suite 1 June 2006
 */

use std::fs;
use std::rc::Rc;
use xrust::parser::xml;
use xrust::trees::smite::Node as SmiteNode;

#[test]
fn rmte3e12() {
    /*
        Test ID:rmt-e3e-12
        Test URI:E12.xml
        Spec Sections:E12
        Description:Default values for attributes may not contain references to external entities.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-3e/E12.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}
