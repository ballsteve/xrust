/*

Sun Microsystems test cases

*/

use std::convert::TryFrom;
use std::fs;
use xrust::Document;

#[test]
fn uri01() {
    /*
        Test ID:uri01
        Test URI:not-wf/uri01.xml
        Spec Sections:4.2.2 [75]
        Description:SYSTEM ids may not have URI fragments
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/uri01.xml").unwrap(),
        None
    ));

    assert!(testxml.is_err());
}
