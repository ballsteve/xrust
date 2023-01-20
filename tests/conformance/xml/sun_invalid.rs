/*

Sun Microsystems test cases

*/

use std::convert::TryFrom;
use std::fs;
use xrust::Document;

#[test]
#[ignore]
fn invdtd01() {
    /*
        Test ID:inv-dtd01
        Test URI:invalid/dtd01.xml
        Spec Sections:3.2.2
        Description:Tests the No Duplicate Types VC
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/dtd01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn invdtd02() {
    /*
        Test ID:inv-dtd02
        Test URI:invalid/dtd02.xml
        Spec Sections:4.2.2
        Description:Tests the "Notation Declared" VC by using an undeclared notation name.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/dtd02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn invdtd03() {
    /*
        Test ID:inv-dtd03
        Test URI:invalid/dtd03.xml
        Spec Sections:3
        Description:Tests the "Element Valid" VC (clause 2) by omitting a required element.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/dtd03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn el01() {
    /*
        Test ID:el01
        Test URI:invalid/el01.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 4) by including an undeclared child element.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/el01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn el02() {
    /*
        Test ID:el02
        Test URI:invalid/el02.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 1) by including elements in an EMPTY content model.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/el02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn el03() {
    /*
        Test ID:el03
        Test URI:invalid/el03.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 3) by including a child element not permitted by a mixed content model.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/el03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn el04() {
    /*
        Test ID:el04
        Test URI:invalid/el04.xml
        Spec Sections:3.2
        Description:Tests the Unique Element Type Declaration VC.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/el04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn el05() {
    /*
        Test ID:el05
        Test URI:invalid/el05.xml
        Spec Sections:3.2.2
        Description:Tests the No Duplicate Types VC.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/el05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn el06() {
    /*
        Test ID:el06
        Test URI:invalid/el06.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 1), using one of the predefined internal entities inside an EMPTY content model.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/el06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn id01() {
    /*
        Test ID:id01
        Test URI:invalid/id01.xml
        Spec Sections:3.3.1
        Description:Tests the ID (is a Name) VC
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/id01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn id02() {
    /*
        Test ID:id02
        Test URI:invalid/id02.xml
        Spec Sections:3.3.1
        Description:Tests the ID (appears once) VC
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/id02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn id03() {
    /*
        Test ID:id03
        Test URI:invalid/id03.xml
        Spec Sections:3.3.1
        Description:Tests the One ID per Element Type VC
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/id03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn id04() {
    /*
        Test ID:id04
        Test URI:invalid/id04.xml
        Spec Sections:3.3.1
        Description:Tests the ID Attribute Default VC
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/id04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn id05() {
    /*
        Test ID:id05
        Test URI:invalid/id05.xml
        Spec Sections:3.3.1
        Description:Tests the ID Attribute Default VC
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/id05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn id06() {
    /*
        Test ID:id06
        Test URI:invalid/id06.xml
        Spec Sections:3.3.1
        Description:Tests the IDREF (is a Name) VC
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/id06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn id07() {
    /*
        Test ID:id07
        Test URI:invalid/id07.xml
        Spec Sections:3.3.1
        Description:Tests the IDREFS (is a Names) VC
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/id07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn id08() {
    /*
        Test ID:id08
        Test URI:invalid/id08.xml
        Spec Sections:3.3.1
        Description:Tests the IDREF (matches an ID) VC
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/id08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn id09() {
    /*
        Test ID:id09
        Test URI:invalid/id09.xml
        Spec Sections:3.3.1
        Description:Tests the IDREF (IDREFS matches an ID) VC
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/id09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn invnotsa01() {
    /*
        Test ID:inv-not-sa01
        Test URI:invalid/not-sa01.xml
        Spec Sections:2.9
        Description:Tests the Standalone Document Declaration VC, ensuring that optional whitespace causes a validity error.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/not-sa01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn invnotsa02() {
    /*
        Test ID:inv-not-sa02
        Test URI:invalid/not-sa02.xml
        Spec Sections:2.9
        Description:Tests the Standalone Document Declaration VC, ensuring that attributes needing normalization cause a validity error.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/not-sa02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn invnotsa04() {
    /*
        Test ID:inv-not-sa04
        Test URI:invalid/not-sa04.xml
        Spec Sections:2.9
        Description:Tests the Standalone Document Declaration VC, ensuring that attributes needing defaulting cause a validity error.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/not-sa04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn invnotsa05() {
    /*
        Test ID:inv-not-sa05
        Test URI:invalid/not-sa05.xml
        Spec Sections:2.9
        Description:Tests the Standalone Document Declaration VC, ensuring that a token attribute that needs normalization causes a validity error.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/not-sa05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn invnotsa06() {
    /*
        Test ID:inv-not-sa06
        Test URI:invalid/not-sa06.xml
        Spec Sections:2.9
        Description:Tests the Standalone Document Declaration VC, ensuring that a NOTATION attribute that needs normalization causes a validity error.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/not-sa06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn invnotsa07() {
    /*
        Test ID:inv-not-sa07
        Test URI:invalid/not-sa07.xml
        Spec Sections:2.9
        Description:Tests the Standalone Document Declaration VC, ensuring that an NMTOKEN attribute needing normalization causes a validity error.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/not-sa07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn invnotsa08() {
    /*
        Test ID:inv-not-sa08
        Test URI:invalid/not-sa08.xml
        Spec Sections:2.9
        Description:Tests the Standalone Document Declaration VC, ensuring that an NMTOKENS attribute needing normalization causes a validity error.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/not-sa08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn invnotsa09() {
    /*
        Test ID:inv-not-sa09
        Test URI:invalid/not-sa09.xml
        Spec Sections:2.9
        Description:Tests the Standalone Document Declaration VC, ensuring that an ID attribute needing normalization causes a validity error.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/not-sa09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn invnotsa10() {
    /*
        Test ID:inv-not-sa10
        Test URI:invalid/not-sa10.xml
        Spec Sections:2.9
        Description:Tests the Standalone Document Declaration VC, ensuring that an IDREF attribute needing normalization causes a validity error.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/not-sa10.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn invnotsa11() {
    /*
        Test ID:inv-not-sa11
        Test URI:invalid/not-sa11.xml
        Spec Sections:2.9
        Description:Tests the Standalone Document Declaration VC, ensuring that an IDREFS attribute needing normalization causes a validity error.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/not-sa11.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn invnotsa12() {
    /*
        Test ID:inv-not-sa12
        Test URI:invalid/not-sa12.xml
        Spec Sections:2.9
        Description:Tests the Standalone Document Declaration VC, ensuring that an ENTITY attribute needing normalization causes a validity error.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/not-sa12.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn invnotsa13() {
    /*
        Test ID:inv-not-sa13
        Test URI:invalid/not-sa13.xml
        Spec Sections:2.9
        Description:Tests the Standalone Document Declaration VC, ensuring that an ENTITIES attribute needing normalization causes a validity error.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/not-sa13.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn invnotsa14() {
    /*
        Test ID:inv-not-sa14
        Test URI:invalid/not-sa14.xml
        Spec Sections:3
        Description:CDATA sections containing only whitespace do not match the nonterminal S, and cannot appear in these positions.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/not-sa14.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn optional01() {
    /*
        Test ID:optional01
        Test URI:invalid/optional01.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 2) for one instance of "children" content model, providing no children where one is required.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/optional01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn optional02() {
    /*
        Test ID:optional02
        Test URI:invalid/optional02.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 2) for one instance of "children" content model, providing two children where one is required.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/optional02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn optional03() {
    /*
        Test ID:optional03
        Test URI:invalid/optional03.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 2) for one instance of "children" content model, providing no children where two are required.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/optional03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn optional04() {
    /*
        Test ID:optional04
        Test URI:invalid/optional04.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 2) for one instance of "children" content model, providing three children where two are required.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/optional04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn optional05() {
    /*
        Test ID:optional05
        Test URI:invalid/optional05.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 2) for one instance of "children" content model, providing no children where one or two are required (one construction of that model).
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/optional05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn optional06() {
    /*
        Test ID:optional06
        Test URI:invalid/optional06.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 2) for one instance of "children" content model, providing no children where one or two are required (a second construction of that model).
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/optional06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn optional07() {
    /*
        Test ID:optional07
        Test URI:invalid/optional07.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 2) for one instance of "children" content model, providing no children where one or two are required (a third construction of that model).
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/optional07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn optional08() {
    /*
        Test ID:optional08
        Test URI:invalid/optional08.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 2) for one instance of "children" content model, providing no children where one or two are required (a fourth construction of that model).
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/optional08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn optional09() {
    /*
        Test ID:optional09
        Test URI:invalid/optional09.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 2) for one instance of "children" content model, providing no children where one or two are required (a fifth construction of that model).
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/optional09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn optional10() {
    /*
        Test ID:optional10
        Test URI:invalid/optional10.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 2) for one instance of "children" content model, providing three children where one or two are required (a basic construction of that model).
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/optional10.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn optional11() {
    /*
        Test ID:optional11
        Test URI:invalid/optional11.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 2) for one instance of "children" content model, providing three children where one or two are required (a second construction of that model).
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/optional11.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn optional12() {
    /*
        Test ID:optional12
        Test URI:invalid/optional12.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 2) for one instance of "children" content model, providing three children where one or two are required (a third construction of that model).
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/optional12.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn optional13() {
    /*
        Test ID:optional13
        Test URI:invalid/optional13.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 2) for one instance of "children" content model, providing three children where one or two are required (a fourth construction of that model).
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/optional13.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn optional14() {
    /*
        Test ID:optional14
        Test URI:invalid/optional14.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 2) for one instance of "children" content model, providing three children where one or two are required (a fifth construction of that model).
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/optional14.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn optional20() {
    /*
        Test ID:optional20
        Test URI:invalid/optional20.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 2) for one instance of "children" content model, providing no children where one or more are required (a sixth construction of that model).
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/optional20.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn optional21() {
    /*
        Test ID:optional21
        Test URI:invalid/optional21.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 2) for one instance of "children" content model, providing no children where one or more are required (a seventh construction of that model).
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/optional21.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn optional22() {
    /*
        Test ID:optional22
        Test URI:invalid/optional22.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 2) for one instance of "children" content model, providing no children where one or more are required (an eigth construction of that model).
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/optional22.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn optional23() {
    /*
        Test ID:optional23
        Test URI:invalid/optional23.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 2) for one instance of "children" content model, providing no children where one or more are required (a ninth construction of that model).
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/optional23.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn optional24() {
    /*
        Test ID:optional24
        Test URI:invalid/optional24.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 2) for one instance of "children" content model, providing no children where one or more are required (a tenth construction of that model).
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/optional24.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn optional25() {
    /*
        Test ID:optional25
        Test URI:invalid/optional25.xml
        Spec Sections:3
        Description:Tests the Element Valid VC (clause 2) for one instance of "children" content model, providing text content where one or more elements are required.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/optional25.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn invrequired00() {
    /*
        Test ID:inv-required00
        Test URI:invalid/required00.xml
        Spec Sections:3.3.2
        Description:Tests the Required Attribute VC.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/required00.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn invrequired01() {
    /*
        Test ID:inv-required01
        Test URI:invalid/required01.xml
        Spec Sections:3.1 2.10
        Description:Tests the Attribute Value Type (declared) VC for the xml:space attribute
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/required01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn invrequired02() {
    /*
        Test ID:inv-required02
        Test URI:invalid/required02.xml
        Spec Sections:3.1 2.12
        Description:Tests the Attribute Value Type (declared) VC for the xml:lang attribute
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/required02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn root() {
    /*
        Test ID:root
        Test URI:invalid/root.xml
        Spec Sections:2.8
        Description:Tests the Root Element Type VC
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/root.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn attr01() {
    /*
        Test ID:attr01
        Test URI:invalid/attr01.xml
        Spec Sections:3.3.1
        Description:Tests the "Entity Name" VC for the ENTITY attribute type.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/attr01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn attr02() {
    /*
        Test ID:attr02
        Test URI:invalid/attr02.xml
        Spec Sections:3.3.1
        Description:Tests the "Entity Name" VC for the ENTITIES attribute type.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/attr02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn attr03() {
    /*
        Test ID:attr03
        Test URI:invalid/attr03.xml
        Spec Sections:3.3.1
        Description:Tests the "Notation Attributes" VC for the NOTATION attribute type, first clause: value must be one of the ones that's declared.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/attr03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn attr04() {
    /*
        Test ID:attr04
        Test URI:invalid/attr04.xml
        Spec Sections:3.3.1
        Description:Tests the "Notation Attributes" VC for the NOTATION attribute type, second clause: the names in the declaration must all be declared.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/attr04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn attr05() {
    /*
        Test ID:attr05
        Test URI:invalid/attr05.xml
        Spec Sections:3.3.1
        Description:Tests the "Name Token" VC for the NMTOKEN attribute type.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/attr05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn attr06() {
    /*
        Test ID:attr06
        Test URI:invalid/attr06.xml
        Spec Sections:3.3.1
        Description:Tests the "Name Token" VC for the NMTOKENS attribute type.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/attr06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn attr07() {
    /*
        Test ID:attr07
        Test URI:invalid/attr07.xml
        Spec Sections:3.3.1
        Description:Tests the "Enumeration" VC by providing a value which wasn't one of the choices.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/attr07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn attr08() {
    /*
        Test ID:attr08
        Test URI:invalid/attr08.xml
        Spec Sections:3.3.2
        Description:Tests the "Fixed Attribute Default" VC by providing the wrong value.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/attr08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn attr09() {
    /*
        Test ID:attr09
        Test URI:invalid/attr09.xml
        Spec Sections:3.3.2
        Description:Tests the "Attribute Default Legal" VC by providing an illegal IDREF value.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/attr09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn attr10() {
    /*
        Test ID:attr10
        Test URI:invalid/attr10.xml
        Spec Sections:3.3.2
        Description:Tests the "Attribute Default Legal" VC by providing an illegal IDREFS value.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/attr10.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn attr11() {
    /*
        Test ID:attr11
        Test URI:invalid/attr11.xml
        Spec Sections:3.3.2
        Description:Tests the "Attribute Default Legal" VC by providing an illegal ENTITY value.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/attr11.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn attr12() {
    /*
        Test ID:attr12
        Test URI:invalid/attr12.xml
        Spec Sections:3.3.2
        Description:Tests the "Attribute Default Legal" VC by providing an illegal ENTITIES value.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/attr12.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn attr13() {
    /*
        Test ID:attr13
        Test URI:invalid/attr13.xml
        Spec Sections:3.3.2
        Description:Tests the "Attribute Default Legal" VC by providing an illegal NMTOKEN value.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/attr13.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn attr14() {
    /*
        Test ID:attr14
        Test URI:invalid/attr14.xml
        Spec Sections:3.3.2
        Description:Tests the "Attribute Default Legal" VC by providing an illegal NMTOKENS value.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/attr14.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn attr15() {
    /*
        Test ID:attr15
        Test URI:invalid/attr15.xml
        Spec Sections:3.3.2
        Description:Tests the "Attribute Default Legal" VC by providing an illegal NOTATIONS value.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/attr15.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn attr16() {
    /*
        Test ID:attr16
        Test URI:invalid/attr16.xml
        Spec Sections:3.3.2
        Description:Tests the "Attribute Default Legal" VC by providing an illegal enumeration value.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/attr16.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn utf16b() {
    /*
        Test ID:utf16b
        Test URI:invalid/utf16b.xml
        Spec Sections:4.3.3 2.8
        Description:Tests reading an invalid "big endian" UTF-16 document
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/utf16b.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn utf16l() {
    /*
        Test ID:utf16l
        Test URI:invalid/utf16l.xml
        Spec Sections:4.3.3 2.8
        Description:Tests reading an invalid "little endian" UTF-16 document
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/utf16l.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn empty() {
    /*
        Test ID:empty
        Test URI:invalid/empty.xml
        Spec Sections:2.4 2.7 [18] 3
        Description:CDATA section containing only white space does not match the nonterminal S, and cannot appear in these positions.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/invalid/empty.xml").unwrap(),
    );

    assert!(testxml.is_err());
}
