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
fn ibmnotwf_p46ibm46n01xml() {
    /*
        Test ID:ibm-not-wf-P46-ibm46n01.xml
        Test URI:not-wf/P46/ibm46n01.xml
        Spec Sections:3.2
        Description:Tests contentspec with wrong key word. the string "empty" is used as the key word in the contentspec of the second elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P46/ibm46n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p46ibm46n02xml() {
    /*
        Test ID:ibm-not-wf-P46-ibm46n02.xml
        Test URI:not-wf/P46/ibm46n02.xml
        Spec Sections:3.2
        Description:Tests contentspec with wrong key word. the string "Empty" is used as the key word in the contentspec of the second elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P46/ibm46n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p46ibm46n03xml() {
    /*
        Test ID:ibm-not-wf-P46-ibm46n03.xml
        Test URI:not-wf/P46/ibm46n03.xml
        Spec Sections:3.2
        Description:Tests contentspec with wrong key word. the string "Any" is used as the key word in the contentspec of the second elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P46/ibm46n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p46ibm46n04xml() {
    /*
        Test ID:ibm-not-wf-P46-ibm46n04.xml
        Test URI:not-wf/P46/ibm46n04.xml
        Spec Sections:3.2
        Description:Tests contentspec with wrong key word. the string "any" is used as the key word in the contentspec of the second elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P46/ibm46n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p46ibm46n05xml() {
    /*
        Test ID:ibm-not-wf-P46-ibm46n05.xml
        Test URI:not-wf/P46/ibm46n05.xml
        Spec Sections:3.2
        Description:Tests contentspec with a wrong option. The string "#CDATA" is used as the contentspec in the second elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P46/ibm46n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p47ibm47n01xml() {
    /*
        Test ID:ibm-not-wf-P47-ibm47n01.xml
        Test URI:not-wf/P47/ibm47n01.xml
        Spec Sections:3.2.1
        Description:Tests children with a required field missing. The "+" is used as the choice or seq field in the second elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P47/ibm47n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p47ibm47n02xml() {
    /*
        Test ID:ibm-not-wf-P47-ibm47n02.xml
        Test URI:not-wf/P47/ibm47n02.xml
        Spec Sections:3.2.1
        Description:Tests children with a required field missing. The "*" is used as the choice or seq field in the second elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P47/ibm47n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p47ibm47n03xml() {
    /*
        Test ID:ibm-not-wf-P47-ibm47n03.xml
        Test URI:not-wf/P47/ibm47n03.xml
        Spec Sections:3.2.1
        Description:Tests children with a required field missing. The "?" is used as the choice or seq field in the second elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P47/ibm47n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p47ibm47n04xml() {
    /*
        Test ID:ibm-not-wf-P47-ibm47n04.xml
        Test URI:not-wf/P47/ibm47n04.xml
        Spec Sections:3.2.1
        Description:Tests children with wrong field ordering. The "*" occurs before the seq field (a,a) in the second elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P47/ibm47n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p47ibm47n05xml() {
    /*
        Test ID:ibm-not-wf-P47-ibm47n05.xml
        Test URI:not-wf/P47/ibm47n05.xml
        Spec Sections:3.2.1
        Description:Tests children with wrong field ordering. The "+" occurs before the choice field (a|a) in the second elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P47/ibm47n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p47ibm47n06xml() {
    /*
        Test ID:ibm-not-wf-P47-ibm47n06.xml
        Test URI:not-wf/P47/ibm47n06.xml
        Spec Sections:3.2.1
        Description:Tests children with wrong key word. The "^" occurs after the seq field in the second elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P47/ibm47n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p48ibm48n01xml() {
    /*
        Test ID:ibm-not-wf-P48-ibm48n01.xml
        Test URI:not-wf/P48/ibm48n01.xml
        Spec Sections:3.2.1
        Description:Tests cp with a required fields missing. The field Name|choice|seq is missing in the second cp in the choice field in the third elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P48/ibm48n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p48ibm48n02xml() {
    /*
        Test ID:ibm-not-wf-P48-ibm48n02.xml
        Test URI:not-wf/P48/ibm48n02.xml
        Spec Sections:3.2.1
        Description:Tests cp with a required fields missing. The field Name|choice|seq is missing in the cp in the third elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P48/ibm48n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p48ibm48n03xml() {
    /*
        Test ID:ibm-not-wf-P48-ibm48n03.xml
        Test URI:not-wf/P48/ibm48n03.xml
        Spec Sections:3.2.1
        Description:Tests cp with a required fields missing. The field Name|choice|seq is missing in the first cp in the choice field in the third elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P48/ibm48n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p48ibm48n04xml() {
    /*
        Test ID:ibm-not-wf-P48-ibm48n04.xml
        Test URI:not-wf/P48/ibm48n04.xml
        Spec Sections:3.2.1
        Description:Tests cp with wrong field ordering. The "+" occurs before the seq (a,a) in the first cp in the choice field in the third elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P48/ibm48n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p48ibm48n05xml() {
    /*
        Test ID:ibm-not-wf-P48-ibm48n05.xml
        Test URI:not-wf/P48/ibm48n05.xml
        Spec Sections:3.2.1
        Description:Tests cp with wrong field ordering. The "*" occurs before the choice (a|b) in the first cp in the seq field in the third elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P48/ibm48n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p48ibm48n06xml() {
    /*
        Test ID:ibm-not-wf-P48-ibm48n06.xml
        Test URI:not-wf/P48/ibm48n06.xml
        Spec Sections:3.2.1
        Description:Tests cp with wrong field ordering. The "?" occurs before the Name "a" in the second cp in the seq field in the third elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P48/ibm48n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p48ibm48n07xml() {
    /*
        Test ID:ibm-not-wf-P48-ibm48n07.xml
        Test URI:not-wf/P48/ibm48n07.xml
        Spec Sections:3.2.1
        Description:Tests cp with wrong key word. The "^" occurs after the Name "a" in the first cp in the choice field in the third elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P48/ibm48n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p49ibm49n01xml() {
    /*
        Test ID:ibm-not-wf-P49-ibm49n01.xml
        Test URI:not-wf/P49/ibm49n01.xml
        Spec Sections:3.2.1
        Description:Tests choice with a required field missing. The two cps are missing in the choice field in the third elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P49/ibm49n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p49ibm49n02xml() {
    /*
        Test ID:ibm-not-wf-P49-ibm49n02.xml
        Test URI:not-wf/P49/ibm49n02.xml
        Spec Sections:3.2.1
        Description:Tests choice with a required field missing. The third cp is missing in the choice field in the fourth elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P49/ibm49n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p49ibm49n03xml() {
    /*
        Test ID:ibm-not-wf-P49-ibm49n03.xml
        Test URI:not-wf/P49/ibm49n03.xml
        Spec Sections:3.2.1
        Description:Tests choice with a wrong separator. The "!" is used as the separator in the choice field in the fourth elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P49/ibm49n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p49ibm49n04xml() {
    /*
        Test ID:ibm-not-wf-P49-ibm49n04.xml
        Test URI:not-wf/P49/ibm49n04.xml
        Spec Sections:3.2.1
        Description:Tests choice with a required field missing. The separator "|" is missing in the choice field (a b)+ in the fourth elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P49/ibm49n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p49ibm49n05xml() {
    /*
        Test ID:ibm-not-wf-P49-ibm49n05.xml
        Test URI:not-wf/P49/ibm49n05.xml
        Spec Sections:3.2.1
        Description:Tests choice with an extra separator. An extra "|" occurs between a and b in the choice field in the fourth elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P49/ibm49n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p49ibm49n06xml() {
    /*
        Test ID:ibm-not-wf-P49-ibm49n06.xml
        Test URI:not-wf/P49/ibm49n06.xml
        Spec Sections:3.2.1
        Description:Tests choice with a required field missing. The closing bracket ")" is missing in the choice field (a |b * in the fourth elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P49/ibm49n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p50ibm50n01xml() {
    /*
        Test ID:ibm-not-wf-P50-ibm50n01.xml
        Test URI:not-wf/P50/ibm50n01.xml
        Spec Sections:3.2.1
        Description:Tests seq with a required field missing. The two cps are missing in the seq field in the fourth elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P50/ibm50n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p50ibm50n02xml() {
    /*
        Test ID:ibm-not-wf-P50-ibm50n02.xml
        Test URI:not-wf/P50/ibm50n02.xml
        Spec Sections:3.2.1
        Description:Tests seq with a required field missing. The third cp is missing in the seq field in the fourth elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P50/ibm50n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p50ibm50n03xml() {
    /*
        Test ID:ibm-not-wf-P50-ibm50n03.xml
        Test URI:not-wf/P50/ibm50n03.xml
        Spec Sections:3.2.1
        Description:Tests seq with a wrong separator. The "|" is used as the separator between a and b in the seq field in the fourth elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P50/ibm50n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p50ibm50n04xml() {
    /*
        Test ID:ibm-not-wf-P50-ibm50n04.xml
        Test URI:not-wf/P50/ibm50n04.xml
        Spec Sections:3.2.1
        Description:Tests seq with a wrong separator. The "." is used as the separator between a and b in the seq field in the fourth elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P50/ibm50n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p50ibm50n05xml() {
    /*
        Test ID:ibm-not-wf-P50-ibm50n05.xml
        Test URI:not-wf/P50/ibm50n05.xml
        Spec Sections:3.2.1
        Description:Tests seq with an extra separator. An extra "," occurs between (a|b) and a in the seq field in the fourth elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P50/ibm50n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p50ibm50n06xml() {
    /*
        Test ID:ibm-not-wf-P50-ibm50n06.xml
        Test URI:not-wf/P50/ibm50n06.xml
        Spec Sections:3.2.1
        Description:Tests seq with a required field missing. The separator between (a|b) and (b|a) is missing in the seq field in the fourth elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P50/ibm50n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p50ibm50n07xml() {
    /*
        Test ID:ibm-not-wf-P50-ibm50n07.xml
        Test URI:not-wf/P50/ibm50n07.xml
        Spec Sections:3.2.1
        Description:Tests seq with wrong closing bracket. The "]" is used as the closing bracket in the seq field in the fourth elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P50/ibm50n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p51ibm51n01xml() {
    /*
        Test ID:ibm-not-wf-P51-ibm51n01.xml
        Test URI:not-wf/P51/ibm51n01.xml
        Spec Sections:3.2.2
        Description:Tests Mixed with a wrong key word. The string "#pcdata" is used as the key word in the Mixed field in the fourth elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P51/ibm51n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p51ibm51n02xml() {
    /*
        Test ID:ibm-not-wf-P51-ibm51n02.xml
        Test URI:not-wf/P51/ibm51n02.xml
        Spec Sections:3.2.2
        Description:Tests Mixed with wrong field ordering. The field #PCDATA does not occur as the first component in the Mixed field in the fourth elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P51/ibm51n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p51ibm51n03xml() {
    /*
        Test ID:ibm-not-wf-P51-ibm51n03.xml
        Test URI:not-wf/P51/ibm51n03.xml
        Spec Sections:3.2.2
        Description:Tests Mixed with a separator missing. The separator "|" is missing in between #PCDATA and a in the Mixed field in the fourth elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P51/ibm51n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p51ibm51n04xml() {
    /*
        Test ID:ibm-not-wf-P51-ibm51n04.xml
        Test URI:not-wf/P51/ibm51n04.xml
        Spec Sections:3.2.2
        Description:Tests Mixed with a wrong key word. The string "#CDATA" is used as the key word in the Mixed field in the fourth elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P51/ibm51n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p51ibm51n05xml() {
    /*
        Test ID:ibm-not-wf-P51-ibm51n05.xml
        Test URI:not-wf/P51/ibm51n05.xml
        Spec Sections:3.2.2
        Description:Tests Mixed with a required field missing. The "*" is missing after the ")" in the Mixed field in the fourth elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P51/ibm51n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p51ibm51n06xml() {
    /*
        Test ID:ibm-not-wf-P51-ibm51n06.xml
        Test URI:not-wf/P51/ibm51n06.xml
        Spec Sections:3.2.2
        Description:Tests Mixed with wrong closing bracket. The "]" is used as the closing bracket in the Mixed field in the fourth elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P51/ibm51n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p51ibm51n07xml() {
    /*
        Test ID:ibm-not-wf-P51-ibm51n07.xml
        Test URI:not-wf/P51/ibm51n07.xml
        Spec Sections:3.2.2
        Description:Tests Mixed with a required field missing. The closing bracket ")" is missing after (#PCDATA in the Mixed field in the fourth elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P51/ibm51n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p52ibm52n01xml() {
    /*
        Test ID:ibm-not-wf-P52-ibm52n01.xml
        Test URI:not-wf/P52/ibm52n01.xml
        Spec Sections:3.3
        Description:Tests AttlistDecl with a required field missing. The Name is missing in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P52/ibm52n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p52ibm52n02xml() {
    /*
        Test ID:ibm-not-wf-P52-ibm52n02.xml
        Test URI:not-wf/P52/ibm52n02.xml
        Spec Sections:3.3
        Description:Tests AttlistDecl with a required field missing. The white space is missing between the beginning sequence and the name in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P52/ibm52n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p52ibm52n03xml() {
    /*
        Test ID:ibm-not-wf-P52-ibm52n03.xml
        Test URI:not-wf/P52/ibm52n03.xml
        Spec Sections:3.3
        Description:Tests AttlistDecl with wrong field ordering. The Name "a" occurs after the first AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P52/ibm52n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p52ibm52n04xml() {
    /*
        Test ID:ibm-not-wf-P52-ibm52n04.xml
        Test URI:not-wf/P52/ibm52n04.xml
        Spec Sections:3.3
        Description:Tests AttlistDecl with wrong key word. The string "Attlist" is used as the key word in the beginning sequence in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P52/ibm52n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p52ibm52n05xml() {
    /*
        Test ID:ibm-not-wf-P52-ibm52n05.xml
        Test URI:not-wf/P52/ibm52n05.xml
        Spec Sections:3.3
        Description:Tests AttlistDecl with a required field missing. The closing bracket "greater than" is missing in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P52/ibm52n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p52ibm52n06xml() {
    /*
        Test ID:ibm-not-wf-P52-ibm52n06.xml
        Test URI:not-wf/P52/ibm52n06.xml
        Spec Sections:3.3
        Description:Tests AttlistDecl with wrong beginning sequence. The string "(less than)ATTLIST" is used as the beginning sequence in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P52/ibm52n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p53ibm53n01xml() {
    /*
        Test ID:ibm-not-wf-P53-ibm53n01.xml
        Test URI:not-wf/P53/ibm53n01.xml
        Spec Sections:3.3
        Description:Tests AttDef with a required field missing. The DefaultDecl is missing in the AttDef for the name "attr1" in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P53/ibm53n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p53ibm53n02xml() {
    /*
        Test ID:ibm-not-wf-P53-ibm53n02.xml
        Test URI:not-wf/P53/ibm53n02.xml
        Spec Sections:3.3
        Description:Tests AttDef with a required field missing. The white space is missing between (abc|def) and "def" in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P53/ibm53n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p53ibm53n03xml() {
    /*
        Test ID:ibm-not-wf-P53-ibm53n03.xml
        Test URI:not-wf/P53/ibm53n03.xml
        Spec Sections:3.3
        Description:Tests AttDef with a required field missing. The AttType is missing for "attr1" in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P53/ibm53n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p53ibm53n04xml() {
    /*
        Test ID:ibm-not-wf-P53-ibm53n04.xml
        Test URI:not-wf/P53/ibm53n04.xml
        Spec Sections:3.3
        Description:Tests AttDef with a required field missing. The white space is missing between "attr1" and (abc|def) in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P53/ibm53n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p53ibm53n05xml() {
    /*
        Test ID:ibm-not-wf-P53-ibm53n05.xml
        Test URI:not-wf/P53/ibm53n05.xml
        Spec Sections:3.3
        Description:Tests AttDef with a required field missing. The Name is missing in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P53/ibm53n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p53ibm53n06xml() {
    /*
        Test ID:ibm-not-wf-P53-ibm53n06.xml
        Test URI:not-wf/P53/ibm53n06.xml
        Spec Sections:3.3
        Description:Tests AttDef with a required field missing. The white space before the name "attr2" is missing in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P53/ibm53n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p53ibm53n07xml() {
    /*
        Test ID:ibm-not-wf-P53-ibm53n07.xml
        Test URI:not-wf/P53/ibm53n07.xml
        Spec Sections:3.3
        Description:Tests AttDef with wrong field ordering. The Name "attr1" occurs after the AttType in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P53/ibm53n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p53ibm53n08xml() {
    /*
        Test ID:ibm-not-wf-P53-ibm53n08.xml
        Test URI:not-wf/P53/ibm53n08.xml
        Spec Sections:3.3
        Description:Tests AttDef with wrong field ordering. The Name "attr1" occurs after the AttType and "default" occurs before the AttType in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P53/ibm53n08.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p54ibm54n01xml() {
    /*
        Test ID:ibm-not-wf-P54-ibm54n01.xml
        Test URI:not-wf/P54/ibm54n01.xml
        Spec Sections:3.3.1
        Description:Tests AttType with a wrong option. The string "BOGUSATTR" is used as the AttType in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P54/ibm54n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p54ibm54n02xml() {
    /*
        Test ID:ibm-not-wf-P54-ibm54n02.xml
        Test URI:not-wf/P54/ibm54n02.xml
        Spec Sections:3.3.1
        Description:Tests AttType with a wrong option. The string "PCDATA" is used as the AttType in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P54/ibm54n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p55ibm55n01xml() {
    /*
        Test ID:ibm-not-wf-P55-ibm55n01.xml
        Test URI:not-wf/P55/ibm55n01.xml
        Spec Sections:3.3.1
        Description:Tests StringType with a wrong key word. The lower case string "cdata" is used as the StringType in the AttType in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P55/ibm55n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p55ibm55n02xml() {
    /*
        Test ID:ibm-not-wf-P55-ibm55n02.xml
        Test URI:not-wf/P55/ibm55n02.xml
        Spec Sections:3.3.1
        Description:Tests StringType with a wrong key word. The string "#CDATA" is used as the StringType in the AttType in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P55/ibm55n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p55ibm55n03xml() {
    /*
        Test ID:ibm-not-wf-P55-ibm55n03.xml
        Test URI:not-wf/P55/ibm55n03.xml
        Spec Sections:3.3.1
        Description:Tests StringType with a wrong key word. The string "CData" is used as the StringType in the AttType in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P55/ibm55n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p56ibm56n01xml() {
    /*
        Test ID:ibm-not-wf-P56-ibm56n01.xml
        Test URI:not-wf/P56/ibm56n01.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType with wrong key word. The type "id" is used in the TokenizedType in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P56/ibm56n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p56ibm56n02xml() {
    /*
        Test ID:ibm-not-wf-P56-ibm56n02.xml
        Test URI:not-wf/P56/ibm56n02.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType with wrong key word. The type "Idref" is used in the TokenizedType in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P56/ibm56n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p56ibm56n03xml() {
    /*
        Test ID:ibm-not-wf-P56-ibm56n03.xml
        Test URI:not-wf/P56/ibm56n03.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType with wrong key word. The type"Idrefs" is used in the TokenizedType in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P56/ibm56n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p56ibm56n04xml() {
    /*
        Test ID:ibm-not-wf-P56-ibm56n04.xml
        Test URI:not-wf/P56/ibm56n04.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType with wrong key word. The type "EntitY" is used in the TokenizedType in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P56/ibm56n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p56ibm56n05xml() {
    /*
        Test ID:ibm-not-wf-P56-ibm56n05.xml
        Test URI:not-wf/P56/ibm56n05.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType with wrong key word. The type "nmTOKEN" is used in the TokenizedType in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P56/ibm56n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p56ibm56n06xml() {
    /*
        Test ID:ibm-not-wf-P56-ibm56n06.xml
        Test URI:not-wf/P56/ibm56n06.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType with wrong key word. The type "NMtokens" is used in the TokenizedType in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P56/ibm56n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p56ibm56n07xml() {
    /*
        Test ID:ibm-not-wf-P56-ibm56n07.xml
        Test URI:not-wf/P56/ibm56n07.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType with wrong key word. The type "#ID" is used in the TokenizedType in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P56/ibm56n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p57ibm57n01xml() {
    /*
        Test ID:ibm-not-wf-P57-ibm57n01.xml
        Test URI:not-wf/P57/ibm57n01.xml
        Spec Sections:3.3.1
        Description:Tests EnumeratedType with an illegal option. The string "NMTOKEN (a|b)" is used in the EnumeratedType in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P57/ibm57n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p58ibm58n01xml() {
    /*
        Test ID:ibm-not-wf-P58-ibm58n01.xml
        Test URI:not-wf/P58/ibm58n01.xml
        Spec Sections:3.3.1
        Description:Tests NotationType with wrong key word. The lower case "notation" is used as the key word in the NotationType in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P58/ibm58n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p58ibm58n02xml() {
    /*
        Test ID:ibm-not-wf-P58-ibm58n02.xml
        Test URI:not-wf/P58/ibm58n02.xml
        Spec Sections:3.3.1
        Description:Tests NotationType with a required field missing. The beginning bracket "(" is missing in the NotationType in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P58/ibm58n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p58ibm58n03xml() {
    /*
        Test ID:ibm-not-wf-P58-ibm58n03.xml
        Test URI:not-wf/P58/ibm58n03.xml
        Spec Sections:3.3.1
        Description:Tests NotationType with a required field missing. The Name is missing in the "()" in the NotationType in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P58/ibm58n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p58ibm58n04xml() {
    /*
        Test ID:ibm-not-wf-P58-ibm58n04.xml
        Test URI:not-wf/P58/ibm58n04.xml
        Spec Sections:3.3.1
        Description:Tests NotationType with a required field missing. The closing bracket is missing in the NotationType in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P58/ibm58n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p58ibm58n05xml() {
    /*
        Test ID:ibm-not-wf-P58-ibm58n05.xml
        Test URI:not-wf/P58/ibm58n05.xml
        Spec Sections:3.3.1
        Description:Tests NotationType with wrong field ordering. The key word "NOTATION" occurs after "(this)" in the NotationType in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P58/ibm58n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p58ibm58n06xml() {
    /*
        Test ID:ibm-not-wf-P58-ibm58n06.xml
        Test URI:not-wf/P58/ibm58n06.xml
        Spec Sections:3.3.1
        Description:Tests NotationType with wrong separator. The "," is used as a separator between "this" and "that" in the NotationType in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P58/ibm58n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p58ibm58n07xml() {
    /*
        Test ID:ibm-not-wf-P58-ibm58n07.xml
        Test URI:not-wf/P58/ibm58n07.xml
        Spec Sections:3.3.1
        Description:Tests NotationType with a required field missing. The white space is missing between "NOTATION" and "(this)" in the NotationType in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P58/ibm58n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p58ibm58n08xml() {
    /*
        Test ID:ibm-not-wf-P58-ibm58n08.xml
        Test URI:not-wf/P58/ibm58n08.xml
        Spec Sections:3.3.1
        Description:Tests NotationType with extra wrong characters. The double quote character occurs after "(" and before ")" in the NotationType in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P58/ibm58n08.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p59ibm59n01xml() {
    /*
        Test ID:ibm-not-wf-P59-ibm59n01.xml
        Test URI:not-wf/P59/ibm59n01.xml
        Spec Sections:3.3.1
        Description:Tests Enumeration with required fields missing. The Nmtokens and "|"s are missing in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P59/ibm59n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p59ibm59n02xml() {
    /*
        Test ID:ibm-not-wf-P59-ibm59n02.xml
        Test URI:not-wf/P59/ibm59n02.xml
        Spec Sections:3.3.1
        Description:Tests Enumeration with a required field missing. The closing bracket ")" is missing in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P59/ibm59n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p59ibm59n03xml() {
    /*
        Test ID:ibm-not-wf-P59-ibm59n03.xml
        Test URI:not-wf/P59/ibm59n03.xml
        Spec Sections:3.3.1
        Description:Tests Enumeration with wrong separator. The "," is used as the separator in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P59/ibm59n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p59ibm59n04xml() {
    /*
        Test ID:ibm-not-wf-P59-ibm59n04.xml
        Test URI:not-wf/P59/ibm59n04.xml
        Spec Sections:3.3.1
        Description:Tests Enumeration with illegal presence. The double quotes occur around the Enumeration value in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P59/ibm59n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p59ibm59n05xml() {
    /*
        Test ID:ibm-not-wf-P59-ibm59n05.xml
        Test URI:not-wf/P59/ibm59n05.xml
        Spec Sections:3.3.1
        Description:Tests Enumeration with a required field missing. The white space is missing between in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P59/ibm59n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p59ibm59n06xml() {
    /*
        Test ID:ibm-not-wf-P59-ibm59n06.xml
        Test URI:not-wf/P59/ibm59n06.xml
        Spec Sections:3.3.1
        Description:Tests Enumeration with a required field missing. The beginning bracket "(" is missing in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P59/ibm59n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p60ibm60n01xml() {
    /*
        Test ID:ibm-not-wf-P60-ibm60n01.xml
        Test URI:not-wf/P60/ibm60n01.xml
        Spec Sections:3.3.2
        Description:Tests DefaultDecl with wrong key word. The string "#required" is used as the key word in the DefaultDecl in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P60/ibm60n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p60ibm60n02xml() {
    /*
        Test ID:ibm-not-wf-P60-ibm60n02.xml
        Test URI:not-wf/P60/ibm60n02.xml
        Spec Sections:3.3.2
        Description:Tests DefaultDecl with wrong key word. The string "Implied" is used as the key word in the DefaultDecl in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P60/ibm60n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p60ibm60n03xml() {
    /*
        Test ID:ibm-not-wf-P60-ibm60n03.xml
        Test URI:not-wf/P60/ibm60n03.xml
        Spec Sections:3.3.2
        Description:Tests DefaultDecl with wrong key word. The string "!IMPLIED" is used as the key word in the DefaultDecl in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P60/ibm60n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p60ibm60n04xml() {
    /*
        Test ID:ibm-not-wf-P60-ibm60n04.xml
        Test URI:not-wf/P60/ibm60n04.xml
        Spec Sections:3.3.2
        Description:Tests DefaultDecl with a required field missing. There is no attribute value specified after the key word "#FIXED" in the DefaultDecl in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P60/ibm60n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p60ibm60n05xml() {
    /*
        Test ID:ibm-not-wf-P60-ibm60n05.xml
        Test URI:not-wf/P60/ibm60n05.xml
        Spec Sections:3.3.2
        Description:Tests DefaultDecl with a required field missing. The white space is missing between the key word "#FIXED" and the attribute value in the DefaultDecl in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P60/ibm60n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p60ibm60n06xml() {
    /*
        Test ID:ibm-not-wf-P60-ibm60n06.xml
        Test URI:not-wf/P60/ibm60n06.xml
        Spec Sections:3.3.2
        Description:Tests DefaultDecl with wrong field ordering. The key word "#FIXED" occurs after the attribute value "introduction" in the DefaultDecl in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P60/ibm60n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p60ibm60n07xml() {
    /*
        Test ID:ibm-not-wf-P60-ibm60n07.xml
        Test URI:not-wf/P60/ibm60n07.xml
        Spec Sections:3.3.2
        Description:Tests DefaultDecl against WFC of P60. The text replacement of the entity "avalue" contains the "less than" character in the DefaultDecl in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P60/ibm60n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p60ibm60n08xml() {
    /*
        Test ID:ibm-not-wf-P60-ibm60n08.xml
        Test URI:not-wf/P60/ibm60n08.xml
        Spec Sections:3.3.2
        Description:Tests DefaultDecl with more than one key word. The "#REQUIRED" and the "#IMPLIED" are used as the key words in the DefaultDecl in the AttDef in the AttlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P60/ibm60n08.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p61ibm61n01xml() {
    /*
        Test ID:ibm-not-wf-P61-ibm61n01.xml
        Test URI:not-wf/P61/ibm61n01.xml
        Spec Sections:3.4
        Description:Tests conditionalSect with a wrong option. The word "NOTINCLUDE" is used as part of an option which is wrong in the coditionalSect.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P61/ibm61n01.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p62ibm62n01xml() {
    /*
        Test ID:ibm-not-wf-P62-ibm62n01.xml
        Test URI:not-wf/P62/ibm62n01.xml
        Spec Sections:3.4
        Description:Tests includeSect with wrong key word. The string "include" is used as a key word in the beginning sequence in the includeSect in the file ibm62n01.dtd.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P62/ibm62n01.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p62ibm62n02xml() {
    /*
        Test ID:ibm-not-wf-P62-ibm62n02.xml
        Test URI:not-wf/P62/ibm62n02.xml
        Spec Sections:3.4
        Description:Tests includeSect with wrong beginning sequence. An extra "[" occurs in the beginning sequence in the includeSect in the file ibm62n02.dtd.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P62/ibm62n02.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p62ibm62n03xml() {
    /*
        Test ID:ibm-not-wf-P62-ibm62n03.xml
        Test URI:not-wf/P62/ibm62n03.xml
        Spec Sections:3.4
        Description:Tests includeSect with wrong beginning sequence. A wrong character "?" occurs in the beginning sequence in the includeSect in the file ibm62n03.dtd.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P62/ibm62n03.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p62ibm62n04xml() {
    /*
        Test ID:ibm-not-wf-P62-ibm62n04.xml
        Test URI:not-wf/P62/ibm62n04.xml
        Spec Sections:3.4
        Description:Tests includeSect with a required field missing. The key word "INCLUDE" is missing in the includeSect in the file ibm62n04.dtd.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P62/ibm62n04.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p62ibm62n05xml() {
    /*
        Test ID:ibm-not-wf-P62-ibm62n05.xml
        Test URI:not-wf/P62/ibm62n05.xml
        Spec Sections:3.4
        Description:Tests includeSect with a required field missing. The "[" is missing after the key word "INCLUDE" in the includeSect in the file ibm62n05.dtd.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P62/ibm62n05.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p62ibm62n06xml() {
    /*
        Test ID:ibm-not-wf-P62-ibm62n06.xml
        Test URI:not-wf/P62/ibm62n06.xml
        Spec Sections:3.4
        Description:Tests includeSect with wrong field ordering. The two external subset declarations occur before the key word "INCLUDE" in the includeSect in the file ibm62n06.dtd.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P62/ibm62n06.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p62ibm62n07xml() {
    /*
        Test ID:ibm-not-wf-P62-ibm62n07.xml
        Test URI:not-wf/P62/ibm62n07.xml
        Spec Sections:3.4
        Description:Tests includeSect with a required field missing. The closing sequence "]](greater than)" is missing in the includeSect in the file ibm62n07.dtd.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P62/ibm62n07.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p62ibm62n08xml() {
    /*
        Test ID:ibm-not-wf-P62-ibm62n08.xml
        Test URI:not-wf/P62/ibm62n08.xml
        Spec Sections:3.4
        Description:Tests includeSect with a required field missing. One "]" is missing in the closing sequence in the includeSect in the file ibm62n08.dtd.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P62/ibm62n08.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p63ibm63n01xml() {
    /*
        Test ID:ibm-not-wf-P63-ibm63n01.xml
        Test URI:not-wf/P63/ibm63n01.xml
        Spec Sections:3.4
        Description:Tests ignoreSect with wrong key word. The string "ignore" is used as a key word in the beginning sequence in the ignoreSect in the file ibm63n01.dtd.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P63/ibm63n01.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p63ibm63n02xml() {
    /*
        Test ID:ibm-not-wf-P63-ibm63n02.xml
        Test URI:not-wf/P63/ibm63n02.xml
        Spec Sections:3.4
        Description:Tests ignoreSect with wrong beginning sequence. An extra "[" occurs in the beginning sequence in the ignoreSect in the file ibm63n02.dtd.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P63/ibm63n02.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p63ibm63n03xml() {
    /*
        Test ID:ibm-not-wf-P63-ibm63n03.xml
        Test URI:not-wf/P63/ibm63n03.xml
        Spec Sections:3.4
        Description:Tests ignoreSect with wrong beginning sequence. A wrong character "?" occurs in the beginning sequence in the ignoreSect in the file ibm63n03.dtd.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P63/ibm63n03.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p63ibm63n04xml() {
    /*
        Test ID:ibm-not-wf-P63-ibm63n04.xml
        Test URI:not-wf/P63/ibm63n04.xml
        Spec Sections:3.4
        Description:Tests ignoreSect with a required field missing. The key word "IGNORE" is missing in the ignoreSect in the file ibm63n04.dtd.
    */

    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/ibm/".to_string())
        .build();
    let parseresult = xml::parse_with_state(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P63/ibm63n04.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}
