/*

Richard Tobin's XML Namespaces 1.1 test suite 14 Feb 2003

*/

use std::fs;
use xrust::item::Node;
use xrust::parser::xml;
use xrust::trees::smite::RNode;

#[test]
fn rmtns11005() {
    /*
        Test ID:rmt-ns11-005
        Test URI:005.xml
        Spec Sections:5
        Description:Illegal use of prefix that has been unbound
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.1/005.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn htbhns11007() {
    /*
        Test ID:ht-bh-ns11-007
        Test URI:007.xml
        Spec Sections:3
        Description:Attempt to unbind xmlns 'namespace'
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.1/007.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn htbhns11008() {
    /*
        Test ID:ht-bh-ns11-008
        Test URI:008.xml
        Spec Sections:3
        Description:Attempt to unbind xml namespace
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.1/008.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}
