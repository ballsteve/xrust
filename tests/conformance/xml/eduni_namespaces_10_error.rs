/*

Richard Tobin's XML Namespaces 1.0 test suite 14 Feb 2003

*/

use std::fs;
use std::rc::Rc;
use xrust::parser::xml;
use xrust::trees::smite::Node as SmiteNode;

#[test]
fn rmtns10004() {
    /*
        Test ID:rmt-ns10-004
        Test URI:004.xml
        Spec Sections:2
        Description:Namespace name test: a relative URI (deprecated)
    */


    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/004.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn rmtns10005() {
    /*
        Test ID:rmt-ns10-005
        Test URI:005.xml
        Spec Sections:2
        Description:Namespace name test: a same-document relative URI (deprecated)
    */


    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/005.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn rmtns10006() {
    /*
        Test ID:rmt-ns10-006
        Test URI:006.xml
        Spec Sections:2
        Description:Namespace name test: an http IRI that is not a URI
    */


    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/006.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}
