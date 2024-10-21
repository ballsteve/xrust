/*

IBM test cases

*/

use std::fs;
use xrust::item::Node;
use xrust::parser::xml;
use xrust::trees::smite::RNode;

#[test]
#[ignore]
fn ibm11valid_p46ibm46i01xml() {
    /*
        Test ID:ibm-1-1-valid-P46-ibm46i01.xml
        Test URI:invalid/P46/ibm46i01.xml
        Spec Sections:3.2.1, 2.2
        Description:An element with Element-Only content contains the character #x85 (NEL not a whitespace character as defined by S).
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/invalid/P46/ibm46i01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn ibm11valid_p46ibm46i02xml() {
    /*
        Test ID:ibm-1-1-valid-P46-ibm46i02.xml
        Test URI:invalid/P46/ibm46i02.xml
        Spec Sections:3.2.1, 2.2
        Description:An element with Element-Only content contains the character #x2028 (LESP not a whitespace character as defined by S).
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/invalid/P46/ibm46i02.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}
