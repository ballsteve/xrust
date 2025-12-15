/*

Sun Microsystems test cases

*/

use std::fs;
use xrust::item::Node;
use xrust::parser::{ParseError, xml};
use xrust::trees::smite::RNode;

fn test_sun_error(xmldoc: &str){
    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        xmldoc,
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_err());
}

#[test]
fn uri01() {
    /*
        Test ID:uri01
        Test URI:not-wf/uri01.xml
        Spec Sections:4.2.2 [75]
        Description:        SYSTEM ids may not have URI fragments
    */

    test_sun_error(fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/uri01.xml")
        .unwrap()
        .as_str())
}
