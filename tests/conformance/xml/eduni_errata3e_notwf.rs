/*
Richard Tobin's XML 1.0 3rd edition errata test suite 1 June 2006
 */

use std::convert::TryFrom;
use std::fs;
use xrust::Document;

#[test]
fn rmte3e12() {
    /*
        Test ID:rmt-e3e-12
        Test URI:E12.xml
        Spec Sections:E12
        Description:Default values for attributes may not contain references to external entities.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-3e/E12.xml").unwrap(),
        None,
        None,
    ));
    assert!(testxml.is_err());
}
