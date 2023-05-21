/*
Richard Tobin's XML 1.0 2nd edition errata test suite.
*/

use std::convert::TryFrom;
use std::fs;
use xrust::Document;

#[test]
#[ignore] //Need to build DTD validation
fn rmte2e2a() {
    /*
        Test ID:rmt-e2e-2a
        Test URI:E2a.xml
        Spec Sections:E2
        Description:Duplicate token in enumerated attribute declaration
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E2a.xml").unwrap(),
        None
    ));
    assert!(testxml.is_err());
}

#[test]
#[ignore] //Need to build DTD validation
fn rmte2e2b() {
    /*
        Test ID:rmt-e2e-2b
        Test URI:E2b.xml
        Spec Sections:E2
        Description:Duplicate token in NOTATION attribute declaration
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E2b.xml").unwrap(),
        None
    ));
    assert!(testxml.is_err());
}

#[test]
fn rmte2e9b() {
    /*
        Test ID:rmt-e2e-9b
        Test URI:E9b.xml
        Spec Sections:E9
        Description:An attribute default must be syntactically correct even if unused
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E9b.xml").unwrap(),
        None
    ));
    assert!(testxml.is_err());
}

#[test]
#[ignore] //No external DTD support yet.
fn rmte2e14() {
    /*
        Test ID:rmt-e2e-14
        Test URI:E14.xml
        Spec Sections:E14
        Description:Declarations mis-nested wrt parameter entities are just validity errors (but note that some parsers treat some such errors as fatal)
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E14.xml").unwrap(),
        None
    ));
    assert!(testxml.is_err());
}

#[test]
#[ignore] //no DTD validation
fn rmte2e15a() {
    /*
        Test ID:rmt-e2e-15a
        Test URI:E15a.xml
        Spec Sections:E15
        Description:Empty content can't contain an entity reference
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E15a.xml").unwrap(),
        None
    ));
    assert!(testxml.is_err());
}

#[test]
#[ignore] //No DTD validation
fn rmte2e15b() {
    /*
        Test ID:rmt-e2e-15b
        Test URI:E15b.xml
        Spec Sections:E15
        Description:Empty content can't contain a comment
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E15b.xml").unwrap(),
        None
    ));
    assert!(testxml.is_err());
}

#[test]
#[ignore] //No DTD validation
fn rmte2e15c() {
    /*
        Test ID:rmt-e2e-15c
        Test URI:E15c.xml
        Spec Sections:E15
        Description:Empty content can't contain a PI
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E15c.xml").unwrap(),
        None
    ));
    assert!(testxml.is_err());
}

#[test]
#[ignore] //No DTD validation
fn rmte2e15d() {
    /*
        Test ID:rmt-e2e-15d
        Test URI:E15d.xml
        Spec Sections:E15
        Description:Empty content can't contain whitespace
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E15d.xml").unwrap(),
        None
    ));
    assert!(testxml.is_err());
}

#[test]
#[ignore] //No DTD validation
fn rmte2e15g() {
    /*
        Test ID:rmt-e2e-15g
        Test URI:E15g.xml
        Spec Sections:E15
        Description:Element content can't contain character reference to whitespace
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E15g.xml").unwrap(),
        None
    ));
    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn rmte2e15h() {
    /*
        Test ID:rmt-e2e-15h
        Test URI:E15h.xml
        Spec Sections:E15
        Description:Element content can't contain entity reference if replacement text is character reference to whitespace
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E15h.xml").unwrap(),
        None
    ));
    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn rmte2e20() {
    /*
        Test ID:rmt-e2e-20
        Test URI:E20.xml
        Spec Sections:E20
        Description:Tokens, after normalization, must be separated by space, not other whitespace characters
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E20.xml").unwrap(),
        None
    ));
    assert!(testxml.is_err());
}
