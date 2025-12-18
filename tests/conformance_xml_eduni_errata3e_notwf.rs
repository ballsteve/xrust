/*
Richard Tobin's XML 1.0 3rd edition errata test suite 1 June 2006
 */

#[cfg(all(test, feature = "test-conformance-xml"))]
use std::fs;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::item::Node;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::parser::{ParseError, xml};
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::trees::smite::RNode;

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmte3e12() {
    /*
        Test ID:rmt-e3e-12
        Test URI:E12.xml
        Spec Sections:E12
        Description:Default values for attributes may not contain references to external entities.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-3e/E12.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_err());
}
