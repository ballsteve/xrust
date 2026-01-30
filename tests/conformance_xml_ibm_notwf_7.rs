/*

IBM test cases

*/
/*

These are split into multiple files so rust can compile and test in parallel.

All tests in this file are ignored, as they are now well formed in the 5th edition of XML 1.0, though
they would not be in previous editions.

This file is kept for historical completeness more than anything.

*/

/*
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
fn test_ibm_notwf(xmldoc: &str) {
    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        xmldoc,
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_err());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n85xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n85.xml
        Test URI:not-wf/P85/ibm85n85.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B29 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n85.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n86xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n86.xml
        Test URI:not-wf/P85/ibm85n86.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B31 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n86.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n87xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n87.xml
        Test URI:not-wf/P85/ibm85n87.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B34 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n87.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n88xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n88.xml
        Test URI:not-wf/P85/ibm85n88.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B3A occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n88.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n89xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n89.xml
        Test URI:not-wf/P85/ibm85n89.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B3E occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n89.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n90xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n90.xml
        Test URI:not-wf/P85/ibm85n90.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B5E occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n90.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n91xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n91.xml
        Test URI:not-wf/P85/ibm85n91.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B62 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n91.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n92xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n92.xml
        Test URI:not-wf/P85/ibm85n92.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B8B occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n92.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n93xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n93.xml
        Test URI:not-wf/P85/ibm85n93.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B91 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n93.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n94xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n94.xml
        Test URI:not-wf/P85/ibm85n94.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B98 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n94.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n95xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n95.xml
        Test URI:not-wf/P85/ibm85n95.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B9B occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n95.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n96xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n96.xml
        Test URI:not-wf/P85/ibm85n96.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B9D occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n96.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n97xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n97.xml
        Test URI:not-wf/P85/ibm85n97.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0BA0 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n97.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n98xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n98.xml
        Test URI:not-wf/P85/ibm85n98.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0BA7 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n98.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n99xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n99.xml
        Test URI:not-wf/P85/ibm85n99.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0BAB occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n99.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p86ibm86n01xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P86-ibm86n01.xml
        Test URI:not-wf/P86/ibm86n01.xml
        Spec Sections:B.
        Description:Tests Ideographic with an illegal character. The character #x4CFF occurs as the first character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P86/ibm86n01.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p86ibm86n02xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P86-ibm86n02.xml
        Test URI:not-wf/P86/ibm86n02.xml
        Spec Sections:B.
        Description:Tests Ideographic with an illegal character. The character #x9FA6 occurs as the first character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P86/ibm86n02.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p86ibm86n03xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P86-ibm86n03.xml
        Test URI:not-wf/P86/ibm86n03.xml
        Spec Sections:B.
        Description:Tests Ideographic with an illegal character. The character #x3008 occurs as the first character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P86/ibm86n03.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p86ibm86n04xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P86-ibm86n04.xml
        Test URI:not-wf/P86/ibm86n04.xml
        Spec Sections:B.
        Description:Tests Ideographic with an illegal character. The character #x302A occurs as the first character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P86/ibm86n04.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n01xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n01.xml
        Test URI:not-wf/P87/ibm87n01.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x02FF occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n01.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n02xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n02.xml
        Test URI:not-wf/P87/ibm87n02.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0346 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n02.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n03xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n03.xml
        Test URI:not-wf/P87/ibm87n03.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0362 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n03.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n04xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n04.xml
        Test URI:not-wf/P87/ibm87n04.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0487 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n04.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n05xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n05.xml
        Test URI:not-wf/P87/ibm87n05.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x05A2 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n05.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n06xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n06.xml
        Test URI:not-wf/P87/ibm87n06.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x05BA occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n06.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n07xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n07.xml
        Test URI:not-wf/P87/ibm87n07.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x05BE occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n07.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n08xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n08.xml
        Test URI:not-wf/P87/ibm87n08.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x05C0 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n08.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n09xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n09.xml
        Test URI:not-wf/P87/ibm87n09.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x05C3 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n09.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n10xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n10.xml
        Test URI:not-wf/P87/ibm87n10.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0653 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n10.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n11xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n11.xml
        Test URI:not-wf/P87/ibm87n11.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x06B8 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n11.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n12xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n12.xml
        Test URI:not-wf/P87/ibm87n12.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x06B9 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n12.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n13xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n13.xml
        Test URI:not-wf/P87/ibm87n13.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x06E9 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n13.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n14xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n14.xml
        Test URI:not-wf/P87/ibm87n14.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x06EE occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n14.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n15xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n15.xml
        Test URI:not-wf/P87/ibm87n15.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0904 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n15.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n16xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n16.xml
        Test URI:not-wf/P87/ibm87n16.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x093B occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n16.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n17xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n17.xml
        Test URI:not-wf/P87/ibm87n17.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x094E occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n17.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n18xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n18.xml
        Test URI:not-wf/P87/ibm87n18.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0955 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n18.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n19xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n19.xml
        Test URI:not-wf/P87/ibm87n19.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0964 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n19.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n20xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n20.xml
        Test URI:not-wf/P87/ibm87n20.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0984 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n20.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n21xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n21.xml
        Test URI:not-wf/P87/ibm87n21.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x09C5 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n21.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n22xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n22.xml
        Test URI:not-wf/P87/ibm87n22.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x09C9 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n22.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n23xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n23.xml
        Test URI:not-wf/P87/ibm87n23.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x09CE occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n23.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n24xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n24.xml
        Test URI:not-wf/P87/ibm87n24.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x09D8 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n24.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n25xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n25.xml
        Test URI:not-wf/P87/ibm87n25.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x09E4 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n25.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n26xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n26.xml
        Test URI:not-wf/P87/ibm87n26.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0A03 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n26.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n27xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n27.xml
        Test URI:not-wf/P87/ibm87n27.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0A3D occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n27.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n28xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n28.xml
        Test URI:not-wf/P87/ibm87n28.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0A46 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n28.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n29xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n29.xml
        Test URI:not-wf/P87/ibm87n29.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0A49 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n29.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n30xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n30.xml
        Test URI:not-wf/P87/ibm87n30.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0A4E occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n30.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n31xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n31.xml
        Test URI:not-wf/P87/ibm87n31.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0A80 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n31.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n32xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n32.xml
        Test URI:not-wf/P87/ibm87n32.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0A84 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n32.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n33xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n33.xml
        Test URI:not-wf/P87/ibm87n33.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0ABB occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n33.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n34xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n34.xml
        Test URI:not-wf/P87/ibm87n34.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0AC6 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n34.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n35xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n35.xml
        Test URI:not-wf/P87/ibm87n35.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0ACA occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n35.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n36xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n36.xml
        Test URI:not-wf/P87/ibm87n36.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0ACE occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n36.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n37xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n37.xml
        Test URI:not-wf/P87/ibm87n37.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0B04 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n37.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n38xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n38.xml
        Test URI:not-wf/P87/ibm87n38.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0B3B occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n38.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n39xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n39.xml
        Test URI:not-wf/P87/ibm87n39.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0B44 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n39.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n40xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n40.xml
        Test URI:not-wf/P87/ibm87n40.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0B4A occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n40.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n41xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n41.xml
        Test URI:not-wf/P87/ibm87n41.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0B4E occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n41.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n42xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n42.xml
        Test URI:not-wf/P87/ibm87n42.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0B58 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n42.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n43xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n43.xml
        Test URI:not-wf/P87/ibm87n43.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0B84 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n43.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n44xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n44.xml
        Test URI:not-wf/P87/ibm87n44.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0BC3 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n44.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n45xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n45.xml
        Test URI:not-wf/P87/ibm87n45.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0BC9 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n45.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n46xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n46.xml
        Test URI:not-wf/P87/ibm87n46.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0BD6 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n46.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n47xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n47.xml
        Test URI:not-wf/P87/ibm87n47.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0C0D occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n47.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n48xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n48.xml
        Test URI:not-wf/P87/ibm87n48.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0C45 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n48.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n49xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n49.xml
        Test URI:not-wf/P87/ibm87n49.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0C49 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n49.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n50xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n50.xml
        Test URI:not-wf/P87/ibm87n50.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0C54 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n50.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n51xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n51.xml
        Test URI:not-wf/P87/ibm87n51.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0C81 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n51.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n52xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n52.xml
        Test URI:not-wf/P87/ibm87n52.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0C84 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n52.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n53xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n53.xml
        Test URI:not-wf/P87/ibm87n53.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0CC5 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n53.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n54xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n54.xml
        Test URI:not-wf/P87/ibm87n54.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0CC9 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n54.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n55xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n55.xml
        Test URI:not-wf/P87/ibm87n55.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0CD4 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n55.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n56xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n56.xml
        Test URI:not-wf/P87/ibm87n56.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0CD7 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n56.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n57xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n57.xml
        Test URI:not-wf/P87/ibm87n57.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0D04 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n57.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n58xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n58.xml
        Test URI:not-wf/P87/ibm87n58.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0D45 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n58.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n59xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n59.xml
        Test URI:not-wf/P87/ibm87n59.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0D49 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n59.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n60xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n60.xml
        Test URI:not-wf/P87/ibm87n60.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0D4E occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n60.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n61xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n61.xml
        Test URI:not-wf/P87/ibm87n61.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0D58 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n61.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n62xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n62.xml
        Test URI:not-wf/P87/ibm87n62.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0E3F occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n62.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n63xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n63.xml
        Test URI:not-wf/P87/ibm87n63.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0E3B occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n63.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n64xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n64.xml
        Test URI:not-wf/P87/ibm87n64.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0E4F occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n64.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n66xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n66.xml
        Test URI:not-wf/P87/ibm87n66.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0EBA occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n66.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n67xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n67.xml
        Test URI:not-wf/P87/ibm87n67.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0EBE occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n67.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n68xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n68.xml
        Test URI:not-wf/P87/ibm87n68.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0ECE occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n68.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n69xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n69.xml
        Test URI:not-wf/P87/ibm87n69.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0F1A occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n69.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n70xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n70.xml
        Test URI:not-wf/P87/ibm87n70.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0F36 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n70.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n71xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n71.xml
        Test URI:not-wf/P87/ibm87n71.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0F38 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n71.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n72xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n72.xml
        Test URI:not-wf/P87/ibm87n72.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0F3B occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n72.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n73xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n73.xml
        Test URI:not-wf/P87/ibm87n73.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0F3A occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n73.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n74xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n74.xml
        Test URI:not-wf/P87/ibm87n74.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0F70 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n74.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n75xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n75.xml
        Test URI:not-wf/P87/ibm87n75.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0F85 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n75.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n76xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n76.xml
        Test URI:not-wf/P87/ibm87n76.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0F8C occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n76.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n77xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n77.xml
        Test URI:not-wf/P87/ibm87n77.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0F96 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n77.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n78xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n78.xml
        Test URI:not-wf/P87/ibm87n78.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0F98 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n78.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n79xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n79.xml
        Test URI:not-wf/P87/ibm87n79.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0FB0 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n79.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n80xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n80.xml
        Test URI:not-wf/P87/ibm87n80.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0FB8 occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n80.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n81xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n81.xml
        Test URI:not-wf/P87/ibm87n81.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0FBA occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n81.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p87ibm87n82xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P87-ibm87n82.xml
        Test URI:not-wf/P87/ibm87n82.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x20DD occurs as the second character in the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n82.xml")
            .unwrap()
            .as_str());
}
 */
