//! Tests for transform module defined generically

use chrono::{Datelike, Local, Timelike};
use qualname::{NamespaceDeclaration, NamespaceMap, NamespacePrefix, NamespaceUri, NcName, QName};
use std::rc::Rc;
use xrust::item::{Item, Node, SequenceTrait};
use xrust::output::OutputSpec;
use xrust::pattern::Pattern;
use xrust::transform::callable::{ActualParameters, Callable, FormalParameters};
use xrust::transform::context::{Context, ContextBuilder, StaticContextBuilder};
use xrust::transform::numbers::{Level, Numbering};
use xrust::transform::template::Template;
use xrust::transform::{
    ArithmeticOperand, ArithmeticOperator, Axis, Grouping, KindTest, NameTest, NodeMatch, NodeTest,
    Order, Transform, WildcardOrName, WildcardOrNamespaceUri,
};
use xrust::value::{Operator, Value, ValueData};
use xrust::xdmerror::{Error, ErrorKind};

pub fn generic_tr_empty<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::<N>::Empty;
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 0);
    Ok(())
}
pub fn generic_tr_singleton_literal<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Literal(Item::<N>::Value(Rc::new(Value::from("this is a test"))));
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.to_string(), "this is a test");
    Ok(())
}
pub fn generic_tr_literal_element<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::LiteralElement(
        QName::from_local_name(NcName::try_from("Test").unwrap()),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "content",
        ))))),
    );
    let mydoc = make_empty_doc();
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let ctxt = ContextBuilder::new().result_document(mydoc).build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.to_xml(), "<Test>content</Test>");
    Ok(())
}

pub fn generic_tr_literal_element_nested<N: Node, G, H>(
    make_empty_doc: G,
    _: H,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::LiteralElement(
        QName::from_local_name(NcName::try_from("Test").unwrap()),
        Box::new(Transform::LiteralElement(
            QName::from_local_name(NcName::try_from("Level-1").unwrap()),
            Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                "content",
            ))))),
        )),
    );
    let mydoc = make_empty_doc();
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let ctxt = ContextBuilder::new().result_document(mydoc).build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.to_xml(), "<Test><Level-1>content</Level-1></Test>");
    Ok(())
}

pub fn generic_tr_element<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Element(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "Test",
        ))))),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "content",
        ))))),
    );
    let mydoc = make_empty_doc();
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let ctxt = ContextBuilder::new().result_document(mydoc).build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.to_xml(), "<Test>content</Test>");
    Ok(())
}

pub fn generic_tr_literal_text_1<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::LiteralText(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "special character: < less than",
        ))))),
        OutputSpec::Normal,
    );
    let mydoc = make_empty_doc();
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let ctxt = ContextBuilder::new().result_document(mydoc).build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.to_xml(), "special character: &lt; less than");
    Ok(())
}

pub fn generic_tr_literal_text_2<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::LiteralText(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "special character: < less than",
        ))))),
        OutputSpec::NoEscape,
    );
    let mydoc = make_empty_doc();
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let ctxt = ContextBuilder::new().result_document(mydoc).build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.to_xml(), "special character: < less than");
    Ok(())
}

pub fn generic_tr_literal_attribute<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::LiteralElement(
        QName::from_local_name(NcName::try_from("Test").unwrap()),
        Box::new(Transform::SequenceItems(vec![
            Transform::LiteralAttribute(
                QName::from_local_name(NcName::try_from("foo").unwrap()),
                Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                    "bar",
                ))))),
            ),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("content")))),
        ])),
    );
    let mydoc = make_empty_doc();
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let ctxt = ContextBuilder::new().result_document(mydoc).build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.to_xml(), "<Test foo='bar'>content</Test>");
    Ok(())
}

pub fn generic_tr_literal_comment<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::LiteralElement(
        QName::from_local_name(NcName::try_from("Test").unwrap()),
        Box::new(Transform::SequenceItems(vec![
            Transform::LiteralComment(Box::new(Transform::Literal(Item::<N>::Value(Rc::new(
                Value::from("bar"),
            ))))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("content")))),
        ])),
    );
    let mydoc = make_empty_doc();
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let ctxt = ContextBuilder::new().result_document(mydoc).build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.to_xml(), "<Test><!--bar-->content</Test>");
    Ok(())
}

pub fn generic_tr_literal_pi<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::LiteralElement(
        QName::from_local_name(NcName::try_from("Test").unwrap()),
        Box::new(Transform::SequenceItems(vec![
            Transform::LiteralProcessingInstruction(
                Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                    "thepi",
                ))))),
                Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                    "bar",
                ))))),
            ),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("content")))),
        ])),
    );
    let mydoc = make_empty_doc();
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let ctxt = ContextBuilder::new().result_document(mydoc).build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.to_xml(), "<Test><?thepi bar?>content</Test>");
    Ok(())
}

pub fn generic_tr_generate_id_ctxt<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::GenerateId(None);
    let sd = make_empty_doc();
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let ctxt = ContextBuilder::new().context(vec![Item::Node(sd)]).build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert!(seq.to_string().len() > 1);
    Ok(())
}

pub fn generic_tr_generate_id_2<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x1 = Transform::GenerateId(Some(Box::new(Transform::Step(NodeMatch {
        axis: Axis::Child,
        nodetest: NodeTest::Name(NameTest::Name(QName::from_local_name(
            NcName::try_from("Test1").unwrap(),
        ))),
    }))));
    let x2 = Transform::GenerateId(Some(Box::new(Transform::Step(NodeMatch {
        axis: Axis::Child,
        nodetest: NodeTest::Name(NameTest::Name(QName::from_local_name(
            NcName::try_from("Test2").unwrap(),
        ))),
    }))));
    let mut sd = make_empty_doc();
    let n1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Test1").unwrap()))
        .expect("unable to create element");
    sd.push(n1.clone()).expect("unable to append child");
    let n2 = sd
        .new_element(QName::from_local_name(NcName::try_from("Test2").unwrap()))
        .expect("unable to create element");
    sd.push(n2.clone()).expect("unable to append child");
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let ctxt = ContextBuilder::new().context(vec![Item::Node(sd)]).build();

    let seq1 = ctxt.dispatch(&mut stctxt, &x1).expect("evaluation failed");
    let seq2 = ctxt.dispatch(&mut stctxt, &x2).expect("evaluation failed");

    assert!(seq1.to_string().len() > 1);
    assert!(seq2.to_string().len() > 1);
    assert_ne!(seq1.to_string(), seq2.to_string());
    Ok(())
}

pub fn generic_tr_message_1<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let mut receiver = String::from("no message received");
    let x = Transform::LiteralElement(
        QName::from_local_name(NcName::try_from("Test").unwrap()),
        Box::new(Transform::SequenceItems(vec![
            Transform::Message(
                Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                    "bar",
                ))))),
                None,
                Box::new(Transform::Empty),
                Box::new(Transform::Empty),
            ),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("content")))),
        ])),
    );
    let mydoc = make_empty_doc();
    let ctxt = ContextBuilder::new().result_document(mydoc).build();
    let mut stctxt = StaticContextBuilder::new()
        .message(|m| {
            receiver = String::from(m);
            Ok(())
        })
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.to_xml(), "<Test>content</Test>");
    assert_eq!(receiver, "bar");
    Ok(())
}

pub fn generic_tr_message_2<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let mut messages: Vec<String> = vec![];
    let x = Transform::LiteralElement(
        QName::from_local_name(NcName::try_from("Test").unwrap()),
        Box::new(Transform::SequenceItems(vec![
            Transform::Message(
                Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                    "first message",
                ))))),
                None,
                Box::new(Transform::Empty),
                Box::new(Transform::Empty),
            ),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("content")))),
            Transform::Message(
                Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                    "second message",
                ))))),
                None,
                Box::new(Transform::Empty),
                Box::new(Transform::Empty),
            ),
        ])),
    );
    let mydoc = make_empty_doc();
    let ctxt = ContextBuilder::new().result_document(mydoc).build();
    let mut stctxt = StaticContextBuilder::new()
        .message(|m| {
            messages.push(String::from(m));
            Ok(())
        })
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.to_xml(), "<Test>content</Test>");
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0], "first message");
    assert_eq!(messages[1], "second message");
    Ok(())
}

pub fn generic_tr_message_term_1<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let mut receiver = String::from("no message received");
    let x = Transform::LiteralElement(
        QName::from_local_name(NcName::try_from("Test").unwrap()),
        Box::new(Transform::SequenceItems(vec![
            Transform::Message(
                Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                    "bar",
                ))))),
                None,
                Box::new(Transform::Empty),
                Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                    "yes",
                ))))),
            ),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("content")))),
        ])),
    );
    let mydoc = make_empty_doc();
    let ctxt = ContextBuilder::new().result_document(mydoc).build();
    let mut stctxt = StaticContextBuilder::new()
        .message(|m| {
            receiver = String::from(m);
            Ok(())
        })
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    match ctxt.dispatch(&mut stctxt, &x) {
        Ok(_) => panic!("evaluation succeeded when it should have failed"),
        Err(e) => {
            assert_eq!(e.kind, ErrorKind::Terminated);
            assert_eq!(e.message, "bar");
            assert_eq!(
                e.code.unwrap().to_string(),
                "{http://www.w3.org/2005/xqt-errors}XTMM9000"
            );
            Ok(())
        }
    }
}

pub fn generic_tr_set_attribute<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // Setup a source document
    let mut sd = make_empty_doc();
    let n = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(n.clone()).expect("unable to append child");

    let x = Transform::SetAttribute(
        QName::from_local_name(NcName::try_from("foo").unwrap()),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "bar",
        ))))),
    );
    let mydoc = make_empty_doc();
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let ctxt = ContextBuilder::new()
        .result_document(mydoc)
        .context(vec![Item::Node(n)])
        .build();
    let _ = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(sd.to_xml(), "<Test foo='bar'/>");
    Ok(())
}

pub fn generic_tr_copy_literal<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Copy(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "this is the original",
        ))))),
        Box::new(Transform::<N>::Empty),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "this is the original");
    Ok(())
}

pub fn generic_tr_copy_context_literal<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Copy(
        Box::new(Transform::ContextItem),
        Box::new(Transform::<N>::Empty),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ContextBuilder::new()
        .context(vec![Item::<N>::Value(Rc::new(Value::from(
            "this is the original",
        )))])
        .build()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "this is the original");
    Ok(())
}

pub fn generic_tr_copy_context_node<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // Setup a source document
    let mut sd = make_empty_doc();
    let mut n = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(n.clone()).expect("unable to append child");
    n.push(
        sd.new_text(Rc::new(Value::from("this is the original")))
            .expect("unable to create text node"),
    )
    .expect("unable to add text node");

    let x = Transform::Copy(
        Box::new(Transform::ContextItem),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "this is the copy",
        ))))),
    );

    let mydoc = make_empty_doc();
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let ctxt = ContextBuilder::new()
        .result_document(mydoc)
        .context(vec![Item::Node(n)])
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");

    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_xml(), "<Test>this is the copy</Test>");
    assert_eq!(sd.to_xml(), "<Test>this is the original</Test>");
    Ok(())
}

pub fn generic_tr_current_node<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // Setup a source document
    let mut sd = make_empty_doc();
    let mut n = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(n.clone()).expect("unable to append child");
    n.push(
        sd.new_text(Rc::new(Value::from("this is the original")))
            .expect("unable to create text node"),
    )
    .expect("unable to add text node");

    let x = Transform::CurrentItem;

    let mydoc = make_empty_doc();
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let ctxt = ContextBuilder::new()
        .result_document(mydoc)
        .context(vec![Item::Node(n.clone())])
        .context_item(Some(Item::Node(n.clone())))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");

    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_xml(), "<Test>this is the original</Test>");
    Ok(())
}

pub fn generic_tr_deep_copy<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // Setup a source document
    let mut sd = make_empty_doc();
    let mut n = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(n.clone()).expect("unable to append child");
    let mut u = sd
        .new_element(QName::from_local_name(NcName::try_from("inner").unwrap()))
        .expect("unable to create element");
    n.push(u.clone()).expect("unable to append child");
    u.push(
        sd.new_text(Rc::new(Value::from("this is the original")))
            .expect("unable to create text node"),
    )
    .expect("unable to add text node");

    let x = Transform::DeepCopy(Box::new(Transform::ContextItem));

    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let ctxt = ContextBuilder::new().context(vec![Item::Node(n)]).build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");

    assert_eq!(seq.len(), 1);
    assert_eq!(
        seq.to_xml(),
        "<Test><inner>this is the original</inner></Test>"
    );
    Ok(())
}

pub fn generic_tr_seq_of_literals<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::SequenceItems(vec![
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from("this is a test")))),
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from(1)))),
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from("end of test")))),
    ]);
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 3);
    assert_eq!(seq.to_string(), "this is a test1end of test");
    Ok(())
}

pub fn generic_tr_seq_of_seqs<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::SequenceItems(vec![
        Transform::SequenceItems(vec![
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("first sequence")))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from(1)))),
        ]),
        Transform::SequenceItems(vec![
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("second sequence")))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from(2)))),
        ]),
    ]);
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 4);
    assert_eq!(seq.to_string(), "first sequence1second sequence2");
    Ok(())
}

pub fn generic_tr_switch_when<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Switch(
        vec![
            (
                Transform::ValueComparison(
                    Operator::Equal,
                    Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                        1,
                    ))))),
                    Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                        2.0,
                    ))))),
                ),
                Transform::Literal(Item::<N>::Value(Rc::new(Value::from("comparison failed")))),
            ),
            (
                Transform::ValueComparison(
                    Operator::Equal,
                    Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                        1,
                    ))))),
                    Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                        1.0,
                    ))))),
                ),
                Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                    "comparison succeeded",
                )))),
            ),
            (
                Transform::ValueComparison(
                    Operator::Equal,
                    Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                        1,
                    ))))),
                    Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                        3.0,
                    ))))),
                ),
                Transform::Literal(Item::<N>::Value(Rc::new(Value::from("comparison failed")))),
            ),
        ],
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "otherwise clause",
        ))))),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.to_string(), "comparison succeeded");
    Ok(())
}

pub fn generic_tr_switch_otherwise<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Switch(
        vec![
            (
                Transform::ValueComparison(
                    Operator::Equal,
                    Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                        1,
                    ))))),
                    Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                        2.0,
                    ))))),
                ),
                Transform::Literal(Item::<N>::Value(Rc::new(Value::from("comparison failed")))),
            ),
            (
                Transform::ValueComparison(
                    Operator::Equal,
                    Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                        1,
                    ))))),
                    Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                        11.0,
                    ))))),
                ),
                Transform::Literal(Item::<N>::Value(Rc::new(Value::from("comparison failed")))),
            ),
            (
                Transform::ValueComparison(
                    Operator::Equal,
                    Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                        1,
                    ))))),
                    Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                        3.0,
                    ))))),
                ),
                Transform::Literal(Item::<N>::Value(Rc::new(Value::from("comparison failed")))),
            ),
        ],
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "otherwise clause",
        ))))),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.to_string(), "otherwise clause");
    Ok(())
}

pub fn generic_tr_loop_lit<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Loop(
        vec![(
            String::from("x"),
            Transform::SequenceItems(vec![
                Transform::Literal(Item::<N>::Value(Rc::new(Value::from("one")))),
                Transform::Literal(Item::<N>::Value(Rc::new(Value::from("two")))),
                Transform::Literal(Item::<N>::Value(Rc::new(Value::from("three")))),
            ]),
        )],
        Box::new(Transform::Concat(vec![
            Transform::VariableReference(String::from("x"), Rc::new(NamespaceMap::new())),
            Transform::VariableReference(String::from("x"), Rc::new(NamespaceMap::new())),
        ])),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.to_string(), "oneonetwotwothreethree");
    Ok(())
}

pub fn generic_tr_context_item<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::ContextItem;
    let c = Context::from(vec![Item::<N>::Value(Rc::new(Value::from(
        "the context item",
    )))]);
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = c.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "the context item");
    Ok(())
}

pub fn generic_tr_context_item_seq<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::SequenceItems(vec![Transform::ContextItem, Transform::ContextItem]);
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let c = Context::from(vec![Item::<N>::Value(Rc::new(Value::from(
        "the context item",
    )))]);
    let seq = c.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.len(), 2);
    assert_eq!(seq.to_string(), "the context itemthe context item");
    Ok(())
}

pub fn generic_tr_root<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // Setup a source document
    let mut sd = make_empty_doc();
    let mut n = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(n.clone()).expect("unable to append child");
    let l1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    n.push(l1.clone()).expect("unable to append child");

    let x = Transform::Root;

    // Now evaluate the combinator with <Level-1> as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(l1)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_xml(), "<Test><Level-1/></Test>");
    Ok(())
}

pub fn generic_tr_path_of_lits<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Compose(vec![
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from("step 1")))),
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from("step 2")))),
    ]);
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "step 2");
    Ok(())
}

pub fn generic_tr_step_child_1<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == child::node()
    let x = Transform::Step(NodeMatch {
        axis: Axis::Child,
        nodetest: NodeTest::Kind(KindTest::Any),
    });

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut n = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(n.clone()).expect("unable to append child");
    let l1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    n.push(l1.clone()).expect("unable to append child");

    // Now evaluate the combinator with <Test> as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(n)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_xml(), "<Level-1/>");
    Ok(())
}

pub fn generic_tr_step_child_many<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == child::node()
    let x = Transform::Step(NodeMatch {
        axis: Axis::Child,
        nodetest: NodeTest::Kind(KindTest::Any),
    });

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut n = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(n.clone()).expect("unable to append child");
    let mut l1_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    n.push(l1_1.clone()).expect("unable to append child");
    let t1 = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    l1_1.push(t1).expect("unable to append text node");
    let mut l1_2 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    n.push(l1_2.clone()).expect("unable to append child");
    let t2 = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    l1_2.push(t2).expect("unable to append text node");

    // Now evaluate the combinator with both <Level-1>s as the context items
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(l1_1), Item::Node(l1_2)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 2);
    assert_eq!(seq.to_xml(), "firstsecond");
    Ok(())
}

pub fn generic_tr_step_self<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == self::Level-1
    let x = Transform::Step(NodeMatch {
        axis: Axis::SelfAxis,
        nodetest: NodeTest::Name(NameTest::Name(QName::from_local_name(
            NcName::try_from("Level-1").unwrap(),
        ))),
    });

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut n = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(n.clone()).expect("unable to append child");
    let l1_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    n.push(l1_1.clone()).expect("unable to append child");
    let t1 = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    n.push(t1.clone()).expect("unable to append text node");
    let l1_2 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    n.push(l1_2.clone()).expect("unable to append child");
    let t2 = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    n.push(t2.clone()).expect("unable to append text node");
    let et = sd
        .new_element(QName::from_local_name(NcName::try_from("extra").unwrap()))
        .expect("unable to create element");
    n.push(et.clone()).expect("unable to append child");

    // Now evaluate the combinator with Test's children as the context items
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![
        Item::Node(l1_1),
        Item::Node(t1),
        Item::Node(l1_2),
        Item::Node(t2),
        Item::Node(et),
    ])
    .dispatch(&mut stctxt, &x)
    .expect("evaluation failed");
    assert_eq!(seq.len(), 2);
    assert_eq!(seq.to_xml(), "<Level-1/><Level-1/>");
    Ok(())
}

pub fn generic_tr_step_selfdoc_pos<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Step(NodeMatch {
        axis: Axis::SelfDocument,
        nodetest: NodeTest::Name(NameTest::Name(QName::from_local_name(
            NcName::try_from("Level-1").unwrap(),
        ))),
    });

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut n = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(n.clone()).expect("unable to append child");
    let l1_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    n.push(l1_1.clone()).expect("unable to append child");
    let t1 = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    n.push(t1.clone()).expect("unable to append text node");
    let l1_2 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    n.push(l1_2.clone()).expect("unable to append child");
    let t2 = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    n.push(t2.clone()).expect("unable to append text node");
    let et = sd
        .new_element(QName::from_local_name(NcName::try_from("extra").unwrap()))
        .expect("unable to create element");
    n.push(et.clone()).expect("unable to append child");

    // Now evaluate the combinator with Test's document node as the context items
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(sd)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    Ok(())
}

pub fn generic_tr_step_selfdoc_neg<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Step(NodeMatch {
        axis: Axis::SelfDocument,
        nodetest: NodeTest::Name(NameTest::Name(QName::from_local_name(
            NcName::try_from("Level-1").unwrap(),
        ))),
    });

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut n = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(n.clone()).expect("unable to append child");
    let l1_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    n.push(l1_1.clone()).expect("unable to append child");
    let t1 = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    n.push(t1.clone()).expect("unable to append text node");
    let l1_2 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    n.push(l1_2.clone()).expect("unable to append child");
    let t2 = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    n.push(t2.clone()).expect("unable to append text node");
    let et = sd
        .new_element(QName::from_local_name(NcName::try_from("extra").unwrap()))
        .expect("unable to create element");
    n.push(et.clone()).expect("unable to append child");

    // Now evaluate the combinator with Test's document element node as the context items
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(n)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 0);
    Ok(())
}

pub fn generic_tr_step_parent<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == parent::*
    let x = Transform::Step(NodeMatch {
        axis: Axis::Parent,
        nodetest: NodeTest::Kind(KindTest::Any),
    });

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut n = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(n.clone()).expect("unable to append child");
    let l1_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    n.push(l1_1.clone()).expect("unable to append child");
    let t1 = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    n.push(t1.clone()).expect("unable to append text node");
    let l1_2 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    n.push(l1_2.clone()).expect("unable to append child");
    let t2 = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    n.push(t2.clone()).expect("unable to append text node");
    let et = sd
        .new_element(QName::from_local_name(NcName::try_from("extra").unwrap()))
        .expect("unable to create element");
    n.push(et.clone()).expect("unable to append child");

    // Now evaluate the combinator with Test's children as the context items
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![
        Item::Node(l1_1),
        Item::Node(t1),
        Item::Node(l1_2),
        Item::Node(t2),
        Item::Node(et),
    ])
    .dispatch(&mut stctxt, &x)
    .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq[0].name().unwrap().to_string(), "Test");
    Ok(())
}

pub fn generic_tr_step_parentdoc_pos<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Step(NodeMatch {
        axis: Axis::ParentDocument,
        nodetest: NodeTest::Kind(KindTest::Any),
    });

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut n = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(n.clone()).expect("unable to append child");
    let l1_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    n.push(l1_1.clone()).expect("unable to append child");
    let t1 = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    n.push(t1.clone()).expect("unable to append text node");

    // Now evaluate the combinator with the root node as the context items
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(sd)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    Ok(())
}

pub fn generic_tr_step_parentdoc_neg<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Step(NodeMatch {
        axis: Axis::ParentDocument,
        nodetest: NodeTest::Kind(KindTest::Any),
    });

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let l1_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    t.push(l1_1.clone()).expect("unable to append child");
    let t1 = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    t.push(t1.clone()).expect("unable to append text node");

    // Now evaluate the combinator with the document element as the context items
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(t)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 0);
    Ok(())
}

pub fn generic_tr_step_descendant<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Step(NodeMatch {
        axis: Axis::Descendant,
        nodetest: NodeTest::Kind(KindTest::Any),
    });

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut l1_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    t.push(l1_1.clone()).expect("unable to append child");
    let t1 = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    t.push(t1.clone()).expect("unable to append text node");
    let mut l2_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-2").unwrap()))
        .expect("unable to create element");
    l1_1.push(l2_1.clone()).expect("unable to append child");
    let t2 = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    l2_1.push(t2.clone()).expect("unable to append text node");

    // Now evaluate the combinator with the document element as the context items
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(t)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 4);
    Ok(())
}

pub fn generic_tr_step_descendant_or_self<N: Node, G, H>(
    make_empty_doc: G,
    _: H,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Step(NodeMatch {
        axis: Axis::DescendantOrSelf,
        nodetest: NodeTest::Kind(KindTest::Any),
    });

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut l1_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    t.push(l1_1.clone()).expect("unable to append child");
    let t1 = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    t.push(t1.clone()).expect("unable to append text node");
    let mut l2_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-2").unwrap()))
        .expect("unable to create element");
    l1_1.push(l2_1.clone()).expect("unable to append child");
    let t2 = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    l2_1.push(t2.clone()).expect("unable to append text node");

    // Now evaluate the combinator with the document element as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(t)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 5);
    Ok(())
}

pub fn generic_tr_step_descendant_or_self_or_root<N: Node, G, H>(
    make_empty_doc: G,
    _: H,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Step(NodeMatch {
        axis: Axis::DescendantOrSelfOrRoot,
        nodetest: NodeTest::Kind(KindTest::Any),
    });

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut l1_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    t.push(l1_1.clone()).expect("unable to append child");
    let t1 = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    t.push(t1.clone()).expect("unable to append text node");
    let mut l2_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-2").unwrap()))
        .expect("unable to create element");
    l1_1.push(l2_1.clone()).expect("unable to append child");
    let t2 = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    l2_1.push(t2.clone()).expect("unable to append text node");

    // Now evaluate the combinator with the root node as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(sd)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 6);
    Ok(())
}

pub fn generic_tr_step_ancestor<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Step(NodeMatch {
        axis: Axis::Ancestor,
        nodetest: NodeTest::Kind(KindTest::Any),
    });

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut l1_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    t.push(l1_1.clone()).expect("unable to append child");
    let t1 = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    t.push(t1.clone()).expect("unable to append text node");
    let mut l2_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-2").unwrap()))
        .expect("unable to create element");
    l1_1.push(l2_1.clone()).expect("unable to append child");
    let t2 = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    l2_1.push(t2.clone()).expect("unable to append text node");

    // Now evaluate the combinator with the lowest node as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(t2)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 4);
    Ok(())
}

pub fn generic_tr_step_ancestor_or_self<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Step(NodeMatch {
        axis: Axis::AncestorOrSelf,
        nodetest: NodeTest::Kind(KindTest::Any),
    });

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut l1_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    t.push(l1_1.clone()).expect("unable to append child");
    let t1 = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    t.push(t1.clone()).expect("unable to append text node");
    let mut l2_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-2").unwrap()))
        .expect("unable to create element");
    l1_1.push(l2_1.clone()).expect("unable to append child");
    let t2 = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    l2_1.push(t2.clone()).expect("unable to append text node");

    // Now evaluate the combinator with the lowest node as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(t2)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 5);
    Ok(())
}

pub fn generic_tr_step_following_sibling<N: Node, G, H>(
    make_empty_doc: G,
    _: H,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == following-sibling::node()
    let x = Transform::Step(NodeMatch {
        axis: Axis::FollowingSibling,
        nodetest: NodeTest::Kind(KindTest::Any),
    });

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let l1_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    t.push(l1_1.clone()).expect("unable to append child");
    let t1 = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    t.push(t1.clone()).expect("unable to append text node");
    let l1_2 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    t.push(l1_2.clone()).expect("unable to append child");
    let t2 = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    t.push(t2.clone()).expect("unable to append text node");
    let et = sd
        .new_element(QName::from_local_name(NcName::try_from("extra").unwrap()))
        .expect("unable to create element");
    t.push(et.clone()).expect("unable to append child");

    // Now evaluate the combinator with Test's first child as the context items
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(l1_1)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 4);
    Ok(())
}

pub fn generic_tr_step_preceding_sibling<N: Node, G, H>(
    make_empty_doc: G,
    _: H,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == preceding-sibling::node()
    let x = Transform::Step(NodeMatch {
        axis: Axis::PrecedingSibling,
        nodetest: NodeTest::Kind(KindTest::Any),
    });

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let l1_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    t.push(l1_1.clone()).expect("unable to append child");
    let t1 = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    t.push(t1.clone()).expect("unable to append text node");
    let l1_2 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    t.push(l1_2.clone()).expect("unable to append child");
    let t2 = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    t.push(t2.clone()).expect("unable to append text node");
    let et = sd
        .new_element(QName::from_local_name(NcName::try_from("extra").unwrap()))
        .expect("unable to create element");
    t.push(et.clone()).expect("unable to append child");

    // Now evaluate the combinator with Test's last child as the context items
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(et)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 4);
    Ok(())
}

pub fn generic_tr_step_following<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == following::node()
    let x = Transform::Step(NodeMatch {
        axis: Axis::Following,
        nodetest: NodeTest::Kind(KindTest::Any),
    });

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut one = sd
        .new_element(QName::from_local_name(NcName::try_from("Left-1").unwrap()))
        .expect("unable to create element");
    t.push(one.clone()).expect("unable to append child");
    let mut two = sd
        .new_element(QName::from_local_name(NcName::try_from("Right-1").unwrap()))
        .expect("unable to create element");
    t.push(two.clone()).expect("unable to append child");
    let three = sd
        .new_element(QName::from_local_name(NcName::try_from("Left-2").unwrap()))
        .expect("unable to create element");
    one.push(three.clone()).expect("unable to append child");
    let mut four = sd
        .new_element(QName::from_local_name(NcName::try_from("Right-2").unwrap()))
        .expect("unable to create element");
    one.push(four.clone()).expect("unable to append child");
    let five = sd
        .new_element(QName::from_local_name(NcName::try_from("Left-2").unwrap()))
        .expect("unable to create element");
    two.push(five.clone()).expect("unable to append child");
    let six = sd
        .new_element(QName::from_local_name(NcName::try_from("Right-2").unwrap()))
        .expect("unable to create element");
    two.push(six.clone()).expect("unable to append child");

    let seven = sd
        .new_element(QName::from_local_name(NcName::try_from("Left-3").unwrap()))
        .expect("unable to create element");
    four.push(seven.clone()).expect("unable to append child");
    let eight = sd
        .new_element(QName::from_local_name(NcName::try_from("Right-3").unwrap()))
        .expect("unable to create element");
    four.push(eight.clone()).expect("unable to append child");

    // Now evaluate the combinator with lowest left node as the context items
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(seven)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 4);
    Ok(())
}

pub fn generic_tr_step_preceding<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == preceding::node()
    let x = Transform::Step(NodeMatch {
        axis: Axis::Preceding,
        nodetest: NodeTest::Kind(KindTest::Any),
    });

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut one = sd
        .new_element(QName::from_local_name(NcName::try_from("Left-1").unwrap()))
        .expect("unable to create element");
    t.push(one.clone()).expect("unable to append child");
    let mut two = sd
        .new_element(QName::from_local_name(NcName::try_from("Right-1").unwrap()))
        .expect("unable to create element");
    t.push(two.clone()).expect("unable to append child");
    let three = sd
        .new_element(QName::from_local_name(NcName::try_from("Left-2").unwrap()))
        .expect("unable to create element");
    one.push(three.clone()).expect("unable to append child");
    let mut four = sd
        .new_element(QName::from_local_name(NcName::try_from("Right-2").unwrap()))
        .expect("unable to create element");
    one.push(four.clone()).expect("unable to append child");
    let five = sd
        .new_element(QName::from_local_name(NcName::try_from("Left-2").unwrap()))
        .expect("unable to create element");
    two.push(five.clone()).expect("unable to append child");
    let six = sd
        .new_element(QName::from_local_name(NcName::try_from("Right-2").unwrap()))
        .expect("unable to create element");
    two.push(six.clone()).expect("unable to append child");

    let seven = sd
        .new_element(QName::from_local_name(NcName::try_from("Left-3").unwrap()))
        .expect("unable to create element");
    four.push(seven.clone()).expect("unable to append child");
    let eight = sd
        .new_element(QName::from_local_name(NcName::try_from("Right-3").unwrap()))
        .expect("unable to create element");
    four.push(eight.clone()).expect("unable to append child");

    // Now evaluate the combinator with last node as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(six)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 6);
    Ok(())
}

pub fn generic_tr_path_step_child<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == child::node()/child::node()
    let x = Transform::Compose(vec![
        Transform::Step(NodeMatch {
            axis: Axis::Child,
            nodetest: NodeTest::Kind(KindTest::Any),
        }),
        Transform::Step(NodeMatch {
            axis: Axis::Child,
            nodetest: NodeTest::Kind(KindTest::Any),
        }),
    ]);

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut l1_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    t.push(l1_1.clone()).expect("unable to append child");
    let t1 = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    l1_1.push(t1).expect("unable to append text node");
    let mut l1_2 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    t.push(l1_2.clone()).expect("unable to append child");
    let t2 = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    l1_2.push(t2).expect("unable to append text node");

    // Now evaluate the combinator with the Test element as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(t)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 2);
    assert_eq!(seq.to_xml(), "firstsecond");
    Ok(())
}

pub fn generic_tr_step_attribute<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == child::node()/attribute::*
    let x = Transform::Compose(vec![
        Transform::Step(NodeMatch {
            axis: Axis::Child,
            nodetest: NodeTest::Kind(KindTest::Any),
        }),
        Transform::Step(NodeMatch {
            axis: Axis::Attribute,
            nodetest: NodeTest::Name(NameTest::Wildcard(
                WildcardOrNamespaceUri::Wildcard,
                WildcardOrName::Wildcard,
            )),
        }),
    ]);

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut l1_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    t.push(l1_1.clone()).expect("unable to append child");
    let t1 = sd
        .new_text(Rc::new(Value::from("one")))
        .expect("unable to create text node");
    l1_1.push(t1).expect("unable to append text node");
    let a1 = sd
        .new_attribute(
            QName::from_local_name(NcName::try_from("name").unwrap()),
            Rc::new(Value::from("first")),
        )
        .expect("unable to create attribute node");
    l1_1.add_attribute(a1)
        .expect("unable to add attribute node");
    let mut l1_2 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    t.push(l1_2.clone()).expect("unable to append child");
    let t2 = sd
        .new_text(Rc::new(Value::from("two")))
        .expect("unable to create text node");
    l1_2.push(t2).expect("unable to append text node");
    let a2 = sd
        .new_attribute(
            QName::from_local_name(NcName::try_from("name").unwrap()),
            Rc::new(Value::from("second")),
        )
        .expect("unable to create attribute node");
    l1_2.add_attribute(a2)
        .expect("unable to add attribute node");

    // Now evaluate the combinator with the Test element as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(t)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 2);
    assert_eq!(seq.to_string(), "firstsecond");
    Ok(())
}

pub fn generic_tr_step_self_attribute_pos<N: Node, G, H>(
    make_empty_doc: G,
    _: H,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Step(NodeMatch {
        axis: Axis::SelfAttribute,
        nodetest: NodeTest::Kind(KindTest::Any),
    });

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut l1_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    t.push(l1_1.clone()).expect("unable to append child");
    let t1 = sd
        .new_text(Rc::new(Value::from("one")))
        .expect("unable to create text node");
    l1_1.push(t1).expect("unable to append text node");
    let a1 = sd
        .new_attribute(
            QName::from_local_name(NcName::try_from("name").unwrap()),
            Rc::new(Value::from("first")),
        )
        .expect("unable to create attribute node");
    l1_1.add_attribute(a1)
        .expect("unable to add attribute node");
    let mut l1_2 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    t.push(l1_2.clone()).expect("unable to append child");
    let t2 = sd
        .new_text(Rc::new(Value::from("two")))
        .expect("unable to create text node");
    l1_2.push(t2).expect("unable to append text node");
    let a2 = sd
        .new_attribute(
            QName::from_local_name(NcName::try_from("name").unwrap()),
            Rc::new(Value::from("second")),
        )
        .expect("unable to create attribute node");
    l1_2.add_attribute(a2.clone())
        .expect("unable to add attribute node");

    // Now evaluate the combinator with an attribute as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(a2)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "second");
    Ok(())
}

pub fn generic_tr_step_self_attribute_neg<N: Node, G, H>(
    make_empty_doc: G,
    _: H,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Step(NodeMatch {
        axis: Axis::SelfAttribute,
        nodetest: NodeTest::Kind(KindTest::Any),
    });

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut l1_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    t.push(l1_1.clone()).expect("unable to append child");
    let t1 = sd
        .new_text(Rc::new(Value::from("one")))
        .expect("unable to create text node");
    l1_1.push(t1).expect("unable to append text node");
    let a1 = sd
        .new_attribute(
            QName::from_local_name(NcName::try_from("name").unwrap()),
            Rc::new(Value::from("first")),
        )
        .expect("unable to create attribute node");
    l1_1.add_attribute(a1)
        .expect("unable to add attribute node");
    let mut l1_2 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    t.push(l1_2.clone()).expect("unable to append child");
    let t2 = sd
        .new_text(Rc::new(Value::from("two")))
        .expect("unable to create text node");
    l1_2.push(t2).expect("unable to append text node");
    let a2 = sd
        .new_attribute(
            QName::from_local_name(NcName::try_from("name").unwrap()),
            Rc::new(Value::from("second")),
        )
        .expect("unable to create attribute node");
    l1_2.add_attribute(a2.clone())
        .expect("unable to add attribute node");

    // Now evaluate the combinator with an element as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(l1_2)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 0);
    Ok(())
}

pub fn generic_tr_predicate<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == child::node()[child::node()]
    let x = Transform::Compose(vec![
        Transform::Step(NodeMatch {
            axis: Axis::Child,
            nodetest: NodeTest::Kind(KindTest::Any),
        }),
        Transform::Filter(Box::new(Transform::Step(NodeMatch {
            axis: Axis::Child,
            nodetest: NodeTest::Kind(KindTest::Any),
        }))),
    ]);

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut l1_1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-1").unwrap()))
        .expect("unable to create element");
    t.push(l1_1.clone()).expect("unable to append child");
    let t1 = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    l1_1.push(t1).expect("unable to append text node");
    let l1_2 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level-2").unwrap()))
        .expect("unable to create element");
    t.push(l1_2.clone()).expect("unable to append child");

    // Now evaluate the combinator with the Test element as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(t)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_xml(), "<Level-1>first</Level-1>");
    Ok(())
}

pub fn generic_tr_or_true<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Or(vec![
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from(0)))),
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from("false")))),
    ]);
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_bool(), true);
    Ok(())
}

pub fn generic_tr_or_false<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Or(vec![Transform::Literal(Item::<N>::Value(Rc::new(
        Value::from(0),
    )))]);
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_bool(), false);
    Ok(())
}

pub fn generic_tr_and_true<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::And(vec![
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from(1)))),
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from("false")))),
    ]);
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_bool(), true);
    Ok(())
}

pub fn generic_tr_and_false<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::And(vec![
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from("true")))),
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from(0)))),
    ]);
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_bool(), false);
    Ok(())
}

pub fn generic_tr_general_compare_true<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::GeneralComparison(
        Operator::Equal,
        Box::new(Transform::SequenceItems(vec![
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("true")))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("false")))),
        ])),
        Box::new(Transform::SequenceItems(vec![
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from(0)))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("true")))),
        ])),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_bool(), true);
    Ok(())
}

pub fn generic_tr_general_compare_false<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::GeneralComparison(
        Operator::Equal,
        Box::new(Transform::SequenceItems(vec![
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("true")))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("false")))),
        ])),
        Box::new(Transform::SequenceItems(vec![
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from(1)))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("foo")))),
        ])),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_bool(), false);
    Ok(())
}

pub fn generic_tr_value_compare_true<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::ValueComparison(
        Operator::Equal,
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "true",
        ))))),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "true",
        ))))),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_bool(), true);
    Ok(())
}

pub fn generic_tr_value_compare_false<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::ValueComparison(
        Operator::Equal,
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "true",
        ))))),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "false",
        ))))),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_bool(), false);
    Ok(())
}

pub fn generic_tr_range_empty<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Range(
        Box::new(Transform::<N>::Empty),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            10,
        ))))),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 0);
    Ok(())
}

pub fn generic_tr_range_many<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Range(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            1,
        ))))),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            10,
        ))))),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 10);
    assert_eq!(seq.to_string(), "12345678910");
    Ok(())
}

pub fn generic_tr_range_one<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Range(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            5,
        ))))),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            5,
        ))))),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "5");
    Ok(())
}

pub fn generic_tr_arithmetic_add<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Arithmetic(vec![
        ArithmeticOperand::new(
            ArithmeticOperator::Noop,
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from(5)))),
        ),
        ArithmeticOperand::new(
            ArithmeticOperator::Add,
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from(5)))),
        ),
    ]);
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "10");
    Ok(())
}

pub fn generic_tr_var_declare<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::VariableDeclaration(
        "foo".to_string(),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "bar",
        ))))),
        Box::new(Transform::VariableReference(
            "foo".to_string(),
            Rc::new(NamespaceMap::new()),
        )),
        Rc::new(NamespaceMap::new()),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "bar");
    Ok(())
}

pub fn generic_tr_union<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == child::a|child::b
    let x = Transform::Union(vec![
        Transform::Step(NodeMatch {
            axis: Axis::Child,
            nodetest: NodeTest::Name(NameTest::Name(QName::from_local_name(
                NcName::try_from("a").unwrap(),
            ))),
        }),
        Transform::Step(NodeMatch {
            axis: Axis::Child,
            nodetest: NodeTest::Name(NameTest::Name(QName::from_local_name(
                NcName::try_from("b").unwrap(),
            ))),
        }),
    ]);

    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let mut a = sd
        .new_element(QName::from_local_name(NcName::try_from("a").unwrap()))
        .expect("unable to create element");
    t.push(a.clone()).expect("unable to append child");
    let t_a = sd
        .new_text(Rc::new(Value::from("first")))
        .expect("unable to create text node");
    a.push(t_a).expect("unable to append text node");
    let mut b = sd
        .new_element(QName::from_local_name(NcName::try_from("b").unwrap()))
        .expect("unable to create element");
    t.push(b.clone()).expect("unable to append child");
    let t_b = sd
        .new_text(Rc::new(Value::from("second")))
        .expect("unable to create text node");
    b.push(t_b).expect("unable to append text node");
    let mut c = sd
        .new_element(QName::from_local_name(NcName::try_from("c").unwrap()))
        .expect("unable to create element");
    t.push(c.clone()).expect("unable to append child");
    let t_c = sd
        .new_text(Rc::new(Value::from("third")))
        .expect("unable to create text node");
    c.push(t_c).expect("unable to append text node");

    // Now evaluate the combinator with the Test element as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(t)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 2);
    assert_eq!(seq.to_xml(), "<a>first</a><b>second</b>");
    Ok(())
}

pub fn generic_tr_for_each<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to element node");
    sd.push(t.clone()).expect("unable to append child");
    let mut l1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level1").unwrap()))
        .expect("unable to element node");
    t.push(l1.clone()).expect("unable to append child");
    l1.push(
        sd.new_text(Rc::new(Value::from("one")))
            .expect("unable to create text node"),
    )
    .expect("unable to append text");
    let mut l2 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level1").unwrap()))
        .expect("unable to element node");
    t.push(l2.clone()).expect("unable to append child");
    l2.push(
        sd.new_text(Rc::new(Value::from("two")))
            .expect("unable to create text node"),
    )
    .expect("unable to append text");
    let mut l3 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level1").unwrap()))
        .expect("unable to element node");
    t.push(l3.clone()).expect("unable to append child");
    l3.push(
        sd.new_text(Rc::new(Value::from("three")))
            .expect("unable to create text node"),
    )
    .expect("unable to append text");

    // xsl:for-each select="/child::* /child::*" body == xsl:text "found a Level-1"
    let x = Transform::ForEach(
        None,
        Box::new(Transform::Compose(vec![
            Transform::Root,
            Transform::Step(NodeMatch {
                axis: Axis::Child,
                nodetest: NodeTest::Kind(KindTest::Any),
            }),
            Transform::Step(NodeMatch {
                axis: Axis::Child,
                nodetest: NodeTest::Kind(KindTest::Any),
            }),
        ])),
        Box::new(Transform::Literal(Item::Value(Rc::new(Value::from(
            "found a Level-1",
        ))))),
        vec![],
    );

    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ContextBuilder::new()
        .context(vec![Item::Node(sd)])
        .build()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 3);
    assert_eq!(
        seq.to_string(),
        "found a Level-1found a Level-1found a Level-1"
    );
    Ok(())
}
pub fn generic_tr_for_each_sort<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to element node");
    sd.push(t.clone()).expect("unable to append child");
    let mut l1 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level1").unwrap()))
        .expect("unable to element node");
    t.push(l1.clone()).expect("unable to append child");
    l1.push(
        sd.new_text(Rc::new(Value::from("one")))
            .expect("unable to create text node"),
    )
    .expect("unable to append text");
    let mut l2 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level1").unwrap()))
        .expect("unable to element node");
    t.push(l2.clone()).expect("unable to append child");
    l2.push(
        sd.new_text(Rc::new(Value::from("two")))
            .expect("unable to create text node"),
    )
    .expect("unable to append text");
    let mut l3 = sd
        .new_element(QName::from_local_name(NcName::try_from("Level1").unwrap()))
        .expect("unable to element node");
    t.push(l3.clone()).expect("unable to append child");
    l3.push(
        sd.new_text(Rc::new(Value::from("three")))
            .expect("unable to create text node"),
    )
    .expect("unable to append text");

    // xsl:for-each select="/child::* /child::*" body == xsl:text "found a Level-1"
    let x = Transform::ForEach(
        None,
        Box::new(Transform::Compose(vec![
            Transform::Root,
            Transform::Step(NodeMatch {
                axis: Axis::Child,
                nodetest: NodeTest::Kind(KindTest::Any),
            }),
            Transform::Step(NodeMatch {
                axis: Axis::Child,
                nodetest: NodeTest::Kind(KindTest::Any),
            }),
        ])),
        Box::new(Transform::ContextItem),
        vec![(Order::Ascending, Transform::ContextItem)],
    );

    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ContextBuilder::new()
        .context(vec![Item::Node(sd)])
        .build()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 3);
    assert_eq!(seq.to_string(), "onethreetwo");
    Ok(())
}

pub fn generic_tr_group_by_1<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // xsl:for-each-group select="1 to 50" group-by=". mod 10" body == xsl:text "group current-grouping-key size count(current-group)"
    let x = Transform::ForEach(
        Some(Grouping::By(vec![Transform::Arithmetic(vec![
            ArithmeticOperand::new(ArithmeticOperator::Noop, Transform::ContextItem),
            ArithmeticOperand::new(
                ArithmeticOperator::Modulo,
                Transform::Literal(Item::<N>::Value(Rc::new(Value::from(10)))),
            ),
        ])])),
        Box::new(Transform::Range(
            Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                1,
            ))))),
            Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                50,
            ))))),
        )),
        Box::new(Transform::LiteralElement(
            QName::from_local_name(NcName::try_from("group").unwrap()),
            Box::new(Transform::SequenceItems(vec![
                Transform::Literal(Item::Value(Rc::new(Value::from("key ")))),
                Transform::CurrentGroupingKey,
                Transform::Literal(Item::Value(Rc::new(Value::from(" #members ")))),
                Transform::Count(Box::new(Transform::CurrentGroup)),
            ])),
        )),
        vec![],
    );

    let resdoc = make_empty_doc();
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ContextBuilder::new()
        .result_document(resdoc)
        .build()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 10);
    Ok(())
    // the groups are not ordered, so it is difficult to test all of the groups are correct
    //assert_eq!(seq[0].to_string(), "key 0 #members 10")
}
pub fn generic_tr_group_by_sort_1<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // xsl:for-each-group select="1 to 50" group-by=". mod 10"
    // body == xsl:text "group current-grouping-key size count(current-group)"
    // sort == current-grouping-key()
    let x = Transform::ForEach(
        Some(Grouping::By(vec![Transform::Arithmetic(vec![
            ArithmeticOperand::new(ArithmeticOperator::Noop, Transform::ContextItem),
            ArithmeticOperand::new(
                ArithmeticOperator::Modulo,
                Transform::Literal(Item::<N>::Value(Rc::new(Value::from(10)))),
            ),
        ])])),
        Box::new(Transform::Range(
            Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                1,
            ))))),
            Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                50,
            ))))),
        )),
        Box::new(Transform::LiteralElement(
            QName::from_local_name(NcName::try_from("group").unwrap()),
            Box::new(Transform::SequenceItems(vec![
                Transform::Literal(Item::Value(Rc::new(Value::from("key ")))),
                Transform::CurrentGroupingKey,
                Transform::Literal(Item::Value(Rc::new(Value::from(" #members ")))),
                Transform::Count(Box::new(Transform::CurrentGroup)),
            ])),
        )),
        vec![(Order::Ascending, Transform::CurrentGroupingKey)],
    );

    let resdoc = make_empty_doc();
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ContextBuilder::new()
        .result_document(resdoc)
        .build()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 10);
    assert_eq!(seq[0].to_string(), "key 0 #members 5");
    assert_eq!(seq[1].to_string(), "key 1 #members 5");
    assert_eq!(seq[2].to_string(), "key 2 #members 5");
    assert_eq!(seq[3].to_string(), "key 3 #members 5");
    assert_eq!(seq[4].to_string(), "key 4 #members 5");
    assert_eq!(seq[5].to_string(), "key 5 #members 5");
    assert_eq!(seq[6].to_string(), "key 6 #members 5");
    assert_eq!(seq[7].to_string(), "key 7 #members 5");
    assert_eq!(seq[8].to_string(), "key 8 #members 5");
    assert_eq!(seq[9].to_string(), "key 9 #members 5");
    Ok(())
}

pub fn generic_tr_group_adjacent_1<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // xsl:for-each-group select="(a, a, b, c, c, c)" group-adjacent="." body == xsl:text "group current-grouping-key size count(current-group)"
    let x = Transform::ForEach(
        Some(Grouping::Adjacent(vec![Transform::ContextItem])),
        Box::new(Transform::SequenceItems(vec![
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("a")))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("a")))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("b")))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("c")))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("c")))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("c")))),
        ])),
        Box::new(Transform::LiteralElement(
            QName::from_local_name(NcName::try_from("group").unwrap()),
            Box::new(Transform::SequenceItems(vec![
                Transform::Literal(Item::Value(Rc::new(Value::from("key ")))),
                Transform::CurrentGroupingKey,
                Transform::Literal(Item::Value(Rc::new(Value::from(" #members ")))),
                Transform::Count(Box::new(Transform::CurrentGroup)),
            ])),
        )),
        vec![],
    );

    let resdoc = make_empty_doc();
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ContextBuilder::new()
        .result_document(resdoc)
        .build()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 3);
    Ok(())
    // the groups are not ordered, so it is difficult to test all of the groups are correct
    //assert_eq!(seq[0].to_string(), "key 0 #members 10")
}
pub fn generic_tr_group_adjacent_sort_1<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // xsl:for-each-group select="(a, a, b, c, c, c)" group-adjacent="." body == xsl:text "group current-grouping-key size count(current-group)"
    let x = Transform::ForEach(
        Some(Grouping::Adjacent(vec![Transform::ContextItem])),
        Box::new(Transform::SequenceItems(vec![
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("a")))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("a")))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("b")))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("c")))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("c")))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("c")))),
        ])),
        Box::new(Transform::LiteralElement(
            QName::from_local_name(NcName::try_from("group").unwrap()),
            Box::new(Transform::SequenceItems(vec![
                Transform::Literal(Item::Value(Rc::new(Value::from("key ")))),
                Transform::CurrentGroupingKey,
                Transform::Literal(Item::Value(Rc::new(Value::from(" #members ")))),
                Transform::Count(Box::new(Transform::CurrentGroup)),
            ])),
        )),
        vec![(Order::Ascending, Transform::CurrentGroupingKey)],
    );

    let resdoc = make_empty_doc();
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ContextBuilder::new()
        .result_document(resdoc)
        .build()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 3);
    assert_eq!(seq[0].to_string(), "key a #members 2");
    assert_eq!(seq[1].to_string(), "key b #members 1");
    assert_eq!(seq[2].to_string(), "key c #members 3");
    Ok(())
}

pub fn generic_tr_group_starting_with_1<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // xsl:for-each-group select="(a, a, b, c, c, c)" group-adjacent="." body == xsl:text "group current-grouping-key size count(current-group)"
    let x = Transform::ForEach(
        Some(Grouping::StartingWith(Box::new(
            Pattern::try_from(".[. eq 'a']").expect("unable to parse pattern"),
        ))),
        Box::new(Transform::SequenceItems(vec![
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("a")))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("b")))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("c")))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("a")))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("b")))),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("c")))),
        ])),
        Box::new(Transform::LiteralElement(
            QName::from_local_name(NcName::try_from("group").unwrap()),
            Box::new(Transform::SequenceItems(vec![
                Transform::Literal(Item::Value(Rc::new(Value::from(" #members ")))),
                Transform::Count(Box::new(Transform::CurrentGroup)),
            ])),
        )),
        vec![],
    );

    let resdoc = make_empty_doc();
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ContextBuilder::new()
        .result_document(resdoc)
        .build()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 2);
    Ok(())
    // the groups are not ordered, so it is difficult to test all of the groups are correct
    //assert_eq!(seq[0].to_string(), "key 0 #members 10")
}

pub fn generic_tr_apply_templates_builtins<N: Node, G, H>(
    make_empty_doc: G,
    _: H,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // Setup a source document
    let mut sd = make_empty_doc();
    let t = sd
        .new_text(Rc::new(Value::from("Test")))
        .expect("unable to text node");
    sd.push(t.clone()).expect("unable to append child");

    // Built-in template rule for "/"
    let x = Transform::ApplyTemplates(Box::new(Transform::Root), None, vec![]);
    let ctxt = ContextBuilder::new()
        .template(Template::new(
            // pattern "/",
            Pattern::try_from("/").expect("unable to create Pattern for \"/\""),
            Transform::ApplyTemplates(
                Box::new(Transform::Step(NodeMatch {
                    axis: Axis::Child,
                    nodetest: NodeTest::Kind(KindTest::Any),
                })),
                None,
                vec![],
            ), // body "apply-templates select=node()",
            None,    // priority
            vec![0], // import
            None,    // document order
            None,    // mode
            String::from("/"),
        ))
        .template(Template::new(
            // pattern child::text()
            Pattern::try_from("child::text()")
                .expect("unable to create Pattern for \"child::text()\""),
            Transform::ContextItem, // body value-of select='.'
            None,
            vec![0],
            None,
            None,
            String::from("child::text()"),
        ))
        .context(vec![Item::Node(sd)])
        .build();

    // Now Evaluate the combinator with the source document root node as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "Test");
    Ok(())
}

pub fn generic_tr_apply_templates_1<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create new element");
    sd.push(t.clone()).expect("unable to add node");
    let c = sd
        .new_text(Rc::new(Value::from("content")))
        .expect("unable to text node");
    t.push(c).expect("unable to append child");

    // Template rule for "Test", plus builtins
    let x = Transform::ApplyTemplates(Box::new(Transform::Root), None, vec![]);
    let ctxt = ContextBuilder::new()
        .template(Template::new(
            // pattern "Test"
            Pattern::try_from("child::Test").expect("unable to create Pattern for \"child::Test\""),
            Transform::SequenceItems(vec![
                Transform::Literal(Item::<N>::Value(Rc::new(Value::from("before ")))),
                Transform::ApplyTemplates(
                    Box::new(Transform::Step(NodeMatch {
                        axis: Axis::Child,
                        nodetest: NodeTest::Kind(KindTest::Any),
                    })),
                    None,
                    vec![],
                ),
                Transform::Literal(Item::<N>::Value(Rc::new(Value::from(" after")))),
            ]), // body "before", "apply-templates select=node()", "after"
            Some(0.0), // priority
            vec![0],   // import
            Some(1),   // document order
            None,      // mode
            String::from("child::Test"),
        ))
        .template(Template::new(
            // pattern "/",
            Pattern::try_from("/").expect("unable to create Pattern for \"/\""),
            Transform::ApplyTemplates(
                Box::new(Transform::Step(NodeMatch {
                    axis: Axis::Child,
                    nodetest: NodeTest::Kind(KindTest::Any),
                })),
                None,
                vec![],
            ), // body "apply-templates select=node()",
            None,    // priority
            vec![0], // import
            None,    // document order
            None,    // mode
            String::from("/"),
        ))
        .template(Template::new(
            // pattern child::text()
            Pattern::try_from("child::text()")
                .expect("unable to create Pattern for \"child::text()\""),
            Transform::ContextItem, // body value-of select='.'
            None,                   // priority
            vec![0],                // import
            None,                   // document order
            None,                   // mode
            String::from("child::text()"),
        ))
        .context(vec![Item::Node(sd)])
        .build();

    // Now Evaluate the combinator with the source document root node as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.len(), 3);
    assert_eq!(seq.to_string(), "before content after");
    Ok(())
}

pub fn generic_tr_apply_templates_2<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create new element");
    sd.push(t.clone()).expect("unable to add node");
    let c = sd
        .new_text(Rc::new(Value::from("content")))
        .expect("unable to text node");
    t.push(c).expect("unable to append child");

    // Template rule for "Test", plus builtins
    // Test template priorities
    let x = Transform::ApplyTemplates(Box::new(Transform::Root), None, vec![]);
    let ctxt = ContextBuilder::new()
        .template(Template::new(
            // pattern "Test"
            Pattern::try_from("child::Test").expect("unable to create Pattern for \"child::Test\""),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                "priority 1 template",
            )))),
            Some(1.0), // priority
            vec![0],   // import
            Some(1),   // document order
            None,      // mode
            String::from("child::Test"),
        ))
        .template(Template::new(
            // pattern "*"
            Pattern::try_from("child::*").expect("unable to create Pattern for \"child::*\""),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                "priority 0 template",
            )))),
            Some(0.0), // priority
            vec![0],   // import
            Some(2),   // document order
            None,      // mode
            String::from("child::*"),
        ))
        .template(Template::new(
            // pattern "/",
            Pattern::try_from("/").expect("unable to create Pattern for \"/\""),
            Transform::ApplyTemplates(
                Box::new(Transform::Step(NodeMatch {
                    axis: Axis::Child,
                    nodetest: NodeTest::Kind(KindTest::Any),
                })),
                None,
                vec![],
            ), // body "apply-templates select=node()",
            None,    // priority
            vec![0], // import
            None,    // document order
            None,    // mode
            String::from("/"),
        ))
        .template(Template::new(
            // pattern child::text()
            Pattern::try_from("child::text()")
                .expect("unable to create Pattern for \"child::text()\""),
            Transform::ContextItem, // body value-of select='.'
            None,                   // priority
            vec![0],                // import
            None,                   // document order
            None,                   // mode
            String::from("child::text()"),
        ))
        .context(vec![Item::Node(sd)])
        .build();

    // Now Evaluate the combinator with the source document root node as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "priority 1 template");
    Ok(())
}
// Multiple apply-templates selecting the same nodes, different modes
pub fn generic_tr_apply_templates_3<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create new element");
    sd.push(t.clone()).expect("unable to add node");
    let mut c1 = sd
        .new_element(QName::from_local_name(NcName::try_from("child").unwrap()))
        .expect("unable to create new element");
    c1.push(
        sd.new_text(Rc::new(Value::from("child 1")))
            .expect("unable to text node"),
    )
    .expect("unable to append child");
    t.push(c1).expect("unable to add child");
    let mut c2 = sd
        .new_element(QName::from_local_name(NcName::try_from("child").unwrap()))
        .expect("unable to create new element");
    c2.push(
        sd.new_text(Rc::new(Value::from("child 2")))
            .expect("unable to text node"),
    )
    .expect("unable to append child");
    t.push(c2).expect("unable to add child");

    // Template rule for "Test", plus builtins
    let x = Transform::ApplyTemplates(Box::new(Transform::Root), None, vec![]);
    let ctxt = ContextBuilder::new()
        .template(Template::new(
            // pattern "Test"
            Pattern::try_from("child::Test").expect("unable to create Pattern for \"child::Test\""),
            Transform::SequenceItems(vec![
                Transform::Literal(Item::<N>::Value(Rc::new(Value::from("before ")))),
                Transform::ApplyTemplates(
                    Box::new(Transform::Step(NodeMatch {
                        axis: Axis::Child,
                        nodetest: NodeTest::Kind(KindTest::Any),
                    })),
                    Some(QName::from_local_name(NcName::try_from("first").unwrap())),
                    vec![],
                ),
                Transform::Literal(Item::<N>::Value(Rc::new(Value::from(" middle ")))),
                Transform::ApplyTemplates(
                    Box::new(Transform::Step(NodeMatch {
                        axis: Axis::Child,
                        nodetest: NodeTest::Kind(KindTest::Any),
                    })),
                    Some(QName::from_local_name(NcName::try_from("second").unwrap())),
                    vec![],
                ),
                Transform::Literal(Item::<N>::Value(Rc::new(Value::from(" after")))),
            ]), // body "before", "apply-templates select=node()", "after"
            Some(0.0), // priority
            vec![0],   // import
            Some(1),   // document order
            None,      // mode
            String::from("child::Test"),
        ))
        .template(Template::new(
            // pattern "/",
            Pattern::try_from("/").expect("unable to create Pattern for \"/\""),
            Transform::ApplyTemplates(
                Box::new(Transform::Step(NodeMatch {
                    axis: Axis::Child,
                    nodetest: NodeTest::Kind(KindTest::Any),
                })),
                None,
                vec![],
            ), // body "apply-templates select=node()",
            None,    // priority
            vec![0], // import
            None,    // document order
            None,    // mode
            String::from("/"),
        ))
        .template(Template::new(
            // pattern child::text()
            Pattern::try_from("child::text()")
                .expect("unable to create Pattern for \"child::text()\""),
            Transform::ContextItem, // body value-of select='.'
            None,                   // priority
            vec![0],                // import
            None,                   // document order
            None,                   // mode
            String::from("child::text()"),
        ))
        .template(Template::new(
            // pattern child::node()
            Pattern::try_from("child::*").expect("unable to create Pattern for \"child::*\""),
            Transform::ApplyTemplates(
                Box::new(Transform::Step(NodeMatch {
                    axis: Axis::Child,
                    nodetest: NodeTest::Kind(KindTest::Any),
                })),
                None,
                vec![],
            ), // body "apply-templates select=node()",
            None,                                                             // priority
            vec![0],                                                          // import
            None,                                                             // document order
            Some(QName::from_local_name(NcName::try_from("first").unwrap())), // mode
            String::from("child::*"),
        ))
        .template(Template::new(
            // pattern child::node()
            Pattern::try_from("child::*").expect("unable to create Pattern for \"child::*\""),
            Transform::ApplyTemplates(
                Box::new(Transform::Step(NodeMatch {
                    axis: Axis::Child,
                    nodetest: NodeTest::Kind(KindTest::Any),
                })),
                None,
                vec![],
            ), // body "apply-templates select=node()",
            None,                                                              // priority
            vec![0],                                                           // import
            None,                                                              // document order
            Some(QName::from_local_name(NcName::try_from("second").unwrap())), // mode
            String::from("child::*"),
        ))
        .template(Template::new(
            // pattern child::text()
            Pattern::try_from("child::text()")
                .expect("unable to create Pattern for \"child::text()\""),
            Transform::ContextItem, // body value-of select='.'
            None,                   // priority
            vec![0],                // import
            None,                   // document order
            Some(QName::from_local_name(NcName::try_from("first").unwrap())), // mode
            String::from("child::text()"),
        ))
        .template(Template::new(
            // pattern child::text()
            Pattern::try_from("child::text()")
                .expect("unable to create Pattern for \"child::text()\""),
            Transform::ContextItem, // body value-of select='.'
            None,                   // priority
            vec![0],                // import
            None,                   // document order
            Some(QName::from_local_name(NcName::try_from("second").unwrap())), // mode
            String::from("child::text()"),
        ))
        .context(vec![Item::Node(sd)])
        .build();

    // Now Evaluate the combinator with the source document root node as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.len(), 7);
    assert_eq!(
        seq.to_string(),
        "before child 1child 2 middle child 1child 2 after"
    );
    Ok(())
}

pub fn generic_tr_apply_templates_import<N: Node, G, H>(
    make_empty_doc: G,
    _: H,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create new element");
    sd.push(t.clone()).expect("unable to add node");
    let c = sd
        .new_text(Rc::new(Value::from("content")))
        .expect("unable to text node");
    t.push(c).expect("unable to append child");

    // Template rule for "Test", an overridden rule, plus builtins
    // Test imported template
    let x = Transform::ApplyTemplates(Box::new(Transform::Root), None, vec![]);
    let ctxt = ContextBuilder::new()
        .template(Template::new(
            // pattern "Test"
            Pattern::try_from("child::Test").expect("unable to create Pattern for \"child::Test\""),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                "priority 1 template, import level 1",
            )))),
            Some(1.0),  // priority
            vec![0, 1], // import
            Some(1),    // document order
            None,       // mode
            String::from("child::Test"),
        ))
        .template(Template::new(
            // pattern "Test"
            Pattern::try_from("child::Test").expect("unable to create Pattern for \"child::Test\""),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                "priority 1 template, import level 0",
            )))),
            Some(1.0), // priority
            vec![0],   // import
            Some(2),   // document order
            None,      // mode
            String::from("child::Test"),
        ))
        .template(Template::new(
            // pattern "*"
            Pattern::try_from("child::*").expect("unable to create Pattern for \"child::*\""),
            Transform::ApplyTemplates(
                Box::new(Transform::Step(NodeMatch {
                    axis: Axis::Child,
                    nodetest: NodeTest::Kind(KindTest::Any),
                })),
                None,
                vec![],
            ), // body "apply-templates select=node()",
            None,    // priority
            vec![0], // import
            None,    // document order
            None,    // mode
            String::from("child::*"),
        ))
        .template(Template::new(
            // pattern "/",
            Pattern::try_from("/").expect("unable to create Pattern for \"/\""),
            Transform::ApplyTemplates(
                Box::new(Transform::Step(NodeMatch {
                    axis: Axis::Child,
                    nodetest: NodeTest::Kind(KindTest::Any),
                })),
                None,
                vec![],
            ), // body "apply-templates select=node()",
            None,    // priority
            vec![0], // import
            None,    // document order
            None,    // mode
            String::from("/"),
        ))
        .template(Template::new(
            // pattern child::text()
            Pattern::try_from("child::text()")
                .expect("unable to create Pattern for \"child::text()\""),
            Transform::ContextItem, // body value-of select='.'
            None,                   // priority
            vec![0],                // import
            None,                   // document order
            None,                   // mode
            String::from("child::text()"),
        ))
        .context(vec![Item::Node(sd)])
        .build();

    // Now Evaluate the combinator with the source document root node as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "priority 1 template, import level 0");
    Ok(())
}

pub fn generic_tr_apply_templates_next_match<N: Node, G, H>(
    make_empty_doc: G,
    _: H,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create new element");
    sd.push(t.clone()).expect("unable to add node");
    let c = sd
        .new_text(Rc::new(Value::from("content")))
        .expect("unable to text node");
    t.push(c).expect("unable to append child");

    // Template rule for "Test", an overridden rule, plus builtins
    let x = Transform::ApplyTemplates(Box::new(Transform::Root), None, vec![]);
    let ctxt = ContextBuilder::new()
        .template(Template::new(
            // pattern "Test"
            Pattern::try_from("child::Test").expect("unable to create Pattern for \"child::Test\""),
            Transform::SequenceItems(vec![
                Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                    "priority 1 template",
                )))),
                Transform::NextMatch,
            ]),
            Some(1.0), // priority
            vec![0],   // import
            Some(1),   // document order
            None,      // mode
            String::from("child::Test"),
        ))
        .template(Template::new(
            // pattern "Test"
            Pattern::try_from("child::Test").expect("unable to create Pattern for \"child::Test\""),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                "priority 0 template",
            )))),
            Some(0.0), // priority
            vec![0],   // import
            Some(2),   // document order
            None,      // mode
            String::from("child::Test"),
        ))
        .template(Template::new(
            // pattern "*"
            Pattern::try_from("child::*").expect("unable to create Pattern for \"child::*\""),
            Transform::ApplyTemplates(
                Box::new(Transform::Step(NodeMatch {
                    axis: Axis::Child,
                    nodetest: NodeTest::Kind(KindTest::Any),
                })),
                None,
                vec![],
            ), // body "apply-templates select=node()",
            None,    // priority
            vec![0], // import
            None,    // document order
            None,    // mode
            String::from("child::*"),
        ))
        .template(Template::new(
            // pattern "/",
            Pattern::try_from("/").expect("unable to create Pattern for \"/\""),
            Transform::ApplyTemplates(
                Box::new(Transform::Step(NodeMatch {
                    axis: Axis::Child,
                    nodetest: NodeTest::Kind(KindTest::Any),
                })),
                None,
                vec![],
            ), // body "apply-templates select=node()",
            None,    // priority
            vec![0], // import
            None,    // document order
            None,    // mode
            String::from("/"),
        ))
        .template(Template::new(
            // pattern child::text()
            Pattern::try_from("child::text()")
                .expect("unable to create Pattern for \"child::text()\""),
            Transform::ContextItem, // body value-of select='.'
            None,                   // priority
            vec![0],                // import
            None,                   // document order
            None,                   // mode
            String::from("child::text()"),
        ))
        .context(vec![Item::Node(sd)])
        .build();

    // Now Evaluate the combinator with the source document root node as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.to_string(), "priority 1 templatepriority 0 template");
    Ok(())
}

pub fn generic_tr_apply_templates_mode<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create new element");
    sd.push(t.clone()).expect("unable to add node");
    let c = sd
        .new_text(Rc::new(Value::from("content")))
        .expect("unable to text node");
    t.push(c).expect("unable to append child");

    // Template rule for "Test", an overridden rule, plus builtins
    let x = Transform::ApplyTemplates(Box::new(Transform::Root), None, vec![]);
    let ctxt = ContextBuilder::new()
        .template(Template::new(
            // pattern "Test"
            Pattern::try_from("child::Test").expect("unable to create Pattern for \"child::Test\""),
            Transform::SequenceItems(vec![
                Transform::Literal(Item::<N>::Value(Rc::new(Value::from("modeless template")))),
                Transform::NextMatch,
            ]),
            Some(1.0), // priority
            vec![0],   // import
            Some(1),   // document order
            None,      // mode
            String::from("child::Test"),
        ))
        .template(Template::new(
            // pattern "Test"
            Pattern::try_from("child::Test").expect("unable to create Pattern for \"child::Test\""),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                "mode 'modetest' template",
            )))),
            Some(0.0), // priority
            vec![0],   // import
            Some(2),   // document order
            Some(QName::from_local_name(
                NcName::try_from("modetest").unwrap(),
            )), // mode
            String::from("child::Test"),
        ))
        .template(Template::new(
            // pattern "*"
            Pattern::try_from("child::*").expect("unable to create Pattern for \"child::*\""),
            Transform::ApplyTemplates(
                Box::new(Transform::Step(NodeMatch {
                    axis: Axis::Child,
                    nodetest: NodeTest::Kind(KindTest::Any),
                })),
                None,
                vec![],
            ), // body "apply-templates select=node()",
            None,    // priority
            vec![0], // import
            None,    // document order
            None,    // mode
            String::from("child::*"),
        ))
        .template(Template::new(
            // pattern "/",
            Pattern::try_from("/").expect("unable to create Pattern for \"/\""),
            Transform::SequenceItems(vec![
                Transform::ApplyTemplates(
                    Box::new(Transform::Step(NodeMatch {
                        axis: Axis::Child,
                        nodetest: NodeTest::Kind(KindTest::Any),
                    })),
                    None,
                    vec![],
                ),
                Transform::ApplyTemplates(
                    Box::new(Transform::Step(NodeMatch {
                        axis: Axis::Child,
                        nodetest: NodeTest::Kind(KindTest::Any),
                    })),
                    Some(QName::from_local_name(
                        NcName::try_from("modetest").unwrap(),
                    )),
                    vec![],
                ),
            ]), // body "apply-templates select=node()", "apply-templates select=node() mode='modetest'"
            Some(1.0), // priority
            vec![0],   // import
            None,      // document order
            None,      // mode
            String::from("/"),
        ))
        .template(Template::new(
            // pattern child::text()
            Pattern::try_from("child::text()")
                .expect("unable to create Pattern for \"child::text()\""),
            Transform::ContextItem, // body value-of select='.'
            None,                   // priority
            vec![0],                // import
            None,                   // document order
            None,                   // mode
            String::from("child::text()"),
        ))
        .context(vec![Item::Node(sd)])
        .build();

    // Now Evaluate the combinator with the source document root node as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.to_string(), "modeless templatemode 'modetest' template");
    Ok(())
}

// apply-templates, 1 sort key, string sort key value
pub fn generic_tr_apply_templates_sort_1<N: Node, G, H>(
    make_empty_doc: G,
    _: H,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create new element");
    sd.push(t.clone()).expect("unable to add node");
    let mut c1 = sd
        .new_element(QName::from_local_name(NcName::try_from("child").unwrap()))
        .expect("unable to create element");
    let tx1 = sd
        .new_text(Rc::new(Value::from("one")))
        .expect("unable to text node");
    c1.push(tx1).expect("unable to append child");
    t.push(c1).expect("unable to append child");
    let mut c2 = sd
        .new_element(QName::from_local_name(NcName::try_from("child").unwrap()))
        .expect("unable to create element");
    let tx2 = sd
        .new_text(Rc::new(Value::from("two")))
        .expect("unable to text node");
    c2.push(tx2).expect("unable to append child");
    t.push(c2).expect("unable to append child");
    let mut c3 = sd
        .new_element(QName::from_local_name(NcName::try_from("child").unwrap()))
        .expect("unable to create element");
    let tx3 = sd
        .new_text(Rc::new(Value::from("three")))
        .expect("unable to text node");
    c3.push(tx3).expect("unable to append child");
    t.push(c3).expect("unable to append child");

    // Template rule for "Test", an overridden rule, plus builtins
    let x = Transform::ApplyTemplates(Box::new(Transform::Root), None, vec![]);
    let ctxt = ContextBuilder::new()
        .template(Template::new(
            // pattern "Test"
            Pattern::try_from("child::Test").expect("unable to create Pattern for \"child::Test\""),
            Transform::ApplyTemplates(
                Box::new(Transform::Step(NodeMatch {
                    axis: Axis::Child,
                    nodetest: NodeTest::Kind(KindTest::Any),
                })),
                None,
                vec![(Order::Ascending, Transform::ContextItem)],
            ), // body "apply-templates select=node() sort",
            Some(1.0), // priority
            vec![0],   // import
            Some(1),   // document order
            None,      // mode
            String::from("child::Test"),
        ))
        .template(Template::new(
            // pattern "*"
            Pattern::try_from("child::*").expect("unable to create Pattern for \"child::*\""),
            Transform::ApplyTemplates(
                Box::new(Transform::Step(NodeMatch {
                    axis: Axis::Child,
                    nodetest: NodeTest::Kind(KindTest::Any),
                })),
                None,
                vec![],
            ), // body "apply-templates select=node()",
            None,    // priority
            vec![0], // import
            None,    // document order
            None,    // mode
            String::from("child::*"),
        ))
        .template(Template::new(
            // pattern "/",
            Pattern::try_from("/").expect("unable to create Pattern for \"/\""),
            Transform::ApplyTemplates(
                Box::new(Transform::Step(NodeMatch {
                    axis: Axis::Child,
                    nodetest: NodeTest::Kind(KindTest::Any),
                })),
                None,
                vec![],
            ), // body "apply-templates select=node()", "apply-templates select=node()"
            Some(1.0), // priority
            vec![0],   // import
            None,      // document order
            None,      // mode
            String::from("/"),
        ))
        .template(Template::new(
            // pattern child::text()
            Pattern::try_from("child::text()")
                .expect("unable to create Pattern for \"child::text()\""),
            Transform::ContextItem, // body value-of select='.'
            None,                   // priority
            vec![0],                // import
            None,                   // document order
            None,                   // mode
            String::from("child::text()"),
        ))
        .context(vec![Item::Node(sd)])
        .build();

    // Now Evaluate the combinator with the source document root node as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.to_string(), "onethreetwo");
    Ok(())
}

pub fn generic_tr_position<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == position()
    // NB. rust indexes start at 0, whereas XPath positions start at 1

    let x = Transform::Position;
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ContextBuilder::new()
        .context(vec![
            Item::<N>::Value(Rc::new(Value::from("one"))),
            Item::<N>::Value(Rc::new(Value::from("two"))),
            Item::<N>::Value(Rc::new(Value::from("three"))),
            Item::<N>::Value(Rc::new(Value::from("four"))),
        ])
        .index(2)
        .build()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "3");
    Ok(())
}

pub fn generic_tr_last<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == last()
    // NB. rust indexes start at 0, whereas XPath positions start at 1

    let x = Transform::Last;
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ContextBuilder::new()
        .context(vec![
            Item::<N>::Value(Rc::new(Value::from("one"))),
            Item::<N>::Value(Rc::new(Value::from("two"))),
            Item::<N>::Value(Rc::new(Value::from("three"))),
            Item::<N>::Value(Rc::new(Value::from("four"))),
        ])
        .index(2)
        .build()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "4");
    Ok(())
}

pub fn generic_tr_count_0<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == count()

    let x = Transform::Count(Box::new(Transform::Empty));
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ContextBuilder::new()
        .context(vec![
            Item::<N>::Value(Rc::new(Value::from("one"))),
            Item::<N>::Value(Rc::new(Value::from("two"))),
            Item::<N>::Value(Rc::new(Value::from("three"))),
            Item::<N>::Value(Rc::new(Value::from("four"))),
        ])
        .build()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "0");
    Ok(())
}

pub fn generic_tr_count_1<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == count()

    let x = Transform::Count(Box::new(Transform::SequenceItems(vec![
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from("abc")))),
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from(1)))),
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from("foo")))),
    ])));
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ContextBuilder::new()
        .context(vec![
            Item::<N>::Value(Rc::new(Value::from("one"))),
            Item::<N>::Value(Rc::new(Value::from("two"))),
            Item::<N>::Value(Rc::new(Value::from("three"))),
            Item::<N>::Value(Rc::new(Value::from("four"))),
        ])
        .index(2)
        .build()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "3");
    Ok(())
}

pub fn generic_tr_localname_0<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let l1 = sd
        .new_element(QName::new_from_parts(
            NcName::try_from("Level-1").unwrap(),
            Some(NamespaceUri::try_from("urn:test-example.com").unwrap()),
        ))
        .expect("unable to create element");
    t.push(l1.clone()).expect("unable to append child");

    let x = Transform::LocalName(Some(Box::new(Transform::ContextItem)));

    // Now evaluate the combinator with <Level-1> as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(l1)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_xml(), "Level-1");
    Ok(())
}

pub fn generic_tr_name_0<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let l1 = sd
        .new_element(QName::new_from_parts(
            NcName::try_from("Level-1").unwrap(),
            Some(NamespaceUri::try_from("urn:test-example.com").unwrap()),
        ))
        .expect("unable to create element");
    t.push(l1.clone()).expect("unable to append child");

    let x = Transform::Name(Some(Box::new(Transform::ContextItem)));

    // Now evaluate the combinator with <Level-1> as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(l1)]).dispatch(&mut stctxt, &x);
    // This is expected to fail since no namespaces have been declared
    assert!(seq.is_err());
    Ok(())
}

// Provide NamespaceMap
pub fn generic_tr_name_1<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let l1 = sd
        .new_element(QName::new_from_parts(
            NcName::try_from("Level-1").unwrap(),
            Some(NamespaceUri::try_from("urn:test-example.com").unwrap()),
        ))
        .expect("unable to create element");
    t.push(l1.clone()).expect("unable to append child");

    let x = Transform::Name(Some(Box::new(Transform::ContextItem)));

    // Now evaluate the combinator with <Level-1> as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let mut namemap = NamespaceMap::new();
    namemap.push(
        NamespaceDeclaration::new(
            Some(NamespacePrefix::try_from("eg").unwrap()),
            NamespaceUri::try_from("urn:test-example.com").unwrap(),
        )
        .unwrap(),
    );
    let ctxt = ContextBuilder::new()
        .context(vec![Item::Node(l1)])
        .namespaces(namemap)
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq[0].to_string(), "eg:Level-1");
    Ok(())
}

// Declare namespace in the document
pub fn generic_tr_name_2<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // Setup a source document
    let mut sd = make_empty_doc();
    let mut t = sd
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    sd.push(t.clone()).expect("unable to append child");
    let l1 = sd
        .new_element(QName::new_from_parts(
            NcName::try_from("Level-1").unwrap(),
            Some(NamespaceUri::try_from("urn:test-example.com").unwrap()),
        ))
        .expect("unable to create element");
    let nsd = sd
        .new_namespace(
            NamespaceUri::try_from("urn:test-example.com").unwrap(),
            Some(NamespacePrefix::try_from("eg").unwrap()),
            true,
        )
        .expect("unable to create namespace node");
    l1.add_namespace(nsd).expect("unable to add namespace");
    t.push(l1.clone()).expect("unable to append child");

    let x = Transform::Name(Some(Box::new(Transform::ContextItem)));

    // Now evaluate the combinator with <Level-1> as the context item
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::from(vec![Item::Node(l1)])
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "eg:Level-1");
    Ok(())
}

pub fn generic_tr_string<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == string(1.0)
    let x = Transform::String(Box::new(Transform::Literal(Item::<N>::Value(Rc::new(
        Value::from(1.0),
    )))));
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "1");
    Ok(())
}

pub fn generic_tr_concat_literal<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == concat("abc", 1, "foo")
    let x = Transform::Concat(vec![
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from("abc")))),
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from(1)))),
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from("foo")))),
    ]);
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "abc1foo");
    Ok(())
}

pub fn generic_tr_starts_with_pos<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == starts-with("abc", "ab")
    let x = Transform::StartsWith(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "abc",
        ))))),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "ab",
        ))))),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_bool(), true);
    Ok(())
}

pub fn generic_tr_starts_with_neg<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == starts-with("abc", "x")
    let x = Transform::StartsWith(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "abc",
        ))))),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "x",
        ))))),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_bool(), false);
    Ok(())
}

pub fn generic_tr_contains_pos<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == contains("abcd", "bc")
    let x = Transform::Contains(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "abcd",
        ))))),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "bc",
        ))))),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_bool(), true);
    Ok(())
}

pub fn generic_tr_contains_neg<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == contains("abcd", "xyz")
    let x = Transform::Contains(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "abcd",
        ))))),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "xyz",
        ))))),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_bool(), false);
    Ok(())
}

pub fn generic_tr_substring_2args<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == substring("abcd", 2)
    let x = Transform::Substring(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "abcd",
        ))))),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            2,
        ))))),
        None,
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "bcd");
    Ok(())
}

pub fn generic_tr_substring_3args<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == substring("abcd", 2, 2)
    let x = Transform::Substring(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "abcd",
        ))))),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            2,
        ))))),
        Some(Box::new(Transform::Literal(Item::<N>::Value(Rc::new(
            Value::from(2),
        ))))),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "bc");
    Ok(())
}

pub fn generic_tr_substring_before<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == substring-before("abcd", "bc")
    let x = Transform::SubstringBefore(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "abcd",
        ))))),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "bc",
        ))))),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "a");
    Ok(())
}

pub fn generic_tr_substring_after<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == substring-after("abcd", "bc")
    let x = Transform::SubstringAfter(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "abcd",
        ))))),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "bc",
        ))))),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "d");
    Ok(())
}

pub fn generic_tr_normalize_space_1<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == normalize-space(" a b  c	d\n")
    let x = Transform::NormalizeSpace(Some(Box::new(Transform::Literal(Item::<N>::Value(
        Rc::new(Value::from(
            " a b  c	d
",
        )),
    )))));
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "a b c d");
    Ok(())
}

pub fn generic_tr_translate_1<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == translate("abcd", "bdc" "BD")
    let x = Transform::Translate(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "abcd",
        ))))),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "bdc",
        ))))),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "BD",
        ))))),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "aBD");
    Ok(())
}

pub fn generic_tr_boolean_string_pos<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == boolean("abcd")
    let x = Transform::Boolean(Box::new(Transform::Literal(Item::<N>::Value(Rc::new(
        Value::from("abcd"),
    )))));
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_bool(), true);
    Ok(())
}

pub fn generic_tr_boolean_string_neg<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == boolean("")
    let x = Transform::Boolean(Box::new(Transform::Literal(Item::<N>::Value(Rc::new(
        Value::from(""),
    )))));
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_bool(), false);
    Ok(())
}

pub fn generic_tr_boolean_int_pos<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == boolean(1)
    let x = Transform::Boolean(Box::new(Transform::Literal(Item::<N>::Value(Rc::new(
        Value::from(1),
    )))));
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_bool(), true);
    Ok(())
}

pub fn generic_tr_boolean_int_neg<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == boolean(0)
    let x = Transform::Boolean(Box::new(Transform::Literal(Item::<N>::Value(Rc::new(
        Value::from(0),
    )))));
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_bool(), false);
    Ok(())
}

pub fn generic_tr_not_pos<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == not("abcd")
    let x = Transform::Not(Box::new(Transform::Literal(Item::<N>::Value(Rc::new(
        Value::from("abcd"),
    )))));
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_bool(), false);
    Ok(())
}

pub fn generic_tr_not_neg<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == not(0)
    let x = Transform::Not(Box::new(Transform::Literal(Item::<N>::Value(Rc::new(
        Value::from(0),
    )))));
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_bool(), true);
    Ok(())
}

pub fn generic_tr_true_literal<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == true()
    let x = Transform::<N>::True;
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_bool(), true);
    Ok(())
}

pub fn generic_tr_false_literal<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == false()
    let x = Transform::<N>::False;
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_bool(), false);
    Ok(())
}

pub fn generic_tr_number<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == number("124")
    let x = Transform::Number(Box::new(Transform::Literal(Item::<N>::Value(Rc::new(
        Value::from("124"),
    )))));
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_int().unwrap(), 124);
    Ok(())
}

pub fn generic_tr_sum<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == sum((1, 2, 4))
    let x = Transform::Sum(Box::new(Transform::SequenceItems(vec![
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from(1)))),
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from(2)))),
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from(4)))),
    ])));
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_int().unwrap(), 7);
    Ok(())
}

pub fn generic_tr_avg<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == sum((1, 2, 4))
    let x = Transform::Avg(Box::new(Transform::SequenceItems(vec![
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from(1)))),
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from(2)))),
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from(4)))),
    ])));
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert!(seq[0].to_double() - 7.0 / 3.0 < 0.01);
    Ok(())
}

pub fn generic_tr_min<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == sum((1, 2, 4))
    let x = Transform::Min(Box::new(Transform::SequenceItems(vec![
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from(1)))),
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from(2)))),
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from(4)))),
    ])));
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_int().unwrap(), 1);
    Ok(())
}

pub fn generic_tr_max<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == sum((1, 2, 4))
    let x = Transform::Max(Box::new(Transform::SequenceItems(vec![
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from(1)))),
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from(2)))),
        Transform::Literal(Item::<N>::Value(Rc::new(Value::from(4)))),
    ])));
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_int().unwrap(), 4);
    Ok(())
}

pub fn generic_tr_floor<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == floor((1.2))
    let x = Transform::Floor(Box::new(Transform::Literal(Item::<N>::Value(Rc::new(
        Value::from(1.2),
    )))));
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq[0].to_double(), 1.0);
    Ok(())
}

pub fn generic_tr_ceiling<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == ceiling((1.2))
    let x = Transform::Ceiling(Box::new(Transform::Literal(Item::<N>::Value(Rc::new(
        Value::from(1.2),
    )))));
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq[0].to_double(), 2.0);
    Ok(())
}

pub fn generic_tr_round_1<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == round((1.23456))
    let x = Transform::Round(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            1.23456,
        ))))),
        None,
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq[0].to_double(), 1.0);
    Ok(())
}

pub fn generic_tr_round_2<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == round((1.23456, 4))
    let x = Transform::Round(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            1.23456,
        ))))),
        Some(Box::new(Transform::Literal(Item::<N>::Value(Rc::new(
            Value::from(4),
        ))))),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert!(seq[0].to_double() - 1.2346 < 0.000001);
    Ok(())
}

pub fn generic_tr_current_date_time<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == current-date-time()
    let x = Transform::<N>::CurrentDateTime;
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    match &seq[0] {
        Item::Value(v) => match v.value {
            ValueData::DateTime(dt) => {
                assert_eq!(dt.year(), Local::now().year());
                assert_eq!(dt.month(), Local::now().month());
                assert_eq!(dt.day(), Local::now().day());
                assert_eq!(dt.hour(), Local::now().hour());
                assert_eq!(dt.minute(), Local::now().minute());
                assert_eq!(dt.second(), Local::now().second()); // It is possible for this to fail if the elapsed time to execute the function call and the test falls across a second quantum
                Ok(())
            }
            _ => panic!("not a singleton dateTime value"),
        },
        _ => panic!("not a value"),
    }
}

pub fn generic_tr_current_date<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == current-date()
    let x = Transform::<N>::CurrentDate;
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    match &seq[0] {
        Item::Value(v) => match v.value {
            ValueData::Date(dt) => {
                assert_eq!(dt.year(), Local::now().year());
                assert_eq!(dt.month(), Local::now().month());
                assert_eq!(dt.day(), Local::now().day());
                Ok(())
            }
            _ => panic!("not a singleton date value"),
        },
        _ => panic!("not a value"),
    }
}

pub fn generic_tr_current_time<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == current-time()
    let x = Transform::<N>::CurrentTime;
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    match &seq[0] {
        Item::Value(v) => match v.value {
            ValueData::Time(dt) => {
                assert_eq!(dt.hour(), Local::now().hour());
                assert_eq!(dt.minute(), Local::now().minute());
                assert_eq!(dt.second(), Local::now().second()); // It is possible for this to fail if the elapsed time to execute the function call and the test falls across a second quantum
                Ok(())
            }
            _ => panic!("not a singleton time value"),
        },
        _ => panic!("not a value"),
    }
}

pub fn generic_tr_format_date_time<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == format-dateTime("2022-01-03T04:05:06.789+10:00", "[H]:[m] [D]/[M]/[Y]")
    let x = Transform::FormatDateTime(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "2022-01-03T04:05:06.789+10:00",
        ))))),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "[H]:[m] [D]/[M]/[Y]",
        ))))),
        None,
        None,
        None,
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "04:05 03/01/2022");
    Ok(())
}

pub fn generic_tr_format_date<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == format-date("2022-01-03", "[D]/[M]/[Y]")
    let x = Transform::FormatDate(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "2022-01-03",
        ))))),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "[D]/[M]/[Y]",
        ))))),
        None,
        None,
        None,
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "03/01/2022");
    Ok(())
}

pub fn generic_tr_format_time<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == format-time("04:05:06.789+10:00", "[H]:[m]")
    let x = Transform::FormatTime(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "04:05:06.789",
        ))))),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "[H]:[m]:[s]",
        ))))),
        None,
        None,
        None,
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "04:05:06");
    Ok(())
}

pub fn generic_tr_format_number_1<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    // XPath == format-number(123.456, "#.##")
    let x = Transform::FormatNumber(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            123.456,
        ))))),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "#.##",
        ))))),
        None,
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = Context::new()
        .dispatch(&mut stctxt, &x)
        .expect("evaluation failed");
    assert_eq!(seq.len(), 1);
    assert_eq!(seq.to_string(), "123.46");
    Ok(())
}

pub fn generic_tr_key_1<N: Node, G, H>(make_empty_doc: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Key(
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "mykey",
        ))))),
        Box::new(Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
            "blue",
        ))))),
        None,
        Rc::new(NamespaceMap::new()),
    );
    let mut sd = make_empty_doc();
    let mut top = sd
        .new_element(QName::from_local_name(NcName::try_from("Top").unwrap()))
        .expect("unable to create element");
    sd.push(top.clone()).expect("unable to add node");
    let mut red1 = sd
        .new_element(QName::from_local_name(NcName::try_from("one").unwrap()))
        .expect("unable to create element");
    red1.push(
        sd.new_text(Rc::new(Value::from("red")))
            .expect("unable to create text"),
    )
    .expect("unable to create element");
    top.push(red1).expect("unable to add node");
    let mut blue1 = sd
        .new_element(QName::from_local_name(NcName::try_from("two").unwrap()))
        .expect("unable to create element");
    blue1
        .push(
            sd.new_text(Rc::new(Value::from("blue")))
                .expect("unable to create text"),
        )
        .expect("unable to create element");
    top.push(blue1).expect("unable to add node");
    let mut yellow1 = sd
        .new_element(QName::from_local_name(NcName::try_from("three").unwrap()))
        .expect("unable to create element");
    yellow1
        .push(
            sd.new_text(Rc::new(Value::from("yellow")))
                .expect("unable to create text"),
        )
        .expect("unable to create element");
    top.push(yellow1).expect("unable to add node");

    let mut ctxt = ContextBuilder::new()
        .context(vec![Item::Node(sd.clone())])
        .build();
    ctxt.declare_key(
        String::from("mykey"),
        Pattern::try_from("child::*").expect("unable to parse pattern"), // Top/*
        Transform::Step(NodeMatch {
            axis: Axis::Child,
            nodetest: NodeTest::Kind(KindTest::Text),
        }),
    );
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    ctxt.populate_key_values(&mut stctxt, sd.clone())
        .expect("unable to populate key values");
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");

    assert_eq!(seq.len(), 1);
    assert_eq!(seq[0].name().unwrap().to_string(), "two");
    Ok(())
}

pub fn generic_tr_callable_named_1<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Invoke(
        QName::from_local_name(NcName::try_from("mycallable").unwrap()),
        ActualParameters::Named(vec![(
            QName::from_local_name(NcName::try_from("param1").unwrap()),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("value 1")))),
        )]),
        Rc::new(NamespaceMap::new()),
    );

    let ctxt = ContextBuilder::new()
        .callable(
            QName::from_local_name(NcName::try_from("mycallable").unwrap()),
            Callable::new(
                Transform::SequenceItems(vec![
                    Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                        "found parameter, value: ",
                    )))),
                    Transform::VariableReference(
                        "param1".to_string(),
                        Rc::new(NamespaceMap::new()),
                    ),
                ]),
                FormalParameters::Named(vec![(
                    QName::from_local_name(NcName::try_from("param1").unwrap()),
                    None,
                )]),
            ),
        )
        .build();

    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");

    assert_eq!(seq.to_string(), "found parameter, value: value 1");
    Ok(())
}

pub fn generic_tr_callable_positional_1<N: Node, G, H>(_: G, _: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::Invoke(
        QName::new_from_parts(
            NcName::try_from("my_func").unwrap(),
            Some(NamespaceUri::try_from("http://example.org/").unwrap()),
        ),
        ActualParameters::Positional(vec![Transform::Literal(Item::<N>::Value(Rc::new(
            Value::from("value 1"),
        )))]),
        Rc::new(NamespaceMap::new()),
    );

    let ctxt = ContextBuilder::new()
        .callable(
            QName::new_from_parts(
                NcName::try_from("my_func").unwrap(),
                Some(NamespaceUri::try_from("http://example.org/").unwrap()),
            ),
            Callable::new(
                Transform::SequenceItems(vec![
                    Transform::Literal(Item::<N>::Value(Rc::new(Value::from(
                        "found parameter, value: ",
                    )))),
                    Transform::VariableReference(
                        "param1".to_string(),
                        Rc::new(NamespaceMap::new()),
                    ),
                ]),
                FormalParameters::Positional(vec![QName::from_local_name(
                    NcName::try_from("param1").unwrap(),
                )]),
            ),
        )
        .build();

    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");

    assert_eq!(seq.to_string(), "found parameter, value: value 1");
    Ok(())
}

pub fn generic_tr_document_1<N: Node, G, H>(
    make_empty_doc: G,
    _: H,
    mut parser: Box<dyn FnMut(&str) -> Result<N, Error>>,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let mut sd = make_empty_doc();
    sd.push(
        sd.new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
            .expect("unable to create element"),
    )
    .expect("unable to add element");

    let x = Transform::SequenceItems(vec![
        Transform::Compose(vec![
            Transform::Step(NodeMatch {
                axis: Axis::Child,
                nodetest: NodeTest::Kind(KindTest::Any),
            }),
            Transform::LocalName(None),
        ]),
        Transform::Compose(vec![
            Transform::Document(
                Box::new(Transform::Literal(Item::Value(Rc::new(Value::from(
                    "urn:test",
                ))))),
                None,
            ),
            Transform::Step(NodeMatch {
                axis: Axis::Child,
                nodetest: NodeTest::Kind(KindTest::Any),
            }),
            Transform::LocalName(None),
        ]),
    ]);

    let ctxt = ContextBuilder::new().context(vec![Item::Node(sd)]).build();
    let mut stctxt = StaticContextBuilder::new()
        .fetcher(|_url| Ok(String::from("<External>document</External>")))
        .parser(|s| parser(s))
        .message(|_| Ok(()))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");

    assert_eq!(seq.to_string(), "TestExternal");
    Ok(())
}

pub fn generic_tr_generate_ints_1<N: Node, G, H>(
    make_empty_doc: G,
    make_doc: H,
    mut parser: Box<dyn FnMut(&str) -> Result<N, Error>>,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let sd = make_doc();

    let n = if let Item::Node(root) = sd.clone() {
        root.descend_iter().last().unwrap()
    } else {
        panic!("unable to find document root node")
    };
    let rd = make_empty_doc();

    let x = Transform::GenerateIntegers(
        Box::new(Transform::Empty),
        Box::new(Transform::ContextItem),
        Box::new(Numbering::new(Level::Single, None, None)),
    );

    let ctxt = ContextBuilder::new()
        .context(vec![Item::Node(n)])
        .result_document(rd)
        .build();
    let mut stctxt = StaticContextBuilder::new()
        .fetcher(|_url| Ok(String::from("<External>document</External>")))
        .parser(|s| parser(s))
        .message(|_| Ok(()))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");

    assert_eq!(seq.to_string(), "2");
    Ok(())
}

pub fn generic_tr_format_ints_1<N: Node, G, H>(
    _: G,
    _: H,
    mut parser: Box<dyn FnMut(&str) -> Result<N, Error>>,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::FormatInteger(
        Box::new(Transform::SequenceItems(vec![Transform::Literal(
            Item::Value(Rc::new(Value::from(42))),
        )])),
        Box::new(Transform::Literal(Item::Value(Rc::new(Value::from("1"))))),
    );

    let ctxt = ContextBuilder::new().build();
    let mut stctxt = StaticContextBuilder::new()
        .fetcher(|_url| Ok(String::from("<External>document</External>")))
        .message(|_| Ok(()))
        .parser(|s| parser(s))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");

    assert_eq!(seq.to_string(), "42");
    Ok(())
}

pub fn generic_tr_format_ints_2<N: Node, G, H>(
    _: G,
    _: H,
    mut parser: Box<dyn FnMut(&str) -> Result<N, Error>>,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::FormatInteger(
        Box::new(Transform::SequenceItems(vec![Transform::Literal(
            Item::Value(Rc::new(Value::from(42))),
        )])),
        Box::new(Transform::Literal(Item::Value(Rc::new(Value::from(
            "0001",
        ))))),
    );

    let ctxt = ContextBuilder::new().build();
    let mut stctxt = StaticContextBuilder::new()
        .fetcher(|_url| Ok(String::from("<External>document</External>")))
        .message(|_| Ok(()))
        .parser(|s| parser(s))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");

    assert_eq!(seq.to_string(), "0042");
    Ok(())
}

pub fn generic_tr_format_ints_3<N: Node, G, H>(
    _: G,
    _: H,
    mut parser: Box<dyn FnMut(&str) -> Result<N, Error>>,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::FormatInteger(
        Box::new(Transform::SequenceItems(vec![Transform::Literal(
            Item::Value(Rc::new(Value::from(42))),
        )])),
        Box::new(Transform::Literal(Item::Value(Rc::new(Value::from("W"))))),
    );

    let ctxt = ContextBuilder::new().build();
    let mut stctxt = StaticContextBuilder::new()
        .fetcher(|_url| Ok(String::from("<External>document</External>")))
        .message(|_| Ok(()))
        .parser(|s| parser(s))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");

    assert_eq!(seq.to_string(), "FORTY TWO");
    Ok(())
}

pub fn generic_tr_format_ints_4<N: Node, G, H>(
    _: G,
    _: H,
    mut parser: Box<dyn FnMut(&str) -> Result<N, Error>>,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::FormatInteger(
        Box::new(Transform::SequenceItems(vec![Transform::Literal(
            Item::Value(Rc::new(Value::from(42))),
        )])),
        Box::new(Transform::Literal(Item::Value(Rc::new(Value::from("w"))))),
    );

    let ctxt = ContextBuilder::new().build();
    let mut stctxt = StaticContextBuilder::new()
        .fetcher(|_url| Ok(String::from("<External>document</External>")))
        .message(|_| Ok(()))
        .parser(|s| parser(s))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");

    assert_eq!(seq.to_string(), "forty two");
    Ok(())
}

pub fn generic_tr_format_ints_5<N: Node, G, H>(
    _: G,
    _: H,
    mut parser: Box<dyn FnMut(&str) -> Result<N, Error>>,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::FormatInteger(
        Box::new(Transform::SequenceItems(vec![Transform::Literal(
            Item::Value(Rc::new(Value::from(42))),
        )])),
        Box::new(Transform::Literal(Item::Value(Rc::new(Value::from("Ww"))))),
    );

    let ctxt = ContextBuilder::new().build();
    let mut stctxt = StaticContextBuilder::new()
        .fetcher(|_url| Ok(String::from("<External>document</External>")))
        .message(|_| Ok(()))
        .parser(|s| parser(s))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");

    assert_eq!(seq.to_string(), "Forty Two");
    Ok(())
}

pub fn generic_tr_format_ints_6<N: Node, G, H>(
    _: G,
    _: H,
    mut parser: Box<dyn FnMut(&str) -> Result<N, Error>>,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::FormatInteger(
        Box::new(Transform::SequenceItems(vec![Transform::Literal(
            Item::Value(Rc::new(Value::from(42))),
        )])),
        Box::new(Transform::Literal(Item::Value(Rc::new(Value::from("i"))))),
    );

    let ctxt = ContextBuilder::new().build();
    let mut stctxt = StaticContextBuilder::new()
        .fetcher(|_url| Ok(String::from("<External>document</External>")))
        .message(|_| Ok(()))
        .parser(|s| parser(s))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");

    assert_eq!(seq.to_string(), "xlii");
    Ok(())
}

pub fn generic_tr_format_ints_7<N: Node, G, H>(
    _: G,
    _: H,
    mut parser: Box<dyn FnMut(&str) -> Result<N, Error>>,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn() -> Item<N>,
{
    let x = Transform::FormatInteger(
        Box::new(Transform::SequenceItems(vec![Transform::Literal(
            Item::Value(Rc::new(Value::from(42))),
        )])),
        Box::new(Transform::Literal(Item::Value(Rc::new(Value::from("I"))))),
    );

    let ctxt = ContextBuilder::new().build();
    let mut stctxt = StaticContextBuilder::new()
        .fetcher(|_url| Ok(String::from("<External>document</External>")))
        .message(|_| Ok(()))
        .parser(|s| parser(s))
        .build();
    let seq = ctxt.dispatch(&mut stctxt, &x).expect("evaluation failed");

    assert_eq!(seq.to_string(), "XLII");
    Ok(())
}
