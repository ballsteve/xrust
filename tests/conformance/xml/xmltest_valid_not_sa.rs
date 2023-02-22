/*

James Clark XMLTEST cases - Standalone

    This contains cases that are valid XML documents.
    This contains cases that are not standalone.

*/

use std::convert::TryFrom;
use std::fs;
use xrust::Document;

#[test]
#[ignore]
fn validnotsa001() {
    /*
        Test ID:valid-not-sa-001
        Test URI:valid/not-sa/001.xml
        Spec Sections:4.2.2 [75]
        Description:Test demonstrates the use of an ExternalID within a document type definition.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/001.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/001.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa002() {
    /*
        Test ID:valid-not-sa-002
        Test URI:valid/not-sa/002.xml
        Spec Sections:4.2.2 [75]
        Description:Test demonstrates the use of an ExternalID within a document type definition.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/002.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/002.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa003() {
    /*
        Test ID:valid-not-sa-003
        Test URI:valid/not-sa/003.xml
        Spec Sections:4.1 [69]
        Description:Test demonstrates the expansion of an external parameter entity that declares an attribute.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/003.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/003.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa004() {
    /*
        Test ID:valid-not-sa-004
        Test URI:valid/not-sa/004.xml
        Spec Sections:4.1 [69]
        Description:Expands an external parameter entity in two different ways, with one of them declaring an attribute.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/004.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/004.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa005() {
    /*
        Test ID:valid-not-sa-005
        Test URI:valid/not-sa/005.xml
        Spec Sections:4.1 [69]
        Description:Test demonstrates the expansion of an external parameter entity that declares an attribute.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/005.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/005.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa006() {
    /*
        Test ID:valid-not-sa-006
        Test URI:valid/not-sa/006.xml
        Spec Sections:3.3 [52]
        Description:Test demonstrates that when more than one definition is provided for the same attribute of a given element type only the first declaration is binding.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/006.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/006.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa007() {
    /*
        Test ID:valid-not-sa-007
        Test URI:valid/not-sa/007.xml
        Spec Sections:3.3 [52]
        Description:Test demonstrates the use of an Attribute list declaration within an external entity.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/007.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/007.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa008() {
    /*
        Test ID:valid-not-sa-008
        Test URI:valid/not-sa/008.xml
        Spec Sections:4.2.2 [75]
        Description:Test demonstrates that an external identifier may include a public identifier.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/008.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/008.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa009() {
    /*
        Test ID:valid-not-sa-009
        Test URI:valid/not-sa/009.xml
        Spec Sections:4.2.2 [75]
        Description:Test demonstrates that an external identifier may include a public identifier.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/009.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/009.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa010() {
    /*
        Test ID:valid-not-sa-010
        Test URI:valid/not-sa/010.xml
        Spec Sections:3.3 [52]
        Description:Test demonstrates that when more that one definition is provided for the same attribute of a given element type only the first declaration is binding.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/010.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/010.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa011() {
    /*
        Test ID:valid-not-sa-011
        Test URI:valid/not-sa/011.xml
        Spec Sections:4.2 4.2.1 [72] [75]
        Description:Test demonstrates a parameter entity declaration whose parameter entity definition is an ExternalID.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/011.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/011.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa012() {
    /*
        Test ID:valid-not-sa-012
        Test URI:valid/not-sa/012.xml
        Spec Sections:4.3.1 [77]
        Description:Test demonstrates an enternal parsed entity that begins with a text declaration.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/012.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/012.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa013() {
    /*
        Test ID:valid-not-sa-013
        Test URI:valid/not-sa/013.xml
        Spec Sections:3.4 [62]
        Description:Test demonstrates the use of the conditional section INCLUDE that will include its contents as part of the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/013.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/013.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa014() {
    /*
        Test ID:valid-not-sa-014
        Test URI:valid/not-sa/014.xml
        Spec Sections:3.4 [62]
        Description:Test demonstrates the use of the conditional section INCLUDE that will include its contents as part of the DTD. The keyword is a parameter-entity reference.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/014.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/014.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa015() {
    /*
        Test ID:valid-not-sa-015
        Test URI:valid/not-sa/015.xml
        Spec Sections:3.4 [63]
        Description:Test demonstrates the use of the conditonal section IGNORE the will ignore its content from being part of the DTD. The keyword is a parameter-entity reference.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/015.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/015.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa016() {
    /*
        Test ID:valid-not-sa-016
        Test URI:valid/not-sa/016.xml
        Spec Sections:3.4 [62]
        Description:Test demonstrates the use of the conditional section INCLUDE that will include its contents as part of the DTD. The keyword is a parameter-entity reference.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/016.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/016.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa017() {
    /*
        Test ID:valid-not-sa-017
        Test URI:valid/not-sa/017.xml
        Spec Sections:4.2 [72]
        Description:Test demonstrates a parameter entity declaration that contains an attribute list declaration.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/017.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/017.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa018() {
    /*
        Test ID:valid-not-sa-018
        Test URI:valid/not-sa/018.xml
        Spec Sections:4.2.2 [75]
        Description:Test demonstrates an EnternalID whose contents contain an parameter entity declaration and a attribute list definition.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/018.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/018.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa019() {
    /*
        Test ID:valid-not-sa-019
        Test URI:valid/not-sa/019.xml
        Spec Sections:4.4.8
        Description:Test demonstrates that a parameter entity will be expanded with spaces on either side.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/019.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/019.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa020() {
    /*
        Test ID:valid-not-sa-020
        Test URI:valid/not-sa/020.xml
        Spec Sections:4.4.8
        Description:Parameter entities expand with spaces on either side.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/020.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/020.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa021() {
    /*
        Test ID:valid-not-sa-021
        Test URI:valid/not-sa/021.xml
        Spec Sections:4.2 [72]
        Description:Test demonstrates a parameter entity declaration that contains a partial attribute list declaration.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/021.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/021.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa023() {
    /*
        Test ID:valid-not-sa-023
        Test URI:valid/not-sa/023.xml
        Spec Sections:2.3 4.1 [10] [69]
        Description:Test demonstrates the use of a parameter entity reference within an attribute list declaration.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/023.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/023.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa024() {
    /*
        Test ID:valid-not-sa-024
        Test URI:valid/not-sa/024.xml
        Spec Sections:2.8, 4.1 [69]
        Description:Constructs an <!ATTLIST...> declaration from several PEs.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/024.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/024.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa025() {
    /*
        Test ID:valid-not-sa-025
        Test URI:valid/not-sa/025.xml
        Spec Sections:4.2
        Description:Test demonstrates that when more that one definition is provided for the same entity only the first declaration is binding.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/025.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/025.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa026() {
    /*
        Test ID:valid-not-sa-026
        Test URI:valid/not-sa/026.xml
        Spec Sections:3.3 [52]
        Description:Test demonstrates that when more that one definition is provided for the same attribute of a given element type only the first declaration is binding.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/026.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/026.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa027() {
    /*
        Test ID:valid-not-sa-027
        Test URI:valid/not-sa/027.xml
        Spec Sections:4.1 [69]
        Description:Test demonstrates a parameter entity reference whose value is NULL.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/027.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/027.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa028() {
    /*
        Test ID:valid-not-sa-028
        Test URI:valid/not-sa/028.xml
        Spec Sections:3.4 [62]
        Description:Test demonstrates the use of the conditional section INCLUDE that will include its contents.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/028.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/028.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa029() {
    /*
        Test ID:valid-not-sa-029
        Test URI:valid/not-sa/029.xml
        Spec Sections:3.4 [62]
        Description:Test demonstrates the use of the conditonal section IGNORE the will ignore its content from being used.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/029.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/029.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa030() {
    /*
        Test ID:valid-not-sa-030
        Test URI:valid/not-sa/030.xml
        Spec Sections:3.4 [62]
        Description:Test demonstrates the use of the conditonal section IGNORE the will ignore its content from being used.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/030.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/030.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}

#[test]
#[ignore]
fn validnotsa031() {
    /*
        Test ID:valid-not-sa-031
        Test URI:valid/not-sa/031.xml
        Spec Sections:2.7
        Description:Expands a general entity which contains a CDATA section with what looks like a markup declaration (but is just text since it's in a CDATA section).
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/031.xml").unwrap(),
    );
    let canonicalxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/valid/not-sa/out/031.xml")
            .unwrap(),
    );

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());
}
