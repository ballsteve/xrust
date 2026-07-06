//! Tests for security-related features

use qualname::{NamespaceUri, NcName, QName};
use std::rc::Rc;
use xrust::ErrorKind;
use xrust::item::{Item, Node};
use xrust::pattern::Pattern;
use xrust::security::{Feature, Policy, SecurityPolicy, SecurityResult};
use xrust::transform::callable::{ActualParameters, FormalParameters};
use xrust::transform::context::{ContextBuilder, StaticContextBuilder};
use xrust::transform::template::Template;
use xrust::transform::{Axis, KindTest, NodeMatch, NodeTest, Transform};
use xrust::value::Value;
use xrust::xdmerror::Error;
use xrust::xslt::from_document;

// Max Depth feature not set - will use default.
// Small number of evaluations (1), should pass.
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
    let mut top1 = src_doc
        .new_element(QName::from_local_name(NcName::try_from("Top").unwrap()))
        .expect("unable to create element");
    src_doc.push(top1.clone()).expect("unable to add element");
    let mut top2 = src_doc
        .new_element(QName::from_local_name(NcName::try_from("Top").unwrap()))
        .expect("unable to create element");
    top1.push(top2.clone()).expect("unable to add element");
    let top3 = src_doc
        .new_element(QName::from_local_name(NcName::try_from("Top").unwrap()))
        .expect("unable to create element");
    top2.push(top3.clone()).expect("unable to add element");

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
    if ctxt
        .dispatch(&mut stctxt, &x)
        .map(|_| ())
        .is_err_and(|e| e.kind == ErrorKind::LimitExceeded)
    {
        Ok(())
    } else {
        panic!("failed to fail")
    }
}

// Create security policy with Max Depth feature set to None.
// Should allow arbitrary depth of evaluation.
// Large depth of evaluations (255), should run OK.
pub fn max_depth_pol_none<N: Node, G>(make_empty_doc: G) -> Result<(), Error>
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
    let mut top = src_doc
        .new_element(QName::from_local_name(NcName::try_from("Top").unwrap()))
        .expect("unable to create element");
    src_doc.push(top.clone()).expect("unable to add element");
    for _ in 0..252 {
        let nxt = src_doc
            .new_element(QName::from_local_name(NcName::try_from("Top").unwrap()))
            .expect("unable to create element");
        top.push(nxt.clone()).expect("unable to add element");
        top = nxt;
    }

    let mut policy: Policy<N> = Policy::new(QName::from_local_name(
        NcName::try_from("test_policy").unwrap(),
    ));
    policy.add(
        QName::new_from_parts(
            NcName::try_from("maximum-depth").unwrap(),
            Some(
                NamespaceUri::try_from(
                    "http://gitlab.gnome.org/World/Rust/markup-rs/xrust/transform",
                )
                .unwrap(),
            ),
        ),
        Feature::new(
            Transform::LiteralElement(
                QName::new_from_parts(
                    NcName::try_from("permitted").unwrap(),
                    Some(
                        NamespaceUri::try_from(
                            "http://gitlab.gnome.org/World/Rust/markup-rs/Security",
                        )
                        .unwrap(),
                    ),
                ),
                Box::new(Transform::Empty),
            ),
            FormalParameters::Named(vec![]),
        ),
    );

    let x = Transform::ApplyTemplates(Box::new(Transform::Root), None, vec![]);
    let ctxt = ContextBuilder::new()
        .policy(Rc::new(policy))
        .expect("unable to set security policy")
        .template(Template::new(
            // pattern "Top"
            Pattern::try_from("child::Top").expect("unable to create Pattern for \"child::Top\""),
            Transform::ApplyTemplates(
                Box::new(Transform::Step(NodeMatch {
                    axis: Axis::Child,
                    nodetest: NodeTest::Kind(KindTest::Any),
                })),
                None,
                vec![],
            ),
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

// Create security policy with Max Depth feature set to a value.
// Evaluate to a depth less than this value, should run OK.
pub fn max_depth_pol_set_1<N: Node, G>(make_empty_doc: G) -> Result<(), Error>
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
    let mut top = src_doc
        .new_element(QName::from_local_name(NcName::try_from("Top").unwrap()))
        .expect("unable to create element");
    src_doc.push(top.clone()).expect("unable to add element");
    // make iteration less than feature value
    for _ in 0..50 {
        let nxt = src_doc
            .new_element(QName::from_local_name(NcName::try_from("Top").unwrap()))
            .expect("unable to create element");
        top.push(nxt.clone()).expect("unable to add element");
        top = nxt;
    }

    let mut policy: Policy<N> = Policy::new(QName::from_local_name(
        NcName::try_from("test_policy").unwrap(),
    ));
    policy.add(
        QName::new_from_parts(
            NcName::try_from("maximum-depth").unwrap(),
            Some(
                NamespaceUri::try_from(
                    "http://gitlab.gnome.org/World/Rust/markup-rs/xrust/transform",
                )
                .unwrap(),
            ),
        ),
        Feature::new(
            Transform::LiteralElement(
                QName::new_from_parts(
                    NcName::try_from("permitted").unwrap(),
                    Some(
                        NamespaceUri::try_from(
                            "http://gitlab.gnome.org/World/Rust/markup-rs/Security",
                        )
                        .unwrap(),
                    ),
                ),
                Box::new(Transform::Literal(Item::Value(Rc::new(Value::from(100))))),
            ),
            FormalParameters::Named(vec![]),
        ),
    );

    let x = Transform::ApplyTemplates(Box::new(Transform::Root), None, vec![]);
    let ctxt = ContextBuilder::new()
        .policy(Rc::new(policy))
        .expect("unable to set security policy")
        .template(Template::new(
            // pattern "Top"
            Pattern::try_from("child::Top").expect("unable to create Pattern for \"child::Top\""),
            Transform::ApplyTemplates(
                Box::new(Transform::Step(NodeMatch {
                    axis: Axis::Child,
                    nodetest: NodeTest::Kind(KindTest::Any),
                })),
                None,
                vec![],
            ),
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

// Create security policy with Max Depth feature set to a value.
// Evaluate to a depth greater than this value, should error.
pub fn max_depth_pol_set_2<N: Node, G>(make_empty_doc: G) -> Result<(), Error>
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
    let mut top = src_doc
        .new_element(QName::from_local_name(NcName::try_from("Top").unwrap()))
        .expect("unable to create element");
    src_doc.push(top.clone()).expect("unable to add element");
    // make iteration less than feature value
    for _ in 0..1150 {
        let nxt = src_doc
            .new_element(QName::from_local_name(NcName::try_from("Top").unwrap()))
            .expect("unable to create element");
        top.push(nxt.clone()).expect("unable to add element");
        top = nxt;
    }

    let mut policy: Policy<N> = Policy::new(QName::from_local_name(
        NcName::try_from("test_policy").unwrap(),
    ));
    policy.add(
        QName::new_from_parts(
            NcName::try_from("maximum-depth").unwrap(),
            Some(
                NamespaceUri::try_from(
                    "http://gitlab.gnome.org/World/Rust/markup-rs/xrust/transform",
                )
                .unwrap(),
            ),
        ),
        Feature::new(
            Transform::LiteralElement(
                QName::new_from_parts(
                    NcName::try_from("permitted").unwrap(),
                    Some(
                        NamespaceUri::try_from(
                            "http://gitlab.gnome.org/World/Rust/markup-rs/Security",
                        )
                        .unwrap(),
                    ),
                ),
                Box::new(Transform::Literal(Item::Value(Rc::new(Value::from(1000))))),
            ),
            FormalParameters::Named(vec![]),
        ),
    );

    let x = Transform::ApplyTemplates(Box::new(Transform::Root), None, vec![]);
    let ctxt = ContextBuilder::new()
        .policy(Rc::new(policy))
        .expect("unable to set security policy")
        .template(Template::new(
            // pattern "Top"
            Pattern::try_from("child::Top").expect("unable to create Pattern for \"child::Top\""),
            Transform::ApplyTemplates(
                Box::new(Transform::Step(NodeMatch {
                    axis: Axis::Child,
                    nodetest: NodeTest::Kind(KindTest::Any),
                })),
                None,
                vec![],
            ),
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
    if ctxt
        .dispatch(&mut stctxt, &x)
        .map(|_| ())
        .is_err_and(|e| e.kind == ErrorKind::LimitExceeded)
    {
        Ok(())
    } else {
        panic!("failed to fail")
    }
}

// Call a named template in an inifinite recursion, should error.
pub fn max_depth_callable_1<N: Node, G, H>(make_empty_doc: G, make_from_str: H) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn(&str) -> Result<N, Error>,
{
    let styledoc = make_from_str(
        "<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
<xsl:template name='recurse'>
  <xsl:param name='count' select='0'/>
  <xsl:message>recurse called <xsl:sequence select='$count'/> times</xsl:message>
  <xsl:choose>
    <xsl:when test='$count lt 250'>
      <xsl:call-template name='recurse'>
        <xsl:with-param name='count' select='$count + 1'/>
      </xsl:call-template>
    </xsl:when>
    <xsl:otherwise>
      <xsl:message>reached 250 recursions</xsl:message>
    </xsl:otherwise>
  </xsl:choose>
</xsl:template>
<xsl:template match='/'>
  <xsl:call-template name='recurse'>
    <xsl:with-param name='count' select='1'/>
  </xsl:call-template>
</xsl:template>
</xsl:stylesheet>",
    )
    .map_err(|e| Error::new(e.kind, format!("error parsing stylesheet: {}", e.message)))?;
    let mut stctxt = StaticContextBuilder::new()
        .message(|m| {
            eprintln!("{}", m);
            Ok(())
        })
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
    src_doc.push(top.clone()).expect("unable to add element");

    let mut ctxt = from_document(styledoc, None, |s| make_from_str(s), |_| Ok(String::new()))?;
    ctxt.context(vec![Item::Node(src_doc.clone())], 0);

    if ctxt
        .evaluate(&mut stctxt)
        .map(|_| ())
        .is_err_and(|e| e.kind == ErrorKind::LimitExceeded)
    {
        Ok(())
    } else {
        panic!("failed to fail")
    }
}

/// Test security_feature method
pub fn sec_feature<N: Node, G>(make_empty_doc: &G) -> Result<(), Error>
where
    G: Fn() -> N,
{
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();
    let mut src_doc = make_empty_doc();
    let top = src_doc
        .new_element(QName::from_local_name(NcName::try_from("Top").unwrap()))
        .expect("unable to create element");
    src_doc.push(top.clone()).expect("unable to add element");

    let mut policy: Policy<N> = Policy::new(QName::from_local_name(
        NcName::try_from("test_policy").unwrap(),
    ));
    policy.add(
        QName::new_from_parts(
            NcName::try_from("maximum-depth").unwrap(),
            Some(
                NamespaceUri::try_from(
                    "http://gitlab.gnome.org/World/Rust/markup-rs/xrust/transform",
                )
                .unwrap(),
            ),
        ),
        Feature::new(
            Transform::LiteralElement(
                QName::new_from_parts(
                    NcName::try_from("permitted").unwrap(),
                    Some(
                        NamespaceUri::try_from(
                            "http://gitlab.gnome.org/World/Rust/markup-rs/Security",
                        )
                        .unwrap(),
                    ),
                ),
                Box::new(Transform::Literal(Item::Value(Rc::new(Value::from(1000))))),
            ),
            FormalParameters::Named(vec![]),
        ),
    );

    let x: Transform<N> = Transform::ApplyTemplates(Box::new(Transform::Root), None, vec![]);
    let ctxt = ContextBuilder::new()
        .policy(Rc::new(policy))
        .expect("unable to set security policy")
        .template(Template::new(
            // pattern "Top"
            Pattern::try_from("child::Top").expect("unable to create Pattern for \"child::Top\""),
            Transform::ApplyTemplates(
                Box::new(Transform::Step(NodeMatch {
                    axis: Axis::Child,
                    nodetest: NodeTest::Kind(KindTest::Any),
                })),
                None,
                vec![],
            ),
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
    let _ = ctxt.security_feature(
        &QName::new_from_parts(
            NcName::try_from("maximum-depth").unwrap(),
            Some(
                NamespaceUri::try_from(
                    "http://gitlab.gnome.org/World/Rust/markup-rs/xrust/transform",
                )
                .unwrap(),
            ),
        ),
        ActualParameters::Named(vec![]),
    )?;

    ctxt.dispatch(&mut stctxt, &x).map(|_| ())
}

/// Test From

pub fn sec_from_1<N: Node + SecurityPolicy, G, H>(
    _make_empty_doc: &G,
    make_from_str: H,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn(&str) -> Result<N, Error>,
{
    // First create a security policy document
    let poldoc = make_from_str("<sec:policy name='testpolicy' xmlns:sec='http://gitlab.gnome.org/World/Rust/markup-rs/Security'
        xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <sec:feature name='testfeature1'>
    <sec:not-permitted/>
  </sec:feature>
  <sec:feature name='testfeature2'>
    <sec:permitted/>
  </sec:feature>
  <sec:feature name='testfeature3'>
    <sec:permitted>
      <xsl:sequence select='42'/>
    </sec:permitted>
  </sec:feature>
  <sec:feature name='testfeature4'>
    <sec:permitted>42</sec:permitted>
  </sec:feature>
</sec:policy>").expect("unable to parse security document");
    let policy = poldoc
        .to_policy()
        .expect("unable to create security policy");
    let f = policy
        .get(
            &QName::from_local_name(NcName::try_from("testfeature3").unwrap()),
            ActualParameters::Named(vec![]),
        )
        .expect("unable to resolve security feature");
    assert_eq!(f, SecurityResult::Permitted(Some(String::from("42"))));
    let f = policy
        .get(
            &QName::from_local_name(NcName::try_from("testfeature4").unwrap()),
            ActualParameters::Named(vec![]),
        )
        .expect("unable to resolve security feature");
    assert_eq!(f, SecurityResult::Permitted(Some(String::from("42"))));
    Ok(())
}

// Security feature template that takes a parameter
pub fn sec_from_2<N: Node + SecurityPolicy, G, H>(
    _make_empty_doc: &G,
    make_from_str: H,
) -> Result<(), Error>
where
    G: Fn() -> N,
    H: Fn(&str) -> Result<N, Error>,
{
    // First create a security policy document
    let poldoc = make_from_str("<sec:policy name='testpolicy' xmlns:sec='http://gitlab.gnome.org/World/Rust/markup-rs/Security'
        xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <sec:feature name='testfeature1'>
    <xsl:param name='input'/>
    <xsl:choose>
      <xsl:when test='$input'>
        <sec:permitted/>
      </xsl:when>
      <xsl:otherwise>
        <sec:not-permitted/>
      </xsl:otherwise>
    </xsl:choose>
  </sec:feature>
</sec:policy>").expect("unable to parse security document");
    let policy = poldoc
        .to_policy()
        .expect("unable to create security policy");
    let f = policy
        .get(
            &QName::from_local_name(NcName::try_from("testfeature1").unwrap()),
            ActualParameters::Named(vec![]),
        )
        .expect("unable to resolve security feature");
    assert_eq!(f, SecurityResult::NotPermitted);
    let f = policy
        .get(
            &QName::from_local_name(NcName::try_from("testfeature1").unwrap()),
            ActualParameters::Named(vec![(
                QName::from_local_name(NcName::try_from("input").unwrap()),
                Transform::Literal(Item::Value(Rc::new(Value::from("yes")))),
            )]),
        )
        .expect("unable to resolve security feature");
    assert_eq!(f, SecurityResult::Permitted(None));
    Ok(())
}
