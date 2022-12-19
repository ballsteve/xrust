/*
Richard Tobin's XML Namespaces 1.0 test suite 14 Feb 2003
 */

use std::convert::TryFrom;
use std::fs;
use xrust::Document;

#[test]
fn rmtns10009() {
    /*
        Test ID:rmt-ns10-009
        Test URI:009.xml
        Spec Sections:1
        Description:Namespace equality test: plain repetition
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/009.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn rmtns10010() {
    /*
        Test ID:rmt-ns10-010
        Test URI:010.xml
        Spec Sections:1
        Description:Namespace equality test: use of character reference
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/010.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn rmtns10011() {
    /*
        Test ID:rmt-ns10-011
        Test URI:011.xml
        Spec Sections:1
        Description:Namespace equality test: use of entity reference
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/011.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn rmtns10012() {
    /*
        Test ID:rmt-ns10-012
        Test URI:012.xml
        Spec Sections:1
        Description:Namespace inequality test: equal after attribute value normalization
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/012.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn rmtns10013() {
    /*
        Test ID:rmt-ns10-013
        Test URI:013.xml
        Spec Sections:3
        Description:Bad QName syntax: multiple colons
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/013.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn rmtns10014() {
    /*
        Test ID:rmt-ns10-014
        Test URI:014.xml
        Spec Sections:3
        Description:Bad QName syntax: colon at end
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/014.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn rmtns10015() {
    /*
        Test ID:rmt-ns10-015
        Test URI:015.xml
        Spec Sections:3
        Description:Bad QName syntax: colon at start
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/015.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn rmtns10016() {
    /*
        Test ID:rmt-ns10-016
        Test URI:016.xml
        Spec Sections:2
        Description:Bad QName syntax: xmlns:
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/016.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn rmtns10023() {
    /*
        Test ID:rmt-ns10-023
        Test URI:023.xml
        Spec Sections:2
        Description:Illegal use of 1.1-style prefix unbinding in 1.0 document
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/023.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn rmtns10025() {
    /*
        Test ID:rmt-ns10-025
        Test URI:025.xml
        Spec Sections:4
        Description:Unbound element prefix
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/025.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn rmtns10026() {
    /*
        Test ID:rmt-ns10-026
        Test URI:026.xml
        Spec Sections:4
        Description:Unbound attribute prefix
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/026.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn rmtns10029() {
    /*
        Test ID:rmt-ns10-029
        Test URI:029.xml
        Spec Sections:NE05
        Description:Reserved prefixes and namespaces: declaring the xml prefix incorrectly
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/029.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn rmtns10030() {
    /*
        Test ID:rmt-ns10-030
        Test URI:030.xml
        Spec Sections:NE05
        Description:Reserved prefixes and namespaces: binding another prefix to the xml namespace
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/030.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn rmtns10031() {
    /*
        Test ID:rmt-ns10-031
        Test URI:031.xml
        Spec Sections:NE05
        Description:Reserved prefixes and namespaces: declaring the xmlns prefix with its correct URI (illegal)
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/031.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn rmtns10032() {
    /*
        Test ID:rmt-ns10-032
        Test URI:032.xml
        Spec Sections:NE05
        Description:Reserved prefixes and namespaces: declaring the xmlns prefix with an incorrect URI
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/032.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn rmtns10033() {
    /*
        Test ID:rmt-ns10-033
        Test URI:033.xml
        Spec Sections:NE05
        Description:Reserved prefixes and namespaces: binding another prefix to the xmlns namespace
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/033.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn rmtns10035() {
    /*
        Test ID:rmt-ns10-035
        Test URI:035.xml
        Spec Sections:5.3
        Description:Attribute uniqueness: repeated identical attribute
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/035.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn rmtns10036() {
    /*
        Test ID:rmt-ns10-036
        Test URI:036.xml
        Spec Sections:5.3
        Description:Attribute uniqueness: repeated attribute with different prefixes
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/036.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn rmtns10042() {
    /*
        Test ID:rmt-ns10-042
        Test URI:042.xml
        Spec Sections:NE08
        Description:Colon in PI name
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/042.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn rmtns10043() {
    /*
        Test ID:rmt-ns10-043
        Test URI:043.xml
        Spec Sections:NE08
        Description:Colon in entity name
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/043.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn rmtns10044() {
    /*
        Test ID:rmt-ns10-044
        Test URI:044.xml
        Spec Sections:NE08
        Description:Colon in entity name
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/044.xml").unwrap(),
    );

    assert!(testxml.is_err());
}
