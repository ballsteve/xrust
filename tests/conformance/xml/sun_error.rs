/*

Sun Microsystems test cases

*/

use std::fs;
use std::rc::Rc;
use xrust::parser::xml;
use xrust::trees::smite::Node as SmiteNode;

#[test]
fn uri01() {
    /*
        Test ID:uri01
        Test URI:not-wf/uri01.xml
        Spec Sections:4.2.2 [75]
        Description:        SYSTEM ids may not have URI fragments
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/uri01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}
