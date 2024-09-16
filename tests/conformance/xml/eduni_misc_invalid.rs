/*

Bjoern Hoehrmann via HST 2013-09-18

*/

use std::fs;
use std::rc::Rc;
use xrust::parser::xml;
use xrust::trees::smite::Node as SmiteNode;

#[test]
#[ignore]
fn hstbh005() {
    /*
        Test ID:hst-bh-005
        Test URI:005.xml
        Spec Sections:3.1 [41]
        Description:xmlns:xml is an attribute as far as validation is concerned and must be declared
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/misc/005.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn hstbh006() {
    /*
        Test ID:hst-bh-006
        Test URI:006.xml
        Spec Sections:3.1 [41]
        Description:xmlns:foo is an attribute as far as validation is concerned and must be declared
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/misc/006.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}
