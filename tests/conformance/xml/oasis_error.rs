/*

OASIS/NIST test cases

*/

use std::fs;
use std::rc::Rc;
use xrust::parser::xml;
use xrust::trees::smite::Node as SmiteNode;

#[test]
#[ignore]
fn op11pass1() {
    /*
        Test ID:o-p11pass1
        Test URI:p11pass1.xml
        Spec Sections:2.3, 4.2.2 [11]
        Description:system literals may not contain URI fragments
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p11pass1.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}



