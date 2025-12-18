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
fn ibmnotwf_p20ibm20n01xml() {
    /*
        Test ID:ibm-not-wf-P20-ibm20n01.xml
        Test URI:not-wf/P20/ibm20n01.xml
        Spec Sections:2.7
        Description:Tests CDATA with an illegal sequence. The CDATA contains the sequence close-bracket close-bracket greater-than.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P20/ibm20n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p21ibm21n01xml() {
    /*
        Test ID:ibm-not-wf-P21-ibm21n01.xml
        Test URI:not-wf/P21/ibm21n01.xml
        Spec Sections:2.7
        Description:Tests CDEnd. One "]" is missing in the CDEnd.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P21/ibm21n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p21ibm21n02xml() {
    /*
        Test ID:ibm-not-wf-P21-ibm21n02.xml
        Test URI:not-wf/P21/ibm21n02.xml
        Spec Sections:2.7
        Description:Tests CDEnd. An extra "]" is placed in the CDEnd.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P21/ibm21n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p21ibm21n03xml() {
    /*
        Test ID:ibm-not-wf-P21-ibm21n03.xml
        Test URI:not-wf/P21/ibm21n03.xml
        Spec Sections:2.7
        Description:Tests CDEnd. A wrong character ")" is placed in the CDEnd.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P21/ibm21n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p22ibm22n01xml() {
    /*
        Test ID:ibm-not-wf-P22-ibm22n01.xml
        Test URI:not-wf/P22/ibm22n01.xml
        Spec Sections:2.8
        Description:Tests prolog with wrong field ordering. The XMLDecl occurs after the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P22/ibm22n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p22ibm22n02xml() {
    /*
        Test ID:ibm-not-wf-P22-ibm22n02.xml
        Test URI:not-wf/P22/ibm22n02.xml
        Spec Sections:2.8
        Description:Tests prolog with wrong field ordering. The Misc (comment) occurs before the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P22/ibm22n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p22ibm22n03xml() {
    /*
        Test ID:ibm-not-wf-P22-ibm22n03.xml
        Test URI:not-wf/P22/ibm22n03.xml
        Spec Sections:2.8
        Description:Tests prolog with wrong field ordering. The XMLDecl occurs after the DTD and a comment. The other comment occurs before the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P22/ibm22n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p23ibm23n01xml() {
    /*
        Test ID:ibm-not-wf-P23-ibm23n01.xml
        Test URI:not-wf/P23/ibm23n01.xml
        Spec Sections:2.8
        Description:Tests XMLDecl with a required field missing. The Versioninfo is missing in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P23/ibm23n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p23ibm23n02xml() {
    /*
        Test ID:ibm-not-wf-P23-ibm23n02.xml
        Test URI:not-wf/P23/ibm23n02.xml
        Spec Sections:2.8
        Description:Tests XMLDecl with wrong field ordering. The VersionInfo occurs after the EncodingDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P23/ibm23n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p23ibm23n03xml() {
    /*
        Test ID:ibm-not-wf-P23-ibm23n03.xml
        Test URI:not-wf/P23/ibm23n03.xml
        Spec Sections:2.8
        Description:Tests XMLDecl with wrong field ordering. The VersionInfo occurs after the SDDecl and the SDDecl occurs after the VersionInfo.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P23/ibm23n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p23ibm23n04xml() {
    /*
        Test ID:ibm-not-wf-P23-ibm23n04.xml
        Test URI:not-wf/P23/ibm23n04.xml
        Spec Sections:2.8
        Description:Tests XMLDecl with wrong key word. An upper case string "XML" is used as the key word in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P23/ibm23n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p23ibm23n05xml() {
    /*
        Test ID:ibm-not-wf-P23-ibm23n05.xml
        Test URI:not-wf/P23/ibm23n05.xml
        Spec Sections:2.8
        Description:Tests XMLDecl with a wrong closing sequence ">".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P23/ibm23n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p23ibm23n06xml() {
    /*
        Test ID:ibm-not-wf-P23-ibm23n06.xml
        Test URI:not-wf/P23/ibm23n06.xml
        Spec Sections:2.8
        Description:Tests XMLDecl with a wrong opening sequence "(less than)!".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P23/ibm23n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p24ibm24n01xml() {
    /*
        Test ID:ibm-not-wf-P24-ibm24n01.xml
        Test URI:not-wf/P24/ibm24n01.xml
        Spec Sections:2.8
        Description:Tests VersionInfo with a required field missing. The VersionNum is missing in the VersionInfo in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P24/ibm24n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p24ibm24n02xml() {
    /*
        Test ID:ibm-not-wf-P24-ibm24n02.xml
        Test URI:not-wf/P24/ibm24n02.xml
        Spec Sections:2.8
        Description:Tests VersionInfo with a required field missing. The white space is missing between the key word "xml" and the VersionInfo in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P24/ibm24n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p24ibm24n03xml() {
    /*
        Test ID:ibm-not-wf-P24-ibm24n03.xml
        Test URI:not-wf/P24/ibm24n03.xml
        Spec Sections:2.8
        Description:Tests VersionInfo with a required field missing. The "=" (equal sign) is missing between the key word "version" and the VersionNum.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P24/ibm24n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p24ibm24n04xml() {
    /*
        Test ID:ibm-not-wf-P24-ibm24n04.xml
        Test URI:not-wf/P24/ibm24n04.xml
        Spec Sections:2.8
        Description:Tests VersionInfo with wrong field ordering. The VersionNum occurs before "=" and "version".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P24/ibm24n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p24ibm24n05xml() {
    /*
        Test ID:ibm-not-wf-P24-ibm24n05.xml
        Test URI:not-wf/P24/ibm24n05.xml
        Spec Sections:2.8
        Description:Tests VersionInfo with wrong field ordering. The "=" occurs after "version" and the VersionNum.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P24/ibm24n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p24ibm24n06xml() {
    /*
        Test ID:ibm-not-wf-P24-ibm24n06.xml
        Test URI:not-wf/P24/ibm24n06.xml
        Spec Sections:2.8
        Description:Tests VersionInfo with the wrong key word "Version".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P24/ibm24n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p24ibm24n07xml() {
    /*
        Test ID:ibm-not-wf-P24-ibm24n07.xml
        Test URI:not-wf/P24/ibm24n07.xml
        Spec Sections:2.8
        Description:Tests VersionInfo with the wrong key word "versioN".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P24/ibm24n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p24ibm24n08xml() {
    /*
        Test ID:ibm-not-wf-P24-ibm24n08.xml
        Test URI:not-wf/P24/ibm24n08.xml
        Spec Sections:2.8
        Description:Tests VersionInfo with mismatched quotes around the VersionNum. version = '1.0" is used as the VersionInfo.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P24/ibm24n08.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p24ibm24n09xml() {
    /*
        Test ID:ibm-not-wf-P24-ibm24n09.xml
        Test URI:not-wf/P24/ibm24n09.xml
        Spec Sections:2.8
        Description:Tests VersionInfo with mismatched quotes around the VersionNum. The closing bracket for the VersionNum is missing.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P24/ibm24n09.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p25ibm25n01xml() {
    /*
        Test ID:ibm-not-wf-P25-ibm25n01.xml
        Test URI:not-wf/P25/ibm25n01.xml
        Spec Sections:2.8
        Description:Tests eq with a wrong key word "==".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P25/ibm25n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p25ibm25n02xml() {
    /*
        Test ID:ibm-not-wf-P25-ibm25n02.xml
        Test URI:not-wf/P25/ibm25n02.xml
        Spec Sections:2.8
        Description:Tests eq with a wrong key word "eq".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P25/ibm25n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p26ibm26n01xml() {
    /*
        Test ID:ibm-not-wf-P26-ibm26n01.xml
        Test URI:not-wf/P26/ibm26n01.xml
        Spec Sections:2.8
        Description:Tests VersionNum with an illegal character "#".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P26/ibm26n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p27ibm27n01xml() {
    /*
        Test ID:ibm-not-wf-P27-ibm27n01.xml
        Test URI:not-wf/P27/ibm27n01.xml
        Spec Sections:2.8
        Description:Tests type of Misc. An element declaration is used as a type of Misc After the element "animal".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P27/ibm27n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p28ibm28n01xml() {
    /*
        Test ID:ibm-not-wf-P28-ibm28n01.xml
        Test URI:not-wf/P28/ibm28n01.xml
        Spec Sections:2.8
        Description:Tests doctypedecl with a required field missing. The Name "animal" is missing in the doctypedecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P28/ibm28n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p28ibm28n02xml() {
    /*
        Test ID:ibm-not-wf-P28-ibm28n02.xml
        Test URI:not-wf/P28/ibm28n02.xml
        Spec Sections:2.8
        Description:Tests doctypedecl with wrong field ordering. The Name "animal" occurs after the markup declarations inside the "[]".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P28/ibm28n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p28ibm28n03xml() {
    /*
        Test ID:ibm-not-wf-P28-ibm28n03.xml
        Test URI:not-wf/P28/ibm28n03.xml
        Spec Sections:2.8
        Description:Tests doctypedecl with wrong field ordering. The Name "animal" occurs after the markup declarations inside the "[]".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P28/ibm28n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p28ibm28n04xml() {
    /*
        Test ID:ibm-not-wf-P28-ibm28n04.xml
        Test URI:not-wf/P28/ibm28n04.xml
        Spec Sections:2.8
        Description:Tests doctypedecl with general entity reference.The "(ampersand)generalE" occurs in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P28/ibm28n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p28ibm28n05xml() {
    /*
        Test ID:ibm-not-wf-P28-ibm28n05.xml
        Test URI:not-wf/P28/ibm28n05.xml
        Spec Sections:2.8
        Description:Tests doctypedecl with wrong key word. A wrong key word "DOCtYPE" occurs on line 2.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P28/ibm28n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p28ibm28n06xml() {
    /*
        Test ID:ibm-not-wf-P28-ibm28n06.xml
        Test URI:not-wf/P28/ibm28n06.xml
        Spec Sections:2.8
        Description:Tests doctypedecl with mismatched brackets. The closing bracket "]" of the DTD is missing.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P28/ibm28n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p28ibm28n07xml() {
    /*
        Test ID:ibm-not-wf-P28-ibm28n07.xml
        Test URI:not-wf/P28/ibm28n07.xml
        Spec Sections:2.8
        Description:Tests doctypedecl with wrong bracket. The opening bracket "{" occurs in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P28/ibm28n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p28ibm28n08xml() {
    /*
        Test ID:ibm-not-wf-P28-ibm28n08.xml
        Test URI:not-wf/P28/ibm28n08.xml
        Spec Sections:2.8
        Description:Tests doctypedecl with wrong opening sequence. The opening sequence "(less than)?DOCTYPE" occurs in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P28/ibm28n08.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p28aibm28an01xml() {
    /*
        Test ID:ibm-not-wf-p28a-ibm28an01.xml
        Test URI:not-wf/p28a/ibm28an01.xml
        Spec Sections:2.8
        Description:This test violates WFC:PE Between Declarations in Production 28a. The last character of a markup declaration is not contained in the same parameter-entity text replacement.
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
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/p28a/ibm28an01.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p29ibm29n01xml() {
    /*
        Test ID:ibm-not-wf-P29-ibm29n01.xml
        Test URI:not-wf/P29/ibm29n01.xml
        Spec Sections:2.8
        Description:Tests markupdecl with an illegal markup declaration. A XMLDecl occurs inside the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P29/ibm29n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p29ibm29n02xml() {
    /*
        Test ID:ibm-not-wf-P29-ibm29n02.xml
        Test URI:not-wf/P29/ibm29n02.xml
        Spec Sections:2.8
        Description:Tests WFC "PEs in Internal Subset". A PE reference occurs inside an elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P29/ibm29n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p29ibm29n03xml() {
    /*
        Test ID:ibm-not-wf-P29-ibm29n03.xml
        Test URI:not-wf/P29/ibm29n03.xml
        Spec Sections:2.8
        Description:Tests WFC "PEs in Internal Subset". A PE reference occurs inside an ATTlistDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P29/ibm29n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p29ibm29n04xml() {
    /*
        Test ID:ibm-not-wf-P29-ibm29n04.xml
        Test URI:not-wf/P29/ibm29n04.xml
        Spec Sections:2.8
        Description:Tests WFC "PEs in Internal Subset". A PE reference occurs inside an EntityDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P29/ibm29n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p29ibm29n05xml() {
    /*
        Test ID:ibm-not-wf-P29-ibm29n05.xml
        Test URI:not-wf/P29/ibm29n05.xml
        Spec Sections:2.8
        Description:Tests WFC "PEs in Internal Subset". A PE reference occurs inside a PI in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P29/ibm29n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p29ibm29n06xml() {
    /*
        Test ID:ibm-not-wf-P29-ibm29n06.xml
        Test URI:not-wf/P29/ibm29n06.xml
        Spec Sections:2.8
        Description:Tests WFC "PEs in Internal Subset". A PE reference occurs inside a comment in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P29/ibm29n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p29ibm29n07xml() {
    /*
        Test ID:ibm-not-wf-P29-ibm29n07.xml
        Test URI:not-wf/P29/ibm29n07.xml
        Spec Sections:2.8
        Description:Tests WFC "PEs in Internal Subset". A PE reference occurs inside a NotationDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P29/ibm29n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p30ibm30n01xml() {
    /*
        Test ID:ibm-not-wf-P30-ibm30n01.xml
        Test URI:not-wf/P30/ibm30n01.xml
        Spec Sections:2.8
        Description:Tests extSubset with wrong field ordering. In the file "ibm30n01.dtd", the TextDecl occurs after the extSubsetDecl (the element declaration).
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
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P30/ibm30n01.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p31ibm31n01xml() {
    /*
        Test ID:ibm-not-wf-P31-ibm31n01.xml
        Test URI:not-wf/P31/ibm31n01.xml
        Spec Sections:2.8
        Description:Tests extSubsetDecl with an illegal field. A general entity reference occurs in file "ibm31n01.dtd".
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
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P31/ibm31n01.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p32ibm32n01xml() {
    /*
        Test ID:ibm-not-wf-P32-ibm32n01.xml
        Test URI:not-wf/P32/ibm32n01.xml
        Spec Sections:2.9
        Description:Tests SDDecl with a required field missing. The leading white space is missing with the SDDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P32/ibm32n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p32ibm32n02xml() {
    /*
        Test ID:ibm-not-wf-P32-ibm32n02.xml
        Test URI:not-wf/P32/ibm32n02.xml
        Spec Sections:2.9
        Description:Tests SDDecl with a required field missing. The "=" sign is missing in the SDDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P32/ibm32n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p32ibm32n03xml() {
    /*
        Test ID:ibm-not-wf-P32-ibm32n03.xml
        Test URI:not-wf/P32/ibm32n03.xml
        Spec Sections:2.9
        Description:Tests SDDecl with wrong key word. The word "Standalone" occurs in the SDDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P32/ibm32n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p32ibm32n04xml() {
    /*
        Test ID:ibm-not-wf-P32-ibm32n04.xml
        Test URI:not-wf/P32/ibm32n04.xml
        Spec Sections:2.9
        Description:Tests SDDecl with wrong key word. The word "Yes" occurs in the SDDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P32/ibm32n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p32ibm32n05xml() {
    /*
        Test ID:ibm-not-wf-P32-ibm32n05.xml
        Test URI:not-wf/P32/ibm32n05.xml
        Spec Sections:2.9
        Description:Tests SDDecl with wrong key word. The word "YES" occurs in the SDDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P32/ibm32n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p32ibm32n06xml() {
    /*
        Test ID:ibm-not-wf-P32-ibm32n06.xml
        Test URI:not-wf/P32/ibm32n06.xml
        Spec Sections:2.9
        Description:Tests SDDecl with wrong key word. The word "No" occurs in the SDDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P32/ibm32n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p32ibm32n07xml() {
    /*
        Test ID:ibm-not-wf-P32-ibm32n07.xml
        Test URI:not-wf/P32/ibm32n07.xml
        Spec Sections:2.9
        Description:Tests SDDecl with wrong key word. The word "NO" occurs in the SDDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P32/ibm32n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p32ibm32n08xml() {
    /*
        Test ID:ibm-not-wf-P32-ibm32n08.xml
        Test URI:not-wf/P32/ibm32n08.xml
        Spec Sections:2.9
        Description:Tests SDDecl with wrong field ordering. The "=" sign occurs after the key word "yes" in the SDDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P32/ibm32n08.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p32ibm32n09xml() {
    /*
        Test ID:ibm-not-wf-P32-ibm32n09.xml
        Test URI:not-wf/P32/ibm32n09.xml
        Spec Sections:2.9
        Description:This is test violates WFC: Entity Declared in P68. The standalone document declaration has the value yes, BUT there is an external markup declaration of an entity (other than amp, lt, gt, apos, quot), and references to this entity appear in the document.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P32/ibm32n09.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p39ibm39n01xml() {
    /*
        Test ID:ibm-not-wf-P39-ibm39n01.xml
        Test URI:not-wf/P39/ibm39n01.xml
        Spec Sections:3
        Description:Tests element with a required field missing. The ETag is missing for the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P39/ibm39n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p39ibm39n02xml() {
    /*
        Test ID:ibm-not-wf-P39-ibm39n02.xml
        Test URI:not-wf/P39/ibm39n02.xml
        Spec Sections:3
        Description:Tests element with a required field missing. The STag is missing for the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P39/ibm39n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p39ibm39n03xml() {
    /*
        Test ID:ibm-not-wf-P39-ibm39n03.xml
        Test URI:not-wf/P39/ibm39n03.xml
        Spec Sections:3
        Description:Tests element with required fields missing. Both the content and the ETag are missing in the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P39/ibm39n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p39ibm39n04xml() {
    /*
        Test ID:ibm-not-wf-P39-ibm39n04.xml
        Test URI:not-wf/P39/ibm39n04.xml
        Spec Sections:3
        Description:Tests element with required fields missing. Both the content and the STag are missing in the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P39/ibm39n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p39ibm39n05xml() {
    /*
        Test ID:ibm-not-wf-P39-ibm39n05.xml
        Test URI:not-wf/P39/ibm39n05.xml
        Spec Sections:3
        Description:Tests element with wrong field ordering. The STag and the ETag are swapped in the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P39/ibm39n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p39ibm39n06xml() {
    /*
        Test ID:ibm-not-wf-P39-ibm39n06.xml
        Test URI:not-wf/P39/ibm39n06.xml
        Spec Sections:3
        Description:Tests element with wrong field ordering. The content occurs after the ETag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P39/ibm39n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p40ibm40n01xml() {
    /*
        Test ID:ibm-not-wf-P40-ibm40n01.xml
        Test URI:not-wf/P40/ibm40n01.xml
        Spec Sections:3.1
        Description:Tests STag with a required field missing. The Name "root" is in the STag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P40/ibm40n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p40ibm40n02xml() {
    /*
        Test ID:ibm-not-wf-P40-ibm40n02.xml
        Test URI:not-wf/P40/ibm40n02.xml
        Spec Sections:3.1
        Description:Tests STag with a required field missing. The white space between the Name "root" and the attribute "attr1" is missing in the STag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P40/ibm40n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p40ibm40n03xml() {
    /*
        Test ID:ibm-not-wf-P40-ibm40n03.xml
        Test URI:not-wf/P40/ibm40n03.xml
        Spec Sections:3.1
        Description:Tests STag with wrong field ordering. The Name "root" occurs after the attribute "attr1" in the STag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P40/ibm40n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p40ibm40n04xml() {
    /*
        Test ID:ibm-not-wf-P40-ibm40n04.xml
        Test URI:not-wf/P40/ibm40n04.xml
        Spec Sections:3.1
        Description:Tests STag with a wrong opening sequence. The string "(less than)!" is used as the opening sequence for the STag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P40/ibm40n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p40ibm40n05xml() {
    /*
        Test ID:ibm-not-wf-P40-ibm40n05.xml
        Test URI:not-wf/P40/ibm40n05.xml
        Spec Sections:3.1
        Description:Tests STag with duplicate attribute names. The attribute name "attr1" occurs twice in the STag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P40/ibm40n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p41ibm41n01xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n01.xml
        Test URI:not-wf/P41/ibm41n01.xml
        Spec Sections:3.1
        Description:Tests Attribute with a required field missing. The attribute name is missing in the Attribute in the STag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p41ibm41n02xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n02.xml
        Test URI:not-wf/P41/ibm41n02.xml
        Spec Sections:3.1
        Description:Tests Attribute with a required field missing. The "=" is missing between the attribute name and the attribute value in the Attribute in the STag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p41ibm41n03xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n03.xml
        Test URI:not-wf/P41/ibm41n03.xml
        Spec Sections:3.1
        Description:Tests Attribute with a required field missing. The AttValue is missing in the Attribute in the STag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p41ibm41n04xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n04.xml
        Test URI:not-wf/P41/ibm41n04.xml
        Spec Sections:3.1
        Description:Tests Attribute with a required field missing. The Name and the "=" are missing in the Attribute in the STag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p41ibm41n05xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n05.xml
        Test URI:not-wf/P41/ibm41n05.xml
        Spec Sections:3.1
        Description:Tests Attribute with a required field missing. The "=" and the AttValue are missing in the Attribute in the STag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p41ibm41n06xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n06.xml
        Test URI:not-wf/P41/ibm41n06.xml
        Spec Sections:3.1
        Description:Tests Attribute with a required field missing. The Name and the AttValue are missing in the Attribute in the STag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p41ibm41n07xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n07.xml
        Test URI:not-wf/P41/ibm41n07.xml
        Spec Sections:3.1
        Description:Tests Attribute with wrong field ordering. The "=" occurs after the Name and the AttValue in the Attribute in the STag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p41ibm41n08xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n08.xml
        Test URI:not-wf/P41/ibm41n08.xml
        Spec Sections:3.1
        Description:Tests Attribute with wrong field ordering. The Name and the AttValue are swapped in the Attribute in the STag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n08.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p41ibm41n09xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n09.xml
        Test URI:not-wf/P41/ibm41n09.xml
        Spec Sections:3.1
        Description:Tests Attribute with wrong field ordering. The "=" occurs before the Name and the AttValue in the Attribute in the STag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n09.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p41ibm41n10xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n10.xml
        Test URI:not-wf/P41/ibm41n10.xml
        Spec Sections:3.1
        Description:Tests Attribute against WFC "no external entity references". A direct reference to the external entity "aExternal" is contained in the value of the attribute "attr1".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n10.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p41ibm41n11xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n11.xml
        Test URI:not-wf/P41/ibm41n11.xml
        Spec Sections:3.1
        Description:Tests Attribute against WFC "no external entity references". A indirect reference to the external entity "aExternal" is contained in the value of the attribute "attr1".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n11.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p41ibm41n12xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n12.xml
        Test URI:not-wf/P41/ibm41n12.xml
        Spec Sections:3.1
        Description:Tests Attribute against WFC "no external entity references". A direct reference to the external unparsed entity "aImage" is contained in the value of the attribute "attr1".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n12.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p41ibm41n13xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n13.xml
        Test URI:not-wf/P41/ibm41n13.xml
        Spec Sections:3.1
        Description:Tests Attribute against WFC "No (less than) character in Attribute Values". The character "less than" is contained in the value of the attribute "attr1".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n13.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p41ibm41n14xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n14.xml
        Test URI:not-wf/P41/ibm41n14.xml
        Spec Sections:3.1
        Description:Tests Attribute against WFC "No (less than) in Attribute Values". The character "less than" is contained in the value of the attribute "attr1" through indirect internal entity reference.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n14.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p42ibm42n01xml() {
    /*
        Test ID:ibm-not-wf-P42-ibm42n01.xml
        Test URI:not-wf/P42/ibm42n01.xml
        Spec Sections:3.1
        Description:Tests ETag with a required field missing. The Name is missing in the ETag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P42/ibm42n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p42ibm42n02xml() {
    /*
        Test ID:ibm-not-wf-P42-ibm42n02.xml
        Test URI:not-wf/P42/ibm42n02.xml
        Spec Sections:3.1
        Description:Tests ETag with a wrong beginning sequence. The string "(less than)\" is used as a beginning sequence of the ETag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P42/ibm42n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p42ibm42n03xml() {
    /*
        Test ID:ibm-not-wf-P42-ibm42n03.xml
        Test URI:not-wf/P42/ibm42n03.xml
        Spec Sections:3.1
        Description:Tests ETag with a wrong beginning sequence. The string "less than" is used as a beginning sequence of the ETag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P42/ibm42n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p42ibm42n04xml() {
    /*
        Test ID:ibm-not-wf-P42-ibm42n04.xml
        Test URI:not-wf/P42/ibm42n04.xml
        Spec Sections:3.1
        Description:Tests ETag with a wrong structure. An white space occurs between The beginning sequence and the Name of the ETag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P42/ibm42n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p42ibm42n05xml() {
    /*
        Test ID:ibm-not-wf-P42-ibm42n05.xml
        Test URI:not-wf/P42/ibm42n05.xml
        Spec Sections:3.1
        Description:Tests ETag with a wrong structure. The ETag of the element "root" contains an Attribute (attr1="any").
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P42/ibm42n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p43ibm43n01xml() {
    /*
        Test ID:ibm-not-wf-P43-ibm43n01.xml
        Test URI:not-wf/P43/ibm43n01.xml
        Spec Sections:3.1
        Description:Tests element content with a wrong option. A NotationDecl is used as the content of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P43/ibm43n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p43ibm43n02xml() {
    /*
        Test ID:ibm-not-wf-P43-ibm43n02.xml
        Test URI:not-wf/P43/ibm43n02.xml
        Spec Sections:3.1
        Description:Tests element content with a wrong option. An elementdecl is used as the content of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P43/ibm43n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p43ibm43n04xml() {
    /*
        Test ID:ibm-not-wf-P43-ibm43n04.xml
        Test URI:not-wf/P43/ibm43n04.xml
        Spec Sections:3.1
        Description:Tests element content with a wrong option. An entitydecl is used as the content of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P43/ibm43n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p43ibm43n05xml() {
    /*
        Test ID:ibm-not-wf-P43-ibm43n05.xml
        Test URI:not-wf/P43/ibm43n05.xml
        Spec Sections:3.1
        Description:Tests element content with a wrong option. An AttlistDecl is used as the content of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P43/ibm43n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p44ibm44n01xml() {
    /*
        Test ID:ibm-not-wf-P44-ibm44n01.xml
        Test URI:not-wf/P44/ibm44n01.xml
        Spec Sections:3.1
        Description:Tests EmptyElemTag with a required field missing. The Name "root" is missing in the EmptyElemTag.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P44/ibm44n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p44ibm44n02xml() {
    /*
        Test ID:ibm-not-wf-P44-ibm44n02.xml
        Test URI:not-wf/P44/ibm44n02.xml
        Spec Sections:3.1
        Description:Tests EmptyElemTag with wrong field ordering. The Attribute (attri1 = "any") occurs before the name of the element "root" in the EmptyElemTag.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P44/ibm44n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p44ibm44n03xml() {
    /*
        Test ID:ibm-not-wf-P44-ibm44n03.xml
        Test URI:not-wf/P44/ibm44n03.xml
        Spec Sections:3.1
        Description:Tests EmptyElemTag with wrong closing sequence. The string "\>" is used as the closing sequence in the EmptyElemtag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P44/ibm44n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p44ibm44n04xml() {
    /*
        Test ID:ibm-not-wf-P44-ibm44n04.xml
        Test URI:not-wf/P44/ibm44n04.xml
        Spec Sections:3.1
        Description:Tests EmptyElemTag which against the WFC "Unique Att Spec". The attribute name "attr1" occurs twice in the EmptyElemTag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P44/ibm44n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p45ibm45n01xml() {
    /*
        Test ID:ibm-not-wf-P45-ibm45n01.xml
        Test URI:not-wf/P45/ibm45n01.xml
        Spec Sections:3.2
        Description:Tests elementdecl with a required field missing. The Name is missing in the second elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P45/ibm45n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p45ibm45n02xml() {
    /*
        Test ID:ibm-not-wf-P45-ibm45n02.xml
        Test URI:not-wf/P45/ibm45n02.xml
        Spec Sections:3.2
        Description:Tests elementdecl with a required field missing. The white space is missing between "aEle" and "(#PCDATA)" in the second elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P45/ibm45n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p45ibm45n03xml() {
    /*
        Test ID:ibm-not-wf-P45-ibm45n03.xml
        Test URI:not-wf/P45/ibm45n03.xml
        Spec Sections:3.2
        Description:Tests elementdecl with a required field missing. The contentspec is missing in the second elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P45/ibm45n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p45ibm45n04xml() {
    /*
        Test ID:ibm-not-wf-P45-ibm45n04.xml
        Test URI:not-wf/P45/ibm45n04.xml
        Spec Sections:3.2
        Description:Tests elementdecl with a required field missing. The contentspec and the white space is missing in the second elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P45/ibm45n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p45ibm45n05xml() {
    /*
        Test ID:ibm-not-wf-P45-ibm45n05.xml
        Test URI:not-wf/P45/ibm45n05.xml
        Spec Sections:3.2
        Description:Tests elementdecl with a required field missing. The Name, the white space, and the contentspec are missing in the second elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P45/ibm45n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p45ibm45n06xml() {
    /*
        Test ID:ibm-not-wf-P45-ibm45n06.xml
        Test URI:not-wf/P45/ibm45n06.xml
        Spec Sections:3.2
        Description:Tests elementdecl with wrong field ordering. The Name occurs after the contentspec in the second elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P45/ibm45n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p45ibm45n07xml() {
    /*
        Test ID:ibm-not-wf-P45-ibm45n07.xml
        Test URI:not-wf/P45/ibm45n07.xml
        Spec Sections:3.2
        Description:Tests elementdecl with wrong beginning sequence. The string "(less than)ELEMENT" is used as the beginning sequence in the second elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P45/ibm45n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p45ibm45n08xml() {
    /*
        Test ID:ibm-not-wf-P45-ibm45n08.xml
        Test URI:not-wf/P45/ibm45n08.xml
        Spec Sections:3.2
        Description:Tests elementdecl with wrong key word. The string "Element" is used as the key word in the second elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P45/ibm45n08.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p45ibm45n09xml() {
    /*
        Test ID:ibm-not-wf-P45-ibm45n09.xml
        Test URI:not-wf/P45/ibm45n09.xml
        Spec Sections:3.2
        Description:Tests elementdecl with wrong key word. The string "element" is used as the key word in the second elementdecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P45/ibm45n09.xml")
            .unwrap()
            .as_str(),
    );
}
