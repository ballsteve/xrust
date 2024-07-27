/*

Richard Tobin's XML 1.0 2nd edition errata test suite.

*/

use crate::conformance::dtdfileresolve;
use std::fs;
use std::rc::Rc;
use xrust::parser::{xml, ParserConfig};
use xrust::trees::smite::Node as SmiteNode;
use xrust::Node;

#[test]
#[ignore]
fn rmte2e9a() {
    /*
        Test ID:rmt-e2e-9a
        Test URI:E9a.xml
        Spec Sections:E9
        Description:An unused attribute default need only be syntactically correct
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E9a.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn rmte2e15e() {
    /*
        Test ID:rmt-e2e-15e
        Test URI:E15e.xml
        Spec Sections:E15
        Description:Element content can contain entity reference if replacement text is whitespace
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E15e.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn rmte2e15f() {
    /*
        Test ID:rmt-e2e-15f
        Test URI:E15f.xml
        Spec Sections:E15
        Description:Element content can contain entity reference if replacement text is whitespace, even if it came from a character reference in the literal entity value
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E15f.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn rmte2e15i() {
    /*
        Test ID:rmt-e2e-15i
        Test URI:E15i.xml
        Spec Sections:E15
        Description:Element content can contain a comment
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E15i.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn rmte2e15j() {
    /*
        Test ID:rmt-e2e-15j
        Test URI:E15j.xml
        Spec Sections:E15
        Description:Element content can contain a PI
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E15j.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn rmte2e15k() {
    /*
        Test ID:rmt-e2e-15k
        Test URI:E15k.xml
        Spec Sections:E15
        Description:Mixed content can contain a comment
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E15k.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn rmte2e15l() {
    /*
        Test ID:rmt-e2e-15l
        Test URI:E15l.xml
        Spec Sections:E15
        Description:Mixed content can contain a PI
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E15l.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
#[ignore]
fn rmte2e18() {
    /*
        Test ID:rmt-e2e-18
        Test URI:E18.xml
        Spec Sections:E18
        Description:External entity containing start of entity declaration is base URI for system identifier
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E18.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E18.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert!(parseresult.unwrap().get_canonical().unwrap() == canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn rmte2e19() {
    /*
        Test ID:rmt-e2e-19
        Test URI:E19.xml
        Spec Sections:E19
        Description:Parameter entities and character references are included-in-literal, but general entities are bypassed.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E19.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E19.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert!(parseresult.unwrap().get_canonical().unwrap() == canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn rmte2e22() {
    /*
        Test ID:rmt-e2e-22
        Test URI:E22.xml
        Spec Sections:E22
        Description:UTF-8 entities may start with a BOM
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E22.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn rmte2e24() {
    /*
        Test ID:rmt-e2e-24
        Test URI:E24.xml
        Spec Sections:E24
        Description:Either the built-in entity or a character reference can be used to represent greater-than after two close-square-brackets
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E24.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn rmte2e29() {
    /*
        Test ID:rmt-e2e-29
        Test URI:E29.xml
        Spec Sections:E29
        Description:Three-letter language codes are allowed
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E29.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
#[ignore]
fn rmte2e36() {
    /*
        Test ID:rmt-e2e-36
        Test URI:E36.xml
        Spec Sections:E36
        Description:An external ATTLIST declaration does not make a document non-standalone if the normalization would have been the same without the declaration
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E36.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn rmte2e41() {
    /*
        Test ID:rmt-e2e-41
        Test URI:E41.xml
        Spec Sections:E41
        Description:An xml:lang attribute may be empty
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E41.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn rmte2e48() {
    /*
        Test ID:rmt-e2e-48
        Test URI:E48.xml
        Spec Sections:E48
        Description:ANY content allows character data
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E48.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
#[ignore]
fn rmte2e50() {
    /*
        Test ID:rmt-e2e-50
        Test URI:E50.xml
        Spec Sections:E50
        Description:All line-ends are normalized, even those not passed to the application. NB this can only be tested effectively in XML 1.1, since CR is in the S production; in 1.1 we can use NEL which isn't.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E50.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn rmte2e60() {
    /*
        Test ID:rmt-e2e-60
        Test URI:E60.xml
        Spec Sections:E60
        Description:Conditional sections are allowed in external parameter entities referred to from the internal subset.
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/eduni/errata-2e/".to_string());

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E60.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_ok());
}
