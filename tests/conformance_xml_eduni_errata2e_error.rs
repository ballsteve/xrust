/*

Richard Tobin's XML 1.0 2nd edition errata test suite.

*/

#[cfg(all(test, feature = "test-conformance-xml"))]
use std::fs;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::item::Node;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::parser::ParseError;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::parser::xml;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::trees::smite::RNode;

#[cfg(all(test, feature = "test-conformance-xml"))]
fn test_eduni_errata2e_error(xmldoc: &str) {
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
fn rmte2e34() {
    /*
        Test ID:rmt-e2e-34
        Test URI:E34.xml
        Spec Sections:E34
        Description:A non-deterministic content model is an error even if the element type is not used.
    */
    test_eduni_errata2e_error(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E34.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmte2e55() {
    /*
        Test ID:rmt-e2e-55
        Test URI:E55.xml
        Spec Sections:E55
        Description:A reference to an unparsed entity in an entity value is an error rather than forbidden (unless the entity is referenced, of course)
    */
    test_eduni_errata2e_error(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E55.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmte2e57() {
    /*
        Test ID:rmt-e2e-57
        Test URI:E57.xml
        Spec Sections:E57
        Description:A value other than preserve or default for xml:space is an error
    */
    test_eduni_errata2e_error(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E57.xml")
            .unwrap()
            .as_str(),
    );
}
