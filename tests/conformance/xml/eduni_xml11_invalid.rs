/*
Richard Tobin's XML 1.1 test suite 13 Feb 2003
 */

use std::convert::TryFrom;
use std::fs;
use xrust::Document;

/*
#[test]
fn rmt015() {
    /*
        This test is deliberately ignored. The charachter is now valid in 5th edition.
    */
    /*
        Test ID:rmt-015
        Test URI:015.xml
        Spec Sections:2.3
        Description:Has a "long s" in a name, legal in XML 1.1, illegal in XML 1.0 thru 4th edition
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/015.xml").unwrap(),
        None,
        None,
    ));
    assert!(testxml.is_err());
}
 */

#[test]
#[ignore]
fn rmt017() {
    /*
        Test ID:rmt-017
        Test URI:017.xml
        Spec Sections:2.3
        Description:Has a Byzantine Musical Symbol Kratimata in a name, legal in XML 1.1, illegal in XML 1.0 thru 4th edition
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/017.xml").unwrap(),
        None,
        None,
    ));
    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn rmt018() {
    /*
        Test ID:rmt-018
        Test URI:018.xml
        Spec Sections:2.3
        Description:Has the last legal namechar in XML 1.1, illegal in XML 1.0 thru 4th edition
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/018.xml").unwrap(),
        None,
        None,
    ));
    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn rmt030() {
    /*
        Test ID:rmt-030
        Test URI:030.xml
        Spec Sections:2.11
        Description:Has a NEL character in an NMTOKENS attribute; well-formed in both XML 1.0 and 1.1, but valid only in 1.1
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/030.xml").unwrap(),
        None,
        None,
    ));
    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn rmt032() {
    /*
        Test ID:rmt-032
        Test URI:032.xml
        Spec Sections:2.11
        Description:Has an LSEP character in an NMTOKENS attribute; well-formed in both XML 1.0 and 1.1, but valid only in 1.1
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/032.xml").unwrap(),
        None,
        None,
    ));
    assert!(testxml.is_err());
}

#[test]
fn rmt036() {
    /*
        Test ID:rmt-036
        Test URI:036.xml
        Spec Sections:2.3
        Description:Has an NMTOKENS attribute containing a NEL character that comes from a character reference in an internal entity. Because NEL is not in the S production (even though real NELs are converted to LF on input), this is invalid in both XML 1.0 and 1.1.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/036.xml").unwrap(),
        None,
        None,
    ));
    assert!(testxml.is_err());
}

#[test]
fn rmt037() {
    /*
        Test ID:rmt-037
        Test URI:037.xml
        Spec Sections:2.3
        Description:Has an NMTOKENS attribute containing a NEL character that comes from a character reference in an internal entity. Because NEL is not in the S production (even though real NELs are converted to LF on input), this is invalid in both XML 1.0 and 1.1.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/037.xml").unwrap(),
        None,
        None,
    ));
    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn rmt046() {
    /*
        Test ID:rmt-046
        Test URI:046.xml
        Spec Sections:2.11
        Description:Has a NEL character in element content whitespace; well-formed in both XML 1.0 and 1.1, but valid only in 1.1
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/046.xml").unwrap(),
        None,
        None,
    ));
    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn rmt048() {
    /*
        Test ID:rmt-048
        Test URI:048.xml
        Spec Sections:2.11
        Description:Has an LSEP character in element content whitespace; well-formed in both XML 1.0 and 1.1, but valid only in 1.1
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/048.xml").unwrap(),
        None,
        None,
    ));
    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn rmt052() {
    /*
        Test ID:rmt-052
        Test URI:052.xml
        Spec Sections:2.3
        Description:Has element content whitespace containing a NEL character that comes from a character reference in an internal entity. Because NEL is not in the S production (even though real NELs are converted to LF on input), this is invalid in both XML 1.0 and 1.1.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/052.xml").unwrap(),
        None,
        None,
    ));
    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn rmt053() {
    /*
        Test ID:rmt-053
        Test URI:053.xml
        Spec Sections:2.3
        Description:Has element content whitespace containing a NEL character that comes from a character reference in an internal entity. Because NEL is not in the S production (even though real NELs are converted to LF on input), this is invalid in both XML 1.0 and 1.1.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/053.xml").unwrap(),
        None,
        None,
    ));
    assert!(testxml.is_err());
}
