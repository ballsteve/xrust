/*

James Clark XMLTEST cases - Standalone

    This contains cases that are valid XML documents.
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
use xrust::validators::Schema;

#[cfg(all(test, feature = "test-conformance-xml"))]
fn test_xmltest_valid_not_sa(xmldoc: &str, xmlcanondoc: &str) {
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
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml,
        xmlcanondoc,
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
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
fn validnotsa001() {
    /*
        Test ID:valid-not-sa-001
        Test URI:valid/not-sa/001.xml
        Spec Sections:4.2.2 [75]
        Description:Test demonstrates the use of an ExternalID within a document type definition.
    */
    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/001.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/001.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa002() {
    /*
        Test ID:valid-not-sa-002
        Test URI:valid/not-sa/002.xml
        Spec Sections:4.2.2 [75]
        Description:Test demonstrates the use of an ExternalID within a document type definition.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/002.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/002.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa003() {
    /*
        Test ID:valid-not-sa-003
        Test URI:valid/not-sa/003.xml
        Spec Sections:4.1 [69]
        Description:Test demonstrates the expansion of an external parameter entity that declares an attribute.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/003.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/003.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa004() {
    /*
        Test ID:valid-not-sa-004
        Test URI:valid/not-sa/004.xml
        Spec Sections:4.1 [69]
        Description:Expands an external parameter entity in two different ways, with one of them declaring an attribute.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/004.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/004.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa005() {
    /*
        Test ID:valid-not-sa-005
        Test URI:valid/not-sa/005.xml
        Spec Sections:4.1 [69]
        Description:Test demonstrates the expansion of an external parameter entity that declares an attribute.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/005.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/005.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa006() {
    /*
        Test ID:valid-not-sa-006
        Test URI:valid/not-sa/006.xml
        Spec Sections:3.3 [52]
        Description:Test demonstrates that when more than one definition is provided for the same attribute of a given element type only the first declaration is binding.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/006.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/006.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa007() {
    /*
        Test ID:valid-not-sa-007
        Test URI:valid/not-sa/007.xml
        Spec Sections:3.3 [52]
        Description:Test demonstrates the use of an Attribute list declaration within an external entity.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/007.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/007.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa008() {
    /*
        Test ID:valid-not-sa-008
        Test URI:valid/not-sa/008.xml
        Spec Sections:4.2.2 [75]
        Description:Test demonstrates that an external identifier may include a public identifier.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/008.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/008.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa009() {
    /*
        Test ID:valid-not-sa-009
        Test URI:valid/not-sa/009.xml
        Spec Sections:4.2.2 [75]
        Description:Test demonstrates that an external identifier may include a public identifier.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/009.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/009.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa010() {
    /*
        Test ID:valid-not-sa-010
        Test URI:valid/not-sa/010.xml
        Spec Sections:3.3 [52]
        Description:Test demonstrates that when more that one definition is provided for the same attribute of a given element type only the first declaration is binding.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/010.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/010.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa011() {
    /*
        Test ID:valid-not-sa-011
        Test URI:valid/not-sa/011.xml
        Spec Sections:4.2 4.2.1 [72] [75]
        Description:Test demonstrates a parameter entity declaration whose parameter entity definition is an ExternalID.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/011.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/011.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa012() {
    /*
        Test ID:valid-not-sa-012
        Test URI:valid/not-sa/012.xml
        Spec Sections:4.3.1 [77]
        Description:Test demonstrates an enternal parsed entity that begins with a text declaration.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/012.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/012.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa013() {
    /*
        Test ID:valid-not-sa-013
        Test URI:valid/not-sa/013.xml
        Spec Sections:3.4 [62]
        Description:Test demonstrates the use of the conditional section INCLUDE that will include its contents as part of the DTD.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/013.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/013.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa014() {
    /*
        Test ID:valid-not-sa-014
        Test URI:valid/not-sa/014.xml
        Spec Sections:3.4 [62]
        Description:Test demonstrates the use of the conditional section INCLUDE that will include its contents as part of the DTD. The keyword is a parameter-entity reference.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/014.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/014.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa015() {
    /*
        Test ID:valid-not-sa-015
        Test URI:valid/not-sa/015.xml
        Spec Sections:3.4 [63]
        Description:Test demonstrates the use of the conditonal section IGNORE the will ignore its content from being part of the DTD. The keyword is a parameter-entity reference.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/015.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/015.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa016() {
    /*
        Test ID:valid-not-sa-016
        Test URI:valid/not-sa/016.xml
        Spec Sections:3.4 [62]
        Description:Test demonstrates the use of the conditional section INCLUDE that will include its contents as part of the DTD. The keyword is a parameter-entity reference.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/016.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/016.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa017() {
    /*
        Test ID:valid-not-sa-017
        Test URI:valid/not-sa/017.xml
        Spec Sections:4.2 [72]
        Description:Test demonstrates a parameter entity declaration that contains an attribute list declaration.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/017.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/017.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa018() {
    /*
        Test ID:valid-not-sa-018
        Test URI:valid/not-sa/018.xml
        Spec Sections:4.2.2 [75]
        Description:Test demonstrates an EnternalID whose contents contain an parameter entity declaration and a attribute list definition.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/018.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/018.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa019() {
    /*
        Test ID:valid-not-sa-019
        Test URI:valid/not-sa/019.xml
        Spec Sections:4.4.8
        Description:Test demonstrates that a parameter entity will be expanded with spaces on either side.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/019.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/019.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa020() {
    /*
        Test ID:valid-not-sa-020
        Test URI:valid/not-sa/020.xml
        Spec Sections:4.4.8
        Description:Parameter entities expand with spaces on either side.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/020.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/020.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa021() {
    /*
        Test ID:valid-not-sa-021
        Test URI:valid/not-sa/021.xml
        Spec Sections:4.2 [72]
        Description:Test demonstrates a parameter entity declaration that contains a partial attribute list declaration.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/021.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/021.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa023() {
    /*
        Test ID:valid-not-sa-023
        Test URI:valid/not-sa/023.xml
        Spec Sections:2.3 4.1 [10] [69]
        Description:Test demonstrates the use of a parameter entity reference within an attribute list declaration.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/023.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/023.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa024() {
    /*
        Test ID:valid-not-sa-024
        Test URI:valid/not-sa/024.xml
        Spec Sections:2.8, 4.1 [69]
        Description:Constructs an <!ATTLIST...> declaration from several PEs.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/024.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/024.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa025() {
    /*
        Test ID:valid-not-sa-025
        Test URI:valid/not-sa/025.xml
        Spec Sections:4.2
        Description:Test demonstrates that when more that one definition is provided for the same entity only the first declaration is binding.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/025.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/025.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa026() {
    /*
        Test ID:valid-not-sa-026
        Test URI:valid/not-sa/026.xml
        Spec Sections:3.3 [52]
        Description:Test demonstrates that when more that one definition is provided for the same attribute of a given element type only the first declaration is binding.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/026.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/026.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa027() {
    /*
        Test ID:valid-not-sa-027
        Test URI:valid/not-sa/027.xml
        Spec Sections:4.1 [69]
        Description:Test demonstrates a parameter entity reference whose value is NULL.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/027.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/027.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa028() {
    /*
        Test ID:valid-not-sa-028
        Test URI:valid/not-sa/028.xml
        Spec Sections:3.4 [62]
        Description:Test demonstrates the use of the conditional section INCLUDE that will include its contents.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/028.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/028.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa029() {
    /*
        Test ID:valid-not-sa-029
        Test URI:valid/not-sa/029.xml
        Spec Sections:3.4 [62]
        Description:Test demonstrates the use of the conditonal section IGNORE the will ignore its content from being used.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/029.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/029.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa030() {
    /*
        Test ID:valid-not-sa-030
        Test URI:valid/not-sa/030.xml
        Spec Sections:3.4 [62]
        Description:Test demonstrates the use of the conditonal section IGNORE the will ignore its content from being used.
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/030.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/030.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn validnotsa031() {
    /*
        Test ID:valid-not-sa-031
        Test URI:valid/not-sa/031.xml
        Spec Sections:2.7
        Description:Expands a general entity which contains a CDATA section with what looks like a markup declaration (but is just text since it's in a CDATA section).
    */

    test_xmltest_valid_not_sa(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/031.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/031.xml")
            .unwrap()
            .as_str(),
    );
}
