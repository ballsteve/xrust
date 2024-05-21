use std::collections::HashMap;
use xrust::item::{Node, NodeType};
use xrust::item_node_tests;
use xrust::item_value_tests;
use xrust::parser::xml::{parse as xmlparse, parse_with_ns};
use xrust::pattern_tests;
use xrust::qname::QualifiedName;
use xrust::transform::context::{Context, ContextBuilder, StaticContext, StaticContextBuilder};
use xrust::transform_tests;
use xrust::trees::smite::{Node as SmiteNode, RNode};
use xrust::xdmerror::{Error, ErrorKind};
use xrust::xpath_tests;
use xrust::xslt_tests;
use xrust::testutils::xsltgeneric::generic_callable_posn_2;

type F = Box<dyn FnMut(&str) -> Result<(), Error>>;

fn make_empty_doc() -> RNode {
    Rc::new(SmiteNode::new())
}

fn make_doc(n: QualifiedName, v: Value) -> RNode {
    let mut d = Rc::new(SmiteNode::new());
    let mut child = d.new_element(n).expect("unable to create element");
    d.push(child.clone()).expect("unable to add element node");
    child
        .push(
            child
                .new_text(Rc::new(v))
                .expect("unable to create text node"),
        )
        .expect("unable to add text node");
    d
}

fn make_sd_raw() -> RNode {
    let doc = Rc::new(SmiteNode::new());
    xmlparse(doc.clone(),
             "<a id='a1'><b id='b1'><a id='a2'><b id='b2'/><b id='b3'/></a><a id='a3'><b id='b4'/><b id='b5'/></a></b><b id='b6'><a id='a4'><b id='b7'/><b id='b8'/></a><a id='a5'><b id='b9'/><b id='b10'/></a></b></a>",
             None, None).expect("unable to parse XML");
    doc
}
fn make_sd_cooked() -> Result<RNode, Error> {
    Ok(make_sd_raw())
}
fn make_sd() -> Item<RNode> {
    Item::Node(make_sd_raw())
}

fn make_from_str(s: &str) -> Result<RNode, Error> {
    let doc = Rc::new(SmiteNode::new());
    xmlparse(doc.clone(), s, None, None)?;
    Ok(doc)
}

fn make_from_str_with_ns(s: &str) -> Result<(RNode, Vec<HashMap<String, String>>), Error> {
    let doc = Rc::new(SmiteNode::new());
    let r = parse_with_ns(doc.clone(), s, None, None)?;
    Ok(r)
}

#[test]
fn xslt_callable_posn_2() {
    generic_callable_posn_2(
        make_from_str,
        make_from_str_with_ns,
        make_sd_cooked,
    ).expect("test failed")
}

item_value_tests!(RNode);
item_node_tests!(make_empty_doc, make_doc, make_sd_raw);
pattern_tests!(RNode, make_empty_doc, make_sd);
transform_tests!(RNode, make_empty_doc, make_doc);
xpath_tests!(RNode, make_empty_doc, make_sd);
xslt_tests!(make_from_str, make_empty_doc, make_from_str_with_ns);
