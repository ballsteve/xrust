/*

James Clark XMLTEST cases - Standalone

    This contains cases that are not well-formed XML documents
    This contains cases that are not standalone.

*/

use crate::conformance::dtdfileresolve;
use std::fs;
use xrust::item::Node;
use xrust::parser::{ParserConfig, xml};
use xrust::trees::smite::RNode;

#[test]
fn notwfnotsa001() {
    /*
        Test ID:not-wf-not-sa-001
        Test URI:not-wf/not-sa/001.xml
        Spec Sections:3.4 [62]
        Description:Conditional sections must be properly terminated ("]>" usedinstead of "]]>").
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/".to_string());

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/001.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfnotsa002() {
    /*
        Test ID:not-wf-not-sa-002
        Test URI:not-wf/not-sa/002.xml
        Spec Sections:2.6 [17]
        Description:Processing instruction target names may not be "XML"in any combination of cases.
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/".to_string());

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/002.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfnotsa003() {
    /*
        Test ID:not-wf-not-sa-003
        Test URI:not-wf/not-sa/003.xml
        Spec Sections:3.4 [62]
        Description:Conditional sections must be properly terminated ("]]>" omitted).
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/".to_string());

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/003.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfnotsa004() {
    /*
        Test ID:not-wf-not-sa-004
        Test URI:not-wf/not-sa/004.xml
        Spec Sections:3.4 [62]
        Description:Conditional sections must be properly terminated ("]]>" omitted).
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/".to_string());

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/004.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfnotsa005() {
    /*
        Test ID:not-wf-not-sa-005
        Test URI:not-wf/not-sa/005.xml
        Spec Sections:4.1
        Description:Tests the Entity Declared VC by referring to anundefined parameter entity within an external entity.
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/".to_string());

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/005.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfnotsa006() {
    /*
        Test ID:not-wf-not-sa-006
        Test URI:not-wf/not-sa/006.xml
        Spec Sections:3.4 [62]
        Description:Conditional sections need a '[' after the INCLUDE or IGNORE.
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/".to_string());

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/006.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfnotsa007() {
    /*
        Test ID:not-wf-not-sa-007
        Test URI:not-wf/not-sa/007.xml
        Spec Sections:4.3.2 [79]
        Description:A <!DOCTYPE ...> declaration may not begin any externalentity; it's only found once, in the document entity.
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/".to_string());

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/007.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfnotsa008() {
    /*
        Test ID:not-wf-not-sa-008
        Test URI:not-wf/not-sa/008.xml
        Spec Sections:4.1 [69]
        Description:In DTDs, the '%' character must be part of a parameterentity reference.
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/".to_string());

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/008.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfnotsa009() {
    /*
        Test ID:not-wf-not-sa-009
        Test URI:not-wf/not-sa/009.xml
        Spec Sections:2.8
        Description:This test violates WFC:PE Between Declarations in Production 28a.The last character of a markup declaration is not contained in the sameparameter-entity text replacement.
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/".to_string());

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/009.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}
