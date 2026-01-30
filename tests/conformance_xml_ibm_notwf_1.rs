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
fn ibmnotwf_p01ibm01n01xml() {
    /*
        Test ID:ibm-not-wf-P01-ibm01n01.xml
        Test URI:not-wf/P01/ibm01n01.xml
        Spec Sections:2.1
        Description:Tests a document with no element. A well-formed document should have at lease one elements.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P01/ibm01n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p01ibm01n02xml() {
    /*
        Test ID:ibm-not-wf-P01-ibm01n02.xml
        Test URI:not-wf/P01/ibm01n02.xml
        Spec Sections:2.1
        Description:Tests a document with wrong ordering of its prolog and element. The element occurs before the xml declaration and the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P01/ibm01n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p01ibm01n03xml() {
    /*
        Test ID:ibm-not-wf-P01-ibm01n03.xml
        Test URI:not-wf/P01/ibm01n03.xml
        Spec Sections:2.1
        Description:Tests a document with wrong combination of misc and element. One PI occurs between two elements.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P01/ibm01n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n01xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n01.xml
        Test URI:not-wf/P02/ibm02n01.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x00
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n02xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n02.xml
        Test URI:not-wf/P02/ibm02n02.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x01
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n03xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n03.xml
        Test URI:not-wf/P02/ibm02n03.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x02
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n04xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n04.xml
        Test URI:not-wf/P02/ibm02n04.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x03
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n05xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n05.xml
        Test URI:not-wf/P02/ibm02n05.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x04
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n06xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n06.xml
        Test URI:not-wf/P02/ibm02n06.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x05
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n07xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n07.xml
        Test URI:not-wf/P02/ibm02n07.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x06
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n08xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n08.xml
        Test URI:not-wf/P02/ibm02n08.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x07
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n08.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n09xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n09.xml
        Test URI:not-wf/P02/ibm02n09.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x08
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n09.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n10xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n10.xml
        Test URI:not-wf/P02/ibm02n10.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x0B
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n10.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n11xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n11.xml
        Test URI:not-wf/P02/ibm02n11.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x0C
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n11.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n12xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n12.xml
        Test URI:not-wf/P02/ibm02n12.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x0E
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n12.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n13xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n13.xml
        Test URI:not-wf/P02/ibm02n13.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x0F
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n13.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n14xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n14.xml
        Test URI:not-wf/P02/ibm02n14.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x10
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n14.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n15xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n15.xml
        Test URI:not-wf/P02/ibm02n15.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x11
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n15.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n16xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n16.xml
        Test URI:not-wf/P02/ibm02n16.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x12
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n16.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n17xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n17.xml
        Test URI:not-wf/P02/ibm02n17.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x13
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n17.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n18xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n18.xml
        Test URI:not-wf/P02/ibm02n18.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x14
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n18.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n19xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n19.xml
        Test URI:not-wf/P02/ibm02n19.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x15
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n19.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n20xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n20.xml
        Test URI:not-wf/P02/ibm02n20.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x16
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n20.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n21xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n21.xml
        Test URI:not-wf/P02/ibm02n21.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x17
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n21.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n22xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n22.xml
        Test URI:not-wf/P02/ibm02n22.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x18
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n22.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n23xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n23.xml
        Test URI:not-wf/P02/ibm02n23.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x19
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n23.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n24xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n24.xml
        Test URI:not-wf/P02/ibm02n24.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x1A
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n24.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n25xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n25.xml
        Test URI:not-wf/P02/ibm02n25.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x1B
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n25.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n26xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n26.xml
        Test URI:not-wf/P02/ibm02n26.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x1C
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n26.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n27xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n27.xml
        Test URI:not-wf/P02/ibm02n27.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x1D
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n27.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n28xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n28.xml
        Test URI:not-wf/P02/ibm02n28.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x1E
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n28.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n29xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n29.xml
        Test URI:not-wf/P02/ibm02n29.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x1F
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n29.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n30xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n30.xml
        Test URI:not-wf/P02/ibm02n30.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #xD800
    */

    test_ibm_notwf(
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n30.xml").as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n31xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n31.xml
        Test URI:not-wf/P02/ibm02n31.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #xDFFF
    */

    test_ibm_notwf(
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n31.xml").as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n32xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n32.xml
        Test URI:not-wf/P02/ibm02n32.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #xFFFE
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n32.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p02ibm02n33xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n33.xml
        Test URI:not-wf/P02/ibm02n33.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #xFFFF
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n33.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p03ibm03n01xml() {
    /*
        Test ID:ibm-not-wf-P03-ibm03n01.xml
        Test URI:not-wf/P03/ibm03n01.xml
        Spec Sections:2.3
        Description:Tests an end tag which contains an illegal space character #x3000 which follows the element name "book".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P03/ibm03n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p04ibm04n01xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n01.xml
        Test URI:not-wf/P04/ibm04n01.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x21
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p04ibm04n02xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n02.xml
        Test URI:not-wf/P04/ibm04n02.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x28
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p04ibm04n03xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n03.xml
        Test URI:not-wf/P04/ibm04n03.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x29
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p04ibm04n04xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n04.xml
        Test URI:not-wf/P04/ibm04n04.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x2B
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p04ibm04n05xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n05.xml
        Test URI:not-wf/P04/ibm04n05.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x2C
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p04ibm04n06xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n06.xml
        Test URI:not-wf/P04/ibm04n06.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x2F
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p04ibm04n07xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n07.xml
        Test URI:not-wf/P04/ibm04n07.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x3B
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p04ibm04n08xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n08.xml
        Test URI:not-wf/P04/ibm04n08.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x3C
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n08.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p04ibm04n09xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n09.xml
        Test URI:not-wf/P04/ibm04n09.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x3D
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n09.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p04ibm04n10xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n10.xml
        Test URI:not-wf/P04/ibm04n10.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x3F
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n10.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p04ibm04n11xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n11.xml
        Test URI:not-wf/P04/ibm04n11.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x5B
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n11.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p04ibm04n12xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n12.xml
        Test URI:not-wf/P04/ibm04n12.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x5C
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n12.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p04ibm04n13xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n13.xml
        Test URI:not-wf/P04/ibm04n13.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x5D
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n13.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p04ibm04n14xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n14.xml
        Test URI:not-wf/P04/ibm04n14.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x5E
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n14.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p04ibm04n15xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n15.xml
        Test URI:not-wf/P04/ibm04n15.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x60
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n15.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p04ibm04n16xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n16.xml
        Test URI:not-wf/P04/ibm04n16.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x7B
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n16.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p04ibm04n17xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n17.xml
        Test URI:not-wf/P04/ibm04n17.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x7C
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n17.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p04ibm04n18xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n18.xml
        Test URI:not-wf/P04/ibm04n18.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x7D
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n18.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p05ibm05n01xml() {
    /*
        Test ID:ibm-not-wf-P05-ibm05n01.xml
        Test URI:not-wf/P05/ibm05n01.xml
        Spec Sections:2.3
        Description:Tests an element name which has an illegal first character. An illegal first character "." is followed by "A_name-starts_with.".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P05/ibm05n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p05ibm05n02xml() {
    /*
        Test ID:ibm-not-wf-P05-ibm05n02.xml
        Test URI:not-wf/P05/ibm05n02.xml
        Spec Sections:2.3
        Description:Tests an element name which has an illegal first character. An illegal first character "-" is followed by "A_name-starts_with-".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P05/ibm05n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p05ibm05n03xml() {
    /*
        Test ID:ibm-not-wf-P05-ibm05n03.xml
        Test URI:not-wf/P05/ibm05n03.xml
        Spec Sections:2.3
        Description:Tests an element name which has an illegal first character. An illegal first character "5" is followed by "A_name-starts_with_digit".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P05/ibm05n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p09ibm09n01xml() {
    /*
        Test ID:ibm-not-wf-P09-ibm09n01.xml
        Test URI:not-wf/P09/ibm09n01.xml
        Spec Sections:2.3
        Description:Tests an internal general entity with an invalid value. The entity "Fullname" contains "%".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P09/ibm09n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p09ibm09n02xml() {
    /*
        Test ID:ibm-not-wf-P09-ibm09n02.xml
        Test URI:not-wf/P09/ibm09n02.xml
        Spec Sections:2.3
        Description:Tests an internal general entity with an invalid value. The entity "Fullname" contains the ampersand character.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P09/ibm09n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p09ibm09n03xml() {
    /*
        Test ID:ibm-not-wf-P09-ibm09n03.xml
        Test URI:not-wf/P09/ibm09n03.xml
        Spec Sections:2.3
        Description:Tests an internal general entity with an invalid value. The entity "Fullname" contains the double quote character in the middle.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P09/ibm09n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p09ibm09n04xml() {
    /*
        Test ID:ibm-not-wf-P09-ibm09n04.xml
        Test URI:not-wf/P09/ibm09n04.xml
        Spec Sections:2.3
        Description:Tests an internal general entity with an invalid value. The closing bracket (double quote) is missing with the value of the entity "FullName".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P09/ibm09n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p10ibm10n01xml() {
    /*
        Test ID:ibm-not-wf-P10-ibm10n01.xml
        Test URI:not-wf/P10/ibm10n01.xml
        Spec Sections:2.3
        Description:Tests an attribute with an invalid value. The value of the attribute "first" contains the character "less than".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P10/ibm10n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p10ibm10n02xml() {
    /*
        Test ID:ibm-not-wf-P10-ibm10n02.xml
        Test URI:not-wf/P10/ibm10n02.xml
        Spec Sections:2.3
        Description:Tests an attribute with an invalid value. The value of the attribute "first" contains the character ampersand.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P10/ibm10n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p10ibm10n03xml() {
    /*
        Test ID:ibm-not-wf-P10-ibm10n03.xml
        Test URI:not-wf/P10/ibm10n03.xml
        Spec Sections:2.3
        Description:Tests an attribute with an invalid value. The value of the attribute "first" contains the double quote character in the middle.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P10/ibm10n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p10ibm10n04xml() {
    /*
        Test ID:ibm-not-wf-P10-ibm10n04.xml
        Test URI:not-wf/P10/ibm10n04.xml
        Spec Sections:2.3
        Description:Tests an attribute with an invalid value. The closing bracket (double quote) is missing with The value of the attribute "first".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P10/ibm10n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p10ibm10n05xml() {
    /*
        Test ID:ibm-not-wf-P10-ibm10n05.xml
        Test URI:not-wf/P10/ibm10n05.xml
        Spec Sections:2.3
        Description:Tests an attribute with an invalid value. The value of the attribute "first" contains the character "less than".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P10/ibm10n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p10ibm10n06xml() {
    /*
        Test ID:ibm-not-wf-P10-ibm10n06.xml
        Test URI:not-wf/P10/ibm10n06.xml
        Spec Sections:2.3
        Description:Tests an attribute with an invalid value. The value of the attribute "first" contains the character ampersand.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P10/ibm10n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p10ibm10n07xml() {
    /*
        Test ID:ibm-not-wf-P10-ibm10n07.xml
        Test URI:not-wf/P10/ibm10n07.xml
        Spec Sections:2.3
        Description:Tests an attribute with an invalid value. The value of the attribute "first" contains the double quote character in the middle.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P10/ibm10n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p10ibm10n08xml() {
    /*
        Test ID:ibm-not-wf-P10-ibm10n08.xml
        Test URI:not-wf/P10/ibm10n08.xml
        Spec Sections:2.3
        Description:Tests an attribute with an invalid value. The closing bracket (single quote) is missing with the value of the attribute "first".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P10/ibm10n08.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p11ibm11n01xml() {
    /*
        Test ID:ibm-not-wf-P11-ibm11n01.xml
        Test URI:not-wf/P11/ibm11n01.xml
        Spec Sections:2.3
        Description:Tests SystemLiteral. The systemLiteral for the element "student" has a double quote character in the middle.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P11/ibm11n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p11ibm11n02xml() {
    /*
        Test ID:ibm-not-wf-P11-ibm11n02.xml
        Test URI:not-wf/P11/ibm11n02.xml
        Spec Sections:2.3
        Description:Tests SystemLiteral. The systemLiteral for the element "student" has a single quote character in the middle.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P11/ibm11n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p11ibm11n03xml() {
    /*
        Test ID:ibm-not-wf-P11-ibm11n03.xml
        Test URI:not-wf/P11/ibm11n03.xml
        Spec Sections:2.3
        Description:Tests SystemLiteral. The closing bracket (double quote) is missing with the systemLiteral for the element "student".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P11/ibm11n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p11ibm11n04xml() {
    /*
        Test ID:ibm-not-wf-P11-ibm11n04.xml
        Test URI:not-wf/P11/ibm11n04.xml
        Spec Sections:2.3
        Description:Tests SystemLiteral. The closing bracket (single quote) is missing with the systemLiteral for the element "student".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P11/ibm11n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p12ibm12n01xml() {
    /*
        Test ID:ibm-not-wf-P12-ibm12n01.xml
        Test URI:not-wf/P12/ibm12n01.xml
        Spec Sections:2.3
        Description:Tests PubidLiteral. The closing bracket (double quote) is missing with the value of the PubidLiteral for the entity "info".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P12/ibm12n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p12ibm12n02xml() {
    /*
        Test ID:ibm-not-wf-P12-ibm12n02.xml
        Test URI:not-wf/P12/ibm12n02.xml
        Spec Sections:2.3
        Description:Tests PubidLiteral. The value of the PubidLiteral for the entity "info" has a single quote character in the middle..
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P12/ibm12n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p12ibm12n03xml() {
    /*
        Test ID:ibm-not-wf-P12-ibm12n03.xml
        Test URI:not-wf/P12/ibm12n03.xml
        Spec Sections:2.3
        Description:Tests PubidLiteral. The closing bracket (single quote) is missing with the value of the PubidLiteral for the entity "info".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P12/ibm12n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p13ibm13n01xml() {
    /*
        Test ID:ibm-not-wf-P13-ibm13n01.xml
        Test URI:not-wf/P13/ibm13n01.xml
        Spec Sections:2.3
        Description:Tests PubidChar. The pubidChar of the PubidLiteral for the entity "info" contains the character "{".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P13/ibm13n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p13ibm13n02xml() {
    /*
        Test ID:ibm-not-wf-P13-ibm13n02.xml
        Test URI:not-wf/P13/ibm13n02.xml
        Spec Sections:2.3
        Description:Tests PubidChar. The pubidChar of the PubidLiteral for the entity "info" contains the character "~".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P13/ibm13n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p13ibm13n03xml() {
    /*
        Test ID:ibm-not-wf-P13-ibm13n03.xml
        Test URI:not-wf/P13/ibm13n03.xml
        Spec Sections:2.3
        Description:Tests PubidChar. The pubidChar of the PubidLiteral for the entity "info" contains the character double quote in the middle.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P13/ibm13n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p14ibm14n01xml() {
    /*
        Test ID:ibm-not-wf-P14-ibm14n01.xml
        Test URI:not-wf/P14/ibm14n01.xml
        Spec Sections:2.4
        Description:Tests CharData. The content of the element "student" contains the sequence close-bracket close-bracket greater-than.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P14/ibm14n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p14ibm14n02xml() {
    /*
        Test ID:ibm-not-wf-P14-ibm14n02.xml
        Test URI:not-wf/P14/ibm14n02.xml
        Spec Sections:2.4
        Description:Tests CharData. The content of the element "student" contains the character "less than".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P14/ibm14n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p14ibm14n03xml() {
    /*
        Test ID:ibm-not-wf-P14-ibm14n03.xml
        Test URI:not-wf/P14/ibm14n03.xml
        Spec Sections:2.4
        Description:Tests CharData. The content of the element "student" contains the character ampersand.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P14/ibm14n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p15ibm15n01xml() {
    /*
        Test ID:ibm-not-wf-P15-ibm15n01.xml
        Test URI:not-wf/P15/ibm15n01.xml
        Spec Sections:2.5
        Description:Tests comment. The text of the second comment contains the character "-".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P15/ibm15n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p15ibm15n02xml() {
    /*
        Test ID:ibm-not-wf-P15-ibm15n02.xml
        Test URI:not-wf/P15/ibm15n02.xml
        Spec Sections:2.5
        Description:Tests comment. The second comment has a wrong closing sequence "-(greater than)".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P15/ibm15n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p15ibm15n03xml() {
    /*
        Test ID:ibm-not-wf-P15-ibm15n03.xml
        Test URI:not-wf/P15/ibm15n03.xml
        Spec Sections:2.5
        Description:Tests comment. The second comment has a wrong beginning sequence "(less than)!-".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P15/ibm15n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p15ibm15n04xml() {
    /*
        Test ID:ibm-not-wf-P15-ibm15n04.xml
        Test URI:not-wf/P15/ibm15n04.xml
        Spec Sections:2.5
        Description:Tests comment. The closing sequence is missing with the second comment.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P15/ibm15n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p16ibm16n01xml() {
    /*
        Test ID:ibm-not-wf-P16-ibm16n01.xml
        Test URI:not-wf/P16/ibm16n01.xml
        Spec Sections:2.6
        Description:Tests PI. The content of the PI includes the sequence "?(greater than)?".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P16/ibm16n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p16ibm16n02xml() {
    /*
        Test ID:ibm-not-wf-P16-ibm16n02.xml
        Test URI:not-wf/P16/ibm16n02.xml
        Spec Sections:2.6
        Description:Tests PI. The PITarget is missing in the PI.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P16/ibm16n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p16ibm16n03xml() {
    /*
        Test ID:ibm-not-wf-P16-ibm16n03.xml
        Test URI:not-wf/P16/ibm16n03.xml
        Spec Sections:2.6
        Description:Tests PI. The PI has a wrong closing sequence ">".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P16/ibm16n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p16ibm16n04xml() {
    /*
        Test ID:ibm-not-wf-P16-ibm16n04.xml
        Test URI:not-wf/P16/ibm16n04.xml
        Spec Sections:2.6
        Description:Tests PI. The closing sequence is missing in the PI.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P16/ibm16n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p17ibm17n01xml() {
    /*
        Test ID:ibm-not-wf-P17-ibm17n01.xml
        Test URI:not-wf/P17/ibm17n01.xml
        Spec Sections:2.6
        Description:Tests PITarget. The PITarget contains the string "XML".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P17/ibm17n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p17ibm17n02xml() {
    /*
        Test ID:ibm-not-wf-P17-ibm17n02.xml
        Test URI:not-wf/P17/ibm17n02.xml
        Spec Sections:2.6
        Description:Tests PITarget. The PITarget contains the string "xML".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P17/ibm17n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p17ibm17n03xml() {
    /*
        Test ID:ibm-not-wf-P17-ibm17n03.xml
        Test URI:not-wf/P17/ibm17n03.xml
        Spec Sections:2.6
        Description:Tests PITarget. The PITarget contains the string "xml".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P17/ibm17n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p17ibm17n04xml() {
    /*
        Test ID:ibm-not-wf-P17-ibm17n04.xml
        Test URI:not-wf/P17/ibm17n04.xml
        Spec Sections:2.6
        Description:Tests PITarget. The PITarget contains the string "xmL".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P17/ibm17n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p18ibm18n01xml() {
    /*
        Test ID:ibm-not-wf-P18-ibm18n01.xml
        Test URI:not-wf/P18/ibm18n01.xml
        Spec Sections:2.7
        Description:Tests CDSect. The CDStart is missing in the CDSect in the content of element "student".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P18/ibm18n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p18ibm18n02xml() {
    /*
        Test ID:ibm-not-wf-P18-ibm18n02.xml
        Test URI:not-wf/P18/ibm18n02.xml
        Spec Sections:2.7
        Description:Tests CDSect. The CDEnd is missing in the CDSect in the content of element "student".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P18/ibm18n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p19ibm19n01xml() {
    /*
        Test ID:ibm-not-wf-P19-ibm19n01.xml
        Test URI:not-wf/P19/ibm19n01.xml
        Spec Sections:2.7
        Description:Tests CDStart. The CDStart contains a lower case string "cdata".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P19/ibm19n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p19ibm19n02xml() {
    /*
        Test ID:ibm-not-wf-P19-ibm19n02.xml
        Test URI:not-wf/P19/ibm19n02.xml
        Spec Sections:2.7
        Description:Tests CDStart. The CDStart contains an extra character "[".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P19/ibm19n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p19ibm19n03xml() {
    /*
        Test ID:ibm-not-wf-P19-ibm19n03.xml
        Test URI:not-wf/P19/ibm19n03.xml
        Spec Sections:2.7
        Description:Tests CDStart. The CDStart contains a wrong character "?".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P19/ibm19n03.xml")
            .unwrap()
            .as_str(),
    );
}
