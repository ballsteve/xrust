/*

Sun Microsystems test cases

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
fn test_sun_error(xmldoc: &str) {
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
fn uri01() {
    /*
        Test ID:uri01
        Test URI:not-wf/uri01.xml
        Spec Sections:4.2.2 [75]
        Description:        SYSTEM ids may not have URI fragments
    */

    test_sun_error(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/uri01.xml")
            .unwrap()
            .as_str(),
    )
}
