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
fn ibmnotwf_p63ibm63n05xml() {
    /*
        Test ID:ibm-not-wf-P63-ibm63n05.xml
        Test URI:not-wf/P63/ibm63n05.xml
        Spec Sections:3.4
        Description:Tests ignoreSect with a required field missing. The "[" is missing after the key word "IGNORE" in the ignoreSect in the file ibm63n05.dtd.
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
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P63/ibm63n05.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p63ibm63n06xml() {
    /*
        Test ID:ibm-not-wf-P63-ibm63n06.xml
        Test URI:not-wf/P63/ibm63n06.xml
        Spec Sections:3.4
        Description:Tests includeSect with wrong field ordering. The two external subset declarations occur before the key word "IGNORE" in the ignoreSect in the file ibm63n06.dtd.
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
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P63/ibm63n06.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p63ibm63n07xml() {
    /*
        Test ID:ibm-not-wf-P63-ibm63n07.xml
        Test URI:not-wf/P63/ibm63n07.xml
        Spec Sections:3.4
        Description:Tests ignoreSect with a required field missing. The closing sequence "]](greater than)" is missing in the ignoreSect in the file ibm63n07.dtd.
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
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P63/ibm63n07.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p64ibm64n01xml() {
    /*
        Test ID:ibm-not-wf-P64-ibm64n01.xml
        Test URI:not-wf/P64/ibm64n01.xml
        Spec Sections:3.4
        Description:Tests ignoreSectContents with wrong beginning sequence. The "?" occurs in beginning sequence the ignoreSectContents in the file ibm64n01.dtd.
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
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P64/ibm64n01.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p64ibm64n02xml() {
    /*
        Test ID:ibm-not-wf-P64-ibm64n02.xml
        Test URI:not-wf/P64/ibm64n02.xml
        Spec Sections:3.4
        Description:Tests ignoreSectContents with a required field missing.The closing sequence is missing in the ignoreSectContents in the file ibm64n02.dtd.
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
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P64/ibm64n02.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p64ibm64n03xml() {
    /*
        Test ID:ibm-not-wf-P64-ibm64n03.xml
        Test URI:not-wf/P64/ibm64n03.xml
        Spec Sections:3.4
        Description:Tests ignoreSectContents with a required field missing.The beginning sequence is missing in the ignoreSectContents in the file ibm64n03.dtd.
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
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P64/ibm64n03.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p65ibm65n01xml() {
    /*
        Test ID:ibm-not-wf-P65-ibm65n01.xml
        Test URI:not-wf/P65/ibm65n01.xml
        Spec Sections:3.4
        Description:Tests Ignore with illegal string included. The string "]](greater than)" is contained before "this" in the Ignore in the ignoreSectContents in the file ibm65n01.dtd
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
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P65/ibm65n01.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p65ibm65n02xml() {
    /*
        Test ID:ibm-not-wf-P65-ibm65n02.xml
        Test URI:not-wf/P65/ibm65n02.xml
        Spec Sections:3.4
        Description:Tests Ignore with illegal string included. The string "(less than)![" is contained before "this" in the Ignore in the ignoreSectContents in the file ibm65n02.dtd
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
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P65/ibm65n02.xml")
            .unwrap()
            .as_str(),
        ps,
        ss,
    );

    assert!(parseresult.is_err());
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p66ibm66n01xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n01.xml
        Test URI:not-wf/P66/ibm66n01.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#002f" is used as the referred character in the CharRef in the EntityDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p66ibm66n02xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n02.xml
        Test URI:not-wf/P66/ibm66n02.xml
        Spec Sections:4.1
        Description:Tests CharRef with the semicolon character missing. The semicolon character is missing at the end of the CharRef in the attribute value in the STag of element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p66ibm66n03xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n03.xml
        Test URI:not-wf/P66/ibm66n03.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "49" is used as the referred character in the CharRef in the EntityDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p66ibm66n04xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n04.xml
        Test URI:not-wf/P66/ibm66n04.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#5~0" is used as the referred character in the attribute value in the EmptyElemTag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p66ibm66n05xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n05.xml
        Test URI:not-wf/P66/ibm66n05.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#x002g" is used as the referred character in the CharRef in the EntityDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p66ibm66n06xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n06.xml
        Test URI:not-wf/P66/ibm66n06.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#x006G" is used as the referred character in the attribute value in the EmptyElemTag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p66ibm66n07xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n07.xml
        Test URI:not-wf/P66/ibm66n07.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#0=2f" is used as the referred character in the CharRef in the EntityDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p66ibm66n08xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n08.xml
        Test URI:not-wf/P66/ibm66n08.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#56.0" is used as the referred character in the attribute value in the EmptyElemTag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n08.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p66ibm66n09xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n09.xml
        Test URI:not-wf/P66/ibm66n09.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#x00/2f" is used as the referred character in the CharRef in the EntityDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n09.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p66ibm66n10xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n10.xml
        Test URI:not-wf/P66/ibm66n10.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#51)" is used as the referred character in the attribute value in the EmptyElemTag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n10.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p66ibm66n11xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n11.xml
        Test URI:not-wf/P66/ibm66n11.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#00 2f" is used as the referred character in the CharRef in the EntityDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n11.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p66ibm66n12xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n12.xml
        Test URI:not-wf/P66/ibm66n12.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#x0000" is used as the referred character in the attribute value in the EmptyElemTag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n12.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p66ibm66n13xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n13.xml
        Test URI:not-wf/P66/ibm66n13.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#x001f" is used as the referred character in the attribute value in the EmptyElemTag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n13.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p66ibm66n14xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n14.xml
        Test URI:not-wf/P66/ibm66n14.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#xfffe" is used as the referred character in the attribute value in the EmptyElemTag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n14.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p66ibm66n15xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n15.xml
        Test URI:not-wf/P66/ibm66n15.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#xffff" is used as the referred character in the attribute value in the EmptyElemTag of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n15.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p68ibm68n01xml() {
    /*
        Test ID:ibm-not-wf-P68-ibm68n01.xml
        Test URI:not-wf/P68/ibm68n01.xml
        Spec Sections:4.1
        Description:Tests EntityRef with a required field missing. The Name is missing in the EntityRef in the content of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P68/ibm68n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p68ibm68n02xml() {
    /*
        Test ID:ibm-not-wf-P68-ibm68n02.xml
        Test URI:not-wf/P68/ibm68n02.xml
        Spec Sections:4.1
        Description:Tests EntityRef with a required field missing. The semicolon is missing in the EntityRef in the attribute value in the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P68/ibm68n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p68ibm68n03xml() {
    /*
        Test ID:ibm-not-wf-P68-ibm68n03.xml
        Test URI:not-wf/P68/ibm68n03.xml
        Spec Sections:4.1
        Description:Tests EntityRef with an extra white space. A white space occurs after the ampersand in the EntityRef in the content of the element "root".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P68/ibm68n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p68ibm68n04xml() {
    /*
        Test ID:ibm-not-wf-P68-ibm68n04.xml
        Test URI:not-wf/P68/ibm68n04.xml
        Spec Sections:4.1
        Description:Tests EntityRef which is against P68 WFC: Entity Declared. The name "aAa" in the EntityRef in the AttValue in the STage of the element "root" does not match the Name of any declared entity in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P68/ibm68n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p68ibm68n05xml() {
    /*
        Test ID:ibm-not-wf-P68-ibm68n05.xml
        Test URI:not-wf/P68/ibm68n05.xml
        Spec Sections:4.1
        Description:Tests EntityRef which is against P68 WFC: Entity Declared. The entity with the name "aaa" in the EntityRef in the AttValue in the STag of the element "root" is not declared.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P68/ibm68n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p68ibm68n06xml() {
    /*
        Test ID:ibm-not-wf-P68-ibm68n06.xml
        Test URI:not-wf/P68/ibm68n06.xml
        Spec Sections:4.1
        Description:Tests EntityRef which is against P68 WFC: Entity Declared. The entity with the name "aaa" in the EntityRef in the AttValue in the STag of the element "root" is externally declared, but standalone is "yes".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P68/ibm68n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p68ibm68n07xml() {
    /*
        Test ID:ibm-not-wf-P68-ibm68n07.xml
        Test URI:not-wf/P68/ibm68n07.xml
        Spec Sections:4.1
        Description:Tests EntityRef which is against P68 WFC: Entity Declared. The entity with the name "aaa" in the EntityRef in the AttValue in the STag of the element "root" is referred before declared.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P68/ibm68n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p68ibm68n08xml() {
    /*
        Test ID:ibm-not-wf-P68-ibm68n08.xml
        Test URI:not-wf/P68/ibm68n08.xml
        Spec Sections:4.1
        Description:Tests EntityRef which is against P68 WFC: Parsed Entity. The EntityRef in the AttValue in the STag of the element "root" contains the name "aImage" of an unparsed entity.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P68/ibm68n08.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p68ibm68n09xml() {
    /*
        Test ID:ibm-not-wf-P68-ibm68n09.xml
        Test URI:not-wf/P68/ibm68n09.xml
        Spec Sections:4.1
        Description:Tests EntityRef which is against P68 WFC: No Recursion. The recursive entity reference occurs with the entity declarations for "aaa" and "bbb" in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P68/ibm68n09.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p68ibm68n10xml() {
    /*
        Test ID:ibm-not-wf-P68-ibm68n10.xml
        Test URI:not-wf/P68/ibm68n10.xml
        Spec Sections:4.1
        Description:Tests EntityRef which is against P68 WFC: No Recursion. The indirect recursive entity reference occurs with the entity declarations for "aaa", "bbb", "ccc", "ddd", and "eee" in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P68/ibm68n10.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p69ibm69n01xml() {
    /*
        Test ID:ibm-not-wf-P69-ibm69n01.xml
        Test URI:not-wf/P69/ibm69n01.xml
        Spec Sections:4.1
        Description:Tests PEReference with a required field missing. The Name "paaa" is missing in the PEReference in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P69/ibm69n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p69ibm69n02xml() {
    /*
        Test ID:ibm-not-wf-P69-ibm69n02.xml
        Test URI:not-wf/P69/ibm69n02.xml
        Spec Sections:4.1
        Description:Tests PEReference with a required field missing. The semicolon is missing in the PEReference "%paaa" in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P69/ibm69n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p69ibm69n03xml() {
    /*
        Test ID:ibm-not-wf-P69-ibm69n03.xml
        Test URI:not-wf/P69/ibm69n03.xml
        Spec Sections:4.1
        Description:Tests PEReference with an extra white space. There is an extra white space occurs before ";" in the PEReference in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P69/ibm69n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p69ibm69n04xml() {
    /*
        Test ID:ibm-not-wf-P69-ibm69n04.xml
        Test URI:not-wf/P69/ibm69n04.xml
        Spec Sections:4.1
        Description:Tests PEReference with an extra white space. There is an extra white space occurs after "%" in the PEReference in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P69/ibm69n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p69ibm69n06xml() {
    /*
        Test ID:ibm-not-wf-P69-ibm69n06.xml
        Test URI:not-wf/P69/ibm69n06.xml
        Spec Sections:4.1
        Description:Tests PEReference which is against P69 WFC: No Recursion. The recursive PE reference occurs with the entity declarations for "paaa" and "bbb" in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P69/ibm69n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p69ibm69n07xml() {
    /*
        Test ID:ibm-not-wf-P69-ibm69n07.xml
        Test URI:not-wf/P69/ibm69n07.xml
        Spec Sections:4.1
        Description:Tests PEReference which is against P69 WFC: No Recursion. The indirect recursive PE reference occurs with the entity declarations for "paaa", "bbb", "ccc", "ddd", and "eee" in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P69/ibm69n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p71ibm70n01xml() {
    /*
        Test ID:ibm-not-wf-P71-ibm70n01.xml
        Test URI:not-wf/P71/ibm70n01.xml
        Spec Sections:4.2
        Description:Tests
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P71/ibm70n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p71ibm71n01xml() {
    /*
        Test ID:ibm-not-wf-P71-ibm71n01.xml
        Test URI:not-wf/P71/ibm71n01.xml
        Spec Sections:4.2
        Description:Tests EntityDecl with a required field missing. The white space is missing between the beginning sequence and the Name "aaa" in the EntityDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P71/ibm71n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p71ibm71n02xml() {
    /*
        Test ID:ibm-not-wf-P71-ibm71n02.xml
        Test URI:not-wf/P71/ibm71n02.xml
        Spec Sections:4.2
        Description:Tests EntityDecl with a required field missing. The white space is missing between the Name "aaa" and the EntityDef "aString" in the EntityDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P71/ibm71n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p71ibm71n03xml() {
    /*
        Test ID:ibm-not-wf-P71-ibm71n03.xml
        Test URI:not-wf/P71/ibm71n03.xml
        Spec Sections:4.2
        Description:Tests EntityDecl with a required field missing. The EntityDef is missing in the EntityDecl with the Name "aaa" in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P71/ibm71n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p71ibm71n04xml() {
    /*
        Test ID:ibm-not-wf-P71-ibm71n04.xml
        Test URI:not-wf/P71/ibm71n04.xml
        Spec Sections:4.2
        Description:Tests EntityDecl with a required field missing. The Name is missing in the EntityDecl with the EntityDef "aString" in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P71/ibm71n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p71ibm71n05xml() {
    /*
        Test ID:ibm-not-wf-P71-ibm71n05.xml
        Test URI:not-wf/P71/ibm71n05.xml
        Spec Sections:4.2
        Description:Tests EntityDecl with wrong ordering. The Name "aaa" occurs after the EntityDef in the EntityDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P71/ibm71n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p71ibm71n06xml() {
    /*
        Test ID:ibm-not-wf-P71-ibm71n06.xml
        Test URI:not-wf/P71/ibm71n06.xml
        Spec Sections:4.2
        Description:Tests EntityDecl with wrong key word. The string "entity" is used as the key word in the beginning sequence in the EntityDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P71/ibm71n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p71ibm71n07xml() {
    /*
        Test ID:ibm-not-wf-P71-ibm71n07.xml
        Test URI:not-wf/P71/ibm71n07.xml
        Spec Sections:4.2
        Description:Tests EntityDecl with a required field missing. The closing bracket (greater than) is missing in the EntityDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P71/ibm71n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p71ibm71n08xml() {
    /*
        Test ID:ibm-not-wf-P71-ibm71n08.xml
        Test URI:not-wf/P71/ibm71n08.xml
        Spec Sections:4.2
        Description:Tests EntityDecl with a required field missing. The exclamation mark is missing in the beginning sequence in the EntityDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P71/ibm71n08.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p72ibm72n01xml() {
    /*
        Test ID:ibm-not-wf-P72-ibm72n01.xml
        Test URI:not-wf/P72/ibm72n01.xml
        Spec Sections:4.2
        Description:Tests PEdecl with a required field missing. The white space is missing between the beginning sequence and the "%" in the PEDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P72/ibm72n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p72ibm72n02xml() {
    /*
        Test ID:ibm-not-wf-P72-ibm72n02.xml
        Test URI:not-wf/P72/ibm72n02.xml
        Spec Sections:4.2
        Description:Tests PEdecl with a required field missing. The Name is missing in the PEDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P72/ibm72n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p72ibm72n03xml() {
    /*
        Test ID:ibm-not-wf-P72-ibm72n03.xml
        Test URI:not-wf/P72/ibm72n03.xml
        Spec Sections:4.2
        Description:Tests PEdecl with a required field missing. The white space is missing between the Name and the PEDef in the PEDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P72/ibm72n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p72ibm72n04xml() {
    /*
        Test ID:ibm-not-wf-P72-ibm72n04.xml
        Test URI:not-wf/P72/ibm72n04.xml
        Spec Sections:4.2
        Description:Tests PEdecl with a required field missing. The PEDef is missing after the Name "paaa" in the PEDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P72/ibm72n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p72ibm72n05xml() {
    /*
        Test ID:ibm-not-wf-P72-ibm72n05.xml
        Test URI:not-wf/P72/ibm72n05.xml
        Spec Sections:4.2
        Description:Tests PEdecl with wrong field ordering. The Name "paaa" occurs after the PEDef in the PEDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P72/ibm72n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p72ibm72n06xml() {
    /*
        Test ID:ibm-not-wf-P72-ibm72n06.xml
        Test URI:not-wf/P72/ibm72n06.xml
        Spec Sections:4.2
        Description:Tests PEdecl with wrong field ordering. The "%" and the Name "paaa" occurs after the PEDef in the PEDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P72/ibm72n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p72ibm72n07xml() {
    /*
        Test ID:ibm-not-wf-P72-ibm72n07.xml
        Test URI:not-wf/P72/ibm72n07.xml
        Spec Sections:4.2
        Description:Tests PEdecl with wrong key word. The string "entity" is used as the key word in the beginning sequence in the PEDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P72/ibm72n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p72ibm72n08xml() {
    /*
        Test ID:ibm-not-wf-P72-ibm72n08.xml
        Test URI:not-wf/P72/ibm72n08.xml
        Spec Sections:4.2
        Description:Tests PEdecl with a required field missing. The closing bracket (greater than) is missing in the PEDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P72/ibm72n08.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p72ibm72n09xml() {
    /*
        Test ID:ibm-not-wf-P72-ibm72n09.xml
        Test URI:not-wf/P72/ibm72n09.xml
        Spec Sections:4.2
        Description:Tests PEdecl with wrong closing sequence. The string "!(greater than)" is used as the closing sequence in the PEDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P72/ibm72n09.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p73ibm73n01xml() {
    /*
        Test ID:ibm-not-wf-P73-ibm73n01.xml
        Test URI:not-wf/P73/ibm73n01.xml
        Spec Sections:4.2
        Description:Tests EntityDef with wrong field ordering. The NDataDecl "NDATA JPGformat" occurs before the ExternalID in the EntityDef in the EntityDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P73/ibm73n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p73ibm73n03xml() {
    /*
        Test ID:ibm-not-wf-P73-ibm73n03.xml
        Test URI:not-wf/P73/ibm73n03.xml
        Spec Sections:4.2
        Description:Tests EntityDef with a required field missing. The ExternalID is missing before the NDataDecl in the EntityDef in the EntityDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P73/ibm73n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p74ibm74n01xml() {
    /*
        Test ID:ibm-not-wf-P74-ibm74n01.xml
        Test URI:not-wf/P74/ibm74n01.xml
        Spec Sections:4.2
        Description:Tests PEDef with extra fields. The NDataDecl occurs after the ExternalID in the PEDef in the PEDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P74/ibm74n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p75ibm75n01xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n01.xml
        Test URI:not-wf/P75/ibm75n01.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with wrong key word. The string "system" is used as the key word in the ExternalID in the EntityDef in the EntityDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p75ibm75n02xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n02.xml
        Test URI:not-wf/P75/ibm75n02.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with wrong key word. The string "public" is used as the key word in the ExternalID in the doctypedecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p75ibm75n03xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n03.xml
        Test URI:not-wf/P75/ibm75n03.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with wrong key word. The string "Public" is used as the key word in the ExternalID in the doctypedecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p75ibm75n04xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n04.xml
        Test URI:not-wf/P75/ibm75n04.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with wrong field ordering. The key word "PUBLIC" occurs after the PublicLiteral and the SystemLiteral in the ExternalID in the doctypedecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p75ibm75n05xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n05.xml
        Test URI:not-wf/P75/ibm75n05.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with a required field missing. The white space between "SYSTEM" and the Systemliteral is missing in the ExternalID in the EntityDef in the EntityDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p75ibm75n06xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n06.xml
        Test URI:not-wf/P75/ibm75n06.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with a required field missing. The Systemliteral is missing after "SYSTEM" in the ExternalID in the EntityDef in the EntityDecl in the DTD.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p75ibm75n07xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n07.xml
        Test URI:not-wf/P75/ibm75n07.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with a required field missing. The white space between the PublicLiteral and the Systemliteral is missing in the ExternalID in the doctypedecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p75ibm75n08xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n08.xml
        Test URI:not-wf/P75/ibm75n08.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with a required field missing. The key word "PUBLIC" is missing in the ExternalID in the doctypedecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n08.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p75ibm75n09xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n09.xml
        Test URI:not-wf/P75/ibm75n09.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with a required field missing. The white space between "PUBLIC" and the PublicLiteral is missing in the ExternalID in the doctypedecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n09.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p75ibm75n10xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n10.xml
        Test URI:not-wf/P75/ibm75n10.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with a required field missing. The PublicLiteral is missing in the ExternalID in the doctypedecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n10.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p75ibm75n11xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n11.xml
        Test URI:not-wf/P75/ibm75n11.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with a required field missing. The PublicLiteral is missing in the ExternalID in the doctypedecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n11.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p75ibm75n12xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n12.xml
        Test URI:not-wf/P75/ibm75n12.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with a required field missing. The SystemLiteral is missing in the ExternalID in the doctypedecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n12.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p75ibm75n13xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n13.xml
        Test URI:not-wf/P75/ibm75n13.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with wrong field ordering. The key word "PUBLIC" occurs after the PublicLiteral in the ExternalID in the doctypedecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n13.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p76ibm76n01xml() {
    /*
        Test ID:ibm-not-wf-P76-ibm76n01.xml
        Test URI:not-wf/P76/ibm76n01.xml
        Spec Sections:4.2.2
        Description:Tests NDataDecl with wrong key word. The string "ndata" is used as the key word in the NDataDecl in the EntityDef in the GEDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P76/ibm76n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p76ibm76n02xml() {
    /*
        Test ID:ibm-not-wf-P76-ibm76n02.xml
        Test URI:not-wf/P76/ibm76n02.xml
        Spec Sections:4.2.2
        Description:Tests NDataDecl with wrong key word. The string "NData" is used as the key word in the NDataDecl in the EntityDef in the GEDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P76/ibm76n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p76ibm76n03xml() {
    /*
        Test ID:ibm-not-wf-P76-ibm76n03.xml
        Test URI:not-wf/P76/ibm76n03.xml
        Spec Sections:4.2.2
        Description:Tests NDataDecl with a required field missing. The leading white space is missing in the NDataDecl in the EntityDef in the GEDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P76/ibm76n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p76ibm76n04xml() {
    /*
        Test ID:ibm-not-wf-P76-ibm76n04.xml
        Test URI:not-wf/P76/ibm76n04.xml
        Spec Sections:4.2.2
        Description:Tests NDataDecl with a required field missing. The key word "NDATA" is missing in the NDataDecl in the EntityDef in the GEDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P76/ibm76n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p76ibm76n05xml() {
    /*
        Test ID:ibm-not-wf-P76-ibm76n05.xml
        Test URI:not-wf/P76/ibm76n05.xml
        Spec Sections:4.2.2
        Description:Tests NDataDecl with a required field missing. The Name after the key word "NDATA" is missing in the NDataDecl in the EntityDef in the GEDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P76/ibm76n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p76ibm76n06xml() {
    /*
        Test ID:ibm-not-wf-P76-ibm76n06.xml
        Test URI:not-wf/P76/ibm76n06.xml
        Spec Sections:4.2.2
        Description:Tests NDataDecl with a required field missing. The white space between "NDATA" and the Name is missing in the NDataDecl in the EntityDef in the GEDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P76/ibm76n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p76ibm76n07xml() {
    /*
        Test ID:ibm-not-wf-P76-ibm76n07.xml
        Test URI:not-wf/P76/ibm76n07.xml
        Spec Sections:4.2.2
        Description:Tests NDataDecl with wrong field ordering. The key word "NDATA" occurs after the Name in the NDataDecl in the EntityDef in the GEDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P76/ibm76n07.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p77ibm77n01xml() {
    /*
        Test ID:ibm-not-wf-P77-ibm77n01.xml
        Test URI:not-wf/P77/ibm77n01.xml
        Spec Sections:4.3.1
        Description:Tests TextDecl with wrong field ordering. The VersionInfo occurs after the EncodingDecl in the TextDecl in the file "ibm77n01.ent".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P77/ibm77n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p77ibm77n02xml() {
    /*
        Test ID:ibm-not-wf-P77-ibm77n02.xml
        Test URI:not-wf/P77/ibm77n02.xml
        Spec Sections:4.3.1
        Description:Tests TextDecl with wrong key word. The string "XML" is used in the beginning sequence in the TextDecl in the file "ibm77n02.ent".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P77/ibm77n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p77ibm77n03xml() {
    /*
        Test ID:ibm-not-wf-P77-ibm77n03.xml
        Test URI:not-wf/P77/ibm77n03.xml
        Spec Sections:4.3.1
        Description:Tests TextDecl with wrong closing sequence. The character "greater than" is used as the closing sequence in the TextDecl in the file "ibm77n03.ent".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P77/ibm77n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p77ibm77n04xml() {
    /*
        Test ID:ibm-not-wf-P77-ibm77n04.xml
        Test URI:not-wf/P77/ibm77n04.xml
        Spec Sections:4.3.1
        Description:Tests TextDecl with a required field missing. The closing sequence is missing in the TextDecl in the file "ibm77n04.ent".
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P77/ibm77n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p78ibm78n01xml() {
    /*
        Test ID:ibm-not-wf-P78-ibm78n01.xml
        Test URI:not-wf/P78/ibm78n01.xml
        Spec Sections:4.3.2
        Description:Tests extParsedEnt with wrong field ordering. The TextDecl occurs after the content in the file ibm78n01.ent.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P78/ibm78n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p78ibm78n02xml() {
    /*
        Test ID:ibm-not-wf-P78-ibm78n02.xml
        Test URI:not-wf/P78/ibm78n02.xml
        Spec Sections:4.3.2
        Description:Tests extParsedEnt with extra field. A blank line occurs before the TextDecl in the file ibm78n02.ent.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P78/ibm78n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p79ibm79n01xml() {
    /*
        Test ID:ibm-not-wf-P79-ibm79n01.xml
        Test URI:not-wf/P79/ibm79n01.xml
        Spec Sections:4.3.2
        Description:Tests extPE with wrong field ordering. The TextDecl occurs after the extSubsetDecl (the white space and the comment) in the file ibm79n01.ent.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P79/ibm79n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p79ibm79n02xml() {
    /*
        Test ID:ibm-not-wf-P79-ibm79n02.xml
        Test URI:not-wf/P79/ibm79n02.xml
        Spec Sections:4.3.2
        Description:Tests extPE with extra field. A blank line occurs before the TextDecl in the file ibm78n02.ent.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P79/ibm79n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p80ibm80n01xml() {
    /*
        Test ID:ibm-not-wf-P80-ibm80n01.xml
        Test URI:not-wf/P80/ibm80n01.xml
        Spec Sections:4.3.3
        Description:Tests EncodingDecl with a required field missing. The leading white space is missing in the EncodingDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P80/ibm80n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p80ibm80n02xml() {
    /*
        Test ID:ibm-not-wf-P80-ibm80n02.xml
        Test URI:not-wf/P80/ibm80n02.xml
        Spec Sections:4.3.3
        Description:Tests EncodingDecl with a required field missing. The "=" sign is missing in the EncodingDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P80/ibm80n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p80ibm80n03xml() {
    /*
        Test ID:ibm-not-wf-P80-ibm80n03.xml
        Test URI:not-wf/P80/ibm80n03.xml
        Spec Sections:4.3.3
        Description:Tests EncodingDecl with a required field missing. The double quoted EncName are missing in the EncodingDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P80/ibm80n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p80ibm80n04xml() {
    /*
        Test ID:ibm-not-wf-P80-ibm80n04.xml
        Test URI:not-wf/P80/ibm80n04.xml
        Spec Sections:4.3.3
        Description:Tests EncodingDecl with wrong field ordering. The string "encoding=" occurs after the double quoted EncName in the EncodingDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P80/ibm80n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p80ibm80n05xml() {
    /*
        Test ID:ibm-not-wf-P80-ibm80n05.xml
        Test URI:not-wf/P80/ibm80n05.xml
        Spec Sections:4.3.3
        Description:Tests EncodingDecl with wrong field ordering. The "encoding" occurs after the double quoted EncName in the EncodingDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P80/ibm80n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p80ibm80n06xml() {
    /*
        Test ID:ibm-not-wf-P80-ibm80n06.xml
        Test URI:not-wf/P80/ibm80n06.xml
        Spec Sections:4.3.3
        Description:Tests EncodingDecl with wrong key word. The string "Encoding" is used as the key word in the EncodingDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P80/ibm80n06.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p81ibm81n01xml() {
    /*
        Test ID:ibm-not-wf-P81-ibm81n01.xml
        Test URI:not-wf/P81/ibm81n01.xml
        Spec Sections:4.3.3
        Description:Tests EncName with an illegal character. The "_" is used as the first character in the EncName in the EncodingDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P81/ibm81n01.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p81ibm81n02xml() {
    /*
        Test ID:ibm-not-wf-P81-ibm81n02.xml
        Test URI:not-wf/P81/ibm81n02.xml
        Spec Sections:4.3.3
        Description:Tests EncName with an illegal character. The "-" is used as the first character in the EncName in the EncodingDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P81/ibm81n02.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p81ibm81n03xml() {
    /*
        Test ID:ibm-not-wf-P81-ibm81n03.xml
        Test URI:not-wf/P81/ibm81n03.xml
        Spec Sections:4.3.3
        Description:Tests EncName with an illegal character. The "." is used as the first character in the EncName in the EncodingDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P81/ibm81n03.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p81ibm81n04xml() {
    /*
        Test ID:ibm-not-wf-P81-ibm81n04.xml
        Test URI:not-wf/P81/ibm81n04.xml
        Spec Sections:4.3.3
        Description:Tests EncName with illegal characters. The "8-" is used as the initial characters in the EncName in the EncodingDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P81/ibm81n04.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p81ibm81n05xml() {
    /*
        Test ID:ibm-not-wf-P81-ibm81n05.xml
        Test URI:not-wf/P81/ibm81n05.xml
        Spec Sections:4.3.3
        Description:Tests EncName with an illegal character. The "~" is used as one character in the EncName in the EncodingDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P81/ibm81n05.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn ibmnotwf_p81ibm81n06xml() {
    /*
        Test ID:ibm-not-wf-P81-ibm81n06.xml
        Test URI:not-wf/P81/ibm81n06.xml
        Spec Sections:4.3.3
        Description:Tests EncName with an illegal character. The "#" is used as one character in the EncName in the EncodingDecl in the XMLDecl.
    */

    test_ibm_notwf(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P81/ibm81n06.xml")
            .unwrap()
            .as_str(),
    );
}
