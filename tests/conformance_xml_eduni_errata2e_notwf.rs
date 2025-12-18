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
fn test_eduni_errata2e_notwf(xmldoc: &str) {
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
fn rmte2e27() {
    /*
        Test ID:rmt-e2e-27
        Test URI:E27.xml
        Spec Sections:E27
        Description:Contains an irregular UTF-8 sequence (i.e. a surrogate pair)
    */
    test_eduni_errata2e_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E27.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmte2e38() {
    /*
        Test ID:rmt-e2e-38
        Test URI:E38.xml
        Spec Sections:E38
        Description:XML 1.0 document refers to 1.1 entity
    */
    test_eduni_errata2e_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E38.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmte2e61() {
    /*
        Test ID:rmt-e2e-61
        Test URI:E61.xml
        Spec Sections:E61
        Description:(From John Cowan) An encoding declaration in ASCII specifying an encoding that is not compatible with ASCII (so the document is not in its declared encoding). It should generate a fatal error.
    */
    test_eduni_errata2e_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E61.xml")
            .unwrap()
            .as_str(),
    );
}
