/*

Richard Tobin's XML 1.1 test suite 13 Feb 2003

*/

use crate::conformance::non_utf8_file_reader;
use std::fs;
use std::rc::Rc;
use xrust::parser::xml;
use xrust::trees::smite::Node as SmiteNode;

#[test]
fn rmt008() {
    /*
        Test ID:rmt-008
        Test URI:008.xml
        Spec Sections:2.8 4.3.4
        Description:an implausibly-versioned document
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/008.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn rmt009() {
    /*
        Test ID:rmt-009
        Test URI:009.xml
        Spec Sections:2.8 4.3.4
        Description:External general entity has implausible version number
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/009.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn rmt055() {
    /*
        Test ID:rmt-055
        Test URI:055.xml
        Spec Sections:2.11
        Description:Has a Latin-1 NEL in the XML declaration (to be made an error in PR)
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        non_utf8_file_reader("tests/conformance/xml/xmlconf/eduni/xml-1.1/055.xml").as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn rmt056() {
    /*
        Test ID:rmt-056
        Test URI:056.xml
        Spec Sections:2.11
        Description:Has a UTF-8 NEL in the XML declaration (to be made an error in PR)
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/056.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn rmt057() {
    /*
        Test ID:rmt-057
        Test URI:057.xml
        Spec Sections:2.11
        Description:Has a UTF-8 LSEP in the XML declaration (to be made an error in PR)
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/057.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}
