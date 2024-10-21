/*

University of Edinburgh XML 1.0 4th edition errata test suite.

*/

use std::fs;
use xrust::item::Node;
use xrust::parser::xml;
use xrust::trees::smite::RNode;

#[test]
fn xrmt008b() {
    /*
        Test ID:x-rmt-008b
        Test URI:008.xml
        Spec Sections:2.8 4.3.4
        Description:a document with version=1.7, legal in XML 1.0 from 5th edition
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/008.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn xrmt5014a() {
    /*
        Test ID:x-rmt5-014a
        Test URI:014a.xml
        Spec Sections:2.3
        Description:Has a "long s" in a name, legal in XML 1.1, legal in XML 1.0 5th edition
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/014a.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
#[ignore]
fn xibm105valid_p04ibm04v01xml() {
    /*
        Test ID:x-ibm-1-0.5-valid-P04-ibm04v01.xml
        Test URI:ibm04v01.xml
        Spec Sections:2.3
        Description:This test case covers legal NameStartChars character ranges plus discrete legal characters for production 04.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04v01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn xibm105valid_p04ibm04av01xml() {
    /*
        Test ID:x-ibm-1-0.5-valid-P04-ibm04av01.xml
        Test URI:ibm04av01.xml
        Spec Sections:2.3
        Description:This test case covers legal NameChars character ranges plus discrete legal characters for production 04a.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm04av01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
#[ignore]
fn xibm105valid_p05ibm05v01xml() {
    /*
        Test ID:x-ibm-1-0.5-valid-P05-ibm05v01.xml
        Test URI:ibm05v01.xml
        Spec Sections:2.3
        Description:This test case covers legal Element Names as per production 5.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm05v01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
#[ignore]
fn xibm105valid_p05ibm05v02xml() {
    /*
        This test is deliberately ignored.
        We only support namespace aware xml documents, and that means no colons in processing instructions.
    */
    /*
        Test ID:x-ibm-1-0.5-valid-P05-ibm05v02.xml
        Test URI:ibm05v02.xml
        Spec Sections:2.3
        Description:This test case covers legal PITarget (Names) as per production 5.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm05v02.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
#[ignore]
fn xibm105valid_p05ibm05v03xml() {
    /*
        This test is deliberately ignored.
        We only support namespace-aware XML, no colons allowed.
    */
    /*
        Test ID:x-ibm-1-0.5-valid-P05-ibm05v03.xml
        Test URI:ibm05v03.xml
        Spec Sections:2.3
        Description:This test case covers legal Attribute (Names) as per production 5.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm05v03.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn xibm105valid_p05ibm05v04xml() {
    /*
        Test ID:x-ibm-1-0.5-valid-P05-ibm05v04.xml
        Test URI:ibm05v04.xml
        Spec Sections:2.3
        Description:This test case covers legal ID/IDREF (Names) as per production 5.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm05v04.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
#[ignore]
fn xibm105valid_p05ibm05v05xml() {
    /*
        Test ID:x-ibm-1-0.5-valid-P05-ibm05v05.xml
        Test URI:ibm05v05.xml
        Spec Sections:2.3
        Description:This test case covers legal ENTITY (Names) as per production 5.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm05v05.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn xibm105valid_p047ibm07v01xml() {
    /*
        Test ID:x-ibm-1-0.5-valid-P047-ibm07v01.xml
        Test URI:ibm07v01.xml
        Spec Sections:2.3
        Description:This test case covers legal NMTOKEN Name character ranges plus discrete legal characters for production 7.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm07v01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n03xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n03.xml
        Test URI:ibm85n03.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0132 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n03.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n04xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n04.xml
        Test URI:ibm85n04.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0133 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n04.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n05xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n05.xml
        Test URI:ibm85n05.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x013F occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n05.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n06xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n06.xml
        Test URI:ibm85n06.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0140 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n06.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n07xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n07.xml
        Test URI:ibm85n07.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0149 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n07.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n08xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n08.xml
        Test URI:ibm85n08.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x017F occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n08.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n09xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n09.xml
        Test URI:ibm85n09.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x01c4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n09.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n10xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n10.xml
        Test URI:ibm85n10.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x01CC occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n10.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n100xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n100.xml
        Test URI:ibm85n100.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0BB6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n100.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n101xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n101.xml
        Test URI:ibm85n101.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0BBA occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n101.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n102xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n102.xml
        Test URI:ibm85n102.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0C0D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n102.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n103xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n103.xml
        Test URI:ibm85n103.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0C11 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n103.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n104xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n104.xml
        Test URI:ibm85n104.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0C29 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n104.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n105xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n105.xml
        Test URI:ibm85n105.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0C34 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n105.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n106xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n106.xml
        Test URI:ibm85n106.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0C5F occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n106.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n107xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n107.xml
        Test URI:ibm85n107.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0C62 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n107.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n108xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n108.xml
        Test URI:ibm85n108.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0C8D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n108.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n109xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n109.xml
        Test URI:ibm85n109.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0C91 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n109.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n11xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n11.xml
        Test URI:ibm85n11.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x01F1 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n11.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n110xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n110.xml
        Test URI:ibm85n110.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0CA9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n110.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n111xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n111.xml
        Test URI:ibm85n111.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0CB4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n111.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n112xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n112.xml
        Test URI:ibm85n112.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0CBA occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n112.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n113xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n113.xml
        Test URI:ibm85n113.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0CDF occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n113.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n114xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n114.xml
        Test URI:ibm85n114.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0CE2 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n114.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n115xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n115.xml
        Test URI:ibm85n115.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0D0D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n115.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n116xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n116.xml
        Test URI:ibm85n116.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0D11 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n116.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n117xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n117.xml
        Test URI:ibm85n117.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0D29 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n117.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n118xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n118.xml
        Test URI:ibm85n118.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0D3A occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n118.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n119xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n119.xml
        Test URI:ibm85n119.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0D62 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n119.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n12xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n12.xml
        Test URI:ibm85n12.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x01F3 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n12.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n120xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n120.xml
        Test URI:ibm85n120.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0E2F occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n120.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n121xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n121.xml
        Test URI:ibm85n121.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0E31 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n121.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n122xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n122.xml
        Test URI:ibm85n122.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0E34 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n122.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n123xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n123.xml
        Test URI:ibm85n123.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0E46 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n123.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n124xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n124.xml
        Test URI:ibm85n124.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0E83 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n124.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n125xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n125.xml
        Test URI:ibm85n125.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0E85 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n125.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n126xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n126.xml
        Test URI:ibm85n126.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0E89 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n126.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n127xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n127.xml
        Test URI:ibm85n127.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0E8B occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n127.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n128xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n128.xml
        Test URI:ibm85n128.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0E8E occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n128.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n129xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n129.xml
        Test URI:ibm85n129.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0E98 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n129.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n13xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n13.xml
        Test URI:ibm85n13.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x01F6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n13.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n130xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n130.xml
        Test URI:ibm85n130.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0EA0 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n130.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n131xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n131.xml
        Test URI:ibm85n131.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0EA4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n131.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n132xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n132.xml
        Test URI:ibm85n132.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0EA6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n132.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n133xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n133.xml
        Test URI:ibm85n133.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0EA8 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n133.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n134xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n134.xml
        Test URI:ibm85n134.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0EAC occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n134.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n135xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n135.xml
        Test URI:ibm85n135.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0EAF occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n135.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n136xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n136.xml
        Test URI:ibm85n136.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0EB1 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n136.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n137xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n137.xml
        Test URI:ibm85n137.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0EB4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n137.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n138xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n138.xml
        Test URI:ibm85n138.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0EBE occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n138.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n139xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n139.xml
        Test URI:ibm85n139.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0EC5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n139.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n14xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n14.xml
        Test URI:ibm85n14.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x01F9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n14.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n140xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n140.xml
        Test URI:ibm85n140.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0F48 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n140.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n141xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n141.xml
        Test URI:ibm85n141.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0F6A occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n141.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n142xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n142.xml
        Test URI:ibm85n142.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x10C6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n142.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n143xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n143.xml
        Test URI:ibm85n143.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x10F7 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n143.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n144xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n144.xml
        Test URI:ibm85n144.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1011 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n144.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n145xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n145.xml
        Test URI:ibm85n145.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1104 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n145.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n146xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n146.xml
        Test URI:ibm85n146.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1108 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n146.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n147xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n147.xml
        Test URI:ibm85n147.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x110A occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n147.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n148xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n148.xml
        Test URI:ibm85n148.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x110D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n148.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n149xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n149.xml
        Test URI:ibm85n149.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x113B occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n149.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n15xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n15.xml
        Test URI:ibm85n15.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x01F9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n15.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n150xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n150.xml
        Test URI:ibm85n150.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x113F occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n150.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n151xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n151.xml
        Test URI:ibm85n151.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1141 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n151.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n152xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n152.xml
        Test URI:ibm85n152.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x114D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n152.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n153xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n153.xml
        Test URI:ibm85n153.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x114f occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n153.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n154xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n154.xml
        Test URI:ibm85n154.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1151 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n154.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n155xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n155.xml
        Test URI:ibm85n155.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1156 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n155.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n156xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n156.xml
        Test URI:ibm85n156.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x115A occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n156.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n157xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n157.xml
        Test URI:ibm85n157.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1162 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n157.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n158xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n158.xml
        Test URI:ibm85n158.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1164 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n158.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n159xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n159.xml
        Test URI:ibm85n159.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1166 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n159.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n16xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n16.xml
        Test URI:ibm85n16.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0230 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n16.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n160xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n160.xml
        Test URI:ibm85n160.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x116B occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n160.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n161xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n161.xml
        Test URI:ibm85n161.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x116F occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n161.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n162xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n162.xml
        Test URI:ibm85n162.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1174 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n162.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n163xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n163.xml
        Test URI:ibm85n163.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x119F occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n163.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n164xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n164.xml
        Test URI:ibm85n164.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x11AC occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n164.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n165xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n165.xml
        Test URI:ibm85n165.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x11B6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n165.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n166xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n166.xml
        Test URI:ibm85n166.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x11B9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n166.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n167xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n167.xml
        Test URI:ibm85n167.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x11BB occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n167.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n168xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n168.xml
        Test URI:ibm85n168.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x11C3 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n168.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n169xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n169.xml
        Test URI:ibm85n169.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x11F1 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n169.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n17xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n17.xml
        Test URI:ibm85n17.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x02AF occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n17.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n170xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n170.xml
        Test URI:ibm85n170.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x11FA occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n170.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n171xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n171.xml
        Test URI:ibm85n171.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1E9C occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n171.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n172xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n172.xml
        Test URI:ibm85n172.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1EFA occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n172.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n173xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n173.xml
        Test URI:ibm85n173.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1F16 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n173.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n174xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n174.xml
        Test URI:ibm85n174.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1F1E occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n174.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n175xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n175.xml
        Test URI:ibm85n175.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1F46 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n175.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n176xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n176.xml
        Test URI:ibm85n176.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1F4F occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n176.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n177xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n177.xml
        Test URI:ibm85n177.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1F58 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n177.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n178xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n178.xml
        Test URI:ibm85n178.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1F5A occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n178.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n179xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n179.xml
        Test URI:ibm85n179.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1F5C occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n179.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n18xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n18.xml
        Test URI:ibm85n18.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x02CF occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n18.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n180xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n180.xml
        Test URI:ibm85n180.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1F5E occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n180.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n181xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n181.xml
        Test URI:ibm85n181.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1F7E occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n181.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n182xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n182.xml
        Test URI:ibm85n182.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1FB5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n182.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n183xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n183.xml
        Test URI:ibm85n183.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1FBD occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n183.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n184xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n184.xml
        Test URI:ibm85n184.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1FBF occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n184.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n185xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n185.xml
        Test URI:ibm85n185.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1FC5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n185.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n186xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n186.xml
        Test URI:ibm85n186.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1FCD occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n186.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n187xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n187.xml
        Test URI:ibm85n187.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1FD5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n187.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n188xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n188.xml
        Test URI:ibm85n188.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1FDC occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n188.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n189xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n189.xml
        Test URI:ibm85n189.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1FED occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n189.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n19xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n19.xml
        Test URI:ibm85n19.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0387 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n19.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n190xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n190.xml
        Test URI:ibm85n190.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1FF5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n190.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n191xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n191.xml
        Test URI:ibm85n191.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x1FFD occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n191.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n192xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n192.xml
        Test URI:ibm85n192.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x2127 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n192.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n193xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n193.xml
        Test URI:ibm85n193.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x212F occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n193.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n194xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n194.xml
        Test URI:ibm85n194.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x2183 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n194.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n195xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n195.xml
        Test URI:ibm85n195.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x3095 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n195.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n196xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n196.xml
        Test URI:ibm85n196.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x30FB occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n196.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n197xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n197.xml
        Test URI:ibm85n197.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x312D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n197.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n198xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n198.xml
        Test URI:ibm85n198.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #xD7A4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n198.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n20xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n20.xml
        Test URI:ibm85n20.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x038B occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n20.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n21xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n21.xml
        Test URI:ibm85n21.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x03A2 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n21.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n22xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n22.xml
        Test URI:ibm85n22.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x03CF occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n22.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n23xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n23.xml
        Test URI:ibm85n23.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x03D7 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n23.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n24xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n24.xml
        Test URI:ibm85n24.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x03DD occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n24.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n25xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n25.xml
        Test URI:ibm85n25.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x03E1 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n25.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n26xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n26.xml
        Test URI:ibm85n26.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x03F4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n26.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n27xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n27.xml
        Test URI:ibm85n27.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x040D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n27.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n28xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n28.xml
        Test URI:ibm85n28.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0450 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n28.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n29xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n29.xml
        Test URI:ibm85n29.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x045D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n29.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n30xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n30.xml
        Test URI:ibm85n30.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0482 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n30.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n31xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n31.xml
        Test URI:ibm85n31.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x04C5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n31.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n32xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n32.xml
        Test URI:ibm85n32.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x04C6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n32.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n33xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n33.xml
        Test URI:ibm85n33.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x04C9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n33.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n34xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n34.xml
        Test URI:ibm85n34.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x04EC occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n34.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n35xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n35.xml
        Test URI:ibm85n35.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x04ED occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n35.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n36xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n36.xml
        Test URI:ibm85n36.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x04F6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n36.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n37xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n37.xml
        Test URI:ibm85n37.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x04FA occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n37.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n38xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n38.xml
        Test URI:ibm85n38.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0557 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n38.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n39xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n39.xml
        Test URI:ibm85n39.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0558 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n39.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n40xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n40.xml
        Test URI:ibm85n40.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0587 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n40.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n41xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n41.xml
        Test URI:ibm85n41.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x05EB occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n41.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n42xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n42.xml
        Test URI:ibm85n42.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x05F3 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n42.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n43xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n43.xml
        Test URI:ibm85n43.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0620 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n43.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n44xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n44.xml
        Test URI:ibm85n44.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x063B occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n44.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n45xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n45.xml
        Test URI:ibm85n45.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x064B occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n45.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n46xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n46.xml
        Test URI:ibm85n46.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x06B8 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n46.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n47xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n47.xml
        Test URI:ibm85n47.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x06BF occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n47.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n48xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n48.xml
        Test URI:ibm85n48.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x06CF occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n48.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n49xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n49.xml
        Test URI:ibm85n49.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x06D4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n49.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n50xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n50.xml
        Test URI:ibm85n50.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x06D6 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n50.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n51xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n51.xml
        Test URI:ibm85n51.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x06E7 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n51.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n52xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n52.xml
        Test URI:ibm85n52.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x093A occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n52.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n53xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n53.xml
        Test URI:ibm85n53.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x093E occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n53.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n54xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n54.xml
        Test URI:ibm85n54.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0962 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n54.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n55xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n55.xml
        Test URI:ibm85n55.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x098D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n55.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n56xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n56.xml
        Test URI:ibm85n56.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0991 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n56.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n57xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n57.xml
        Test URI:ibm85n57.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0992 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n57.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n58xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n58.xml
        Test URI:ibm85n58.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x09A9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n58.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n59xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n59.xml
        Test URI:ibm85n59.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x09B1 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n59.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n60xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n60.xml
        Test URI:ibm85n60.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x09B5 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n60.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n61xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n61.xml
        Test URI:ibm85n61.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x09BA occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n61.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n62xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n62.xml
        Test URI:ibm85n62.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x09DE occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n62.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n63xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n63.xml
        Test URI:ibm85n63.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x09E2 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n63.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n64xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n64.xml
        Test URI:ibm85n64.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x09F2 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n64.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n65xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n65.xml
        Test URI:ibm85n65.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0A0B occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n65.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n66xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n66.xml
        Test URI:ibm85n66.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0A11 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n66.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n67xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n67.xml
        Test URI:ibm85n67.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0A29 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n67.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n68xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n68.xml
        Test URI:ibm85n68.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0A31 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n68.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n69xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n69.xml
        Test URI:ibm85n69.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0A34 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n69.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n70xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n70.xml
        Test URI:ibm85n70.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0A37 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n70.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n71xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n71.xml
        Test URI:ibm85n71.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0A3A occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n71.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n72xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n72.xml
        Test URI:ibm85n72.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0A5D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n72.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n73xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n73.xml
        Test URI:ibm85n73.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0A70 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n73.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n74xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n74.xml
        Test URI:ibm85n74.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0A75 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n74.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n75xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n75.xml
        Test URI:ibm85n75.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #xA84 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n75.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n76xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n76.xml
        Test URI:ibm85n76.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0ABC occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n76.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n77xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n77.xml
        Test URI:ibm85n77.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0A92 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n77.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n78xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n78.xml
        Test URI:ibm85n78.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0AA9 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n78.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n79xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n79.xml
        Test URI:ibm85n79.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0AB1 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n79.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n80xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n80.xml
        Test URI:ibm85n80.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0AB4 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n80.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n81xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n81.xml
        Test URI:ibm85n81.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0ABA occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n81.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n82xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n82.xml
        Test URI:ibm85n82.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0B04 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n82.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n83xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n83.xml
        Test URI:ibm85n83.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0B0D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n83.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n84xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n84.xml
        Test URI:ibm85n84.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0B11 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n84.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n85xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n85.xml
        Test URI:ibm85n85.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0B29 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n85.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n86xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n86.xml
        Test URI:ibm85n86.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0B31 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n86.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n87xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n87.xml
        Test URI:ibm85n87.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0B34 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n87.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n88xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n88.xml
        Test URI:ibm85n88.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0B3A occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n88.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n89xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n89.xml
        Test URI:ibm85n89.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0B3E occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n89.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n90xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n90.xml
        Test URI:ibm85n90.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0B5E occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n90.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n91xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n91.xml
        Test URI:ibm85n91.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0B62 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n91.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n92xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n92.xml
        Test URI:ibm85n92.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0B8B occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n92.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n93xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n93.xml
        Test URI:ibm85n93.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0B91 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n93.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n94xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n94.xml
        Test URI:ibm85n94.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0B98 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n94.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n95xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n95.xml
        Test URI:ibm85n95.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0B9B occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n95.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n96xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n96.xml
        Test URI:ibm85n96.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0B9D occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n96.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n97xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n97.xml
        Test URI:ibm85n97.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0BA0 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n97.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n98xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n98.xml
        Test URI:ibm85n98.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0BA7 occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n98.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p85ibm85n99xml() {
    /*
        Test ID:ibm-valid-P85-ibm85n99.xml
        Test URI:ibm85n99.xml
        Spec Sections:B.
        Description:Tests BaseChar with an only legal per 5th edition character. The character #x0BAB occurs as the first character of the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm85n99.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p86ibm86n01xml() {
    /*
        Test ID:ibm-valid-P86-ibm86n01.xml
        Test URI:ibm86n01.xml
        Spec Sections:B.
        Description:Tests Ideographic with an only legal per 5th edition character. The character #x4CFF occurs as the first character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm86n01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p86ibm86n02xml() {
    /*
        Test ID:ibm-valid-P86-ibm86n02.xml
        Test URI:ibm86n02.xml
        Spec Sections:B.
        Description:Tests Ideographic with an only legal per 5th edition character. The character #x9FA6 occurs as the first character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm86n02.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p86ibm86n03xml() {
    /*
        Test ID:ibm-valid-P86-ibm86n03.xml
        Test URI:ibm86n03.xml
        Spec Sections:B.
        Description:Tests Ideographic with an only legal per 5th edition character. The character #x3008 occurs as the first character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm86n03.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p86ibm86n04xml() {
    /*
        Test ID:ibm-valid-P86-ibm86n04.xml
        Test URI:ibm86n04.xml
        Spec Sections:B.
        Description:Tests Ideographic with an only legal per 5th edition character. The character #x302A occurs as the first character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm86n04.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n01xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n01.xml
        Test URI:ibm87n01.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x02FF occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n01.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n02xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n02.xml
        Test URI:ibm87n02.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0346 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n02.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n03xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n03.xml
        Test URI:ibm87n03.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0362 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n03.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n04xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n04.xml
        Test URI:ibm87n04.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0487 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n04.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n05xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n05.xml
        Test URI:ibm87n05.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x05A2 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n05.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n06xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n06.xml
        Test URI:ibm87n06.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x05BA occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n06.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n07xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n07.xml
        Test URI:ibm87n07.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x05BE occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n07.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n08xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n08.xml
        Test URI:ibm87n08.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x05C0 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n08.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n09xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n09.xml
        Test URI:ibm87n09.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x05C3 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n09.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n10xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n10.xml
        Test URI:ibm87n10.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0653 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n10.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n11xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n11.xml
        Test URI:ibm87n11.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x06B8 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n11.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n12xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n12.xml
        Test URI:ibm87n12.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x06B9 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n12.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n13xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n13.xml
        Test URI:ibm87n13.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x06E9 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n13.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n14xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n14.xml
        Test URI:ibm87n14.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x06EE occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n14.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n15xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n15.xml
        Test URI:ibm87n15.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0904 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n15.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n16xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n16.xml
        Test URI:ibm87n16.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x093B occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n16.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n17xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n17.xml
        Test URI:ibm87n17.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x094E occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n17.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n18xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n18.xml
        Test URI:ibm87n18.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0955 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n18.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n19xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n19.xml
        Test URI:ibm87n19.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0964 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n19.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n20xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n20.xml
        Test URI:ibm87n20.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0984 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n20.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n21xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n21.xml
        Test URI:ibm87n21.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x09C5 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n21.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n22xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n22.xml
        Test URI:ibm87n22.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x09C9 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n22.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n23xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n23.xml
        Test URI:ibm87n23.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x09CE occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n23.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n24xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n24.xml
        Test URI:ibm87n24.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x09D8 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n24.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n25xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n25.xml
        Test URI:ibm87n25.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x09E4 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n25.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n26xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n26.xml
        Test URI:ibm87n26.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0A03 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n26.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n27xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n27.xml
        Test URI:ibm87n27.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0A3D occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n27.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n28xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n28.xml
        Test URI:ibm87n28.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0A46 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n28.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n29xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n29.xml
        Test URI:ibm87n29.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0A49 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n29.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n30xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n30.xml
        Test URI:ibm87n30.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0A4E occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n30.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n31xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n31.xml
        Test URI:ibm87n31.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0A80 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n31.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n32xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n32.xml
        Test URI:ibm87n32.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0A84 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n32.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n33xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n33.xml
        Test URI:ibm87n33.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0ABB occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n33.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n34xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n34.xml
        Test URI:ibm87n34.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0AC6 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n34.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n35xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n35.xml
        Test URI:ibm87n35.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0ACA occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n35.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n36xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n36.xml
        Test URI:ibm87n36.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0ACE occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n36.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n37xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n37.xml
        Test URI:ibm87n37.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0B04 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n37.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n38xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n38.xml
        Test URI:ibm87n38.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0B3B occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n38.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n39xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n39.xml
        Test URI:ibm87n39.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0B44 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n39.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n40xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n40.xml
        Test URI:ibm87n40.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0B4A occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n40.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n41xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n41.xml
        Test URI:ibm87n41.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0B4E occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n41.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n42xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n42.xml
        Test URI:ibm87n42.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0B58 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n42.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n43xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n43.xml
        Test URI:ibm87n43.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0B84 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n43.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n44xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n44.xml
        Test URI:ibm87n44.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0BC3 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n44.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n45xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n45.xml
        Test URI:ibm87n45.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0BC9 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n45.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n46xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n46.xml
        Test URI:ibm87n46.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0BD6 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n46.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n47xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n47.xml
        Test URI:ibm87n47.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0C0D occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n47.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n48xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n48.xml
        Test URI:ibm87n48.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0C45 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n48.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n49xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n49.xml
        Test URI:ibm87n49.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0C49 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n49.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n50xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n50.xml
        Test URI:ibm87n50.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0C54 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n50.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n51xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n51.xml
        Test URI:ibm87n51.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0C81 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n51.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n52xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n52.xml
        Test URI:ibm87n52.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0C84 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n52.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n53xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n53.xml
        Test URI:ibm87n53.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0CC5 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n53.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n54xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n54.xml
        Test URI:ibm87n54.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0CC9 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n54.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n55xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n55.xml
        Test URI:ibm87n55.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0CD4 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n55.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n56xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n56.xml
        Test URI:ibm87n56.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0CD7 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n56.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n57xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n57.xml
        Test URI:ibm87n57.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0D04 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n57.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n58xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n58.xml
        Test URI:ibm87n58.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0D45 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n58.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n59xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n59.xml
        Test URI:ibm87n59.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0D49 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n59.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n60xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n60.xml
        Test URI:ibm87n60.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0D4E occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n60.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n61xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n61.xml
        Test URI:ibm87n61.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0D58 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n61.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n62xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n62.xml
        Test URI:ibm87n62.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0E3F occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n62.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n63xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n63.xml
        Test URI:ibm87n63.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0E3B occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n63.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n64xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n64.xml
        Test URI:ibm87n64.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0E4F occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n64.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n66xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n66.xml
        Test URI:ibm87n66.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0EBA occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n66.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n67xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n67.xml
        Test URI:ibm87n67.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0EBE occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n67.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n68xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n68.xml
        Test URI:ibm87n68.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0ECE occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n68.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n69xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n69.xml
        Test URI:ibm87n69.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0F1A occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n69.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n70xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n70.xml
        Test URI:ibm87n70.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0F36 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n70.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n71xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n71.xml
        Test URI:ibm87n71.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0F38 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n71.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n72xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n72.xml
        Test URI:ibm87n72.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0F3B occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n72.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n73xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n73.xml
        Test URI:ibm87n73.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0F3A occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n73.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n74xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n74.xml
        Test URI:ibm87n74.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0F70 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n74.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n75xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n75.xml
        Test URI:ibm87n75.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0F85 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n75.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n76xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n76.xml
        Test URI:ibm87n76.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0F8C occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n76.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n77xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n77.xml
        Test URI:ibm87n77.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0F96 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n77.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n78xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n78.xml
        Test URI:ibm87n78.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0F98 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n78.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n79xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n79.xml
        Test URI:ibm87n79.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0FB0 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n79.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n80xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n80.xml
        Test URI:ibm87n80.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0FB8 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n80.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n81xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n81.xml
        Test URI:ibm87n81.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x0FBA occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n81.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n82xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n82.xml
        Test URI:ibm87n82.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x20DD occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n82.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n83xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n83.xml
        Test URI:ibm87n83.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x20E2 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n83.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n84xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n84.xml
        Test URI:ibm87n84.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x3030 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n84.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p87ibm87n85xml() {
    /*
        Test ID:ibm-valid-P87-ibm87n85.xml
        Test URI:ibm87n85.xml
        Spec Sections:B.
        Description:Tests CombiningChar with an only legal per 5th edition character. The character #x309B occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm87n85.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p88ibm88n03xml() {
    /*
        Test ID:ibm-valid-P88-ibm88n03.xml
        Test URI:ibm88n03.xml
        Spec Sections:B.
        Description:Tests Digit with an only legal per 5th edition character. The character #x066A occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm88n03.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p88ibm88n04xml() {
    /*
        Test ID:ibm-valid-P88-ibm88n04.xml
        Test URI:ibm88n04.xml
        Spec Sections:B.
        Description:Tests Digit with an only legal per 5th edition character. The character #x06FA occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm88n04.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p88ibm88n05xml() {
    /*
        Test ID:ibm-valid-P88-ibm88n05.xml
        Test URI:ibm88n05.xml
        Spec Sections:B.
        Description:Tests Digit with an only legal per 5th edition character. The character #x0970 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm88n05.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p88ibm88n06xml() {
    /*
        Test ID:ibm-valid-P88-ibm88n06.xml
        Test URI:ibm88n06.xml
        Spec Sections:B.
        Description:Tests Digit with an only legal per 5th edition character. The character #x09F2 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm88n06.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p88ibm88n08xml() {
    /*
        Test ID:ibm-valid-P88-ibm88n08.xml
        Test URI:ibm88n08.xml
        Spec Sections:B.
        Description:Tests Digit with an only legal per 5th edition character. The character #x0AF0 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm88n08.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p88ibm88n09xml() {
    /*
        Test ID:ibm-valid-P88-ibm88n09.xml
        Test URI:ibm88n09.xml
        Spec Sections:B.
        Description:Tests Digit with an only legal per 5th edition character. The character #x0B70 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm88n09.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p88ibm88n10xml() {
    /*
        Test ID:ibm-valid-P88-ibm88n10.xml
        Test URI:ibm88n10.xml
        Spec Sections:B.
        Description:Tests Digit with an only legal per 5th edition character. The character #x0C65 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm88n10.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p88ibm88n11xml() {
    /*
        Test ID:ibm-valid-P88-ibm88n11.xml
        Test URI:ibm88n11.xml
        Spec Sections:B.
        Description:Tests Digit with an only legal per 5th edition character. The character #x0CE5 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm88n11.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p88ibm88n12xml() {
    /*
        Test ID:ibm-valid-P88-ibm88n12.xml
        Test URI:ibm88n12.xml
        Spec Sections:B.
        Description:Tests Digit with an only legal per 5th edition character. The character #x0CF0 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm88n12.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p88ibm88n13xml() {
    /*
        Test ID:ibm-valid-P88-ibm88n13.xml
        Test URI:ibm88n13.xml
        Spec Sections:B.
        Description:Tests Digit with an only legal per 5th edition character. The character #x0D70 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm88n13.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p88ibm88n14xml() {
    /*
        Test ID:ibm-valid-P88-ibm88n14.xml
        Test URI:ibm88n14.xml
        Spec Sections:B.
        Description:Tests Digit with an only legal per 5th edition character. The character #x0E5A occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm88n14.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p88ibm88n15xml() {
    /*
        Test ID:ibm-valid-P88-ibm88n15.xml
        Test URI:ibm88n15.xml
        Spec Sections:B.
        Description:Tests Digit with an only legal per 5th edition character. The character #x0EDA occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm88n15.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p88ibm88n16xml() {
    /*
        Test ID:ibm-valid-P88-ibm88n16.xml
        Test URI:ibm88n16.xml
        Spec Sections:B.
        Description:Tests Digit with an only legal per 5th edition character. The character #x0F2A occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm88n16.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p89ibm89n03xml() {
    /*
        Test ID:ibm-valid-P89-ibm89n03.xml
        Test URI:ibm89n03.xml
        Spec Sections:B.
        Description:Tests Extender with an only legal per 5th edition character. The character #x02D2 occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm89n03.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p89ibm89n04xml() {
    /*
        Test ID:ibm-valid-P89-ibm89n04.xml
        Test URI:ibm89n04.xml
        Spec Sections:B.
        Description:Tests Extender with an only legal per 5th edition character. The character #x03FE occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm89n04.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}

#[test]
fn ibmvalid_p89ibm89n05xml() {
    /*
        Test ID:ibm-valid-P89-ibm89n05.xml
        Test URI:ibm89n05.xml
        Spec Sections:B.
        Description:Tests Extender with an only legal per 5th edition character. The character #x065F occurs as the second character in the PITarget in the PI in the DTD.
    */

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/ibm89n05.xml")
            .unwrap()
            .as_str(),
        None,
    );

    assert!(parseresult.is_ok());
}
