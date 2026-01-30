/*

James Clark XMLTEST cases

    This contains cases that are not well-formed XML documents

*/

#[cfg(all(test, feature = "test-conformance-xml"))]
use std::fs;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::item::Node;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::parser::{ParseError, xml};
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::trees::smite::RNode;

#[cfg(all(test, feature = "test-conformance-xml"))]
fn test_not_wf_ext_sa(xmldoc: &str) {
    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        xmldoc,
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn notwfextsa001() {
    /*
        Test ID:not-wf-ext-sa-001
        Test URI:not-wf/ext-sa/001.xml
        Spec Sections:4.1
        Description:Tests the No Recursion WFC by having an external general entity be self-recursive.
    */

    test_not_wf_ext_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/ext-sa/001.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn notwfextsa002() {
    /*
        Test ID:not-wf-ext-sa-002
        Test URI:not-wf/ext-sa/002.xml
        Spec Sections:4.3.1 4.3.2 [77, 78]
        Description:External entities have "text declarations", which do not permit the "standalone=..." attribute that's allowed in XML declarations.
    */

    test_not_wf_ext_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/ext-sa/002.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn notwfextsa003() {
    /*
        Test ID:not-wf-ext-sa-003
        Test URI:not-wf/ext-sa/003.xml
        Spec Sections:2.6 [17]
        Description:Only one text declaration is permitted; a second one looks like an illegal processing instruction (target names of "xml" in any case are not allowed).
    */

    test_not_wf_ext_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/ext-sa/003.xml")
            .unwrap()
            .as_str(),
    );
}
