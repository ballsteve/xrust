use crate::conformance::dtdfileresolve;
use std::fs;
use std::rc::Rc;
use xrust::parser::{xml, ParserConfig};
use xrust::trees::smite::Node as SmiteNode;
use xrust::Node;

#[test]
#[ignore]
fn pe01() {
    /*
        Test ID:pe01
        Test URI:valid/pe01.xml
        Spec Sections:2.8
        Description:    Parameter entities references are NOT RECOGNIZED in default attribute    values.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/pe01.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn dtd00() {
    /*
        Test ID:dtd00
        Test URI:valid/dtd00.xml
        Spec Sections:3.2.2 [51]
        Description:Tests parsing of alternative forms of text-only mixedcontent declaration.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/dtd00.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/dtd00.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn dtd01() {
    /*
        Test ID:dtd01
        Test URI:valid/dtd01.xml
        Spec Sections:2.5 [15]
        Description:Comments don't get parameter entity expansion
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/dtd01.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/dtd01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn element() {
    /*
        Test ID:element
        Test URI:valid/element.xml
        Spec Sections:3
        Description:Tests clauses 1, 3, and 4 of the Element Validvalidity constraint.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/element.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/element.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn ext01() {
    /*
        Test ID:ext01
        Test URI:valid/ext01.xml
        Spec Sections:4.3.1 4.3.2 [77] [78]
        Description:Tests use of external parsed entities with and without content.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/ext01.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/ext01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn ext02() {
    /*
        Test ID:ext02
        Test URI:valid/ext02.xml
        Spec Sections:4.3.2 [78]
        Description:Tests use of external parsed entities with differentencodings than the base document.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/ext02.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/ext02.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn notsa01() {
    /*
        Test ID:not-sa01
        Test URI:valid/not-sa01.xml
        Spec Sections:2.9
        Description:A non-standalone document is valid if declared as such.
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/sun/valid/".to_string());

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/not-sa01.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/not-sa01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn notsa02() {
    /*
        Test ID:not-sa02
        Test URI:valid/not-sa02.xml
        Spec Sections:2.9
        Description:A non-standalone document is valid if declared as such.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/not-sa02.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/not-sa02.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn notsa03() {
    /*
        Test ID:not-sa03
        Test URI:valid/not-sa03.xml
        Spec Sections:2.9
        Description:A non-standalone document is valid if declared as such.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/not-sa03.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/not-sa03.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn notsa04() {
    /*
        Test ID:not-sa04
        Test URI:valid/not-sa04.xml
        Spec Sections:2.9
        Description:A non-standalone document is valid if declared as such.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/not-sa04.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/not-sa04.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn notation01() {
    /*
        Test ID:notation01
        Test URI:valid/notation01.xml
        Spec Sections:4.7 [82]
        Description:NOTATION declarations don't need SYSTEM IDs; andexternally declared notations may be used to declareunparsed entities in the internal DTD subset.The notation must be reported to the application.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/notation01.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/notation01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn optional() {
    /*
        Test ID:optional
        Test URI:valid/optional.xml
        Spec Sections:3 3.2.1 [47]
        Description:Tests declarations of "children" content models, andthe validity constraints associated with them.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/optional.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/optional.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn required00() {
    /*
        Test ID:required00
        Test URI:valid/required00.xml
        Spec Sections:3.3.2 [60]
        Description:Tests the #REQUIRED attribute declaration syntax, andthe associated validity constraint.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/required00.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/required00.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn sa01() {
    /*
        Test ID:sa01
        Test URI:valid/sa01.xml
        Spec Sections:2.9 [32]
        Description:A document may be marked 'standalone' if any optional whitespace is defined within the internal DTD subset.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/sa01.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/sa01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn sa02() {
    /*
        Test ID:sa02
        Test URI:valid/sa02.xml
        Spec Sections:2.9 [32]
        Description:A document may be marked 'standalone' if anyattributes that need normalization aredefined within the internal DTD subset.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/sa02.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/sa02.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn sa03() {
    /*
        Test ID:sa03
        Test URI:valid/sa03.xml
        Spec Sections:2.9 [32]
        Description:A document may be marked 'standalone' if anythe defined entities need expanding are internal,and no attributes need defaulting or normalization.On output, requires notations to be correctly reported.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/sa03.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/sa03.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn sa04() {
    /*
        Test ID:sa04
        Test URI:valid/sa04.xml
        Spec Sections:2.9 [32]
        Description:Like sa03 but relies on attributedefaulting defined in the internal subset.On output, requires notations to be correctly reported.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/sa04.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/sa04.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn sa05() {
    /*
        Test ID:sa05
        Test URI:valid/sa05.xml
        Spec Sections:2.9 [32]
        Description:Like sa01 but this document is standalone since it has no optional whitespace.On output, requires notations to be correctly reported.
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/sun/valid/".to_string());

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/sa05.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/sa05.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn vsgml01() {
    /*
        Test ID:v-sgml01
        Test URI:valid/sgml01.xml
        Spec Sections:3.3.1 [59]
        Description:XML permits token reuse, while SGML does not.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/sgml01.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/sgml01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn vlang01() {
    /*
        Test ID:v-lang01
        Test URI:valid/v-lang01.xml
        Spec Sections:2.12 [35]
        Description:Tests a lowercase ISO language code.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/v-lang01.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/v-lang01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn vlang02() {
    /*
        Test ID:v-lang02
        Test URI:valid/v-lang02.xml
        Spec Sections:2.12 [35]
        Description:Tests a ISO language code with a subcode.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/v-lang02.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/v-lang02.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn vlang03() {
    /*
        Test ID:v-lang03
        Test URI:valid/v-lang03.xml
        Spec Sections:2.12 [36]
        Description:Tests a IANA language code with a subcode.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/v-lang03.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/v-lang03.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn vlang04() {
    /*
        Test ID:v-lang04
        Test URI:valid/v-lang04.xml
        Spec Sections:2.12 [37]
        Description:Tests a user language code with a subcode.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/v-lang04.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/v-lang04.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn vlang05() {
    /*
        Test ID:v-lang05
        Test URI:valid/v-lang05.xml
        Spec Sections:2.12 [35]
        Description:Tests an uppercase ISO language code.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/v-lang05.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/v-lang05.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn vlang06() {
    /*
        Test ID:v-lang06
        Test URI:valid/v-lang06.xml
        Spec Sections:2.12 [37]
        Description:Tests a user language code.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/v-lang06.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/v-lang06.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn vpe00() {
    /*
        Test ID:v-pe00
        Test URI:valid/pe00.xml
        Spec Sections:4.5
        Description:Tests construction of internal entity replacement text, usingan example in the XML specification.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/pe00.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/pe00.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
fn vpe03() {
    /*
        Test ID:v-pe03
        Test URI:valid/pe03.xml
        Spec Sections:4.5
        Description:Tests construction of internal entity replacement text, usingan example in the XML specification.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/pe03.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/pe03.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}

#[test]
#[ignore]
fn vpe02() {
    /*
        Test ID:v-pe02
        Test URI:valid/pe02.xml
        Spec Sections:4.5
        Description:Tests construction of internal entity replacement text, usinga complex example in the XML specification.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/pe02.xml")
            .unwrap()
            .as_str(),
        None,
    );
    let canonicalxml = Rc::new(SmiteNode::new());
    let canonicalparseresult = xml::parse(
        canonicalxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/sun/valid/out/pe02.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());
    assert_eq!(parseresult.unwrap().get_canonical().unwrap(), canonicalparseresult.unwrap());
}
