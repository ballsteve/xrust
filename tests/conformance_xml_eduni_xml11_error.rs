/*

Richard Tobin's XML 1.1 test suite 13 Feb 2003

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
fn test_eduni_xml11_error(xmldoc: &str) {
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
fn rmt008() {
    /*
        Test ID:rmt-008
        Test URI:008.xml
        Spec Sections:2.8 4.3.4
        Description:an implausibly-versioned document
    */

    test_eduni_xml11_error(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/008.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt009() {
    /*
        Test ID:rmt-009
        Test URI:009.xml
        Spec Sections:2.8 4.3.4
        Description:External general entity has implausible version number
    */

    test_eduni_xml11_error(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/009.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt055() {
    /*
        Test ID:rmt-055
        Test URI:055.xml
        Spec Sections:2.11
        Description:Has a Latin-1 NEL in the XML declaration (to be made an error in PR)
    */

    test_eduni_xml11_error(
        non_utf8_file_reader("tests/conformance/xml/xmlconf/eduni/xml-1.1/055.xml").as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt056() {
    /*
        Test ID:rmt-056
        Test URI:056.xml
        Spec Sections:2.11
        Description:Has a UTF-8 NEL in the XML declaration (to be made an error in PR)
    */

    test_eduni_xml11_error(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/056.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt057() {
    /*
        Test ID:rmt-057
        Test URI:057.xml
        Spec Sections:2.11
        Description:Has a UTF-8 LSEP in the XML declaration (to be made an error in PR)
    */

    test_eduni_xml11_error(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/057.xml")
            .unwrap()
            .as_str(),
    );
}
