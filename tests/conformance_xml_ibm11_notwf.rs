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
fn test_ibm11_notwf(xmldoc: &str, docloc: &str) {
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
fn ibm11notwf_p02ibm02n01xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n01.xml
        Test URI:not-wf/P02/ibm02n01.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x1.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n01.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n02xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n02.xml
        Test URI:not-wf/P02/ibm02n02.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x2.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n02.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n03xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n03.xml
        Test URI:not-wf/P02/ibm02n03.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x3.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n03.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n04xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n04.xml
        Test URI:not-wf/P02/ibm02n04.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x4.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n04.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n05xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n05.xml
        Test URI:not-wf/P02/ibm02n05.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x5.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n05.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n06xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n06.xml
        Test URI:not-wf/P02/ibm02n06.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x6.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n06.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n07xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n07.xml
        Test URI:not-wf/P02/ibm02n07.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x7.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n07.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n08xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n08.xml
        Test URI:not-wf/P02/ibm02n08.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x8.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n08.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n09xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n09.xml
        Test URI:not-wf/P02/ibm02n09.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x0.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n09.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n10xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n10.xml
        Test URI:not-wf/P02/ibm02n10.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x100.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n10.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n11xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n11.xml
        Test URI:not-wf/P02/ibm02n11.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x0B.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n11.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n12xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n12.xml
        Test URI:not-wf/P02/ibm02n12.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x0C.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n12.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n14xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n14.xml
        Test URI:not-wf/P02/ibm02n14.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x0E.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n14.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n15xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n15.xml
        Test URI:not-wf/P02/ibm02n15.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x0F.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n15.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n16xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n16.xml
        Test URI:not-wf/P02/ibm02n16.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x10.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n16.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n17xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n17.xml
        Test URI:not-wf/P02/ibm02n17.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x11.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n17.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n18xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n18.xml
        Test URI:not-wf/P02/ibm02n18.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x12.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n18.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n19xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n19.xml
        Test URI:not-wf/P02/ibm02n19.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x13.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n19.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n20xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n20.xml
        Test URI:not-wf/P02/ibm02n20.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x14.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n20.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n21xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n21.xml
        Test URI:not-wf/P02/ibm02n21.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x15.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n21.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n22xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n22.xml
        Test URI:not-wf/P02/ibm02n22.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x16.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n22.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n23xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n23.xml
        Test URI:not-wf/P02/ibm02n23.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x17.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n23.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n24xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n24.xml
        Test URI:not-wf/P02/ibm02n24.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x18.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n24.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n25xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n25.xml
        Test URI:not-wf/P02/ibm02n25.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x19.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n25.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n26xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n26.xml
        Test URI:not-wf/P02/ibm02n26.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x1A.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n26.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n27xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n27.xml
        Test URI:not-wf/P02/ibm02n27.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x1B.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n27.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n28xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n28.xml
        Test URI:not-wf/P02/ibm02n28.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x1C.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n28.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n29xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n29.xml
        Test URI:not-wf/P02/ibm02n29.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x1D.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n29.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n30xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n30.xml
        Test URI:not-wf/P02/ibm02n30.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x1E.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n30.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n31xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n31.xml
        Test URI:not-wf/P02/ibm02n31.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x1F.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n31.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n32xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n32.xml
        Test URI:not-wf/P02/ibm02n32.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x7F.
    */
    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n32.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n33xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n33.xml
        Test URI:not-wf/P02/ibm02n33.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x80.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n33.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n34xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n34.xml
        Test URI:not-wf/P02/ibm02n34.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x81.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n34.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n35xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n35.xml
        Test URI:not-wf/P02/ibm02n35.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x82.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n35.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n36xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n36.xml
        Test URI:not-wf/P02/ibm02n36.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x83.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n36.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n37xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n37.xml
        Test URI:not-wf/P02/ibm02n37.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x84.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n37.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n38xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n38.xml
        Test URI:not-wf/P02/ibm02n38.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control characters x82, x83 and x84.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n38.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n39xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n39.xml
        Test URI:not-wf/P02/ibm02n39.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x86.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n39.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n40xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n40.xml
        Test URI:not-wf/P02/ibm02n40.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x87.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n40.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n41xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n41.xml
        Test URI:not-wf/P02/ibm02n41.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x88.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n41.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n42xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n42.xml
        Test URI:not-wf/P02/ibm02n42.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x89.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n42.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n43xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n43.xml
        Test URI:not-wf/P02/ibm02n43.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x8A.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n43.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n44xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n44.xml
        Test URI:not-wf/P02/ibm02n44.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x8B.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n44.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n45xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n45.xml
        Test URI:not-wf/P02/ibm02n45.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x8C.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n45.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n46xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n46.xml
        Test URI:not-wf/P02/ibm02n46.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x8D.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n46.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n47xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n47.xml
        Test URI:not-wf/P02/ibm02n47.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x8E.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n47.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n48xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n48.xml
        Test URI:not-wf/P02/ibm02n48.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x8F.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n48.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n49xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n49.xml
        Test URI:not-wf/P02/ibm02n49.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x90.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n49.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n50xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n50.xml
        Test URI:not-wf/P02/ibm02n50.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x91.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n50.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n51xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n51.xml
        Test URI:not-wf/P02/ibm02n51.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x92.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n51.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n52xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n52.xml
        Test URI:not-wf/P02/ibm02n52.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x93.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n52.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n53xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n53.xml
        Test URI:not-wf/P02/ibm02n53.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x94.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n53.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n54xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n54.xml
        Test URI:not-wf/P02/ibm02n54.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x95.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n54.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n55xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n55.xml
        Test URI:not-wf/P02/ibm02n55.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x96.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n55.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n56xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n56.xml
        Test URI:not-wf/P02/ibm02n56.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x97.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n56.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n57xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n57.xml
        Test URI:not-wf/P02/ibm02n57.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x98.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n57.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n58xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n58.xml
        Test URI:not-wf/P02/ibm02n58.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x99.
    */

    test_ibm11_notwf(
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n58.xml")
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n59xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n59.xml
        Test URI:not-wf/P02/ibm02n59.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x9A.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n59.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n60xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n60.xml
        Test URI:not-wf/P02/ibm02n60.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x9B.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n60.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n61xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n61.xml
        Test URI:not-wf/P02/ibm02n61.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x9C.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n61.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n62xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n62.xml
        Test URI:not-wf/P02/ibm02n62.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x9D.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n62.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n63xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n63.xml
        Test URI:not-wf/P02/ibm02n63.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control character 0x9E.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n63.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n64xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n64.xml
        Test URI:not-wf/P02/ibm02n64.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control characters present in an external entity.
    */
    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n64.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n65xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n65.xml
        Test URI:not-wf/P02/ibm02n65.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control characters present in an external entity.
    */
    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n65.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n66xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n66.xml
        Test URI:not-wf/P02/ibm02n66.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded control characters present in an external entity.
    */
    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n66.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n67xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n67.xml
        Test URI:not-wf/P02/ibm02n67.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded character 0xD800. (Invalid UTF8 sequence)
    */

    test_ibm11_notwf(
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n67.xml")
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n68xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n68.xml
        Test URI:not-wf/P02/ibm02n68.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded character 0xFFFE.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n68.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n69xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n69.xml
        Test URI:not-wf/P02/ibm02n69.xml
        Spec Sections:2.2,4.1
        Description:This test contains embeded character 0xFFFF.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n69.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n70xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n70.xml
        Test URI:not-wf/P02/ibm02n70.xml
        Spec Sections:2.2,4.1
        Description:This test contains a reference to character 0xFFFE.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n70.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p02ibm02n71xml() {
    /*
        Test ID:ibm-1-1-not-wf-P02-ibm02n71.xml
        Test URI:not-wf/P02/ibm02n71.xml
        Spec Sections:2.2,4.1
        Description:This test contains a reference to character 0xFFFF.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/ibm02n71.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P02/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n01xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n01.xml
        Test URI:not-wf/P04/ibm04n01.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #x300
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n01.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n02xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n02.xml
        Test URI:not-wf/P04/ibm04n02.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x333
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n02.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n03xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n03.xml
        Test URI:not-wf/P04/ibm04n03.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x369
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n03.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n04xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n04.xml
        Test URI:not-wf/P04/ibm04n04.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x37E
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n04.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n05xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n05.xml
        Test URI:not-wf/P04/ibm04n05.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2000
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n05.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n06xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n06.xml
        Test URI:not-wf/P04/ibm04n06.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2001
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n06.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n07xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n07.xml
        Test URI:not-wf/P04/ibm04n07.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2002
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n07.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n08xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n08.xml
        Test URI:not-wf/P04/ibm04n08.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2005
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n08.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n09xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n09.xml
        Test URI:not-wf/P04/ibm04n09.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x200B
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n09.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n10xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n10.xml
        Test URI:not-wf/P04/ibm04n10.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x200E
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n10.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n11xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n11.xml
        Test URI:not-wf/P04/ibm04n11.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x200F
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n11.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n12xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n12.xml
        Test URI:not-wf/P04/ibm04n12.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2069
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n12.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n13xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n13.xml
        Test URI:not-wf/P04/ibm04n13.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2190
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n13.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n14xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n14.xml
        Test URI:not-wf/P04/ibm04n14.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x23FF
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n14.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n15xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n15.xml
        Test URI:not-wf/P04/ibm04n15.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x280F
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n15.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n16xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n16.xml
        Test URI:not-wf/P04/ibm04n16.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2A00
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n16.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n17xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n17.xml
        Test URI:not-wf/P04/ibm04n17.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2EDC
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n17.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n18xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n18.xml
        Test URI:not-wf/P04/ibm04n18.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2B00
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n18.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n19xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n19.xml
        Test URI:not-wf/P04/ibm04n19.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2BFF
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n19.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n20xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n20.xml
        Test URI:not-wf/P04/ibm04n20.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x3000
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n20.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n21xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n21.xml
        Test URI:not-wf/P04/ibm04n21.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xD800
    */

    test_ibm11_notwf(
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n21.xml")
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n22xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n22.xml
        Test URI:not-wf/P04/ibm04n22.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xD801
    */

    test_ibm11_notwf(
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n22.xml")
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n23xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n23.xml
        Test URI:not-wf/P04/ibm04n23.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xDAFF
    */

    test_ibm11_notwf(
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n23.xml")
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n24xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n24.xml
        Test URI:not-wf/P04/ibm04n24.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xDFFF
    */

    test_ibm11_notwf(
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n24.xml")
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n25xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n25.xml
        Test URI:not-wf/P04/ibm04n25.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xEFFF
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n25.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n26xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n26.xml
        Test URI:not-wf/P04/ibm04n26.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xF1FF
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n26.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n27xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n27.xml
        Test URI:not-wf/P04/ibm04n27.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xF8FF
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n27.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04ibm04n28xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04-ibm04n28.xml
        Test URI:not-wf/P04/ibm04n28.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xFFFFF
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/ibm04n28.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an01xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an01.xml
        Test URI:not-wf/P04a/ibm04an01.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #xB8
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an01.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an02xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an02.xml
        Test URI:not-wf/P04a/ibm04an02.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xA1
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an02.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an03xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an03.xml
        Test URI:not-wf/P04a/ibm04an03.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xAF
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an03.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an04xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an04.xml
        Test URI:not-wf/P04a/ibm04an04.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x37E
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an04.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an05xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an05.xml
        Test URI:not-wf/P04a/ibm04an05.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2000
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an05.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an06xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an06.xml
        Test URI:not-wf/P04a/ibm04an06.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2001
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an06.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an07xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an07.xml
        Test URI:not-wf/P04a/ibm04an07.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2002
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an07.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an08xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an08.xml
        Test URI:not-wf/P04a/ibm04an08.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2005
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an08.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an09xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an09.xml
        Test URI:not-wf/P04a/ibm04an09.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x200B
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an09.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an10xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an10.xml
        Test URI:not-wf/P04a/ibm04an10.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x200E
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an10.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an11xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an11.xml
        Test URI:not-wf/P04a/ibm04an11.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2038
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an11.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an12xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an12.xml
        Test URI:not-wf/P04a/ibm04an12.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2041
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an12.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an13xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an13.xml
        Test URI:not-wf/P04a/ibm04an13.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2190
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an13.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an14xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an14.xml
        Test URI:not-wf/P04a/ibm04an14.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x23FF
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an14.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an15xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an15.xml
        Test URI:not-wf/P04a/ibm04an15.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x280F
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an15.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an16xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an16.xml
        Test URI:not-wf/P04a/ibm04an16.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2A00
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an16.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an17xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an17.xml
        Test URI:not-wf/P04a/ibm04an17.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xFDD0
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an17.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an18xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an18.xml
        Test URI:not-wf/P04a/ibm04an18.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xFDEF
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an18.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an19xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an19.xml
        Test URI:not-wf/P04a/ibm04an19.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2FFF
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an19.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an20xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an20.xml
        Test URI:not-wf/P04a/ibm04an20.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x3000
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an20.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an21xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an21.xml
        Test URI:not-wf/P04a/ibm04an21.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xD800
    */

    test_ibm11_notwf(
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an21.xml")
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an22xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an22.xml
        Test URI:not-wf/P04a/ibm04an22.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xD801
    */

    test_ibm11_notwf(
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an22.xml")
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an23xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an23.xml
        Test URI:not-wf/P04a/ibm04an23.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xDAFF
    */

    test_ibm11_notwf(
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an23.xml")
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an24xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an24.xml
        Test URI:not-wf/P04a/ibm04an24.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xDFFF
    */

    test_ibm11_notwf(
        non_utf8_file_reader("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an24.xml")
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an25xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an25.xml
        Test URI:not-wf/P04a/ibm04an25.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xEFFF
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an25.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an26xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an26.xml
        Test URI:not-wf/P04a/ibm04an26.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xF1FF
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an26.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an27xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an27.xml
        Test URI:not-wf/P04a/ibm04an27.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xF8FF
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an27.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p04aibm04an28xml() {
    /*
        Test ID:ibm-1-1-not-wf-P04a-ibm04an28.xml
        Test URI:not-wf/P04a/ibm04an28.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xFFFFF
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/ibm04an28.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P04a/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p05ibm05n01xml() {
    /*
        Test ID:ibm-1-1-not-wf-P05-ibm05n01.xml
        Test URI:not-wf/P05/ibm05n01.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal Name containing #0x0B
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P05/ibm05n01.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P05/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p05ibm05n02xml() {
    /*
        Test ID:ibm-1-1-not-wf-P05-ibm05n02.xml
        Test URI:not-wf/P05/ibm05n02.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal Name containing #0x300
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P05/ibm05n02.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P05/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p05ibm05n03xml() {
    /*
        Test ID:ibm-1-1-not-wf-P05-ibm05n03.xml
        Test URI:not-wf/P05/ibm05n03.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal Name containing #0x36F
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P05/ibm05n03.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P05/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p05ibm05n04xml() {
    /*
        Test ID:ibm-1-1-not-wf-P05-ibm05n04.xml
        Test URI:not-wf/P05/ibm05n04.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal Name containing #0x203F
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P05/ibm05n04.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P05/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p05ibm05n05xml() {
    /*
        Test ID:ibm-1-1-not-wf-P05-ibm05n05.xml
        Test URI:not-wf/P05/ibm05n05.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal Name containing #x2040
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P05/ibm05n05.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P05/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p05ibm05n06xml() {
    /*
        Test ID:ibm-1-1-not-wf-P05-ibm05n06.xml
        Test URI:not-wf/P05/ibm05n06.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal Name containing #0xB7
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P05/ibm05n06.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P05/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n01xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n01.xml
        Test URI:not-wf/P77/ibm77n01.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and that of the external dtd 1.0. The external dtd contains the invalid XML1.1 but valid XML 1.0 character #x7F.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n01.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n02xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n02.xml
        Test URI:not-wf/P77/ibm77n02.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and that of the external dtd 1.0. The external dtd contains a comment with the invalid XML1.1 but valid XML 1.0 character #x80.
    */
    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n02.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n03xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n03.xml
        Test URI:not-wf/P77/ibm77n03.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and that of the external dtd 1.0. The external dtd contains a PI with the invalid XML1.1 but valid XML 1.0 character #x9F.
    */
    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n03.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n04xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n04.xml
        Test URI:not-wf/P77/ibm77n04.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and that of the external entity 1.0. The external entity the contains invalid XML1.1 but valid XML 1.0 character #x89.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n04.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n05xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n05.xml
        Test URI:not-wf/P77/ibm77n05.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and that of the external entity 1.0. The external entity contains the invalid XML1.1 but valid XML 1.0 character #x94.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n05.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n06xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n06.xml
        Test URI:not-wf/P77/ibm77n06.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and that of the external entity 1.0. The external entity contains the invalid XML1.1 but valid XML 1.0 character #x9F.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n06.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n07xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n07.xml
        Test URI:not-wf/P77/ibm77n07.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and the external dtd does not contain a textDecl. The external entity contains the invalid XML1.1 but valid XML 1.0 character #x7F.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n07.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n08xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n08.xml
        Test URI:not-wf/P77/ibm77n08.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and the external dtd does not contain a VersionNum in the textDecl. The external entity contains the invalid XML1.1 but valid XML 1.0 character #x9B.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n08.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n09xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n09.xml
        Test URI:not-wf/P77/ibm77n09.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and the external dtd does not contain a textDecl. The external entity contains the invalid XML1.1 but valid XML 1.0 character #x8D.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n09.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n10xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n10.xml
        Test URI:not-wf/P77/ibm77n10.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and the external dtd does not contain a VersionNum in the textDecl. The external entity contains the invalid XML 1.1 but valid XML 1.0 character #x84.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n10.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n11xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n11.xml
        Test URI:not-wf/P77/ibm77n11.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and the external dtd does not contain a textDecl. The external entity contains the invalid XML 1.1 but valid XML 1.0 character #x88.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n11.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n12xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n12.xml
        Test URI:not-wf/P77/ibm77n12.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the document entity is 1.1 and the external dtd does not contain a textDecl. The external entity contains the invalid XML 1.1 but valid XML 1.0 character #x8E.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n12.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n13xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n13.xml
        Test URI:not-wf/P77/ibm77n13.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the primary document entity is 1.0 and that of the external dtd is 1.0. The external dtd contains an external entity whose VersionNum is 1.1.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n13.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n14xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n14.xml
        Test URI:not-wf/P77/ibm77n14.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the primary document entity is 1.1 and that of the external dtd is 1.0. The external dtd contains an element declaration with an invalid XML 1.1 and 1.0 name.
    */
    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n14.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n15xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n15.xml
        Test URI:not-wf/P77/ibm77n15.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the primary document entity is 1.1 and testDecl of the external dtd is absent. The external dtd contains an external entity whose VersionNum is 1.1 containing a valid XML1.0 but an invalid XML 1.1 character #x7F.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n15.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n16xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n16.xml
        Test URI:not-wf/P77/ibm77n16.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the primary document entity is 1.0 and VersioNum of the external entity is absent. The replacement text of the entity contains an element followed by the valid XML 1.1 of line character NEL #x85 in its empty elem tag.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n16.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n17xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n17.xml
        Test URI:not-wf/P77/ibm77n17.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the primary document entity is absent and that of the external entity is 1.0. The textDecl in the external entity contains an invalid XML1.0 but valid XML 1.1 enf of line character NEL #x85.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n17.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n18xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n18.xml
        Test URI:not-wf/P77/ibm77n18.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the primary document entity is absent and that of the external entity is 1.0. The textDecl in the external entity contains an invalid XML1.0 but valid XML 1.1 of line character Unicode line separator #x2028.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n18.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n19xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n19.xml
        Test URI:not-wf/P77/ibm77n19.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the primary document entity is 1.1 and that of the external dtd is absent. The external dtd contains an external entity whose VersionNum is absent and it contains a valid XML 1.0 but an invalid XML 1.1 character #x94.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n19.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n20xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n20.xml
        Test URI:not-wf/P77/ibm77n20.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the primary document entity is 1.1 and that of the external dtd is 1.1. The external dtd contains an external entity whose VersionNum is absent and it contains a valid XML 1.0 but an invalid XML 1.1 character #x8F.
    */

    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n20.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibm11notwf_p77ibm77n21xml() {
    /*
        Test ID:ibm-1-1-not-wf-P77-ibm77n21.xml
        Test URI:not-wf/P77/ibm77n21.xml
        Spec Sections:4.3.4
        Description:The VersionNum of the primary document entity is 1.1 and the texlDecl of the external dtd is absent. The external dtd contains a reference to an external parameter entity whose VersionNum is absent from the textDecl and it contains an invalid XML 1.1 character #x8F.
    */
    test_ibm11_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/ibm77n21.xml")
            .unwrap()
            .as_str(),
        "tests/conformance/xml/xmlconf/ibm/xml-1.1/not-wf/P77/",
    );
}
