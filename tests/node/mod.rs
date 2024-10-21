//! Generic node tests

use std::rc::Rc;
use xrust::item::{Node, NodeType};
use xrust::qname::QualifiedName;
use xrust::value::Value;
use xrust::xdmerror::Error;

pub fn get_attr_node<N: Node, G>(make_doc: G) -> Result<(), Error>
where
    G: Fn() -> N,
{
    let mut sd = make_doc();
    let t = sd.new_element(Rc::new(QualifiedName::new(
        None,
        None,
        String::from("Test"),
    )))?;
    sd.push(t.clone())?;
    let a1 = sd.new_attribute(
        Rc::new(QualifiedName::new(None, None, String::from("role"))),
        Rc::new(Value::from("testing")),
    )?;
    t.add_attribute(a1)?;
    let a2 = sd.new_attribute(
        Rc::new(QualifiedName::new(None, None, String::from("phase"))),
        Rc::new(Value::from("one")),
    )?;
    t.add_attribute(a2)?;

    // NB. attributes could be returned in a different order
    assert!(
        sd.to_xml() == "<Test role='testing' phase='one'></Test>"
            || sd.to_xml() == "<Test phase='one' role='testing'></Test>"
    );

    match t.get_attribute_node(&QualifiedName::new(None, None, "role")) {
        Some(at) => {
            assert_eq!(at.node_type(), NodeType::Attribute);
            assert_eq!(at.to_string(), "testing");
            Ok(())
        }
        None => panic!("unable to find attribute \"role\""),
    }
}
