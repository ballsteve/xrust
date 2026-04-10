//! Tests for security-related features

use qualname::{NcName, QName};
use std::rc::Rc;
use xrust::item::{Item, Node};
use xrust::pattern::Pattern;
use xrust::transform::context::{ContextBuilder, StaticContext, StaticContextBuilder};
use xrust::transform::template::Template;
use xrust::transform::{Axis, KindTest, NodeMatch, NodeTest, Transform};
use xrust::value::Value;
use xrust::xdmerror::Error;

// Max Depth feature not set - will use default.
// Small number of evaluations, should pass.
pub fn max_depth_np_1<N: Node, G>(make_empty_doc: G) -> Result<(), Error>
where
    G: Fn() -> N,
{
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| {
            Err(Error::new(
                xrust::ErrorKind::NotImplemented,
                "not implemented",
            ))
        })
        .parser(|_| {
            Err(Error::new(
                xrust::ErrorKind::NotImplemented,
                "not implemented",
            ))
        })
        .build();

    let mut src_doc = make_empty_doc();
    let top = src_doc
        .new_element(QName::from_local_name(NcName::try_from("Top").unwrap()))
        .expect("unable to create element");
    src_doc.push(top).expect("unable to add element");

    let x = Transform::ApplyTemplates(Box::new(Transform::Root), None, vec![]);
    let ctxt = ContextBuilder::new()
        .template(Template::new(
            // pattern "Top"
            Pattern::try_from("child::Top").expect("unable to create Pattern for \"child::Top\""),
            Transform::Literal(Item::<N>::Value(Rc::new(Value::from("at the top")))),
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
        .context(vec![Item::Node(src_doc)])
        .build();
    ctxt.dispatch(&mut stctxt, &x).map(|_| ())
}

// Max Depth feature not set - will use default.
// Large number of evaluations (infinite), should terminate with error.
pub fn max_depth_np_2<N: Node, G>(make_empty_doc: G) -> Result<(), Error>
where
    G: Fn() -> N,
{
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| {
            Err(Error::new(
                xrust::ErrorKind::NotImplemented,
                "not implemented",
            ))
        })
        .parser(|_| {
            Err(Error::new(
                xrust::ErrorKind::NotImplemented,
                "not implemented",
            ))
        })
        .build();

    let mut src_doc = make_empty_doc();
    let top = src_doc
        .new_element(QName::from_local_name(NcName::try_from("Top").unwrap()))
        .expect("unable to create element");
    src_doc.push(top).expect("unable to add element");

    let x = Transform::ApplyTemplates(Box::new(Transform::Root), None, vec![]);
    let ctxt = ContextBuilder::new()
        .template(Template::new(
            // pattern "Top"
            Pattern::try_from("child::Top").expect("unable to create Pattern for \"child::Top\""),
            Transform::ApplyTemplates(Box::new(Transform::ContextItem), None, vec![]), // infinite loop
            Some(0.0),                                                                 // priority
            vec![0],                                                                   // import
            Some(1), // document order
            None,    // mode
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
        .context(vec![Item::Node(src_doc)])
        .build();
    ctxt.dispatch(&mut stctxt, &x).map(|_| ())
}
