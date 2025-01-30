// Support functions for smite tests

use std::rc::Rc;

use xrust::item::{Item, Node};
use xrust::namespace::NamespaceMap;
use xrust::parser::xml::{parse as xmlparse, parse_with_ns};
use xrust::qname_in::{Internment, QualifiedName};
use xrust::trees::smite_in::RNode;
use xrust::value::Value;
use xrust::xdmerror::Error;

#[allow(dead_code)]
pub fn make_empty_doc_in() -> RNode {
    RNode::new_document()
}
#[allow(dead_code)]
pub fn make_empty_doc_cooked_in() -> Result<RNode, Error> {
    Ok(RNode::new_document())
}

#[allow(dead_code)]
pub fn make_doc_in(n: QualifiedName, v: Value, intern: &mut Internment) -> RNode {
    let mut d = RNode::new_document();
    let mut child = d
        .new_element_in(n, intern)
        .expect("unable to create element");
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

#[allow(dead_code)]
pub fn make_sd_raw_in() -> RNode {
    let doc = RNode::new_document();
    xmlparse(doc.clone(),
             "<a id='a1'><b id='b1'><a id='a2'><b id='b2'/><b id='b3'/></a><a id='a3'><b id='b4'/><b id='b5'/></a></b><b id='b6'><a id='a4'><b id='b7'/><b id='b8'/></a><a id='a5'><b id='b9'/><b id='b10'/></a></b></a>",
             None).expect("unable to parse XML");
    doc
}
#[allow(dead_code)]
pub fn make_sd_cooked_in() -> Result<RNode, Error> {
    Ok(make_sd_raw_in())
}
#[allow(dead_code)]
pub fn make_sd_in() -> Item<RNode> {
    Item::Node(make_sd_raw_in())
}

#[allow(dead_code)]
pub fn make_from_str_in(s: &str) -> Result<RNode, Error> {
    let doc = RNode::new_document();
    xmlparse(doc.clone(), s, None)?;
    Ok(doc)
}

#[allow(dead_code)]
pub fn make_from_str_with_ns(s: &str) -> Result<(RNode, Rc<NamespaceMap>), Error> {
    let doc = RNode::new_document();
    let r = parse_with_ns(doc.clone(), s, None)?;
    Ok(r)
}
