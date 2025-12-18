/*

Bjoern Hoehrmann via HST 2013-09-18

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
use xrust::validators::Schema;

#[cfg(all(test, feature = "test-conformance-xml"))]
fn test_eduni_misc_invalid(xmldoc: &str) {
    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        xmldoc,
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_err());
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn hstbh005() {
    /*
        Test ID:hst-bh-005
        Test URI:005.xml
        Spec Sections:3.1 [41]
        Description:xmlns:xml is an attribute as far as validation is concerned and must be declared
    */
    test_eduni_misc_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/misc/005.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn hstbh006() {
    /*
        Test ID:hst-bh-006
        Test URI:006.xml
        Spec Sections:3.1 [41]
        Description:xmlns:foo is an attribute as far as validation is concerned and must be declared
    */
    test_eduni_misc_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/misc/006.xml")
            .unwrap()
            .as_str(),
    );
}
