/*
Richard Tobin's XML 1.0 2nd edition errata test suite.
*/

use crate::conformance::non_utf8_file_reader;
use std::convert::TryFrom;
use std::fs;
use xrust::Document;

#[test]
#[ignore]
fn rmte2e27() {
    /*
        Test ID:rmt-e2e-27
        Test URI:E27.xml
        Spec Sections:E27
        Description:Contains an irregular UTF-8 sequence (i.e. a surrogate pair)
    */

    let testxml = Document::try_from((
        non_utf8_file_reader("tests/conformance/xml/xmlconf/eduni/errata-2e/E27.xml"),
        //fs::read_to_string().unwrap(),
        None,
        None,
    ));
    assert!(testxml.is_err());
}

#[test]
fn rmte2e38() {
    /*
        Test ID:rmt-e2e-38
        Test URI:E38.xml
        Spec Sections:E38
        Description:XML 1.0 document refers to 1.1 entity
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E38.xml").unwrap(),
        None,
        None,
    ));
    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn rmte2e61() {
    /*
        Test ID:rmt-e2e-61
        Test URI:E61.xml
        Spec Sections:E61
        Description:(From John Cowan) An encoding declaration in ASCII specifying an encoding that is not compatible with ASCII (so the document is not in its declared encoding). It should generate a fatal error.
    */
    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E61.xml").unwrap(),
        None,
        None,
    ));
    assert!(testxml.is_err());
}
