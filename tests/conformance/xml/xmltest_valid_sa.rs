/*

James Clark XMLTEST cases - Standalone

    This contains cases that are valid XML documents.
    This contains cases that are standalone (as defined in XML)
        and do not have references to external general entities.

*/

use crate::conformance::{dtdfileresolve, non_utf8_file_reader};
use std::fs;
use xrust::item::Node;
use xrust::parser::{ParseError, ParserStateBuilder, StaticStateBuilder, xml};
use xrust::trees::smite::RNode;
use xrust::validators::Schema;

#[test]
fn validsa001() {
    /*
        Test ID:valid-sa-001
        Test URI:valid/sa/001.xml
        Spec Sections:3.2.2 [51]
        Description:Test demonstrates an Element Type Declaration with Mixed Content.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/001.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/001.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa002() {
    /*
        Test ID:valid-sa-002
        Test URI:valid/sa/002.xml
        Spec Sections:3.1 [40]
        Description:Test demonstrates that whitespace is permitted after the tag name in a Start-tag.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/002.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/002.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa003() {
    /*
        Test ID:valid-sa-003
        Test URI:valid/sa/003.xml
        Spec Sections:3.1 [42]
        Description:Test demonstrates that whitespace is permitted after the tag name in an End-tag.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/003.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/003.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa004() {
    /*
        Test ID:valid-sa-004
        Test URI:valid/sa/004.xml
        Spec Sections:3.1 [41]
        Description:Test demonstrates a valid attribute specification within a Start-tag.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/004.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/004.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa005() {
    /*
        Test ID:valid-sa-005
        Test URI:valid/sa/005.xml
        Spec Sections:3.1 [40]
        Description:Test demonstrates a valid attribute specification within a Start-tag thatcontains whitespace on both sides of the equal sign.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/005.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/005.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa006() {
    /*
        Test ID:valid-sa-006
        Test URI:valid/sa/006.xml
        Spec Sections:3.1 [41]
        Description:Test demonstrates that the AttValue within a Start-tag can use a single quote as a delimter.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/006.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/006.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa007() {
    /*
        Test ID:valid-sa-007
        Test URI:valid/sa/007.xml
        Spec Sections:3.1 4.6 [43]
        Description:Test demonstrates numeric character references can be used for element content.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/007.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/007.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn validsa008() {
    /*
        Test ID:valid-sa-008
        Test URI:valid/sa/008.xml
        Spec Sections:2.4 3.1 [43]
        Description:Test demonstrates character references can be used for element content.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/008.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/008.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa009() {
    /*
        Test ID:valid-sa-009
        Test URI:valid/sa/009.xml
        Spec Sections:2.3 3.1 [43]
        Description:Test demonstrates that PubidChar can be used for element content.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/009.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/009.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa010() {
    /*
        Test ID:valid-sa-010
        Test URI:valid/sa/010.xml
        Spec Sections:3.1 [40]
        Description:Test demonstrates that whitespace is valid after the Attribute in a Start-tag.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/010.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/010.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa011() {
    /*
        Test ID:valid-sa-011
        Test URI:valid/sa/011.xml
        Spec Sections:3.1 [40]
        Description:Test demonstrates mutliple Attibutes within the Start-tag.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/011.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/011.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

/*
#[test]
#[ignore]
fn validsa012() {

    /*
        This test is deliberately ignored. Although these are valid XML documents,
        XML without namespaces is not something we wish to handle.
    */

    /*
        Test ID:valid-sa-012
        Test URI:valid/sa/012.xml
        Spec Sections:2.3 [4]
        Description:Uses a legal XML 1.0 name consisting of a single colon character (disallowed by the latest XML Namespaces draft).
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/012.xml").unwrap(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),Some(|_: &_| Err(ParseError::MissingNameSpace))
    ));
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/012.xml").unwrap(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),Some(|_: &_| Err(ParseError::MissingNameSpace))
    ));

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}
 */

#[test]
fn validsa013() {
    /*
        Test ID:valid-sa-013
        Test URI:valid/sa/013.xml
        Spec Sections:2.3 3.1 [13] [40]
        Description:Test demonstrates that the Attribute in a Start-tag can consist of numerals along with special characters.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/013.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/013.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa014() {
    /*
        Test ID:valid-sa-014
        Test URI:valid/sa/014.xml
        Spec Sections:2.3 3.1 [13] [40]
        Description:Test demonstrates that all lower case letters are valid for the Attribute in a Start-tag.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/014.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/014.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa015() {
    /*
        Test ID:valid-sa-015
        Test URI:valid/sa/015.xml
        Spec Sections:2.3 3.1 [13] [40]
        Description:Test demonstrates that all upper case letters are valid for the Attribute in a Start-tag.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/015.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/015.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa016() {
    /*
        Test ID:valid-sa-016
        Test URI:valid/sa/016.xml
        Spec Sections:2.6 3.1 [16] [43]
        Description:Test demonstrates that Processing Instructions are valid element content.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/016.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/016.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa017() {
    /*
        Test ID:valid-sa-017
        Test URI:valid/sa/017.xml
        Spec Sections:2.6 3.1 [16] [43]
        Description:Test demonstrates that Processing Instructions are valid element content and there can be more than one.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/017.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/017.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn validsa018() {
    /*
        Test ID:valid-sa-018
        Test URI:valid/sa/018.xml
        Spec Sections:2.7 3.1 [18] [43]
        Description:Test demonstrates that CDATA sections are valid element content.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/018.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/018.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn validsa019() {
    /*
        Test ID:valid-sa-019
        Test URI:valid/sa/019.xml
        Spec Sections:2.7 3.1 [18] [43]
        Description:Test demonstrates that CDATA sections are valid element content and thatampersands may occur in their literal form.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/019.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/019.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn validsa020() {
    /*
        Test ID:valid-sa-020
        Test URI:valid/sa/020.xml
        Spec Sections:2.7 3.1 [18] [43]
        Description:Test demonstractes that CDATA sections are valid element content and thateveryting between the CDStart and CDEnd is recognized as character data not markup.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/020.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/020.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa021() {
    /*
        Test ID:valid-sa-021
        Test URI:valid/sa/021.xml
        Spec Sections:2.5 3.1 [15] [43]
        Description:Test demonstrates that comments are valid element content.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/021.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/021.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa022() {
    /*
        Test ID:valid-sa-022
        Test URI:valid/sa/022.xml
        Spec Sections:2.5 3.1 [15] [43]
        Description:Test demonstrates that comments are valid element content and that all characters before the double-hypen right angle combination are considered part of thecomment.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/022.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/022.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa023() {
    /*
        Test ID:valid-sa-023
        Test URI:valid/sa/023.xml
        Spec Sections:3.1 [43]
        Description:Test demonstrates that Entity References are valid element content.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/023.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/023.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa024() {
    /*
        Test ID:valid-sa-024
        Test URI:valid/sa/024.xml
        Spec Sections:3.1 4.1 [43] [66]
        Description:Test demonstrates that Entity References are valid element content and also demonstrates a valid Entity Declaration.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/024.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/024.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa025() {
    /*
        Test ID:valid-sa-025
        Test URI:valid/sa/025.xml
        Spec Sections:3.2 [46]
        Description:Test demonstrates an Element Type Declaration and that the contentspec can be of mixed content.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/025.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/025.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa026() {
    /*
        Test ID:valid-sa-026
        Test URI:valid/sa/026.xml
        Spec Sections:3.2 [46]
        Description:Test demonstrates an Element Type Declaration and that EMPTY is a valid contentspec.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/026.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/026.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa027() {
    /*
        Test ID:valid-sa-027
        Test URI:valid/sa/027.xml
        Spec Sections:3.2 [46]
        Description:Test demonstrates an Element Type Declaration and that ANY is a valid contenspec.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/027.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/027.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa028() {
    /*
        Test ID:valid-sa-028
        Test URI:valid/sa/028.xml
        Spec Sections:2.8 [24]
        Description:Test demonstrates a valid prolog that uses double quotes as delimeters around the VersionNum.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/028.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/028.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa029() {
    /*
        Test ID:valid-sa-029
        Test URI:valid/sa/029.xml
        Spec Sections:2.8 [24]
        Description:Test demonstrates a valid prolog that uses single quotes as delimters around the VersionNum.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/029.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/029.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa030() {
    /*
        Test ID:valid-sa-030
        Test URI:valid/sa/030.xml
        Spec Sections:2.8 [25]
        Description:Test demonstrates a valid prolog that contains whitespace on both sides of the equal sign in the VersionInfo.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/030.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/030.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa031() {
    /*
        Test ID:valid-sa-031
        Test URI:valid/sa/031.xml
        Spec Sections:4.3.3 [80]
        Description:Test demonstrates a valid EncodingDecl within the prolog.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/031.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/031.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa032() {
    /*
        Test ID:valid-sa-032
        Test URI:valid/sa/032.xml
        Spec Sections:2.9 [32]
        Description:Test demonstrates a valid SDDecl within the prolog.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/032.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/032.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa033() {
    /*
        Test ID:valid-sa-033
        Test URI:valid/sa/033.xml
        Spec Sections:2.8 [23]
        Description:Test demonstrates that both a EncodingDecl and SDDecl are valid within the prolog.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/033.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/033.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    //assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa034() {
    /*
        Test ID:valid-sa-034
        Test URI:valid/sa/034.xml
        Spec Sections:3.1 [44]
        Description:Test demonstrates the correct syntax for an Empty element tag.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/034.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/034.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa035() {
    /*
        Test ID:valid-sa-035
        Test URI:valid/sa/035.xml
        Spec Sections:3.1 [44]
        Description:Test demonstrates that whitespace is permissible after the name in an Empty element tag.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/035.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/035.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa036() {
    /*
        Test ID:valid-sa-036
        Test URI:valid/sa/036.xml
        Spec Sections:2.6 [16]
        Description:Test demonstrates a valid processing instruction.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/036.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/036.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn validsa017a() {
    /*
        Test ID:valid-sa-017a
        Test URI:valid/sa/017a.xml
        Spec Sections:2.6 3.1 [16] [43]
        Description:Test demonstrates that two apparently wrong Processing Instructions make aright one, with very odd content "some data ? > <?".
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/017a.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/017a.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa037() {
    /*
        Test ID:valid-sa-037
        Test URI:valid/sa/037.xml
        Spec Sections:2.6 [15]
        Description:Test demonstrates a valid comment and that it may appear anywhere in the document including at the end.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/037.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/037.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa038() {
    /*
        Test ID:valid-sa-038
        Test URI:valid/sa/038.xml
        Spec Sections:2.6 [15]
        Description:Test demonstrates a valid comment and that it may appear anywhere in the document including the beginning.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/038.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/038.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa039() {
    /*
        Test ID:valid-sa-039
        Test URI:valid/sa/039.xml
        Spec Sections:2.6 [16]
        Description:Test demonstrates a valid processing instruction and that it may appear at the beginning of the document.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/039.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/039.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa040() {
    /*
        Test ID:valid-sa-040
        Test URI:valid/sa/040.xml
        Spec Sections:3.3 3.3.1 [52] [54]
        Description:Test demonstrates an Attribute List declaration that uses a StringType as the AttType.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/040.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/040.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa041() {
    /*
        Test ID:valid-sa-041
        Test URI:valid/sa/041.xml
        Spec Sections:3.3.1 4.1 [54] [66]
        Description:Test demonstrates an Attribute List declaration that uses a StringType as the AttType and also expands the CDATA attribute with a character reference.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/041.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/041.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa042() {
    /*
        Test ID:valid-sa-042
        Test URI:valid/sa/042.xml
        Spec Sections:3.3.1 4.1 [54] [66]
        Description:Test demonstrates an Attribute List declaration that uses a StringType as the AttType and also expands the CDATA attribute with a character reference. The test also shows that the leading zeros in the character reference are ignored.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/042.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/042.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa043() {
    /*
        Test ID:valid-sa-043
        Test URI:valid/sa/043.xml
        Spec Sections:3.3
        Description:An element's attributes may be declared before its content model; and attribute values may contain newlines.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/043.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/043.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa044() {
    /*
        Test ID:valid-sa-044
        Test URI:valid/sa/044.xml
        Spec Sections:3.1 [44]
        Description:Test demonstrates that the empty-element tag must be use for an elements that are declared EMPTY.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/044.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/044.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa045() {
    /*
        Test ID:valid-sa-045
        Test URI:valid/sa/045.xml
        Spec Sections:3.3 [52]
        Description:Tests whether more than one definition can be provided for the same attribute of a given element type with the first declaration being binding.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/045.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/045.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa046() {
    /*
        Test ID:valid-sa-046
        Test URI:valid/sa/046.xml
        Spec Sections:3.3 [52]
        Description:Test demonstrates that when more than one AttlistDecl is provided for a given element type, the contents of all those provided are merged.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/046.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/046.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa047() {
    /*
        Test ID:valid-sa-047
        Test URI:valid/sa/047.xml
        Spec Sections:3.1 [43]
        Description:Test demonstrates that extra whitespace is normalized into single space character.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/047.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/047.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa048() {
    /*
        Test ID:valid-sa-048
        Test URI:valid/sa/048.xml
        Spec Sections:2.4 3.1 [14] [43]
        Description:Test demonstrates that character data is valid element content.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/048.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/048.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa049() {
    /*
        Test ID:valid-sa-049
        Test URI:valid/sa/049.xml
        Spec Sections:2.2 [2]
        Description:Test demonstrates that characters outside of normal ascii range can be used as element content.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        non_utf8_file_reader("tests/conformance/xml/xmlconf/xmltest/valid/sa/049.xml").as_str(),
        //fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/049.xml").unwrap().as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/049.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa050() {
    /*
        Test ID:valid-sa-050
        Test URI:valid/sa/050.xml
        Spec Sections:2.2 [2]
        Description:Test demonstrates that characters outside of normal ascii range can be used as element content.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        non_utf8_file_reader("tests/conformance/xml/xmlconf/xmltest/valid/sa/050.xml").as_str(),
        //fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/050.xml").unwrap().as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/050.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa051() {
    /*
        Test ID:valid-sa-051
        Test URI:valid/sa/051.xml
        Spec Sections:2.2 [2]
        Description:The document is encoded in UTF-16 and uses some name characters well outside of the normal ASCII range.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        non_utf8_file_reader("tests/conformance/xml/xmlconf/xmltest/valid/sa/051.xml").as_str(),
        //fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/051.xml").unwrap().as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/051.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa052() {
    /*
        Test ID:valid-sa-052
        Test URI:valid/sa/052.xml
        Spec Sections:2.2 [2]
        Description:The document is encoded in UTF-8 and the text inside the root element uses two non-ASCII characters, encoded in UTF-8 and each of which expands to a Unicode surrogate pair.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/052.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/052.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa053() {
    /*
        Test ID:valid-sa-053
        Test URI:valid/sa/053.xml
        Spec Sections:4.4.2
        Description:Tests inclusion of a well-formed internal entity, which holds an element required by the content model.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/053.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/053.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa054() {
    /*
        Test ID:valid-sa-054
        Test URI:valid/sa/054.xml
        Spec Sections:3.1 [40] [42]
        Description:Test demonstrates that extra whitespace within Start-tags and End-tags are nomalized into single spaces.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/054.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/054.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa055() {
    /*
        Test ID:valid-sa-055
        Test URI:valid/sa/055.xml
        Spec Sections:2.6 2.10 [16]
        Description:Test demonstrates that extra whitespace within a processing instruction willnormalized into s single space character.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/055.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/055.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa056() {
    /*
        Test ID:valid-sa-056
        Test URI:valid/sa/056.xml
        Spec Sections:3.3.1 4.1 [54] [66]
        Description:Test demonstrates an Attribute List declaration that uses a StringType as the AttType and also expands the CDATA attribute with a character reference. The test also shows that the leading zeros in the character reference are ignored.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/056.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/056.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa057() {
    /*
        Test ID:valid-sa-057
        Test URI:valid/sa/057.xml
        Spec Sections:3.2.1 [47]
        Description:Test demonstrates an element content model whose element can occur zero or more times.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/057.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/057.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa058() {
    /*
        Test ID:valid-sa-058
        Test URI:valid/sa/058.xml
        Spec Sections:3.3.3
        Description:Test demonstrates that extra whitespace be normalized into a single space character in an attribute of type NMTOKENS.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/058.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/058.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa059() {
    /*
        Test ID:valid-sa-059
        Test URI:valid/sa/059.xml
        Spec Sections:3.2 3.3 [46] [53]
        Description:Test demonstrates an Element Type Declaration that uses the contentspec of EMPTY. The element cannot have any contents and must always appear as an empty element in the document. The test also shows an Attribute-list declaration with multiple AttDef's.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/059.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/059.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa060() {
    /*
        Test ID:valid-sa-060
        Test URI:valid/sa/060.xml
        Spec Sections:4.1 [66]
        Description:Test demonstrates the use of decimal Character References within element content.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/060.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/060.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa061() {
    /*
        Test ID:valid-sa-061
        Test URI:valid/sa/061.xml
        Spec Sections:4.1 [66]
        Description:Test demonstrates the use of decimal Character References within element content.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/061.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/061.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa062() {
    /*
        Test ID:valid-sa-062
        Test URI:valid/sa/062.xml
        Spec Sections:4.1 [66]
        Description:Test demonstrates the use of hexadecimal Character References within element.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/062.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/062.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa063() {
    /*
        Test ID:valid-sa-063
        Test URI:valid/sa/063.xml
        Spec Sections:2.3 [5]
        Description:The document is encoded in UTF-8 and the name of the root element type uses non-ASCII characters.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/063.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/063.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa064() {
    /*
        Test ID:valid-sa-064
        Test URI:valid/sa/064.xml
        Spec Sections:4.1 [66]
        Description:Tests in-line handling of two legal character references, which each expand to a Unicode surrogate pair.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/064.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/064.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa065() {
    /*
        Test ID:valid-sa-065
        Test URI:valid/sa/065.xml
        Spec Sections:4.5
        Description:Tests ability to define an internal entity which can't legally be expanded (contains an unquoted <).
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/065.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/065.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa066() {
    /*
        Test ID:valid-sa-066
        Test URI:valid/sa/066.xml
        Spec Sections:4.1 [66]
        Description:Expands a CDATA attribute with a character reference.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/066.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/066.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn validsa067() {
    /*
        Test ID:valid-sa-067
        Test URI:valid/sa/067.xml
        Spec Sections:4.1 [66]
        Description:Test demonstrates the use of decimal character references within element content.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/067.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/067.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn validsa068() {
    /*
        Test ID:valid-sa-068
        Test URI:valid/sa/068.xml
        Spec Sections:2.11, 4.5
        Description:Tests definition of an internal entity holding a carriage return character reference, which must not be normalized before reporting to the application. Line break normalization only occurs when parsing external parsed entities.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/068.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/068.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa069() {
    /*
        Test ID:valid-sa-069
        Test URI:valid/sa/069.xml
        Spec Sections:4.7
        Description:Verifies that an XML parser will parse a NOTATION declaration; the output phase of this test ensures that it's reported to the application.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/069.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/069.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn validsa070() {
    /*
        Test ID:valid-sa-070
        Test URI:valid/sa/070.xml
        Spec Sections:4.4.8
        Description:Verifies that internal parameter entities are correctly expanded within the internal subset.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/070.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/070.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa071() {
    /*
        Test ID:valid-sa-071
        Test URI:valid/sa/071.xml
        Spec Sections:3.3 3.3.1 [52] [56]
        Description:Test demonstrates that an AttlistDecl can use ID as the TokenizedType within the Attribute type. The test also shows that IMPLIED is a valid DefaultDecl.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/071.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/071.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa072() {
    /*
        Test ID:valid-sa-072
        Test URI:valid/sa/072.xml
        Spec Sections:3.3 3.3.1 [52] [56]
        Description:Test demonstrates that an AttlistDecl can use IDREF as the TokenizedType within the Attribute type. The test also shows that IMPLIED is a valid DefaultDecl.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/072.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/072.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa073() {
    /*
        Test ID:valid-sa-073
        Test URI:valid/sa/073.xml
        Spec Sections:3.3 3.3.1 [52] [56]
        Description:Test demonstrates that an AttlistDecl can use IDREFS as the TokenizedType within the Attribute type. The test also shows that IMPLIED is a valid DefaultDecl.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/073.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/073.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa074() {
    /*
        Test ID:valid-sa-074
        Test URI:valid/sa/074.xml
        Spec Sections:3.3 3.3.1 [52] [56]
        Description:Test demonstrates that an AttlistDecl can use ENTITY as the TokenizedType within the Attribute type. The test also shows that IMPLIED is a valid DefaultDecl.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/074.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/074.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa075() {
    /*
        Test ID:valid-sa-075
        Test URI:valid/sa/075.xml
        Spec Sections:3.3 3.3.1 [52] [56]
        Description:Test demonstrates that an AttlistDecl can use ENTITIES as the TokenizedType within the Attribute type. The test also shows that IMPLIED is a valid DefaultDecl.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/075.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/075.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa076() {
    /*
        Test ID:valid-sa-076
        Test URI:valid/sa/076.xml
        Spec Sections:3.3.1
        Description:Verifies that an XML parser will parse a NOTATION attribute; the output phase of this test ensures that both notations are reported to the application.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/076.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/076.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa077() {
    /*
        Test ID:valid-sa-077
        Test URI:valid/sa/077.xml
        Spec Sections:3.3 3.3.1 [52] [54]
        Description:Test demonstrates that an AttlistDecl can use an EnumeratedType within the Attribute type. The test also shows that IMPLIED is a valid DefaultDecl.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/077.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/077.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa078() {
    /*
        Test ID:valid-sa-078
        Test URI:valid/sa/078.xml
        Spec Sections:3.3 3.3.1 [52] [54]
        Description:Test demonstrates that an AttlistDecl can use an StringType of CDATA within the Attribute type. The test also shows that REQUIRED is a valid DefaultDecl.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/078.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/078.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa079() {
    /*
        Test ID:valid-sa-079
        Test URI:valid/sa/079.xml
        Spec Sections:3.3 3.3.2 [52] [60]
        Description:Test demonstrates that an AttlistDecl can use an StringType of CDATA within the Attribute type. The test also shows that FIXED is a valid DefaultDecl and that a value can be given to the attribute in the Start-tag as well as the AttListDecl.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/079.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/079.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa080() {
    /*
        Test ID:valid-sa-080
        Test URI:valid/sa/080.xml
        Spec Sections:3.3 3.3.2 [52] [60]
        Description:Test demonstrates that an AttlistDecl can use an StringType of CDATA within the Attribute type. The test also shows that FIXED is a valid DefaultDecl and that an value can be given to the attribute.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/080.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/080.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa081() {
    /*
        Test ID:valid-sa-081
        Test URI:valid/sa/081.xml
        Spec Sections:3.2.1 [50]
        Description:Test demonstrates the use of the optional character following a name or list to govern the number of times an element or content particles in the list occur.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/081.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/081.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn validsa082() {
    /*
        Test ID:valid-sa-082
        Test URI:valid/sa/082.xml
        Spec Sections:4.2 [72]
        Description:Tests that an external PE may be defined (but not referenced).
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/082.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/082.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn validsa083() {
    /*
        Test ID:valid-sa-083
        Test URI:valid/sa/083.xml
        Spec Sections:4.2 [72]
        Description:Tests that an external PE may be defined (but not referenced).
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/083.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/083.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa084() {
    /*
        Test ID:valid-sa-084
        Test URI:valid/sa/084.xml
        Spec Sections:2.10
        Description:Test demonstrates that although whitespace can be used to set apart markup for greater readability it is not necessary.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/084.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/084.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa085() {
    /*
        Test ID:valid-sa-085
        Test URI:valid/sa/085.xml
        Spec Sections:4
        Description:Parameter and General entities use different namespaces, so there can be an entity of each type with a given name.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/085.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/085.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa086() {
    /*
        Test ID:valid-sa-086
        Test URI:valid/sa/086.xml
        Spec Sections:4.2
        Description:Tests whether entities may be declared more than once, with the first declaration being the binding one.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/086.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/086.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa087() {
    /*
        Test ID:valid-sa-087
        Test URI:valid/sa/087.xml
        Spec Sections:4.5
        Description:Tests whether character references in internal entities are expanded early enough, by relying on correct handling to make the entity be well formed.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/087.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/087.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn validsa088() {
    /*
        Test ID:valid-sa-088
        Test URI:valid/sa/088.xml
        Spec Sections:4.5
        Description:Tests whether entity references in internal entities are expanded late enough, by relying on correct handling to make the expanded text be valid. (If it's expanded too early, the entity will parse as an element that's not valid in that context.)
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/088.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/088.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa089() {
    /*
        Test ID:valid-sa-089
        Test URI:valid/sa/089.xml
        Spec Sections:4.1 [66]
        Description:Tests entity expansion of three legal character references, which each expand to a Unicode surrogate pair.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/089.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/089.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa090() {
    /*
        Test ID:valid-sa-090
        Test URI:valid/sa/090.xml
        Spec Sections:3.3.1
        Description:Verifies that an XML parser will parse a NOTATION attribute; the output phase of this test ensures that the notation is reported to the application.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/090.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/090.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn validsa091() {
    /*
        Test ID:valid-sa-091
        Test URI:valid/sa/091.xml
        Spec Sections:3.3.1
        Description:Verifies that an XML parser will parse an ENTITY attribute; the output phase of this test ensures that the notation is reported to the application, and for validating parsers it further tests that the entity is so reported.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/091.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/091.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa092() {
    /*
        Test ID:valid-sa-092
        Test URI:valid/sa/092.xml
        Spec Sections:2.3 2.10
        Description:Test demostrates that extra whitespace is normalized into a single space character.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/092.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/092.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa093() {
    /*
        Test ID:valid-sa-093
        Test URI:valid/sa/093.xml
        Spec Sections:2.10
        Description:Test demonstrates that extra whitespace is not intended for inclusion in the delivered version of the document.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/093.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/093.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa094() {
    /*
        Test ID:valid-sa-094
        Test URI:valid/sa/094.xml
        Spec Sections:2.8
        Description:Attribute defaults with a DTD have special parsing rules, different from other strings. That means that characters found there may look like an undefined parameter entity reference "within a markup declaration", but they aren't ... so they can't be violating the PEs in Internal Subset WFC.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/094.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/094.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn validsa095() {
    /*
        Test ID:valid-sa-095
        Test URI:valid/sa/095.xml
        Spec Sections:3.3.3
        Description:Basically an output test, this requires extra whitespace to be normalized into a single space character in an attribute of type NMTOKENS.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/095.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/095.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa096() {
    /*
        Test ID:valid-sa-096
        Test URI:valid/sa/096.xml
        Spec Sections:3.3.3
        Description:Test demonstrates that extra whitespace is normalized into a single space character in an attribute of type NMTOKENS.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/096.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/096.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn validsa097() {
    /*
        Test ID:valid-sa-097
        Test URI:valid/sa/097.xml
        Spec Sections:3.3
        Description:Basically an output test, this tests whether an externally defined attribute declaration (with a default) takes proper precedence over a subsequent internal declaration.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/xmltest/valid/sa/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/097.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/097.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn validsa098() {
    /*
        Test ID:valid-sa-098
        Test URI:valid/sa/098.xml
        Spec Sections:2.6 2.10 [16]
        Description:Test demonstrates that extra whitespace within a processing instruction is converted into a single space character.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/098.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/098.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa099() {
    /*
        Test ID:valid-sa-099
        Test URI:valid/sa/099.xml
        Spec Sections:4.3.3 [81]
        Description:Test demonstrates the name of the encoding can be composed of lowercase characters.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/099.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/099.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn validsa100() {
    /*
        Test ID:valid-sa-100
        Test URI:valid/sa/100.xml
        Spec Sections:2.3 [12]
        Description:Makes sure that PUBLIC identifiers may have some strange characters. NOTE: The XML editors have said that the XML specification errata will specify that parameter entity expansion does not occur in PUBLIC identifiers, so that the '%' character will not flag a malformed parameter entity reference.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/xmltest/valid/sa/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/100.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/100.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa101() {
    /*
        Test ID:valid-sa-101
        Test URI:valid/sa/101.xml
        Spec Sections:4.5
        Description:This tests whether entity expansion is (incorrectly) done while processing entity declarations; if it is, the entity value literal will terminate prematurely.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/101.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/101.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa102() {
    /*
        Test ID:valid-sa-102
        Test URI:valid/sa/102.xml
        Spec Sections:3.3.3
        Description:Test demonstrates that a CDATA attribute can pass a double quote as its value.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/102.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/102.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn validsa103() {
    /*
        Test ID:valid-sa-103
        Test URI:valid/sa/103.xml
        Spec Sections:3.3.3
        Description:Test demonstrates that an attribute can pass a less than sign as its value.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/103.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/103.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa104() {
    /*
        Test ID:valid-sa-104
        Test URI:valid/sa/104.xml
        Spec Sections:3.1 [40]
        Description:Test demonstrates that extra whitespace within an Attribute of a Start-tag is normalized to a single space character.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/104.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/104.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa105() {
    /*
        Test ID:valid-sa-105
        Test URI:valid/sa/105.xml
        Spec Sections:3.3.3
        Description:Basically an output test, this requires a CDATA attribute with a tab character to be passed through as one space.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/105.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/105.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa106() {
    /*
        Test ID:valid-sa-106
        Test URI:valid/sa/106.xml
        Spec Sections:3.3.3
        Description:Basically an output test, this requires a CDATA attribute with a newline character to be passed through as one space.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/106.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/106.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa107() {
    /*
        Test ID:valid-sa-107
        Test URI:valid/sa/107.xml
        Spec Sections:3.3.3
        Description:Basically an output test, this requires a CDATA attribute with a return character to be passed through as one space.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/107.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/107.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa108() {
    /*
        Test ID:valid-sa-108
        Test URI:valid/sa/108.xml
        Spec Sections:2.11, 3.3.3
        Description:This tests normalization of end-of-line characters (CRLF) within entities to LF, primarily as an output test.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/108.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/108.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa109() {
    /*
        Test ID:valid-sa-109
        Test URI:valid/sa/109.xml
        Spec Sections:2.3 3.1 [10][40][41]
        Description:Test demonstrates that an attribute can have a null value.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/109.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/109.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn validsa110() {
    /*
        Test ID:valid-sa-110
        Test URI:valid/sa/110.xml
        Spec Sections:3.3.3
        Description:Basically an output test, this requires that a CDATA attribute with a CRLF be normalized to one space.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/110.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/110.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa111() {
    /*
        Test ID:valid-sa-111
        Test URI:valid/sa/111.xml
        Spec Sections:3.3.3
        Description:Character references expanding to spaces doesn't affect treatment of attributes.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/111.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/111.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa112() {
    /*
        Test ID:valid-sa-112
        Test URI:valid/sa/112.xml
        Spec Sections:3.2.1 [48][49]
        Description:Test demonstrates shows the use of content particles within the element content.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/112.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/112.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa113() {
    /*
        Test ID:valid-sa-113
        Test URI:valid/sa/113.xml
        Spec Sections:3.3 [52][53]
        Description:Test demonstrates that it is not an error to have attributes declared for an element not itself declared.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/113.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/113.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn validsa114() {
    /*
        Test ID:valid-sa-114
        Test URI:valid/sa/114.xml
        Spec Sections:2.7 [20]
        Description:Test demonstrates that all text within a valid CDATA section is considered text and not recognized as markup.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/114.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/114.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa115() {
    /*
        Test ID:valid-sa-115
        Test URI:valid/sa/115.xml
        Spec Sections:3.3.3
        Description:Test demonstrates that an entity reference is processed by recursively processing the replacement text of the entity.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/115.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/115.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa116() {
    /*
        Test ID:valid-sa-116
        Test URI:valid/sa/116.xml
        Spec Sections:2.11
        Description:Test demonstrates that a line break within CDATA will be normalized.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/116.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/116.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa117() {
    /*
        Test ID:valid-sa-117
        Test URI:valid/sa/117.xml
        Spec Sections:4.5
        Description:Test demonstrates that entity expansion is done while processing entity declarations.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/117.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/117.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa118() {
    /*
        Test ID:valid-sa-118
        Test URI:valid/sa/118.xml
        Spec Sections:4.5
        Description:Test demonstrates that entity expansion is done while processing entity declarations.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/118.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/118.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn validsa119() {
    /*
        Test ID:valid-sa-119
        Test URI:valid/sa/119.xml
        Spec Sections:2.5
        Description:Comments may contain any legal XML characters; only the string "--" is disallowed.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/119.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml.clone(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/sa/out/119.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}
