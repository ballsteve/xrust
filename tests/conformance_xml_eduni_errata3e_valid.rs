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
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::validators::Schema;

#[cfg(all(test, feature = "test-conformance-xml"))]
fn test_eduni_errata3e_valid(xmldoc: &str) {
    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        xmldoc,
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmte3e05a() {
    /*
        Test ID:rmt-e3e-05a
        Test URI:E05a.xml
        Spec Sections:E05
        Description:CDATA sections may occur in Mixed content.
    */
    test_eduni_errata3e_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-3e/E05a.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmte3e05b() {
    /*
        Test ID:rmt-e3e-05b
        Test URI:E05b.xml
        Spec Sections:E05
        Description:CDATA sections, comments and PIs may occur in ANY content.
    */
    test_eduni_errata3e_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-3e/E05b.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmte3e06i() {
    /*
        Test ID:rmt-e3e-06i
        Test URI:E06i.xml
        Spec Sections:E06
        Description:Non-syntactic validity errors in default attributes only happen if the attribute is in fact defaulted.
    */
    test_eduni_errata3e_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-3e/E06i.xml")
            .unwrap()
            .as_str(),
    );
}
