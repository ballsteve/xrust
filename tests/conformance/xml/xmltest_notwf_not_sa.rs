/*

James Clark XMLTEST cases - Standalone

    This contains cases that are not well-formed XML documents
    This contains cases that are not standalone.

*/

use std::convert::TryFrom;
use std::fs;
use xrust::parsexml;

#[test]
fn notwfnotsa001() {
    /*
        Test ID:not-wf-not-sa-001
        Test URI:not-wf/not-sa/001.xml
        Spec Sections:3.4 [62]
        Description:Conditional sections must be properly terminated ("]>" used instead of "]]>").
    */

    let testxml = parsexml::XMLDocument::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/001.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn notwfnotsa002() {
    /*
        Test ID:not-wf-not-sa-002
        Test URI:not-wf/not-sa/002.xml
        Spec Sections:2.6 [17]
        Description:Processing instruction target names may not be "XML" in any combination of cases.
    */

    let testxml = parsexml::XMLDocument::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/002.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn notwfnotsa003() {
    /*
        Test ID:not-wf-not-sa-003
        Test URI:not-wf/not-sa/003.xml
        Spec Sections:3.4 [62]
        Description:Conditional sections must be properly terminated ("]]>" omitted).
    */

    let testxml = parsexml::XMLDocument::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/003.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn notwfnotsa004() {
    /*
        Test ID:not-wf-not-sa-004
        Test URI:not-wf/not-sa/004.xml
        Spec Sections:3.4 [62]
        Description:Conditional sections must be properly terminated ("]]>" omitted).
    */

    let testxml = parsexml::XMLDocument::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/004.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn notwfnotsa005() {
    /*
        Test ID:not-wf-not-sa-005
        Test URI:not-wf/not-sa/005.xml
        Spec Sections:4.1
        Description:Tests the Entity Declared VC by referring to an undefined parameter entity within an external entity.
    */

    let testxml = parsexml::XMLDocument::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/005.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn notwfnotsa006() {
    /*
        Test ID:not-wf-not-sa-006
        Test URI:not-wf/not-sa/006.xml
        Spec Sections:3.4 [62]
        Description:Conditional sections need a '[' after the INCLUDE or IGNORE.
    */

    let testxml = parsexml::XMLDocument::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/006.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn notwfnotsa007() {
    /*
        Test ID:not-wf-not-sa-007
        Test URI:not-wf/not-sa/007.xml
        Spec Sections:4.3.2 [79]
        Description:A <!DOCTYPE ...> declaration may not begin any external entity; it's only found once, in the document entity.
    */

    let testxml = parsexml::XMLDocument::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/007.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn notwfnotsa008() {
    /*
        Test ID:not-wf-not-sa-008
        Test URI:not-wf/not-sa/008.xml
        Spec Sections:4.1 [69]
        Description:In DTDs, the '%' character must be part of a parameter entity reference.
    */

    let testxml = parsexml::XMLDocument::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/008.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn notwfnotsa009() {
    /*
        Test ID:not-wf-not-sa-009
        Test URI:not-wf/not-sa/009.xml
        Spec Sections:2.8
        Description:This test violates WFC:PE Between Declarations in Production 28a. The last character of a markup declaration is not contained in the same parameter-entity text replacement.
    */

    let testxml = parsexml::XMLDocument::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/009.xml").unwrap(),
    );

    assert!(testxml.is_err());
}
