use std::convert::TryFrom;
use xrust::qname::QualifiedName;
use xrust::rctree::{ADoc, ADocBuilder, ANodeBuilder, RANode, RBNode};

mod item_value;
mod item_node;

// Run the generic Item/Value tests
item_value_tests!(RBNode, RANode);

// Now run tests for Item/Node
//let ad = ADocBuilder::new()
//    .content(vec![
//	ANode::Element(QualifiedName::new(None, None, String::from("Test")), vec![], vec![])
//    ])
//    .build();
//let bd = BDoc::try_from(ad).expect("unable to convert ADoc to BDoc");

fn make_adoc() -> RANode {
    Rc::new(
	ANodeBuilder::new(NodeType::Document)
	    .build()
    )
}
fn make_bdoc(qn: QualifiedName, v: Value) -> RBNode {
    let mut n1 = Rc::new(
	ANodeBuilder::new(NodeType::Element)
	    .name(qn)
	    .build()
    );
    let n2 = Rc::new(
	ANodeBuilder::new(NodeType::Text)
	    .value(v)
	    .build()
    );
    n1.push(n2)
	.expect("unable to add node");
    RBNode::try_from(
	ADocBuilder::new()
	    .content(vec![n1])
	    .build()
    ).expect("unable to convert ADoc to BNode document")
}

item_node_mutable_tests!(make_adoc);
item_node_immutable_tests!(make_bdoc, RBNode);

