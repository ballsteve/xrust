/*

Sun Microsystems test cases

*/
use std::convert::TryFrom;
use std::fs;
use xrust::Document;

#[test]
fn notwfsa03() {
    /*
        Test ID:not-wf-sa03
        Test URI:not-wf/not-sa03.xml
        Spec Sections:2.9
        Description:Tests the Entity Declared WFC, ensuring that a reference to externally defined entity causes a well-formedness error.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/not-sa03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn attlist01() {
    /*
        Test ID:attlist01
        Test URI:not-wf/attlist01.xml
        Spec Sections:3.3.1 [56]
        Description:SGML's NUTOKEN is not allowed.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/attlist01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn attlist02() {
    /*
        Test ID:attlist02
        Test URI:not-wf/attlist02.xml
        Spec Sections:3.3.1 [56]
        Description:SGML's NUTOKENS attribute type is not allowed.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/attlist02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn attlist03() {
    /*
        Test ID:attlist03
        Test URI:not-wf/attlist03.xml
        Spec Sections:3.3.1 [59]
        Description:Comma doesn't separate enumerations, unlike in SGML.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/attlist03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn attlist04() {
    /*
        Test ID:attlist04
        Test URI:not-wf/attlist04.xml
        Spec Sections:3.3.1 [56]
        Description:SGML's NUMBER attribute type is not allowed.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/attlist04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn attlist05() {
    /*
        Test ID:attlist05
        Test URI:not-wf/attlist05.xml
        Spec Sections:3.3.1 [56]
        Description:SGML's NUMBERS attribute type is not allowed.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/attlist05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn attlist06() {
    /*
        Test ID:attlist06
        Test URI:not-wf/attlist06.xml
        Spec Sections:3.3.1 [56]
        Description:SGML's NAME attribute type is not allowed.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/attlist06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn attlist07() {
    /*
        Test ID:attlist07
        Test URI:not-wf/attlist07.xml
        Spec Sections:3.3.1 [56]
        Description:SGML's NAMES attribute type is not allowed.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/attlist07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn attlist08() {
    /*
        Test ID:attlist08
        Test URI:not-wf/attlist08.xml
        Spec Sections:3.3.1 [56]
        Description:SGML's #CURRENT is not allowed.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/attlist08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn attlist09() {
    /*
        Test ID:attlist09
        Test URI:not-wf/attlist09.xml
        Spec Sections:3.3.1 [56]
        Description:SGML's #CONREF is not allowed.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/attlist09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn attlist10() {
    /*
        Test ID:attlist10
        Test URI:not-wf/attlist10.xml
        Spec Sections:3.1 [40]
        Description:Whitespace required between attributes
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/attlist10.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn attlist11() {
    /*
        Test ID:attlist11
        Test URI:not-wf/attlist11.xml
        Spec Sections:3.1 [44]
        Description:Whitespace required between attributes
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/attlist11.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn cond01() {
    /*
        Test ID:cond01
        Test URI:not-wf/cond01.xml
        Spec Sections:3.4 [61]
        Description:Only INCLUDE and IGNORE are conditional section keywords
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/cond01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn cond02() {
    /*
        Test ID:cond02
        Test URI:not-wf/cond02.xml
        Spec Sections:3.4 [61]
        Description:Must have keyword in conditional sections
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/cond02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn content01() {
    /*
        Test ID:content01
        Test URI:not-wf/content01.xml
        Spec Sections:3.2.1 [48]
        Description:No whitespace before "?" in content model
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/content01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn content02() {
    /*
        Test ID:content02
        Test URI:not-wf/content02.xml
        Spec Sections:3.2.1 [48]
        Description:No whitespace before "*" in content model
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/content02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn content03() {
    /*
        Test ID:content03
        Test URI:not-wf/content03.xml
        Spec Sections:3.2.1 [48]
        Description:No whitespace before "+" in content model
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/content03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn decl01() {
    /*
        Test ID:decl01
        Test URI:not-wf/decl01.xml
        Spec Sections:4.3.1 [77]
        Description:External entities may not have standalone decls.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/decl01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn nwfdtd00() {
    /*
        Test ID:nwf-dtd00
        Test URI:not-wf/dtd00.xml
        Spec Sections:3.2.1 [55]
        Description:Comma mandatory in content model
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/dtd00.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn nwfdtd01() {
    /*
        Test ID:nwf-dtd01
        Test URI:not-wf/dtd01.xml
        Spec Sections:3.2.1 [55]
        Description:Can't mix comma and vertical bar in content models
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/dtd01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn dtd02() {
    /*
        Test ID:dtd02
        Test URI:not-wf/dtd02.xml
        Spec Sections:4.1 [69]
        Description:PE name immediately after "%"
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/dtd02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn dtd03() {
    /*
        Test ID:dtd03
        Test URI:not-wf/dtd03.xml
        Spec Sections:4.1 [69]
        Description:PE name immediately followed by ";"
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/dtd03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn dtd04() {
    /*
        Test ID:dtd04
        Test URI:not-wf/dtd04.xml
        Spec Sections:4.2.2 [75]
        Description:PUBLIC literal must be quoted
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/dtd04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn dtd05() {
    /*
        Test ID:dtd05
        Test URI:not-wf/dtd05.xml
        Spec Sections:4.2.2 [75]
        Description:SYSTEM identifier must be quoted
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/dtd05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn dtd07() {
    /*
        Test ID:dtd07
        Test URI:not-wf/dtd07.xml
        Spec Sections:4.3.1 [77]
        Description:Text declarations (which optionally begin any external entity)are required to have "encoding=...".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/dtd07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn element00() {
    /*
        Test ID:element00
        Test URI:not-wf/element00.xml
        Spec Sections:3.1 [42]
        Description:EOF in middle of incomplete ETAG
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/element00.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn element01() {
    /*
        Test ID:element01
        Test URI:not-wf/element01.xml
        Spec Sections:3.1 [42]
        Description:EOF in middle of incomplete ETAG
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/element01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn element02() {
    /*
        Test ID:element02
        Test URI:not-wf/element02.xml
        Spec Sections:3.1 [43]
        Description:Illegal markup (<%@ ... %>)
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/element02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn element03() {
    /*
        Test ID:element03
        Test URI:not-wf/element03.xml
        Spec Sections:3.1 [43]
        Description:Illegal markup (<% ... %>)
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/element03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn element04() {
    /*
        Test ID:element04
        Test URI:not-wf/element04.xml
        Spec Sections:3.1 [43]
        Description:Illegal markup (<!ELEMENT ... >)
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/element04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn encoding01() {
    /*
        Test ID:encoding01
        Test URI:not-wf/encoding01.xml
        Spec Sections:4.3.3 [81]
        Description:Illegal character " " in encoding name
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/encoding01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn encoding02() {
    /*
        Test ID:encoding02
        Test URI:not-wf/encoding02.xml
        Spec Sections:4.3.3 [81]
        Description:Illegal character "/" in encoding name
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/encoding02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn encoding03() {
    /*
        Test ID:encoding03
        Test URI:not-wf/encoding03.xml
        Spec Sections:4.3.3 [81]
        Description:Illegal character reference in encoding name
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/encoding03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn encoding04() {
    /*
        Test ID:encoding04
        Test URI:not-wf/encoding04.xml
        Spec Sections:4.3.3 [81]
        Description:Illegal character ":" in encoding name
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/encoding04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn encoding05() {
    /*
        Test ID:encoding05
        Test URI:not-wf/encoding05.xml
        Spec Sections:4.3.3 [81]
        Description:Illegal character "@" in encoding name
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/encoding05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn encoding06() {
    /*
        Test ID:encoding06
        Test URI:not-wf/encoding06.xml
        Spec Sections:4.3.3 [81]
        Description:Illegal character "+" in encoding name
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/encoding06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn encoding07() {
    /*
        Test ID:encoding07
        Test URI:not-wf/encoding07.xml
        Spec Sections:4.3.1 [77]
        Description:Text declarations (which optionally begin any external entity)are required to have "encoding=...".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/encoding07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn pi() {
    /*
        Test ID:pi
        Test URI:not-wf/pi.xml
        Spec Sections:2.6 [16]
        Description:No space between PI target name and data
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/pi.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn pubid01() {
    /*
        Test ID:pubid01
        Test URI:not-wf/pubid01.xml
        Spec Sections:2.3 [12]
        Description:Illegal entity ref in public ID
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/pubid01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn pubid02() {
    /*
        Test ID:pubid02
        Test URI:not-wf/pubid02.xml
        Spec Sections:2.3 [12]
        Description:Illegal characters in public ID
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/pubid02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn pubid03() {
    /*
        Test ID:pubid03
        Test URI:not-wf/pubid03.xml
        Spec Sections:2.3 [12]
        Description:Illegal characters in public ID
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/pubid03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn pubid04() {
    /*
        Test ID:pubid04
        Test URI:not-wf/pubid04.xml
        Spec Sections:2.3 [12]
        Description:Illegal characters in public ID
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/pubid04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn pubid05() {
    /*
        Test ID:pubid05
        Test URI:not-wf/pubid05.xml
        Spec Sections:2.3 [12]
        Description:SGML-ism: public ID without system ID
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/pubid05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn sgml01() {
    /*
        Test ID:sgml01
        Test URI:not-wf/sgml01.xml
        Spec Sections:3 [39]
        Description:SGML-ism: omitted end tag for EMPTY content
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/sgml01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn sgml02() {
    /*
        Test ID:sgml02
        Test URI:not-wf/sgml02.xml
        Spec Sections:2.8
        Description:XML declaration must be at the very beginning of a document;it"s not a processing instruction
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/sgml02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn sgml03() {
    /*
        Test ID:sgml03
        Test URI:not-wf/sgml03.xml
        Spec Sections:2.5 [15]
        Description:Comments may not contain "--"
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/sgml03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn sgml04() {
    /*
        Test ID:sgml04
        Test URI:not-wf/sgml04.xml
        Spec Sections:3.3 [52]
        Description:ATTLIST declarations apply to only one element, unlike SGML
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/sgml04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn sgml05() {
    /*
        Test ID:sgml05
        Test URI:not-wf/sgml05.xml
        Spec Sections:3.2 [45]
        Description:ELEMENT declarations apply to only one element, unlike SGML
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/sgml05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn sgml06() {
    /*
        Test ID:sgml06
        Test URI:not-wf/sgml06.xml
        Spec Sections:3.3 [52]
        Description:ATTLIST declarations are never global, unlike in SGML
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/sgml06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn sgml07() {
    /*
        Test ID:sgml07
        Test URI:not-wf/sgml07.xml
        Spec Sections:3.2 [45]
        Description:SGML Tag minimization specifications are not allowed
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/sgml07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn sgml08() {
    /*
        Test ID:sgml08
        Test URI:not-wf/sgml08.xml
        Spec Sections:3.2 [45]
        Description:SGML Tag minimization specifications are not allowed
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/sgml08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn sgml09() {
    /*
        Test ID:sgml09
        Test URI:not-wf/sgml09.xml
        Spec Sections:3.2 [45]
        Description:SGML Content model exception specifications are not allowed
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/sgml09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn sgml10() {
    /*
        Test ID:sgml10
        Test URI:not-wf/sgml10.xml
        Spec Sections:3.2 [45]
        Description:SGML Content model exception specifications are not allowed
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/sgml10.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn sgml11() {
    /*
        Test ID:sgml11
        Test URI:not-wf/sgml11.xml
        Spec Sections:3.2 [46]
        Description:CDATA is not a valid content model spec
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/sgml11.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn sgml12() {
    /*
        Test ID:sgml12
        Test URI:not-wf/sgml12.xml
        Spec Sections:3.2 [46]
        Description:RCDATA is not a valid content model spec
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/sgml12.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn sgml13() {
    /*
        Test ID:sgml13
        Test URI:not-wf/sgml13.xml
        Spec Sections:3.2.1 [47]
        Description:SGML Unordered content models not allowed
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/sgml13.xml").unwrap(),
    );

    assert!(testxml.is_err());
}
