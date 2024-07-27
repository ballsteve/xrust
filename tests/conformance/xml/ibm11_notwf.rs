/*

IBM test cases

*/

use crate::conformance::{dtdfileresolve, non_utf8_file_reader};
use std::fs;
use std::rc::Rc;
use xrust::parser::{xml, ParserConfig};
use xrust::trees::smite::Node as SmiteNode;

#[test]
fn ibm11notwf_p02ibm02n01xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n01.xml
        Test URI:not-wf/P02/ibm02n01.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x1.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n02xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n02.xml
        Test URI:not-wf/P02/ibm02n02.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x2.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n02.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n03xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n03.xml
        Test URI:not-wf/P02/ibm02n03.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x3.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n03.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n04xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n04.xml
        Test URI:not-wf/P02/ibm02n04.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x4.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n04.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n05xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n05.xml
        Test URI:not-wf/P02/ibm02n05.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x5.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n05.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n06xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n06.xml
        Test URI:not-wf/P02/ibm02n06.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x6.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n06.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n07xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n07.xml
        Test URI:not-wf/P02/ibm02n07.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x7.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n07.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n08xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n08.xml
        Test URI:not-wf/P02/ibm02n08.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x8.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n08.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n09xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n09.xml
        Test URI:not-wf/P02/ibm02n09.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x0.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n09.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n10xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n10.xml
        Test URI:not-wf/P02/ibm02n10.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x100.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n10.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n11xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n11.xml
        Test URI:not-wf/P02/ibm02n11.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x0B.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n11.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n12xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n12.xml
        Test URI:not-wf/P02/ibm02n12.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x0C.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n12.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}


#[test]
fn ibm11notwf_p02ibm02n14xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n14.xml
        Test URI:not-wf/P02/ibm02n14.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x0E.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n14.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n15xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n15.xml
        Test URI:not-wf/P02/ibm02n15.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x0F.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n15.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n16xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n16.xml
        Test URI:not-wf/P02/ibm02n16.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x10.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n16.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n17xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n17.xml
        Test URI:not-wf/P02/ibm02n17.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x11.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n17.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n18xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n18.xml
        Test URI:not-wf/P02/ibm02n18.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x12.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n18.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n19xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n19.xml
        Test URI:not-wf/P02/ibm02n19.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x13.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n19.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n20xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n20.xml
        Test URI:not-wf/P02/ibm02n20.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x14.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n20.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n21xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n21.xml
        Test URI:not-wf/P02/ibm02n21.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x15.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n21.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n22xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n22.xml
        Test URI:not-wf/P02/ibm02n22.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x16.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n22.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n23xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n23.xml
        Test URI:not-wf/P02/ibm02n23.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x17.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n23.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n24xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n24.xml
        Test URI:not-wf/P02/ibm02n24.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x18.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n24.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n25xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n25.xml
        Test URI:not-wf/P02/ibm02n25.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x19.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n25.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n26xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n26.xml
        Test URI:not-wf/P02/ibm02n26.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x1A.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n26.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n27xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n27.xml
        Test URI:not-wf/P02/ibm02n27.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x1B.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n27.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n28xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n28.xml
        Test URI:not-wf/P02/ibm02n28.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x1C.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n28.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n29xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n29.xml
        Test URI:not-wf/P02/ibm02n29.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x1D.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n29.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n30xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n30.xml
        Test URI:not-wf/P02/ibm02n30.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x1E.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n30.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n31xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n31.xml
        Test URI:not-wf/P02/ibm02n31.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x1F.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n31.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n32xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n32.xml
        Test URI:not-wf/P02/ibm02n32.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x7F.
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string());

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n32.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n33xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n33.xml
        Test URI:not-wf/P02/ibm02n33.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x80.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n33.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n34xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n34.xml
        Test URI:not-wf/P02/ibm02n34.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x81.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n34.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n35xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n35.xml
        Test URI:not-wf/P02/ibm02n35.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x82.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n35.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n36xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n36.xml
        Test URI:not-wf/P02/ibm02n36.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x83.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n36.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n37xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n37.xml
        Test URI:not-wf/P02/ibm02n37.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x84.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n37.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n38xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n38.xml
        Test URI:not-wf/P02/ibm02n38.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control characters x82, x83 and x84.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n38.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n39xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n39.xml
        Test URI:not-wf/P02/ibm02n39.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x86.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n39.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n40xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n40.xml
        Test URI:not-wf/P02/ibm02n40.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x87.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n40.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n41xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n41.xml
        Test URI:not-wf/P02/ibm02n41.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x88.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n41.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n42xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n42.xml
        Test URI:not-wf/P02/ibm02n42.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x89.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n42.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n43xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n43.xml
        Test URI:not-wf/P02/ibm02n43.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x8A.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n43.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n44xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n44.xml
        Test URI:not-wf/P02/ibm02n44.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x8B.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n44.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n45xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n45.xml
        Test URI:not-wf/P02/ibm02n45.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x8C.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n45.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n46xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n46.xml
        Test URI:not-wf/P02/ibm02n46.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x8D.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n46.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n47xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n47.xml
        Test URI:not-wf/P02/ibm02n47.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x8E.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n47.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n48xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n48.xml
        Test URI:not-wf/P02/ibm02n48.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x8F.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n48.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n49xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n49.xml
        Test URI:not-wf/P02/ibm02n49.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x90.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n49.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n50xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n50.xml
        Test URI:not-wf/P02/ibm02n50.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x91.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n50.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n51xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n51.xml
        Test URI:not-wf/P02/ibm02n51.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x92.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n51.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n52xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n52.xml
        Test URI:not-wf/P02/ibm02n52.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x93.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n52.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n53xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n53.xml
        Test URI:not-wf/P02/ibm02n53.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x94.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n53.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n54xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n54.xml
        Test URI:not-wf/P02/ibm02n54.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x95.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n54.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n55xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n55.xml
        Test URI:not-wf/P02/ibm02n55.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x96.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n55.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n56xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n56.xml
        Test URI:not-wf/P02/ibm02n56.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x97.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n56.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n57xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n57.xml
        Test URI:not-wf/P02/ibm02n57.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x98.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n57.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn ibm11notwf_p02ibm02n58xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n58.xml
        Test URI:not-wf/P02/ibm02n58.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x99.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n58.xml")
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n59xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n59.xml
        Test URI:not-wf/P02/ibm02n59.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x9A.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n59.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n60xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n60.xml
        Test URI:not-wf/P02/ibm02n60.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x9B.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n60.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n61xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n61.xml
        Test URI:not-wf/P02/ibm02n61.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x9C.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n61.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n62xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n62.xml
        Test URI:not-wf/P02/ibm02n62.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x9D.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n62.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n63xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n63.xml
        Test URI:not-wf/P02/ibm02n63.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x9E.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n63.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n64xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n64.xml
        Test URI:not-wf/P02/ibm02n64.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control characters present in an external entity.
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string());

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n64.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n65xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n65.xml
        Test URI:not-wf/P02/ibm02n65.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control characters present in an external entity.
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string());

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n65.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n66xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n66.xml
        Test URI:not-wf/P02/ibm02n66.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control characters present in an external entity.
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string());

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n66.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn ibm11notwf_p02ibm02n67xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n67.xml
        Test URI:not-wf/P02/ibm02n67.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded character 0xD800. (Invalid UTF8 sequence)
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n67.xml")
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n68xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n68.xml
        Test URI:not-wf/P02/ibm02n68.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded character 0xFFFE.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n68.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n69xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n69.xml
        Test URI:not-wf/P02/ibm02n69.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded character 0xFFFF.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n69.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n70xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n70.xml
        Test URI:not-wf/P02/ibm02n70.xml
        Spec Sections:2.2,4.1
        Description:This test contains a reference to character 0xFFFE.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n70.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p02ibm02n71xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n71.xml
        Test URI:not-wf/P02/ibm02n71.xml
        Spec Sections:2.2,4.1
        Description:This test contains a reference to character 0xFFFF.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n71.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}




#[test]
fn ibm11notwf_p04ibm04n01xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n01.xml
        Test URI:not-wf/P04/ibm04n01.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #x300
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n02xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n02.xml
        Test URI:not-wf/P04/ibm04n02.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x333
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n02.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n03xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n03.xml
        Test URI:not-wf/P04/ibm04n03.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x369
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n03.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n04xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n04.xml
        Test URI:not-wf/P04/ibm04n04.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x37E
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n04.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n05xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n05.xml
        Test URI:not-wf/P04/ibm04n05.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2000
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n05.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n06xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n06.xml
        Test URI:not-wf/P04/ibm04n06.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2001
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n06.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n07xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n07.xml
        Test URI:not-wf/P04/ibm04n07.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2002
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n07.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n08xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n08.xml
        Test URI:not-wf/P04/ibm04n08.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2005
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n08.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n09xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n09.xml
        Test URI:not-wf/P04/ibm04n09.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x200B
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n09.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n10xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n10.xml
        Test URI:not-wf/P04/ibm04n10.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x200E
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n10.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n11xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n11.xml
        Test URI:not-wf/P04/ibm04n11.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x200F
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n11.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n12xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n12.xml
        Test URI:not-wf/P04/ibm04n12.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2069
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n12.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n13xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n13.xml
        Test URI:not-wf/P04/ibm04n13.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2190
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n13.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n14xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n14.xml
        Test URI:not-wf/P04/ibm04n14.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x23FF
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n14.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n15xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n15.xml
        Test URI:not-wf/P04/ibm04n15.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x280F
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n15.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n16xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n16.xml
        Test URI:not-wf/P04/ibm04n16.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2A00
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n16.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n17xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n17.xml
        Test URI:not-wf/P04/ibm04n17.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2EDC
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n17.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n18xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n18.xml
        Test URI:not-wf/P04/ibm04n18.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2B00
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n18.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n19xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n19.xml
        Test URI:not-wf/P04/ibm04n19.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2BFF
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n19.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n20xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n20.xml
        Test URI:not-wf/P04/ibm04n20.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x3000
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n20.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n21xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n21.xml
        Test URI:not-wf/P04/ibm04n21.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xD800
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n21.xml")
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n22xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n22.xml
        Test URI:not-wf/P04/ibm04n22.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xD801
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n22.xml")
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n23xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n23.xml
        Test URI:not-wf/P04/ibm04n23.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xDAFF
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n23.xml")
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n24xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n24.xml
        Test URI:not-wf/P04/ibm04n24.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xDFFF
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n24.xml")
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n25xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n25.xml
        Test URI:not-wf/P04/ibm04n25.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xEFFF
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n25.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n26xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n26.xml
        Test URI:not-wf/P04/ibm04n26.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xF1FF
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n26.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n27xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n27.xml
        Test URI:not-wf/P04/ibm04n27.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xF8FF
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n27.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04ibm04n28xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n28.xml
        Test URI:not-wf/P04/ibm04n28.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xFFFFF
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n28.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}




#[test]
fn ibm11notwf_p04aibm04an01xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an01.xml
        Test URI:not-wf/P04a/ibm04an01.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #xB8
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an02xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an02.xml
        Test URI:not-wf/P04a/ibm04an02.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xA1
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an02.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an03xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an03.xml
        Test URI:not-wf/P04a/ibm04an03.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xAF
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an03.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an04xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an04.xml
        Test URI:not-wf/P04a/ibm04an04.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x37E
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an04.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an05xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an05.xml
        Test URI:not-wf/P04a/ibm04an05.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2000
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an05.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an06xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an06.xml
        Test URI:not-wf/P04a/ibm04an06.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2001
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an06.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an07xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an07.xml
        Test URI:not-wf/P04a/ibm04an07.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2002
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an07.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an08xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an08.xml
        Test URI:not-wf/P04a/ibm04an08.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2005
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an08.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an09xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an09.xml
        Test URI:not-wf/P04a/ibm04an09.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x200B
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an09.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an10xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an10.xml
        Test URI:not-wf/P04a/ibm04an10.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x200E
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an10.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an11xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an11.xml
        Test URI:not-wf/P04a/ibm04an11.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2038
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an11.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an12xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an12.xml
        Test URI:not-wf/P04a/ibm04an12.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2041
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an12.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an13xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an13.xml
        Test URI:not-wf/P04a/ibm04an13.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2190
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an13.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an14xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an14.xml
        Test URI:not-wf/P04a/ibm04an14.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x23FF
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an14.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an15xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an15.xml
        Test URI:not-wf/P04a/ibm04an15.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x280F
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an15.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an16xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an16.xml
        Test URI:not-wf/P04a/ibm04an16.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2A00
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an16.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an17xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an17.xml
        Test URI:not-wf/P04a/ibm04an17.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xFDD0
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an17.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an18xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an18.xml
        Test URI:not-wf/P04a/ibm04an18.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xFDEF
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an18.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an19xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an19.xml
        Test URI:not-wf/P04a/ibm04an19.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2FFF
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an19.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an20xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an20.xml
        Test URI:not-wf/P04a/ibm04an20.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x3000
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an20.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an21xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an21.xml
        Test URI:not-wf/P04a/ibm04an21.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xD800
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an21.xml")
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an22xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an22.xml
        Test URI:not-wf/P04a/ibm04an22.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xD801
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an22.xml")
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an23xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an23.xml
        Test URI:not-wf/P04a/ibm04an23.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xDAFF
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an23.xml")
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an24xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an24.xml
        Test URI:not-wf/P04a/ibm04an24.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xDFFF
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an24.xml")
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an25xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an25.xml
        Test URI:not-wf/P04a/ibm04an25.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xEFFF
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an25.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an26xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an26.xml
        Test URI:not-wf/P04a/ibm04an26.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xF1FF
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an26.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an27xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an27.xml
        Test URI:not-wf/P04a/ibm04an27.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xF8FF
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an27.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p04aibm04an28xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an28.xml
        Test URI:not-wf/P04a/ibm04an28.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xFFFFF
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an28.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}




#[test]
fn ibm11notwf_p05ibm05n01xml() {
    /*
        Test ID:ibm-1-1-not-wf-P05-ibm05n01.xml
        Test URI:not-wf/P05/ibm05n01.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal Name containing #0x0B
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P05/ibm05n01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p05ibm05n02xml() {
    /*
        Test ID:ibm-1-1-not-wf-P05-ibm05n02.xml
        Test URI:not-wf/P05/ibm05n02.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal Name containing #0x300
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P05/ibm05n02.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p05ibm05n03xml() {
    /*
        Test ID:ibm-1-1-not-wf-P05-ibm05n03.xml
        Test URI:not-wf/P05/ibm05n03.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal Name containing #0x36F
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P05/ibm05n03.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p05ibm05n04xml() {
    /*
        Test ID:ibm-1-1-not-wf-P05-ibm05n04.xml
        Test URI:not-wf/P05/ibm05n04.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal Name containing #0x203F
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P05/ibm05n04.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p05ibm05n05xml() {
    /*
        Test ID:ibm-1-1-not-wf-P05-ibm05n05.xml
        Test URI:not-wf/P05/ibm05n05.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal Name containing #x2040
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P05/ibm05n05.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p05ibm05n06xml() {
    /*
        Test ID:ibm-1-1-not-wf-P05-ibm05n06.xml
        Test URI:not-wf/P05/ibm05n06.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal Name containing #0xB7
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P05/ibm05n06.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}





#[test]
fn ibm11notwf_p77ibm77n01xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n01.xml
        Test URI:not-wf/P77/ibm77n01.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and that of the external dtd 1.0. The external dtd contains the invalid XML1.1 but valid XML 1.0 character #x7F.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p77ibm77n02xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n02.xml
        Test URI:not-wf/P77/ibm77n02.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and that of the external dtd 1.0. The external dtd contains a comment with the invalid XML1.1 but valid XML 1.0 character #x80.
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string());

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n02.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p77ibm77n03xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n03.xml
        Test URI:not-wf/P77/ibm77n03.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and that of the external dtd 1.0. The external dtd contains a PI with the invalid XML1.1 but valid XML 1.0 character #x9F.
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string());

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n03.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p77ibm77n04xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n04.xml
        Test URI:not-wf/P77/ibm77n04.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and that of the external entity 1.0. The external entity the contains invalid XML1.1 but valid XML 1.0 character #x89.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n04.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p77ibm77n05xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n05.xml
        Test URI:not-wf/P77/ibm77n05.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and that of the external entity 1.0. The external entity contains the invalid XML1.1 but valid XML 1.0 character #x94.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n05.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p77ibm77n06xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n06.xml
        Test URI:not-wf/P77/ibm77n06.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and that of the external entity 1.0. The external entity contains the invalid XML1.1 but valid XML 1.0 character #x9F.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n06.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p77ibm77n07xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n07.xml
        Test URI:not-wf/P77/ibm77n07.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and the external dtd does not contain a textDecl. The external entity contains the invalid XML1.1 but valid XML 1.0 character #x7F.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n07.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p77ibm77n08xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n08.xml
        Test URI:not-wf/P77/ibm77n08.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and the external dtd does not contain a VersionNum in the textDecl. The external entity contains the invalid XML1.1 but valid XML 1.0 character #x9B.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n08.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p77ibm77n09xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n09.xml
        Test URI:not-wf/P77/ibm77n09.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and the external dtd does not contain a textDecl. The external entity contains the invalid XML1.1 but valid XML 1.0 character #x8D.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n09.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p77ibm77n10xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n10.xml
        Test URI:not-wf/P77/ibm77n10.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and the external dtd does not contain a VersionNum in the textDecl. The external entity contains the invalid XML 1.1 but valid XML 1.0 character #x84.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n10.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p77ibm77n11xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n11.xml
        Test URI:not-wf/P77/ibm77n11.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and the external dtd does not contain a textDecl. The external entity contains the invalid XML 1.1 but valid XML 1.0 character #x88.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n11.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p77ibm77n12xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n12.xml
        Test URI:not-wf/P77/ibm77n12.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and the external dtd does not contain a textDecl. The external entity contains the invalid XML 1.1 but valid XML 1.0 character #x8E.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n12.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p77ibm77n13xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n13.xml
        Test URI:not-wf/P77/ibm77n13.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the primary document entity is 1.0 and that of the external dtd is 1.0. The external dtd contains an external entity whose VersionNum is 1.1.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n13.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p77ibm77n14xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n14.xml
        Test URI:not-wf/P77/ibm77n14.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the primary document entity is 1.1 and that of the external dtd is 1.0. The external dtd contains an element declaration with an invalid XML 1.1 and 1.0 name.
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string());

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n14.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p77ibm77n15xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n15.xml
        Test URI:not-wf/P77/ibm77n15.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the primary document entity is 1.1 and testDecl of the external dtd is absent. The external dtd contains an external entity whose VersionNum is 1.1 containing a valid XML1.0 but an invalid XML 1.1 character #x7F.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n15.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p77ibm77n16xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n16.xml
        Test URI:not-wf/P77/ibm77n16.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the primary document entity is 1.0 and VersioNum of the external entity is absent. The replacement text of the entity contains an element followed by the valid XML 1.1 of line character NEL #x85 in its empty elem tag.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n16.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p77ibm77n17xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n17.xml
        Test URI:not-wf/P77/ibm77n17.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the primary document entity is absent and that of the external entity is 1.0. The textDecl in the external entity contains an invalid XML1.0 but valid XML 1.1 enf of line character NEL #x85.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n17.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p77ibm77n18xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n18.xml
        Test URI:not-wf/P77/ibm77n18.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the primary document entity is absent and that of the external entity is 1.0. The textDecl in the external entity contains an invalid XML1.0 but valid XML 1.1 of line character Unicode line separator #x2028.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n18.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p77ibm77n19xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n19.xml
        Test URI:not-wf/P77/ibm77n19.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the primary document entity is 1.1 and that of the external dtd is absent. The external dtd contains an external entity whose VersionNum is absent and it contains a valid XML 1.0 but an invalid XML 1.1 character #x94.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n19.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p77ibm77n20xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n20.xml
        Test URI:not-wf/P77/ibm77n20.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the primary document entity is 1.1 and that of the external dtd is 1.1. The external dtd contains an external entity whose VersionNum is absent and it contains a valid XML 1.0 but an invalid XML 1.1 character #x8F.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n20.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
fn ibm11notwf_p77ibm77n21xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n21.xml
        Test URI:not-wf/P77/ibm77n21.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the primary document entity is 1.1 and the texlDecl of the external dtd is absent. The external dtd contains a reference to an external parameter entity whose VersionNum is absent from the textDecl and it contains an invalid XML 1.1 character #x8F.
    */

    let mut pc = ParserConfig::new();
    pc.ext_dtd_resolver = Some(dtdfileresolve());
    pc.docloc = Some("tests/conformance/xml/xmlconf/ibm/xml-1.1/".to_string());

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n21.xml")
            .unwrap()
            .as_str(),
        Some(pc),
    );

    assert!(parseresult.is_err());
}