use qualname::QName;
use xrust::item::{Node, NodeType};
use xrust::item_node_tests;
use xrust::item_value_tests;
use xrust::trees::smite::RNode;

mod node;
mod smite;

item_value_tests!(RNode);

// Item Node tests

item_node_tests!(smite::make_empty_doc, smite::make_doc, smite::make_sd_raw);

#[test]
fn node_get_attr_node() {
    node::get_attr_node::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
#[test]
fn node_to_xml_special_1() {
    node::to_xml_special_1::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
#[test]
fn node_to_xml_special_2() {
    node::to_xml_special_2::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
