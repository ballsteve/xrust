/*

IBM test cases

*/

use crate::conformance::dtdfileresolve;
use std::fs;
use xrust::item::Node;
use xrust::parser::{ParseError, ParserStateBuilder, StaticStateBuilder, xml};
use xrust::trees::smite::RNode;
use xrust::validators::Schema;

#[test]
fn ibm11valid_p02ibm02v01xml() {
    /*
        Test ID:ibm-1-1-valid-P02-ibm02v01.xml
        Test URI:valid/P02/ibm02v01.xml
        Spec Sections:2.2
        Description:This test case covers legal character ranges plus discrete legal characters for production 02 of the XML1.1 sepcification.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P02/ibm02v01.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
fn ibm11valid_p02ibm02v02xml() {
    /*
        Test ID:ibm-1-1-valid-P02-ibm02v02.xml
        Test URI:valid/P02/ibm02v02.xml
        Spec Sections:2.2,4.1
        Description:This test case covers control characters x1 to x1F and x7F to x9F which should only appear as character references.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P02/ibm02v02.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p02ibm02v03xml() {
    /*
        Test ID:ibm-1-1-valid-P02-ibm02v03.xml
        Test URI:valid/P02/ibm02v03.xml
        Spec Sections:2.2,4.1
        Description:This test case covers control characters x1 to x1F and x7F to x9F which appear as character references as an entity's replacement text.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P02/ibm02v03.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
fn ibm11valid_p02ibm02v04xml() {
    /*
        Test ID:ibm-1-1-valid-P02-ibm02v04.xml
        Test URI:valid/P02/ibm02v04.xml
        Spec Sections:2.2,4.1
        Description:This test case contains embeded whitespace characters some form the range 1 - 1F.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P02/ibm02v04.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
fn ibm11valid_p02ibm02v05xml() {
    /*
        Test ID:ibm-1-1-valid-P02-ibm02v05.xml
        Test URI:valid/P02/ibm02v05.xml
        Spec Sections:2.2,4.1
        Description:This test case contains valid char references that match the char production.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P02/ibm02v05.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p02ibm02v06xml() {
    /*
        Test ID:ibm-1-1-valid-P02-ibm02v06.xml
        Test URI:valid/P02/ibm02v06.xml
        Spec Sections:2.2,4.1
        Description:This test case contains valid char references in the CDATA section, comment and processing instruction of an external entity that match the char production.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P02/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P02/ibm02v06.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
fn ibm11valid_p03ibm03v01xml() {
    /*
        Test ID:ibm-1-1-valid-P03-ibm03v01.xml
        Test URI:valid/P03/ibm03v01.xml
        Spec Sections:2.11
        Description:The two character sequence #x0D #x85 in an external entity must be normalized to a single newline.
    */
    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/".to_string())
        .build();

    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/ibm03v01.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/out/ibm03v01.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn ibm11valid_p03ibm03v02xml() {
    /*
        Test ID:ibm-1-1-valid-P03-ibm03v02.xml
        Test URI:valid/P03/ibm03v02.xml
        Spec Sections:2.11
        Description:The single character sequence #x85 in an external entity must be normalized to a single newline.
    */
    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/ibm03v02.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/out/ibm03v02.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn ibm11valid_p03ibm03v03xml() {
    /*
        Test ID:ibm-1-1-valid-P03-ibm03v03.xml
        Test URI:valid/P03/ibm03v03.xml
        Spec Sections:2.11
        Description:The two character sequence #x0D #x85 in an external entity must be normalized to a single newline.
    */
    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/ibm03v03.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/out/ibm03v03.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn ibm11valid_p03ibm03v04xml() {
    /*
        Test ID:ibm-1-1-valid-P03-ibm03v04.xml
        Test URI:valid/P03/ibm03v04.xml
        Spec Sections:2.11
        Description:The single character sequence #x85 in an external entity must be normalized to a single newline.
    */
    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/ibm03v04.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/out/ibm03v04.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn ibm11valid_p03ibm03v05xml() {
    /*
        Test ID:ibm-1-1-valid-P03-ibm03v05.xml
        Test URI:valid/P03/ibm03v05.xml
        Spec Sections:2.11
        Description:The two character sequence #x0D #x85 in a document entity must be normalized to a single newline.
    */
    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/ibm03v05.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/out/ibm03v05.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn ibm11valid_p03ibm03v06xml() {
    /*
        Test ID:ibm-1-1-valid-P03-ibm03v06.xml
        Test URI:valid/P03/ibm03v06.xml
        Spec Sections:2.11
        Description:The single character sequence #x85 in a document entity must be normalized to a single newline.
    */
    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/ibm03v06.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/out/ibm03v06.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn ibm11valid_p03ibm03v07xml() {
    /*
        Test ID:ibm-1-1-valid-P03-ibm03v07.xml
        Test URI:valid/P03/ibm03v07.xml
        Spec Sections:2.11
        Description:The single character sequence #x2028 in a document entity must be normalized to a single newline.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/ibm03v07.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/out/ibm03v07.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn ibm11valid_p03ibm03v08xml() {
    /*
        Test ID:ibm-1-1-valid-P03-ibm03v08.xml
        Test URI:valid/P03/ibm03v08.xml
        Spec Sections:2.11
        Description:The single character sequence #x85 in the XMLDecl must be normalized to a single newline.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/ibm03v08.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/out/ibm03v08.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn ibm11valid_p03ibm03v09xml() {
    /*
        Test ID:ibm-1-1-valid-P03-ibm03v09.xml
        Test URI:valid/P03/ibm03v09.xml
        Spec Sections:2.11
        Description:The single character sequence #x2028 in the XMLDecl must be normalized to a single newline. (This test is questionable)
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/ibm03v09.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P03/out/ibm03v09.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
}

/*
#[test]
#[ignore]
fn ibm11valid_p04ibm04v01xml() {
    /*
        This test is deliberately ignored. Although these are valid XML documents,
        XML without namespaces is not something we wish to handle.
    */
    /*
        Test ID:ibm-1-1-valid-P04-ibm04v01.xml
        Test URI:valid/P04/ibm04v01.xml
        Spec Sections:2.3
        Description:This test case covers legal NameStartChars character ranges plus discrete legal characters for production 04.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P04/ibm04v01.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())

}
 */

#[test]
fn ibm11valid_p04ibm04av01xml() {
    /*
        Test ID:ibm-1-1-valid-P04-ibm04av01.xml
        Test URI:valid/P04a/ibm04av01.xml
        Spec Sections:2.3
        Description:This test case covers legal NameChars character ranges plus discrete legal characters for production 04a.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P04a/ibm04av01.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

/*
#[test]
#[ignore]
fn ibm11valid_p05ibm05v01xml() {
    /*
        This test is deliberately ignored. Although these are valid XML documents,
        XML without namespaces is not something we wish to handle.
    */
    /*
        Test ID:ibm-1-1-valid-P05-ibm05v01.xml
        Test URI:valid/P05/ibm05v01.xml
        Spec Sections:2.3
        Description:This test case covers legal Element Names as per production 5.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P05/ibm05v01.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())

}
 */

#[test]
#[ignore]
fn ibm11valid_p05ibm05v02xml() {
    /*
        Test ID:ibm-1-1-valid-P05-ibm05v02.xml
        Test URI:valid/P05/ibm05v02.xml
        Spec Sections:2.3
        Description:This test case covers legal PITarget (Names) as per production 5.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P05/ibm05v02.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

/*
#[test]
#[ignore]
fn ibm11valid_p05ibm05v03xml() {
    /*
        This test is deliberately ignored. Although these are valid XML documents,
        XML without namespaces is not something we wish to handle.
    */
    /*
        Test ID:ibm-1-1-valid-P05-ibm05v03.xml
        Test URI:valid/P05/ibm05v03.xml
        Spec Sections:2.3
        Description:This test case covers legal Attribute (Names) as per production 5.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P05/ibm05v03.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())

}
*/

#[test]
fn ibm11valid_p05ibm05v04xml() {
    /*
        Test ID:ibm-1-1-valid-P05-ibm05v04.xml
        Test URI:valid/P05/ibm05v04.xml
        Spec Sections:2.3
        Description:This test case covers legal ID/IDREF (Names) as per production 5.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P05/ibm05v04.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

/*
#[test]
#[ignore]
fn ibm11valid_p05ibm05v05xml() {
    /*
        This test is deliberately ignored. Although these are valid XML documents,
        XML without namespaces is not something we wish to handle.
    */
    /*
        Test ID:ibm-1-1-valid-P05-ibm05v05.xml
        Test URI:valid/P05/ibm05v05.xml
        Spec Sections:2.3
        Description:This test case covers legal ENTITY (Names) as per production 5.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P05/ibm05v05.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())

}
 */

#[test]
fn ibm11valid_p047ibm07v01xml() {
    /*
        Test ID:ibm-1-1-valid-P047-ibm07v01.xml
        Test URI:valid/P07/ibm07v01.xml
        Spec Sections:2.3
        Description:This test case covers legal NMTOKEN Name character ranges plus discrete legal characters for production 7.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P07/ibm07v01.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v01xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v01.xml
        Test URI:valid/P77/ibm77v01.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 whereas the VersionNum of the external DTD is 1.0. The character #xC0 which is a valid XML 1.1 but an invalid XML 1.0 character is present in both documents.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v01.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v02xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v02.xml
        Test URI:valid/P77/ibm77v02.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 whereas the VersionNum of the external DTD is 1.0. The character #x1FFF which is a valid XML 1.1 but an invalid XML 1.0 character is present in both documents.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v02.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v03xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v03.xml
        Test URI:valid/P77/ibm77v03.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 whereas the VersionNum of the external DTD is 1.0. The character #xF901 which is a valid XML 1.1 but an invalid XML 1.0 character is present in both documents.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v03.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v04xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v04.xml
        Test URI:valid/P77/ibm77v04.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 whereas the VersionNum of the external entity is 1.0. The character #xD6 which is a valid XML 1.1 but an invalid XML 1.0 character is present in both documents.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v04.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v05xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v05.xml
        Test URI:valid/P77/ibm77v05.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 whereas the VersionNum of the external entity is 1.0. The character #x1FFF which is a valid XML 1.1 but an invalid XML 1.0 character is present in both documents.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v05.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v06xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v06.xml
        Test URI:valid/P77/ibm77v06.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 whereas the VersionNum of the external entity is 1.0. The character #xF901 which is a valid XML 1.1 but an invalid XML 1.0 character is present in both documents.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v06.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v07xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v07.xml
        Test URI:valid/P77/ibm77v07.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document and external dtd is 1.1 and both contain the valid XML1.1 but invalid XML1.0 character #xD8.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v07.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v08xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v08.xml
        Test URI:valid/P77/ibm77v08.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document and external dtd is 1.1 and both contain the valid XML1.1 but invalid XML1.0 character #x1FFF.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v08.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v09xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v09.xml
        Test URI:valid/P77/ibm77v09.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document and external dtd is 1.1 and both contain the valid XML1.1 but invalid XML1.0 character #xF901.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v09.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v10xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v10.xml
        Test URI:valid/P77/ibm77v10.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document and external entity is 1.1 and both contain the valid XML1.1 but invalid XML1.0 character #xF6.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v10.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v11xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v11.xml
        Test URI:valid/P77/ibm77v11.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document and external entity is 1.1 and both contain the valid XML1.1 but invalid XML1.0 character #x1FFF.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v11.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v12xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v12.xml
        Test URI:valid/P77/ibm77v12.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document and external entity is 1.1 and both contain the valid XML1.1 but invalid XML1.0 character #xF901.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v12.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v13xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v13.xml
        Test URI:valid/P77/ibm77v13.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 but the external dtd does not contain a textDecl and both contain the valid XML1.1 but invalid XML1.0 character #xF8.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v13.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v14xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v14.xml
        Test URI:valid/P77/ibm77v14.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 but the external dtd does not contain a textDecl and both contain the valid XML1.1 but invalid XML1.0 character #x1FFF.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v14.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v15xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v15.xml
        Test URI:valid/P77/ibm77v15.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 but the external dtd does not contain a textDecl and both contain the valid XML1.1 but invalid XML1.0 character #xF901.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v15.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v16xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v16.xml
        Test URI:valid/P77/ibm77v16.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 but the external entity does not contain a textDecl and both contain the valid XML1.1 but invalid XML1.0 character #x2FF.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v16.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v17xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v17.xml
        Test URI:valid/P77/ibm77v17.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 but the external entity does not contain a textDecl and both contain the valid XML1.1 but invalid XML1.0 character #x1FFF.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v17.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v18xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v18.xml
        Test URI:valid/P77/ibm77v18.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 but the external entity does not contain a textDecl and both contain the valid XML1.1 but invalid XML1.0 character #xF901.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v18.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v19xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v19.xml
        Test URI:valid/P77/ibm77v19.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document and external dtd is 1.1. The replacement text of an entity declared in the external DTD contains a reference to the character #x7F. This entity is not referenced in the document entity.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v19.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v20xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v20.xml
        Test URI:valid/P77/ibm77v20.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document and external dtd is 1.1. The replacement text of an entity declared in the external DTD contains a reference to the character #x80. This entity is not referenced in the document entity.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v20.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v21xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v21.xml
        Test URI:valid/P77/ibm77v21.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document and external dtd is 1.1. The replacement text of an entity declared in the external DTD contains a reference to the character #x9F. This entity is not referenced in the document entity.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v21.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v22xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v22.xml
        Test URI:valid/P77/ibm77v22.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document and the external entity is 1.1. The entity contains a reference to the character #x7F.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v22.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v23xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v23.xml
        Test URI:valid/P77/ibm77v23.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document and the external entity is 1.1. The entity contains a reference to the character #x80.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v23.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v24xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v24.xml
        Test URI:valid/P77/ibm77v24.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document and the external entity is 1.1. The entity contains a reference to the character #x9F.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v24.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v25xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v25.xml
        Test URI:valid/P77/ibm77v25.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document is 1.1 and the textDecl is missing in the external DTD. The replacement text of an entity declared in the external DTD contains a reference to the character #x7F, #x8F. This entity is not referenced in the document entity.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v25.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v26xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v26.xml
        Test URI:valid/P77/ibm77v26.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document is 1.1 and the textDecl is missing in the external DTD. The replacement text of an entity declared in the external DTD contains a reference to the character #x80, #x90. This entity is not referenced in the document entity.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v26.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v27xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v27.xml
        Test URI:valid/P77/ibm77v27.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document is 1.1 and the textDecl is missing in the external DTD. The replacement text of an entity declared in the external DTD contains a reference to the character #x81, #x9F. This entity is not referenced in the document entity.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v27.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v28xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v28.xml
        Test URI:valid/P77/ibm77v28.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document is 1.1 and the textDecl is missing in the external entity. The replacement text of an entity declared in the external DTD contains a reference to the character #x7F, #x80, #x9F.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v28.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v29xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v29.xml
        Test URI:valid/P77/ibm77v29.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document is 1.1 and the textDecl is missing in the external entity. The replacement text of an entity declared in the external DTD contains a reference to the character #x85, #x8F.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v29.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}

#[test]
#[ignore]
fn ibm11valid_p77ibm77v30xml() {
    /*
        Test ID:ibm-1-1-valid-P77-ibm77v30.xml
        Test URI:valid/P77/ibm77v30.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document is 1.1 and the textDecl is missing in the external entity. The replacement text of an entity declared in the external DTD contains a reference to the character #x1, #x7F.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/valid/P77/ibm77v30.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(parseresult.unwrap().validate(Schema::DTD).is_ok())
}
