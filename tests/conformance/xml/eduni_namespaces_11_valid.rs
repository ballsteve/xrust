/*
Richard Tobin's XML Namespaces 1.1 test suite 14 Feb 2003
 */

use std::convert::TryFrom;
use std::fs;
use xrust::Document;

#[test]
#[ignore]
fn rmtns11001() {
    /*
        Test ID:rmt-ns11-001
        Test URI:001.xml
        Spec Sections:2.1
        Description:Namespace name test: a perfectly good http IRI that is not a URI
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.1/001.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn rmtns11002() {
    /*
        Test ID:rmt-ns11-002
        Test URI:002.xml
        Spec Sections:2.3
        Description:Namespace inequality test: different escaping of non-ascii letter
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.1/002.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn rmtns11003() {
    /*
        Test ID:rmt-ns11-003
        Test URI:003.xml
        Spec Sections:6.1
        Description:1.1 style prefix unbinding
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.1/003.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn rmtns11004() {
    /*
        Test ID:rmt-ns11-004
        Test URI:004.xml
        Spec Sections:6.1
        Description:1.1 style prefix unbinding and rebinding
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.1/004.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn rmtns11006() {
    /*
        Test ID:rmt-ns11-006
        Test URI:006.xml
        Spec Sections:2.1
        Description:Test whether non-Latin-1 characters are accepted in IRIs, and whether they are correctly distinguished
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.1/006.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}
