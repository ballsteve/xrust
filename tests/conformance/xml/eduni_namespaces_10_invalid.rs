/*

Richard Tobin's XML Namespaces 1.0 test suite 14 Feb 2003

*/

use std::fs;
use xrust::item::Node;
use xrust::parser::xml;
use xrust::trees::smite::RNode;

#[test]
#[ignore]
fn rmtns10017() {
    /*
        Test ID:rmt-ns10-017
        Test URI:017.xml
        Spec Sections:-
        Description:Simple legal case: no namespaces
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/017.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn rmtns10018() {
    /*
        Test ID:rmt-ns10-018
        Test URI:018.xml
        Spec Sections:5.2
        Description:Simple legal case: default namespace
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/018.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn rmtns10019() {
    /*
        Test ID:rmt-ns10-019
        Test URI:019.xml
        Spec Sections:4
        Description:Simple legal case: prefixed element
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/019.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn rmtns10020() {
    /*
        Test ID:rmt-ns10-020
        Test URI:020.xml
        Spec Sections:4
        Description:Simple legal case: prefixed attribute
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/020.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn rmtns10021() {
    /*
        Test ID:rmt-ns10-021
        Test URI:021.xml
        Spec Sections:5.2
        Description:Simple legal case: default namespace and unbinding
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/021.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn rmtns10022() {
    /*
        Test ID:rmt-ns10-022
        Test URI:022.xml
        Spec Sections:5.2
        Description:Simple legal case: default namespace and rebinding
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/022.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn rmtns10024() {
    /*
        Test ID:rmt-ns10-024
        Test URI:024.xml
        Spec Sections:5.1
        Description:Simple legal case: prefix rebinding
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/024.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn rmtns10027() {
    /*
        Test ID:rmt-ns10-027
        Test URI:027.xml
        Spec Sections:2
        Description:Reserved prefixes and namespaces: using the xml prefix undeclared
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/027.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn rmtns10028() {
    /*
        Test ID:rmt-ns10-028
        Test URI:028.xml
        Spec Sections:NE05
        Description:Reserved prefixes and namespaces: declaring the xml prefix correctly
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/028.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn rmtns10034() {
    /*
        Test ID:rmt-ns10-034
        Test URI:034.xml
        Spec Sections:NE05
        Description:Reserved prefixes and namespaces: binding a reserved prefix
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/034.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn rmtns10037() {
    /*
        Test ID:rmt-ns10-037
        Test URI:037.xml
        Spec Sections:5.3
        Description:Attribute uniqueness: different attributes with same local name
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/037.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn rmtns10038() {
    /*
        Test ID:rmt-ns10-038
        Test URI:038.xml
        Spec Sections:5.3
        Description:Attribute uniqueness: prefixed and unprefixed attributes with same local name
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/038.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn rmtns10039() {
    /*
        Test ID:rmt-ns10-039
        Test URI:039.xml
        Spec Sections:5.3
        Description:Attribute uniqueness: prefixed and unprefixed attributes with same local name, with default namespace
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/039.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn rmtns10040() {
    /*
        Test ID:rmt-ns10-040
        Test URI:040.xml
        Spec Sections:5.3
        Description:Attribute uniqueness: prefixed and unprefixed attributes with same local name, with default namespace and element in default namespace
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/040.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn rmtns10041() {
    /*
        Test ID:rmt-ns10-041
        Test URI:041.xml
        Spec Sections:5.3
        Description:Attribute uniqueness: prefixed and unprefixed attributes with same local name, element in same namespace as prefixed attribute
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/041.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn rmtns10045() {
    /*
        Test ID:rmt-ns10-045
        Test URI:045.xml
        Spec Sections:NE08
        Description:Colon in ID attribute name
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/045.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn rmtns10046() {
    /*
        Test ID:rmt-ns10-046
        Test URI:046.xml
        Spec Sections:NE08
        Description:Colon in ID attribute name
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/046.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}
