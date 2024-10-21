/*

Richard Tobin's XML 1.0 2nd edition errata test suite.

*/

use std::fs;
use xrust::item::Node;
use xrust::parser::xml;
use xrust::trees::smite::RNode;

#[test]
#[ignore]
fn rmte2e34() {
    /*
        Test ID:rmt-e2e-34
        Test URI:E34.xml
        Spec Sections:E34
        Description:A non-deterministic content model is an error even if the element type is not used.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E34.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn rmte2e55() {
    /*
        Test ID:rmt-e2e-55
        Test URI:E55.xml
        Spec Sections:E55
        Description:A reference to an unparsed entity in an entity value is an error rather than forbidden (unless the entity is referenced, of course)
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E55.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn rmte2e57() {
    /*
        Test ID:rmt-e2e-57
        Test URI:E57.xml
        Spec Sections:E57
        Description:A value other than preserve or default for xml:space is an error
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E57.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}
