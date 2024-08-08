/*

University of Edinburgh XML 1.0 4th edition errata test suite.

*/
use std::fs;
use std::rc::Rc;
use xrust::item::NodeType;
use xrust::parser::{xml, ParserConfig};
use xrust::trees::smite::Node as SmiteNode;
use xrust::Node;

#[test]
fn parser_config_namespace_nodes_1() {
    let doc = r#"<doc xmlns="namespace"
                  xmlns:a="namespace1"
                  xmlns:b="namespace2"
                  xmlns:c="namespace3"
                  xmlns:d="namespace4"
                  xmlns:e="namespace5"
             >
                 <element1/>
                 <element2 xmlns="namespace6"/>
                 <element3 xmlns:f="namespace7"/>
                 <element4 xmlns:f="namespace8"/>
                 <element5>
                     <element6/>
                 </element5>
             </doc>"#;

    let mut pc = ParserConfig::new();
    pc.namespace_nodes = false;

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(testxml, doc, Some(pc));

    assert!(parseresult.is_ok());

    let doc = parseresult.clone().unwrap().first_child().unwrap();
    let mut docchildren = doc
        .child_iter()
        .filter(|n| n.node_type() == NodeType::Element);
    let element1 = docchildren.next().unwrap();
    let element2 = docchildren.next().unwrap();
    let element3 = docchildren.next().unwrap();
    let element4 = docchildren.next().unwrap();
    let element5 = docchildren.next().unwrap();
    let element6 = element5
        .child_iter()
        .filter(|n| n.node_type() == NodeType::Element)
        .next()
        .unwrap();

    assert_eq!(doc.namespace_iter().count(), 7);
    assert_eq!(element1.namespace_iter().count(), 7);
    assert_eq!(element2.namespace_iter().count(), 7);
    assert_eq!(element3.namespace_iter().count(), 8);
    assert_eq!(element4.namespace_iter().count(), 8);
    assert_eq!(element5.namespace_iter().count(), 7);
    assert_eq!(element6.namespace_iter().count(), 7);
}

#[test]
fn parser_config_namespace_nodes_2() {
    let doc = r#"<doc xmlns="namespace"
                  xmlns:a="namespace1"
                  xmlns:b="namespace2"
                  xmlns:c="namespace3"
                  xmlns:d="namespace4"
                  xmlns:e="namespace5"
             >
                 <element1/>
                 <element2 xmlns="namespace6"/>
                 <element3 xmlns:f="namespace7"/>
                 <element4 xmlns:f="namespace8"/>
                 <element5>
                     <element6/>
                 </element5>
             </doc>"#;

    let mut pc = ParserConfig::new();
    pc.namespace_nodes = true;

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(testxml, doc, Some(pc));

    assert!(parseresult.is_ok());

    let doc = parseresult.clone().unwrap().first_child().unwrap();
    let mut docchildren = doc
        .child_iter()
        .filter(|n| n.node_type() == NodeType::Element);
    let element1 = docchildren.next().unwrap();
    let element2 = docchildren.next().unwrap();
    let element3 = docchildren.next().unwrap();
    let element4 = docchildren.next().unwrap();
    let element5 = docchildren.next().unwrap();
    let element6 = element5
        .child_iter()
        .filter(|n| n.node_type() == NodeType::Element)
        .next()
        .unwrap();

    assert_eq!(doc.namespace_iter().count(), 7);
    assert_eq!(element1.namespace_iter().count(), 7);
    assert_eq!(element2.namespace_iter().count(), 7);
    assert_eq!(element3.namespace_iter().count(), 8);
    assert_eq!(element4.namespace_iter().count(), 8);
    assert_eq!(element5.namespace_iter().count(), 7);
    assert_eq!(element6.namespace_iter().count(), 7);
}

#[test]
fn parser_config_namespace_nodes_3() {
    let doc = r#"<doc xmlns="namespace"
                  xmlns:a="namespace1"
                  xmlns:b="namespace2"
                  xmlns:c="namespace3"
                  xmlns:d="namespace4"
                  xmlns:e="namespace5"
             >
                 <element1/>
                 <element2 xmlns="namespace6"/>
                 <element3 xmlns:f="namespace7"/>
                 <element4 xmlns:f="namespace8"/>
                 <element5 xmlns="">
                     <element6/>
                 </element5>
             </doc>"#;

    let mut pc = ParserConfig::new();
    pc.namespace_nodes = true;

    let testxml = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(testxml, doc, Some(pc));

    assert!(parseresult.is_ok());

    let doc = parseresult.clone().unwrap().first_child().unwrap();
    let mut docchildren = doc
        .child_iter()
        .filter(|n| n.node_type() == NodeType::Element);
    let element1 = docchildren.next().unwrap();
    let element2 = docchildren.next().unwrap();
    let element3 = docchildren.next().unwrap();
    let element4 = docchildren.next().unwrap();
    let element5 = docchildren.next().unwrap();
    let element6 = element5
        .child_iter()
        .filter(|n| n.node_type() == NodeType::Element)
        .next()
        .unwrap();

    assert_eq!(doc.namespace_iter().count(), 7);
    assert_eq!(element1.namespace_iter().count(), 7);
    assert_eq!(element2.namespace_iter().count(), 7);
    assert_eq!(element3.namespace_iter().count(), 8);
    assert_eq!(element4.namespace_iter().count(), 8);
    assert_eq!(element5.namespace_iter().count(), 7);
    assert_eq!(element6.namespace_iter().count(), 7);
}


#[test]
fn parser_issue_94() {
    /*
        Github issue number 94

        Although rare, UTF-8 strings can start with a byte order mark, we strip this automatically.
    */

    let data = fs::read_to_string("tests/xml/issue-94.xml").unwrap();
    let source = Rc::new(SmiteNode::new());

    let parseresult = xml::parse(source.clone(), &data, None);

    assert!(parseresult.is_ok())

}
