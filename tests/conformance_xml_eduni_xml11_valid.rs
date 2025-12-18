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
use xrust::validators::Schema;


#[cfg(all(test, feature = "test-conformance-xml"))]
fn test_eduni_xml11_valid(xmldoc: &str, xmlcanondoc: &str) {
    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        xmldoc,
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );
    let canonicalxml = RNode::new_document();
    let canonicalparseresult = xml::parse(
        canonicalxml,
        xmlcanondoc,
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert!(canonicalparseresult.is_ok());

    let doc = parseresult.unwrap();

    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_ok());

    assert_eq!(doc.get_canonical().unwrap(), canonicalparseresult.unwrap());
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
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt006() {
    /*
        Test ID:rmt-006
        Test URI:006.xml
        Spec Sections:2.8 4.3.4
        Description:Second-level external general entity has later version number than first-level, but not later than document, so not an error.
    */

    test_eduni_xml11_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/006.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/006.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt007() {
    /*
        Test ID:rmt-007
        Test URI:007.xml
        Spec Sections:2.8 4.3.4
        Description:A vanilla XML 1.1 document
    */

    test_eduni_xml11_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/007.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/007.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt010() {
    /*
        Test ID:rmt-010
        Test URI:010.xml
        Spec Sections:2.2
        Description:Contains a C1 control, legal in XML 1.0, illegal in XML 1.1
    */

    test_eduni_xml11_valid(
        non_utf8_file_reader("tests/conformance/xml/xmlconf/eduni/xml-1.1/010.xml").as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/010.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt012() {
    /*
        Test ID:rmt-012
        Test URI:012.xml
        Spec Sections:2.2
        Description:Contains a DEL, legal in XML 1.0, illegal in XML 1.1
    */

    test_eduni_xml11_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/012.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/012.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt022() {
    /*
        Test ID:rmt-022
        Test URI:022.xml
        Spec Sections:2.11
        Description:Has a NEL character; legal in both XML 1.0 and 1.1, but different canonical output because of normalization in 1.1
    */

    test_eduni_xml11_valid(
        non_utf8_file_reader("tests/conformance/xml/xmlconf/eduni/xml-1.1/022.xml").as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/022.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt023() {
    /*
        Test ID:rmt-023
        Test URI:023.xml
        Spec Sections:2.11
        Description:Has a NEL character; legal in both XML 1.0 and 1.1, but different canonical output because of normalization in 1.1
    */

    test_eduni_xml11_valid(
        non_utf8_file_reader("tests/conformance/xml/xmlconf/eduni/xml-1.1/023.xml").as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/023.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt024() {
    /*
        Test ID:rmt-024
        Test URI:024.xml
        Spec Sections:2.11
        Description:Has an LSEP character; legal in both XML 1.0 and 1.1, but different canonical output because of normalization in 1.1
    */

    test_eduni_xml11_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/024.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/024.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt025() {
    /*
        Test ID:rmt-025
        Test URI:025.xml
        Spec Sections:2.11
        Description:Has an LSEP character; legal in both XML 1.0 and 1.1, but different canonical output because of normalization in 1.1
    */

    test_eduni_xml11_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/025.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/025.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt026() {
    /*
        Test ID:rmt-026
        Test URI:026.xml
        Spec Sections:2.11
        Description:Has CR-NEL; legal in both XML 1.0 and 1.1, but different canonical output because of normalization in 1.1
    */

    test_eduni_xml11_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/026.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/026.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt027() {
    /*
        Test ID:rmt-027
        Test URI:027.xml
        Spec Sections:2.11
        Description:Has CR-NEL; legal in both XML 1.0 and 1.1, but different canonical output because of normalization in 1.1
    */

    test_eduni_xml11_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/027.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/027.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt028() {
    /*
        Test ID:rmt-028
        Test URI:028.xml
        Spec Sections:2.11
        Description:Has CR-LSEP; legal in both XML 1.0 and 1.1, but different canonical output because of normalization in 1.1. Note that CR and LSEP are not combined into a single LF
    */

    test_eduni_xml11_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/028.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/028.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt029() {
    /*
        Test ID:rmt-029
        Test URI:029.xml
        Spec Sections:2.11
        Description:Has CR-LSEP; legal in both XML 1.0 and 1.1, but different canonical output because of normalization in 1.1
    */

    test_eduni_xml11_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/029.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/029.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt031() {
    /*
        Test ID:rmt-031
        Test URI:031.xml
        Spec Sections:2.11
        Description:Has a NEL character in an NMTOKENS attribute; well-formed in both XML 1.0 and 1.1, but valid only in 1.1
    */

    test_eduni_xml11_valid(
        non_utf8_file_reader("tests/conformance/xml/xmlconf/eduni/xml-1.1/031.xml").as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/031.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt033() {
    /*
        Test ID:rmt-033
        Test URI:033.xml
        Spec Sections:2.11
        Description:Has an LSEP character in an NMTOKENS attribute; well-formed in both XML 1.0 and 1.1, but valid only in 1.1
    */

    test_eduni_xml11_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/033.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/033.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt034() {
    /*
        Test ID:rmt-034
        Test URI:034.xml
        Spec Sections:2.3
        Description:Has an NMTOKENS attribute containing a CR character that comes from a character reference in an internal entity. Because CR is in the S production, this is valid in both XML 1.0 and 1.1.
    */

    test_eduni_xml11_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/034.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/034.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt035() {
    /*
        Test ID:rmt-035
        Test URI:035.xml
        Spec Sections:2.3
        Description:Has an NMTOKENS attribute containing a CR character that comes from a character reference in an internal entity. Because CR is in the S production, this is valid in both XML 1.0 and 1.1.
    */

    test_eduni_xml11_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/035.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/035.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt040() {
    /*
        Test ID:rmt-040
        Test URI:040.xml
        Spec Sections:2.2
        Description:Contains a C1 control character (partial line up), legal in XML 1.0 but not 1.1
    */

    test_eduni_xml11_valid(
        non_utf8_file_reader("tests/conformance/xml/xmlconf/eduni/xml-1.1/040.xml").as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/040.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt043() {
    /*
        Test ID:rmt-043
        Test URI:043.xml
        Spec Sections:4.1
        Description:Contains a character reference to a C0 control character (form-feed), legal in XML 1.1 but not 1.0
    */

    test_eduni_xml11_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/043.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/043.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt044() {
    /*
        Test ID:rmt-044
        Test URI:044.xml
        Spec Sections:4.1
        Description:Contains a character reference to a C1 control character (partial line up), legal in both XML 1.0 and 1.1 (but for different reasons)
    */

    test_eduni_xml11_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/044.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/044.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt045() {
    /*
        Test ID:rmt-045
        Test URI:045.xml
        Spec Sections:4.1
        Description:Contains a character reference to a C1 control character (partial line up), legal in both XML 1.0 and 1.1 (but for different reasons)
    */

    test_eduni_xml11_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/045.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/045.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt047() {
    /*
        Test ID:rmt-047
        Test URI:047.xml
        Spec Sections:2.11
        Description:Has a NEL character in element content whitespace; well-formed in both XML 1.0 and 1.1, but valid only in 1.1
    */

    test_eduni_xml11_valid(
        non_utf8_file_reader("tests/conformance/xml/xmlconf/eduni/xml-1.1/047.xml").as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/047.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt049() {
    /*
        Test ID:rmt-049
        Test URI:049.xml
        Spec Sections:2.11
        Description:has an LSEP character in element content whitespace; well-formed in both XML 1.0 and 1.1, but valid only in 1.1
    */

    test_eduni_xml11_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/049.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/049.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt050() {
    /*
        Test ID:rmt-050
        Test URI:050.xml
        Spec Sections:2.3
        Description:Has element content whitespace containing a CR character that comes from a character reference in an internal entity. Because CR is in the S production, this is valid in both XML 1.0 and 1.1.
    */

    test_eduni_xml11_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/050.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/050.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt051() {
    /*
        Test ID:rmt-051
        Test URI:051.xml
        Spec Sections:2.3
        Description:Has element content whitespace containing a CR character that comes from a character reference in an internal entity. Because CR is in the S production, this is valid in both XML 1.0 and 1.1.
    */

    test_eduni_xml11_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/051.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/051.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn rmt054() {
    /*
        Test ID:rmt-054
        Test URI:054.xml
        Spec Sections:4.3.2
        Description:Contains a character reference to a C0 control character (form-feed) in an entity value. This will be legal (in XML 1.1) when the entity declaration is parsed, but what about when it is used? According to the grammar in the CR spec, it should be illegal (because the replacement text must match "content"), but this is probably not intended. This will be fixed in the PR version.
    */

    test_eduni_xml11_valid(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/054.xml")
            .unwrap()
            .as_str(),
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/xml-1.1/out/054.xml")
            .unwrap()
            .as_str(),
    );
}
