/*

University of Edinburgh XML 1.0 4th edition errata test suite.

*/

use crate::conformance::dtdfileresolve;
use std::fs;
use xrust::item::Node;
use xrust::parser::{ParseError, ParserStateBuilder, StaticStateBuilder, xml};
use xrust::trees::smite::RNode;
use xrust::validators::Schema;

#[test]
#[ignore]
fn invalidbo1() {
    /*
        Test ID:invalid-bo-1
        Test URI:inclbom_be.xml
        Spec Sections:4.3.3
        Description:Byte order mark in general entity should go away (big-endian)
    */
    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/eduni/errata-4e/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/inclbom_be.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_err());
}

#[test]
#[ignore]
fn invalidbo2() {
    /*
        Test ID:invalid-bo-2
        Test URI:inclbom_le.xml
        Spec Sections:4.3.3
        Description:Byte order mark in general entity should go away (little-endian)
    */
    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/eduni/errata-4e/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/inclbom_le.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );
    assert!(parseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_err());
}

#[test]
#[ignore]
fn invalidbo3() {
    /*
        Test ID:invalid-bo-3
        Test URI:incl8bom.xml
        Spec Sections:4.3.3
        Description:Byte order mark in general entity should go away (utf-8)
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/incl8bom.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_err());
}

#[test]
#[ignore]
fn invalidbo4() {
    /*
        Test ID:invalid-bo-4
        Test URI:inclbombom_be.xml
        Spec Sections:4.3.3
        Description:Two byte order marks in general entity produce only one (big-endian)
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/inclbombom_be.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_err());
}

#[test]
#[ignore]
fn invalidbo5() {
    /*
        Test ID:invalid-bo-5
        Test URI:inclbombom_le.xml
        Spec Sections:4.3.3
        Description:Two byte order marks in general entity produce only one (little-endian)
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/inclbombom_le.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_err());
}

#[test]
#[ignore]
fn invalidbo6() {
    /*
        Test ID:invalid-bo-6
        Test URI:incl8bombom.xml
        Spec Sections:4.3.3
        Description:Two byte order marks in general entity produce only one (utf-8)
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/incl8bombom.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_err());
}

/*
#[test]
#[ignore]
fn invalidsa140() {

    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */

    /*
        Test ID:invalid-sa-140
        Test URI:140.xml
        Spec Sections:2.3 [4]
        Description:Character '&#x309a;' is a CombiningChar, not a Letter, but as of 5th edition, may begin a name (c.f. xmltest/not-wf/sa/140.xml).
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/140.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_err());
}
 */

/*
#[test]
#[ignore]
fn invalidsa141() {

    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */

    /*
        Test ID:invalid-sa-141
        Test URI:141.xml
        Spec Sections:2.3 [5]
        Description:As of 5th edition, character #x0E5C is legal in XML names (c.f. xmltest/not-wf/sa/141.xml).
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/141.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_err());
}
 */

/*
#[test]
#[ignore]
fn xrmt5014() {

    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */

    /*
        Test ID:x-rmt5-014
        Test URI:014.xml
        Spec Sections:2.3
        Description:Has a "long s" in a name, legal in XML 1.1, legal in XML 1.0 5th edition
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/014.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_err());
}
 */

/*
#[test]
#[ignore]
fn xrmt5016() {

    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */

    /*
        Test ID:x-rmt5-016
        Test URI:016.xml
        Spec Sections:2.3
        Description:Has a Byzantine Musical Symbol Kratimata in a name, legal in XML 1.1, legal in XML 1.0 5th edition
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/016.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_err());
}
 */

/*
#[test]
#[ignore]
fn xrmt5019() {

    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */

    /*
        Test ID:x-rmt5-019
        Test URI:019.xml
        Spec Sections:2.3
        Description:Has the last legal namechar in XML 1.1, legal in XML 1.0 5th edition
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/019.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(testxml.is_err());
}
*/

#[test]
fn ibminvalid_p89ibm89n06xml() {
    /*
        Test ID:ibm-invalid-P89-ibm89n06.xml
        Test URI:ibm89n06.xml
        Spec Sections:B.
        Description:Tests Extender with an only legal per 5th edition character. The character #x0EC7 occurs as the second character in the PITarget in the PI in the prolog, and in an element name.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm89n06.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_err());
}

#[test]
fn ibminvalid_p89ibm89n07xml() {
    /*
        Test ID:ibm-invalid-P89-ibm89n07.xml
        Test URI:ibm89n07.xml
        Spec Sections:B.
        Description:Tests Extender with an only legal per 5th edition character. The character #x3006 occurs as the second character in the PITarget in the PI in the prolog, and in an element name.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm89n07.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_err());
}

#[test]
fn ibminvalid_p89ibm89n08xml() {
    /*
        Test ID:ibm-invalid-P89-ibm89n08.xml
        Test URI:ibm89n08.xml
        Spec Sections:B.
        Description:Tests Extender with an only legal per 5th edition character. The character #x3030 occurs as the second character in the PITarget in the PI in the prolog, and in an element name.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm89n08.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_err());
}

#[test]
fn ibminvalid_p89ibm89n09xml() {
    /*
        Test ID:ibm-invalid-P89-ibm89n09.xml
        Test URI:ibm89n09.xml
        Spec Sections:B.
        Description:Tests Extender with an only legal per 5th edition character. The character #x3036 occurs as the second character in the PITarget in the PI in the prolog, and in an element name.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm89n09.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_err());
}

#[test]
fn ibminvalid_p89ibm89n10xml() {
    /*
        Test ID:ibm-invalid-P89-ibm89n10.xml
        Test URI:ibm89n10.xml
        Spec Sections:B.
        Description:Tests Extender with an only legal per 5th edition character. The character #x309C occurs as the second character in the PITarget in the PI in the prolog, and in an element name.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm89n10.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_err());
}

#[test]
fn ibminvalid_p89ibm89n11xml() {
    /*
        Test ID:ibm-invalid-P89-ibm89n11.xml
        Test URI:ibm89n11.xml
        Spec Sections:B.
        Description:Tests Extender with an only legal per 5th edition character. The character #x309F occurs as the second character in the PITarget in the PI in the prolog, and in an element name.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm89n11.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_err());
}

#[test]
fn ibminvalid_p89ibm89n12xml() {
    /*
        Test ID:ibm-invalid-P89-ibm89n12.xml
        Test URI:ibm89n12.xml
        Spec Sections:B.
        Description:Tests Extender with an only legal per 5th edition character. The character #x30FF occurs as the second character in the PITarget in the PI in the prolog, and in an element name.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm89n12.xml")
            .unwrap()
            .as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_err());
}
