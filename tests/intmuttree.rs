use xrust::evaluate_tests;
use xrust::intmuttree::Document;
use xrust::intmuttree::{NodeBuilder, RNode};
use xrust::item::{Node, NodeType};
use xrust::item_node_tests;
use xrust::item_value_tests;
use xrust::qname::QualifiedName;
use xrust::xdmerror::Error;
use xrust::xpath_tests;
use xrust::xslt_tests;
use xrust::transcomb_tests;

// Run the generic Item/Value tests
item_value_tests!(RNode);
transcomb_tests!(RNode, make_empty_doc);

fn make_empty_doc() -> RNode {
    NodeBuilder::new(NodeType::Document).build()
}

fn make_doc(n: QualifiedName, v: Value) -> RNode {
    let mut d = NodeBuilder::new(NodeType::Document).build();
    let mut child = NodeBuilder::new(NodeType::Element).name(n).build();
    d.push(child.clone()).expect("unable to append child");
    child
        .push(NodeBuilder::new(NodeType::Text).value(v).build())
        .expect("unable to append child");
    d
}

fn make_sd() -> Rc<Item<RNode>> {
    let e = Document::try_from(
        "<a id='a1'><b id='b1'><a id='a2'><b id='b2'/><b id='b3'/></a><a id='a3'><b id='b4'/><b id='b5'/></a></b><b id='b6'><a id='a4'><b id='b7'/><b id='b8'/></a><a id='a5'><b id='b9'/><b id='b10'/></a></b></a>",
    )
    .expect("failed to parse XML")
    .content[0]
        .clone();
    let mut d = NodeBuilder::new(NodeType::Document).build();
    d.push(e).expect("unable to append node");
    Rc::new(Item::Node(d))
}

fn make_from_str(s: &str) -> Result<RNode, Error> {
    let e = Document::try_from(s).expect("failed to parse XML").content[0].clone();
    let mut d = NodeBuilder::new(NodeType::Document).build();
    d.push(e).expect("unable to append node");
    Ok(d)
}

item_node_tests!(make_empty_doc, make_doc);
evaluate_tests!(make_empty_doc);
xpath_tests!(make_empty_doc, make_sd);
xslt_tests!(make_from_str, make_empty_doc);
