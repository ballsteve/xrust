/*

OASIS/NIST test cases

*/

#[cfg(all(test, feature = "test-conformance-xml"))]
use std::fs;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::item::Node;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::parser::{ParseError, xml};
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::trees::smite::RNode;

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op11pass1() {
    /*
       Conforming XML 1.0 Processors are permitted to ignore certain errors, or to report them at user option.
       For this one, we are ignoring the error. If you want us to start reporting it, raise a ticket!
    */
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
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    //assert!(parseresult.is_err());
}
