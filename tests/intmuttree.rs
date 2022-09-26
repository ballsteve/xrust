use xrust::intmuttree::{NodeBuilder, RNode};
use xrust::qname::QualifiedName;

mod evaluate;
mod item_node;
mod item_value;

// Run the generic Item/Value tests
item_value_tests!(RNode);

fn make_empty_doc() -> RNode {
    NodeBuilder::new(NodeType::Document).build()
}

fn make_doc(n: QualifiedName, v: Value) -> RNode {
    let mut d = NodeBuilder::new(NodeType::Document).build();
    let mut child = NodeBuilder::new(NodeType::Element).name(n).build();
    d.push(child.clone());
    child.push(NodeBuilder::new(NodeType::Text).value(v).build());
    d
}

item_node_tests!(make_empty_doc, make_doc);
evaluate_tests!(make_empty_doc);
