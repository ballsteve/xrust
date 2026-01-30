/*

IBM test cases

*/

#[cfg(all(test, feature = "test-conformance-xml"))]
use std::fs;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::item::Node;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::parser::{ParseError, xml};
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::trees::smite::RNode;

#[cfg(all(test, feature = "test-conformance-xml"))]
fn test_ibm_invalid(xmldoc: &str) {
    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        xmldoc,
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p28ibm28i01xml() {
    /*
        Test ID:ibm-invalid-P28-ibm28i01.xml
        Test URI:invalid/P28/ibm28i01.xml
        Spec Sections:2.8
        Description:The test violates VC:Root Element Type in P28. The Name in the document type declaration does not match the element type of the root element.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P28/ibm28i01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p32ibm32i01xml() {
    /*
        Test ID:ibm-invalid-P32-ibm32i01.xml
        Test URI:invalid/P32/ibm32i01.xml
        Spec Sections:2.9
        Description:This test violates VC: Standalone Document Declaration in P32. The standalone document declaration has the value yes, BUT there is an external markup declaration of attributes with default values, and the associated element appears in the document with specified values for those attributes.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P32/ibm32i01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p32ibm32i03xml() {
    /*
        Test ID:ibm-invalid-P32-ibm32i03.xml
        Test URI:invalid/P32/ibm32i03.xml
        Spec Sections:2.9
        Description:This test violates VC: Standalone Document Declaration in P32. The standalone document declaration has the value yes, BUT there is an external markup declaration of attributes with values that will change if normalized.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P32/ibm32i03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p32ibm32i04xml() {
    /*
        Test ID:ibm-invalid-P32-ibm32i04.xml
        Test URI:invalid/P32/ibm32i04.xml
        Spec Sections:2.9
        Description:This test violates VC: Standalone Document Declaration in P32. The standalone document declaration has the value yes, BUT there is an external markup declaration of element with element content, and white space occurs directly within the mixed content.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P32/ibm32i04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p39ibm39i01xml() {
    /*
        Test ID:ibm-invalid-P39-ibm39i01.xml
        Test URI:invalid/P39/ibm39i01.xml
        Spec Sections:3
        Description:This test violates VC: Element Valid in P39. Element a is declared empty in DTD, but has content in the document.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P39/ibm39i01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p39ibm39i02xml() {
    /*
        Test ID:ibm-invalid-P39-ibm39i02.xml
        Test URI:invalid/P39/ibm39i02.xml
        Spec Sections:3
        Description:This test violates VC: Element Valid in P39. root is declared only having element children in DTD, but have text content in the document.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P39/ibm39i02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p39ibm39i03xml() {
    /*
        Test ID:ibm-invalid-P39-ibm39i03.xml
        Test URI:invalid/P39/ibm39i03.xml
        Spec Sections:3
        Description:This test violates VC: Element Valid in P39. Illegal elements are inserted in b's content of Mixed type.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P39/ibm39i03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p39ibm39i04xml() {
    /*
        Test ID:ibm-invalid-P39-ibm39i04.xml
        Test URI:invalid/P39/ibm39i04.xml
        Spec Sections:3
        Description:This test violates VC: Element Valid in P39. Element c has undeclared element as its content of ANY type
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P39/ibm39i04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p41ibm41i01xml() {
    /*
        Test ID:ibm-invalid-P41-ibm41i01.xml
        Test URI:invalid/P41/ibm41i01.xml
        Spec Sections:3.1
        Description:This test violates VC: Attribute Value Type in P41. attr1 for Element b is not declared.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P41/ibm41i01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p41ibm41i02xml() {
    /*
        Test ID:ibm-invalid-P41-ibm41i02.xml
        Test URI:invalid/P41/ibm41i02.xml
        Spec Sections:3.1
        Description:This test violates VC: Attribute Value Type in P41. attr3 for Element b is given a value that does not match the declaration in the DTD.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P41/ibm41i02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p45ibm45i01xml() {
    /*
        Test ID:ibm-invalid-P45-ibm45i01.xml
        Test URI:invalid/P45/ibm45i01.xml
        Spec Sections:3.2
        Description:This test violates VC: Unique Element Type Declaration. Element not_unique has been declared 3 time in the DTD.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P45/ibm45i01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p49ibm49i01xml() {
    /*
        Test ID:ibm-invalid-P49-ibm49i01.xml
        Test URI:invalid/P49/ibm49i01.xml
        Spec Sections:3.2.1
        Description:Violates VC:Proper Group/PE Nesting in P49. Open and close parenthesis for a choice content model are in different PE replace Texts.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P49/ibm49i01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p50ibm50i01xml() {
    /*
        Test ID:ibm-invalid-P50-ibm50i01.xml
        Test URI:invalid/P50/ibm50i01.xml
        Spec Sections:3.2.1
        Description:Violates VC:Proper Group/PE Nesting in P50. Open and close parenthesis for a seq content model are in different PE replace Texts.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P50/ibm50i01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p51ibm51i01xml() {
    /*
        Test ID:ibm-invalid-P51-ibm51i01.xml
        Test URI:invalid/P51/ibm51i01.xml
        Spec Sections:3.2.2
        Description:Violates VC:Proper Group/PE Nesting in P51. Open and close parenthesis for a Mixed content model are in different PE replace Texts.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P51/ibm51i01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p51ibm51i03xml() {
    /*
        Test ID:ibm-invalid-P51-ibm51i03.xml
        Test URI:invalid/P51/ibm51i03.xml
        Spec Sections:3.2.2
        Description:Violates VC:No Duplicate Types in P51. Element a appears twice in the Mixed content model of Element e.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P51/ibm51i03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p56ibm56i01xml() {
    /*
        Test ID:ibm-invalid-P56-ibm56i01.xml
        Test URI:invalid/P56/ibm56i01.xml
        Spec Sections:3.3.1
        Description:Tests invalid TokenizedType which is against P56 VC: ID. The value of the ID attribute "UniqueName" is "@999" which does not meet the Name production.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P56/ibm56i01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p56ibm56i02xml() {
    /*
        Test ID:ibm-invalid-P56-ibm56i02.xml
        Test URI:invalid/P56/ibm56i02.xml
        Spec Sections:3.3.1
        Description:Tests invalid TokenizedType which is against P56 VC: ID. The two ID attributes "attr" and "UniqueName" have the same value "Ac999" for the element "b" and the element "tokenizer".
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P56/ibm56i02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p56ibm56i03xml() {
    /*
        Test ID:ibm-invalid-P56-ibm56i03.xml
        Test URI:invalid/P56/ibm56i03.xml
        Spec Sections:3.3.1
        Description:Tests invalid TokenizedType which is against P56 VC: ID Attribute Default. The "#FIXED" occurs in the DefaultDecl for the ID attribute "UniqueName".
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P56/ibm56i03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p56ibm56i05xml() {
    /*
        Test ID:ibm-invalid-P56-ibm56i05.xml
        Test URI:invalid/P56/ibm56i05.xml
        Spec Sections:3.3.1
        Description:Tests invalid TokenizedType which is against P56 VC: ID Attribute Default. The constant string "BOGUS" occurs in the DefaultDecl for the ID attribute "UniqueName".
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P56/ibm56i05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p56ibm56i06xml() {
    /*
        Test ID:ibm-invalid-P56-ibm56i06.xml
        Test URI:invalid/P56/ibm56i06.xml
        Spec Sections:3.3.1
        Description:Tests invalid TokenizedType which is against P56 VC: One ID per Element Type. The element "a" has two ID attributes "first" and "second".
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P56/ibm56i06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p56ibm56i07xml() {
    /*
        Test ID:ibm-invalid-P56-ibm56i07.xml
        Test URI:invalid/P56/ibm56i07.xml
        Spec Sections:3.3.1
        Description:Tests invalid TokenizedType which is against P56 VC: IDREF. The value of the IDREF attribute "reference" is "@456" which does not meet the Name production.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P56/ibm56i07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p56ibm56i08xml() {
    /*
        Test ID:ibm-invalid-P56-ibm56i08.xml
        Test URI:invalid/P56/ibm56i08.xml
        Spec Sections:3.3.1
        Description:Tests invalid TokenizedType which is against P56 VC: IDREF. The value of the IDREF attribute "reference" is "BC456" which does not match the value assigned to any ID attributes.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P56/ibm56i08.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p56ibm56i09xml() {
    /*
        Test ID:ibm-invalid-P56-ibm56i09.xml
        Test URI:invalid/P56/ibm56i09.xml
        Spec Sections:3.3.1
        Description:Tests invalid TokenizedType which is against P56 VC: IDREFS. The value of the IDREFS attribute "reference" is "AC456 #567" which does not meet the Names production.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P56/ibm56i09.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p56ibm56i10xml() {
    /*
        Test ID:ibm-invalid-P56-ibm56i10.xml
        Test URI:invalid/P56/ibm56i10.xml
        Spec Sections:3.3.1
        Description:Tests invalid TokenizedType which is against P56 VC: IDREFS. The value of the IDREFS attribute "reference" is "EF456 DE355" which does not match the values assigned to two ID attributes.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P56/ibm56i10.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p56ibm56i11xml() {
    /*
        Test ID:ibm-invalid-P56-ibm56i11.xml
        Test URI:invalid/P56/ibm56i11.xml
        Spec Sections:3.3.1
        Description:Tests invalid TokenizedType which is against P56 VC: Entity Name. The value of the ENTITY attribute "sun" is "ima ge" which does not meet the Name production.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P56/ibm56i11.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p56ibm56i12xml() {
    /*
        Test ID:ibm-invalid-P56-ibm56i12.xml
        Test URI:invalid/P56/ibm56i12.xml
        Spec Sections:3.3.1
        Description:Tests invalid TokenizedType which is against P56 VC: Entity Name. The value of the ENTITY attribute "sun" is "notimage" which does not match the name of any unparsed entity declared.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P56/ibm56i12.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p56ibm56i13xml() {
    /*
        Test ID:ibm-invalid-P56-ibm56i13.xml
        Test URI:invalid/P56/ibm56i13.xml
        Spec Sections:3.3.1
        Description:Tests invalid TokenizedType which is against P56 VC: Entity Name. The value of the ENTITY attribute "sun" is "parsedentity" which matches the name of a parsed entity instead of an unparsed entity declared.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P56/ibm56i13.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p56ibm56i14xml() {
    /*
        Test ID:ibm-invalid-P56-ibm56i14.xml
        Test URI:invalid/P56/ibm56i14.xml
        Spec Sections:3.3.1
        Description:Tests invalid TokenizedType which is against P56 VC: Entity Name. The value of the ENTITIES attribute "sun" is "#image1 @image" which does not meet the Names production.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P56/ibm56i14.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p56ibm56i15xml() {
    /*
        Test ID:ibm-invalid-P56-ibm56i15.xml
        Test URI:invalid/P56/ibm56i15.xml
        Spec Sections:3.3.1
        Description:Tests invalid TokenizedType which is against P56 VC: ENTITIES. The value of the ENTITIES attribute "sun" is "image3 image4" which does not match the names of two unparsed entities declared.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P56/ibm56i15.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p56ibm56i16xml() {
    /*
        Test ID:ibm-invalid-P56-ibm56i16.xml
        Test URI:invalid/P56/ibm56i16.xml
        Spec Sections:3.3.1
        Description:Tests invalid TokenizedType which is against P56 VC: ENTITIES. The value of the ENTITIES attribute "sun" is "parsedentity1 parsedentity2" which matches the names of two parsed entities instead of two unparsed entities declared.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P56/ibm56i16.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p56ibm56i17xml() {
    /*
        Test ID:ibm-invalid-P56-ibm56i17.xml
        Test URI:invalid/P56/ibm56i17.xml
        Spec Sections:3.3.1
        Description:Tests invalid TokenizedType which is against P56 VC: Name Token. The value of the NMTOKEN attribute "thistoken" is "x : image" which does not meet the Nmtoken production.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P56/ibm56i17.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p56ibm56i18xml() {
    /*
        Test ID:ibm-invalid-P56-ibm56i18.xml
        Test URI:invalid/P56/ibm56i18.xml
        Spec Sections:3.3.1
        Description:Tests invalid TokenizedType which is against P56 VC: Name Token. The value of the NMTOKENS attribute "thistoken" is "@lang y: #country" which does not meet the Nmtokens production.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P56/ibm56i18.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p58ibm58i01xml() {
    /*
        Test ID:ibm-invalid-P58-ibm58i01.xml
        Test URI:invalid/P58/ibm58i01.xml
        Spec Sections:3.3.1
        Description:Tests invalid NotationType which is against P58 VC: Notation Attributes. The attribute "content-encoding" with value "raw" is not a value from the list "(base64|uuencode)".
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P58/ibm58i01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p58ibm58i02xml() {
    /*
        Test ID:ibm-invalid-P58-ibm58i02.xml
        Test URI:invalid/P58/ibm58i02.xml
        Spec Sections:3.3.1
        Description:Tests invalid NotationType which is against P58 VC: Notation Attributes. The attribute "content-encoding" with value "raw" is a value from the list "(base64|uuencode|raw|ascii)", but "raw" is not a declared notation.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P58/ibm58i02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p59ibm59i01xml() {
    /*
        Test ID:ibm-invalid-P59-ibm59i01.xml
        Test URI:invalid/P59/ibm59i01.xml
        Spec Sections:3.3.1
        Description:Tests invalid Enumeration which is against P59 VC: Enumeration. The value of the attribute is "ONE" which matches neither "one" nor "two" as declared in the Enumeration in the AttDef in the AttlistDecl.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P59/ibm59i01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p60ibm60i01xml() {
    /*
        Test ID:ibm-invalid-P60-ibm60i01.xml
        Test URI:invalid/P60/ibm60i01.xml
        Spec Sections:3.3.2
        Description:Tests invalid DefaultDecl which is against P60 VC: Required Attribute. The attribute "chapter" for the element "two" is declared as #REQUIRED in the DefaultDecl in the AttlistDecl, but the value of this attribute is not given.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P60/ibm60i01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p60ibm60i02xml() {
    /*
        Test ID:ibm-invalid-P60-ibm60i02.xml
        Test URI:invalid/P60/ibm60i02.xml
        Spec Sections:3.3.2
        Description:Tests invalid DefaultDecl which is against P60 VC: Fixed Attribute Default.. The attribute "chapter" for the element "one" is declared as #FIXED with the given value "Introduction" in the DefaultDecl in the AttlistDecl, but the value of a instance of this attribute is assigned to "JavaBeans".
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P60/ibm60i02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p60ibm60i03xml() {
    /*
        Test ID:ibm-invalid-P60-ibm60i03.xml
        Test URI:invalid/P60/ibm60i03.xml
        Spec Sections:3.3.2
        Description:Tests invalid DefaultDecl which is against P60 VC: Attribute Default Legal. The declared default value "c" is not legal for the type (a|b) in the AttDef in the AttlistDecl.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P60/ibm60i03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p60ibm60i04xml() {
    /*
        Test ID:ibm-invalid-P60-ibm60i04.xml
        Test URI:invalid/P60/ibm60i04.xml
        Spec Sections:3.3.2
        Description:Tests invalid DefaultDecl which is against P60 VC: Attribute Default Legal. The declared default value "@#$" is not legal for the type NMTOKEN the AttDef in the AttlistDecl.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P60/ibm60i04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p76ibm76i01xml() {
    /*
        Test ID:ibm-invalid-P76-ibm76i01.xml
        Test URI:invalid/P76/ibm76i01.xml
        Spec Sections:4.2.2
        Description:Tests invalid NDataDecl which is against P76 VC: Notation declared. The Name "JPGformat" in the NDataDecl in the EntityDecl for "ge2" does not match the Name of any declared notation.
    */

    test_ibm_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P76/ibm76i01.xml")
            .unwrap()
            .as_str(),
    );
}
