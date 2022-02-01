/*

James Clark XMLTEST cases

    This contains cases that are well-formed XML documents but are not valid XML documents

*/

use std::fs;
use xrust::parsexml;


#[test]
#[ignore]
fn invalid002() {
    /*
        Test ID:invalid--002
        Test URI:invalid/002.xml
        Spec Sections:3.2.1
        Description:Tests the "Proper Group/PE Nesting" validity constraint by fragmenting a content model between two parameter entities.
    */

    let testxml = parsexml::parse(&fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/invalid/002.xml").unwrap());

    assert!(testxml.is_err());

}

#[test]
#[ignore]
fn invalid005() {
    /*
        Test ID:invalid--005
        Test URI:invalid/005.xml
        Spec Sections:2.8
        Description:Tests the "Proper Declaration/PE Nesting" validity constraint by fragmenting an element declaration between two parameter entities.
    */

    let testxml = parsexml::parse(&fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/invalid/005.xml").unwrap());

    assert!(testxml.is_err());

}

#[test]
#[ignore]
fn invalid006() {
    /*
        Test ID:invalid--006
        Test URI:invalid/006.xml
        Spec Sections:2.8
        Description:Tests the "Proper Declaration/PE Nesting" validity constraint by fragmenting an element declaration between two parameter entities.
    */

    let testxml = parsexml::parse(&fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/invalid/006.xml").unwrap());

    assert!(testxml.is_err());

}

#[test]
#[ignore]
fn invalidnotsa022() {
    /*
        Test ID:invalid-not-sa-022
        Test URI:invalid/not-sa/022.xml
        Spec Sections:3.4 [62]
        Description:Test the "Proper Conditional Section/ PE Nesting" validity constraint.
    */

    let testxml = parsexml::parse(&fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/invalid/not-sa/022.xml").unwrap());

    assert!(testxml.is_err());

}