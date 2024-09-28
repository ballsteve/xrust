/*

James Clark XMLTEST cases

    This contains cases that are not well-formed XML documents

*/

use std::fs;
use xrust::parser::xml;
use xrust::item::Node;
use xrust::trees::smite::RNode;

#[test]
#[ignore]
fn notwfextsa001() {
    /*
        Test ID:not-wf-ext-sa-001
        Test URI:not-wf/ext-sa/001.xml
        Spec Sections:4.1
        Description:Tests the No Recursion WFC by having an external general entity be self-recursive.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/ext-sa/001.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn notwfextsa002() {
    /*
        Test ID:not-wf-ext-sa-002
        Test URI:not-wf/ext-sa/002.xml
        Spec Sections:4.3.1 4.3.2 [77, 78]
        Description:External entities have "text declarations", which do not permit the "standalone=..." attribute that's allowed in XML declarations.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/ext-sa/002.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn notwfextsa003() {
    /*
        Test ID:not-wf-ext-sa-003
        Test URI:not-wf/ext-sa/003.xml
        Spec Sections:2.6 [17]
        Description:Only one text declaration is permitted; a second one looks like an illegal processing instruction (target names of "xml" in any case are not allowed).
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/ext-sa/003.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}
