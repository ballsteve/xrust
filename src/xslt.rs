/*! ## An XSLT compiler

Compile an XSLT stylesheet into a Sequence [Constructor].

Once the stylesheet has been compiled, it may then be evaluated by the evaluation module.

NB. This module, by default, does not resolve include or import statements. See the xrust-net crate for a helper module to do that.

```rust
use std::rc::Rc;
use xrust::xdmerror::Error;
use xrust::qname::QualifiedName;
use xrust::item::{Item, Node, NodeType, Sequence, SequenceTrait};
use xrust::evaluate::{Evaluator, StaticContext};
use xrust::xslt::from_document;
use xrust::intmuttree::{Document, RNode, NodeBuilder};

// A little helper function that wraps the toplevel node in a Document
fn make_from_str(s: &str) -> Result<RNode, Error> {
    let e = Document::try_from(s).expect("failed to parse XML").content[0].clone();
    let mut d = NodeBuilder::new(NodeType::Document).build();
    d.push(e).expect("unable to append node");
    Ok(d)
}

// First setup a static context for the evaluator
let mut sc = StaticContext::new_with_builtins();

// The source document (a tree)
let src = Rc::new(Item::Node(
    make_from_str("<Example><Title>XSLT in Rust</Title><Paragraph>A simple document.</Paragraph></Example>")
    .expect("unable to parse XML")
));

// The XSL stylesheet
let style = make_from_str("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::Example'><html><xsl:apply-templates/></html></xsl:template>
  <xsl:template match='child::Title'><head><title><xsl:apply-templates/></title></head></xsl:template>
  <xsl:template match='child::Paragraph'><body><p><xsl:apply-templates/></p></body></xsl:template>
</xsl:stylesheet>")
    .expect("unable to parse stylesheet");

// Compile the stylesheet
let ev = from_document(
    style,
    &mut sc,
    None,
    make_from_str,
    |_| Ok(String::new())
)
    .expect("failed to compile stylesheet");

// Make an empty result document
let rd = NodeBuilder::new(NodeType::Document).build();

// Prime the stylesheet evaluation by finding the template for the document root
// and making the document root the initial context
let t = ev.find_match(&src, None, &rd)
    .expect("unable to find match");

// Let 'er rip!
// Evaluate the sequence constructor with the source document as the initial context
let seq = ev.evaluate(Some(vec![Rc::clone(&src)]), Some(0), &t, &rd)
    .expect("evaluation failed");

// Serialise the sequence as XML
assert_eq!(seq.to_xml(), "<html><head><title>XSLT in Rust</title></head><body><p>A simple document.</p></body></html>")
*/

use crate::evaluate::*;
use crate::item::{Node, NodeType};
use crate::output::*;
use crate::qname::*;
use crate::value::*;
use crate::xdmerror::*;
use crate::xpath::*;
use std::convert::TryFrom;
use url::Url;

const XSLTNS: &str = "http://www.w3.org/1999/XSL/Transform";

/// Compiles a [Node] into an [Evaluator].
/// NB. Due to whitespace stripping, this is destructive of the stylesheet.
/// The argument f is a function that parses a string to a [Node]. The argument g is a function that resolves a URL to a string. These are used for include and import modules.
pub fn from_document<N: Node>(
    styledoc: N,
    sc: &mut StaticContext<N>,
    b: Option<Url>,
    f: fn(&str) -> Result<N, Error>,
    g: fn(&Url) -> Result<String, Error>,
) -> Result<Evaluator<N>, Error> {
    let mut ev = Evaluator::new();
    if b.is_some() {
        ev.set_baseurl(b.unwrap())
    }

    // Check that this is a valid XSLT stylesheet
    // There must be a single element as a child of the root node, and it must be named xsl:stylesheet or xsl:transform
    let mut rnit = styledoc.child_iter();
    let stylenode = match rnit.next() {
        Some(root) => {
            if !(root.name().get_nsuri_ref() == Some(XSLTNS)
                && (root.name().get_localname() == "stylesheet"
                    || root.name().get_localname() == "transform"))
            {
                return Result::Err(Error::new(
                    ErrorKind::TypeError,
                    "not an XSLT stylesheet".to_string(),
                ));
            } else {
                root
            }
        }
        None => {
            return Result::Err(Error::new(
                ErrorKind::TypeError,
                String::from("document does not have document element"),
            ))
        }
    };
    // TODO: rnit.next() should == None

    // TODO: check version attribute

    // Strip whitespace from the stylesheet
    strip_whitespace(
        styledoc.clone(),
        true,
        &vec![NodeTest::Name(NameTest {
            ns: None,
            prefix: None,
            name: Some(WildcardOrName::Wildcard),
        })],
        &vec![NodeTest::Name(NameTest {
            ns: Some(WildcardOrName::Name(XSLTNS.to_string())),
            prefix: Some("xsl".to_string()),
            name: Some(WildcardOrName::Name("text".to_string())),
        })],
    )?;

    // Define the builtin templates
    // See XSLT 6.7. This implements text-only-copy.
    // TODO: Support deep-copy, shallow-copy, deep-skin, shallow-skip and fail

    // This matches "/" and processes the root element
    let bi1pat = to_pattern(vec![Constructor::Path(vec![vec![Constructor::Root]])])?;
    let bi1bod = vec![Constructor::ApplyTemplates(vec![Constructor::Step(
        NodeMatch {
            axis: Axis::Child,
            nodetest: NodeTest::Kind(KindTest::AnyKindTest),
        },
        vec![],
    )])];
    ev.add_builtin_template(bi1pat, bi1bod, None, -1.0, 0);
    // This matches "*" and applies templates to all children
    let bi2pat = to_pattern(vec![Constructor::Path(vec![vec![Constructor::Step(
        NodeMatch {
            axis: Axis::Child,
            nodetest: NodeTest::Name(NameTest {
                ns: None,
                prefix: None,
                name: Some(WildcardOrName::Wildcard),
            }),
        },
        vec![],
    )]])])?;
    let bi2bod = vec![Constructor::ApplyTemplates(vec![Constructor::Step(
        NodeMatch {
            axis: Axis::Child,
            nodetest: NodeTest::Kind(KindTest::AnyKindTest),
        },
        vec![],
    )])];
    ev.add_builtin_template(bi2pat, bi2bod, None, -1.0, 0);
    // This matches "text()" and copies content
    let bi3pat = to_pattern(vec![Constructor::Path(vec![vec![Constructor::Step(
        NodeMatch {
            axis: Axis::Child,
            nodetest: NodeTest::Kind(KindTest::TextTest),
        },
        vec![],
    )]])])?;
    let bi3bod = vec![Constructor::ContextItem];
    ev.add_builtin_template(bi3pat, bi3bod, None, -1.0, 0);

    // Setup the serialization of the primary result document
    stylenode
        .child_iter()
        .skip_while(|c| {
            !(c.is_element()
                && c.name().get_nsuri_ref() == Some(XSLTNS)
                && c.name().get_localname() == "output")
        })
        .nth(0)
        .map(|c| {
            let b: bool = match c
                .get_attribute(&QualifiedName::new(None, None, "indent".to_string()))
                .to_string()
                .as_str()
            {
                "yes" | "true" | "1" => true,
                _ => false,
            };

            let mut od = OutputDefinition::new();
            od.set_indent(b);
            ev.set_output_definition(od);
        });

    eprintln!("doing includes");
    // Iterate over children, looking for includes
    // * resolve href
    // * fetch document
    // * parse XML
    // * replace xsl:include element with content
    stylenode
        .child_iter()
        .filter(|c| {
            c.is_element()
                && c.name().get_nsuri_ref() == Some(XSLTNS)
                && c.name().get_localname() == "include"
        })
        .try_for_each(|mut c| {
            let h = c.get_attribute(&QualifiedName::new(None, None, "href".to_string()));
            let url = match ev.baseurl().map_or_else(
                || Url::parse(h.to_string().as_str()),
                |base| base.join(h.to_string().as_str()),
            ) {
                Ok(u) => u,
                Err(_) => {
                    return Result::Err(Error {
                        kind: ErrorKind::Unknown,
                        message: format!(
                            "unable to parse href URL \"{}\" baseurl \"{}\"",
                            h.to_string(),
                            ev.baseurl()
                                .map_or(String::from("--no base--"), |b| b.to_string())
                        ),
                    })
                }
            };
            eprintln!("got URL, about to fetch data");
            let xml = g(&url)?;
            eprintln!("got data, about to parse");
            let module = f(xml.as_str().trim())?;
            // TODO: check that the module is a valid XSLT stylesheet, etc
            // Copy each top-level element of the module to the main stylesheet,
            // inserting before the xsl:include node
            eprintln!("inserting elements");
            let moddoc = module.first_child().unwrap();
            moddoc.child_iter().try_for_each(|mc| {
                c.insert_before(mc)?;
                Ok::<(), Error>(())
            })?;
            // Remove the xsl:include element node
            c.pop()?;
            eprintln!("done");
            Ok(())
        })?;

    eprintln!("doing imports");
    // Iterate over children, looking for imports
    // * resolve href
    // * fetch document
    // * parse XML
    // * replace xsl:import element with content
    stylenode
        .child_iter()
        .filter(|c| {
            c.is_element()
                && c.name().get_nsuri_ref() == Some(XSLTNS)
                && c.name().get_localname() == "import"
        })
        .try_for_each(|mut c| {
            let h = c.get_attribute(&QualifiedName::new(None, None, "href".to_string()));
            let url = match ev.baseurl().map_or_else(
                || Url::parse(h.to_string().as_str()),
                |base| base.join(h.to_string().as_str()),
            ) {
                Ok(u) => u,
                Err(_) => {
                    return Result::Err(Error {
                        kind: ErrorKind::Unknown,
                        message: format!(
                            "unable to parse href URL \"{}\" baseurl \"{}\"",
                            h.to_string(),
                            ev.baseurl()
                                .map_or(String::from("--no base--"), |b| b.to_string())
                        ),
                    })
                }
            };
            let xml = g(&url)?;
            let module = f(xml.as_str().trim())?;
            // TODO: check that the module is a valid XSLT stylesheet, etc
            // Copy each top-level element of the module to the main stylesheet,
            // inserting before the xsl:include node
            // TODO: Don't Panic
            let moddoc = module.first_child().unwrap();
            moddoc.child_iter().try_for_each(|mc| {
                if mc.node_type() == NodeType::Element {
                    // Add the import precedence attribute
                    let newnode = mc.deep_copy()?;
                    let newat = styledoc.new_attribute(
                        QualifiedName::new(
                            Some(String::from("http://github.com/ballsteve/xrust")),
                            None,
                            String::from("import"),
                        ),
                        Value::from(1),
                    )?;
                    newnode.add_attribute(newat)?;
                    c.insert_before(newnode)?;
                } else {
                    let newnode = mc.deep_copy()?;
                    c.insert_before(newnode)?;
                }
                Ok::<(), Error>(())
            })?;
            // Remove the xsl:import element node
            c.pop()?;
            Ok::<(), Error>(())
        })?;

    eprintln!("doing templates");
    // Iterate over children, looking for templates
    // * compile match pattern
    // * compile content into sequence constructor
    // * register template in dynamic context
    stylenode
        .child_iter()
        .filter(|c| {
            c.is_element()
                && c.name().get_nsuri_ref() == Some(XSLTNS)
                && c.name().get_localname() == "template"
        })
        .try_for_each(|c| {
            let m = c.get_attribute(&QualifiedName::new(None, None, "match".to_string()));
            let n = m.clone().to_string();
            let a = parse(&n).expect("failed to parse match expression");
            let mut pat = to_pattern(a).expect("failed to compile match pattern");
            let mut body = vec![];
            c.child_iter().try_for_each(|d| {
                body.push(to_constructor(d)?);
                Ok::<(), Error>(())
            })?;
            sc.static_analysis(&mut pat);
            sc.static_analysis(&mut body);
            // Determine the priority of the template
            let prio;
            let pr = c.get_attribute(&QualifiedName::new(None, None, "priority".to_string()));
            match pr.to_string().as_str() {
                "" => {
                    // Calculate the default priority
                    // TODO: more work to be done interpreting XSLT 6.5
                    if pat.len() <= 1 {
                        match &pat[0] {
                            Constructor::Root => prio = -0.5,
                            Constructor::Path(_) => prio = -0.5,
                            Constructor::Step(nm, _pred) => match &nm.nodetest {
                                NodeTest::Name(nt) => match (nt.ns.as_ref(), nt.name.as_ref()) {
                                    (
                                        Some(WildcardOrName::Wildcard),
                                        Some(WildcardOrName::Wildcard),
                                    ) => prio = -0.5,
                                    (
                                        Some(WildcardOrName::Wildcard),
                                        Some(WildcardOrName::Name(_)),
                                    )
                                    | (
                                        Some(WildcardOrName::Name(_)),
                                        Some(WildcardOrName::Wildcard),
                                    ) => prio = -0.25,
                                    (None, Some(WildcardOrName::Wildcard)) => prio = -0.25,
                                    (
                                        Some(WildcardOrName::Name(_)),
                                        Some(WildcardOrName::Name(_)),
                                    ) => prio = 0.0,
                                    (None, Some(WildcardOrName::Name(_))) => prio = 0.0,
                                    _ => prio = 0.5,
                                },
                                NodeTest::Kind(kt) => match kt {
                                    KindTest::DocumentTest
                                    | KindTest::ElementTest
                                    | KindTest::AttributeTest => prio = -0.5,
                                    _ => prio = 0.5,
                                },
                            },
                            _ => prio = 0.5,
                        }
                    } else {
                        // TODO: calculate the priority of each branch of the pattern
                        prio = 0.5
                    }
                }
                _ => prio = pr.to_string().parse::<f64>().unwrap(), // TODO: better error handling
            }
            // Set the import precedence
            let mut import: usize = 0;
            let im = c.get_attribute(&QualifiedName::new(
                Some(String::from("http://github.com/ballsteve/xrust")),
                None,
                String::from("import"),
            ));
            if im.to_string() != "" {
                import = im.to_int()? as usize
            }
            ev.add_template(pat, body, None, prio, import);
            Ok::<(), Error>(())
        })?;

    eprintln!("done");
    Ok(ev)
}

/// Compile a node in a template to a sequence constructor
fn to_constructor<N: Node>(n: N) -> Result<Constructor<N>, Error> {
    match n.node_type() {
        NodeType::Text => Ok(Constructor::Literal(Value::String(n.to_string()))),
        NodeType::Element => {
            match (n.name().get_nsuri_ref(), n.name().get_localname().as_str()) {
                (Some(XSLTNS), "text") => {
                    let doe = n.get_attribute(&QualifiedName::new(
                        None,
                        None,
                        "disable-output-escaping".to_string(),
                    ));
                    if doe.to_string().len() != 0 {
                        match &doe.to_string()[..] {
                            "yes" => Ok(Constructor::Literal(Value::String(n.to_string()))),
                            "no" => {
                                let text = n
                                    .to_string()
                                    .replace("&", "&amp;")
                                    .replace(">", "&gt;")
                                    .replace("<", "&lt;")
                                    .replace("'", "&apos;")
                                    .replace("\"", "&quot;");
                                Ok(Constructor::Literal(Value::from(text)))
                            }
                            _ => {
                                return Result::Err(Error {
                                    kind: ErrorKind::TypeError,
                                    message:
                                        "disable-output-escaping only accepts values yes or no."
                                            .to_string(),
                                })
                            }
                        }
                    } else {
                        let text = n
                            .to_string()
                            .replace("&", "&amp;")
                            .replace(">", "&gt;")
                            .replace("<", "&lt;")
                            .replace("'", "&apos;")
                            .replace("\"", "&quot;");
                        Ok(Constructor::Literal(Value::from(text)))
                    }
                }
                (Some(XSLTNS), "apply-templates") => {
                    let sel =
                        n.get_attribute(&QualifiedName::new(None, None, "select".to_string()));
                    if sel.to_string().len() != 0 {
                        Ok(Constructor::ApplyTemplates(parse(&sel.to_string())?))
                    } else {
                        // If there is no select attribute, then default is "child::node()"
                        Ok(Constructor::ApplyTemplates(vec![Constructor::Step(
                            NodeMatch {
                                axis: Axis::Child,
                                nodetest: NodeTest::Kind(KindTest::AnyKindTest),
                            },
                            vec![],
                        )]))
                    }
                }
                (Some(XSLTNS), "apply-imports") => Ok(Constructor::ApplyImports),
                (Some(XSLTNS), "sequence") => {
                    let s = n.get_attribute(&QualifiedName::new(None, None, "select".to_string()));
                    if s.to_string().len() != 0 {
                        let cons = parse(&s.to_string())?;
                        if cons.len() > 1 {
                            return Result::Err(Error {
                                kind: ErrorKind::TypeError,
                                message: "select attribute has more than one sequence constructor"
                                    .to_string(),
                            });
                        }
                        Ok(cons[0].clone())
                    } else {
                        return Result::Err(Error {
                            kind: ErrorKind::TypeError,
                            message: "missing select attribute".to_string(),
                        });
                    }
                }
                (Some(XSLTNS), "if") => {
                    let t = n.get_attribute(&QualifiedName::new(None, None, "test".to_string()));
                    if t.to_string().len() != 0 {
                        Ok(Constructor::Switch(
                            vec![
                                parse(&t.to_string())?,
                                n.child_iter().try_fold(vec![], |mut body, e| {
                                    body.push(to_constructor(e)?);
                                    Ok(body)
                                })?,
                            ],
                            vec![],
                        ))
                    } else {
                        return Result::Err(Error {
                            kind: ErrorKind::TypeError,
                            message: "missing test attribute".to_string(),
                        });
                    }
                }
                (Some(XSLTNS), "choose") => {
                    let mut when: Vec<Vec<Constructor<N>>> = Vec::new();
                    let mut otherwise: Vec<Constructor<N>> = Vec::new();
                    let mut status: Option<Error> = None;
                    n.child_iter()
			.try_for_each(|m| {
			    // look for when elements
	      		    // then find an otherwise
	      		    // fail on anything else (apart from whitespace, comments, PIs)
			    match m.node_type() {
				NodeType::Element => {
      				    match (m.name().get_nsuri_ref(), m.name().get_localname().as_str()) {
        				(Some(XSLTNS), "when") => {
					    if otherwise.len() == 0 {
						let t = m.get_attribute(&QualifiedName::new(None, None, "test".to_string()));
						if t.to_string().len() != 0 {
						    when.push(
		    					parse(&t.to_string())?
						    );
						    when.push(
							m.child_iter()
							    .try_fold(
								vec![],
								|mut body, e| {
								    body.push(to_constructor(e)?);
								    Ok(body)
								}
							    )?
						    );
						} else {
	      					    status.replace(Error{kind: ErrorKind::TypeError, message: "missing test attribute".to_string()});
						}
					    } else {
						status.replace(Error{kind: ErrorKind::TypeError, message: "invalid content in choose element: when follows otherwise".to_string()});
					    }
					}
        				(Some(XSLTNS), "otherwise") => {
					    if when.len() != 0 {
						m.child_iter()
						    .try_for_each(|e| {
							otherwise.push(to_constructor(e)?);
							Ok::<(), Error>(())
						    })?;
					    } else {
						status.replace(Error{kind: ErrorKind::TypeError, message: "invalid content in choose element: no when elements".to_string()});
					    }
					}
					_ => {
					    status.replace(Error{kind: ErrorKind::TypeError, message: "invalid element content in choose element".to_string()});
					}
				    }
				}
				NodeType::Text => {
				    if !n.to_string().trim().is_empty() {
					status.replace(Error{kind: ErrorKind::TypeError, message: "invalid text content in choose element".to_string()});
				    }
				}
				NodeType::Comment |
				NodeType::ProcessingInstruction => {}
				_ => {
				    status.replace(Error{kind: ErrorKind::TypeError, message: "invalid content in choose element".to_string()});
				}
			    }
			    Ok::<(), Error>(())
			})?;
                    match status {
                        Some(e) => Result::Err(e),
                        None => Ok(Constructor::Switch(when, otherwise)),
                    }
                }
                (Some(XSLTNS), "for-each") => {
                    let s = n.get_attribute(&QualifiedName::new(None, None, "select".to_string()));
                    if s.to_string().len() != 0 {
                        Ok(Constructor::ForEach(
                            parse(&s.to_string())?,
                            n.child_iter().try_fold(vec![], |mut body, e| {
                                body.push(to_constructor(e)?);
                                Ok(body)
                            })?,
                            None,
                        ))
                    } else {
                        return Result::Err(Error {
                            kind: ErrorKind::TypeError,
                            message: "missing select attribute".to_string(),
                        });
                    }
                }
                (Some(XSLTNS), "for-each-group") => {
                    let s = n.get_attribute(&QualifiedName::new(None, None, "select".to_string()));
                    if s.to_string().len() != 0 {
                        match (
                            n.get_attribute(&QualifiedName::new(
                                None,
                                None,
                                "group-by".to_string(),
                            ))
                            .to_string()
                            .as_str(),
                            n.get_attribute(&QualifiedName::new(
                                None,
                                None,
                                "group-adjacent".to_string(),
                            ))
                            .to_string()
                            .as_str(),
                            n.get_attribute(&QualifiedName::new(
                                None,
                                None,
                                "group-starting-with".to_string(),
                            ))
                            .to_string()
                            .as_str(),
                            n.get_attribute(&QualifiedName::new(
                                None,
                                None,
                                "group-ending-with".to_string(),
                            ))
                            .to_string()
                            .as_str(),
                        ) {
                            (by, "", "", "") => Ok(Constructor::ForEach(
                                parse(&s.to_string())?,
                                n.child_iter().try_fold(vec![], |mut body, e| {
                                    body.push(to_constructor(e)?);
                                    Ok(body)
                                })?,
                                Some(Grouping::By(parse(&by.to_string())?)),
                            )),
                            ("", adj, "", "") => Ok(Constructor::ForEach(
                                parse(&s.to_string())?,
                                n.child_iter().try_fold(vec![], |mut body, e| {
                                    body.push(to_constructor(e)?);
                                    Ok(body)
                                })?,
                                Some(Grouping::Adjacent(parse(&adj.to_string())?)),
                            )),
                            // TODO: group-starting-with and group-ending-with
                            _ => Result::Err(Error {
                                kind: ErrorKind::NotImplemented,
                                message: "invalid grouping attribute(s) specified".to_string(),
                            }),
                        }
                    } else {
                        return Result::Err(Error {
                            kind: ErrorKind::TypeError,
                            message: "missing select attribute".to_string(),
                        });
                    }
                }
                (Some(XSLTNS), "copy") => {
                    // TODO: handle select attribute
                    Ok(Constructor::Copy(
                        vec![],
                        // The content of this element is a template for the content of the new item
                        n.child_iter().try_fold(vec![], |mut body, e| {
                            body.push(to_constructor(e)?);
                            Ok(body)
                        })?,
                    ))
                }
                (Some(XSLTNS), "copy-of") => {
                    let s = n.get_attribute(&QualifiedName::new(None, None, "select".to_string()));
                    if s.to_string().len() != 0 {
                        Ok(Constructor::DeepCopy(parse(&s.to_string())?))
                    } else {
                        return Result::Err(Error {
                            kind: ErrorKind::TypeError,
                            message: "missing select attribute".to_string(),
                        });
                    }
                }
                (Some(XSLTNS), "attribute") => {
                    let m = n.get_attribute(&QualifiedName::new(None, None, "name".to_string()));
                    if m.to_string().len() != 0 {
                        Ok(Constructor::LiteralAttribute(
                            QualifiedName::new(None, None, m.to_string()),
                            n.child_iter().try_fold(vec![], |mut body, e| {
                                body.push(to_constructor(e)?);
                                Ok(body)
                            })?,
                        ))
                    } else {
                        return Result::Err(Error {
                            kind: ErrorKind::TypeError,
                            message: "missing select attribute".to_string(),
                        });
                    }
                }
                (Some(XSLTNS), u) => Ok(Constructor::NotImplemented(format!(
                    "unsupported XSL element \"{}\"",
                    u
                ))),
                (_, a) => {
                    // TODO: Handle qualified element name
                    let mut content = vec![];
                    n.attribute_iter().try_for_each(|e| {
                        content.push(to_constructor(e)?);
                        Ok::<(), Error>(())
                    })?;
                    n.child_iter().try_for_each(|e| {
                        content.push(to_constructor(e)?);
                        Ok::<(), Error>(())
                    })?;
                    Ok(Constructor::LiteralElement(
                        QualifiedName::new(None, None, a.to_string()),
                        content,
                    ))
                }
            }
        }
        NodeType::Attribute => {
            // Get value as a Value
            Ok(Constructor::LiteralAttribute(
                n.name(),
                vec![Constructor::Literal(Value::String(n.to_string()))],
            ))
        }
        _ => {
            // TODO: literal elements, etc, pretty much everything in the XSLT spec
            println!("found a strange element");
            Ok(Constructor::NotImplemented(
                "other template content".to_string(),
            ))
        }
    }
}

/// Strip whitespace nodes from a XDM tree.
/// See [XSLT 4.3](https://www.w3.org/TR/2017/REC-xslt-30-20170608/#stylesheet-stripping).
/// The [Node] argument must be the document node of the tree.
pub fn strip_whitespace<N: Node>(
    t: N,
    cpi: bool, // strip comments and PIs?
    strip: &Vec<NodeTest>,
    preserve: &Vec<NodeTest>,
) -> Result<(), Error> {
    t.child_iter().try_for_each(|n| {
        strip_whitespace_node(n, cpi, strip, preserve, true)?;
        Ok(())
    })?;
    Ok(())
}

/// Strip whitespace nodes from a XDM tree.
/// This function operates under the direction of the xsl:strip-space and xsl:preserve-space directives in a XSLT stylesheet.
pub fn strip_source_document<N: Node>(src: N, style: N) -> Result<(), Error> {
    // Find strip-space element, if any, and use it to construct a vector of NodeTests.
    // Ditto for preserve-space.
    let mut ss: Vec<NodeTest> = vec![];
    let mut ps: Vec<NodeTest> = vec![];
    style.child_iter().try_for_each(|n| {
        // n should be the xsl:stylesheet element
        n.child_iter().try_for_each(|m| {
            match (
                m.node_type(),
                m.name().get_nsuri_ref(),
                m.name().get_localname().as_str(),
            ) {
                (NodeType::Element, Some(XSLTNS), "strip-space") => {
                    let v =
                        m.get_attribute(&QualifiedName::new(None, None, "elements".to_string()));
                    if v.to_string().len() != 0 {
                        v.to_string().split_whitespace().try_for_each(|t| {
                            ss.push(NodeTest::try_from(t)?);
                            Ok::<(), Error>(())
                        })?
                    } else {
                        return Result::Err(Error::new(
                            ErrorKind::Unknown,
                            String::from("missing elements attribute"),
                        ));
                    }
                }
                (NodeType::Element, Some(XSLTNS), "preserve-space") => {
                    let v =
                        m.get_attribute(&QualifiedName::new(None, None, "elements".to_string()));
                    if v.to_string().len() != 0 {
                        v.to_string().split_whitespace().try_for_each(|t| {
                            ps.push(NodeTest::try_from(t)?);
                            Ok::<(), Error>(())
                        })?
                    } else {
                        return Result::Err(Error::new(
                            ErrorKind::Unknown,
                            String::from("missing elements attribute"),
                        ));
                    }
                }
                _ => {}
            }
            Ok::<(), Error>(())
        })?;
        Ok::<(), Error>(())
    })?;

    strip_whitespace(src, false, &ss, &ps)
}

// TODO: the rules for stripping/preserving are a lot more complex
// TODO: Return Result so that errors can be propagated
fn strip_whitespace_node<N: Node>(
    mut n: N,
    cpi: bool, // strip comments and PIs?
    strip: &Vec<NodeTest>,
    preserve: &Vec<NodeTest>,
    keep: bool,
) -> Result<(), Error> {
    match n.node_type() {
        NodeType::Comment | NodeType::ProcessingInstruction => {
            if cpi {
                n.pop()?;
                // TODO: Merge text nodes that are now adjacent
            }
        }
        NodeType::Element => {
            // Determine if this element toggles the strip/preserve setting
            // Match a strip NodeTest or a preserve NodeTest
            // The 'strength' of the match determines which setting wins
            let mut ss = -1.0;
            let mut ps = -1.0;
            strip.iter().for_each(|t| match t {
                NodeTest::Kind(KindTest::AnyKindTest) | NodeTest::Kind(KindTest::ElementTest) => {
                    ss = -0.5
                }
                NodeTest::Name(nt) => match (nt.ns.as_ref(), nt.name.as_ref()) {
                    (None, Some(WildcardOrName::Wildcard)) => {
                        ss = -0.25;
                    }
                    (None, Some(WildcardOrName::Name(name))) => {
                        match (n.name().get_nsuri(), n.name().get_localname()) {
                            (Some(_), _) => {}
                            (None, ename) => {
                                if *name == ename {
                                    ss = 0.5;
                                }
                            }
                        }
                    }
                    (Some(WildcardOrName::Name(ns)), Some(WildcardOrName::Name(name))) => {
                        match (n.name().get_nsuri(), n.name().get_localname()) {
                            (Some(ens), ename) => {
                                if *ns == ens && *name == ename {
                                    ss = 0.5;
                                }
                            }
                            (None, ename) => {
                                if *name == ename {
                                    ss = 0.5;
                                }
                            }
                        }
                    }
                    (Some(WildcardOrName::Wildcard), Some(WildcardOrName::Name(_))) => {
                        ss = -0.25;
                    }
                    (Some(WildcardOrName::Name(_)), Some(WildcardOrName::Wildcard)) => {
                        ss = -0.25;
                    }
                    (Some(WildcardOrName::Wildcard), Some(WildcardOrName::Wildcard)) => {
                        ss = -0.5;
                    }
                    _ => {}
                },
                _ => {}
            });
            preserve.iter().for_each(|t| match t {
                NodeTest::Kind(KindTest::AnyKindTest) | NodeTest::Kind(KindTest::ElementTest) => {
                    ps = -0.5
                }
                NodeTest::Name(nt) => match (nt.ns.as_ref(), nt.name.as_ref()) {
                    (None, Some(WildcardOrName::Name(name))) => {
                        match (n.name().get_nsuri(), n.name().get_localname()) {
                            (Some(_), _) => {}
                            (None, ename) => {
                                if *name == ename {
                                    ps = 0.5;
                                }
                            }
                        }
                    }
                    (Some(WildcardOrName::Name(ns)), Some(WildcardOrName::Name(name))) => {
                        match (n.name().get_nsuri(), n.name().get_localname()) {
                            (Some(ens), ename) => {
                                if *ns == ens && *name == ename {
                                    ps = 0.5;
                                }
                            }
                            (None, ename) => {
                                if *name == ename {
                                    ps = 0.5;
                                }
                            }
                        }
                    }
                    (Some(WildcardOrName::Wildcard), Some(WildcardOrName::Name(_))) => {
                        ps = -0.25;
                    }
                    (Some(WildcardOrName::Name(_)), Some(WildcardOrName::Wildcard)) => {
                        ps = -0.25;
                    }
                    (Some(WildcardOrName::Wildcard), Some(WildcardOrName::Wildcard)) => {
                        ps = -0.5;
                    }
                    _ => {}
                },
                _ => {}
            });
            n.child_iter().try_for_each(|m| {
                strip_whitespace_node(
                    m,
                    cpi,
                    strip,
                    preserve,
                    if ss > -1.0 {
                        if ps >= ss {
                            // Assume preserve-space is later in document order than strip-space
                            true
                        } else {
                            false
                        }
                    } else {
                        if ps > -1.0 {
                            true
                        } else {
                            keep
                        }
                    },
                )
            })?
        }
        NodeType::Text => {
            if n.to_string().trim().is_empty() && !keep {
                n.pop()?;
            }
        }
        _ => {}
    }
    Ok(())
}
