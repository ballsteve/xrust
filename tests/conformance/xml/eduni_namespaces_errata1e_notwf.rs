/*

Richard Tobin's XML Namespaces 1.0/1.1 2nd edition test suite 1 June 2006

*/

use std::fs;
use xrust::item::Node;
use xrust::parser::xml;
use xrust::trees::smite::RNode;

#[test]
fn rmtnse1013a() {
    /*
        Test ID:rmt-ns-e1.0-13a
        Test URI:NE13a.xml
        Spec Sections:NE13
        Description:The xml namespace must not be declared as the default namespace.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/errata-1e/NE13a.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn rmtnse1013b() {
    /*
        Test ID:rmt-ns-e1.0-13b
        Test URI:NE13b.xml
        Spec Sections:NE13
        Description:The xmlns namespace must not be declared as the default namespace.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/errata-1e/NE13b.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn rmtnse1013c() {
    /*
        Test ID:rmt-ns-e1.0-13c
        Test URI:NE13c.xml
        Spec Sections:NE13
        Description:Elements must not have the prefix xmlns.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/errata-1e/NE13c.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}
