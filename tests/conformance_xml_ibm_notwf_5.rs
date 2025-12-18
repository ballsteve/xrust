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

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p81ibm81n07xml() {
    /*
        Test ID:ibm-not-wf-P81-ibm81n07.xml
        Test URI:not-wf/P81/ibm81n07.xml
        Spec Sections:4.3.3
        Description:Tests EncName with an illegal character. The ":" is used as one character in the EncName in the EncodingDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P81/ibm81n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p81ibm81n08xml() {
    /*
        Test ID:ibm-not-wf-P81-ibm81n08.xml
        Test URI:not-wf/P81/ibm81n08.xml
        Spec Sections:4.3.3
        Description:Tests EncName with an illegal character. The "/" is used as one character in the EncName in the EncodingDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P81/ibm81n08.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p81ibm81n09xml() {
    /*
        Test ID:ibm-not-wf-P81-ibm81n09.xml
        Test URI:not-wf/P81/ibm81n09.xml
        Spec Sections:4.3.3
        Description:Tests EncName with an illegal character. The ";" is used as one character in the EncName in the EncodingDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P81/ibm81n09.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p82ibm82n01xml() {
    /*
        Test ID:ibm-not-wf-P82-ibm82n01.xml
        Test URI:not-wf/P82/ibm82n01.xml
        Spec Sections:4.7
        Description:Tests NotationDecl with a required field missing. The white space after the beginning sequence of the NotationDecl is missing in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P82/ibm82n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p82ibm82n02xml() {
    /*
        Test ID:ibm-not-wf-P82-ibm82n02.xml
        Test URI:not-wf/P82/ibm82n02.xml
        Spec Sections:4.7
        Description:Tests NotationDecl with a required field missing. The Name in the NotationDecl is missing in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P82/ibm82n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p82ibm82n03xml() {
    /*
        Test ID:ibm-not-wf-P82-ibm82n03.xml
        Test URI:not-wf/P82/ibm82n03.xml
        Spec Sections:4.7
        Description:Tests NotationDecl with a required field missing. The externalID or the PublicID is missing in the NotationDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P82/ibm82n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p82ibm82n04xml() {
    /*
        Test ID:ibm-not-wf-P82-ibm82n04.xml
        Test URI:not-wf/P82/ibm82n04.xml
        Spec Sections:4.7
        Description:Tests NotationDecl with wrong field ordering. The Name occurs after the "SYSTEM" and the externalID in the NotationDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P82/ibm82n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p82ibm82n05xml() {
    /*
        Test ID:ibm-not-wf-P82-ibm82n05.xml
        Test URI:not-wf/P82/ibm82n05.xml
        Spec Sections:4.7
        Description:Tests NotationDecl with wrong key word. The string "notation" is used as a key word in the NotationDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P82/ibm82n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p82ibm82n06xml() {
    /*
        Test ID:ibm-not-wf-P82-ibm82n06.xml
        Test URI:not-wf/P82/ibm82n06.xml
        Spec Sections:4.7
        Description:Tests NotationDecl with a required field missing. The closing bracket (the greater than character) is missing in the NotationDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P82/ibm82n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p82ibm82n07xml() {
    /*
        Test ID:ibm-not-wf-P82-ibm82n07.xml
        Test URI:not-wf/P82/ibm82n07.xml
        Spec Sections:4.7
        Description:Tests NotationDecl with wrong beginning sequence. The "!" is missing in the beginning sequence in the NotationDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P82/ibm82n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p82ibm82n08xml() {
    /*
        Test ID:ibm-not-wf-P82-ibm82n08.xml
        Test URI:not-wf/P82/ibm82n08.xml
        Spec Sections:4.7
        Description:Tests NotationDecl with wrong closing sequence. The extra "!" occurs in the closing sequence in the NotationDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P82/ibm82n08.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p83ibm83n01xml() {
    /*
        Test ID:ibm-not-wf-P83-ibm83n01.xml
        Test URI:not-wf/P83/ibm83n01.xml
        Spec Sections:4.7
        Description:Tests PublicID with wrong key word. The string "public" is used as the key word in the PublicID in the NotationDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P83/ibm83n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p83ibm83n02xml() {
    /*
        Test ID:ibm-not-wf-P83-ibm83n02.xml
        Test URI:not-wf/P83/ibm83n02.xml
        Spec Sections:4.7
        Description:Tests PublicID with wrong key word. The string "Public" is used as the key word in the PublicID in the NotationDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P83/ibm83n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p83ibm83n03xml() {
    /*
        Test ID:ibm-not-wf-P83-ibm83n03.xml
        Test URI:not-wf/P83/ibm83n03.xml
        Spec Sections:4.7
        Description:Tests PublicID with a required field missing. The key word "PUBLIC" is missing in the PublicID in the NotationDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P83/ibm83n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p83ibm83n04xml() {
    /*
        Test ID:ibm-not-wf-P83-ibm83n04.xml
        Test URI:not-wf/P83/ibm83n04.xml
        Spec Sections:4.7
        Description:Tests PublicID with a required field missing. The white space between the "PUBLIC" and the PubidLiteral is missing in the PublicID in the NotationDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P83/ibm83n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p83ibm83n05xml() {
    /*
        Test ID:ibm-not-wf-P83-ibm83n05.xml
        Test URI:not-wf/P83/ibm83n05.xml
        Spec Sections:4.7
        Description:Tests PublicID with a required field missing. The PubidLiteral is missing in the PublicID in the NotationDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P83/ibm83n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p83ibm83n06xml() {
    /*
        Test ID:ibm-not-wf-P83-ibm83n06.xml
        Test URI:not-wf/P83/ibm83n06.xml
        Spec Sections:4.7
        Description:Tests PublicID with wrong field ordering. The key word "PUBLIC" occurs after the PubidLiteral in the PublicID in the NotationDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P83/ibm83n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n01xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n01.xml
        Test URI:not-wf/P85/ibm85n01.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x00D7 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n02xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n02.xml
        Test URI:not-wf/P85/ibm85n02.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x00F7 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n02.xml")
            .unwrap()
            .as_str(),
    );
}

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n03xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n03.xml
        Test URI:not-wf/P85/ibm85n03.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0132 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n03.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n04xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n04.xml
        Test URI:not-wf/P85/ibm85n04.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0133 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n04.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n05xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n05.xml
        Test URI:not-wf/P85/ibm85n05.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x013F occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n05.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n06xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n06.xml
        Test URI:not-wf/P85/ibm85n06.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0140 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n06.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n07xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n07.xml
        Test URI:not-wf/P85/ibm85n07.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0149 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n07.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n08xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n08.xml
        Test URI:not-wf/P85/ibm85n08.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x017F occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n08.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n09xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n09.xml
        Test URI:not-wf/P85/ibm85n09.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x01c4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n09.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n10xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n10.xml
        Test URI:not-wf/P85/ibm85n10.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x01CC occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n10.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n100xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n100.xml
        Test URI:not-wf/P85/ibm85n100.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0BB6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n100.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n101xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n101.xml
        Test URI:not-wf/P85/ibm85n101.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0BBA occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n101.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n102xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n102.xml
        Test URI:not-wf/P85/ibm85n102.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0C0D occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n102.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n103xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n103.xml
        Test URI:not-wf/P85/ibm85n103.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0C11 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n103.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n104xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n104.xml
        Test URI:not-wf/P85/ibm85n104.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0C29 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n104.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n105xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n105.xml
        Test URI:not-wf/P85/ibm85n105.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0C34 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n105.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n106xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n106.xml
        Test URI:not-wf/P85/ibm85n106.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0C5F occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n106.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n107xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n107.xml
        Test URI:not-wf/P85/ibm85n107.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0C62 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n107.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n108xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n108.xml
        Test URI:not-wf/P85/ibm85n108.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0C8D occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n108.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n109xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n109.xml
        Test URI:not-wf/P85/ibm85n109.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0C91 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n109.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n11xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n11.xml
        Test URI:not-wf/P85/ibm85n11.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x01F1 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n11.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n110xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n110.xml
        Test URI:not-wf/P85/ibm85n110.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0CA9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n110.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n111xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n111.xml
        Test URI:not-wf/P85/ibm85n111.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0CB4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n111.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n112xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n112.xml
        Test URI:not-wf/P85/ibm85n112.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0CBA occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n112.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n113xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n113.xml
        Test URI:not-wf/P85/ibm85n113.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0CDF occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n113.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n114xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n114.xml
        Test URI:not-wf/P85/ibm85n114.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0CE2 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n114.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n115xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n115.xml
        Test URI:not-wf/P85/ibm85n115.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0D0D occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n115.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n116xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n116.xml
        Test URI:not-wf/P85/ibm85n116.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0D11 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n116.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n117xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n117.xml
        Test URI:not-wf/P85/ibm85n117.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0D29 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n117.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n118xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n118.xml
        Test URI:not-wf/P85/ibm85n118.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0D3A occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n118.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n119xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n119.xml
        Test URI:not-wf/P85/ibm85n119.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0D62 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n119.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n12xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n12.xml
        Test URI:not-wf/P85/ibm85n12.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x01F3 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n12.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n120xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n120.xml
        Test URI:not-wf/P85/ibm85n120.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0E2F occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n120.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n121xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n121.xml
        Test URI:not-wf/P85/ibm85n121.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0E31 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n121.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n122xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n122.xml
        Test URI:not-wf/P85/ibm85n122.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0E34 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n122.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n123xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n123.xml
        Test URI:not-wf/P85/ibm85n123.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0E46 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n123.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n124xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n124.xml
        Test URI:not-wf/P85/ibm85n124.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0E83 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n124.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n125xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n125.xml
        Test URI:not-wf/P85/ibm85n125.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0E85 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n125.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n126xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n126.xml
        Test URI:not-wf/P85/ibm85n126.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0E89 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n126.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n127xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n127.xml
        Test URI:not-wf/P85/ibm85n127.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0E8B occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n127.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n128xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n128.xml
        Test URI:not-wf/P85/ibm85n128.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0E8E occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n128.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n129xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n129.xml
        Test URI:not-wf/P85/ibm85n129.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0E98 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n129.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n13xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n13.xml
        Test URI:not-wf/P85/ibm85n13.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x01F6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n13.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n130xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n130.xml
        Test URI:not-wf/P85/ibm85n130.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0EA0 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n130.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n131xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n131.xml
        Test URI:not-wf/P85/ibm85n131.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0EA4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n131.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n132xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n132.xml
        Test URI:not-wf/P85/ibm85n132.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0EA6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n132.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n133xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n133.xml
        Test URI:not-wf/P85/ibm85n133.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0EA8 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n133.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n134xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n134.xml
        Test URI:not-wf/P85/ibm85n134.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0EAC occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n134.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n135xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n135.xml
        Test URI:not-wf/P85/ibm85n135.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0EAF occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n135.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n136xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n136.xml
        Test URI:not-wf/P85/ibm85n136.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0EB1 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n136.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n137xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n137.xml
        Test URI:not-wf/P85/ibm85n137.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0EB4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n137.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n138xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n138.xml
        Test URI:not-wf/P85/ibm85n138.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0EBE occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n138.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n139xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n139.xml
        Test URI:not-wf/P85/ibm85n139.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0EC5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n139.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n14xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n14.xml
        Test URI:not-wf/P85/ibm85n14.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x01F9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n14.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n140xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n140.xml
        Test URI:not-wf/P85/ibm85n140.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0F48 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n140.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n141xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n141.xml
        Test URI:not-wf/P85/ibm85n141.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0F6A occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n141.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n142xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n142.xml
        Test URI:not-wf/P85/ibm85n142.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x10C6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n142.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n143xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n143.xml
        Test URI:not-wf/P85/ibm85n143.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x10F7 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n143.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n144xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n144.xml
        Test URI:not-wf/P85/ibm85n144.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1011 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n144.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n145xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n145.xml
        Test URI:not-wf/P85/ibm85n145.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1104 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n145.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n146xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n146.xml
        Test URI:not-wf/P85/ibm85n146.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1108 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n146.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n147xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n147.xml
        Test URI:not-wf/P85/ibm85n147.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x110A occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n147.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n148xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n148.xml
        Test URI:not-wf/P85/ibm85n148.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x110D occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n148.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n149xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n149.xml
        Test URI:not-wf/P85/ibm85n149.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x113B occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n149.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n15xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n15.xml
        Test URI:not-wf/P85/ibm85n15.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x01F9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n15.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n150xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n150.xml
        Test URI:not-wf/P85/ibm85n150.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x113F occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n150.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n151xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n151.xml
        Test URI:not-wf/P85/ibm85n151.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1141 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n151.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n152xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n152.xml
        Test URI:not-wf/P85/ibm85n152.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x114D occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n152.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n153xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n153.xml
        Test URI:not-wf/P85/ibm85n153.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x114f occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n153.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n154xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n154.xml
        Test URI:not-wf/P85/ibm85n154.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1151 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n154.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n155xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n155.xml
        Test URI:not-wf/P85/ibm85n155.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1156 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n155.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n156xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n156.xml
        Test URI:not-wf/P85/ibm85n156.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x115A occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n156.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n157xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n157.xml
        Test URI:not-wf/P85/ibm85n157.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1162 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n157.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n158xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n158.xml
        Test URI:not-wf/P85/ibm85n158.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1164 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n158.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n159xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n159.xml
        Test URI:not-wf/P85/ibm85n159.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1166 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n159.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n16xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n16.xml
        Test URI:not-wf/P85/ibm85n16.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0230 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n16.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n160xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n160.xml
        Test URI:not-wf/P85/ibm85n160.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x116B occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n160.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n161xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n161.xml
        Test URI:not-wf/P85/ibm85n161.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x116F occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n161.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n162xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n162.xml
        Test URI:not-wf/P85/ibm85n162.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1174 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n162.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n163xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n163.xml
        Test URI:not-wf/P85/ibm85n163.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x119F occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n163.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n164xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n164.xml
        Test URI:not-wf/P85/ibm85n164.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x11AC occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n164.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n165xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n165.xml
        Test URI:not-wf/P85/ibm85n165.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x11B6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n165.xml")
            .unwrap()
            .as_str());
}
 */

/*
#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p85ibm85n166xml() {
    /*
        This test is deliberately ignored.
        This document is now well formed, as per the 5th edition.
    */
    /*
        Test ID:ibm-not-wf-P85-ibm85n166.xml
        Test URI:not-wf/P85/ibm85n166.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x11B9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    test_ibm_notwf(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n166.xml")
            .unwrap()
            .as_str());
}
 */
