/*
University of Edinburgh XML 1.0 4th edition errata test suite.
*/

use std::convert::TryFrom;
use std::fs;
use xrust::Document;

#[test]
fn xibm105notwf_p04ibm04n02xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n02.xml
        Test URI:ibm04n02.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x333
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n03xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n03.xml
        Test URI:ibm04n03.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x369
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n04xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n04.xml
        Test URI:ibm04n04.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x37E
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n05xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n05.xml
        Test URI:ibm04n05.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2000
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n06xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n06.xml
        Test URI:ibm04n06.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2001
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n07xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n07.xml
        Test URI:ibm04n07.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2002
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n08xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n08.xml
        Test URI:ibm04n08.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2005
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n09xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n09.xml
        Test URI:ibm04n09.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x200B
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n10xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n10.xml
        Test URI:ibm04n10.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x200E
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n10.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n11xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n11.xml
        Test URI:ibm04n11.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x200F
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n11.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n12xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n12.xml
        Test URI:ibm04n12.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2069
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n12.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n13xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n13.xml
        Test URI:ibm04n13.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2190
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n13.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n14xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n14.xml
        Test URI:ibm04n14.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x23FF
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n14.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n15xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n15.xml
        Test URI:ibm04n15.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x280F
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n15.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n16xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n16.xml
        Test URI:ibm04n16.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2A00
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n16.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n17xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n17.xml
        Test URI:ibm04n17.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2EDC
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n17.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n18xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n18.xml
        Test URI:ibm04n18.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2B00
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n18.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n19xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n19.xml
        Test URI:ibm04n19.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x2BFF
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n19.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n20xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n20.xml
        Test URI:ibm04n20.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0x3000
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n20.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n21xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n21.xml
        Test URI:ibm04n21.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xD800
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n21.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n22xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n22.xml
        Test URI:ibm04n22.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xD801
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n22.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n23xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n23.xml
        Test URI:ibm04n23.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xDAFF
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n23.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n24xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n24.xml
        Test URI:ibm04n24.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xDFFF
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n24.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n25xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n25.xml
        Test URI:ibm04n25.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xEFFF
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n25.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n26xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n26.xml
        Test URI:ibm04n26.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xF1FF
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n26.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n27xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n27.xml
        Test URI:ibm04n27.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xF8FF
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n27.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04ibm04n28xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04-ibm04n28.xml
        Test URI:ibm04n28.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameStartChar: #0xFFFFF
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04n28.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an01xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an01.xml
        Test URI:ibm04an01.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #xB8
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an02xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an02.xml
        Test URI:ibm04an02.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xA1
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an03xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an03.xml
        Test URI:ibm04an03.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xAF
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an04xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an04.xml
        Test URI:ibm04an04.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x37E
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an05xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an05.xml
        Test URI:ibm04an05.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2000
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an06xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an06.xml
        Test URI:ibm04an06.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2001
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an07xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an07.xml
        Test URI:ibm04an07.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2002
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an07.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an08xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an08.xml
        Test URI:ibm04an08.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2005
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an08.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an09xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an09.xml
        Test URI:ibm04an09.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x200B
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an09.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an10xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an10.xml
        Test URI:ibm04an10.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x200E
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an10.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an11xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an11.xml
        Test URI:ibm04an11.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2038
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an11.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an12xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an12.xml
        Test URI:ibm04an12.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2041
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an12.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an13xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an13.xml
        Test URI:ibm04an13.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2190
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an13.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an14xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an14.xml
        Test URI:ibm04an14.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x23FF
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an14.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an15xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an15.xml
        Test URI:ibm04an15.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x280F
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an15.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an16xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an16.xml
        Test URI:ibm04an16.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2A00
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an16.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an17xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an17.xml
        Test URI:ibm04an17.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xFDD0
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an17.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an18xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an18.xml
        Test URI:ibm04an18.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xFDEF
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an18.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an19xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an19.xml
        Test URI:ibm04an19.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x2FFF
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an19.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an20xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an20.xml
        Test URI:ibm04an20.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0x3000
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an20.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an21xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an21.xml
        Test URI:ibm04an21.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xD800
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an21.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an22xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an22.xml
        Test URI:ibm04an22.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xD801
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an22.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an23xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an23.xml
        Test URI:ibm04an23.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xDAFF
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an23.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an24xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an24.xml
        Test URI:ibm04an24.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xDFFF
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an24.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an25xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an25.xml
        Test URI:ibm04an25.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xEFFF
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an25.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an26xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an26.xml
        Test URI:ibm04an26.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xF1FF
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an26.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an27xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an27.xml
        Test URI:ibm04an27.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xF8FF
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an27.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p04aibm04an28xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P04a-ibm04an28.xml
        Test URI:ibm04an28.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal NameChar: #0xFFFFF
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04an28.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p05ibm05n01xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P05-ibm05n01.xml
        Test URI:ibm05n01.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal Name containing #0x0B
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm05n01.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p05ibm05n02xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P05-ibm05n02.xml
        Test URI:ibm05n02.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal Name containing #0x300
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm05n02.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p05ibm05n03xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P05-ibm05n03.xml
        Test URI:ibm05n03.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal Name containing #0x36F
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm05n03.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p05ibm05n04xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P05-ibm05n04.xml
        Test URI:ibm05n04.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal Name containing #0x203F
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm05n04.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p05ibm05n05xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P05-ibm05n05.xml
        Test URI:ibm05n05.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal Name containing #x2040
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm05n05.xml").unwrap(),
    );

    assert!(testxml.is_err());
}

#[test]
fn xibm105notwf_p05ibm05n06xml() {
    /*
        Test ID:x-ibm-1-0.5-not-wf-P05-ibm05n06.xml
        Test URI:ibm05n06.xml
        Spec Sections:2.3
        Description:Tests an element with an illegal Name containing #0xB7
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm05n06.xml").unwrap(),
    );

    assert!(testxml.is_err());
}
