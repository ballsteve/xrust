/*

James Clark XMLTEST cases - Standalone

    This contains cases that are not well-formed XML documents
    This contains cases that are not standalone.

*/

use std::fs;
use std::rc::Rc;
use xrust::parser::xml;
use xrust::trees::smite::Node as SmiteNode;

#[test]
fn notwfsa001() {
    /*
        Test ID:not-wf-sa-001
        Test URI:not-wf/sa/001.xml
        Spec Sections:3.1 [41]
        Description:Attribute values must start with attribute names, not "?".
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/001.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa002() {
    /*
        Test ID:not-wf-sa-002
        Test URI:not-wf/sa/002.xml
        Spec Sections:2.3 [4]
        Description:Names may not start with "."; it's not a Letter.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/002.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa003() {
    /*
        Test ID:not-wf-sa-003
        Test URI:not-wf/sa/003.xml
        Spec Sections:2.6 [16]
        Description:Processing Instruction target name is required.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/003.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa004() {
    /*
        Test ID:not-wf-sa-004
        Test URI:not-wf/sa/004.xml
        Spec Sections:2.6 [16]
        Description:SGML-ism: processing instructions end in '?>' not '>'.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/004.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa005() {
    /*
        Test ID:not-wf-sa-005
        Test URI:not-wf/sa/005.xml
        Spec Sections:2.6 [16]
        Description:Processing instructions end in '?>' not '?'.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/005.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa006() {
    /*
        Test ID:not-wf-sa-006
        Test URI:not-wf/sa/006.xml
        Spec Sections:2.5 [16]
        Description:XML comments may not contain "--"
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/006.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa007() {
    /*
        Test ID:not-wf-sa-007
        Test URI:not-wf/sa/007.xml
        Spec Sections:4.1 [68]
        Description:General entity references have no whitespace after the entity name and before the semicolon.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/007.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa008() {
    /*
        Test ID:not-wf-sa-008
        Test URI:not-wf/sa/008.xml
        Spec Sections:2.3 [5]
        Description:Entity references must include names, which don't begin with '.' (it's not a Letter or other name start character).
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/008.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa009() {
    /*
        Test ID:not-wf-sa-009
        Test URI:not-wf/sa/009.xml
        Spec Sections:4.1 [66]
        Description:Character references may have only decimal or numeric strings.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/009.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa010() {
    /*
        Test ID:not-wf-sa-010
        Test URI:not-wf/sa/010.xml
        Spec Sections:4.1 [68]
        Description:Ampersand may only appear as part of a general entity reference.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/010.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa011() {
    /*
        Test ID:not-wf-sa-011
        Test URI:not-wf/sa/011.xml
        Spec Sections:3.1 [41]
        Description:SGML-ism: attribute values must be explicitly assigned a value, it can't act as a boolean toggle.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/011.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa012() {
    /*
        Test ID:not-wf-sa-012
        Test URI:not-wf/sa/012.xml
        Spec Sections:2.3 [10]
        Description:SGML-ism: attribute values must be quoted in all cases.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/012.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa013() {
    /*
        Test ID:not-wf-sa-013
        Test URI:not-wf/sa/013.xml
        Spec Sections:2.3 [10]
        Description:The quotes on both ends of an attribute value must match.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/013.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa014() {
    /*
        Test ID:not-wf-sa-014
        Test URI:not-wf/sa/014.xml
        Spec Sections:2.3 [10]
        Description:Attribute values may not contain literal '<' characters.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/014.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa015() {
    /*
        Test ID:not-wf-sa-015
        Test URI:not-wf/sa/015.xml
        Spec Sections:3.1 [41]
        Description:Attribute values need a value, not just an equals sign.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/015.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa016() {
    /*
        Test ID:not-wf-sa-016
        Test URI:not-wf/sa/016.xml
        Spec Sections:3.1 [41]
        Description:Attribute values need an associated name.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/016.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa017() {
    /*
        Test ID:not-wf-sa-017
        Test URI:not-wf/sa/017.xml
        Spec Sections:2.7 [18]
        Description:CDATA sections need a terminating ']]>'.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/017.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa018() {
    /*
        Test ID:not-wf-sa-018
        Test URI:not-wf/sa/018.xml
        Spec Sections:2.7 [19]
        Description:CDATA sections begin with a literal '<![CDATA[', no space.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/018.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa019() {
    /*
        Test ID:not-wf-sa-019
        Test URI:not-wf/sa/019.xml
        Spec Sections:3.1 [42]
        Description:End tags may not be abbreviated as '</>'.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/019.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa020() {
    /*
        Test ID:not-wf-sa-020
        Test URI:not-wf/sa/020.xml
        Spec Sections:2.3 [10]
        Description:Attribute values may not contain literal '&' characters except as part of an entity reference.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/020.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa021() {
    /*
        Test ID:not-wf-sa-021
        Test URI:not-wf/sa/021.xml
        Spec Sections:2.3 [10]
        Description:Attribute values may not contain literal '&' characters except as part of an entity reference.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/021.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa022() {
    /*
        Test ID:not-wf-sa-022
        Test URI:not-wf/sa/022.xml
        Spec Sections:4.1 [66]
        Description:Character references end with semicolons, always!
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/022.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa023() {
    /*
        Test ID:not-wf-sa-023
        Test URI:not-wf/sa/023.xml
        Spec Sections:2.3 [5]
        Description:Digits are not valid name start characters.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/023.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa024() {
    /*
        Test ID:not-wf-sa-024
        Test URI:not-wf/sa/024.xml
        Spec Sections:2.3 [5]
        Description:Digits are not valid name start characters.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/024.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa025() {
    /*
        Test ID:not-wf-sa-025
        Test URI:not-wf/sa/025.xml
        Spec Sections:2.4 [14]
        Description:Text may not contain a literal ']]>' sequence.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/025.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa026() {
    /*
        Test ID:not-wf-sa-026
        Test URI:not-wf/sa/026.xml
        Spec Sections:2.4 [14]
        Description:Text may not contain a literal ']]>' sequence.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/026.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa027() {
    /*
        Test ID:not-wf-sa-027
        Test URI:not-wf/sa/027.xml
        Spec Sections:2.5 [15]
        Description:Comments must be terminated with "-->".
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/027.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa028() {
    /*
        Test ID:not-wf-sa-028
        Test URI:not-wf/sa/028.xml
        Spec Sections:2.6 [16]
        Description:Processing instructions must end with '?>'.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/028.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa029() {
    /*
        Test ID:not-wf-sa-029
        Test URI:not-wf/sa/029.xml
        Spec Sections:2.4 [14]
        Description:Text may not contain a literal ']]>' sequence.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/029.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa030() {
    /*
        Test ID:not-wf-sa-030
        Test URI:not-wf/sa/030.xml
        Spec Sections:2.2 [2]
        Description:A form feed is not a legal XML character.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/030.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa031() {
    /*
        Test ID:not-wf-sa-031
        Test URI:not-wf/sa/031.xml
        Spec Sections:2.2 [2]
        Description:A form feed is not a legal XML character.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/031.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa032() {
    /*
        Test ID:not-wf-sa-032
        Test URI:not-wf/sa/032.xml
        Spec Sections:2.2 [2]
        Description:A form feed is not a legal XML character.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/032.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa033() {
    /*
        Test ID:not-wf-sa-033
        Test URI:not-wf/sa/033.xml
        Spec Sections:2.2 [2]
        Description:An ESC (octal 033) is not a legal XML character.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/033.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa034() {
    /*
        Test ID:not-wf-sa-034
        Test URI:not-wf/sa/034.xml
        Spec Sections:2.2 [2]
        Description:A form feed is not a legal XML character.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/034.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa035() {
    /*
        Test ID:not-wf-sa-035
        Test URI:not-wf/sa/035.xml
        Spec Sections:3.1 [43]
        Description:The '<' character is a markup delimiter and must start an element, CDATA section, PI, or comment.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/035.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa036() {
    /*
        Test ID:not-wf-sa-036
        Test URI:not-wf/sa/036.xml
        Spec Sections:2.8 [27]
        Description:Text may not appear after the root element.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/036.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa037() {
    /*
        Test ID:not-wf-sa-037
        Test URI:not-wf/sa/037.xml
        Spec Sections:2.8 [27]
        Description:Character references may not appear after the root element.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/037.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa038() {
    /*
        Test ID:not-wf-sa-038
        Test URI:not-wf/sa/038.xml
        Spec Sections:3.1
        Description:Tests the "Unique Att Spec" WF constraint by providing multiple values for an attribute.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/038.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa039() {
    /*
        Test ID:not-wf-sa-039
        Test URI:not-wf/sa/039.xml
        Spec Sections:3
        Description:Tests the Element Type Match WFC - end tag name must match start tag name.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/039.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa040() {
    /*
        Test ID:not-wf-sa-040
        Test URI:not-wf/sa/040.xml
        Spec Sections:2.8 [27]
        Description:Provides two document elements.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/040.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa041() {
    /*
        Test ID:not-wf-sa-041
        Test URI:not-wf/sa/041.xml
        Spec Sections:2.8 [27]
        Description:Provides two document elements.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/041.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa042() {
    /*
        Test ID:not-wf-sa-042
        Test URI:not-wf/sa/042.xml
        Spec Sections:3.1 [42]
        Description:Invalid End Tag
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/042.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa043() {
    /*
        Test ID:not-wf-sa-043
        Test URI:not-wf/sa/043.xml
        Spec Sections:2.8 [27]
        Description:Provides #PCDATA text after the document element.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/043.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa044() {
    /*
        Test ID:not-wf-sa-044
        Test URI:not-wf/sa/044.xml
        Spec Sections:2.8 [27]
        Description:Provides two document elements.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/044.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa045() {
    /*
        Test ID:not-wf-sa-045
        Test URI:not-wf/sa/045.xml
        Spec Sections:3.1 [44]
        Description:Invalid Empty Element Tag
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/045.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa046() {
    /*
        Test ID:not-wf-sa-046
        Test URI:not-wf/sa/046.xml
        Spec Sections:3.1 [40]
        Description:This start (or empty element) tag was not terminated correctly.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/046.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa047() {
    /*
        Test ID:not-wf-sa-047
        Test URI:not-wf/sa/047.xml
        Spec Sections:3.1 [44]
        Description:Invalid empty element tag invalid whitespace
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/047.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa048() {
    /*
        Test ID:not-wf-sa-048
        Test URI:not-wf/sa/048.xml
        Spec Sections:2.8 [27]
        Description:Provides a CDATA section after the root element.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/048.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa049() {
    /*
        Test ID:not-wf-sa-049
        Test URI:not-wf/sa/049.xml
        Spec Sections:3.1 [40]
        Description:Missing start tag
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/049.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa050() {
    /*
        Test ID:not-wf-sa-050
        Test URI:not-wf/sa/050.xml
        Spec Sections:2.1 [1]
        Description:Empty document, with no root element.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/050.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa051() {
    /*
        Test ID:not-wf-sa-051
        Test URI:not-wf/sa/051.xml
        Spec Sections:2.7 [18]
        Description:CDATA is invalid at top level of document.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/051.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa052() {
    /*
        Test ID:not-wf-sa-052
        Test URI:not-wf/sa/052.xml
        Spec Sections:4.1 [66]
        Description:Invalid character reference.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/052.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa053() {
    /*
        Test ID:not-wf-sa-053
        Test URI:not-wf/sa/053.xml
        Spec Sections:3.1 [42]
        Description:End tag does not match start tag.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/053.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa054() {
    /*
        Test ID:not-wf-sa-054
        Test URI:not-wf/sa/054.xml
        Spec Sections:4.2.2 [75]
        Description:PUBLIC requires two literals.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/054.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa055() {
    /*
        Test ID:not-wf-sa-055
        Test URI:not-wf/sa/055.xml
        Spec Sections:2.8 [28]
        Description:Invalid Document Type Definition format.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/055.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa056() {
    /*
        Test ID:not-wf-sa-056
        Test URI:not-wf/sa/056.xml
        Spec Sections:2.8 [28]
        Description:Invalid Document Type Definition format - misplaced comment.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/056.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa057() {
    /*
        Test ID:not-wf-sa-057
        Test URI:not-wf/sa/057.xml
        Spec Sections:3.2 [45]
        Description:This isn't SGML; comments can't exist in declarations.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/057.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa058() {
    /*
        Test ID:not-wf-sa-058
        Test URI:not-wf/sa/058.xml
        Spec Sections:3.3.1 [54]
        Description:Invalid character , in ATTLIST enumeration
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/058.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa059() {
    /*
        Test ID:not-wf-sa-059
        Test URI:not-wf/sa/059.xml
        Spec Sections:3.3.1 [59]
        Description:String literal must be in quotes.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/059.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa060() {
    /*
        Test ID:not-wf-sa-060
        Test URI:not-wf/sa/060.xml
        Spec Sections:3.3.1 [56]
        Description:Invalid type NAME defined in ATTLIST.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/060.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa061() {
    /*
        Test ID:not-wf-sa-061
        Test URI:not-wf/sa/061.xml
        Spec Sections:4.2.2 [75]
        Description:External entity declarations require whitespace between public and system IDs.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/061.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa062() {
    /*
        Test ID:not-wf-sa-062
        Test URI:not-wf/sa/062.xml
        Spec Sections:4.2 [71]
        Description:Entity declarations need space after the entity name.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/062.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa063() {
    /*
        Test ID:not-wf-sa-063
        Test URI:not-wf/sa/063.xml
        Spec Sections:2.8 [29]
        Description:Conditional sections may only appear in the external DTD subset.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/063.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa064() {
    /*
        Test ID:not-wf-sa-064
        Test URI:not-wf/sa/064.xml
        Spec Sections:3.3 [53]
        Description:Space is required between attribute type and default values in <!ATTLIST...> declarations.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/064.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa065() {
    /*
        Test ID:not-wf-sa-065
        Test URI:not-wf/sa/065.xml
        Spec Sections:3.3 [53]
        Description:Space is required between attribute name and type in <!ATTLIST...> declarations.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/065.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa066() {
    /*
        Test ID:not-wf-sa-066
        Test URI:not-wf/sa/066.xml
        Spec Sections:3.3 [52]
        Description:Required whitespace is missing.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/066.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa067() {
    /*
        Test ID:not-wf-sa-067
        Test URI:not-wf/sa/067.xml
        Spec Sections:3.3 [53]
        Description:Space is required between attribute type and default values in <!ATTLIST...> declarations.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/067.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa068() {
    /*
        Test ID:not-wf-sa-068
        Test URI:not-wf/sa/068.xml
        Spec Sections:3.3.1 [58]
        Description:Space is required between NOTATION keyword and list of enumerated choices in <!ATTLIST...> declarations.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/068.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa069() {
    /*
        Test ID:not-wf-sa-069
        Test URI:not-wf/sa/069.xml
        Spec Sections:4.2.2 [76]
        Description:Space is required before an NDATA entity annotation.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/069.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa070() {
    /*
        Test ID:not-wf-sa-070
        Test URI:not-wf/sa/070.xml
        Spec Sections:2.5 [16]
        Description:XML comments may not contain "--"
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/070.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa071() {
    /*
        Test ID:not-wf-sa-071
        Test URI:not-wf/sa/071.xml
        Spec Sections:4.1 [68]
        Description:ENTITY can't reference itself directly or indirectly.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/071.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa072() {
    /*
        Test ID:not-wf-sa-072
        Test URI:not-wf/sa/072.xml
        Spec Sections:4.1 [68]
        Description:Undefined ENTITY foo.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/072.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa073() {
    /*
        Test ID:not-wf-sa-073
        Test URI:not-wf/sa/073.xml
        Spec Sections:4.1 [68]
        Description:Undefined ENTITY f.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/073.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa074() {
    /*
        Test ID:not-wf-sa-074
        Test URI:not-wf/sa/074.xml
        Spec Sections:4.3.2
        Description:Internal general parsed entities are only well formed if they match the "content" production.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/074.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa075() {
    /*
        Test ID:not-wf-sa-075
        Test URI:not-wf/sa/075.xml
        Spec Sections:4.1 [68]
        Description:ENTITY can't reference itself directly or indirectly.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/075.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa076() {
    /*
        Test ID:not-wf-sa-076
        Test URI:not-wf/sa/076.xml
        Spec Sections:4.1 [68]
        Description:Undefined ENTITY foo.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/076.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa077() {
    /*
        Test ID:not-wf-sa-077
        Test URI:not-wf/sa/077.xml
        Spec Sections:41. [68]
        Description:Undefined ENTITY bar.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/077.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa078() {
    /*
        Test ID:not-wf-sa-078
        Test URI:not-wf/sa/078.xml
        Spec Sections:4.1 [68]
        Description:Undefined ENTITY foo.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/078.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa079() {
    /*
        Test ID:not-wf-sa-079
        Test URI:not-wf/sa/079.xml
        Spec Sections:4.1 [68]
        Description:ENTITY can't reference itself directly or indirectly.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/079.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa080() {
    /*
        Test ID:not-wf-sa-080
        Test URI:not-wf/sa/080.xml
        Spec Sections:4.1 [68]
        Description:ENTITY can't reference itself directly or indirectly.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/080.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa081() {
    /*
        Test ID:not-wf-sa-081
        Test URI:not-wf/sa/081.xml
        Spec Sections:3.1
        Description:This tests the No External Entity References WFC, since the entity is referred to within an attribute.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/081.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa082() {
    /*
        Test ID:not-wf-sa-082
        Test URI:not-wf/sa/082.xml
        Spec Sections:3.1
        Description:This tests the No External Entity References WFC, since the entity is referred to within an attribute.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/082.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa083() {
    /*
        Test ID:not-wf-sa-083
        Test URI:not-wf/sa/083.xml
        Spec Sections:4.2.2 [76]
        Description:Undefined NOTATION n.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/083.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa084() {
    /*
        Test ID:not-wf-sa-084
        Test URI:not-wf/sa/084.xml
        Spec Sections:4.1
        Description:Tests the Parsed Entity WFC by referring to an unparsed entity. (This precedes the error of not declaring that entity's notation, which may be detected any time before the DTD parsing is completed.)
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/084.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa085() {
    /*
        Test ID:not-wf-sa-085
        Test URI:not-wf/sa/085.xml
        Spec Sections:2.3 [13]
        Description:Public IDs may not contain "[".
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/085.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa086() {
    /*
        Test ID:not-wf-sa-086
        Test URI:not-wf/sa/086.xml
        Spec Sections:2.3 [13]
        Description:Public IDs may not contain "[".
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/086.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa087() {
    /*
        Test ID:not-wf-sa-087
        Test URI:not-wf/sa/087.xml
        Spec Sections:2.3 [13]
        Description:Public IDs may not contain "[".
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/087.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa088() {
    /*
        Test ID:not-wf-sa-088
        Test URI:not-wf/sa/088.xml
        Spec Sections:2.3 [10]
        Description:Attribute values are terminated by literal quote characters, and any entity expansion is done afterwards.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/088.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa089() {
    /*
        Test ID:not-wf-sa-089
        Test URI:not-wf/sa/089.xml
        Spec Sections:4.2 [74]
        Description:Parameter entities "are" always parsed; NDATA annotations are not permitted.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/089.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa090() {
    /*
        Test ID:not-wf-sa-090
        Test URI:not-wf/sa/090.xml
        Spec Sections:2.3 [10]
        Description:Attributes may not contain a literal "<" character; this one has one because of reference expansion.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/090.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa091() {
    /*
        Test ID:not-wf-sa-091
        Test URI:not-wf/sa/091.xml
        Spec Sections:4.2 [74]
        Description:Parameter entities "are" always parsed; NDATA annotations are not permitted.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/091.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa092() {
    /*
        Test ID:not-wf-sa-092
        Test URI:not-wf/sa/092.xml
        Spec Sections:4.5
        Description:The replacement text of this entity has an illegal reference, because the character reference is expanded immediately.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/092.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa093() {
    /*
        Test ID:not-wf-sa-093
        Test URI:not-wf/sa/093.xml
        Spec Sections:4.1 [66]
        Description:Hexadecimal character references may not use the uppercase 'X'.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/093.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa094() {
    /*
        Test ID:not-wf-sa-094
        Test URI:not-wf/sa/094.xml
        Spec Sections:2.8 [24]
        Description:Prolog VERSION must be lowercase.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/094.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa095() {
    /*
        Test ID:not-wf-sa-095
        Test URI:not-wf/sa/095.xml
        Spec Sections:2.8 [23]
        Description:VersionInfo must come before EncodingDecl.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/095.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa096() {
    /*
        Test ID:not-wf-sa-096
        Test URI:not-wf/sa/096.xml
        Spec Sections:2.9 [32]
        Description:Space is required before the standalone declaration.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/096.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa097() {
    /*
        Test ID:not-wf-sa-097
        Test URI:not-wf/sa/097.xml
        Spec Sections:2.8 [24]
        Description:Both quotes surrounding VersionNum must be the same.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/097.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa098() {
    /*
        Test ID:not-wf-sa-098
        Test URI:not-wf/sa/098.xml
        Spec Sections:2.8 [23]
        Description:Only one "version=..." string may appear in an XML declaration.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/098.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa099() {
    /*
        Test ID:not-wf-sa-099
        Test URI:not-wf/sa/099.xml
        Spec Sections:2.8 [23]
        Description:Only three pseudo-attributes are in the XML declaration, and "valid=..." is not one of them.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/099.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa100() {
    /*
        Test ID:not-wf-sa-100
        Test URI:not-wf/sa/100.xml
        Spec Sections:2.9 [32]
        Description:Only "yes" and "no" are permitted as values of "standalone".
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/100.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa101() {
    /*
        Test ID:not-wf-sa-101
        Test URI:not-wf/sa/101.xml
        Spec Sections:4.3.3 [81]
        Description:Space is not permitted in an encoding name.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/101.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa102() {
    /*
        Test ID:not-wf-sa-102
        Test URI:not-wf/sa/102.xml
        Spec Sections:2.8 [26]
        Description:Provides an illegal XML version number; spaces are illegal.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/102.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa103() {
    /*
        Test ID:not-wf-sa-103
        Test URI:not-wf/sa/103.xml
        Spec Sections:4.3.2
        Description:End-tag required for element foo.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/103.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa104() {
    /*
        Test ID:not-wf-sa-104
        Test URI:not-wf/sa/104.xml
        Spec Sections:4.3.2
        Description:Internal general parsed entities are only well formed if they match the "content" production.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/104.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa105() {
    /*
        Test ID:not-wf-sa-105
        Test URI:not-wf/sa/105.xml
        Spec Sections:2.7
        Description:Invalid placement of CDATA section.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/105.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa106() {
    /*
        Test ID:not-wf-sa-106
        Test URI:not-wf/sa/106.xml
        Spec Sections:4.2
        Description:Invalid placement of entity declaration.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/106.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa107() {
    /*
        Test ID:not-wf-sa-107
        Test URI:not-wf/sa/107.xml
        Spec Sections:2.8 [28]
        Description:Invalid document type declaration. CDATA alone is invalid.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/107.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa108() {
    /*
        Test ID:not-wf-sa-108
        Test URI:not-wf/sa/108.xml
        Spec Sections:2.7 [19]
        Description:No space in '<![CDATA['.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/108.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa109() {
    /*
        Test ID:not-wf-sa-109
        Test URI:not-wf/sa/109.xml
        Spec Sections:4.2 [70]
        Description:Tags invalid within EntityDecl.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/109.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa110() {
    /*
        Test ID:not-wf-sa-110
        Test URI:not-wf/sa/110.xml
        Spec Sections:4.1 [68]
        Description:Entity reference must be in content of element.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/110.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa111() {
    /*
        Test ID:not-wf-sa-111
        Test URI:not-wf/sa/111.xml
        Spec Sections:3.1 [43]
        Description:Entiry reference must be in content of element not Start-tag.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/111.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa112() {
    /*
        Test ID:not-wf-sa-112
        Test URI:not-wf/sa/112.xml
        Spec Sections:2.7 [19]
        Description:CDATA sections start '<![CDATA[', not '<!cdata['.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/112.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa113() {
    /*
        Test ID:not-wf-sa-113
        Test URI:not-wf/sa/113.xml
        Spec Sections:2.3 [9]
        Description:Parameter entity values must use valid reference syntax; this reference is malformed.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/113.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa114() {
    /*
        Test ID:not-wf-sa-114
        Test URI:not-wf/sa/114.xml
        Spec Sections:2.3 [9]
        Description:General entity values must use valid reference syntax; this reference is malformed.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/114.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa115() {
    /*
        Test ID:not-wf-sa-115
        Test URI:not-wf/sa/115.xml
        Spec Sections:4.5
        Description:The replacement text of this entity is an illegal character reference, which must be rejected when it is parsed in the context of an attribute value.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/115.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa116() {
    /*
        Test ID:not-wf-sa-116
        Test URI:not-wf/sa/116.xml
        Spec Sections:4.3.2
        Description:Internal general parsed entities are only well formed if they match the "content" production. This is a partial character reference, not a full one.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/116.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa117() {
    /*
        Test ID:not-wf-sa-117
        Test URI:not-wf/sa/117.xml
        Spec Sections:4.3.2
        Description:Internal general parsed entities are only well formed if they match the "content" production. This is a partial character reference, not a full one.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/117.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa118() {
    /*
        Test ID:not-wf-sa-118
        Test URI:not-wf/sa/118.xml
        Spec Sections:4.1 [68]
        Description:Entity reference expansion is not recursive.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/118.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa119() {
    /*
        Test ID:not-wf-sa-119
        Test URI:not-wf/sa/119.xml
        Spec Sections:4.3.2
        Description:Internal general parsed entities are only well formed if they match the "content" production. This is a partial character reference, not a full one.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/119.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa120() {
    /*
        Test ID:not-wf-sa-120
        Test URI:not-wf/sa/120.xml
        Spec Sections:4.5
        Description:Character references are expanded in the replacement text of an internal entity, which is then parsed as usual. Accordingly, & must be doubly quoted - encoded either as &amp; or as &#38;#38;.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/120.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa121() {
    /*
        Test ID:not-wf-sa-121
        Test URI:not-wf/sa/121.xml
        Spec Sections:4.1 [68]
        Description:A name of an ENTITY was started with an invalid character.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/121.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa122() {
    /*
        Test ID:not-wf-sa-122
        Test URI:not-wf/sa/122.xml
        Spec Sections:3.2.1 [47]
        Description:Invalid syntax mixed connectors are used.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/122.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa123() {
    /*
        Test ID:not-wf-sa-123
        Test URI:not-wf/sa/123.xml
        Spec Sections:3.2.1 [48]
        Description:Invalid syntax mismatched parenthesis.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/123.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa124() {
    /*
        Test ID:not-wf-sa-124
        Test URI:not-wf/sa/124.xml
        Spec Sections:3.2.2 [51]
        Description:Invalid format of Mixed-content declaration.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/124.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa125() {
    /*
        Test ID:not-wf-sa-125
        Test URI:not-wf/sa/125.xml
        Spec Sections:3.2.2 [51]
        Description:Invalid syntax extra set of parenthesis not necessary.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/125.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa126() {
    /*
        Test ID:not-wf-sa-126
        Test URI:not-wf/sa/126.xml
        Spec Sections:3.2.2 [51]
        Description:Invalid syntax Mixed-content must be defined as zero or more.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/126.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa127() {
    /*
        Test ID:not-wf-sa-127
        Test URI:not-wf/sa/127.xml
        Spec Sections:3.2.2 [51]
        Description:Invalid syntax Mixed-content must be defined as zero or more.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/127.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa128() {
    /*
        Test ID:not-wf-sa-128
        Test URI:not-wf/sa/128.xml
        Spec Sections:2.7 [18]
        Description:Invalid CDATA syntax.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/128.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa129() {
    /*
        Test ID:not-wf-sa-129
        Test URI:not-wf/sa/129.xml
        Spec Sections:3.2 [45]
        Description:Invalid syntax for Element Type Declaration.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/129.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa130() {
    /*
        Test ID:not-wf-sa-130
        Test URI:not-wf/sa/130.xml
        Spec Sections:3.2 [45]
        Description:Invalid syntax for Element Type Declaration.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/130.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa131() {
    /*
        Test ID:not-wf-sa-131
        Test URI:not-wf/sa/131.xml
        Spec Sections:3.2 [45]
        Description:Invalid syntax for Element Type Declaration.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/131.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa132() {
    /*
        Test ID:not-wf-sa-132
        Test URI:not-wf/sa/132.xml
        Spec Sections:3.2.1 [50]
        Description:Invalid syntax mixed connectors used.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/132.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa133() {
    /*
        Test ID:not-wf-sa-133
        Test URI:not-wf/sa/133.xml
        Spec Sections:3.2.1
        Description:Illegal whitespace before optional character causes syntax error.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/133.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa134() {
    /*
        Test ID:not-wf-sa-134
        Test URI:not-wf/sa/134.xml
        Spec Sections:3.2.1
        Description:Illegal whitespace before optional character causes syntax error.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/134.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa135() {
    /*
        Test ID:not-wf-sa-135
        Test URI:not-wf/sa/135.xml
        Spec Sections:3.2.1 [47]
        Description:Invalid character used as connector.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/135.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa136() {
    /*
        Test ID:not-wf-sa-136
        Test URI:not-wf/sa/136.xml
        Spec Sections:3.2 [45]
        Description:Tag omission is invalid in XML.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/136.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa137() {
    /*
        Test ID:not-wf-sa-137
        Test URI:not-wf/sa/137.xml
        Spec Sections:3.2 [45]
        Description:Space is required before a content model.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/137.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa138() {
    /*
        Test ID:not-wf-sa-138
        Test URI:not-wf/sa/138.xml
        Spec Sections:3.2.1 [48]
        Description:Invalid syntax for content particle.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/138.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa139() {
    /*
        Test ID:not-wf-sa-139
        Test URI:not-wf/sa/139.xml
        Spec Sections:3.2.1 [46]
        Description:The element-content model should not be empty.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/139.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

/*
#[test]
#[ignore]
fn notwfsa140() {

    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */

    /*
        Test ID:not-wf-sa-140
        Test URI:not-wf/sa/140.xml
        Spec Sections:2.3 [4]
        Description:Character '&#x309a;' is a CombiningChar, not a Letter, and so may not begin a name.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/140.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}
 */

/*
#[test]
#[ignore]
fn notwfsa141() {

    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:not-wf-sa-141
        Test URI:not-wf/sa/141.xml
        Spec Sections:2.3 [5]
        Description:Character #x0E5C is not legal in XML names.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/141.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}
 */

#[test]
fn notwfsa142() {
    /*
        Test ID:not-wf-sa-142
        Test URI:not-wf/sa/142.xml
        Spec Sections:2.2 [2]
        Description:Character #x0000 is not legal anywhere in an XML document.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/142.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa143() {
    /*
        Test ID:not-wf-sa-143
        Test URI:not-wf/sa/143.xml
        Spec Sections:2.2 [2]
        Description:Character #x001F is not legal anywhere in an XML document.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/143.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa144() {
    /*
        Test ID:not-wf-sa-144
        Test URI:not-wf/sa/144.xml
        Spec Sections:2.2 [2]
        Description:Character #xFFFF is not legal anywhere in an XML document.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/144.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa145() {
    /*
        Test ID:not-wf-sa-145
        Test URI:not-wf/sa/145.xml
        Spec Sections:2.2 [2]
        Description:Character #xD800 is not legal anywhere in an XML document. (If it appeared in a UTF-16 surrogate pair, it'd represent half of a UCS-4 character and so wouldn't really be in the document.)
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/145.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa146() {
    /*
        Test ID:not-wf-sa-146
        Test URI:not-wf/sa/146.xml
        Spec Sections:2.2 [2]
        Description:Character references must also refer to legal XML characters; #x00110000 is one more than the largest legal character.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/146.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa147() {
    /*
        Test ID:not-wf-sa-147
        Test URI:not-wf/sa/147.xml
        Spec Sections:2.8 [22]
        Description:XML Declaration may not be preceded by whitespace.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/147.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa148() {
    /*
        Test ID:not-wf-sa-148
        Test URI:not-wf/sa/148.xml
        Spec Sections:2.8 [22]
        Description:XML Declaration may not be preceded by comments or whitespace.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/148.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa149() {
    /*
        Test ID:not-wf-sa-149
        Test URI:not-wf/sa/149.xml
        Spec Sections:2.8 [28]
        Description:XML Declaration may not be within a DTD.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/149.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa150() {
    /*
        Test ID:not-wf-sa-150
        Test URI:not-wf/sa/150.xml
        Spec Sections:3.1 [43]
        Description:XML declarations may not be within element content.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/150.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa151() {
    /*
        Test ID:not-wf-sa-151
        Test URI:not-wf/sa/151.xml
        Spec Sections:2.8 [27]
        Description:XML declarations may not follow document content.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/151.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa152() {
    /*
        Test ID:not-wf-sa-152
        Test URI:not-wf/sa/152.xml
        Spec Sections:2.8 [22]
        Description:XML declarations must include the "version=..." string.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/152.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa153() {
    /*
        Test ID:not-wf-sa-153
        Test URI:not-wf/sa/153.xml
        Spec Sections:4.3.2
        Description:Text declarations may not begin internal parsed entities; they may only appear at the beginning of external parsed (parameter or general) entities.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/153.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa154() {
    /*
        Test ID:not-wf-sa-154
        Test URI:not-wf/sa/154.xml
        Spec Sections:2.8 2.6 [23, 17]
        Description:'<?XML ...?>' is neither an XML declaration nor a legal processing instruction target name.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/154.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa155() {
    /*
        Test ID:not-wf-sa-155
        Test URI:not-wf/sa/155.xml
        Spec Sections:2.8 2.6 [23, 17]
        Description:'<?xmL ...?>' is neither an XML declaration nor a legal processing instruction target name.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/155.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa156() {
    /*
        Test ID:not-wf-sa-156
        Test URI:not-wf/sa/156.xml
        Spec Sections:2.8 2.6 [23, 17]
        Description:'<?xMl ...?>' is neither an XML declaration nor a legal processing instruction target name.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/156.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa157() {
    /*
        Test ID:not-wf-sa-157
        Test URI:not-wf/sa/157.xml
        Spec Sections:2.6 [17]
        Description:'<?xmL ...?>' is not a legal processing instruction target name.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/157.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa158() {
    /*
        Test ID:not-wf-sa-158
        Test URI:not-wf/sa/158.xml
        Spec Sections:3.3 [52]
        Description:SGML-ism: "#NOTATION gif" can't have attributes.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/158.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa159() {
    /*
        Test ID:not-wf-sa-159
        Test URI:not-wf/sa/159.xml
        Spec Sections:2.3 [9]
        Description:Uses '&' unquoted in an entity declaration, which is illegal syntax for an entity reference.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/159.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa160() {
    /*
        Test ID:not-wf-sa-160
        Test URI:not-wf/sa/160.xml
        Spec Sections:2.8
        Description:Violates the PEs in Internal Subset WFC by using a PE reference within a declaration.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/160.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa161() {
    /*
        Test ID:not-wf-sa-161
        Test URI:not-wf/sa/161.xml
        Spec Sections:2.8
        Description:Violates the PEs in Internal Subset WFC by using a PE reference within a declaration.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/161.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa162() {
    /*
        Test ID:not-wf-sa-162
        Test URI:not-wf/sa/162.xml
        Spec Sections:2.8
        Description:Violates the PEs in Internal Subset WFC by using a PE reference within a declaration.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/162.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa163() {
    /*
        Test ID:not-wf-sa-163
        Test URI:not-wf/sa/163.xml
        Spec Sections:4.1 [69]
        Description:Invalid placement of Parameter entity reference.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/163.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa164() {
    /*
        Test ID:not-wf-sa-164
        Test URI:not-wf/sa/164.xml
        Spec Sections:4.1 [69]
        Description:Invalid placement of Parameter entity reference.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/164.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa165() {
    /*
        Test ID:not-wf-sa-165
        Test URI:not-wf/sa/165.xml
        Spec Sections:4.2 [72]
        Description:Parameter entity declarations must have a space before the '%'.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/165.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa166() {
    /*
        Test ID:not-wf-sa-166
        Test URI:not-wf/sa/166.xml
        Spec Sections:2.2 [2]
        Description:Character FFFF is not legal anywhere in an XML document.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/166.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa167() {
    /*
        Test ID:not-wf-sa-167
        Test URI:not-wf/sa/167.xml
        Spec Sections:2.2 [2]
        Description:Character FFFE is not legal anywhere in an XML document.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/167.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn notwfsa168() {
    /*
        Test ID:not-wf-sa-168
        Test URI:not-wf/sa/168.xml
        Spec Sections:2.2 [2]
        Description:An unpaired surrogate (D800) is not legal anywhere in an XML document.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/168.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn notwfsa169() {
    /*
        Test ID:not-wf-sa-169
        Test URI:not-wf/sa/169.xml
        Spec Sections:2.2 [2]
        Description:An unpaired surrogate (DC00) is not legal anywhere in an XML document.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/169.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn notwfsa170() {
    /*
        Test ID:not-wf-sa-170
        Test URI:not-wf/sa/170.xml
        Spec Sections:2.2 [2]
        Description:Four byte UTF-8 encodings can encode UCS-4 characters which are beyond the range of legal XML characters (and can't be expressed in Unicode surrogate pairs). This document holds such a character.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/170.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa171() {
    /*
        Test ID:not-wf-sa-171
        Test URI:not-wf/sa/171.xml
        Spec Sections:2.2 [2]
        Description:Character FFFF is not legal anywhere in an XML document.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/171.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa172() {
    /*
        Test ID:not-wf-sa-172
        Test URI:not-wf/sa/172.xml
        Spec Sections:2.2 [2]
        Description:Character FFFF is not legal anywhere in an XML document.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/172.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa173() {
    /*
        Test ID:not-wf-sa-173
        Test URI:not-wf/sa/173.xml
        Spec Sections:2.2 [2]
        Description:Character FFFF is not legal anywhere in an XML document.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/173.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa174() {
    /*
        Test ID:not-wf-sa-174
        Test URI:not-wf/sa/174.xml
        Spec Sections:2.2 [2]
        Description:Character FFFF is not legal anywhere in an XML document.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/174.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa175() {
    /*
        Test ID:not-wf-sa-175
        Test URI:not-wf/sa/175.xml
        Spec Sections:2.2 [2]
        Description:Character FFFF is not legal anywhere in an XML document.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/175.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa176() {
    /*
        Test ID:not-wf-sa-176
        Test URI:not-wf/sa/176.xml
        Spec Sections:3 [39]
        Description:Start tags must have matching end tags.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/176.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa177() {
    /*
        Test ID:not-wf-sa-177
        Test URI:not-wf/sa/177.xml
        Spec Sections:2.2 [2]
        Description:Character FFFF is not legal anywhere in an XML document.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/177.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa178() {
    /*
        Test ID:not-wf-sa-178
        Test URI:not-wf/sa/178.xml
        Spec Sections:3.1 [41]
        Description:Invalid syntax matching double quote is missing.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/178.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa179() {
    /*
        Test ID:not-wf-sa-179
        Test URI:not-wf/sa/179.xml
        Spec Sections:4.1 [66]
        Description:Invalid syntax matching double quote is missing.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/179.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa180() {
    /*
        Test ID:not-wf-sa-180
        Test URI:not-wf/sa/180.xml
        Spec Sections:4.1
        Description:The Entity Declared WFC requires entities to be declared before they are used in an attribute list declaration.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/180.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa181() {
    /*
        Test ID:not-wf-sa-181
        Test URI:not-wf/sa/181.xml
        Spec Sections:4.3.2
        Description:Internal parsed entities must match the content production to be well formed.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/181.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa182() {
    /*
        Test ID:not-wf-sa-182
        Test URI:not-wf/sa/182.xml
        Spec Sections:4.3.2
        Description:Internal parsed entities must match the content production to be well formed.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/182.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa183() {
    /*
        Test ID:not-wf-sa-183
        Test URI:not-wf/sa/183.xml
        Spec Sections:3.2.2 [51]
        Description:Mixed content declarations may not include content particles.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/183.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa184() {
    /*
        Test ID:not-wf-sa-184
        Test URI:not-wf/sa/184.xml
        Spec Sections:3.2.2 [51]
        Description:In mixed content models, element names must not be parenthesized.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/184.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa185() {
    /*
        Test ID:not-wf-sa-185
        Test URI:not-wf/sa/185.xml
        Spec Sections:4.1
        Description:Tests the Entity Declared WFC. Note: a nonvalidating parser is permitted not to report this WFC violation, since it would need to read an external parameter entity to distinguish it from a violation of the Standalone Declaration VC.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/185.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn notwfsa186() {
    /*
        Test ID:not-wf-sa-186
        Test URI:not-wf/sa/186.xml
        Spec Sections:3.1 [44]
        Description:Whitespace is required between attribute/value pairs.
    */
    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/xmltest/not-wf/sa/186.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}
