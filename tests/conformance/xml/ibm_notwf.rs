/*

IBM test cases

*/
use std::convert::TryFrom;
use std::fs;
use xrust::Document;

#[test]
#[ignore]
fn ibmnotwf_p01ibm01n01xml() {
    /*
        Test ID:ibm-not-wf-P01-ibm01n01.xml
        Test URI:not-wf/P01/ibm01n01.xml
        Spec Sections:2.1
        Description:Tests a document with no element. A well-formed document should have at lease one elements.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P01/ibm01n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p01ibm01n02xml() {
    /*
        Test ID:ibm-not-wf-P01-ibm01n02.xml
        Test URI:not-wf/P01/ibm01n02.xml
        Spec Sections:2.1
        Description:Tests a document with wrong ordering of its prolog and element. The element occurs before the xml declaration and the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P01/ibm01n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p01ibm01n03xml() {
    /*
        Test ID:ibm-not-wf-P01-ibm01n03.xml
        Test URI:not-wf/P01/ibm01n03.xml
        Spec Sections:2.1
        Description:Tests a document with wrong combination of misc and element. One PI occurs between two elements.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P01/ibm01n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p02ibm02n01xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n01.xml
        Test URI:not-wf/P02/ibm02n01.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x00
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p02ibm02n02xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n02.xml
        Test URI:not-wf/P02/ibm02n02.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x01
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n03xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n03.xml
        Test URI:not-wf/P02/ibm02n03.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x02
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n04xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n04.xml
        Test URI:not-wf/P02/ibm02n04.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x03
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n05xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n05.xml
        Test URI:not-wf/P02/ibm02n05.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x04
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n06xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n06.xml
        Test URI:not-wf/P02/ibm02n06.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x05
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n07xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n07.xml
        Test URI:not-wf/P02/ibm02n07.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x06
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n08xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n08.xml
        Test URI:not-wf/P02/ibm02n08.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x07
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n09xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n09.xml
        Test URI:not-wf/P02/ibm02n09.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x08
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n10xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n10.xml
        Test URI:not-wf/P02/ibm02n10.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x0B
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n10.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n11xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n11.xml
        Test URI:not-wf/P02/ibm02n11.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x0C
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n11.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n12xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n12.xml
        Test URI:not-wf/P02/ibm02n12.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x0E
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n12.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n13xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n13.xml
        Test URI:not-wf/P02/ibm02n13.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x0F
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n13.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n14xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n14.xml
        Test URI:not-wf/P02/ibm02n14.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x10
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n14.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n15xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n15.xml
        Test URI:not-wf/P02/ibm02n15.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x11
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n15.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n16xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n16.xml
        Test URI:not-wf/P02/ibm02n16.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x12
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n16.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n17xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n17.xml
        Test URI:not-wf/P02/ibm02n17.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x13
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n17.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n18xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n18.xml
        Test URI:not-wf/P02/ibm02n18.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x14
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n18.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n19xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n19.xml
        Test URI:not-wf/P02/ibm02n19.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x15
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n19.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n20xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n20.xml
        Test URI:not-wf/P02/ibm02n20.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x16
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n20.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n21xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n21.xml
        Test URI:not-wf/P02/ibm02n21.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x17
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n21.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n22xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n22.xml
        Test URI:not-wf/P02/ibm02n22.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x18
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n22.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n23xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n23.xml
        Test URI:not-wf/P02/ibm02n23.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x19
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n23.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n24xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n24.xml
        Test URI:not-wf/P02/ibm02n24.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x1A
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n24.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n25xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n25.xml
        Test URI:not-wf/P02/ibm02n25.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x1B
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n25.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n26xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n26.xml
        Test URI:not-wf/P02/ibm02n26.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x1C
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n26.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n27xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n27.xml
        Test URI:not-wf/P02/ibm02n27.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x1D
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n27.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n28xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n28.xml
        Test URI:not-wf/P02/ibm02n28.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x1E
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n28.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n29xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n29.xml
        Test URI:not-wf/P02/ibm02n29.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #x1F
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n29.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p02ibm02n30xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n30.xml
        Test URI:not-wf/P02/ibm02n30.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #xD800
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n30.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p02ibm02n31xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n31.xml
        Test URI:not-wf/P02/ibm02n31.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #xDFFF
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n31.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n32xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n32.xml
        Test URI:not-wf/P02/ibm02n32.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #xFFFE
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n32.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p02ibm02n33xml() {
    /*
        Test ID:ibm-not-wf-P02-ibm02n33.xml
        Test URI:not-wf/P02/ibm02n33.xml
        Spec Sections:2.2
        Description:Tests a comment which contains an illegal Char: #xFFFF
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P02/ibm02n33.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p03ibm03n01xml() {
    /*
        Test ID:ibm-not-wf-P03-ibm03n01.xml
        Test URI:not-wf/P03/ibm03n01.xml
        Spec Sections:2.3
        Description:Tests an end tag which contains an illegal space character #x3000 which follows the element name "book".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P03/ibm03n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p04ibm04n01xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n01.xml
        Test URI:not-wf/P04/ibm04n01.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x21
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p04ibm04n02xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n02.xml
        Test URI:not-wf/P04/ibm04n02.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x28
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p04ibm04n03xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n03.xml
        Test URI:not-wf/P04/ibm04n03.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x29
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p04ibm04n04xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n04.xml
        Test URI:not-wf/P04/ibm04n04.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x2B
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p04ibm04n05xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n05.xml
        Test URI:not-wf/P04/ibm04n05.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x2C
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p04ibm04n06xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n06.xml
        Test URI:not-wf/P04/ibm04n06.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x2F
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p04ibm04n07xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n07.xml
        Test URI:not-wf/P04/ibm04n07.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x3B
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p04ibm04n08xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n08.xml
        Test URI:not-wf/P04/ibm04n08.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x3C
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p04ibm04n09xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n09.xml
        Test URI:not-wf/P04/ibm04n09.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x3D
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p04ibm04n10xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n10.xml
        Test URI:not-wf/P04/ibm04n10.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x3F
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n10.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p04ibm04n11xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n11.xml
        Test URI:not-wf/P04/ibm04n11.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x5B
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n11.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p04ibm04n12xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n12.xml
        Test URI:not-wf/P04/ibm04n12.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x5C
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n12.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p04ibm04n13xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n13.xml
        Test URI:not-wf/P04/ibm04n13.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x5D
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n13.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p04ibm04n14xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n14.xml
        Test URI:not-wf/P04/ibm04n14.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x5E
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n14.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p04ibm04n15xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n15.xml
        Test URI:not-wf/P04/ibm04n15.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x60
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n15.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p04ibm04n16xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n16.xml
        Test URI:not-wf/P04/ibm04n16.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x7B
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n16.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p04ibm04n17xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n17.xml
        Test URI:not-wf/P04/ibm04n17.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x7C
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n17.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p04ibm04n18xml() {
    /*
        Test ID:ibm-not-wf-P04-ibm04n18.xml
        Test URI:not-wf/P04/ibm04n18.xml
        Spec Sections:2.3
        Description:Tests an element name which contains an illegal ASCII NameChar. "IllegalNameChar" is followed by #x7D
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P04/ibm04n18.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p05ibm05n01xml() {
    /*
        Test ID:ibm-not-wf-P05-ibm05n01.xml
        Test URI:not-wf/P05/ibm05n01.xml
        Spec Sections:2.3
        Description:Tests an element name which has an illegal first character. An illegal first character "." is followed by "A_name-starts_with.".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P05/ibm05n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p05ibm05n02xml() {
    /*
        Test ID:ibm-not-wf-P05-ibm05n02.xml
        Test URI:not-wf/P05/ibm05n02.xml
        Spec Sections:2.3
        Description:Tests an element name which has an illegal first character. An illegal first character "-" is followed by "A_name-starts_with-".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P05/ibm05n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p05ibm05n03xml() {
    /*
        Test ID:ibm-not-wf-P05-ibm05n03.xml
        Test URI:not-wf/P05/ibm05n03.xml
        Spec Sections:2.3
        Description:Tests an element name which has an illegal first character. An illegal first character "5" is followed by "A_name-starts_with_digit".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P05/ibm05n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p09ibm09n01xml() {
    /*
        Test ID:ibm-not-wf-P09-ibm09n01.xml
        Test URI:not-wf/P09/ibm09n01.xml
        Spec Sections:2.3
        Description:Tests an internal general entity with an invalid value. The entity "Fullname" contains "%".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P09/ibm09n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p09ibm09n02xml() {
    /*
        Test ID:ibm-not-wf-P09-ibm09n02.xml
        Test URI:not-wf/P09/ibm09n02.xml
        Spec Sections:2.3
        Description:Tests an internal general entity with an invalid value. The entity "Fullname" contains the ampersand character.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P09/ibm09n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p09ibm09n03xml() {
    /*
        Test ID:ibm-not-wf-P09-ibm09n03.xml
        Test URI:not-wf/P09/ibm09n03.xml
        Spec Sections:2.3
        Description:Tests an internal general entity with an invalid value. The entity "Fullname" contains the double quote character in the middle.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P09/ibm09n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p09ibm09n04xml() {
    /*
        Test ID:ibm-not-wf-P09-ibm09n04.xml
        Test URI:not-wf/P09/ibm09n04.xml
        Spec Sections:2.3
        Description:Tests an internal general entity with an invalid value. The closing bracket (double quote) is missing with the value of the entity "FullName".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P09/ibm09n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p10ibm10n01xml() {
    /*
        Test ID:ibm-not-wf-P10-ibm10n01.xml
        Test URI:not-wf/P10/ibm10n01.xml
        Spec Sections:2.3
        Description:Tests an attribute with an invalid value. The value of the attribute "first" contains the character "less than".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P10/ibm10n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p10ibm10n02xml() {
    /*
        Test ID:ibm-not-wf-P10-ibm10n02.xml
        Test URI:not-wf/P10/ibm10n02.xml
        Spec Sections:2.3
        Description:Tests an attribute with an invalid value. The value of the attribute "first" contains the character ampersand.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P10/ibm10n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p10ibm10n03xml() {
    /*
        Test ID:ibm-not-wf-P10-ibm10n03.xml
        Test URI:not-wf/P10/ibm10n03.xml
        Spec Sections:2.3
        Description:Tests an attribute with an invalid value. The value of the attribute "first" contains the double quote character in the middle.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P10/ibm10n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p10ibm10n04xml() {
    /*
        Test ID:ibm-not-wf-P10-ibm10n04.xml
        Test URI:not-wf/P10/ibm10n04.xml
        Spec Sections:2.3
        Description:Tests an attribute with an invalid value. The closing bracket (double quote) is missing with The value of the attribute "first".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P10/ibm10n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p10ibm10n05xml() {
    /*
        Test ID:ibm-not-wf-P10-ibm10n05.xml
        Test URI:not-wf/P10/ibm10n05.xml
        Spec Sections:2.3
        Description:Tests an attribute with an invalid value. The value of the attribute "first" contains the character "less than".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P10/ibm10n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p10ibm10n06xml() {
    /*
        Test ID:ibm-not-wf-P10-ibm10n06.xml
        Test URI:not-wf/P10/ibm10n06.xml
        Spec Sections:2.3
        Description:Tests an attribute with an invalid value. The value of the attribute "first" contains the character ampersand.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P10/ibm10n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p10ibm10n07xml() {
    /*
        Test ID:ibm-not-wf-P10-ibm10n07.xml
        Test URI:not-wf/P10/ibm10n07.xml
        Spec Sections:2.3
        Description:Tests an attribute with an invalid value. The value of the attribute "first" contains the double quote character in the middle.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P10/ibm10n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p10ibm10n08xml() {
    /*
        Test ID:ibm-not-wf-P10-ibm10n08.xml
        Test URI:not-wf/P10/ibm10n08.xml
        Spec Sections:2.3
        Description:Tests an attribute with an invalid value. The closing bracket (single quote) is missing with the value of the attribute "first".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P10/ibm10n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p11ibm11n01xml() {
    /*
        Test ID:ibm-not-wf-P11-ibm11n01.xml
        Test URI:not-wf/P11/ibm11n01.xml
        Spec Sections:2.3
        Description:Tests SystemLiteral. The systemLiteral for the element "student" has a double quote character in the middle.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P11/ibm11n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p11ibm11n02xml() {
    /*
        Test ID:ibm-not-wf-P11-ibm11n02.xml
        Test URI:not-wf/P11/ibm11n02.xml
        Spec Sections:2.3
        Description:Tests SystemLiteral. The systemLiteral for the element "student" has a single quote character in the middle.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P11/ibm11n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p11ibm11n03xml() {
    /*
        Test ID:ibm-not-wf-P11-ibm11n03.xml
        Test URI:not-wf/P11/ibm11n03.xml
        Spec Sections:2.3
        Description:Tests SystemLiteral. The closing bracket (double quote) is missing with the systemLiteral for the element "student".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P11/ibm11n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p11ibm11n04xml() {
    /*
        Test ID:ibm-not-wf-P11-ibm11n04.xml
        Test URI:not-wf/P11/ibm11n04.xml
        Spec Sections:2.3
        Description:Tests SystemLiteral. The closing bracket (single quote) is missing with the systemLiteral for the element "student".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P11/ibm11n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p12ibm12n01xml() {
    /*
        Test ID:ibm-not-wf-P12-ibm12n01.xml
        Test URI:not-wf/P12/ibm12n01.xml
        Spec Sections:2.3
        Description:Tests PubidLiteral. The closing bracket (double quote) is missing with the value of the PubidLiteral for the entity "info".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P12/ibm12n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p12ibm12n02xml() {
    /*
        Test ID:ibm-not-wf-P12-ibm12n02.xml
        Test URI:not-wf/P12/ibm12n02.xml
        Spec Sections:2.3
        Description:Tests PubidLiteral. The value of the PubidLiteral for the entity "info" has a single quote character in the middle..
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P12/ibm12n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p12ibm12n03xml() {
    /*
        Test ID:ibm-not-wf-P12-ibm12n03.xml
        Test URI:not-wf/P12/ibm12n03.xml
        Spec Sections:2.3
        Description:Tests PubidLiteral. The closing bracket (single quote) is missing with the value of the PubidLiteral for the entity "info".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P12/ibm12n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p13ibm13n01xml() {
    /*
        Test ID:ibm-not-wf-P13-ibm13n01.xml
        Test URI:not-wf/P13/ibm13n01.xml
        Spec Sections:2.3
        Description:Tests PubidChar. The pubidChar of the PubidLiteral for the entity "info" contains the character "{".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P13/ibm13n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p13ibm13n02xml() {
    /*
        Test ID:ibm-not-wf-P13-ibm13n02.xml
        Test URI:not-wf/P13/ibm13n02.xml
        Spec Sections:2.3
        Description:Tests PubidChar. The pubidChar of the PubidLiteral for the entity "info" contains the character "~".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P13/ibm13n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p13ibm13n03xml() {
    /*
        Test ID:ibm-not-wf-P13-ibm13n03.xml
        Test URI:not-wf/P13/ibm13n03.xml
        Spec Sections:2.3
        Description:Tests PubidChar. The pubidChar of the PubidLiteral for the entity "info" contains the character double quote in the middle.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P13/ibm13n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p14ibm14n01xml() {
    /*
        Test ID:ibm-not-wf-P14-ibm14n01.xml
        Test URI:not-wf/P14/ibm14n01.xml
        Spec Sections:2.4
        Description:Tests CharData. The content of the element "student" contains the sequence close-bracket close-bracket greater-than.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P14/ibm14n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p14ibm14n02xml() {
    /*
        Test ID:ibm-not-wf-P14-ibm14n02.xml
        Test URI:not-wf/P14/ibm14n02.xml
        Spec Sections:2.4
        Description:Tests CharData. The content of the element "student" contains the character "less than".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P14/ibm14n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p14ibm14n03xml() {
    /*
        Test ID:ibm-not-wf-P14-ibm14n03.xml
        Test URI:not-wf/P14/ibm14n03.xml
        Spec Sections:2.4
        Description:Tests CharData. The content of the element "student" contains the character ampersand.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P14/ibm14n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p15ibm15n01xml() {
    /*
        Test ID:ibm-not-wf-P15-ibm15n01.xml
        Test URI:not-wf/P15/ibm15n01.xml
        Spec Sections:2.5
        Description:Tests comment. The text of the second comment contains the character "-".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P15/ibm15n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p15ibm15n02xml() {
    /*
        Test ID:ibm-not-wf-P15-ibm15n02.xml
        Test URI:not-wf/P15/ibm15n02.xml
        Spec Sections:2.5
        Description:Tests comment. The second comment has a wrong closing sequence "-(greater than)".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P15/ibm15n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p15ibm15n03xml() {
    /*
        Test ID:ibm-not-wf-P15-ibm15n03.xml
        Test URI:not-wf/P15/ibm15n03.xml
        Spec Sections:2.5
        Description:Tests comment. The second comment has a wrong beginning sequence "(less than)!-".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P15/ibm15n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p15ibm15n04xml() {
    /*
        Test ID:ibm-not-wf-P15-ibm15n04.xml
        Test URI:not-wf/P15/ibm15n04.xml
        Spec Sections:2.5
        Description:Tests comment. The closing sequence is missing with the second comment.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P15/ibm15n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p16ibm16n01xml() {
    /*
        Test ID:ibm-not-wf-P16-ibm16n01.xml
        Test URI:not-wf/P16/ibm16n01.xml
        Spec Sections:2.6
        Description:Tests PI. The content of the PI includes the sequence "?(greater than)?".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P16/ibm16n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p16ibm16n02xml() {
    /*
        Test ID:ibm-not-wf-P16-ibm16n02.xml
        Test URI:not-wf/P16/ibm16n02.xml
        Spec Sections:2.6
        Description:Tests PI. The PITarget is missing in the PI.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P16/ibm16n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p16ibm16n03xml() {
    /*
        Test ID:ibm-not-wf-P16-ibm16n03.xml
        Test URI:not-wf/P16/ibm16n03.xml
        Spec Sections:2.6
        Description:Tests PI. The PI has a wrong closing sequence ">".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P16/ibm16n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p16ibm16n04xml() {
    /*
        Test ID:ibm-not-wf-P16-ibm16n04.xml
        Test URI:not-wf/P16/ibm16n04.xml
        Spec Sections:2.6
        Description:Tests PI. The closing sequence is missing in the PI.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P16/ibm16n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p17ibm17n01xml() {
    /*
        Test ID:ibm-not-wf-P17-ibm17n01.xml
        Test URI:not-wf/P17/ibm17n01.xml
        Spec Sections:2.6
        Description:Tests PITarget. The PITarget contains the string "XML".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P17/ibm17n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p17ibm17n02xml() {
    /*
        Test ID:ibm-not-wf-P17-ibm17n02.xml
        Test URI:not-wf/P17/ibm17n02.xml
        Spec Sections:2.6
        Description:Tests PITarget. The PITarget contains the string "xML".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P17/ibm17n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p17ibm17n03xml() {
    /*
        Test ID:ibm-not-wf-P17-ibm17n03.xml
        Test URI:not-wf/P17/ibm17n03.xml
        Spec Sections:2.6
        Description:Tests PITarget. The PITarget contains the string "xml".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P17/ibm17n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p17ibm17n04xml() {
    /*
        Test ID:ibm-not-wf-P17-ibm17n04.xml
        Test URI:not-wf/P17/ibm17n04.xml
        Spec Sections:2.6
        Description:Tests PITarget. The PITarget contains the string "xmL".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P17/ibm17n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p18ibm18n01xml() {
    /*
        Test ID:ibm-not-wf-P18-ibm18n01.xml
        Test URI:not-wf/P18/ibm18n01.xml
        Spec Sections:2.7
        Description:Tests CDSect. The CDStart is missing in the CDSect in the content of element "student".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P18/ibm18n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p18ibm18n02xml() {
    /*
        Test ID:ibm-not-wf-P18-ibm18n02.xml
        Test URI:not-wf/P18/ibm18n02.xml
        Spec Sections:2.7
        Description:Tests CDSect. The CDEnd is missing in the CDSect in the content of element "student".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P18/ibm18n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p19ibm19n01xml() {
    /*
        Test ID:ibm-not-wf-P19-ibm19n01.xml
        Test URI:not-wf/P19/ibm19n01.xml
        Spec Sections:2.7
        Description:Tests CDStart. The CDStart contains a lower case string "cdata".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P19/ibm19n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p19ibm19n02xml() {
    /*
        Test ID:ibm-not-wf-P19-ibm19n02.xml
        Test URI:not-wf/P19/ibm19n02.xml
        Spec Sections:2.7
        Description:Tests CDStart. The CDStart contains an extra character "[".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P19/ibm19n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p19ibm19n03xml() {
    /*
        Test ID:ibm-not-wf-P19-ibm19n03.xml
        Test URI:not-wf/P19/ibm19n03.xml
        Spec Sections:2.7
        Description:Tests CDStart. The CDStart contains a wrong character "?".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P19/ibm19n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p20ibm20n01xml() {
    /*
        Test ID:ibm-not-wf-P20-ibm20n01.xml
        Test URI:not-wf/P20/ibm20n01.xml
        Spec Sections:2.7
        Description:Tests CDATA with an illegal sequence. The CDATA contains the sequence close-bracket close-bracket greater-than.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P20/ibm20n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p21ibm21n01xml() {
    /*
        Test ID:ibm-not-wf-P21-ibm21n01.xml
        Test URI:not-wf/P21/ibm21n01.xml
        Spec Sections:2.7
        Description:Tests CDEnd. One "]" is missing in the CDEnd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P21/ibm21n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p21ibm21n02xml() {
    /*
        Test ID:ibm-not-wf-P21-ibm21n02.xml
        Test URI:not-wf/P21/ibm21n02.xml
        Spec Sections:2.7
        Description:Tests CDEnd. An extra "]" is placed in the CDEnd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P21/ibm21n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p21ibm21n03xml() {
    /*
        Test ID:ibm-not-wf-P21-ibm21n03.xml
        Test URI:not-wf/P21/ibm21n03.xml
        Spec Sections:2.7
        Description:Tests CDEnd. A wrong character ")" is placed in the CDEnd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P21/ibm21n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p22ibm22n01xml() {
    /*
        Test ID:ibm-not-wf-P22-ibm22n01.xml
        Test URI:not-wf/P22/ibm22n01.xml
        Spec Sections:2.8
        Description:Tests prolog with wrong field ordering. The XMLDecl occurs after the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P22/ibm22n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p22ibm22n02xml() {
    /*
        Test ID:ibm-not-wf-P22-ibm22n02.xml
        Test URI:not-wf/P22/ibm22n02.xml
        Spec Sections:2.8
        Description:Tests prolog with wrong field ordering. The Misc (comment) occurs before the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P22/ibm22n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p22ibm22n03xml() {
    /*
        Test ID:ibm-not-wf-P22-ibm22n03.xml
        Test URI:not-wf/P22/ibm22n03.xml
        Spec Sections:2.8
        Description:Tests prolog with wrong field ordering. The XMLDecl occurs after the DTD and a comment. The other comment occurs before the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P22/ibm22n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p23ibm23n01xml() {
    /*
        Test ID:ibm-not-wf-P23-ibm23n01.xml
        Test URI:not-wf/P23/ibm23n01.xml
        Spec Sections:2.8
        Description:Tests XMLDecl with a required field missing. The Versioninfo is missing in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P23/ibm23n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p23ibm23n02xml() {
    /*
        Test ID:ibm-not-wf-P23-ibm23n02.xml
        Test URI:not-wf/P23/ibm23n02.xml
        Spec Sections:2.8
        Description:Tests XMLDecl with wrong field ordering. The VersionInfo occurs after the EncodingDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P23/ibm23n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p23ibm23n03xml() {
    /*
        Test ID:ibm-not-wf-P23-ibm23n03.xml
        Test URI:not-wf/P23/ibm23n03.xml
        Spec Sections:2.8
        Description:Tests XMLDecl with wrong field ordering. The VersionInfo occurs after the SDDecl and the SDDecl occurs after the VersionInfo.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P23/ibm23n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p23ibm23n04xml() {
    /*
        Test ID:ibm-not-wf-P23-ibm23n04.xml
        Test URI:not-wf/P23/ibm23n04.xml
        Spec Sections:2.8
        Description:Tests XMLDecl with wrong key word. An upper case string "XML" is used as the key word in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P23/ibm23n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p23ibm23n05xml() {
    /*
        Test ID:ibm-not-wf-P23-ibm23n05.xml
        Test URI:not-wf/P23/ibm23n05.xml
        Spec Sections:2.8
        Description:Tests XMLDecl with a wrong closing sequence ">".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P23/ibm23n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p23ibm23n06xml() {
    /*
        Test ID:ibm-not-wf-P23-ibm23n06.xml
        Test URI:not-wf/P23/ibm23n06.xml
        Spec Sections:2.8
        Description:Tests XMLDecl with a wrong opening sequence "(less than)!".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P23/ibm23n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p24ibm24n01xml() {
    /*
        Test ID:ibm-not-wf-P24-ibm24n01.xml
        Test URI:not-wf/P24/ibm24n01.xml
        Spec Sections:2.8
        Description:Tests VersionInfo with a required field missing. The VersionNum is missing in the VersionInfo in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P24/ibm24n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p24ibm24n02xml() {
    /*
        Test ID:ibm-not-wf-P24-ibm24n02.xml
        Test URI:not-wf/P24/ibm24n02.xml
        Spec Sections:2.8
        Description:Tests VersionInfo with a required field missing. The white space is missing between the key word "xml" and the VersionInfo in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P24/ibm24n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p24ibm24n03xml() {
    /*
        Test ID:ibm-not-wf-P24-ibm24n03.xml
        Test URI:not-wf/P24/ibm24n03.xml
        Spec Sections:2.8
        Description:Tests VersionInfo with a required field missing. The "=" (equal sign) is missing between the key word "version" and the VersionNum.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P24/ibm24n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p24ibm24n04xml() {
    /*
        Test ID:ibm-not-wf-P24-ibm24n04.xml
        Test URI:not-wf/P24/ibm24n04.xml
        Spec Sections:2.8
        Description:Tests VersionInfo with wrong field ordering. The VersionNum occurs before "=" and "version".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P24/ibm24n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p24ibm24n05xml() {
    /*
        Test ID:ibm-not-wf-P24-ibm24n05.xml
        Test URI:not-wf/P24/ibm24n05.xml
        Spec Sections:2.8
        Description:Tests VersionInfo with wrong field ordering. The "=" occurs after "version" and the VersionNum.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P24/ibm24n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p24ibm24n06xml() {
    /*
        Test ID:ibm-not-wf-P24-ibm24n06.xml
        Test URI:not-wf/P24/ibm24n06.xml
        Spec Sections:2.8
        Description:Tests VersionInfo with the wrong key word "Version".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P24/ibm24n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p24ibm24n07xml() {
    /*
        Test ID:ibm-not-wf-P24-ibm24n07.xml
        Test URI:not-wf/P24/ibm24n07.xml
        Spec Sections:2.8
        Description:Tests VersionInfo with the wrong key word "versioN".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P24/ibm24n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p24ibm24n08xml() {
    /*
        Test ID:ibm-not-wf-P24-ibm24n08.xml
        Test URI:not-wf/P24/ibm24n08.xml
        Spec Sections:2.8
        Description:Tests VersionInfo with mismatched quotes around the VersionNum. version = '1.0" is used as the VersionInfo.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P24/ibm24n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p24ibm24n09xml() {
    /*
        Test ID:ibm-not-wf-P24-ibm24n09.xml
        Test URI:not-wf/P24/ibm24n09.xml
        Spec Sections:2.8
        Description:Tests VersionInfo with mismatched quotes around the VersionNum. The closing bracket for the VersionNum is missing.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P24/ibm24n09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p25ibm25n01xml() {
    /*
        Test ID:ibm-not-wf-P25-ibm25n01.xml
        Test URI:not-wf/P25/ibm25n01.xml
        Spec Sections:2.8
        Description:Tests eq with a wrong key word "==".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P25/ibm25n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p25ibm25n02xml() {
    /*
        Test ID:ibm-not-wf-P25-ibm25n02.xml
        Test URI:not-wf/P25/ibm25n02.xml
        Spec Sections:2.8
        Description:Tests eq with a wrong key word "eq".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P25/ibm25n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p26ibm26n01xml() {
    /*
        Test ID:ibm-not-wf-P26-ibm26n01.xml
        Test URI:not-wf/P26/ibm26n01.xml
        Spec Sections:2.8
        Description:Tests VersionNum with an illegal character "#".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P26/ibm26n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p27ibm27n01xml() {
    /*
        Test ID:ibm-not-wf-P27-ibm27n01.xml
        Test URI:not-wf/P27/ibm27n01.xml
        Spec Sections:2.8
        Description:Tests type of Misc. An element declaration is used as a type of Misc After the element "animal".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P27/ibm27n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p28ibm28n01xml() {
    /*
        Test ID:ibm-not-wf-P28-ibm28n01.xml
        Test URI:not-wf/P28/ibm28n01.xml
        Spec Sections:2.8
        Description:Tests doctypedecl with a required field missing. The Name "animal" is missing in the doctypedecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P28/ibm28n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p28ibm28n02xml() {
    /*
        Test ID:ibm-not-wf-P28-ibm28n02.xml
        Test URI:not-wf/P28/ibm28n02.xml
        Spec Sections:2.8
        Description:Tests doctypedecl with wrong field ordering. The Name "animal" occurs after the markup declarations inside the "[]".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P28/ibm28n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p28ibm28n03xml() {
    /*
        Test ID:ibm-not-wf-P28-ibm28n03.xml
        Test URI:not-wf/P28/ibm28n03.xml
        Spec Sections:2.8
        Description:Tests doctypedecl with wrong field ordering. The Name "animal" occurs after the markup declarations inside the "[]".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P28/ibm28n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p28ibm28n04xml() {
    /*
        Test ID:ibm-not-wf-P28-ibm28n04.xml
        Test URI:not-wf/P28/ibm28n04.xml
        Spec Sections:2.8
        Description:Tests doctypedecl with general entity reference.The "(ampersand)generalE" occurs in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P28/ibm28n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p28ibm28n05xml() {
    /*
        Test ID:ibm-not-wf-P28-ibm28n05.xml
        Test URI:not-wf/P28/ibm28n05.xml
        Spec Sections:2.8
        Description:Tests doctypedecl with wrong key word. A wrong key word "DOCtYPE" occurs on line 2.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P28/ibm28n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p28ibm28n06xml() {
    /*
        Test ID:ibm-not-wf-P28-ibm28n06.xml
        Test URI:not-wf/P28/ibm28n06.xml
        Spec Sections:2.8
        Description:Tests doctypedecl with mismatched brackets. The closing bracket "]" of the DTD is missing.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P28/ibm28n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p28ibm28n07xml() {
    /*
        Test ID:ibm-not-wf-P28-ibm28n07.xml
        Test URI:not-wf/P28/ibm28n07.xml
        Spec Sections:2.8
        Description:Tests doctypedecl with wrong bracket. The opening bracket "{" occurs in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P28/ibm28n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p28ibm28n08xml() {
    /*
        Test ID:ibm-not-wf-P28-ibm28n08.xml
        Test URI:not-wf/P28/ibm28n08.xml
        Spec Sections:2.8
        Description:Tests doctypedecl with wrong opening sequence. The opening sequence "(less than)?DOCTYPE" occurs in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P28/ibm28n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p28aibm28an01xml() {
    /*
        Test ID:ibm-not-wf-p28a-ibm28an01.xml
        Test URI:not-wf/p28a/ibm28an01.xml
        Spec Sections:2.8
        Description:This test violates WFC:PE Between Declarations in Production 28a. The last character of a markup declaration is not contained in the same parameter-entity text replacement.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/p28a/ibm28an01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p29ibm29n01xml() {
    /*
        Test ID:ibm-not-wf-P29-ibm29n01.xml
        Test URI:not-wf/P29/ibm29n01.xml
        Spec Sections:2.8
        Description:Tests markupdecl with an illegal markup declaration. A XMLDecl occurs inside the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P29/ibm29n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p29ibm29n02xml() {
    /*
        Test ID:ibm-not-wf-P29-ibm29n02.xml
        Test URI:not-wf/P29/ibm29n02.xml
        Spec Sections:2.8
        Description:Tests WFC "PEs in Internal Subset". A PE reference occurs inside an elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P29/ibm29n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p29ibm29n03xml() {
    /*
        Test ID:ibm-not-wf-P29-ibm29n03.xml
        Test URI:not-wf/P29/ibm29n03.xml
        Spec Sections:2.8
        Description:Tests WFC "PEs in Internal Subset". A PE reference occurs inside an ATTlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P29/ibm29n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p29ibm29n04xml() {
    /*
        Test ID:ibm-not-wf-P29-ibm29n04.xml
        Test URI:not-wf/P29/ibm29n04.xml
        Spec Sections:2.8
        Description:Tests WFC "PEs in Internal Subset". A PE reference occurs inside an EntityDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P29/ibm29n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p29ibm29n05xml() {
    /*
        Test ID:ibm-not-wf-P29-ibm29n05.xml
        Test URI:not-wf/P29/ibm29n05.xml
        Spec Sections:2.8
        Description:Tests WFC "PEs in Internal Subset". A PE reference occurs inside a PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P29/ibm29n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p29ibm29n06xml() {
    /*
        Test ID:ibm-not-wf-P29-ibm29n06.xml
        Test URI:not-wf/P29/ibm29n06.xml
        Spec Sections:2.8
        Description:Tests WFC "PEs in Internal Subset". A PE reference occurs inside a comment in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P29/ibm29n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p29ibm29n07xml() {
    /*
        Test ID:ibm-not-wf-P29-ibm29n07.xml
        Test URI:not-wf/P29/ibm29n07.xml
        Spec Sections:2.8
        Description:Tests WFC "PEs in Internal Subset". A PE reference occurs inside a NotationDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P29/ibm29n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p30ibm30n01xml() {
    /*
        Test ID:ibm-not-wf-P30-ibm30n01.xml
        Test URI:not-wf/P30/ibm30n01.xml
        Spec Sections:2.8
        Description:Tests extSubset with wrong field ordering. In the file "ibm30n01.dtd", the TextDecl occurs after the extSubsetDecl (the element declaration).
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P30/ibm30n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p31ibm31n01xml() {
    /*
        Test ID:ibm-not-wf-P31-ibm31n01.xml
        Test URI:not-wf/P31/ibm31n01.xml
        Spec Sections:2.8
        Description:Tests extSubsetDecl with an illegal field. A general entity reference occurs in file "ibm31n01.dtd".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P31/ibm31n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p32ibm32n01xml() {
    /*
        Test ID:ibm-not-wf-P32-ibm32n01.xml
        Test URI:not-wf/P32/ibm32n01.xml
        Spec Sections:2.9
        Description:Tests SDDecl with a required field missing. The leading white space is missing with the SDDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P32/ibm32n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p32ibm32n02xml() {
    /*
        Test ID:ibm-not-wf-P32-ibm32n02.xml
        Test URI:not-wf/P32/ibm32n02.xml
        Spec Sections:2.9
        Description:Tests SDDecl with a required field missing. The "=" sign is missing in the SDDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P32/ibm32n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p32ibm32n03xml() {
    /*
        Test ID:ibm-not-wf-P32-ibm32n03.xml
        Test URI:not-wf/P32/ibm32n03.xml
        Spec Sections:2.9
        Description:Tests SDDecl with wrong key word. The word "Standalone" occurs in the SDDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P32/ibm32n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p32ibm32n04xml() {
    /*
        Test ID:ibm-not-wf-P32-ibm32n04.xml
        Test URI:not-wf/P32/ibm32n04.xml
        Spec Sections:2.9
        Description:Tests SDDecl with wrong key word. The word "Yes" occurs in the SDDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P32/ibm32n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p32ibm32n05xml() {
    /*
        Test ID:ibm-not-wf-P32-ibm32n05.xml
        Test URI:not-wf/P32/ibm32n05.xml
        Spec Sections:2.9
        Description:Tests SDDecl with wrong key word. The word "YES" occurs in the SDDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P32/ibm32n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p32ibm32n06xml() {
    /*
        Test ID:ibm-not-wf-P32-ibm32n06.xml
        Test URI:not-wf/P32/ibm32n06.xml
        Spec Sections:2.9
        Description:Tests SDDecl with wrong key word. The word "No" occurs in the SDDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P32/ibm32n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p32ibm32n07xml() {
    /*
        Test ID:ibm-not-wf-P32-ibm32n07.xml
        Test URI:not-wf/P32/ibm32n07.xml
        Spec Sections:2.9
        Description:Tests SDDecl with wrong key word. The word "NO" occurs in the SDDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P32/ibm32n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p32ibm32n08xml() {
    /*
        Test ID:ibm-not-wf-P32-ibm32n08.xml
        Test URI:not-wf/P32/ibm32n08.xml
        Spec Sections:2.9
        Description:Tests SDDecl with wrong field ordering. The "=" sign occurs after the key word "yes" in the SDDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P32/ibm32n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p32ibm32n09xml() {
    /*
        Test ID:ibm-not-wf-P32-ibm32n09.xml
        Test URI:not-wf/P32/ibm32n09.xml
        Spec Sections:2.9
        Description:This is test violates WFC: Entity Declared in P68. The standalone document declaration has the value yes, BUT there is an external markup declaration of an entity (other than amp, lt, gt, apos, quot), and references to this entity appear in the document.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P32/ibm32n09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p39ibm39n01xml() {
    /*
        Test ID:ibm-not-wf-P39-ibm39n01.xml
        Test URI:not-wf/P39/ibm39n01.xml
        Spec Sections:3
        Description:Tests element with a required field missing. The ETag is missing for the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P39/ibm39n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p39ibm39n02xml() {
    /*
        Test ID:ibm-not-wf-P39-ibm39n02.xml
        Test URI:not-wf/P39/ibm39n02.xml
        Spec Sections:3
        Description:Tests element with a required field missing. The STag is missing for the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P39/ibm39n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p39ibm39n03xml() {
    /*
        Test ID:ibm-not-wf-P39-ibm39n03.xml
        Test URI:not-wf/P39/ibm39n03.xml
        Spec Sections:3
        Description:Tests element with required fields missing. Both the content and the ETag are missing in the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P39/ibm39n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p39ibm39n04xml() {
    /*
        Test ID:ibm-not-wf-P39-ibm39n04.xml
        Test URI:not-wf/P39/ibm39n04.xml
        Spec Sections:3
        Description:Tests element with required fields missing. Both the content and the STag are missing in the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P39/ibm39n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p39ibm39n05xml() {
    /*
        Test ID:ibm-not-wf-P39-ibm39n05.xml
        Test URI:not-wf/P39/ibm39n05.xml
        Spec Sections:3
        Description:Tests element with wrong field ordering. The STag and the ETag are swapped in the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P39/ibm39n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p39ibm39n06xml() {
    /*
        Test ID:ibm-not-wf-P39-ibm39n06.xml
        Test URI:not-wf/P39/ibm39n06.xml
        Spec Sections:3
        Description:Tests element with wrong field ordering. The content occurs after the ETag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P39/ibm39n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p40ibm40n01xml() {
    /*
        Test ID:ibm-not-wf-P40-ibm40n01.xml
        Test URI:not-wf/P40/ibm40n01.xml
        Spec Sections:3.1
        Description:Tests STag with a required field missing. The Name "root" is in the STag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P40/ibm40n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p40ibm40n02xml() {
    /*
        Test ID:ibm-not-wf-P40-ibm40n02.xml
        Test URI:not-wf/P40/ibm40n02.xml
        Spec Sections:3.1
        Description:Tests STag with a required field missing. The white space between the Name "root" and the attribute "attr1" is missing in the STag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P40/ibm40n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p40ibm40n03xml() {
    /*
        Test ID:ibm-not-wf-P40-ibm40n03.xml
        Test URI:not-wf/P40/ibm40n03.xml
        Spec Sections:3.1
        Description:Tests STag with wrong field ordering. The Name "root" occurs after the attribute "attr1" in the STag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P40/ibm40n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p40ibm40n04xml() {
    /*
        Test ID:ibm-not-wf-P40-ibm40n04.xml
        Test URI:not-wf/P40/ibm40n04.xml
        Spec Sections:3.1
        Description:Tests STag with a wrong opening sequence. The string "(less than)!" is used as the opening sequence for the STag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P40/ibm40n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p40ibm40n05xml() {
    /*
        Test ID:ibm-not-wf-P40-ibm40n05.xml
        Test URI:not-wf/P40/ibm40n05.xml
        Spec Sections:3.1
        Description:Tests STag with duplicate attribute names. The attribute name "attr1" occurs twice in the STag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P40/ibm40n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p41ibm41n01xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n01.xml
        Test URI:not-wf/P41/ibm41n01.xml
        Spec Sections:3.1
        Description:Tests Attribute with a required field missing. The attribute name is missing in the Attribute in the STag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p41ibm41n02xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n02.xml
        Test URI:not-wf/P41/ibm41n02.xml
        Spec Sections:3.1
        Description:Tests Attribute with a required field missing. The "=" is missing between the attribute name and the attribute value in the Attribute in the STag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p41ibm41n03xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n03.xml
        Test URI:not-wf/P41/ibm41n03.xml
        Spec Sections:3.1
        Description:Tests Attribute with a required field missing. The AttValue is missing in the Attribute in the STag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p41ibm41n04xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n04.xml
        Test URI:not-wf/P41/ibm41n04.xml
        Spec Sections:3.1
        Description:Tests Attribute with a required field missing. The Name and the "=" are missing in the Attribute in the STag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p41ibm41n05xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n05.xml
        Test URI:not-wf/P41/ibm41n05.xml
        Spec Sections:3.1
        Description:Tests Attribute with a required field missing. The "=" and the AttValue are missing in the Attribute in the STag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p41ibm41n06xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n06.xml
        Test URI:not-wf/P41/ibm41n06.xml
        Spec Sections:3.1
        Description:Tests Attribute with a required field missing. The Name and the AttValue are missing in the Attribute in the STag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p41ibm41n07xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n07.xml
        Test URI:not-wf/P41/ibm41n07.xml
        Spec Sections:3.1
        Description:Tests Attribute with wrong field ordering. The "=" occurs after the Name and the AttValue in the Attribute in the STag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p41ibm41n08xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n08.xml
        Test URI:not-wf/P41/ibm41n08.xml
        Spec Sections:3.1
        Description:Tests Attribute with wrong field ordering. The Name and the AttValue are swapped in the Attribute in the STag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p41ibm41n09xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n09.xml
        Test URI:not-wf/P41/ibm41n09.xml
        Spec Sections:3.1
        Description:Tests Attribute with wrong field ordering. The "=" occurs before the Name and the AttValue in the Attribute in the STag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p41ibm41n10xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n10.xml
        Test URI:not-wf/P41/ibm41n10.xml
        Spec Sections:3.1
        Description:Tests Attribute against WFC "no external entity references". A direct reference to the external entity "aExternal" is contained in the value of the attribute "attr1".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n10.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p41ibm41n11xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n11.xml
        Test URI:not-wf/P41/ibm41n11.xml
        Spec Sections:3.1
        Description:Tests Attribute against WFC "no external entity references". A indirect reference to the external entity "aExternal" is contained in the value of the attribute "attr1".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n11.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p41ibm41n12xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n12.xml
        Test URI:not-wf/P41/ibm41n12.xml
        Spec Sections:3.1
        Description:Tests Attribute against WFC "no external entity references". A direct reference to the external unparsed entity "aImage" is contained in the value of the attribute "attr1".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n12.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p41ibm41n13xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n13.xml
        Test URI:not-wf/P41/ibm41n13.xml
        Spec Sections:3.1
        Description:Tests Attribute against WFC "No (less than) character in Attribute Values". The character "less than" is contained in the value of the attribute "attr1".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n13.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p41ibm41n14xml() {
    /*
        Test ID:ibm-not-wf-P41-ibm41n14.xml
        Test URI:not-wf/P41/ibm41n14.xml
        Spec Sections:3.1
        Description:Tests Attribute against WFC "No (less than) in Attribute Values". The character "less than" is contained in the value of the attribute "attr1" through indirect internal entity reference.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P41/ibm41n14.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p42ibm42n01xml() {
    /*
        Test ID:ibm-not-wf-P42-ibm42n01.xml
        Test URI:not-wf/P42/ibm42n01.xml
        Spec Sections:3.1
        Description:Tests ETag with a required field missing. The Name is missing in the ETag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P42/ibm42n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p42ibm42n02xml() {
    /*
        Test ID:ibm-not-wf-P42-ibm42n02.xml
        Test URI:not-wf/P42/ibm42n02.xml
        Spec Sections:3.1
        Description:Tests ETag with a wrong beginning sequence. The string "(less than)\" is used as a beginning sequence of the ETag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P42/ibm42n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p42ibm42n03xml() {
    /*
        Test ID:ibm-not-wf-P42-ibm42n03.xml
        Test URI:not-wf/P42/ibm42n03.xml
        Spec Sections:3.1
        Description:Tests ETag with a wrong beginning sequence. The string "less than" is used as a beginning sequence of the ETag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P42/ibm42n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p42ibm42n04xml() {
    /*
        Test ID:ibm-not-wf-P42-ibm42n04.xml
        Test URI:not-wf/P42/ibm42n04.xml
        Spec Sections:3.1
        Description:Tests ETag with a wrong structure. An white space occurs between The beginning sequence and the Name of the ETag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P42/ibm42n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p42ibm42n05xml() {
    /*
        Test ID:ibm-not-wf-P42-ibm42n05.xml
        Test URI:not-wf/P42/ibm42n05.xml
        Spec Sections:3.1
        Description:Tests ETag with a wrong structure. The ETag of the element "root" contains an Attribute (attr1="any").
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P42/ibm42n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p43ibm43n01xml() {
    /*
        Test ID:ibm-not-wf-P43-ibm43n01.xml
        Test URI:not-wf/P43/ibm43n01.xml
        Spec Sections:3.1
        Description:Tests element content with a wrong option. A NotationDecl is used as the content of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P43/ibm43n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p43ibm43n02xml() {
    /*
        Test ID:ibm-not-wf-P43-ibm43n02.xml
        Test URI:not-wf/P43/ibm43n02.xml
        Spec Sections:3.1
        Description:Tests element content with a wrong option. An elementdecl is used as the content of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P43/ibm43n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p43ibm43n04xml() {
    /*
        Test ID:ibm-not-wf-P43-ibm43n04.xml
        Test URI:not-wf/P43/ibm43n04.xml
        Spec Sections:3.1
        Description:Tests element content with a wrong option. An entitydecl is used as the content of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P43/ibm43n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p43ibm43n05xml() {
    /*
        Test ID:ibm-not-wf-P43-ibm43n05.xml
        Test URI:not-wf/P43/ibm43n05.xml
        Spec Sections:3.1
        Description:Tests element content with a wrong option. An AttlistDecl is used as the content of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P43/ibm43n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p44ibm44n01xml() {
    /*
        Test ID:ibm-not-wf-P44-ibm44n01.xml
        Test URI:not-wf/P44/ibm44n01.xml
        Spec Sections:3.1
        Description:Tests EmptyElemTag with a required field missing. The Name "root" is missing in the EmptyElemTag.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P44/ibm44n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p44ibm44n02xml() {
    /*
        Test ID:ibm-not-wf-P44-ibm44n02.xml
        Test URI:not-wf/P44/ibm44n02.xml
        Spec Sections:3.1
        Description:Tests EmptyElemTag with wrong field ordering. The Attribute (attri1 = "any") occurs before the name of the element "root" in the EmptyElemTag.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P44/ibm44n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p44ibm44n03xml() {
    /*
        Test ID:ibm-not-wf-P44-ibm44n03.xml
        Test URI:not-wf/P44/ibm44n03.xml
        Spec Sections:3.1
        Description:Tests EmptyElemTag with wrong closing sequence. The string "\>" is used as the closing sequence in the EmptyElemtag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P44/ibm44n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p44ibm44n04xml() {
    /*
        Test ID:ibm-not-wf-P44-ibm44n04.xml
        Test URI:not-wf/P44/ibm44n04.xml
        Spec Sections:3.1
        Description:Tests EmptyElemTag which against the WFC "Unique Att Spec". The attribute name "attr1" occurs twice in the EmptyElemTag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P44/ibm44n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p45ibm45n01xml() {
    /*
        Test ID:ibm-not-wf-P45-ibm45n01.xml
        Test URI:not-wf/P45/ibm45n01.xml
        Spec Sections:3.2
        Description:Tests elementdecl with a required field missing. The Name is missing in the second elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P45/ibm45n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p45ibm45n02xml() {
    /*
        Test ID:ibm-not-wf-P45-ibm45n02.xml
        Test URI:not-wf/P45/ibm45n02.xml
        Spec Sections:3.2
        Description:Tests elementdecl with a required field missing. The white space is missing between "aEle" and "(#PCDATA)" in the second elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P45/ibm45n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p45ibm45n03xml() {
    /*
        Test ID:ibm-not-wf-P45-ibm45n03.xml
        Test URI:not-wf/P45/ibm45n03.xml
        Spec Sections:3.2
        Description:Tests elementdecl with a required field missing. The contentspec is missing in the second elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P45/ibm45n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p45ibm45n04xml() {
    /*
        Test ID:ibm-not-wf-P45-ibm45n04.xml
        Test URI:not-wf/P45/ibm45n04.xml
        Spec Sections:3.2
        Description:Tests elementdecl with a required field missing. The contentspec and the white space is missing in the second elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P45/ibm45n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p45ibm45n05xml() {
    /*
        Test ID:ibm-not-wf-P45-ibm45n05.xml
        Test URI:not-wf/P45/ibm45n05.xml
        Spec Sections:3.2
        Description:Tests elementdecl with a required field missing. The Name, the white space, and the contentspec are missing in the second elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P45/ibm45n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p45ibm45n06xml() {
    /*
        Test ID:ibm-not-wf-P45-ibm45n06.xml
        Test URI:not-wf/P45/ibm45n06.xml
        Spec Sections:3.2
        Description:Tests elementdecl with wrong field ordering. The Name occurs after the contentspec in the second elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P45/ibm45n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p45ibm45n07xml() {
    /*
        Test ID:ibm-not-wf-P45-ibm45n07.xml
        Test URI:not-wf/P45/ibm45n07.xml
        Spec Sections:3.2
        Description:Tests elementdecl with wrong beginning sequence. The string "(less than)ELEMENT" is used as the beginning sequence in the second elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P45/ibm45n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p45ibm45n08xml() {
    /*
        Test ID:ibm-not-wf-P45-ibm45n08.xml
        Test URI:not-wf/P45/ibm45n08.xml
        Spec Sections:3.2
        Description:Tests elementdecl with wrong key word. The string "Element" is used as the key word in the second elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P45/ibm45n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p45ibm45n09xml() {
    /*
        Test ID:ibm-not-wf-P45-ibm45n09.xml
        Test URI:not-wf/P45/ibm45n09.xml
        Spec Sections:3.2
        Description:Tests elementdecl with wrong key word. The string "element" is used as the key word in the second elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P45/ibm45n09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p46ibm46n01xml() {
    /*
        Test ID:ibm-not-wf-P46-ibm46n01.xml
        Test URI:not-wf/P46/ibm46n01.xml
        Spec Sections:3.2
        Description:Tests contentspec with wrong key word. the string "empty" is used as the key word in the contentspec of the second elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P46/ibm46n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p46ibm46n02xml() {
    /*
        Test ID:ibm-not-wf-P46-ibm46n02.xml
        Test URI:not-wf/P46/ibm46n02.xml
        Spec Sections:3.2
        Description:Tests contentspec with wrong key word. the string "Empty" is used as the key word in the contentspec of the second elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P46/ibm46n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p46ibm46n03xml() {
    /*
        Test ID:ibm-not-wf-P46-ibm46n03.xml
        Test URI:not-wf/P46/ibm46n03.xml
        Spec Sections:3.2
        Description:Tests contentspec with wrong key word. the string "Any" is used as the key word in the contentspec of the second elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P46/ibm46n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p46ibm46n04xml() {
    /*
        Test ID:ibm-not-wf-P46-ibm46n04.xml
        Test URI:not-wf/P46/ibm46n04.xml
        Spec Sections:3.2
        Description:Tests contentspec with wrong key word. the string "any" is used as the key word in the contentspec of the second elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P46/ibm46n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p46ibm46n05xml() {
    /*
        Test ID:ibm-not-wf-P46-ibm46n05.xml
        Test URI:not-wf/P46/ibm46n05.xml
        Spec Sections:3.2
        Description:Tests contentspec with a wrong option. The string "#CDATA" is used as the contentspec in the second elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P46/ibm46n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p47ibm47n01xml() {
    /*
        Test ID:ibm-not-wf-P47-ibm47n01.xml
        Test URI:not-wf/P47/ibm47n01.xml
        Spec Sections:3.2.1
        Description:Tests children with a required field missing. The "+" is used as the choice or seq field in the second elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P47/ibm47n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p47ibm47n02xml() {
    /*
        Test ID:ibm-not-wf-P47-ibm47n02.xml
        Test URI:not-wf/P47/ibm47n02.xml
        Spec Sections:3.2.1
        Description:Tests children with a required field missing. The "*" is used as the choice or seq field in the second elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P47/ibm47n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p47ibm47n03xml() {
    /*
        Test ID:ibm-not-wf-P47-ibm47n03.xml
        Test URI:not-wf/P47/ibm47n03.xml
        Spec Sections:3.2.1
        Description:Tests children with a required field missing. The "?" is used as the choice or seq field in the second elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P47/ibm47n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p47ibm47n04xml() {
    /*
        Test ID:ibm-not-wf-P47-ibm47n04.xml
        Test URI:not-wf/P47/ibm47n04.xml
        Spec Sections:3.2.1
        Description:Tests children with wrong field ordering. The "*" occurs before the seq field (a,a) in the second elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P47/ibm47n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p47ibm47n05xml() {
    /*
        Test ID:ibm-not-wf-P47-ibm47n05.xml
        Test URI:not-wf/P47/ibm47n05.xml
        Spec Sections:3.2.1
        Description:Tests children with wrong field ordering. The "+" occurs before the choice field (a|a) in the second elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P47/ibm47n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p47ibm47n06xml() {
    /*
        Test ID:ibm-not-wf-P47-ibm47n06.xml
        Test URI:not-wf/P47/ibm47n06.xml
        Spec Sections:3.2.1
        Description:Tests children with wrong key word. The "^" occurs after the seq field in the second elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P47/ibm47n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p48ibm48n01xml() {
    /*
        Test ID:ibm-not-wf-P48-ibm48n01.xml
        Test URI:not-wf/P48/ibm48n01.xml
        Spec Sections:3.2.1
        Description:Tests cp with a required fields missing. The field Name|choice|seq is missing in the second cp in the choice field in the third elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P48/ibm48n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p48ibm48n02xml() {
    /*
        Test ID:ibm-not-wf-P48-ibm48n02.xml
        Test URI:not-wf/P48/ibm48n02.xml
        Spec Sections:3.2.1
        Description:Tests cp with a required fields missing. The field Name|choice|seq is missing in the cp in the third elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P48/ibm48n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p48ibm48n03xml() {
    /*
        Test ID:ibm-not-wf-P48-ibm48n03.xml
        Test URI:not-wf/P48/ibm48n03.xml
        Spec Sections:3.2.1
        Description:Tests cp with a required fields missing. The field Name|choice|seq is missing in the first cp in the choice field in the third elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P48/ibm48n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p48ibm48n04xml() {
    /*
        Test ID:ibm-not-wf-P48-ibm48n04.xml
        Test URI:not-wf/P48/ibm48n04.xml
        Spec Sections:3.2.1
        Description:Tests cp with wrong field ordering. The "+" occurs before the seq (a,a) in the first cp in the choice field in the third elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P48/ibm48n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p48ibm48n05xml() {
    /*
        Test ID:ibm-not-wf-P48-ibm48n05.xml
        Test URI:not-wf/P48/ibm48n05.xml
        Spec Sections:3.2.1
        Description:Tests cp with wrong field ordering. The "*" occurs before the choice (a|b) in the first cp in the seq field in the third elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P48/ibm48n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p48ibm48n06xml() {
    /*
        Test ID:ibm-not-wf-P48-ibm48n06.xml
        Test URI:not-wf/P48/ibm48n06.xml
        Spec Sections:3.2.1
        Description:Tests cp with wrong field ordering. The "?" occurs before the Name "a" in the second cp in the seq field in the third elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P48/ibm48n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p48ibm48n07xml() {
    /*
        Test ID:ibm-not-wf-P48-ibm48n07.xml
        Test URI:not-wf/P48/ibm48n07.xml
        Spec Sections:3.2.1
        Description:Tests cp with wrong key word. The "^" occurs after the Name "a" in the first cp in the choice field in the third elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P48/ibm48n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p49ibm49n01xml() {
    /*
        Test ID:ibm-not-wf-P49-ibm49n01.xml
        Test URI:not-wf/P49/ibm49n01.xml
        Spec Sections:3.2.1
        Description:Tests choice with a required field missing. The two cps are missing in the choice field in the third elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P49/ibm49n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p49ibm49n02xml() {
    /*
        Test ID:ibm-not-wf-P49-ibm49n02.xml
        Test URI:not-wf/P49/ibm49n02.xml
        Spec Sections:3.2.1
        Description:Tests choice with a required field missing. The third cp is missing in the choice field in the fourth elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P49/ibm49n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p49ibm49n03xml() {
    /*
        Test ID:ibm-not-wf-P49-ibm49n03.xml
        Test URI:not-wf/P49/ibm49n03.xml
        Spec Sections:3.2.1
        Description:Tests choice with a wrong separator. The "!" is used as the separator in the choice field in the fourth elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P49/ibm49n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p49ibm49n04xml() {
    /*
        Test ID:ibm-not-wf-P49-ibm49n04.xml
        Test URI:not-wf/P49/ibm49n04.xml
        Spec Sections:3.2.1
        Description:Tests choice with a required field missing. The separator "|" is missing in the choice field (a b)+ in the fourth elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P49/ibm49n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p49ibm49n05xml() {
    /*
        Test ID:ibm-not-wf-P49-ibm49n05.xml
        Test URI:not-wf/P49/ibm49n05.xml
        Spec Sections:3.2.1
        Description:Tests choice with an extra separator. An extra "|" occurs between a and b in the choice field in the fourth elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P49/ibm49n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p49ibm49n06xml() {
    /*
        Test ID:ibm-not-wf-P49-ibm49n06.xml
        Test URI:not-wf/P49/ibm49n06.xml
        Spec Sections:3.2.1
        Description:Tests choice with a required field missing. The closing bracket ")" is missing in the choice field (a |b * in the fourth elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P49/ibm49n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p50ibm50n01xml() {
    /*
        Test ID:ibm-not-wf-P50-ibm50n01.xml
        Test URI:not-wf/P50/ibm50n01.xml
        Spec Sections:3.2.1
        Description:Tests seq with a required field missing. The two cps are missing in the seq field in the fourth elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P50/ibm50n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p50ibm50n02xml() {
    /*
        Test ID:ibm-not-wf-P50-ibm50n02.xml
        Test URI:not-wf/P50/ibm50n02.xml
        Spec Sections:3.2.1
        Description:Tests seq with a required field missing. The third cp is missing in the seq field in the fourth elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P50/ibm50n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p50ibm50n03xml() {
    /*
        Test ID:ibm-not-wf-P50-ibm50n03.xml
        Test URI:not-wf/P50/ibm50n03.xml
        Spec Sections:3.2.1
        Description:Tests seq with a wrong separator. The "|" is used as the separator between a and b in the seq field in the fourth elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P50/ibm50n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p50ibm50n04xml() {
    /*
        Test ID:ibm-not-wf-P50-ibm50n04.xml
        Test URI:not-wf/P50/ibm50n04.xml
        Spec Sections:3.2.1
        Description:Tests seq with a wrong separator. The "." is used as the separator between a and b in the seq field in the fourth elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P50/ibm50n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p50ibm50n05xml() {
    /*
        Test ID:ibm-not-wf-P50-ibm50n05.xml
        Test URI:not-wf/P50/ibm50n05.xml
        Spec Sections:3.2.1
        Description:Tests seq with an extra separator. An extra "," occurs between (a|b) and a in the seq field in the fourth elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P50/ibm50n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p50ibm50n06xml() {
    /*
        Test ID:ibm-not-wf-P50-ibm50n06.xml
        Test URI:not-wf/P50/ibm50n06.xml
        Spec Sections:3.2.1
        Description:Tests seq with a required field missing. The separator between (a|b) and (b|a) is missing in the seq field in the fourth elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P50/ibm50n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p50ibm50n07xml() {
    /*
        Test ID:ibm-not-wf-P50-ibm50n07.xml
        Test URI:not-wf/P50/ibm50n07.xml
        Spec Sections:3.2.1
        Description:Tests seq with wrong closing bracket. The "]" is used as the closing bracket in the seq field in the fourth elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P50/ibm50n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p51ibm51n01xml() {
    /*
        Test ID:ibm-not-wf-P51-ibm51n01.xml
        Test URI:not-wf/P51/ibm51n01.xml
        Spec Sections:3.2.2
        Description:Tests Mixed with a wrong key word. The string "#pcdata" is used as the key word in the Mixed field in the fourth elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P51/ibm51n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p51ibm51n02xml() {
    /*
        Test ID:ibm-not-wf-P51-ibm51n02.xml
        Test URI:not-wf/P51/ibm51n02.xml
        Spec Sections:3.2.2
        Description:Tests Mixed with wrong field ordering. The field #PCDATA does not occur as the first component in the Mixed field in the fourth elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P51/ibm51n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p51ibm51n03xml() {
    /*
        Test ID:ibm-not-wf-P51-ibm51n03.xml
        Test URI:not-wf/P51/ibm51n03.xml
        Spec Sections:3.2.2
        Description:Tests Mixed with a separator missing. The separator "|" is missing in between #PCDATA and a in the Mixed field in the fourth elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P51/ibm51n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p51ibm51n04xml() {
    /*
        Test ID:ibm-not-wf-P51-ibm51n04.xml
        Test URI:not-wf/P51/ibm51n04.xml
        Spec Sections:3.2.2
        Description:Tests Mixed with a wrong key word. The string "#CDATA" is used as the key word in the Mixed field in the fourth elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P51/ibm51n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p51ibm51n05xml() {
    /*
        Test ID:ibm-not-wf-P51-ibm51n05.xml
        Test URI:not-wf/P51/ibm51n05.xml
        Spec Sections:3.2.2
        Description:Tests Mixed with a required field missing. The "*" is missing after the ")" in the Mixed field in the fourth elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P51/ibm51n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p51ibm51n06xml() {
    /*
        Test ID:ibm-not-wf-P51-ibm51n06.xml
        Test URI:not-wf/P51/ibm51n06.xml
        Spec Sections:3.2.2
        Description:Tests Mixed with wrong closing bracket. The "]" is used as the closing bracket in the Mixed field in the fourth elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P51/ibm51n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p51ibm51n07xml() {
    /*
        Test ID:ibm-not-wf-P51-ibm51n07.xml
        Test URI:not-wf/P51/ibm51n07.xml
        Spec Sections:3.2.2
        Description:Tests Mixed with a required field missing. The closing bracket ")" is missing after (#PCDATA in the Mixed field in the fourth elementdecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P51/ibm51n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p52ibm52n01xml() {
    /*
        Test ID:ibm-not-wf-P52-ibm52n01.xml
        Test URI:not-wf/P52/ibm52n01.xml
        Spec Sections:3.3
        Description:Tests AttlistDecl with a required field missing. The Name is missing in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P52/ibm52n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p52ibm52n02xml() {
    /*
        Test ID:ibm-not-wf-P52-ibm52n02.xml
        Test URI:not-wf/P52/ibm52n02.xml
        Spec Sections:3.3
        Description:Tests AttlistDecl with a required field missing. The white space is missing between the beginning sequence and the name in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P52/ibm52n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p52ibm52n03xml() {
    /*
        Test ID:ibm-not-wf-P52-ibm52n03.xml
        Test URI:not-wf/P52/ibm52n03.xml
        Spec Sections:3.3
        Description:Tests AttlistDecl with wrong field ordering. The Name "a" occurs after the first AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P52/ibm52n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p52ibm52n04xml() {
    /*
        Test ID:ibm-not-wf-P52-ibm52n04.xml
        Test URI:not-wf/P52/ibm52n04.xml
        Spec Sections:3.3
        Description:Tests AttlistDecl with wrong key word. The string "Attlist" is used as the key word in the beginning sequence in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P52/ibm52n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p52ibm52n05xml() {
    /*
        Test ID:ibm-not-wf-P52-ibm52n05.xml
        Test URI:not-wf/P52/ibm52n05.xml
        Spec Sections:3.3
        Description:Tests AttlistDecl with a required field missing. The closing bracket "greater than" is missing in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P52/ibm52n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p52ibm52n06xml() {
    /*
        Test ID:ibm-not-wf-P52-ibm52n06.xml
        Test URI:not-wf/P52/ibm52n06.xml
        Spec Sections:3.3
        Description:Tests AttlistDecl with wrong beginning sequence. The string "(less than)ATTLIST" is used as the beginning sequence in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P52/ibm52n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p53ibm53n01xml() {
    /*
        Test ID:ibm-not-wf-P53-ibm53n01.xml
        Test URI:not-wf/P53/ibm53n01.xml
        Spec Sections:3.3
        Description:Tests AttDef with a required field missing. The DefaultDecl is missing in the AttDef for the name "attr1" in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P53/ibm53n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p53ibm53n02xml() {
    /*
        Test ID:ibm-not-wf-P53-ibm53n02.xml
        Test URI:not-wf/P53/ibm53n02.xml
        Spec Sections:3.3
        Description:Tests AttDef with a required field missing. The white space is missing between (abc|def) and "def" in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P53/ibm53n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p53ibm53n03xml() {
    /*
        Test ID:ibm-not-wf-P53-ibm53n03.xml
        Test URI:not-wf/P53/ibm53n03.xml
        Spec Sections:3.3
        Description:Tests AttDef with a required field missing. The AttType is missing for "attr1" in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P53/ibm53n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p53ibm53n04xml() {
    /*
        Test ID:ibm-not-wf-P53-ibm53n04.xml
        Test URI:not-wf/P53/ibm53n04.xml
        Spec Sections:3.3
        Description:Tests AttDef with a required field missing. The white space is missing between "attr1" and (abc|def) in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P53/ibm53n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p53ibm53n05xml() {
    /*
        Test ID:ibm-not-wf-P53-ibm53n05.xml
        Test URI:not-wf/P53/ibm53n05.xml
        Spec Sections:3.3
        Description:Tests AttDef with a required field missing. The Name is missing in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P53/ibm53n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p53ibm53n06xml() {
    /*
        Test ID:ibm-not-wf-P53-ibm53n06.xml
        Test URI:not-wf/P53/ibm53n06.xml
        Spec Sections:3.3
        Description:Tests AttDef with a required field missing. The white space before the name "attr2" is missing in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P53/ibm53n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p53ibm53n07xml() {
    /*
        Test ID:ibm-not-wf-P53-ibm53n07.xml
        Test URI:not-wf/P53/ibm53n07.xml
        Spec Sections:3.3
        Description:Tests AttDef with wrong field ordering. The Name "attr1" occurs after the AttType in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P53/ibm53n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p53ibm53n08xml() {
    /*
        Test ID:ibm-not-wf-P53-ibm53n08.xml
        Test URI:not-wf/P53/ibm53n08.xml
        Spec Sections:3.3
        Description:Tests AttDef with wrong field ordering. The Name "attr1" occurs after the AttType and "default" occurs before the AttType in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P53/ibm53n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p54ibm54n01xml() {
    /*
        Test ID:ibm-not-wf-P54-ibm54n01.xml
        Test URI:not-wf/P54/ibm54n01.xml
        Spec Sections:3.3.1
        Description:Tests AttType with a wrong option. The string "BOGUSATTR" is used as the AttType in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P54/ibm54n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p54ibm54n02xml() {
    /*
        Test ID:ibm-not-wf-P54-ibm54n02.xml
        Test URI:not-wf/P54/ibm54n02.xml
        Spec Sections:3.3.1
        Description:Tests AttType with a wrong option. The string "PCDATA" is used as the AttType in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P54/ibm54n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p55ibm55n01xml() {
    /*
        Test ID:ibm-not-wf-P55-ibm55n01.xml
        Test URI:not-wf/P55/ibm55n01.xml
        Spec Sections:3.3.1
        Description:Tests StringType with a wrong key word. The lower case string "cdata" is used as the StringType in the AttType in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P55/ibm55n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p55ibm55n02xml() {
    /*
        Test ID:ibm-not-wf-P55-ibm55n02.xml
        Test URI:not-wf/P55/ibm55n02.xml
        Spec Sections:3.3.1
        Description:Tests StringType with a wrong key word. The string "#CDATA" is used as the StringType in the AttType in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P55/ibm55n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p55ibm55n03xml() {
    /*
        Test ID:ibm-not-wf-P55-ibm55n03.xml
        Test URI:not-wf/P55/ibm55n03.xml
        Spec Sections:3.3.1
        Description:Tests StringType with a wrong key word. The string "CData" is used as the StringType in the AttType in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P55/ibm55n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p56ibm56n01xml() {
    /*
        Test ID:ibm-not-wf-P56-ibm56n01.xml
        Test URI:not-wf/P56/ibm56n01.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType with wrong key word. The type "id" is used in the TokenizedType in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P56/ibm56n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p56ibm56n02xml() {
    /*
        Test ID:ibm-not-wf-P56-ibm56n02.xml
        Test URI:not-wf/P56/ibm56n02.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType with wrong key word. The type "Idref" is used in the TokenizedType in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P56/ibm56n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p56ibm56n03xml() {
    /*
        Test ID:ibm-not-wf-P56-ibm56n03.xml
        Test URI:not-wf/P56/ibm56n03.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType with wrong key word. The type"Idrefs" is used in the TokenizedType in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P56/ibm56n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p56ibm56n04xml() {
    /*
        Test ID:ibm-not-wf-P56-ibm56n04.xml
        Test URI:not-wf/P56/ibm56n04.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType with wrong key word. The type "EntitY" is used in the TokenizedType in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P56/ibm56n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p56ibm56n05xml() {
    /*
        Test ID:ibm-not-wf-P56-ibm56n05.xml
        Test URI:not-wf/P56/ibm56n05.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType with wrong key word. The type "nmTOKEN" is used in the TokenizedType in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P56/ibm56n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p56ibm56n06xml() {
    /*
        Test ID:ibm-not-wf-P56-ibm56n06.xml
        Test URI:not-wf/P56/ibm56n06.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType with wrong key word. The type "NMtokens" is used in the TokenizedType in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P56/ibm56n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p56ibm56n07xml() {
    /*
        Test ID:ibm-not-wf-P56-ibm56n07.xml
        Test URI:not-wf/P56/ibm56n07.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType with wrong key word. The type "#ID" is used in the TokenizedType in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P56/ibm56n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p57ibm57n01xml() {
    /*
        Test ID:ibm-not-wf-P57-ibm57n01.xml
        Test URI:not-wf/P57/ibm57n01.xml
        Spec Sections:3.3.1
        Description:Tests EnumeratedType with an illegal option. The string "NMTOKEN (a|b)" is used in the EnumeratedType in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P57/ibm57n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p58ibm58n01xml() {
    /*
        Test ID:ibm-not-wf-P58-ibm58n01.xml
        Test URI:not-wf/P58/ibm58n01.xml
        Spec Sections:3.3.1
        Description:Tests NotationType with wrong key word. The lower case "notation" is used as the key word in the NotationType in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P58/ibm58n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p58ibm58n02xml() {
    /*
        Test ID:ibm-not-wf-P58-ibm58n02.xml
        Test URI:not-wf/P58/ibm58n02.xml
        Spec Sections:3.3.1
        Description:Tests NotationType with a required field missing. The beginning bracket "(" is missing in the NotationType in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P58/ibm58n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p58ibm58n03xml() {
    /*
        Test ID:ibm-not-wf-P58-ibm58n03.xml
        Test URI:not-wf/P58/ibm58n03.xml
        Spec Sections:3.3.1
        Description:Tests NotationType with a required field missing. The Name is missing in the "()" in the NotationType in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P58/ibm58n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p58ibm58n04xml() {
    /*
        Test ID:ibm-not-wf-P58-ibm58n04.xml
        Test URI:not-wf/P58/ibm58n04.xml
        Spec Sections:3.3.1
        Description:Tests NotationType with a required field missing. The closing bracket is missing in the NotationType in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P58/ibm58n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p58ibm58n05xml() {
    /*
        Test ID:ibm-not-wf-P58-ibm58n05.xml
        Test URI:not-wf/P58/ibm58n05.xml
        Spec Sections:3.3.1
        Description:Tests NotationType with wrong field ordering. The key word "NOTATION" occurs after "(this)" in the NotationType in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P58/ibm58n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p58ibm58n06xml() {
    /*
        Test ID:ibm-not-wf-P58-ibm58n06.xml
        Test URI:not-wf/P58/ibm58n06.xml
        Spec Sections:3.3.1
        Description:Tests NotationType with wrong separator. The "," is used as a separator between "this" and "that" in the NotationType in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P58/ibm58n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p58ibm58n07xml() {
    /*
        Test ID:ibm-not-wf-P58-ibm58n07.xml
        Test URI:not-wf/P58/ibm58n07.xml
        Spec Sections:3.3.1
        Description:Tests NotationType with a required field missing. The white space is missing between "NOTATION" and "(this)" in the NotationType in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P58/ibm58n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p58ibm58n08xml() {
    /*
        Test ID:ibm-not-wf-P58-ibm58n08.xml
        Test URI:not-wf/P58/ibm58n08.xml
        Spec Sections:3.3.1
        Description:Tests NotationType with extra wrong characters. The double quote character occurs after "(" and before ")" in the NotationType in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P58/ibm58n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p59ibm59n01xml() {
    /*
        Test ID:ibm-not-wf-P59-ibm59n01.xml
        Test URI:not-wf/P59/ibm59n01.xml
        Spec Sections:3.3.1
        Description:Tests Enumeration with required fields missing. The Nmtokens and "|"s are missing in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P59/ibm59n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p59ibm59n02xml() {
    /*
        Test ID:ibm-not-wf-P59-ibm59n02.xml
        Test URI:not-wf/P59/ibm59n02.xml
        Spec Sections:3.3.1
        Description:Tests Enumeration with a required field missing. The closing bracket ")" is missing in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P59/ibm59n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p59ibm59n03xml() {
    /*
        Test ID:ibm-not-wf-P59-ibm59n03.xml
        Test URI:not-wf/P59/ibm59n03.xml
        Spec Sections:3.3.1
        Description:Tests Enumeration with wrong separator. The "," is used as the separator in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P59/ibm59n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p59ibm59n04xml() {
    /*
        Test ID:ibm-not-wf-P59-ibm59n04.xml
        Test URI:not-wf/P59/ibm59n04.xml
        Spec Sections:3.3.1
        Description:Tests Enumeration with illegal presence. The double quotes occur around the Enumeration value in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P59/ibm59n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p59ibm59n05xml() {
    /*
        Test ID:ibm-not-wf-P59-ibm59n05.xml
        Test URI:not-wf/P59/ibm59n05.xml
        Spec Sections:3.3.1
        Description:Tests Enumeration with a required field missing. The white space is missing between in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P59/ibm59n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p59ibm59n06xml() {
    /*
        Test ID:ibm-not-wf-P59-ibm59n06.xml
        Test URI:not-wf/P59/ibm59n06.xml
        Spec Sections:3.3.1
        Description:Tests Enumeration with a required field missing. The beginning bracket "(" is missing in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P59/ibm59n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p60ibm60n01xml() {
    /*
        Test ID:ibm-not-wf-P60-ibm60n01.xml
        Test URI:not-wf/P60/ibm60n01.xml
        Spec Sections:3.3.2
        Description:Tests DefaultDecl with wrong key word. The string "#required" is used as the key word in the DefaultDecl in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P60/ibm60n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p60ibm60n02xml() {
    /*
        Test ID:ibm-not-wf-P60-ibm60n02.xml
        Test URI:not-wf/P60/ibm60n02.xml
        Spec Sections:3.3.2
        Description:Tests DefaultDecl with wrong key word. The string "Implied" is used as the key word in the DefaultDecl in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P60/ibm60n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p60ibm60n03xml() {
    /*
        Test ID:ibm-not-wf-P60-ibm60n03.xml
        Test URI:not-wf/P60/ibm60n03.xml
        Spec Sections:3.3.2
        Description:Tests DefaultDecl with wrong key word. The string "!IMPLIED" is used as the key word in the DefaultDecl in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P60/ibm60n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p60ibm60n04xml() {
    /*
        Test ID:ibm-not-wf-P60-ibm60n04.xml
        Test URI:not-wf/P60/ibm60n04.xml
        Spec Sections:3.3.2
        Description:Tests DefaultDecl with a required field missing. There is no attribute value specified after the key word "#FIXED" in the DefaultDecl in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P60/ibm60n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p60ibm60n05xml() {
    /*
        Test ID:ibm-not-wf-P60-ibm60n05.xml
        Test URI:not-wf/P60/ibm60n05.xml
        Spec Sections:3.3.2
        Description:Tests DefaultDecl with a required field missing. The white space is missing between the key word "#FIXED" and the attribute value in the DefaultDecl in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P60/ibm60n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p60ibm60n06xml() {
    /*
        Test ID:ibm-not-wf-P60-ibm60n06.xml
        Test URI:not-wf/P60/ibm60n06.xml
        Spec Sections:3.3.2
        Description:Tests DefaultDecl with wrong field ordering. The key word "#FIXED" occurs after the attribute value "introduction" in the DefaultDecl in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P60/ibm60n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p60ibm60n07xml() {
    /*
        Test ID:ibm-not-wf-P60-ibm60n07.xml
        Test URI:not-wf/P60/ibm60n07.xml
        Spec Sections:3.3.2
        Description:Tests DefaultDecl against WFC of P60. The text replacement of the entity "avalue" contains the "less than" character in the DefaultDecl in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P60/ibm60n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p60ibm60n08xml() {
    /*
        Test ID:ibm-not-wf-P60-ibm60n08.xml
        Test URI:not-wf/P60/ibm60n08.xml
        Spec Sections:3.3.2
        Description:Tests DefaultDecl with more than one key word. The "#REQUIRED" and the "#IMPLIED" are used as the key words in the DefaultDecl in the AttDef in the AttlistDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P60/ibm60n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p61ibm61n01xml() {
    /*
        Test ID:ibm-not-wf-P61-ibm61n01.xml
        Test URI:not-wf/P61/ibm61n01.xml
        Spec Sections:3.4
        Description:Tests conditionalSect with a wrong option. The word "NOTINCLUDE" is used as part of an option which is wrong in the coditionalSect.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P61/ibm61n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p62ibm62n01xml() {
    /*
        Test ID:ibm-not-wf-P62-ibm62n01.xml
        Test URI:not-wf/P62/ibm62n01.xml
        Spec Sections:3.4
        Description:Tests includeSect with wrong key word. The string "include" is used as a key word in the beginning sequence in the includeSect in the file ibm62n01.dtd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P62/ibm62n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p62ibm62n02xml() {
    /*
        Test ID:ibm-not-wf-P62-ibm62n02.xml
        Test URI:not-wf/P62/ibm62n02.xml
        Spec Sections:3.4
        Description:Tests includeSect with wrong beginning sequence. An extra "[" occurs in the beginning sequence in the includeSect in the file ibm62n02.dtd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P62/ibm62n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p62ibm62n03xml() {
    /*
        Test ID:ibm-not-wf-P62-ibm62n03.xml
        Test URI:not-wf/P62/ibm62n03.xml
        Spec Sections:3.4
        Description:Tests includeSect with wrong beginning sequence. A wrong character "?" occurs in the beginning sequence in the includeSect in the file ibm62n03.dtd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P62/ibm62n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p62ibm62n04xml() {
    /*
        Test ID:ibm-not-wf-P62-ibm62n04.xml
        Test URI:not-wf/P62/ibm62n04.xml
        Spec Sections:3.4
        Description:Tests includeSect with a required field missing. The key word "INCLUDE" is missing in the includeSect in the file ibm62n04.dtd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P62/ibm62n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p62ibm62n05xml() {
    /*
        Test ID:ibm-not-wf-P62-ibm62n05.xml
        Test URI:not-wf/P62/ibm62n05.xml
        Spec Sections:3.4
        Description:Tests includeSect with a required field missing. The "[" is missing after the key word "INCLUDE" in the includeSect in the file ibm62n05.dtd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P62/ibm62n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p62ibm62n06xml() {
    /*
        Test ID:ibm-not-wf-P62-ibm62n06.xml
        Test URI:not-wf/P62/ibm62n06.xml
        Spec Sections:3.4
        Description:Tests includeSect with wrong field ordering. The two external subset declarations occur before the key word "INCLUDE" in the includeSect in the file ibm62n06.dtd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P62/ibm62n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p62ibm62n07xml() {
    /*
        Test ID:ibm-not-wf-P62-ibm62n07.xml
        Test URI:not-wf/P62/ibm62n07.xml
        Spec Sections:3.4
        Description:Tests includeSect with a required field missing. The closing sequence "]](greater than)" is missing in the includeSect in the file ibm62n07.dtd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P62/ibm62n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p62ibm62n08xml() {
    /*
        Test ID:ibm-not-wf-P62-ibm62n08.xml
        Test URI:not-wf/P62/ibm62n08.xml
        Spec Sections:3.4
        Description:Tests includeSect with a required field missing. One "]" is missing in the closing sequence in the includeSect in the file ibm62n08.dtd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P62/ibm62n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p63ibm63n01xml() {
    /*
        Test ID:ibm-not-wf-P63-ibm63n01.xml
        Test URI:not-wf/P63/ibm63n01.xml
        Spec Sections:3.4
        Description:Tests ignoreSect with wrong key word. The string "ignore" is used as a key word in the beginning sequence in the ignoreSect in the file ibm63n01.dtd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P63/ibm63n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p63ibm63n02xml() {
    /*
        Test ID:ibm-not-wf-P63-ibm63n02.xml
        Test URI:not-wf/P63/ibm63n02.xml
        Spec Sections:3.4
        Description:Tests ignoreSect with wrong beginning sequence. An extra "[" occurs in the beginning sequence in the ignoreSect in the file ibm63n02.dtd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P63/ibm63n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p63ibm63n03xml() {
    /*
        Test ID:ibm-not-wf-P63-ibm63n03.xml
        Test URI:not-wf/P63/ibm63n03.xml
        Spec Sections:3.4
        Description:Tests ignoreSect with wrong beginning sequence. A wrong character "?" occurs in the beginning sequence in the ignoreSect in the file ibm63n03.dtd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P63/ibm63n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p63ibm63n04xml() {
    /*
        Test ID:ibm-not-wf-P63-ibm63n04.xml
        Test URI:not-wf/P63/ibm63n04.xml
        Spec Sections:3.4
        Description:Tests ignoreSect with a required field missing. The key word "IGNORE" is missing in the ignoreSect in the file ibm63n04.dtd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P63/ibm63n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p63ibm63n05xml() {
    /*
        Test ID:ibm-not-wf-P63-ibm63n05.xml
        Test URI:not-wf/P63/ibm63n05.xml
        Spec Sections:3.4
        Description:Tests ignoreSect with a required field missing. The "[" is missing after the key word "IGNORE" in the ignoreSect in the file ibm63n05.dtd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P63/ibm63n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p63ibm63n06xml() {
    /*
        Test ID:ibm-not-wf-P63-ibm63n06.xml
        Test URI:not-wf/P63/ibm63n06.xml
        Spec Sections:3.4
        Description:Tests includeSect with wrong field ordering. The two external subset declarations occur before the key word "IGNORE" in the ignoreSect in the file ibm63n06.dtd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P63/ibm63n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p63ibm63n07xml() {
    /*
        Test ID:ibm-not-wf-P63-ibm63n07.xml
        Test URI:not-wf/P63/ibm63n07.xml
        Spec Sections:3.4
        Description:Tests ignoreSect with a required field missing. The closing sequence "]](greater than)" is missing in the ignoreSect in the file ibm63n07.dtd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P63/ibm63n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p64ibm64n01xml() {
    /*
        Test ID:ibm-not-wf-P64-ibm64n01.xml
        Test URI:not-wf/P64/ibm64n01.xml
        Spec Sections:3.4
        Description:Tests ignoreSectContents with wrong beginning sequence. The "?" occurs in beginning sequence the ignoreSectContents in the file ibm64n01.dtd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P64/ibm64n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p64ibm64n02xml() {
    /*
        Test ID:ibm-not-wf-P64-ibm64n02.xml
        Test URI:not-wf/P64/ibm64n02.xml
        Spec Sections:3.4
        Description:Tests ignoreSectContents with a required field missing.The closing sequence is missing in the ignoreSectContents in the file ibm64n02.dtd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P64/ibm64n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p64ibm64n03xml() {
    /*
        Test ID:ibm-not-wf-P64-ibm64n03.xml
        Test URI:not-wf/P64/ibm64n03.xml
        Spec Sections:3.4
        Description:Tests ignoreSectContents with a required field missing.The beginning sequence is missing in the ignoreSectContents in the file ibm64n03.dtd.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P64/ibm64n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p65ibm65n01xml() {
    /*
        Test ID:ibm-not-wf-P65-ibm65n01.xml
        Test URI:not-wf/P65/ibm65n01.xml
        Spec Sections:3.4
        Description:Tests Ignore with illegal string included. The string "]](greater than)" is contained before "this" in the Ignore in the ignoreSectContents in the file ibm65n01.dtd
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P65/ibm65n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p65ibm65n02xml() {
    /*
        Test ID:ibm-not-wf-P65-ibm65n02.xml
        Test URI:not-wf/P65/ibm65n02.xml
        Spec Sections:3.4
        Description:Tests Ignore with illegal string included. The string "(less than)![" is contained before "this" in the Ignore in the ignoreSectContents in the file ibm65n02.dtd
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P65/ibm65n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p66ibm66n01xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n01.xml
        Test URI:not-wf/P66/ibm66n01.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#002f" is used as the referred character in the CharRef in the EntityDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p66ibm66n02xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n02.xml
        Test URI:not-wf/P66/ibm66n02.xml
        Spec Sections:4.1
        Description:Tests CharRef with the semicolon character missing. The semicolon character is missing at the end of the CharRef in the attribute value in the STag of element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p66ibm66n03xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n03.xml
        Test URI:not-wf/P66/ibm66n03.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "49" is used as the referred character in the CharRef in the EntityDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p66ibm66n04xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n04.xml
        Test URI:not-wf/P66/ibm66n04.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#5~0" is used as the referred character in the attribute value in the EmptyElemTag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p66ibm66n05xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n05.xml
        Test URI:not-wf/P66/ibm66n05.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#x002g" is used as the referred character in the CharRef in the EntityDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p66ibm66n06xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n06.xml
        Test URI:not-wf/P66/ibm66n06.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#x006G" is used as the referred character in the attribute value in the EmptyElemTag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p66ibm66n07xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n07.xml
        Test URI:not-wf/P66/ibm66n07.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#0=2f" is used as the referred character in the CharRef in the EntityDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p66ibm66n08xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n08.xml
        Test URI:not-wf/P66/ibm66n08.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#56.0" is used as the referred character in the attribute value in the EmptyElemTag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p66ibm66n09xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n09.xml
        Test URI:not-wf/P66/ibm66n09.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#x00/2f" is used as the referred character in the CharRef in the EntityDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p66ibm66n10xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n10.xml
        Test URI:not-wf/P66/ibm66n10.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#51)" is used as the referred character in the attribute value in the EmptyElemTag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n10.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p66ibm66n11xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n11.xml
        Test URI:not-wf/P66/ibm66n11.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#00 2f" is used as the referred character in the CharRef in the EntityDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n11.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p66ibm66n12xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n12.xml
        Test URI:not-wf/P66/ibm66n12.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#x0000" is used as the referred character in the attribute value in the EmptyElemTag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n12.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p66ibm66n13xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n13.xml
        Test URI:not-wf/P66/ibm66n13.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#x001f" is used as the referred character in the attribute value in the EmptyElemTag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n13.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p66ibm66n14xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n14.xml
        Test URI:not-wf/P66/ibm66n14.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#xfffe" is used as the referred character in the attribute value in the EmptyElemTag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n14.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p66ibm66n15xml() {
    /*
        Test ID:ibm-not-wf-P66-ibm66n15.xml
        Test URI:not-wf/P66/ibm66n15.xml
        Spec Sections:4.1
        Description:Tests CharRef with an illegal character referred to. The "#xffff" is used as the referred character in the attribute value in the EmptyElemTag of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P66/ibm66n15.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p68ibm68n01xml() {
    /*
        Test ID:ibm-not-wf-P68-ibm68n01.xml
        Test URI:not-wf/P68/ibm68n01.xml
        Spec Sections:4.1
        Description:Tests EntityRef with a required field missing. The Name is missing in the EntityRef in the content of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P68/ibm68n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p68ibm68n02xml() {
    /*
        Test ID:ibm-not-wf-P68-ibm68n02.xml
        Test URI:not-wf/P68/ibm68n02.xml
        Spec Sections:4.1
        Description:Tests EntityRef with a required field missing. The semicolon is missing in the EntityRef in the attribute value in the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P68/ibm68n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p68ibm68n03xml() {
    /*
        Test ID:ibm-not-wf-P68-ibm68n03.xml
        Test URI:not-wf/P68/ibm68n03.xml
        Spec Sections:4.1
        Description:Tests EntityRef with an extra white space. A white space occurs after the ampersand in the EntityRef in the content of the element "root".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P68/ibm68n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p68ibm68n04xml() {
    /*
        Test ID:ibm-not-wf-P68-ibm68n04.xml
        Test URI:not-wf/P68/ibm68n04.xml
        Spec Sections:4.1
        Description:Tests EntityRef which is against P68 WFC: Entity Declared. The name "aAa" in the EntityRef in the AttValue in the STage of the element "root" does not match the Name of any declared entity in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P68/ibm68n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p68ibm68n05xml() {
    /*
        Test ID:ibm-not-wf-P68-ibm68n05.xml
        Test URI:not-wf/P68/ibm68n05.xml
        Spec Sections:4.1
        Description:Tests EntityRef which is against P68 WFC: Entity Declared. The entity with the name "aaa" in the EntityRef in the AttValue in the STag of the element "root" is not declared.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P68/ibm68n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p68ibm68n06xml() {
    /*
        Test ID:ibm-not-wf-P68-ibm68n06.xml
        Test URI:not-wf/P68/ibm68n06.xml
        Spec Sections:4.1
        Description:Tests EntityRef which is against P68 WFC: Entity Declared. The entity with the name "aaa" in the EntityRef in the AttValue in the STag of the element "root" is externally declared, but standalone is "yes".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P68/ibm68n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p68ibm68n07xml() {
    /*
        Test ID:ibm-not-wf-P68-ibm68n07.xml
        Test URI:not-wf/P68/ibm68n07.xml
        Spec Sections:4.1
        Description:Tests EntityRef which is against P68 WFC: Entity Declared. The entity with the name "aaa" in the EntityRef in the AttValue in the STag of the element "root" is referred before declared.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P68/ibm68n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p68ibm68n08xml() {
    /*
        Test ID:ibm-not-wf-P68-ibm68n08.xml
        Test URI:not-wf/P68/ibm68n08.xml
        Spec Sections:4.1
        Description:Tests EntityRef which is against P68 WFC: Parsed Entity. The EntityRef in the AttValue in the STag of the element "root" contains the name "aImage" of an unparsed entity.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P68/ibm68n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p68ibm68n09xml() {
    /*
        Test ID:ibm-not-wf-P68-ibm68n09.xml
        Test URI:not-wf/P68/ibm68n09.xml
        Spec Sections:4.1
        Description:Tests EntityRef which is against P68 WFC: No Recursion. The recursive entity reference occurs with the entity declarations for "aaa" and "bbb" in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P68/ibm68n09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p68ibm68n10xml() {
    /*
        Test ID:ibm-not-wf-P68-ibm68n10.xml
        Test URI:not-wf/P68/ibm68n10.xml
        Spec Sections:4.1
        Description:Tests EntityRef which is against P68 WFC: No Recursion. The indirect recursive entity reference occurs with the entity declarations for "aaa", "bbb", "ccc", "ddd", and "eee" in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P68/ibm68n10.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p69ibm69n01xml() {
    /*
        Test ID:ibm-not-wf-P69-ibm69n01.xml
        Test URI:not-wf/P69/ibm69n01.xml
        Spec Sections:4.1
        Description:Tests PEReference with a required field missing. The Name "paaa" is missing in the PEReference in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P69/ibm69n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p69ibm69n02xml() {
    /*
        Test ID:ibm-not-wf-P69-ibm69n02.xml
        Test URI:not-wf/P69/ibm69n02.xml
        Spec Sections:4.1
        Description:Tests PEReference with a required field missing. The semicolon is missing in the PEReference "%paaa" in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P69/ibm69n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p69ibm69n03xml() {
    /*
        Test ID:ibm-not-wf-P69-ibm69n03.xml
        Test URI:not-wf/P69/ibm69n03.xml
        Spec Sections:4.1
        Description:Tests PEReference with an extra white space. There is an extra white space occurs before ";" in the PEReference in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P69/ibm69n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p69ibm69n04xml() {
    /*
        Test ID:ibm-not-wf-P69-ibm69n04.xml
        Test URI:not-wf/P69/ibm69n04.xml
        Spec Sections:4.1
        Description:Tests PEReference with an extra white space. There is an extra white space occurs after "%" in the PEReference in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P69/ibm69n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p69ibm69n05xml() {
    /*
        Test ID:ibm-not-wf-P69-ibm69n05.xml
        Test URI:not-wf/P69/ibm69n05.xml
        Spec Sections:4.1
        Description:Based on E29 substantial source: minutes XML-Syntax 1999-02-24 E38 in XML 1.0 Errata, this WFC does not apply to P69, but the VC Entity declared still apply. Tests PEReference which is against P69 WFC: Entity Declared. The PE with the name "paaa" is referred before declared in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P69/ibm69n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p69ibm69n06xml() {
    /*
        Test ID:ibm-not-wf-P69-ibm69n06.xml
        Test URI:not-wf/P69/ibm69n06.xml
        Spec Sections:4.1
        Description:Tests PEReference which is against P69 WFC: No Recursion. The recursive PE reference occurs with the entity declarations for "paaa" and "bbb" in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P69/ibm69n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p69ibm69n07xml() {
    /*
        Test ID:ibm-not-wf-P69-ibm69n07.xml
        Test URI:not-wf/P69/ibm69n07.xml
        Spec Sections:4.1
        Description:Tests PEReference which is against P69 WFC: No Recursion. The indirect recursive PE reference occurs with the entity declarations for "paaa", "bbb", "ccc", "ddd", and "eee" in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P69/ibm69n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p71ibm70n01xml() {
    /*
        Test ID:ibm-not-wf-P71-ibm70n01.xml
        Test URI:not-wf/P71/ibm70n01.xml
        Spec Sections:4.2
        Description:Tests
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P71/ibm70n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p71ibm71n01xml() {
    /*
        Test ID:ibm-not-wf-P71-ibm71n01.xml
        Test URI:not-wf/P71/ibm71n01.xml
        Spec Sections:4.2
        Description:Tests EntityDecl with a required field missing. The white space is missing between the beginning sequence and the Name "aaa" in the EntityDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P71/ibm71n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p71ibm71n02xml() {
    /*
        Test ID:ibm-not-wf-P71-ibm71n02.xml
        Test URI:not-wf/P71/ibm71n02.xml
        Spec Sections:4.2
        Description:Tests EntityDecl with a required field missing. The white space is missing between the Name "aaa" and the EntityDef "aString" in the EntityDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P71/ibm71n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p71ibm71n03xml() {
    /*
        Test ID:ibm-not-wf-P71-ibm71n03.xml
        Test URI:not-wf/P71/ibm71n03.xml
        Spec Sections:4.2
        Description:Tests EntityDecl with a required field missing. The EntityDef is missing in the EntityDecl with the Name "aaa" in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P71/ibm71n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p71ibm71n04xml() {
    /*
        Test ID:ibm-not-wf-P71-ibm71n04.xml
        Test URI:not-wf/P71/ibm71n04.xml
        Spec Sections:4.2
        Description:Tests EntityDecl with a required field missing. The Name is missing in the EntityDecl with the EntityDef "aString" in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P71/ibm71n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p71ibm71n05xml() {
    /*
        Test ID:ibm-not-wf-P71-ibm71n05.xml
        Test URI:not-wf/P71/ibm71n05.xml
        Spec Sections:4.2
        Description:Tests EntityDecl with wrong ordering. The Name "aaa" occurs after the EntityDef in the EntityDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P71/ibm71n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p71ibm71n06xml() {
    /*
        Test ID:ibm-not-wf-P71-ibm71n06.xml
        Test URI:not-wf/P71/ibm71n06.xml
        Spec Sections:4.2
        Description:Tests EntityDecl with wrong key word. The string "entity" is used as the key word in the beginning sequence in the EntityDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P71/ibm71n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p71ibm71n07xml() {
    /*
        Test ID:ibm-not-wf-P71-ibm71n07.xml
        Test URI:not-wf/P71/ibm71n07.xml
        Spec Sections:4.2
        Description:Tests EntityDecl with a required field missing. The closing bracket (greater than) is missing in the EntityDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P71/ibm71n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p71ibm71n08xml() {
    /*
        Test ID:ibm-not-wf-P71-ibm71n08.xml
        Test URI:not-wf/P71/ibm71n08.xml
        Spec Sections:4.2
        Description:Tests EntityDecl with a required field missing. The exclamation mark is missing in the beginning sequence in the EntityDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P71/ibm71n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p72ibm72n01xml() {
    /*
        Test ID:ibm-not-wf-P72-ibm72n01.xml
        Test URI:not-wf/P72/ibm72n01.xml
        Spec Sections:4.2
        Description:Tests PEdecl with a required field missing. The white space is missing between the beginning sequence and the "%" in the PEDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P72/ibm72n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p72ibm72n02xml() {
    /*
        Test ID:ibm-not-wf-P72-ibm72n02.xml
        Test URI:not-wf/P72/ibm72n02.xml
        Spec Sections:4.2
        Description:Tests PEdecl with a required field missing. The Name is missing in the PEDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P72/ibm72n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p72ibm72n03xml() {
    /*
        Test ID:ibm-not-wf-P72-ibm72n03.xml
        Test URI:not-wf/P72/ibm72n03.xml
        Spec Sections:4.2
        Description:Tests PEdecl with a required field missing. The white space is missing between the Name and the PEDef in the PEDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P72/ibm72n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p72ibm72n04xml() {
    /*
        Test ID:ibm-not-wf-P72-ibm72n04.xml
        Test URI:not-wf/P72/ibm72n04.xml
        Spec Sections:4.2
        Description:Tests PEdecl with a required field missing. The PEDef is missing after the Name "paaa" in the PEDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P72/ibm72n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p72ibm72n05xml() {
    /*
        Test ID:ibm-not-wf-P72-ibm72n05.xml
        Test URI:not-wf/P72/ibm72n05.xml
        Spec Sections:4.2
        Description:Tests PEdecl with wrong field ordering. The Name "paaa" occurs after the PEDef in the PEDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P72/ibm72n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p72ibm72n06xml() {
    /*
        Test ID:ibm-not-wf-P72-ibm72n06.xml
        Test URI:not-wf/P72/ibm72n06.xml
        Spec Sections:4.2
        Description:Tests PEdecl with wrong field ordering. The "%" and the Name "paaa" occurs after the PEDef in the PEDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P72/ibm72n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p72ibm72n07xml() {
    /*
        Test ID:ibm-not-wf-P72-ibm72n07.xml
        Test URI:not-wf/P72/ibm72n07.xml
        Spec Sections:4.2
        Description:Tests PEdecl with wrong key word. The string "entity" is used as the key word in the beginning sequence in the PEDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P72/ibm72n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p72ibm72n08xml() {
    /*
        Test ID:ibm-not-wf-P72-ibm72n08.xml
        Test URI:not-wf/P72/ibm72n08.xml
        Spec Sections:4.2
        Description:Tests PEdecl with a required field missing. The closing bracket (greater than) is missing in the PEDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P72/ibm72n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p72ibm72n09xml() {
    /*
        Test ID:ibm-not-wf-P72-ibm72n09.xml
        Test URI:not-wf/P72/ibm72n09.xml
        Spec Sections:4.2
        Description:Tests PEdecl with wrong closing sequence. The string "!(greater than)" is used as the closing sequence in the PEDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P72/ibm72n09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p73ibm73n01xml() {
    /*
        Test ID:ibm-not-wf-P73-ibm73n01.xml
        Test URI:not-wf/P73/ibm73n01.xml
        Spec Sections:4.2
        Description:Tests EntityDef with wrong field ordering. The NDataDecl "NDATA JPGformat" occurs before the ExternalID in the EntityDef in the EntityDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P73/ibm73n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p73ibm73n03xml() {
    /*
        Test ID:ibm-not-wf-P73-ibm73n03.xml
        Test URI:not-wf/P73/ibm73n03.xml
        Spec Sections:4.2
        Description:Tests EntityDef with a required field missing. The ExternalID is missing before the NDataDecl in the EntityDef in the EntityDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P73/ibm73n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p74ibm74n01xml() {
    /*
        Test ID:ibm-not-wf-P74-ibm74n01.xml
        Test URI:not-wf/P74/ibm74n01.xml
        Spec Sections:4.2
        Description:Tests PEDef with extra fields. The NDataDecl occurs after the ExternalID in the PEDef in the PEDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P74/ibm74n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p75ibm75n01xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n01.xml
        Test URI:not-wf/P75/ibm75n01.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with wrong key word. The string "system" is used as the key word in the ExternalID in the EntityDef in the EntityDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p75ibm75n02xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n02.xml
        Test URI:not-wf/P75/ibm75n02.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with wrong key word. The string "public" is used as the key word in the ExternalID in the doctypedecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p75ibm75n03xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n03.xml
        Test URI:not-wf/P75/ibm75n03.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with wrong key word. The string "Public" is used as the key word in the ExternalID in the doctypedecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p75ibm75n04xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n04.xml
        Test URI:not-wf/P75/ibm75n04.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with wrong field ordering. The key word "PUBLIC" occurs after the PublicLiteral and the SystemLiteral in the ExternalID in the doctypedecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p75ibm75n05xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n05.xml
        Test URI:not-wf/P75/ibm75n05.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with a required field missing. The white space between "SYSTEM" and the Systemliteral is missing in the ExternalID in the EntityDef in the EntityDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p75ibm75n06xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n06.xml
        Test URI:not-wf/P75/ibm75n06.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with a required field missing. The Systemliteral is missing after "SYSTEM" in the ExternalID in the EntityDef in the EntityDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p75ibm75n07xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n07.xml
        Test URI:not-wf/P75/ibm75n07.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with a required field missing. The white space between the PublicLiteral and the Systemliteral is missing in the ExternalID in the doctypedecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p75ibm75n08xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n08.xml
        Test URI:not-wf/P75/ibm75n08.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with a required field missing. The key word "PUBLIC" is missing in the ExternalID in the doctypedecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p75ibm75n09xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n09.xml
        Test URI:not-wf/P75/ibm75n09.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with a required field missing. The white space between "PUBLIC" and the PublicLiteral is missing in the ExternalID in the doctypedecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p75ibm75n10xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n10.xml
        Test URI:not-wf/P75/ibm75n10.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with a required field missing. The PublicLiteral is missing in the ExternalID in the doctypedecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n10.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p75ibm75n11xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n11.xml
        Test URI:not-wf/P75/ibm75n11.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with a required field missing. The PublicLiteral is missing in the ExternalID in the doctypedecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n11.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p75ibm75n12xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n12.xml
        Test URI:not-wf/P75/ibm75n12.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with a required field missing. The SystemLiteral is missing in the ExternalID in the doctypedecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n12.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p75ibm75n13xml() {
    /*
        Test ID:ibm-not-wf-P75-ibm75n13.xml
        Test URI:not-wf/P75/ibm75n13.xml
        Spec Sections:4.2.2
        Description:Tests ExternalID with wrong field ordering. The key word "PUBLIC" occurs after the PublicLiteral in the ExternalID in the doctypedecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P75/ibm75n13.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p76ibm76n01xml() {
    /*
        Test ID:ibm-not-wf-P76-ibm76n01.xml
        Test URI:not-wf/P76/ibm76n01.xml
        Spec Sections:4.2.2
        Description:Tests NDataDecl with wrong key word. The string "ndata" is used as the key word in the NDataDecl in the EntityDef in the GEDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P76/ibm76n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p76ibm76n02xml() {
    /*
        Test ID:ibm-not-wf-P76-ibm76n02.xml
        Test URI:not-wf/P76/ibm76n02.xml
        Spec Sections:4.2.2
        Description:Tests NDataDecl with wrong key word. The string "NData" is used as the key word in the NDataDecl in the EntityDef in the GEDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P76/ibm76n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p76ibm76n03xml() {
    /*
        Test ID:ibm-not-wf-P76-ibm76n03.xml
        Test URI:not-wf/P76/ibm76n03.xml
        Spec Sections:4.2.2
        Description:Tests NDataDecl with a required field missing. The leading white space is missing in the NDataDecl in the EntityDef in the GEDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P76/ibm76n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p76ibm76n04xml() {
    /*
        Test ID:ibm-not-wf-P76-ibm76n04.xml
        Test URI:not-wf/P76/ibm76n04.xml
        Spec Sections:4.2.2
        Description:Tests NDataDecl with a required field missing. The key word "NDATA" is missing in the NDataDecl in the EntityDef in the GEDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P76/ibm76n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p76ibm76n05xml() {
    /*
        Test ID:ibm-not-wf-P76-ibm76n05.xml
        Test URI:not-wf/P76/ibm76n05.xml
        Spec Sections:4.2.2
        Description:Tests NDataDecl with a required field missing. The Name after the key word "NDATA" is missing in the NDataDecl in the EntityDef in the GEDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P76/ibm76n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p76ibm76n06xml() {
    /*
        Test ID:ibm-not-wf-P76-ibm76n06.xml
        Test URI:not-wf/P76/ibm76n06.xml
        Spec Sections:4.2.2
        Description:Tests NDataDecl with a required field missing. The white space between "NDATA" and the Name is missing in the NDataDecl in the EntityDef in the GEDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P76/ibm76n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p76ibm76n07xml() {
    /*
        Test ID:ibm-not-wf-P76-ibm76n07.xml
        Test URI:not-wf/P76/ibm76n07.xml
        Spec Sections:4.2.2
        Description:Tests NDataDecl with wrong field ordering. The key word "NDATA" occurs after the Name in the NDataDecl in the EntityDef in the GEDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P76/ibm76n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p77ibm77n01xml() {
    /*
        Test ID:ibm-not-wf-P77-ibm77n01.xml
        Test URI:not-wf/P77/ibm77n01.xml
        Spec Sections:4.3.1
        Description:Tests TextDecl with wrong field ordering. The VersionInfo occurs after the EncodingDecl in the TextDecl in the file "ibm77n01.ent".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P77/ibm77n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p77ibm77n02xml() {
    /*
        Test ID:ibm-not-wf-P77-ibm77n02.xml
        Test URI:not-wf/P77/ibm77n02.xml
        Spec Sections:4.3.1
        Description:Tests TextDecl with wrong key word. The string "XML" is used in the beginning sequence in the TextDecl in the file "ibm77n02.ent".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P77/ibm77n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p77ibm77n03xml() {
    /*
        Test ID:ibm-not-wf-P77-ibm77n03.xml
        Test URI:not-wf/P77/ibm77n03.xml
        Spec Sections:4.3.1
        Description:Tests TextDecl with wrong closing sequence. The character "greater than" is used as the closing sequence in the TextDecl in the file "ibm77n03.ent".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P77/ibm77n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p77ibm77n04xml() {
    /*
        Test ID:ibm-not-wf-P77-ibm77n04.xml
        Test URI:not-wf/P77/ibm77n04.xml
        Spec Sections:4.3.1
        Description:Tests TextDecl with a required field missing. The closing sequence is missing in the TextDecl in the file "ibm77n04.ent".
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P77/ibm77n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p78ibm78n01xml() {
    /*
        Test ID:ibm-not-wf-P78-ibm78n01.xml
        Test URI:not-wf/P78/ibm78n01.xml
        Spec Sections:4.3.2
        Description:Tests extParsedEnt with wrong field ordering. The TextDecl occurs after the content in the file ibm78n01.ent.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P78/ibm78n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p78ibm78n02xml() {
    /*
        Test ID:ibm-not-wf-P78-ibm78n02.xml
        Test URI:not-wf/P78/ibm78n02.xml
        Spec Sections:4.3.2
        Description:Tests extParsedEnt with extra field. A blank line occurs before the TextDecl in the file ibm78n02.ent.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P78/ibm78n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p79ibm79n01xml() {
    /*
        Test ID:ibm-not-wf-P79-ibm79n01.xml
        Test URI:not-wf/P79/ibm79n01.xml
        Spec Sections:4.3.2
        Description:Tests extPE with wrong field ordering. The TextDecl occurs after the extSubsetDecl (the white space and the comment) in the file ibm79n01.ent.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P79/ibm79n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p79ibm79n02xml() {
    /*
        Test ID:ibm-not-wf-P79-ibm79n02.xml
        Test URI:not-wf/P79/ibm79n02.xml
        Spec Sections:4.3.2
        Description:Tests extPE with extra field. A blank line occurs before the TextDecl in the file ibm78n02.ent.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P79/ibm79n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p80ibm80n01xml() {
    /*
        Test ID:ibm-not-wf-P80-ibm80n01.xml
        Test URI:not-wf/P80/ibm80n01.xml
        Spec Sections:4.3.3
        Description:Tests EncodingDecl with a required field missing. The leading white space is missing in the EncodingDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P80/ibm80n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p80ibm80n02xml() {
    /*
        Test ID:ibm-not-wf-P80-ibm80n02.xml
        Test URI:not-wf/P80/ibm80n02.xml
        Spec Sections:4.3.3
        Description:Tests EncodingDecl with a required field missing. The "=" sign is missing in the EncodingDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P80/ibm80n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p80ibm80n03xml() {
    /*
        Test ID:ibm-not-wf-P80-ibm80n03.xml
        Test URI:not-wf/P80/ibm80n03.xml
        Spec Sections:4.3.3
        Description:Tests EncodingDecl with a required field missing. The double quoted EncName are missing in the EncodingDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P80/ibm80n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p80ibm80n04xml() {
    /*
        Test ID:ibm-not-wf-P80-ibm80n04.xml
        Test URI:not-wf/P80/ibm80n04.xml
        Spec Sections:4.3.3
        Description:Tests EncodingDecl with wrong field ordering. The string "encoding=" occurs after the double quoted EncName in the EncodingDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P80/ibm80n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p80ibm80n05xml() {
    /*
        Test ID:ibm-not-wf-P80-ibm80n05.xml
        Test URI:not-wf/P80/ibm80n05.xml
        Spec Sections:4.3.3
        Description:Tests EncodingDecl with wrong field ordering. The "encoding" occurs after the double quoted EncName in the EncodingDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P80/ibm80n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p80ibm80n06xml() {
    /*
        Test ID:ibm-not-wf-P80-ibm80n06.xml
        Test URI:not-wf/P80/ibm80n06.xml
        Spec Sections:4.3.3
        Description:Tests EncodingDecl with wrong key word. The string "Encoding" is used as the key word in the EncodingDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P80/ibm80n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p81ibm81n01xml() {
    /*
        Test ID:ibm-not-wf-P81-ibm81n01.xml
        Test URI:not-wf/P81/ibm81n01.xml
        Spec Sections:4.3.3
        Description:Tests EncName with an illegal character. The "_" is used as the first character in the EncName in the EncodingDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P81/ibm81n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p81ibm81n02xml() {
    /*
        Test ID:ibm-not-wf-P81-ibm81n02.xml
        Test URI:not-wf/P81/ibm81n02.xml
        Spec Sections:4.3.3
        Description:Tests EncName with an illegal character. The "-" is used as the first character in the EncName in the EncodingDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P81/ibm81n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p81ibm81n03xml() {
    /*
        Test ID:ibm-not-wf-P81-ibm81n03.xml
        Test URI:not-wf/P81/ibm81n03.xml
        Spec Sections:4.3.3
        Description:Tests EncName with an illegal character. The "." is used as the first character in the EncName in the EncodingDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P81/ibm81n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p81ibm81n04xml() {
    /*
        Test ID:ibm-not-wf-P81-ibm81n04.xml
        Test URI:not-wf/P81/ibm81n04.xml
        Spec Sections:4.3.3
        Description:Tests EncName with illegal characters. The "8-" is used as the initial characters in the EncName in the EncodingDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P81/ibm81n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p81ibm81n05xml() {
    /*
        Test ID:ibm-not-wf-P81-ibm81n05.xml
        Test URI:not-wf/P81/ibm81n05.xml
        Spec Sections:4.3.3
        Description:Tests EncName with an illegal character. The "~" is used as one character in the EncName in the EncodingDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P81/ibm81n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p81ibm81n06xml() {
    /*
        Test ID:ibm-not-wf-P81-ibm81n06.xml
        Test URI:not-wf/P81/ibm81n06.xml
        Spec Sections:4.3.3
        Description:Tests EncName with an illegal character. The "#" is used as one character in the EncName in the EncodingDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P81/ibm81n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p81ibm81n07xml() {
    /*
        Test ID:ibm-not-wf-P81-ibm81n07.xml
        Test URI:not-wf/P81/ibm81n07.xml
        Spec Sections:4.3.3
        Description:Tests EncName with an illegal character. The ":" is used as one character in the EncName in the EncodingDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P81/ibm81n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p81ibm81n08xml() {
    /*
        Test ID:ibm-not-wf-P81-ibm81n08.xml
        Test URI:not-wf/P81/ibm81n08.xml
        Spec Sections:4.3.3
        Description:Tests EncName with an illegal character. The "/" is used as one character in the EncName in the EncodingDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P81/ibm81n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p81ibm81n09xml() {
    /*
        Test ID:ibm-not-wf-P81-ibm81n09.xml
        Test URI:not-wf/P81/ibm81n09.xml
        Spec Sections:4.3.3
        Description:Tests EncName with an illegal character. The ";" is used as one character in the EncName in the EncodingDecl in the XMLDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P81/ibm81n09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p82ibm82n01xml() {
    /*
        Test ID:ibm-not-wf-P82-ibm82n01.xml
        Test URI:not-wf/P82/ibm82n01.xml
        Spec Sections:4.7
        Description:Tests NotationDecl with a required field missing. The white space after the beginning sequence of the NotationDecl is missing in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P82/ibm82n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p82ibm82n02xml() {
    /*
        Test ID:ibm-not-wf-P82-ibm82n02.xml
        Test URI:not-wf/P82/ibm82n02.xml
        Spec Sections:4.7
        Description:Tests NotationDecl with a required field missing. The Name in the NotationDecl is missing in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P82/ibm82n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p82ibm82n03xml() {
    /*
        Test ID:ibm-not-wf-P82-ibm82n03.xml
        Test URI:not-wf/P82/ibm82n03.xml
        Spec Sections:4.7
        Description:Tests NotationDecl with a required field missing. The externalID or the PublicID is missing in the NotationDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P82/ibm82n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p82ibm82n04xml() {
    /*
        Test ID:ibm-not-wf-P82-ibm82n04.xml
        Test URI:not-wf/P82/ibm82n04.xml
        Spec Sections:4.7
        Description:Tests NotationDecl with wrong field ordering. The Name occurs after the "SYSTEM" and the externalID in the NotationDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P82/ibm82n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p82ibm82n05xml() {
    /*
        Test ID:ibm-not-wf-P82-ibm82n05.xml
        Test URI:not-wf/P82/ibm82n05.xml
        Spec Sections:4.7
        Description:Tests NotationDecl with wrong key word. The string "notation" is used as a key word in the NotationDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P82/ibm82n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p82ibm82n06xml() {
    /*
        Test ID:ibm-not-wf-P82-ibm82n06.xml
        Test URI:not-wf/P82/ibm82n06.xml
        Spec Sections:4.7
        Description:Tests NotationDecl with a required field missing. The closing bracket (the greater than character) is missing in the NotationDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P82/ibm82n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p82ibm82n07xml() {
    /*
        Test ID:ibm-not-wf-P82-ibm82n07.xml
        Test URI:not-wf/P82/ibm82n07.xml
        Spec Sections:4.7
        Description:Tests NotationDecl with wrong beginning sequence. The "!" is missing in the beginning sequence in the NotationDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P82/ibm82n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p82ibm82n08xml() {
    /*
        Test ID:ibm-not-wf-P82-ibm82n08.xml
        Test URI:not-wf/P82/ibm82n08.xml
        Spec Sections:4.7
        Description:Tests NotationDecl with wrong closing sequence. The extra "!" occurs in the closing sequence in the NotationDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P82/ibm82n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p83ibm83n01xml() {
    /*
        Test ID:ibm-not-wf-P83-ibm83n01.xml
        Test URI:not-wf/P83/ibm83n01.xml
        Spec Sections:4.7
        Description:Tests PublicID with wrong key word. The string "public" is used as the key word in the PublicID in the NotationDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P83/ibm83n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p83ibm83n02xml() {
    /*
        Test ID:ibm-not-wf-P83-ibm83n02.xml
        Test URI:not-wf/P83/ibm83n02.xml
        Spec Sections:4.7
        Description:Tests PublicID with wrong key word. The string "Public" is used as the key word in the PublicID in the NotationDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P83/ibm83n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p83ibm83n03xml() {
    /*
        Test ID:ibm-not-wf-P83-ibm83n03.xml
        Test URI:not-wf/P83/ibm83n03.xml
        Spec Sections:4.7
        Description:Tests PublicID with a required field missing. The key word "PUBLIC" is missing in the PublicID in the NotationDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P83/ibm83n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p83ibm83n04xml() {
    /*
        Test ID:ibm-not-wf-P83-ibm83n04.xml
        Test URI:not-wf/P83/ibm83n04.xml
        Spec Sections:4.7
        Description:Tests PublicID with a required field missing. The white space between the "PUBLIC" and the PubidLiteral is missing in the PublicID in the NotationDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P83/ibm83n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p83ibm83n05xml() {
    /*
        Test ID:ibm-not-wf-P83-ibm83n05.xml
        Test URI:not-wf/P83/ibm83n05.xml
        Spec Sections:4.7
        Description:Tests PublicID with a required field missing. The PubidLiteral is missing in the PublicID in the NotationDecl in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P83/ibm83n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p83ibm83n06xml() {
    /*
        Test ID:ibm-not-wf-P83-ibm83n06.xml
        Test URI:not-wf/P83/ibm83n06.xml
        Spec Sections:4.7
        Description:Tests PublicID with wrong field ordering. The key word "PUBLIC" occurs after the PubidLiteral in the PublicID in the NotationDecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P83/ibm83n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n01xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n01.xml
        Test URI:not-wf/P85/ibm85n01.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x00D7 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n02xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n02.xml
        Test URI:not-wf/P85/ibm85n02.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x00F7 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n03xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n03.xml
        Test URI:not-wf/P85/ibm85n03.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0132 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n04xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n04.xml
        Test URI:not-wf/P85/ibm85n04.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0133 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n05xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n05.xml
        Test URI:not-wf/P85/ibm85n05.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x013F occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n06xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n06.xml
        Test URI:not-wf/P85/ibm85n06.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0140 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n07xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n07.xml
        Test URI:not-wf/P85/ibm85n07.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0149 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n08xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n08.xml
        Test URI:not-wf/P85/ibm85n08.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x017F occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n09xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n09.xml
        Test URI:not-wf/P85/ibm85n09.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x01c4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n10xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n10.xml
        Test URI:not-wf/P85/ibm85n10.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x01CC occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n10.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n100xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n100.xml
        Test URI:not-wf/P85/ibm85n100.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0BB6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n100.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n101xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n101.xml
        Test URI:not-wf/P85/ibm85n101.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0BBA occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n101.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n102xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n102.xml
        Test URI:not-wf/P85/ibm85n102.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0C0D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n102.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n103xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n103.xml
        Test URI:not-wf/P85/ibm85n103.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0C11 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n103.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n104xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n104.xml
        Test URI:not-wf/P85/ibm85n104.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0C29 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n104.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n105xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n105.xml
        Test URI:not-wf/P85/ibm85n105.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0C34 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n105.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n106xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n106.xml
        Test URI:not-wf/P85/ibm85n106.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0C5F occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n106.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n107xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n107.xml
        Test URI:not-wf/P85/ibm85n107.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0C62 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n107.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n108xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n108.xml
        Test URI:not-wf/P85/ibm85n108.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0C8D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n108.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n109xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n109.xml
        Test URI:not-wf/P85/ibm85n109.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0C91 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n109.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n11xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n11.xml
        Test URI:not-wf/P85/ibm85n11.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x01F1 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n11.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n110xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n110.xml
        Test URI:not-wf/P85/ibm85n110.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0CA9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n110.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n111xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n111.xml
        Test URI:not-wf/P85/ibm85n111.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0CB4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n111.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n112xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n112.xml
        Test URI:not-wf/P85/ibm85n112.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0CBA occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n112.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n113xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n113.xml
        Test URI:not-wf/P85/ibm85n113.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0CDF occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n113.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n114xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n114.xml
        Test URI:not-wf/P85/ibm85n114.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0CE2 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n114.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n115xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n115.xml
        Test URI:not-wf/P85/ibm85n115.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0D0D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n115.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n116xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n116.xml
        Test URI:not-wf/P85/ibm85n116.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0D11 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n116.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n117xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n117.xml
        Test URI:not-wf/P85/ibm85n117.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0D29 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n117.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n118xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n118.xml
        Test URI:not-wf/P85/ibm85n118.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0D3A occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n118.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n119xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n119.xml
        Test URI:not-wf/P85/ibm85n119.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0D62 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n119.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n12xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n12.xml
        Test URI:not-wf/P85/ibm85n12.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x01F3 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n12.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n120xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n120.xml
        Test URI:not-wf/P85/ibm85n120.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0E2F occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n120.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n121xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n121.xml
        Test URI:not-wf/P85/ibm85n121.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0E31 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n121.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n122xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n122.xml
        Test URI:not-wf/P85/ibm85n122.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0E34 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n122.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n123xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n123.xml
        Test URI:not-wf/P85/ibm85n123.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0E46 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n123.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n124xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n124.xml
        Test URI:not-wf/P85/ibm85n124.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0E83 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n124.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n125xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n125.xml
        Test URI:not-wf/P85/ibm85n125.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0E85 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n125.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n126xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n126.xml
        Test URI:not-wf/P85/ibm85n126.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0E89 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n126.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n127xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n127.xml
        Test URI:not-wf/P85/ibm85n127.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0E8B occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n127.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n128xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n128.xml
        Test URI:not-wf/P85/ibm85n128.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0E8E occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n128.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n129xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n129.xml
        Test URI:not-wf/P85/ibm85n129.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0E98 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n129.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n13xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n13.xml
        Test URI:not-wf/P85/ibm85n13.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x01F6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n13.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n130xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n130.xml
        Test URI:not-wf/P85/ibm85n130.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0EA0 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n130.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n131xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n131.xml
        Test URI:not-wf/P85/ibm85n131.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0EA4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n131.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n132xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n132.xml
        Test URI:not-wf/P85/ibm85n132.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0EA6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n132.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n133xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n133.xml
        Test URI:not-wf/P85/ibm85n133.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0EA8 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n133.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n134xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n134.xml
        Test URI:not-wf/P85/ibm85n134.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0EAC occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n134.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n135xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n135.xml
        Test URI:not-wf/P85/ibm85n135.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0EAF occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n135.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n136xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n136.xml
        Test URI:not-wf/P85/ibm85n136.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0EB1 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n136.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n137xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n137.xml
        Test URI:not-wf/P85/ibm85n137.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0EB4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n137.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n138xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n138.xml
        Test URI:not-wf/P85/ibm85n138.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0EBE occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n138.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n139xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n139.xml
        Test URI:not-wf/P85/ibm85n139.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0EC5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n139.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n14xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n14.xml
        Test URI:not-wf/P85/ibm85n14.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x01F9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n14.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n140xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n140.xml
        Test URI:not-wf/P85/ibm85n140.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0F48 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n140.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n141xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n141.xml
        Test URI:not-wf/P85/ibm85n141.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0F6A occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n141.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n142xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n142.xml
        Test URI:not-wf/P85/ibm85n142.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x10C6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n142.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n143xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n143.xml
        Test URI:not-wf/P85/ibm85n143.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x10F7 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n143.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n144xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n144.xml
        Test URI:not-wf/P85/ibm85n144.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1011 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n144.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n145xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n145.xml
        Test URI:not-wf/P85/ibm85n145.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1104 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n145.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n146xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n146.xml
        Test URI:not-wf/P85/ibm85n146.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1108 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n146.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n147xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n147.xml
        Test URI:not-wf/P85/ibm85n147.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x110A occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n147.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n148xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n148.xml
        Test URI:not-wf/P85/ibm85n148.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x110D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n148.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n149xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n149.xml
        Test URI:not-wf/P85/ibm85n149.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x113B occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n149.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n15xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n15.xml
        Test URI:not-wf/P85/ibm85n15.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x01F9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n15.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n150xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n150.xml
        Test URI:not-wf/P85/ibm85n150.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x113F occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n150.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n151xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n151.xml
        Test URI:not-wf/P85/ibm85n151.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1141 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n151.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n152xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n152.xml
        Test URI:not-wf/P85/ibm85n152.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x114D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n152.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n153xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n153.xml
        Test URI:not-wf/P85/ibm85n153.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x114f occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n153.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n154xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n154.xml
        Test URI:not-wf/P85/ibm85n154.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1151 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n154.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n155xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n155.xml
        Test URI:not-wf/P85/ibm85n155.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1156 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n155.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n156xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n156.xml
        Test URI:not-wf/P85/ibm85n156.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x115A occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n156.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n157xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n157.xml
        Test URI:not-wf/P85/ibm85n157.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1162 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n157.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n158xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n158.xml
        Test URI:not-wf/P85/ibm85n158.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1164 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n158.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n159xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n159.xml
        Test URI:not-wf/P85/ibm85n159.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1166 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n159.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n16xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n16.xml
        Test URI:not-wf/P85/ibm85n16.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0230 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n16.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n160xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n160.xml
        Test URI:not-wf/P85/ibm85n160.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x116B occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n160.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n161xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n161.xml
        Test URI:not-wf/P85/ibm85n161.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x116F occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n161.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n162xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n162.xml
        Test URI:not-wf/P85/ibm85n162.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1174 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n162.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n163xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n163.xml
        Test URI:not-wf/P85/ibm85n163.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x119F occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n163.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n164xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n164.xml
        Test URI:not-wf/P85/ibm85n164.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x11AC occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n164.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n165xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n165.xml
        Test URI:not-wf/P85/ibm85n165.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x11B6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n165.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n166xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n166.xml
        Test URI:not-wf/P85/ibm85n166.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x11B9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n166.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n167xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n167.xml
        Test URI:not-wf/P85/ibm85n167.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x11BB occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n167.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n168xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n168.xml
        Test URI:not-wf/P85/ibm85n168.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x11C3 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n168.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n169xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n169.xml
        Test URI:not-wf/P85/ibm85n169.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x11F1 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n169.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n17xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n17.xml
        Test URI:not-wf/P85/ibm85n17.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x02AF occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n17.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n170xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n170.xml
        Test URI:not-wf/P85/ibm85n170.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x11FA occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n170.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n171xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n171.xml
        Test URI:not-wf/P85/ibm85n171.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1E9C occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n171.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n172xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n172.xml
        Test URI:not-wf/P85/ibm85n172.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1EFA occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n172.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n173xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n173.xml
        Test URI:not-wf/P85/ibm85n173.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1F16 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n173.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n174xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n174.xml
        Test URI:not-wf/P85/ibm85n174.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1F1E occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n174.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n175xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n175.xml
        Test URI:not-wf/P85/ibm85n175.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1F46 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n175.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n176xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n176.xml
        Test URI:not-wf/P85/ibm85n176.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1F4F occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n176.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n177xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n177.xml
        Test URI:not-wf/P85/ibm85n177.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1F58 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n177.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n178xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n178.xml
        Test URI:not-wf/P85/ibm85n178.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1F5A occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n178.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n179xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n179.xml
        Test URI:not-wf/P85/ibm85n179.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1F5C occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n179.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n18xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n18.xml
        Test URI:not-wf/P85/ibm85n18.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x02CF occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n18.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n180xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n180.xml
        Test URI:not-wf/P85/ibm85n180.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1F5E occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n180.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n181xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n181.xml
        Test URI:not-wf/P85/ibm85n181.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1F7E occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n181.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n182xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n182.xml
        Test URI:not-wf/P85/ibm85n182.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1FB5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n182.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n183xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n183.xml
        Test URI:not-wf/P85/ibm85n183.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1FBD occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n183.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n184xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n184.xml
        Test URI:not-wf/P85/ibm85n184.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1FBF occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n184.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n185xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n185.xml
        Test URI:not-wf/P85/ibm85n185.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1FC5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n185.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n186xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n186.xml
        Test URI:not-wf/P85/ibm85n186.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1FCD occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n186.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n187xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n187.xml
        Test URI:not-wf/P85/ibm85n187.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1FD5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n187.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n188xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n188.xml
        Test URI:not-wf/P85/ibm85n188.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1FDC occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n188.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n189xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n189.xml
        Test URI:not-wf/P85/ibm85n189.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1FED occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n189.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n19xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n19.xml
        Test URI:not-wf/P85/ibm85n19.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0387 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n19.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n190xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n190.xml
        Test URI:not-wf/P85/ibm85n190.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1FF5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n190.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n191xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n191.xml
        Test URI:not-wf/P85/ibm85n191.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x1FFD occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n191.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n192xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n192.xml
        Test URI:not-wf/P85/ibm85n192.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x2127 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n192.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n193xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n193.xml
        Test URI:not-wf/P85/ibm85n193.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x212F occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n193.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n194xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n194.xml
        Test URI:not-wf/P85/ibm85n194.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x2183 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n194.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n195xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n195.xml
        Test URI:not-wf/P85/ibm85n195.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x3095 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n195.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n196xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n196.xml
        Test URI:not-wf/P85/ibm85n196.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x30FB occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n196.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n197xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n197.xml
        Test URI:not-wf/P85/ibm85n197.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x312D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n197.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n198xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n198.xml
        Test URI:not-wf/P85/ibm85n198.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #xD7A4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n198.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n20xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n20.xml
        Test URI:not-wf/P85/ibm85n20.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x038B occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n20.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n21xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n21.xml
        Test URI:not-wf/P85/ibm85n21.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x03A2 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n21.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n22xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n22.xml
        Test URI:not-wf/P85/ibm85n22.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x03CF occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n22.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n23xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n23.xml
        Test URI:not-wf/P85/ibm85n23.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x03D7 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n23.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n24xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n24.xml
        Test URI:not-wf/P85/ibm85n24.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x03DD occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n24.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n25xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n25.xml
        Test URI:not-wf/P85/ibm85n25.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x03E1 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n25.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n26xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n26.xml
        Test URI:not-wf/P85/ibm85n26.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x03F4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n26.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n27xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n27.xml
        Test URI:not-wf/P85/ibm85n27.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x040D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n27.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n28xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n28.xml
        Test URI:not-wf/P85/ibm85n28.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0450 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n28.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n29xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n29.xml
        Test URI:not-wf/P85/ibm85n29.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x045D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n29.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n30xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n30.xml
        Test URI:not-wf/P85/ibm85n30.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0482 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n30.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n31xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n31.xml
        Test URI:not-wf/P85/ibm85n31.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x04C5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n31.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n32xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n32.xml
        Test URI:not-wf/P85/ibm85n32.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x04C6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n32.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n33xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n33.xml
        Test URI:not-wf/P85/ibm85n33.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x04C9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n33.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n34xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n34.xml
        Test URI:not-wf/P85/ibm85n34.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x04EC occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n34.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n35xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n35.xml
        Test URI:not-wf/P85/ibm85n35.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x04ED occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n35.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n36xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n36.xml
        Test URI:not-wf/P85/ibm85n36.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x04F6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n36.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n37xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n37.xml
        Test URI:not-wf/P85/ibm85n37.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x04FA occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n37.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n38xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n38.xml
        Test URI:not-wf/P85/ibm85n38.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0557 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n38.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n39xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n39.xml
        Test URI:not-wf/P85/ibm85n39.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0558 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n39.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n40xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n40.xml
        Test URI:not-wf/P85/ibm85n40.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0587 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n40.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n41xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n41.xml
        Test URI:not-wf/P85/ibm85n41.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x05EB occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n41.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n42xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n42.xml
        Test URI:not-wf/P85/ibm85n42.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x05F3 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n42.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n43xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n43.xml
        Test URI:not-wf/P85/ibm85n43.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0620 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n43.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n44xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n44.xml
        Test URI:not-wf/P85/ibm85n44.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x063B occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n44.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n45xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n45.xml
        Test URI:not-wf/P85/ibm85n45.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x064B occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n45.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n46xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n46.xml
        Test URI:not-wf/P85/ibm85n46.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x06B8 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n46.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n47xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n47.xml
        Test URI:not-wf/P85/ibm85n47.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x06BF occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n47.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n48xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n48.xml
        Test URI:not-wf/P85/ibm85n48.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x06CF occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n48.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n49xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n49.xml
        Test URI:not-wf/P85/ibm85n49.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x06D4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n49.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n50xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n50.xml
        Test URI:not-wf/P85/ibm85n50.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x06D6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n50.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n51xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n51.xml
        Test URI:not-wf/P85/ibm85n51.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x06E7 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n51.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n52xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n52.xml
        Test URI:not-wf/P85/ibm85n52.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x093A occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n52.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n53xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n53.xml
        Test URI:not-wf/P85/ibm85n53.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x093E occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n53.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n54xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n54.xml
        Test URI:not-wf/P85/ibm85n54.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0962 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n54.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n55xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n55.xml
        Test URI:not-wf/P85/ibm85n55.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x098D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n55.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n56xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n56.xml
        Test URI:not-wf/P85/ibm85n56.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0991 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n56.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n57xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n57.xml
        Test URI:not-wf/P85/ibm85n57.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0992 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n57.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n58xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n58.xml
        Test URI:not-wf/P85/ibm85n58.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x09A9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n58.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n59xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n59.xml
        Test URI:not-wf/P85/ibm85n59.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x09B1 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n59.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n60xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n60.xml
        Test URI:not-wf/P85/ibm85n60.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x09B5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n60.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n61xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n61.xml
        Test URI:not-wf/P85/ibm85n61.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x09BA occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n61.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n62xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n62.xml
        Test URI:not-wf/P85/ibm85n62.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x09DE occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n62.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n63xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n63.xml
        Test URI:not-wf/P85/ibm85n63.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x09E2 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n63.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n64xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n64.xml
        Test URI:not-wf/P85/ibm85n64.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x09F2 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n64.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n65xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n65.xml
        Test URI:not-wf/P85/ibm85n65.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A0B occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n65.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n66xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n66.xml
        Test URI:not-wf/P85/ibm85n66.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A11 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n66.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n67xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n67.xml
        Test URI:not-wf/P85/ibm85n67.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A29 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n67.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n68xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n68.xml
        Test URI:not-wf/P85/ibm85n68.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A31 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n68.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n69xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n69.xml
        Test URI:not-wf/P85/ibm85n69.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A34 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n69.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n70xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n70.xml
        Test URI:not-wf/P85/ibm85n70.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A37 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n70.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n71xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n71.xml
        Test URI:not-wf/P85/ibm85n71.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A3A occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n71.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n72xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n72.xml
        Test URI:not-wf/P85/ibm85n72.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A5D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n72.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n73xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n73.xml
        Test URI:not-wf/P85/ibm85n73.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A70 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n73.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n74xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n74.xml
        Test URI:not-wf/P85/ibm85n74.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A75 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n74.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n75xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n75.xml
        Test URI:not-wf/P85/ibm85n75.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #xA84 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n75.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n76xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n76.xml
        Test URI:not-wf/P85/ibm85n76.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0ABC occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n76.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n77xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n77.xml
        Test URI:not-wf/P85/ibm85n77.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0A92 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n77.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n78xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n78.xml
        Test URI:not-wf/P85/ibm85n78.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0AA9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n78.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n79xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n79.xml
        Test URI:not-wf/P85/ibm85n79.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0AB1 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n79.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n80xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n80.xml
        Test URI:not-wf/P85/ibm85n80.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0AB4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n80.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n81xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n81.xml
        Test URI:not-wf/P85/ibm85n81.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0ABA occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n81.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n82xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n82.xml
        Test URI:not-wf/P85/ibm85n82.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B04 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n82.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n83xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n83.xml
        Test URI:not-wf/P85/ibm85n83.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B0D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n83.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n84xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n84.xml
        Test URI:not-wf/P85/ibm85n84.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B11 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n84.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n85xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n85.xml
        Test URI:not-wf/P85/ibm85n85.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B29 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n85.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n86xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n86.xml
        Test URI:not-wf/P85/ibm85n86.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B31 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n86.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n87xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n87.xml
        Test URI:not-wf/P85/ibm85n87.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B34 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n87.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n88xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n88.xml
        Test URI:not-wf/P85/ibm85n88.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B3A occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n88.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n89xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n89.xml
        Test URI:not-wf/P85/ibm85n89.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B3E occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n89.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n90xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n90.xml
        Test URI:not-wf/P85/ibm85n90.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B5E occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n90.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n91xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n91.xml
        Test URI:not-wf/P85/ibm85n91.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B62 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n91.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n92xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n92.xml
        Test URI:not-wf/P85/ibm85n92.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B8B occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n92.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n93xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n93.xml
        Test URI:not-wf/P85/ibm85n93.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B91 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n93.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n94xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n94.xml
        Test URI:not-wf/P85/ibm85n94.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B98 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n94.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n95xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n95.xml
        Test URI:not-wf/P85/ibm85n95.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B9B occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n95.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n96xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n96.xml
        Test URI:not-wf/P85/ibm85n96.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0B9D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n96.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n97xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n97.xml
        Test URI:not-wf/P85/ibm85n97.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0BA0 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n97.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n98xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n98.xml
        Test URI:not-wf/P85/ibm85n98.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0BA7 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n98.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p85ibm85n99xml() {
    /*
        Test ID:ibm-not-wf-P85-ibm85n99.xml
        Test URI:not-wf/P85/ibm85n99.xml
        Spec Sections:B.
        Description:Tests BaseChar with an illegal character. The character #x0BAB occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P85/ibm85n99.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p86ibm86n01xml() {
    /*
        Test ID:ibm-not-wf-P86-ibm86n01.xml
        Test URI:not-wf/P86/ibm86n01.xml
        Spec Sections:B.
        Description:Tests Ideographic with an illegal character. The character #x4CFF occurs as the first character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P86/ibm86n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p86ibm86n02xml() {
    /*
        Test ID:ibm-not-wf-P86-ibm86n02.xml
        Test URI:not-wf/P86/ibm86n02.xml
        Spec Sections:B.
        Description:Tests Ideographic with an illegal character. The character #x9FA6 occurs as the first character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P86/ibm86n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p86ibm86n03xml() {
    /*
        Test ID:ibm-not-wf-P86-ibm86n03.xml
        Test URI:not-wf/P86/ibm86n03.xml
        Spec Sections:B.
        Description:Tests Ideographic with an illegal character. The character #x3008 occurs as the first character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P86/ibm86n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p86ibm86n04xml() {
    /*
        Test ID:ibm-not-wf-P86-ibm86n04.xml
        Test URI:not-wf/P86/ibm86n04.xml
        Spec Sections:B.
        Description:Tests Ideographic with an illegal character. The character #x302A occurs as the first character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P86/ibm86n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n01xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n01.xml
        Test URI:not-wf/P87/ibm87n01.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x02FF occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n02xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n02.xml
        Test URI:not-wf/P87/ibm87n02.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0346 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n03xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n03.xml
        Test URI:not-wf/P87/ibm87n03.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0362 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n04xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n04.xml
        Test URI:not-wf/P87/ibm87n04.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0487 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n05xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n05.xml
        Test URI:not-wf/P87/ibm87n05.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x05A2 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n06xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n06.xml
        Test URI:not-wf/P87/ibm87n06.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x05BA occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n07xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n07.xml
        Test URI:not-wf/P87/ibm87n07.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x05BE occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n08xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n08.xml
        Test URI:not-wf/P87/ibm87n08.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x05C0 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n09xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n09.xml
        Test URI:not-wf/P87/ibm87n09.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x05C3 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n10xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n10.xml
        Test URI:not-wf/P87/ibm87n10.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0653 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n10.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n11xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n11.xml
        Test URI:not-wf/P87/ibm87n11.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x06B8 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n11.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n12xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n12.xml
        Test URI:not-wf/P87/ibm87n12.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x06B9 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n12.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n13xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n13.xml
        Test URI:not-wf/P87/ibm87n13.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x06E9 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n13.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n14xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n14.xml
        Test URI:not-wf/P87/ibm87n14.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x06EE occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n14.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n15xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n15.xml
        Test URI:not-wf/P87/ibm87n15.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0904 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n15.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n16xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n16.xml
        Test URI:not-wf/P87/ibm87n16.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x093B occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n16.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n17xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n17.xml
        Test URI:not-wf/P87/ibm87n17.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x094E occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n17.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n18xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n18.xml
        Test URI:not-wf/P87/ibm87n18.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0955 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n18.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n19xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n19.xml
        Test URI:not-wf/P87/ibm87n19.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0964 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n19.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n20xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n20.xml
        Test URI:not-wf/P87/ibm87n20.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0984 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n20.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n21xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n21.xml
        Test URI:not-wf/P87/ibm87n21.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x09C5 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n21.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n22xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n22.xml
        Test URI:not-wf/P87/ibm87n22.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x09C9 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n22.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n23xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n23.xml
        Test URI:not-wf/P87/ibm87n23.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x09CE occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n23.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n24xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n24.xml
        Test URI:not-wf/P87/ibm87n24.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x09D8 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n24.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n25xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n25.xml
        Test URI:not-wf/P87/ibm87n25.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x09E4 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n25.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n26xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n26.xml
        Test URI:not-wf/P87/ibm87n26.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0A03 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n26.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n27xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n27.xml
        Test URI:not-wf/P87/ibm87n27.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0A3D occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n27.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n28xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n28.xml
        Test URI:not-wf/P87/ibm87n28.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0A46 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n28.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n29xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n29.xml
        Test URI:not-wf/P87/ibm87n29.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0A49 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n29.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n30xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n30.xml
        Test URI:not-wf/P87/ibm87n30.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0A4E occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n30.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n31xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n31.xml
        Test URI:not-wf/P87/ibm87n31.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0A80 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n31.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n32xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n32.xml
        Test URI:not-wf/P87/ibm87n32.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0A84 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n32.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n33xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n33.xml
        Test URI:not-wf/P87/ibm87n33.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0ABB occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n33.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n34xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n34.xml
        Test URI:not-wf/P87/ibm87n34.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0AC6 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n34.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n35xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n35.xml
        Test URI:not-wf/P87/ibm87n35.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0ACA occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n35.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n36xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n36.xml
        Test URI:not-wf/P87/ibm87n36.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0ACE occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n36.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n37xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n37.xml
        Test URI:not-wf/P87/ibm87n37.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0B04 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n37.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n38xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n38.xml
        Test URI:not-wf/P87/ibm87n38.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0B3B occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n38.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n39xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n39.xml
        Test URI:not-wf/P87/ibm87n39.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0B44 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n39.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n40xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n40.xml
        Test URI:not-wf/P87/ibm87n40.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0B4A occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n40.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n41xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n41.xml
        Test URI:not-wf/P87/ibm87n41.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0B4E occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n41.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n42xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n42.xml
        Test URI:not-wf/P87/ibm87n42.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0B58 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n42.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n43xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n43.xml
        Test URI:not-wf/P87/ibm87n43.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0B84 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n43.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n44xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n44.xml
        Test URI:not-wf/P87/ibm87n44.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0BC3 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n44.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n45xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n45.xml
        Test URI:not-wf/P87/ibm87n45.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0BC9 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n45.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n46xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n46.xml
        Test URI:not-wf/P87/ibm87n46.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0BD6 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n46.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n47xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n47.xml
        Test URI:not-wf/P87/ibm87n47.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0C0D occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n47.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n48xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n48.xml
        Test URI:not-wf/P87/ibm87n48.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0C45 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n48.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n49xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n49.xml
        Test URI:not-wf/P87/ibm87n49.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0C49 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n49.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n50xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n50.xml
        Test URI:not-wf/P87/ibm87n50.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0C54 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n50.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n51xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n51.xml
        Test URI:not-wf/P87/ibm87n51.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0C81 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n51.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n52xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n52.xml
        Test URI:not-wf/P87/ibm87n52.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0C84 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n52.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n53xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n53.xml
        Test URI:not-wf/P87/ibm87n53.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0CC5 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n53.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n54xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n54.xml
        Test URI:not-wf/P87/ibm87n54.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0CC9 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n54.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n55xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n55.xml
        Test URI:not-wf/P87/ibm87n55.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0CD4 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n55.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n56xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n56.xml
        Test URI:not-wf/P87/ibm87n56.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0CD7 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n56.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n57xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n57.xml
        Test URI:not-wf/P87/ibm87n57.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0D04 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n57.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n58xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n58.xml
        Test URI:not-wf/P87/ibm87n58.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0D45 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n58.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n59xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n59.xml
        Test URI:not-wf/P87/ibm87n59.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0D49 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n59.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n60xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n60.xml
        Test URI:not-wf/P87/ibm87n60.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0D4E occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n60.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n61xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n61.xml
        Test URI:not-wf/P87/ibm87n61.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0D58 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n61.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n62xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n62.xml
        Test URI:not-wf/P87/ibm87n62.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0E3F occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n62.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n63xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n63.xml
        Test URI:not-wf/P87/ibm87n63.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0E3B occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n63.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n64xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n64.xml
        Test URI:not-wf/P87/ibm87n64.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0E4F occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n64.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n66xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n66.xml
        Test URI:not-wf/P87/ibm87n66.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0EBA occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n66.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n67xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n67.xml
        Test URI:not-wf/P87/ibm87n67.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0EBE occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n67.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n68xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n68.xml
        Test URI:not-wf/P87/ibm87n68.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0ECE occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n68.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n69xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n69.xml
        Test URI:not-wf/P87/ibm87n69.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0F1A occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n69.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n70xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n70.xml
        Test URI:not-wf/P87/ibm87n70.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0F36 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n70.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n71xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n71.xml
        Test URI:not-wf/P87/ibm87n71.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0F38 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n71.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n72xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n72.xml
        Test URI:not-wf/P87/ibm87n72.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0F3B occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n72.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n73xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n73.xml
        Test URI:not-wf/P87/ibm87n73.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0F3A occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n73.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n74xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n74.xml
        Test URI:not-wf/P87/ibm87n74.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0F70 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n74.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n75xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n75.xml
        Test URI:not-wf/P87/ibm87n75.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0F85 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n75.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n76xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n76.xml
        Test URI:not-wf/P87/ibm87n76.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0F8C occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n76.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n77xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n77.xml
        Test URI:not-wf/P87/ibm87n77.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0F96 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n77.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n78xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n78.xml
        Test URI:not-wf/P87/ibm87n78.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0F98 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n78.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n79xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n79.xml
        Test URI:not-wf/P87/ibm87n79.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0FB0 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n79.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n80xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n80.xml
        Test URI:not-wf/P87/ibm87n80.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0FB8 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n80.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n81xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n81.xml
        Test URI:not-wf/P87/ibm87n81.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x0FBA occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n81.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n82xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n82.xml
        Test URI:not-wf/P87/ibm87n82.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x20DD occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n82.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n83xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n83.xml
        Test URI:not-wf/P87/ibm87n83.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x20E2 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n83.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n84xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n84.xml
        Test URI:not-wf/P87/ibm87n84.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x3030 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n84.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p87ibm87n85xml() {
    /*
        Test ID:ibm-not-wf-P87-ibm87n85.xml
        Test URI:not-wf/P87/ibm87n85.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an illegal character. The character #x309B occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P87/ibm87n85.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p88ibm88n01xml() {
    /*
        Test ID:ibm-not-wf-P88-ibm88n01.xml
        Test URI:not-wf/P88/ibm88n01.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0029 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p88ibm88n02xml() {
    /*
        Test ID:ibm-not-wf-P88-ibm88n02.xml
        Test URI:not-wf/P88/ibm88n02.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x003B occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p88ibm88n03xml() {
    /*
        Test ID:ibm-not-wf-P88-ibm88n03.xml
        Test URI:not-wf/P88/ibm88n03.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x066A occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p88ibm88n04xml() {
    /*
        Test ID:ibm-not-wf-P88-ibm88n04.xml
        Test URI:not-wf/P88/ibm88n04.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x06FA occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p88ibm88n05xml() {
    /*
        Test ID:ibm-not-wf-P88-ibm88n05.xml
        Test URI:not-wf/P88/ibm88n05.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0970 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p88ibm88n06xml() {
    /*
        Test ID:ibm-not-wf-P88-ibm88n06.xml
        Test URI:not-wf/P88/ibm88n06.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x09F2 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p88ibm88n08xml() {
    /*
        Test ID:ibm-not-wf-P88-ibm88n08.xml
        Test URI:not-wf/P88/ibm88n08.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0AF0 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p88ibm88n09xml() {
    /*
        Test ID:ibm-not-wf-P88-ibm88n09.xml
        Test URI:not-wf/P88/ibm88n09.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0B70 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p88ibm88n10xml() {
    /*
        Test ID:ibm-not-wf-P88-ibm88n10.xml
        Test URI:not-wf/P88/ibm88n10.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0C65 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n10.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p88ibm88n11xml() {
    /*
        Test ID:ibm-not-wf-P88-ibm88n11.xml
        Test URI:not-wf/P88/ibm88n11.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0CE5 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n11.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p88ibm88n12xml() {
    /*
        Test ID:ibm-not-wf-P88-ibm88n12.xml
        Test URI:not-wf/P88/ibm88n12.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0CF0 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n12.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p88ibm88n13xml() {
    /*
        Test ID:ibm-not-wf-P88-ibm88n13.xml
        Test URI:not-wf/P88/ibm88n13.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0D70 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n13.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p88ibm88n14xml() {
    /*
        Test ID:ibm-not-wf-P88-ibm88n14.xml
        Test URI:not-wf/P88/ibm88n14.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0E5A occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n14.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p88ibm88n15xml() {
    /*
        Test ID:ibm-not-wf-P88-ibm88n15.xml
        Test URI:not-wf/P88/ibm88n15.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0EDA occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n15.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p88ibm88n16xml() {
    /*
        Test ID:ibm-not-wf-P88-ibm88n16.xml
        Test URI:not-wf/P88/ibm88n16.xml
        Spec Sections:B.
        Description:Tests Digit with an illegal character. The character #x0F2A occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P88/ibm88n16.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p89ibm89n01xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n01.xml
        Test URI:not-wf/P89/ibm89n01.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x00B6 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p89ibm89n02xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n02.xml
        Test URI:not-wf/P89/ibm89n02.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x00B8 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p89ibm89n03xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n03.xml
        Test URI:not-wf/P89/ibm89n03.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x02D2 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p89ibm89n04xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n04.xml
        Test URI:not-wf/P89/ibm89n04.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x03FE occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn ibmnotwf_p89ibm89n05xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n05.xml
        Test URI:not-wf/P89/ibm89n05.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x065F occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p89ibm89n06xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n06.xml
        Test URI:not-wf/P89/ibm89n06.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x0EC7 occurs as the second character in the PITarget in the PI in the DTD. [Also contains two top-level elements -- one should be removed]
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p89ibm89n07xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n07.xml
        Test URI:not-wf/P89/ibm89n07.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x3006 occurs as the second character in the PITarget in the PI in the DTD. [Also contains two top-level elements -- one should be removed]
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p89ibm89n08xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n08.xml
        Test URI:not-wf/P89/ibm89n08.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x3030 occurs as the second character in the PITarget in the PI in the DTD. [Also contains two top-level elements -- one should be removed]
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p89ibm89n09xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n09.xml
        Test URI:not-wf/P89/ibm89n09.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x3036 occurs as the second character in the PITarget in the PI in the DTD. [Also contains two top-level elements -- one should be removed]
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p89ibm89n10xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n10.xml
        Test URI:not-wf/P89/ibm89n10.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x309C occurs as the second character in the PITarget in the PI in the DTD. [Also contains two top-level elements -- one should be removed]
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n10.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p89ibm89n11xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n11.xml
        Test URI:not-wf/P89/ibm89n11.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x309F occurs as the second character in the PITarget in the PI in the DTD. [Also contains two top-level elements -- one should be removed]
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n11.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn ibmnotwf_p89ibm89n12xml() {
    /*
        Test ID:ibm-not-wf-P89-ibm89n12.xml
        Test URI:not-wf/P89/ibm89n12.xml
        Spec Sections:B.
        Description:Tests Extender with an illegal character. The character #x30FF occurs as the second character in the PITarget in the PI in the DTD. [Also contains two top-level elements -- one should be removed]
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/ibm/not-wf/P89/ibm89n12.xml").unwrap(),
    );

    assert!(testxml.is_err());
}
