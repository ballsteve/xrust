/*
Richard Tobin's XML Namespaces 1.0 test suite 14 Feb 2003
 */

use std::convert::TryFrom;
use std::fs;
use xrust::Document;

#[test]
fn rmtns10001() {
    /*
        Test ID:rmt-ns10-001
        Test URI:001.xml
        Spec Sections:2
        Description:Namespace name test: a perfectly good http URI
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/001.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn rmtns10002() {
    /*
        Test ID:rmt-ns10-002
        Test URI:002.xml
        Spec Sections:2
        Description:Namespace name test: a syntactically plausible URI with a fictitious scheme
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/002.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn rmtns10003() {
    /*
        Test ID:rmt-ns10-003
        Test URI:003.xml
        Spec Sections:2
        Description:Namespace name test: a perfectly good http URI with a fragment
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/003.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn rmtns10007() {
    /*
        Test ID:rmt-ns10-007
        Test URI:007.xml
        Spec Sections:1
        Description:Namespace inequality test: different capitalization
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/007.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn rmtns10008() {
    /*
        Test ID:rmt-ns10-008
        Test URI:008.xml
        Spec Sections:1
        Description:Namespace inequality test: different escaping
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/008.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn htns10047() {
    /*
        Test ID:ht-ns10-047
        Test URI:047.xml
        Spec Sections:NE03
        Description:Reserved name: _not_ an error
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/047.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn htns10048() {
    /*
        Test ID:ht-ns10-048
        Test URI:048.xml
        Spec Sections:NE03
        Description:Reserved name: _not_ an error
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/048.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}
