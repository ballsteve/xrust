/*

James Clark XMLTEST cases

    This contains cases that are well-formed XML documents but are not valid XML documents

*/

#[cfg(all(test, feature = "test-conformance-xml"))]
use std::fs;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::item::Node;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::parser::{ParseError, ParserStateBuilder, StaticStateBuilder, xml};
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::trees::smite::RNode;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::{Error, ErrorKind};

#[cfg(all(test, feature = "test-conformance-xml"))]
fn test_xmltest_invalid(xmldoc: &str) {
    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/".to_string())
        .build();
    let parseresult = xml::parse_with_state(xmldoc, ps, ss);

    assert!(parseresult.is_err());
}
#[cfg(all(test, feature = "test-conformance-xml"))]
fn dtdfileresolve() -> fn(Option<String>, String) -> Result<String, Error> {
    move |locdir, uri| {
        let u = match locdir {
            None => uri,
            Some(ld) => ld + uri.as_str(),
        };
        match fs::read_to_string(u) {
            Err(_) => Err(Error::new(
                ErrorKind::Unknown,
                "Unable to read external DTD".to_string(),
            )),
            Ok(s) => Ok(s),
        }
    }
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn invalid002() {
    /*
        Test ID:invalid--002
        Test URI:invalid/002.xml
        Spec Sections:3.2.1
        Description:Tests the "Proper Group/PE Nesting" validity constraint by fragmenting a content model between two parameter entities.
    */

    test_xmltest_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/invalid/002.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn invalid005() {
    /*
        Test ID:invalid--005
        Test URI:invalid/005.xml
        Spec Sections:2.8
        Description:Tests the "Proper Declaration/PE Nesting" validity constraint by fragmenting an element declaration between two parameter entities.
    */

    test_xmltest_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/invalid/005.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn invalid006() {
    /*
        Test ID:invalid--006
        Test URI:invalid/006.xml
        Spec Sections:2.8
        Description:Tests the "Proper Declaration/PE Nesting" validity constraint by fragmenting an element declaration between two parameter entities.
    */

    test_xmltest_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/invalid/006.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn invalidnotsa022() {
    /*
        Test ID:invalid-not-sa-022
        Test URI:invalid/not-sa/022.xml
        Spec Sections:3.4 [62]
        Description:Test the "Proper Conditional Section/ PE Nesting" validity constraint.
    */

    test_xmltest_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/invalid/not-sa/022.xml")
            .unwrap()
            .as_str(),
    );
}
