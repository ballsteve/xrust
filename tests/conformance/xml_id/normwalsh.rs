use qualname::{NamespaceMap, NamespacePrefix};
use std::fs;
use xrust::item::{Node, NodeType};
use xrust::parser::{ParseError, xml};
use xrust::trees::smite::RNode;

/*

    https://www.w3.org/XML/2005/01/xml-id/
    Test catalog submitted by Norm Walsh of Sun Microsystems

*/

#[test]
fn normal_001() {
    /*
        Test ID:normal_001
        Spec Sections:4
        Description:Check ID normalization
        Expected:xml:id on para is an ID (te st)
    */
    let testxml = RNode::new_document();
    let nm = NamespaceMap::new();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml_id/normwalsh/001_normalize.xml")
            .unwrap()
            .as_str(),
        Some(|p: &NamespacePrefix| {
            nm.namespace_uri(&Some(p.clone()))
                .ok_or(ParseError::MissingNameSpace)
        }),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.clone().unwrap().first_child().unwrap();
    let mut docchildren = doc
        .child_iter()
        .filter(|n| n.node_type() == NodeType::Element);
    let para = docchildren.next().unwrap();

    assert_eq!(
        para.attribute_iter().next().unwrap().to_string(),
        "te st".to_string()
    );
}

#[test]
fn undecl_001() {
    /*
        Test ID:undecl_001
        Spec Sections:4
        Description:Check that xml:id does not have to be declared
        Expected:xml:id on para is an ID (test)
    */
    let testxml = RNode::new_document();
    let nm = NamespaceMap::new();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml_id/normwalsh/002_undecl.xml")
            .unwrap()
            .as_str(),
        Some(|p: &NamespacePrefix| {
            nm.namespace_uri(&Some(p.clone()))
                .ok_or(ParseError::MissingNameSpace)
        }),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.clone().unwrap().first_child().unwrap();
    let mut docchildren = doc
        .child_iter()
        .filter(|n| n.node_type() == NodeType::Element);
    let para = docchildren.next().unwrap();

    assert_eq!(
        para.attribute_iter().next().unwrap().to_string(),
        "test".to_string()
    );
}

#[test]
fn declar_001() {
    /*
        Test ID:declar_001
        Spec Sections:4
        Description:Check that xml:id can be declared correctly with a DTD
        Expected:xml:id on para is an ID (id)
    */
    let testxml = RNode::new_document();
    let nm = NamespaceMap::new();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml_id/normwalsh/003_dtd.xml")
            .unwrap()
            .as_str(),
        Some(|p: &NamespacePrefix| {
            nm.namespace_uri(&Some(p.clone()))
                .ok_or(ParseError::MissingNameSpace)
        }),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.clone().unwrap().first_child().unwrap();
    let mut docchildren = doc
        .child_iter()
        .filter(|n| n.node_type() == NodeType::Element);
    let para = docchildren.next().unwrap();

    assert_eq!(
        para.attribute_iter().next().unwrap().to_string(),
        "id".to_string()
    );
}

#[test]
fn declar_002() {
    /*
        Test ID:declar_002
        Spec Sections:4
        Description:Check that xml:id can be declared correctly with a schema
        Expected:xml:id on para is an ID (id)
    */
    let testxml = RNode::new_document();
    let nm = NamespaceMap::new();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml_id/normwalsh/004_schema.xml")
            .unwrap()
            .as_str(),
        Some(|p: &NamespacePrefix| {
            nm.namespace_uri(&Some(p.clone()))
                .ok_or(ParseError::MissingNameSpace)
        }),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.clone().unwrap().first_child().unwrap();
    let mut docchildren = doc
        .child_iter()
        .filter(|n| n.node_type() == NodeType::Element);
    let para = docchildren.next().unwrap();

    assert_eq!(
        para.attribute_iter().next().unwrap().to_string(),
        "id".to_string()
    );
}

#[test]
fn baddcl_001() {
    /*
        Test ID:baddcl_001
        Spec Sections:4
        Description:Check that an incorrect DTD declaration is caught
        Expected:Must generate invalid declared type error.
    */
    let testxml = RNode::new_document();
    let nm = NamespaceMap::new();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml_id/normwalsh/005_errdtdbad.xml")
            .unwrap()
            .as_str(),
        Some(|p: &NamespacePrefix| {
            nm.namespace_uri(&Some(p.clone()))
                .ok_or(ParseError::MissingNameSpace)
        }),
    );

    assert!(parseresult.is_err());
}

#[test]
fn dupdup_001() {
    /*
        Test ID:dupdup_001
        Spec Sections:4
        Description:Test to see if duplicate IDs are detected.
        Expected:Should generate duplicate ID error; may report both elements.
    */
    let testxml = RNode::new_document();
    let nm = NamespaceMap::new();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml_id/normwalsh/005_errdup.xml")
            .unwrap()
            .as_str(),
        Some(|p: &NamespacePrefix| {
            nm.namespace_uri(&Some(p.clone()))
                .ok_or(ParseError::MissingNameSpace)
        }),
    );

    assert!(parseresult.is_err());
}

#[test]
#[ignore]
fn baddcl_002() {
    /*
        Test ID:baddcl_002
        Spec Sections:4
        Description:Check that an incorrect schema declaration is caught
        Expected:Must generate invalid declared type error; proper evaluation requires a schema-aware processor.
    */

    /* We need to support XSD to properly test this */

    let testxml = RNode::new_document();
    let nm = NamespaceMap::new();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml_id/normwalsh/006_errschemabad.xml")
            .unwrap()
            .as_str(),
        Some(|p: &NamespacePrefix| {
            nm.namespace_uri(&Some(p.clone()))
                .ok_or(ParseError::MissingNameSpace)
        }),
    );

    assert!(parseresult.is_err());
}

#[test]
fn dupdup_002() {
    /*
        Test ID:dupdup_002
        Spec Sections:4
        Description:Test to see if duplicate IDs are detected.
        Expected:Should generate duplicate ID error; may report both elements.
    */
    let testxml = RNode::new_document();
    let nm = NamespaceMap::new();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml_id/normwalsh/007_errdup.xml")
            .unwrap()
            .as_str(),
        Some(|p: &NamespacePrefix| {
            nm.namespace_uri(&Some(p.clone()))
                .ok_or(ParseError::MissingNameSpace)
        }),
    );

    assert!(parseresult.is_err());
}

#[test]
fn okchar_001() {
    /*
        Test ID:okchar_001
        Spec Sections:4
        Description:Check that an XML 1.0 document accepts 1.0 IDs
        Expected:xml:id on p is an ID (anid)
    */
    let testxml = RNode::new_document();
    let nm = NamespaceMap::new();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml_id/normwalsh/008_ok10.xml")
            .unwrap()
            .as_str(),
        Some(|p: &NamespacePrefix| {
            nm.namespace_uri(&Some(p.clone()))
                .ok_or(ParseError::MissingNameSpace)
        }),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.clone().unwrap().first_child().unwrap();
    let mut docchildren = doc
        .child_iter()
        .filter(|n| n.node_type() == NodeType::Element);
    let para = docchildren.next().unwrap();

    assert_eq!(
        para.attribute_iter().next().unwrap().to_string(),
        "anid".to_string()
    );
}

#[test]
fn okchar_002() {
    /*
        Test ID:okchar_002
        Spec Sections:4
        Description:Check that an XML 1.1 document accepts 1.1 IDs
        Expected:xml:id on p is an ID (id&#11264;ok)
        Note: Will fail if an XML 1.0 processor is used.
    */
    let testxml = RNode::new_document();
    let nm = NamespaceMap::new();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml_id/normwalsh/009_ok11.xml")
            .unwrap()
            .as_str(),
        Some(|p: &NamespacePrefix| {
            nm.namespace_uri(&Some(p.clone()))
                .ok_or(ParseError::MissingNameSpace)
        }),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.clone().unwrap().first_child().unwrap();
    let mut docchildren = doc
        .child_iter()
        .filter(|n| n.node_type() == NodeType::Element);
    let para = docchildren.next().unwrap();

    assert_eq!(
        para.attribute_iter().next().unwrap().to_string(),
        "idⰀok".to_string()
    );
}

#[test]
fn xref_001() {
    /*
        Test ID:xref___001
        Spec Sections:4
        Description:Check that IDREFs work
        Expected:id on para is an ID (id1) xml:id on para is an ID (id2)
    */
    let testxml = RNode::new_document();
    let nm = NamespaceMap::new();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml_id/normwalsh/010_okxref.xml")
            .unwrap()
            .as_str(),
        Some(|p: &NamespacePrefix| {
            nm.namespace_uri(&Some(p.clone()))
                .ok_or(ParseError::MissingNameSpace)
        }),
    );

    assert!(parseresult.is_ok());
}

#[test]
fn normal_002() {
    /*
        Test ID:normal_002
        Spec Sections:4
        Description:Check that an ID is normalized
        Expected:xml:id on p is an ID (anid)
    */
    let testxml = RNode::new_document();
    let nm = NamespaceMap::new();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml_id/normwalsh/011_oknormalize.xml")
            .unwrap()
            .as_str(),
        Some(|p: &NamespacePrefix| {
            nm.namespace_uri(&Some(p.clone()))
                .ok_or(ParseError::MissingNameSpace)
        }),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.clone().unwrap().first_child().unwrap();
    let mut docchildren = doc
        .child_iter()
        .filter(|n| n.node_type() == NodeType::Element);
    let p = docchildren.next().unwrap();

    assert_eq!(
        p.attribute_iter().next().unwrap().to_string(),
        "anid".to_string()
    );
}

#[test]
#[ignore]
fn normal_003() {
    /*
        Test ID:normal_003
        Spec Sections:4
        Description:Check that an ID is normalized
        Expected:xml:id on para is an ID (&#x0D; p2)
    */
    let testxml = RNode::new_document();
    let nm = NamespaceMap::new();
    let parseresult = xml::parse(
        testxml,
        fs::read_to_string("tests/conformance/xml_id/normwalsh/012_value.xml")
            .unwrap()
            .as_str(),
        Some(|p: &NamespacePrefix| {
            nm.namespace_uri(&Some(p.clone()))
                .ok_or(ParseError::MissingNameSpace)
        }),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.clone().unwrap().first_child().unwrap();
    let mut docchildren = doc
        .child_iter()
        .filter(|n| n.node_type() == NodeType::Element);
    let p = docchildren.next().unwrap();

    assert_eq!(
        p.attribute_iter().next().unwrap().to_string(),
        "anid".to_string()
    );
}
