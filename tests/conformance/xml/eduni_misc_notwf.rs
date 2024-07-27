/*

Bjoern Hoehrmann via HST 2013-09-18

*/

use std::fs;
use std::rc::Rc;
use xrust::parser::xml;
use xrust::trees::smite::Node as SmiteNode;

#[test]
fn hstbh001() {
    /*
        Test ID:hst-bh-001
        Test URI:001.xml
        Spec Sections:2.2 [2], 4.1 [66]
        Description:decimal charref > 10FFFF, indeed > max 32 bit integer, checking for recovery from possible overflow
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/misc/001.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn hstbh002() {
    /*
        Test ID:hst-bh-002
        Test URI:002.xml
        Spec Sections:2.2 [2], 4.1 [66]
        Description:hex charref > 10FFFF, indeed > max 32 bit integer, checking for recovery from possible overflow
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/misc/002.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn hstbh003() {
    /*
        Test ID:hst-bh-003
        Test URI:003.xml
        Spec Sections:2.2 [2], 4.1 [66]
        Description:decimal charref > 10FFFF, indeed > max 64 bit integer, checking for recovery from possible overflow
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/misc/003.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn hstbh004() {
    /*
        Test ID:hst-bh-004
        Test URI:004.xml
        Spec Sections:2.2 [2], 4.1 [66]
        Description:hex charref > 10FFFF, indeed > max 64 bit integer, checking for recovery from possible overflow
    */


    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/misc/004.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn hstlhs007() {
    /*
        Test ID:hst-lhs-007
        Test URI:007.xml
        Spec Sections:4.3.3
        Description:UTF-8 BOM plus xml decl of iso-8859-1 incompatible
    */


    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/misc/007.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn hstlhs008() {
    /*
        Test ID:hst-lhs-008
        Test URI:008.xml
        Spec Sections:4.3.3
        Description:UTF-16 BOM plus xml decl of utf-8 (using UTF-16 coding) incompatible
    */


    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/misc/008.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn hstlhs009() {
    /*
        Test ID:hst-lhs-009
        Test URI:009.xml
        Spec Sections:4.3.3
        Description:UTF-16 BOM plus xml decl of utf-8 (using UTF-8 coding) incompatible
    */


    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/misc/009.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}
