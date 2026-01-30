/*

IBM test cases

*/

#[cfg(all(test, feature = "test-conformance-xml"))]
use std::fs;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::item::Node;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::parser::{ParseError, ParserStateBuilder, StaticStateBuilder, xml};
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::trees::smite::RNode;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::{Error, ErrorKind};

#[cfg(all(test, feature = "test-conformance-xml"))]
fn test_ibm_error(xmldoc: &str, docloc: &str) {
    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location(docloc.to_string())
        .build();
    let parseresult = xml::parse_with_state(xmldoc, ps, ss);

    assert!(parseresult.is_err());
}
#[cfg(all(test, feature = "test-conformance-xml"))]
fn dtdfileresolve() -> fn(Option<String>, String) -> Result<String, Error> {
    move |locdir, uri| {
        let u = match locdir {
            None => uri,
            Some(ld) => ld + uri.as_str(),
        };
        match fs::read_to_string(u) {
            Err(_) => Err(Error::new(
                ErrorKind::Unknown,
                "Unable to read external DTD".to_string(),
            )),
            Ok(s) => Ok(s),
        }
    }
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p69ibm69n05xml() {
    /*
        Test ID:ibm-not-wf-P69-ibm69n05.xml
        Test URI:not-wf/P69/ibm69n05.xml
        Spec Sections:4.1
        Description:Based on E29 substantial source: minutes XML-Syntax 1999-02-24 E38 in XML 1.0 Errata, this WFC does not apply to P69, but the VC Entity declared still apply. Tests PEReference which is against P69 WFC: Entity Declared. The PE with the name "paaa" is referred before declared in the DTD.
    */

    test_ibm_error(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P69/ibm69n05.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/not-wf/P69/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p68ibm68i01xml() {
    /*
        Test ID:ibm-invalid-P68-ibm68i01.xml
        Test URI:invalid/P68/ibm68i01.xml
        Spec Sections:4.1
        Description:Tests invalid EntityRef which is against P68 VC: Entity Declared. The GE with the name "ge2" is referred in the file ibm68i01.dtd", but not declared.
    */
    test_ibm_error(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P68/ibm68i01.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/invalid/P68/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p68ibm68i02xml() {
    /*
        Test ID:ibm-invalid-P68-ibm68i02.xml
        Test URI:invalid/P68/ibm68i02.xml
        Spec Sections:4.1
        Description:Tests invalid EntityRef which is against P68 VC: Entity Declared. The GE with the name "ge1" is referred before declared in the file ibm68i01.dtd".
    */
    test_ibm_error(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P68/ibm68i02.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/invalid/P68/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p68ibm68i03xml() {
    /*
        Test ID:ibm-invalid-P68-ibm68i03.xml
        Test URI:invalid/P68/ibm68i03.xml
        Spec Sections:4.1
        Description:Tests invalid EntityRef which is against P68 VC: Entity Declared. The GE with the name "ge2" is referred in the file ibm68i03.ent", but not declared.
    */
    test_ibm_error(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P68/ibm68i03.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/invalid/P68/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p68ibm68i04xml() {
    /*
        Test ID:ibm-invalid-P68-ibm68i04.xml
        Test URI:invalid/P68/ibm68i04.xml
        Spec Sections:4.1
        Description:Tests invalid EntityRef which is against P68 VC: Entity Declared. The GE with the name "ge1" is referred before declared in the file ibm68i04.ent".
    */
    test_ibm_error(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P68/ibm68i04.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/invalid/P68/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p69ibm69i01xml() {
    /*
        Test ID:ibm-invalid-P69-ibm69i01.xml
        Test URI:invalid/P69/ibm69i01.xml
        Spec Sections:4.1
        Description:Tests invalid PEReference which is against P69 VC: Entity Declared. The Name "pe2" in the PEReference in the file ibm69i01.dtd does not match the Name of any declared PE.
    */
    test_ibm_error(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P69/ibm69i01.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/invalid/P69/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p69ibm69i02xml() {
    /*
        Test ID:ibm-invalid-P69-ibm69i02.xml
        Test URI:invalid/P69/ibm69i02.xml
        Spec Sections:4.1
        Description:Tests invalid PEReference which is against P69 VC: Entity Declared. The PE with the name "pe1" is referred before declared in the file ibm69i02.dtd
    */
    test_ibm_error(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P69/ibm69i02.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/invalid/P69/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p69ibm69i03xml() {
    /*
        Test ID:ibm-invalid-P69-ibm69i03.xml
        Test URI:invalid/P69/ibm69i03.xml
        Spec Sections:4.1
        Description:Tests invalid PEReference which is against P69 VC: Entity Declared. The Name "pe3" in the PEReference in the file ibm69i03.ent does not match the Name of any declared PE.
    */
    test_ibm_error(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P69/ibm69i03.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/invalid/P69/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibminvalid_p69ibm69i04xml() {
    /*
        Test ID:ibm-invalid-P69-ibm69i04.xml
        Test URI:invalid/P69/ibm69i04.xml
        Spec Sections:4.1
        Description:Tests invalid PEReference which is against P69 VC: Entity Declared. The PE with the name "pe2" is referred before declared in the file ibm69i04.ent.
    */
    test_ibm_error(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/invalid/P69/ibm69i04.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/invalid/P69/",
    );
}
