/*

Richard Tobin's XML 1.1 test suite 13 Feb 2003

 */

use crate::conformance::dtdfileresolve;
use std::fs;
use xrust::item::Node;
use xrust::parser::{xml, ParserConfig};
use xrust::trees::smite::RNode;

#[test]
fn rmt001() {
    /*
        Test ID:rmt-001
        Test URI:001.xml
        Spec Sections:2.8 4.3.4
        Description:External subset has later version number
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/eduni/xml-1.1/".to_string());

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/001.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}

#[test]
fn rmt002() {
    /*
        Test ID:rmt-002
        Test URI:002.xml
        Spec Sections:2.8 4.3.4
        Description:External PE has later version number
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/eduni/xml-1.1/".to_string());

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/002.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}

#[test]
fn rmt003() {
    /*
        Test ID:rmt-003
        Test URI:003.xml
        Spec Sections:2.8 4.3.4
        Description:External general entity has later version number
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/eduni/xml-1.1/".to_string());

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/003.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}

#[test]
fn rmt004() {
    /*
        Test ID:rmt-004
        Test URI:004.xml
        Spec Sections:2.8 4.3.4
        Description:External general entity has later version number (no decl means 1.0)
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/eduni/xml-1.1/".to_string());

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/004.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}

#[test]
fn rmt005() {
    /*
        Test ID:rmt-005
        Test URI:005.xml
        Spec Sections:2.8 4.3.4
        Description:Indirect external general entity has later version number
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/eduni/xml-1.1/".to_string());

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/005.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn rmt011() {
    /*
        Test ID:rmt-011
        Test URI:011.xml
        Spec Sections:2.2
        Description:Contains a C1 control, legal in XML 1.0, illegal in XML 1.1
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/011.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn rmt013() {
    /*
        Test ID:rmt-013
        Test URI:013.xml
        Spec Sections:2.2
        Description:Contains a DEL, legal in XML 1.0, illegal in XML 1.1
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/013.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

/*
#[test]
#[ignore]
fn rmt014() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:rmt-014
        Test URI:014.xml
        Spec Sections:2.3
        Description:Has a "long s" in a name, legal in XML 1.1, illegal in XML 1.0 thru 4th edition
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/014.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}
 */

/*
#[test]
#[ignore]
fn rmt016() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:rmt-016
        Test URI:016.xml
        Spec Sections:2.3
        Description:Has a Byzantine Musical Symbol Kratimata in a name, legal in XML 1.1, illegal in XML 1.0 thru 4th edition
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/016.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}
 */

/*
#[test]
#[ignore]
fn rmt019() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:rmt-019
        Test URI:019.xml
        Spec Sections:2.3
        Description:Has the last legal namechar in XML 1.1, illegal in XML 1.0 thru 4th edition
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/019.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}
 */

#[test]
fn rmt020() {
    /*
        Test ID:rmt-020
        Test URI:020.xml
        Spec Sections:2.3
        Description:Has the first character after the last legal namechar in XML 1.1, illegal in both XML 1.0 and 1.1
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/020.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn rmt021() {
    /*
        Test ID:rmt-021
        Test URI:021.xml
        Spec Sections:2.3
        Description:Has the first character after the last legal namechar in XML 1.1, illegal in both XML 1.0 and 1.1
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/021.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn rmt038() {
    /*
        Test ID:rmt-038
        Test URI:038.xml
        Spec Sections:2.2
        Description:Contains a C0 control character (form-feed), illegal in both XML 1.0 and 1.1
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/038.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn rmt039() {
    /*
        Test ID:rmt-039
        Test URI:039.xml
        Spec Sections:2.2
        Description:Contains a C0 control character (form-feed), illegal in both XML 1.0 and 1.1
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/039.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn rmt041() {
    /*
        Test ID:rmt-041
        Test URI:041.xml
        Spec Sections:2.2
        Description:Contains a C1 control character (partial line up), legal in XML 1.0 but not 1.1
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/041.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn rmt042() {
    /*
        Test ID:rmt-042
        Test URI:042.xml
        Spec Sections:4.1
        Description:Contains a character reference to a C0 control character (form-feed), legal in XML 1.1 but not 1.0
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/042.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}
