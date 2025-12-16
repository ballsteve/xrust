/*

Richard Tobin's XML 1.0 2nd edition errata test suite.

*/

use crate::conformance::dtdfileresolve;
use std::fs;
use xrust::item::Node;
use xrust::parser::{ParseError, ParserStateBuilder, StaticStateBuilder, xml};
use xrust::trees::smite::RNode;
use xrust::validators::Schema;

fn test_eduni_errata2e_invalid(xmldoc: &str) {
    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();
    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/eduni/errata-2e/".to_string())
        .build();

    let parseresult = xml::parse_with_state(xmldoc, ps, ss);

    assert!(parseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_err());
}

#[test]
#[ignore]
fn rmte2e2a() {
    /*
        Test ID:rmt-e2e-2a
        Test URI:E2a.xml
        Spec Sections:E2
        Description:Duplicate token in enumerated attribute declaration
    */
    test_eduni_errata2e_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E2a.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
fn rmte2e2b() {
    /*
        Test ID:rmt-e2e-2b
        Test URI:E2b.xml
        Spec Sections:E2
        Description:Duplicate token in NOTATION attribute declaration
    */
    test_eduni_errata2e_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E2b.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
fn rmte2e9b() {
    /*
        Test ID:rmt-e2e-9b
        Test URI:E9b.xml
        Spec Sections:E9
        Description:An attribute default must be syntactically correct even if unused
    */
    test_eduni_errata2e_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E9b.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
fn rmte2e14() {
    /*
        Test ID:rmt-e2e-14
        Test URI:E14.xml
        Spec Sections:E14
        Description:Declarations mis-nested wrt parameter entities are just validity errors (but note that some parsers treat some such errors as fatal)
    */
    test_eduni_errata2e_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E14.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
fn rmte2e15a() {
    /*
        Test ID:rmt-e2e-15a
        Test URI:E15a.xml
        Spec Sections:E15
        Description:Empty content can't contain an entity reference
    */
    test_eduni_errata2e_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E15a.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
fn rmte2e15b() {
    /*
        Test ID:rmt-e2e-15b
        Test URI:E15b.xml
        Spec Sections:E15
        Description:Empty content can't contain a comment
    */
    test_eduni_errata2e_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E15b.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
fn rmte2e15c() {
    /*
        Test ID:rmt-e2e-15c
        Test URI:E15c.xml
        Spec Sections:E15
        Description:Empty content can't contain a PI
    */
    test_eduni_errata2e_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E15c.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
fn rmte2e15d() {
    /*
        Test ID:rmt-e2e-15d
        Test URI:E15d.xml
        Spec Sections:E15
        Description:Empty content can't contain whitespace
    */
    test_eduni_errata2e_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E15d.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
fn rmte2e15g() {
    /*
        Test ID:rmt-e2e-15g
        Test URI:E15g.xml
        Spec Sections:E15
        Description:Element content can't contain character reference to whitespace
    */
    test_eduni_errata2e_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E15g.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
fn rmte2e15h() {
    /*
        Test ID:rmt-e2e-15h
        Test URI:E15h.xml
        Spec Sections:E15
        Description:Element content can't contain entity reference if replacement text is character reference to whitespace
    */
    test_eduni_errata2e_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E15h.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
fn rmte2e20() {
    /*
        Test ID:rmt-e2e-20
        Test URI:E20.xml
        Spec Sections:E20
        Description:Tokens, after normalization, must be separated by space, not other whitespace characters
    */
    test_eduni_errata2e_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-2e/E20.xml")
            .unwrap()
            .as_str(),
    );
}
