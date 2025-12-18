/*

James Clark XMLTEST cases - Standalone

    This contains cases that are not well-formed XML documents
    This contains cases that are not standalone.

*/

#[cfg(all(test, feature = "test-conformance-xml"))]
use std::fs;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::{Error, ErrorKind};
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::item::Node;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::parser::{ParseError, ParserStateBuilder, StaticStateBuilder, xml};
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::trees::smite::RNode;

#[cfg(all(test, feature = "test-conformance-xml"))]
fn test_xmltest_notwf_not_sa(xmldoc: &str) {
    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/".to_string())
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
#[cfg(all(test, feature = "test-conformance-xml"))]
fn notwfnotsa001() {
    /*
        Test ID:not-wf-not-sa-001
        Test URI:not-wf/not-sa/001.xml
        Spec Sections:3.4 [62]
        Description:Conditional sections must be properly terminated ("]>" usedinstead of "]]>").
    */

    test_xmltest_notwf_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/001.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn notwfnotsa002() {
    /*
        Test ID:not-wf-not-sa-002
        Test URI:not-wf/not-sa/002.xml
        Spec Sections:2.6 [17]
        Description:Processing instruction target names may not be "XML"in any combination of cases.
    */

    test_xmltest_notwf_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/002.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn notwfnotsa003() {
    /*
        Test ID:not-wf-not-sa-003
        Test URI:not-wf/not-sa/003.xml
        Spec Sections:3.4 [62]
        Description:Conditional sections must be properly terminated ("]]>" omitted).
    */

    test_xmltest_notwf_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/003.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn notwfnotsa004() {
    /*
        Test ID:not-wf-not-sa-004
        Test URI:not-wf/not-sa/004.xml
        Spec Sections:3.4 [62]
        Description:Conditional sections must be properly terminated ("]]>" omitted).
    */

    test_xmltest_notwf_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/004.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn notwfnotsa005() {
    /*
        Test ID:not-wf-not-sa-005
        Test URI:not-wf/not-sa/005.xml
        Spec Sections:4.1
        Description:Tests the Entity Declared VC by referring to anundefined parameter entity within an external entity.
    */

    test_xmltest_notwf_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/005.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn notwfnotsa006() {
    /*
        Test ID:not-wf-not-sa-006
        Test URI:not-wf/not-sa/006.xml
        Spec Sections:3.4 [62]
        Description:Conditional sections need a '[' after the INCLUDE or IGNORE.
    */

    test_xmltest_notwf_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/006.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn notwfnotsa007() {
    /*
        Test ID:not-wf-not-sa-007
        Test URI:not-wf/not-sa/007.xml
        Spec Sections:4.3.2 [79]
        Description:A <!DOCTYPE ...> declaration may not begin any externalentity; it's only found once, in the document entity.
    */

    test_xmltest_notwf_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/007.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn notwfnotsa008() {
    /*
        Test ID:not-wf-not-sa-008
        Test URI:not-wf/not-sa/008.xml
        Spec Sections:4.1 [69]
        Description:In DTDs, the '%' character must be part of a parameterentity reference.
    */

    test_xmltest_notwf_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/008.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn notwfnotsa009() {
    /*
        Test ID:not-wf-not-sa-009
        Test URI:not-wf/not-sa/009.xml
        Spec Sections:2.8
        Description:This test violates WFC:PE Between Declarations in Production 28a.The last character of a markup declaration is not contained in the sameparameter-entity text replacement.
    */

    test_xmltest_notwf_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/not-sa/009.xml")
            .unwrap()
            .as_str(),
    );
}
