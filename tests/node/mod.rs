//! Generic node tests

use std::rc::Rc;
use xrust::item::{Node, NodeType};
use xrust::output::OutputSpec;
use xrust::qname::QualifiedName;
use xrust::value::{Value, ValueBuilder, ValueData};
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

pub fn to_xml_special_1<N: Node, G>(make_doc: G) -> Result<(), Error>
where
    G: Fn() -> N,
{
    let mut sd = make_doc();
    let mut t = sd.new_element(Rc::new(QualifiedName::new(
        None,
        None,
        String::from("Test"),
    )))?;
    sd.push(t.clone())?;
    let a1 = sd.new_attribute(
        Rc::new(QualifiedName::new(None, None, String::from("attr"))),
        Rc::new(Value::from("'")),
    )?;
    t.add_attribute(a1)?;
    let t1 = sd
        .new_text(Rc::new(Value::from(
            r##"
        XML escape test: < > & ' "
"##,
        )))
        .expect("unable to create text node");
    t.push(t1).expect("unable to add text node");
    assert_eq!(
        t.to_xml(),
        r##"<Test attr='&apos;'>
        XML escape test: &lt; &gt; &amp; &apos; &quot;
</Test>"##
    );
    Ok(())
}

pub fn to_xml_special_2<N: Node, G>(make_doc: G) -> Result<(), Error>
where
    G: Fn() -> N,
{
    let mut sd = make_doc();
    let mut t = sd.new_element(Rc::new(QualifiedName::new(
        None,
        None,
        String::from("Test"),
    )))?;
    sd.push(t.clone())?;
    let a1 = sd.new_attribute(
        Rc::new(QualifiedName::new(None, None, String::from("attr"))),
        Rc::new(Value::from("'")),
    )?;
    t.add_attribute(a1)?;
    let t1 = sd
        .new_text(Rc::new(
            ValueBuilder::new()
                .value(ValueData::String(String::from(
                    r##"
        XML escape test: < > & ' "
"##,
                )))
                .output(OutputSpec::NoEscape)
                .build(),
        ))
        .expect("unable to create text node");
    t.push(t1).expect("unable to add text node");
    assert_eq!(
        t.to_xml(),
        r##"<Test attr='&apos;'>
        XML escape test: < > & ' "
</Test>"##
    );
    Ok(())
}
