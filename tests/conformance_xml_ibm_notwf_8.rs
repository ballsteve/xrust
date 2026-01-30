/*

IBM test cases

*/
/*

These are split into multiple files so rust can compile and test in parallel.

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
fn test_ibm_notwf(xmldoc: &str) {
    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        xmldoc,
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_err());
}

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n83xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n83.xml
        Test URI:not-wf/P87/ibm87n83.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x20E2 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n83.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n84xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n84.xml
        Test URI:not-wf/P87/ibm87n84.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x3030 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n84.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n85xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n85.xml
        Test URI:not-wf/P87/ibm87n85.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x309B occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n85.xml")
            .unwrap()
            .as_str());
}
 */

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p88ibm88n01xml() {
    /*
        Test ID:ibm-not-wf-P88-ibm88n01.xml
        Test URI:not-wf/P88/ibm88n01.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0029 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p88ibm88n02xml() {
    /*
        Test ID:ibm-not-wf-P88-ibm88n02.xml
        Test URI:not-wf/P88/ibm88n02.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x003B occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n02.xml")
            .unwrap()
            .as_str(),
    );
}

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p88ibm88n03xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P88-ibm88n03.xml
        Test URI:not-wf/P88/ibm88n03.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x066A occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n03.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p88ibm88n04xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P88-ibm88n04.xml
        Test URI:not-wf/P88/ibm88n04.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x06FA occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n04.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p88ibm88n05xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P88-ibm88n05.xml
        Test URI:not-wf/P88/ibm88n05.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0970 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n05.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p88ibm88n06xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P88-ibm88n06.xml
        Test URI:not-wf/P88/ibm88n06.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x09F2 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n06.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p88ibm88n08xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P88-ibm88n08.xml
        Test URI:not-wf/P88/ibm88n08.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0AF0 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n08.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p88ibm88n09xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P88-ibm88n09.xml
        Test URI:not-wf/P88/ibm88n09.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0B70 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n09.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p88ibm88n10xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P88-ibm88n10.xml
        Test URI:not-wf/P88/ibm88n10.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0C65 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n10.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p88ibm88n11xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P88-ibm88n11.xml
        Test URI:not-wf/P88/ibm88n11.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0CE5 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n11.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p88ibm88n12xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P88-ibm88n12.xml
        Test URI:not-wf/P88/ibm88n12.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0CF0 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n12.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p88ibm88n13xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P88-ibm88n13.xml
        Test URI:not-wf/P88/ibm88n13.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0D70 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n13.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p88ibm88n14xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P88-ibm88n14.xml
        Test URI:not-wf/P88/ibm88n14.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0E5A occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n14.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p88ibm88n15xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P88-ibm88n15.xml
        Test URI:not-wf/P88/ibm88n15.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0EDA occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n15.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p88ibm88n16xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P88-ibm88n16.xml
        Test URI:not-wf/P88/ibm88n16.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0F2A occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n16.xml")
            .unwrap()
            .as_str());
}
 */

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p89ibm89n01xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n01.xml
        Test URI:not-wf/P89/ibm89n01.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x00B6 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p89ibm89n02xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n02.xml
        Test URI:not-wf/P89/ibm89n02.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x00B8 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n02.xml")
            .unwrap()
            .as_str(),
    );
}

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p89ibm89n03xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P89-ibm89n03.xml
        Test URI:not-wf/P89/ibm89n03.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x02D2 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n03.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p89ibm89n04xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P89-ibm89n04.xml
        Test URI:not-wf/P89/ibm89n04.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x03FE occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n04.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p89ibm89n05xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P89-ibm89n05.xml
        Test URI:not-wf/P89/ibm89n05.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x065F occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n05.xml")
            .unwrap()
            .as_str());
}
 */

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p89ibm89n06xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n06.xml
        Test URI:not-wf/P89/ibm89n06.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x0EC7 occurs as the second character in the PITarget in the PI in the DTD. [Also contains two top-level elements -- one should be removed]
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p89ibm89n07xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n07.xml
        Test URI:not-wf/P89/ibm89n07.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x3006 occurs as the second character in the PITarget in the PI in the DTD. [Also contains two top-level elements -- one should be removed]
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p89ibm89n08xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n08.xml
        Test URI:not-wf/P89/ibm89n08.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x3030 occurs as the second character in the PITarget in the PI in the DTD. [Also contains two top-level elements -- one should be removed]
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n08.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p89ibm89n09xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n09.xml
        Test URI:not-wf/P89/ibm89n09.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x3036 occurs as the second character in the PITarget in the PI in the DTD. [Also contains two top-level elements -- one should be removed]
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n09.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p89ibm89n10xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n10.xml
        Test URI:not-wf/P89/ibm89n10.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x309C occurs as the second character in the PITarget in the PI in the DTD. [Also contains two top-level elements -- one should be removed]
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n10.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p89ibm89n11xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n11.xml
        Test URI:not-wf/P89/ibm89n11.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x309F occurs as the second character in the PITarget in the PI in the DTD. [Also contains two top-level elements -- one should be removed]
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n11.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p89ibm89n12xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n12.xml
        Test URI:not-wf/P89/ibm89n12.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x30FF occurs as the second character in the PITarget in the PI in the DTD. [Also contains two top-level elements -- one should be removed]
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n12.xml")
            .unwrap()
            .as_str(),
    );
}
