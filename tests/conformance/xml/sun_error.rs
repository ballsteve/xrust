/*

Sun Microsystems test cases

*/

use std::fs;
use xrust::item::Node;
use xrust::parser::{ParseError, xml};
use xrust::trees::smite::RNode;

#[test]
fn uri01() {
    /*
        Test ID:uri01
        Test URI:not-wf/uri01.xml
        Spec Sections:4.2.2 [75]
        Description:        SYSTEM ids may not have URI fragments
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/uri01.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_err());
}
