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
use xrust::validators::Schema;

#[cfg(all(test, feature = "test-conformance-xml"))]
fn test_eduni_namespaces_10_valid(xmldoc: &str) {
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
fn rmtns10001() {
    /*
        Test ID:rmt-ns10-001
        Test URI:001.xml
        Spec Sections:2
        Description:Namespace name test: a perfectly good http URI
    */
    test_eduni_namespaces_10_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/001.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmtns10002() {
    /*
        Test ID:rmt-ns10-002
        Test URI:002.xml
        Spec Sections:2
        Description:Namespace name test: a syntactically plausible URI with a fictitious scheme
    */
    test_eduni_namespaces_10_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/002.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmtns10003() {
    /*
        Test ID:rmt-ns10-003
        Test URI:003.xml
        Spec Sections:2
        Description:Namespace name test: a perfectly good http URI with a fragment
    */
    test_eduni_namespaces_10_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/002.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmtns10007() {
    /*
        Test ID:rmt-ns10-007
        Test URI:007.xml
        Spec Sections:1
        Description:Namespace inequality test: different capitalization
    */
    test_eduni_namespaces_10_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/007.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmtns10008() {
    /*
        Test ID:rmt-ns10-008
        Test URI:008.xml
        Spec Sections:1
        Description:Namespace inequality test: different escaping
    */
    test_eduni_namespaces_10_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/008.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn htns10047() {
    /*
        Test ID:ht-ns10-047
        Test URI:047.xml
        Spec Sections:NE03
        Description:Reserved name: _not_ an error
    */
    test_eduni_namespaces_10_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/047.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn htns10048() {
    /*
        Test ID:ht-ns10-048
        Test URI:048.xml
        Spec Sections:NE03
        Description:Reserved name: _not_ an error
    */
    test_eduni_namespaces_10_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/048.xml")
            .unwrap()
            .as_str(),
    );
}
