/*

Richard Tobin's XML Namespaces 1.0 test suite 14 Feb 2003

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
fn test_eduni_namespaces_10_error(xmldoc: &str) {
    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        xmldoc,
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmtns10004() {
    /*
        Test ID:rmt-ns10-004
        Test URI:004.xml
        Spec Sections:2
        Description:Namespace name test: a relative URI (deprecated)
    */
    test_eduni_namespaces_10_error(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/004.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmtns10005() {
    /*
        Test ID:rmt-ns10-005
        Test URI:005.xml
        Spec Sections:2
        Description:Namespace name test: a same-document relative URI (deprecated)
    */
    test_eduni_namespaces_10_error(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/005.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmtns10006() {
    /*
        Test ID:rmt-ns10-006
        Test URI:006.xml
        Spec Sections:2
        Description:Namespace name test: an http IRI that is not a URI
    */
    test_eduni_namespaces_10_error(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/006.xml")
            .unwrap()
            .as_str(),
    );
}
