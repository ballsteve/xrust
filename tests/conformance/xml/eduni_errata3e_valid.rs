/*
Richard Tobin's XML 1.0 3rd edition errata test suite 1 June 2006
 */

use std::convert::TryFrom;
use std::fs;
use xrust::Document;

#[test]
fn rmte3e05a() {
    /*
        Test ID:rmt-e3e-05a
        Test URI:E05a.xml
        Spec Sections:E05
        Description:CDATA sections may occur in Mixed content.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-3e/E05a.xml").unwrap(),
        None,
        None,
    ));
    assert!(testxml.is_ok());
}

#[test]
fn rmte3e05b() {
    /*
        Test ID:rmt-e3e-05b
        Test URI:E05b.xml
        Spec Sections:E05
        Description:CDATA sections, comments and PIs may occur in ANY content.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-3e/E05b.xml").unwrap(),
        None,
        None,
    ));
    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn rmte3e06i() {
    /*
        Test ID:rmt-e3e-06i
        Test URI:E06i.xml
        Spec Sections:E06
        Description:Non-syntactic validity errors in default attributes only happen if the attribute is in fact defaulted.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-3e/E06i.xml").unwrap(),
        None,
        None,
    ));
    assert!(testxml.is_ok());
}
