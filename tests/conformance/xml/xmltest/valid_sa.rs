/*

James Clark XMLTEST cases - Standalone

Note: The below are tests using from the XML test suite, with the entities and attlists removed. If in
      future entities, DTDs and the like are supported we should build those out.

*/

use std::fs;
use xrust::parsexml;


#[test]
fn validsa001() {
    /*
        Test ID:valid-sa-001
        Test URI:valid/sa/001.xml
        Spec Sections:3.2.2 [51]
        Description:Test demonstrates an Element Type Declaration with Mixed Content.
    */

    let testxml = parsexml::parse(&fs::read_to_string("tests/conformance/xml/xmltest/valid/sa/001.xml").unwrap());
    let canonicalxml = parsexml::parse(&fs::read_to_string("tests/conformance/xml/xmltest/valid/sa/out/001.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

