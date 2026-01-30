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
#[cfg(all(test, feature = "test-conformance-xml"))]
use encoding_rs::UTF_8;
#[cfg(all(test, feature = "test-conformance-xml"))]
use encoding_rs::UTF_16BE;
#[cfg(all(test, feature = "test-conformance-xml"))]
use encoding_rs::UTF_16LE;
#[cfg(all(test, feature = "test-conformance-xml"))]
use encoding_rs::WINDOWS_1252;
#[cfg(all(test, feature = "test-conformance-xml"))]
use encoding_rs_io::DecodeReaderBytesBuilder;
#[cfg(all(test, feature = "test-conformance-xml"))]
use std::fs::File;
#[cfg(all(test, feature = "test-conformance-xml"))]
use std::io::{Read, Seek, SeekFrom};
#[cfg(all(test, feature = "test-conformance-xml"))]
pub fn non_utf8_file_reader(filedir: &str) -> String {
    /*
       xRust itself will most likely be UTF-8 only, but there are UTF-16 files in the conformance
       suite. I could change them, but best leave as-is in case we do try to support later.
    */
    let mut file_in = File::open(filedir).unwrap();
    let mut buffer = [0; 4];

    // read exactly 4 bytes
    let _ = file_in.read_exact(&mut buffer);
    let _ = file_in.seek(SeekFrom::Start(0));

    let enc = match buffer {
        //[0, 0, 254, 255] => {} //UCS-4, big-endian machine (1234 order)
        //[255, 254, 0, 0] => {} //UCS-4, little-endian machine (4321 order)
        //[0, 0, 255, 254] => {} //UCS-4, unusual octet order (2143)
        //[254, 255, 0, 0] => {} //UCS-4, unusual octet order (3412)
        [254, 255, _, _] => Some(UTF_16BE), //UTF-16, big-endian
        [255, 254, _, _] => Some(UTF_16LE), //UTF-16, little-endian
        [239, 187, 191, _] => Some(UTF_8),  //UTF-8
        [60, 63, 120, 109] => Some(WINDOWS_1252), //UTF-8
        _ => Some(UTF_8),                   //Other
    };

    let mut decoded_stream = DecodeReaderBytesBuilder::new().encoding(enc).build(file_in);

    let mut dest = String::new();
    let _ = decoded_stream.read_to_string(&mut dest);

    dest
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n167xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n167.xml
        Test URI:not-wf/P85/ibm85n167.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x11BB occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n167.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n168xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n168.xml
        Test URI:not-wf/P85/ibm85n168.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x11C3 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n168.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n169xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n169.xml
        Test URI:not-wf/P85/ibm85n169.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x11F1 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n169.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n17xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n17.xml
        Test URI:not-wf/P85/ibm85n17.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x02AF occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n17.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n170xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n170.xml
        Test URI:not-wf/P85/ibm85n170.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x11FA occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n170.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n171xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n171.xml
        Test URI:not-wf/P85/ibm85n171.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1E9C occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n171.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n172xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n172.xml
        Test URI:not-wf/P85/ibm85n172.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1EFA occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n172.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n173xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n173.xml
        Test URI:not-wf/P85/ibm85n173.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1F16 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n173.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n174xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n174.xml
        Test URI:not-wf/P85/ibm85n174.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1F1E occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n174.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n175xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n175.xml
        Test URI:not-wf/P85/ibm85n175.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1F46 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n175.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n176xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n176.xml
        Test URI:not-wf/P85/ibm85n176.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1F4F occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n176.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n177xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n177.xml
        Test URI:not-wf/P85/ibm85n177.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1F58 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n177.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n178xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n178.xml
        Test URI:not-wf/P85/ibm85n178.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1F5A occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n178.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n179xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n179.xml
        Test URI:not-wf/P85/ibm85n179.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1F5C occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n179.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n18xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n18.xml
        Test URI:not-wf/P85/ibm85n18.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x02CF occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n18.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n180xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n180.xml
        Test URI:not-wf/P85/ibm85n180.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1F5E occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n180.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n181xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n181.xml
        Test URI:not-wf/P85/ibm85n181.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1F7E occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n181.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n182xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n182.xml
        Test URI:not-wf/P85/ibm85n182.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1FB5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n182.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n183xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n183.xml
        Test URI:not-wf/P85/ibm85n183.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1FBD occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n183.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n184xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n184.xml
        Test URI:not-wf/P85/ibm85n184.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1FBF occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n184.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n185xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n185.xml
        Test URI:not-wf/P85/ibm85n185.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1FC5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n185.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n186xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n186.xml
        Test URI:not-wf/P85/ibm85n186.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1FCD occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n186.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n187xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n187.xml
        Test URI:not-wf/P85/ibm85n187.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1FD5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n187.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n188xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n188.xml
        Test URI:not-wf/P85/ibm85n188.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1FDC occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n188.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n189xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n189.xml
        Test URI:not-wf/P85/ibm85n189.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1FED occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n189.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n19xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n19.xml
        Test URI:not-wf/P85/ibm85n19.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0387 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n19.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n190xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n190.xml
        Test URI:not-wf/P85/ibm85n190.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1FF5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n190.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n191xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n191.xml
        Test URI:not-wf/P85/ibm85n191.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1FFD occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n191.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n192xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n192.xml
        Test URI:not-wf/P85/ibm85n192.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x2127 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n192.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n193xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n193.xml
        Test URI:not-wf/P85/ibm85n193.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x212F occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n193.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n194xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n194.xml
        Test URI:not-wf/P85/ibm85n194.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x2183 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n194.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n195xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n195.xml
        Test URI:not-wf/P85/ibm85n195.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x3095 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n195.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n196xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n196.xml
        Test URI:not-wf/P85/ibm85n196.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x30FB occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n196.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n197xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n197.xml
        Test URI:not-wf/P85/ibm85n197.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x312D occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n197.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n198xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n198.xml
        Test URI:not-wf/P85/ibm85n198.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #xD7A4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n198.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n20xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n20.xml
        Test URI:not-wf/P85/ibm85n20.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x038B occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n20.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n21xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n21.xml
        Test URI:not-wf/P85/ibm85n21.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x03A2 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n21.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n22xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n22.xml
        Test URI:not-wf/P85/ibm85n22.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x03CF occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n22.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n23xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n23.xml
        Test URI:not-wf/P85/ibm85n23.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x03D7 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n23.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n24xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n24.xml
        Test URI:not-wf/P85/ibm85n24.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x03DD occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n24.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n25xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n25.xml
        Test URI:not-wf/P85/ibm85n25.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x03E1 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n25.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n26xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n26.xml
        Test URI:not-wf/P85/ibm85n26.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x03F4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n26.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n27xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n27.xml
        Test URI:not-wf/P85/ibm85n27.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x040D occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n27.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n28xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n28.xml
        Test URI:not-wf/P85/ibm85n28.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0450 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n28.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n29xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n29.xml
        Test URI:not-wf/P85/ibm85n29.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x045D occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n29.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n30xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n30.xml
        Test URI:not-wf/P85/ibm85n30.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0482 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n30.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n31xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n31.xml
        Test URI:not-wf/P85/ibm85n31.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x04C5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n31.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n32xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n32.xml
        Test URI:not-wf/P85/ibm85n32.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x04C6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n32.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n33xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n33.xml
        Test URI:not-wf/P85/ibm85n33.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x04C9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n33.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n34xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n34.xml
        Test URI:not-wf/P85/ibm85n34.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x04EC occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n34.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n35xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n35.xml
        Test URI:not-wf/P85/ibm85n35.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x04ED occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n35.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n36xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n36.xml
        Test URI:not-wf/P85/ibm85n36.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x04F6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n36.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n37xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n37.xml
        Test URI:not-wf/P85/ibm85n37.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x04FA occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n37.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n38xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n38.xml
        Test URI:not-wf/P85/ibm85n38.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0557 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n38.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n39xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n39.xml
        Test URI:not-wf/P85/ibm85n39.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0558 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n39.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n40xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n40.xml
        Test URI:not-wf/P85/ibm85n40.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0587 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n40.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n41xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n41.xml
        Test URI:not-wf/P85/ibm85n41.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x05EB occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n41.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n42xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n42.xml
        Test URI:not-wf/P85/ibm85n42.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x05F3 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n42.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n43xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n43.xml
        Test URI:not-wf/P85/ibm85n43.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0620 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n43.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n44xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n44.xml
        Test URI:not-wf/P85/ibm85n44.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x063B occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n44.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n45xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n45.xml
        Test URI:not-wf/P85/ibm85n45.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x064B occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n45.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n46xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n46.xml
        Test URI:not-wf/P85/ibm85n46.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x06B8 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n46.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n47xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n47.xml
        Test URI:not-wf/P85/ibm85n47.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x06BF occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n47.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n48xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n48.xml
        Test URI:not-wf/P85/ibm85n48.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x06CF occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n48.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n49xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n49.xml
        Test URI:not-wf/P85/ibm85n49.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x06D4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n49.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n50xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n50.xml
        Test URI:not-wf/P85/ibm85n50.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x06D6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n50.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n51xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n51.xml
        Test URI:not-wf/P85/ibm85n51.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x06E7 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n51.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n52xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n52.xml
        Test URI:not-wf/P85/ibm85n52.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x093A occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n52.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n53xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n53.xml
        Test URI:not-wf/P85/ibm85n53.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x093E occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n53.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n54xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n54.xml
        Test URI:not-wf/P85/ibm85n54.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0962 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n54.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n55xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n55.xml
        Test URI:not-wf/P85/ibm85n55.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x098D occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n55.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n56xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n56.xml
        Test URI:not-wf/P85/ibm85n56.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0991 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n56.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n57xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n57.xml
        Test URI:not-wf/P85/ibm85n57.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0992 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n57.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n58xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n58.xml
        Test URI:not-wf/P85/ibm85n58.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x09A9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n58.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n59xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n59.xml
        Test URI:not-wf/P85/ibm85n59.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x09B1 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n59.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n60xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n60.xml
        Test URI:not-wf/P85/ibm85n60.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x09B5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n60.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n61xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n61.xml
        Test URI:not-wf/P85/ibm85n61.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x09BA occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n61.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n62xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n62.xml
        Test URI:not-wf/P85/ibm85n62.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x09DE occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n62.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n63xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n63.xml
        Test URI:not-wf/P85/ibm85n63.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x09E2 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n63.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n64xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n64.xml
        Test URI:not-wf/P85/ibm85n64.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x09F2 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n64.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n65xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n65.xml
        Test URI:not-wf/P85/ibm85n65.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A0B occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n65.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n66xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n66.xml
        Test URI:not-wf/P85/ibm85n66.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A11 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n66.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n67xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n67.xml
        Test URI:not-wf/P85/ibm85n67.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A29 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n67.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n68xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n68.xml
        Test URI:not-wf/P85/ibm85n68.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A31 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n68.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n69xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n69.xml
        Test URI:not-wf/P85/ibm85n69.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A34 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n69.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n70xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n70.xml
        Test URI:not-wf/P85/ibm85n70.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A37 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n70.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n71xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n71.xml
        Test URI:not-wf/P85/ibm85n71.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A3A occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n71.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n72xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n72.xml
        Test URI:not-wf/P85/ibm85n72.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A5D occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n72.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n73xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n73.xml
        Test URI:not-wf/P85/ibm85n73.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A70 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n73.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n74xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n74.xml
        Test URI:not-wf/P85/ibm85n74.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A75 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n74.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n75xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n75.xml
        Test URI:not-wf/P85/ibm85n75.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #xA84 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n75.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n76xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n76.xml
        Test URI:not-wf/P85/ibm85n76.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0ABC occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n76.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n77xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n77.xml
        Test URI:not-wf/P85/ibm85n77.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A92 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n77.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n78xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n78.xml
        Test URI:not-wf/P85/ibm85n78.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0AA9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n78.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n79xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n79.xml
        Test URI:not-wf/P85/ibm85n79.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0AB1 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n79.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n80xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n80.xml
        Test URI:not-wf/P85/ibm85n80.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0AB4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n80.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n81xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n81.xml
        Test URI:not-wf/P85/ibm85n81.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0ABA occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n81.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n82xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n82.xml
        Test URI:not-wf/P85/ibm85n82.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B04 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n82.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n83xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n83.xml
        Test URI:not-wf/P85/ibm85n83.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B0D occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n83.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n84xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n84.xml
        Test URI:not-wf/P85/ibm85n84.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B11 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n84.xml")
            .unwrap()
            .as_str());
}
 */
