/*

IBM test cases

*/

use std::fs;
use std::rc::Rc;
use xrust::parser::xml;
use xrust::trees::smite::Node as SmiteNode;

#[test]
#[ignore]
fn ibmnotwf_p69ibm69n05xml() {
    /*
        Test ID:ibm-not-wf-P69-ibm69n05.xml
        Test URI:not-wf/P69/ibm69n05.xml
        Spec Sections:4.1
        Description:Based on E29 substantial source: minutes XML-Syntax 1999-02-24 E38 in XML 1.0 Errata, this WFC does not apply to P69, but the VC Entity declared still apply. Tests PEReference which is against P69 WFC: Entity Declared. The PE with the name "paaa" is referred before declared in the DTD.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P69/ibm69n05.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn ibminvalid_p68ibm68i01xml() {
    /*
        Test ID:ibm-invalid-P68-ibm68i01.xml
        Test URI:invalid/P68/ibm68i01.xml
        Spec Sections:4.1
        Description:Tests invalid EntityRef which is against P68 VC: Entity Declared. The GE with the name "ge2" is referred in the file ibm68i01.dtd", but not declared.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P68/ibm68i01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn ibminvalid_p68ibm68i02xml() {
    /*
        Test ID:ibm-invalid-P68-ibm68i02.xml
        Test URI:invalid/P68/ibm68i02.xml
        Spec Sections:4.1
        Description:Tests invalid EntityRef which is against P68 VC: Entity Declared. The GE with the name "ge1" is referred before declared in the file ibm68i01.dtd".
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P68/ibm68i02.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn ibminvalid_p68ibm68i03xml() {
    /*
        Test ID:ibm-invalid-P68-ibm68i03.xml
        Test URI:invalid/P68/ibm68i03.xml
        Spec Sections:4.1
        Description:Tests invalid EntityRef which is against P68 VC: Entity Declared. The GE with the name "ge2" is referred in the file ibm68i03.ent", but not declared.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P68/ibm68i03.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn ibminvalid_p68ibm68i04xml() {
    /*
        Test ID:ibm-invalid-P68-ibm68i04.xml
        Test URI:invalid/P68/ibm68i04.xml
        Spec Sections:4.1
        Description:Tests invalid EntityRef which is against P68 VC: Entity Declared. The GE with the name "ge1" is referred before declared in the file ibm68i04.ent".
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P68/ibm68i04.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}



#[test]
#[ignore]
fn ibminvalid_p69ibm69i01xml() {
    /*
        Test ID:ibm-invalid-P69-ibm69i01.xml
        Test URI:invalid/P69/ibm69i01.xml
        Spec Sections:4.1
        Description:Tests invalid PEReference which is against P69 VC: Entity Declared. The Name "pe2" in the PEReference in the file ibm69i01.dtd does not match the Name of any declared PE.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P69/ibm69i01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn ibminvalid_p69ibm69i02xml() {
    /*
        Test ID:ibm-invalid-P69-ibm69i02.xml
        Test URI:invalid/P69/ibm69i02.xml
        Spec Sections:4.1
        Description:Tests invalid PEReference which is against P69 VC: Entity Declared. The PE with the name "pe1" is referred before declared in the file ibm69i02.dtd
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P69/ibm69i02.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn ibminvalid_p69ibm69i03xml() {
    /*
        Test ID:ibm-invalid-P69-ibm69i03.xml
        Test URI:invalid/P69/ibm69i03.xml
        Spec Sections:4.1
        Description:Tests invalid PEReference which is against P69 VC: Entity Declared. The Name "pe3" in the PEReference in the file ibm69i03.ent does not match the Name of any declared PE.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P69/ibm69i03.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn ibminvalid_p69ibm69i04xml() {
    /*
        Test ID:ibm-invalid-P69-ibm69i04.xml
        Test URI:invalid/P69/ibm69i04.xml
        Spec Sections:4.1
        Description:Tests invalid PEReference which is against P69 VC: Entity Declared. The PE with the name "pe2" is referred before declared in the file ibm69i04.ent.
    */

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P69/ibm69i04.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_err());
}

    
    
        
    

