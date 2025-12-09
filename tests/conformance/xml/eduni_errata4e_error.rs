/*

University of Edinburgh XML 1.0 4th edition errata test suite.

*/

use std::fs;
use xrust::item::Node;
use xrust::parser::{ParseError, xml};
use xrust::trees::smite::RNode;

#[test]
fn invalidbo7() {
    /*
        Test ID:invalid-bo-7
        Test URI:inclbomboom_be.xml
        Spec Sections:4.3.3
        Description:A byte order mark and a backwards one in general entity cause an illegal char. error (big-endian)
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/inclbomboom_be.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_err());
}

#[test]
fn invalidbo8() {
    /*
        Test ID:invalid-bo-8
        Test URI:inclbomboom_le.xml
        Spec Sections:4.3.3
        Description:A byte order mark and a backwards one in general entity cause an illegal char. error (little-endian)
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/inclbomboom_le.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_err());
}

#[test]
fn invalidbo9() {
    /*
        Test ID:invalid-bo-9
        Test URI:incl8bomboom.xml
        Spec Sections:4.3.3
        Description:A byte order mark and a backwards one in general entity cause an illegal char. error (utf-8)
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/incl8bomboom.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_err());
}

/*
#[test]
#[ignore]
fn xrmt008() {
    /*
        This test is deliberately ignored.
        In 5th edition, any document number other than 1.1 is treated as a 1.0 document.
    */
    /*
        Test ID:x-rmt-008
        Test URI:008.xml
        Spec Sections:2.8 4.3.4
        Description:a document with version=1.7, illegal in XML 1.0 through 4th edition
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/008.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_err());
}
*/
