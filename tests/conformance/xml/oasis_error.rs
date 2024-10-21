/*

OASIS/NIST test cases

*/

use std::fs;
use xrust::item::Node;
use xrust::parser::xml;
use xrust::trees::smite::RNode;

#[test]
#[ignore]
fn op11pass1() {
    /*
        Test ID:o-p11pass1
        Test URI:p11pass1.xml
        Spec Sections:2.3, 4.2.2 [11]
        Description:system literals may not contain URI fragments
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p11pass1.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}
