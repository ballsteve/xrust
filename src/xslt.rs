/*! ## An XSLT compiler

Compile an XSLT stylesheet into a [Sequence] [Constructor].

Once the stylesheet has been compiled, it may then be evaluated by the evaluation module.

*/

use std::rc::Rc;
use std::convert::TryFrom;
use url::Url;
//use reqwest::blocking::get;
use crate::xdmerror::*;
use crate::output::*;
use crate::qname::*;
use crate::value::*;
use crate::node::*;
use crate::item::*;
use crate::evaluate::*;
use crate::xpath::*;

const XSLTNS: &str = "http://www.w3.org/1999/XSL/Transform";

/// Compiles a [Document] into an Evaluator.
/// NB. Due to whitespace stripping, this is destructive of the stylesheet.
pub fn from_document<'a, F1>(
  styledoc: &'a mut Tree,
  sc: &'a mut StaticContext,
  b: Option<Url>,
  mut p: F1,		// A closure that parses a string to a dyn Document
) -> Result<Evaluator<'a>, Error>
where
  F1: FnMut(String) -> Result<Tree, Error>
{
    let mut ev = Evaluator::new();
    if b.is_some() {
	ev.set_baseurl(b.unwrap())
    }

    // Check that this is a valid XSLT stylesheet
    // There must be a single element as a child of the root node, and it must be named xsl:stylesheet or xsl:transform
    let r = styledoc.get_doc_node();
    let mut rnit = r.child_iter();
    let stylenode = match rnit.next(styledoc) {
	Some(root) => {
	    if !(root.to_name(styledoc).get_nsuri_ref() == Some(XSLTNS) &&
		 (root.to_name(styledoc).get_localname() == "stylesheet" ||
		  root.to_name(styledoc).get_localname() == "transform")) {
		return Result::Err(Error::new(ErrorKind::TypeError, "not an XSLT stylesheet".to_string()))
	    } else {
		root
	    }
	}
	None => return Result::Err(Error::new(ErrorKind::TypeError, String::from("document does not have document element")))
    };
    // TODO: rnit.next(styledoc) should == None

    // TODO: check version attribute

    // Strip whitespace from the stylesheet
    strip_whitespace(
	styledoc,
	true,
	vec![NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})],
	vec![NodeTest::Name(NameTest{ns: Some(WildcardOrName::Name(XSLTNS.to_string())), prefix: Some("xsl".to_string()), name: Some(WildcardOrName::Name("text".to_string()))})]
    );

    // Define the builtin templates
    // See XSLT 6.7. This implements text-only-copy.
    // TODO: Support deep-copy, shallow-copy, deep-skin, shallow-skip and fail

    // This matches "/" and processes the root element
    let bi1pat = to_pattern(
	vec![Constructor::Path(
	    vec![
		vec![Constructor::Root],
            ]
	)])?;
    let bi1bod = vec![
        Constructor::ApplyTemplates(
            vec![Constructor::Step(
	        NodeMatch{axis: Axis::Child, nodetest: NodeTest::Kind(KindTest::AnyKindTest)},
		vec![]
	    )],
	),
      ];
    ev.add_builtin_template(bi1pat, bi1bod, None, -1.0);
    // This matches "*" and applies templates to all children
    let bi2pat = to_pattern(
	vec![Constructor::Path(
	    vec![
		vec![Constructor::Step(
		    NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})},
		    vec![]
		)],
            ]
	)])?;
    let bi2bod = vec![
        Constructor::ApplyTemplates(
            vec![Constructor::Step(
	        NodeMatch{axis: Axis::Child, nodetest: NodeTest::Kind(KindTest::AnyKindTest)},
		vec![]
	    )],
	),
    ];
    ev.add_builtin_template(bi2pat, bi2bod, None, -1.0);
    // This matches "text()" and copies content
    let bi3pat = to_pattern(
	vec![Constructor::Path(
	    vec![
		vec![Constructor::Step(
		    NodeMatch{axis: Axis::Child, nodetest: NodeTest::Kind(KindTest::TextTest)},
	    vec![]
		)],
            ]
	)])?;
    let bi3bod = vec![Constructor::ContextItem];
    ev.add_builtin_template(bi3pat, bi3bod, None, -1.0);

    // Setup the serialization of the primary result document
    let mut serit = stylenode.child_iter();
    loop {
	match serit.next(styledoc) {
	    Some(c) => {
		if c.is_element(styledoc) &&
		    c.to_name(styledoc).get_nsuri_ref() == Some(XSLTNS) &&
		    c.to_name(styledoc).get_localname() == "output" {
			match c.get_attribute(styledoc, &QualifiedName::new(None, None, "indent".to_string())) {
			    Some(i) => {
				let b: bool = match i.to_string(styledoc).as_str() {
				    "yes" |
				    "true" |
				    "1" => true,
				    _ => false,
				};

      				let mut od = OutputDefinition::new();
      				od.set_indent(b);
      				ev.set_output_definition(od);
			    }
			    None => {}
			}
		}
	    }
	    None => break,
	}
    }

    // Iterate over children, looking for includes
    // * resolve href
    // * fetch document
    // * parse XML
    // * replace xsl:include element with content
    let mut incit = stylenode.child_iter();
    loop {
	match incit.next(styledoc) {
	    Some(c) => {
		if c.is_element(styledoc) &&
		    c.to_name(styledoc).get_nsuri_ref() == Some(XSLTNS) &&
		    c.to_name(styledoc).get_localname() == "include" {
			match c.get_attribute(styledoc, &QualifiedName::new(None, None, "href".to_string())) {
			    Some(h) => {
				let url = match ev.baseurl().map_or_else(
				    || Url::parse(h.to_string(styledoc).as_str()),
				    |base| base.join(h.to_string(styledoc).as_str()),
				) {
				    Ok(u) => u,
				    Err(_) => return Result::Err(Error{kind: ErrorKind::Unknown, message: "unable to parse href URL".to_string()}),
				};
				let xml = reqwest::blocking::get(url.to_string())
				    .map_err(|_| Error{kind: ErrorKind::Unknown, message: "unable to fetch href URL".to_string()})?
				    .text()
				    .map_err(|_| Error{kind: ErrorKind::Unknown, message: "unable to extract module data".to_string()})?;
				let _module = p(xml)?;
	
			    }
			    None => {
				return Result::Err(Error{kind: ErrorKind::TypeError, message: "include does not have a href attribute".to_string()})
			    }
			}
		    }
	    }
	    None => break,
	}
    }

    // Iterate over children, looking for templates
    // * compile match pattern
    // * compile content into sequence constructor
    // * register template in dynamic context
    let mut tempit = stylenode.child_iter();
    loop {
	match tempit.next(styledoc) {
	    Some(c) => {
		if c.is_element(styledoc) &&
		    c.to_name(styledoc).get_nsuri_ref() == Some(XSLTNS) &&
		    c.to_name(styledoc).get_localname() == "template" {
			match c.get_attribute(styledoc, &QualifiedName::new(None, None, "match".to_string())) {
			    Some(m) => {
				let n = m.clone().to_string(styledoc);
				let a = parse(&n).expect("failed to parse match expression");
				let mut pat = to_pattern(a).expect("failed to compile match pattern");
				let mut body = vec![];
				let mut cit = c.child_iter();
				loop {
				    match cit.next(styledoc) {
					Some(d) => {
					    body.push(to_constructor(d, styledoc)?)
					}
					None => break,
				    }
				}
				sc.static_analysis(&mut pat);
				sc.static_analysis(&mut body);
				// Determine the priority of the template
				let prio;
				match c.get_attribute(styledoc, &QualifiedName::new(None, None, "priority".to_string())) {
				    Some(pr) => prio = pr.to_string(styledoc).parse::<f64>().unwrap(), // TODO: better error handling
				    None => {
					// Calculate the default priority
					// TODO: more work to be done interpreting XSLT 6.5
					if pat.len() <= 1 {
					    match &pat[0] {
						Constructor::Root => prio = -0.5,
						Constructor::Path(_) => prio = -0.5,
						Constructor::Step(nm, _pred) => {
						    match &nm.nodetest {
							NodeTest::Name(nt) => {
							    match (nt.ns.as_ref(), nt.name.as_ref()) {
								(Some(WildcardOrName::Wildcard), Some(WildcardOrName::Wildcard)) => prio =-0.5,
								(Some(WildcardOrName::Wildcard), Some(WildcardOrName::Name(_))) |
								(Some(WildcardOrName::Name(_)), Some(WildcardOrName::Wildcard)) => prio = -0.25,
								(None, Some(WildcardOrName::Wildcard)) => prio = -0.25,
								(Some(WildcardOrName::Name(_)), Some(WildcardOrName::Name(_))) => prio = 0.0,
								(None, Some(WildcardOrName::Name(_))) => prio = 0.0,
								_ => prio = 0.5,
							    }
							}
							NodeTest::Kind(kt) => {
							    match kt {
								KindTest::DocumentTest |
								KindTest::ElementTest |
								KindTest::AttributeTest => prio = -0.5,
								_ => prio = 0.5,
							    }
							}
						    }
						}
						_ => prio = 0.5,
					    }
					} else {
					    // TODO: calculate the priority of each branch of the pattern
					    prio = 0.5
					}
				    }
				}
				ev.add_template(pat, body, None, prio);
			    }
			    None => {
				return Result::Err(Error{kind: ErrorKind::TypeError, message: "template does not have a match attribute".to_string()})
			    }
			}
		    }
	    }
	    None => break,
	}
    }

    Ok(ev)
}

/// Compile a node in a template to a sequence constructor
fn to_constructor(n: Node, d: &Tree) -> Result<Constructor, Error> {
    match n.node_type(d) {
	NodeType::Text => {
	    Ok(Constructor::Literal(Value::String(n.to_string(d))))
	}
	NodeType::Element => {
	    match (n.to_name(d).get_nsuri_ref(), n.to_name(d).get_localname().as_str()) {
		(Some(XSLTNS), "text") => {
		    match n.get_attribute(d, &QualifiedName::new(None, None, "disable-output-escaping".to_string())){
			Some(doe) => {
			    match &doe.to_string(d)[..]  {
				"yes" => Ok(Constructor::Literal(Value::String(n.to_string(d)))),
				"no" => {
				    let text = n.to_string(d)
					.replace("&","&amp;")
					.replace(">", "&gt;")
					.replace("<", "&lt;")
					.replace("'", "&apos;")
					.replace("\"", "&quot;");
				    Ok(Constructor::Literal(Value::from(text)))
				}
				_ => {
				    return Result::Err(Error{kind: ErrorKind::TypeError, message: "disable-output-escaping only accepts values yes or no.".to_string()})
				}
			    }
			}
			None => {
			    let text = n.to_string(d)
				.replace("&","&amp;")
				.replace(">", "&gt;")
				.replace("<", "&lt;")
				.replace("'", "&apos;")
				.replace("\"", "&quot;");
			    Ok(Constructor::Literal(Value::from(text)))
			}
		    }
		}
		(Some(XSLTNS), "apply-templates") => {
		    match n.get_attribute(d, &QualifiedName::new(None, None, "select".to_string())) {
			Some(sel) => {
			    Ok(Constructor::ApplyTemplates(
				parse(&sel.to_string(d))?
			    ))
			}
			None => {
			    // If there is no select attribute, then default is "child::node()"
			    Ok(Constructor::ApplyTemplates(
				vec![
	      			    Constructor::Step(
					NodeMatch{
	      				    axis: Axis::Child,
	      				    nodetest: NodeTest::Kind(KindTest::AnyKindTest)
	    				},
	    				vec![]
	      			    )
	    			]
			    ))
			}
		    }
		}
		(Some(XSLTNS), "sequence") => {
		    match n.get_attribute(d, &QualifiedName::new(None, None, "select".to_string())) {
			Some(s) => {
			    let cons = parse(&s.to_string(d))?;
			    if cons.len() > 1 {
				return Result::Err(Error{kind: ErrorKind::TypeError, message: "select attribute has more than one sequence constructor".to_string()})
			    }
			    Ok(cons[0].clone())
			}
			None => {
			    return Result::Err(Error{kind: ErrorKind::TypeError, message: "missing select attribute".to_string()})
			}
		    }
		}
		(Some(XSLTNS), "if") => {
		    match n.get_attribute(d, &QualifiedName::new(None, None, "test".to_string())) {
			Some(t) => {
			    let mut cit = n.child_iter();
			    let mut body = vec![];
			    loop {
				match cit.next(d) {
				    Some(e) => {
					body.push(to_constructor(e, d)?)
				    }
				    None => break,
				}
			    }
			    Ok(
				Constructor::Switch(
				    vec![
					parse(&t.to_string(d))?,
					body
				    ],
				    vec![],
				)
			    )
			}
			None => {
			    return Result::Err(Error{kind: ErrorKind::TypeError, message: "missing test attribute".to_string()})
			}
		    }
		}
		(Some(XSLTNS), "choose") => {
		    let mut when: Vec<Vec<Constructor>> = Vec::new();
		    let mut otherwise: Vec<Constructor> = Vec::new();
		    let mut status: Option<Error> = None;
		    let mut cit = n.child_iter();
		    loop {
			match cit.next(d) {
			    Some(m) => {
				// look for when elements
	      			// then find an otherwise
	      			// fail on anything else (apart from whitespace, comments, PIs)
				match m.node_type(d) {
				    NodeType::Element => {
      					match (m.to_name(d).get_nsuri_ref(), m.to_name(d).get_localname().as_str()) {
        				    (Some(XSLTNS), "when") => {
						if otherwise.len() == 0 {
						    match m.get_attribute(d, &QualifiedName::new(None, None, "test".to_string())) {
							Some(t) => {
							    let mut wit = m.child_iter();
							    let mut body = vec![];
							    loop {
								match wit.next(d) {
								    Some(e) => {
									body.push(to_constructor(e, d)?)
								    }
								    None => break,
								}
							    }
							    when.push(
		    						parse(&t.to_string(d))?
							    );
							    when.push(body);
							}
	    						None => {
	      						    status.replace(Error{kind: ErrorKind::TypeError, message: "missing test attribute".to_string()});
	    						}
						    }
						} else {
						    status.replace(Error{kind: ErrorKind::TypeError, message: "invalid content in choose element: when follows otherwise".to_string()});
						}
					    }
        				    (Some(XSLTNS), "otherwise") => {
						if when.len() != 0 {
						    let mut oit = m.child_iter();
						    loop {
							match oit.next(d) {
							    Some(e) => {
								otherwise.push(to_constructor(e, d)?)
							    }
							    None => break,
							}
						    }
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
					if !n.to_string(d).trim().is_empty() {
					    status.replace(Error{kind: ErrorKind::TypeError, message: "invalid text content in choose element".to_string()});
					}
				    }
				    NodeType::Comment |
				    NodeType::ProcessingInstruction => {}
				    _ => {
					status.replace(Error{kind: ErrorKind::TypeError, message: "invalid content in choose element".to_string()});
				    }
				}
			    }
			    None => break,
			}
		    }
		    match status {
			Some(e) => Result::Err(e),
			None => Ok(
			    Constructor::Switch(
				when,
				otherwise,
			    )
			)
		    }
		}
		(Some(XSLTNS), "for-each") => {
		    match n.get_attribute(d, &QualifiedName::new(None, None, "select".to_string())) {
			Some(s) => {
			    let mut cit = n.child_iter();
			    let mut body = vec![];
			    loop {
				match cit.next(d) {
				    Some(e) => {
					body.push(to_constructor(e, d)?)
				    }
				    None => break,
				}
			    }
			    Ok(
				Constructor::ForEach(
				    parse(&s.to_string(d))?,
				    body,
				    None,
				)
			    )
			}
			None => {
			    return Result::Err(Error{kind: ErrorKind::TypeError, message: "missing select attribute".to_string()})
			}
		    }
		}
		(Some(XSLTNS), "for-each-group") => {
		    match n.get_attribute(d, &QualifiedName::new(None, None, "select".to_string())) {
			Some(s) => {
			    match (n.get_attribute(d, &QualifiedName::new(None, None, "group-by".to_string())),
				   n.get_attribute(d, &QualifiedName::new(None, None, "group-adjacent".to_string())),
				   n.get_attribute(d, &QualifiedName::new(None, None, "group-starting-with".to_string())),
				   n.get_attribute(d, &QualifiedName::new(None, None, "group-ending-with".to_string()))) {
				(Some(by), None, None, None) => {
				    let mut cit = n.child_iter();
				    let mut body = vec![];
				    loop {
					match cit.next(d) {
					    Some(e) => {
						body.push(to_constructor(e, d)?)
					    }
					    None => break,
					}
				    }
				    Ok(
					Constructor::ForEach(
					    parse(&s.to_string(d))?,
					    body,
					    Some(Grouping::By(parse(&by.to_string(d))?)),
					)
	      			    )
				}
				(None, Some(adj), None, None) => {
				    let mut cit = n.child_iter();
				    let mut body = vec![];
				    loop {
					match cit.next(d) {
					    Some(e) => {
						body.push(to_constructor(e, d)?)
					    }
					    None => break,
					}
				    }
				    Ok(
					Constructor::ForEach(
					    parse(&s.to_string(d))?,
					    body,
					    Some(Grouping::Adjacent(parse(&adj.to_string(d))?)),
					)
	      			    )
				}
				// TODO: group-starting-with and group-ending-with
				_ => {
				    Result::Err(Error{kind: ErrorKind::NotImplemented, message: "invalid grouping attribute(s) specified".to_string()})
				}
			    }
			}
			None => {
			    return Result::Err(Error{kind: ErrorKind::TypeError, message: "missing select attribute".to_string()})
			}
		    }
		}
		(Some(XSLTNS), "copy") => {
		    // TODO: handle select attribute
		    let mut cit = n.child_iter();
		    let mut body = vec![];
		    loop {
			match cit.next(d) {
			    Some(e) => {
				body.push(to_constructor(e, d)?)
			    }
			    None => break,
			}
		    }
		    Ok(Constructor::Copy(
			vec![],
			// The content of this element is a template for the content of the new item
			body,
		    ))
		}
		(Some(XSLTNS), "copy-of") => {
		    match n.get_attribute(d, &QualifiedName::new(None, None, "select".to_string())) {
			Some(s) => {
			    Ok(Constructor::DeepCopy(
				parse(&s.to_string(d))?,
			    ))
			}
			None => {
			    return Result::Err(Error{kind: ErrorKind::TypeError, message: "missing select attribute".to_string()})
			}
		    }
		}
		(Some(XSLTNS), "attribute") => {
		    match n.get_attribute(d, &QualifiedName::new(None, None, "name".to_string())) {
			Some(m) => {
			    let mut cit = n.child_iter();
			    let mut body = vec![];
			    loop {
				match cit.next(d) {
				    Some(e) => {
					body.push(to_constructor(e, d)?)
				    }
				    None => break,
				}
			    }
			    Ok(Constructor::LiteralAttribute(
				QualifiedName::new(None, None, m.to_string(d)),
				body,
			    ))
			}
			None => {
			    return Result::Err(Error{kind: ErrorKind::TypeError, message: "missing select attribute".to_string()})
			}
		    }
		}
		(Some(XSLTNS), u) => {
		    Ok(Constructor::NotImplemented(format!("unsupported XSL element \"{}\"", u)))
		}
		(_, a) => {
		    // TODO: Handle qualified element name
		    let mut content = vec![];
		    let mut ait = n.attribute_iter(d);
		    loop {
			match ait.next() {
			    Some(e) => {
				content.push(to_constructor(e, d)?)
			    }
			    None => break,
			}
		    }
		    let mut cit = n.child_iter();
		    loop {
			match cit.next(d) {
			    Some(e) => {
				content.push(to_constructor(e, d)?)
			    }
			    None => break,
			}
		    }
		    Ok(Constructor::LiteralElement(
			QualifiedName::new(None, None, a.to_string()),
			content
		    ))
		}
	    }
	}
	NodeType::Attribute => {
	    // Get value as a Value
	    Ok(Constructor::LiteralAttribute(n.to_name(d), vec![Constructor::Literal(Value::String(n.to_string(d)))]))
	}
	_ => {
	    // TODO: literal elements, etc, pretty much everything in the XSLT spec
	    Ok(Constructor::NotImplemented("other template content".to_string()))
	}
    }
}

/// Strip whitespace nodes from a XDM [Document].
/// See [XSLT 4.3](https://www.w3.org/TR/2017/REC-xslt-30-20170608/#stylesheet-stripping)
pub fn strip_whitespace(
  d: &mut Tree,
  cpi: bool, // strip comments and PIs?
  strip: Vec<NodeTest>,
  preserve: Vec<NodeTest>,
) {
    let mut cit = d.get_doc_node().child_iter();
    match cit.next(d) {
	Some(n) => strip_whitespace_node(
	    n,
	    d,
	    cpi,
	    strip,
	    preserve,
	    true
	),
	None => {}
    }
}

/// Strip whitespace nodes from a XDM [Document].
/// This function operates under the direction of the xsl:strip-space and xsl:preserve-space directives in a XSLT stylesheet.
pub fn strip_source_document(
  src: &mut Tree,
  style: &Tree,
) {
    // Find strip-space element, if any, and use it to construct a vector of NodeTests.
    // Ditto for preserve-space.
    let mut ss: Vec<NodeTest> = vec![];
    let mut ps: Vec<NodeTest> = vec![];
    let mut sit = style.get_doc_node().child_iter();
    match sit.next(style) {
	Some(n) => {
	    // this should be the xsl:stylesheet element
	    let mut cit = n.child_iter();
	    loop {
		match cit.next(style) {
		    Some(m) => {
			match (m.node_type(style), m.to_name(style).get_nsuri_ref(), m.to_name(style).get_localname().as_str()) {
			    (NodeType::Element, Some(XSLTNS), "strip-space") => {
				match m.get_attribute(style, &QualifiedName::new(None, None, "elements".to_string())) {
				    Some(v) => {
					// TODO: Don't Panic
					v.to_string(style).split_whitespace()
					    .for_each(|t| {
						ss.push(NodeTest::try_from(t).expect("not a NodeTest"))
					    })
				    }
				    None => {}	// should return an error
				}
			    }
			    (NodeType::Element, Some(XSLTNS), "preserve-space") => {
				match m.get_attribute(style, &QualifiedName::new(None, None, "elements".to_string())) {
				    Some(v) => {
					// TODO: Don't Panic
					v.to_string(style).split_whitespace()
					    .for_each(|t| {
						ps.push(NodeTest::try_from(t).expect("not a NodeTest"))
					    })
				    }
				    None => {}	// should return an error
				}
			    }
			    _ => {},
			}
		    }
		    None => break,
		}
	    }
	}
	None => {}
    }

    strip_whitespace(src, false, ss, ps);
}

// TODO: the rules for stripping/preserving are a lot more complex
// TODO: Return Result so that errors can be propagated
fn strip_whitespace_node(
    n: Node,
    d: &mut Tree,
    cpi: bool, // strip comments and PIs?
    strip: Vec<NodeTest>,
    preserve: Vec<NodeTest>,
    keep: bool
) {
    match n.node_type(d) {
	NodeType::Comment |
	NodeType::ProcessingInstruction => {
	    if cpi {
		n.remove(d).expect("unable to remove text node");
      		// TODO: Merge text nodes that are now adjacent
	    }
	}
	NodeType::Element => {
	    // Determine if this element toggles the strip/preserve setting
	    // Match a strip NodeTest or a preserve NodeTest
	    // The 'strength' of the match determines which setting wins
	    let mut ss = -1.0;
	    let mut ps = -1.0;
	    strip.iter()
		.for_each(|t| {
		    match t {
			NodeTest::Kind(KindTest::AnyKindTest) |
			NodeTest::Kind(KindTest::ElementTest) => ss = -0.5,
			NodeTest::Name(nt) => {
			    match (nt.ns.as_ref(), nt.name.as_ref()) {
				(None, Some(WildcardOrName::Wildcard)) => {
				    ss = -0.25;
				}
				(None, Some(WildcardOrName::Name(name))) => {
				    match (n.to_name(d).get_nsuri(), n.to_name(d).get_localname()) {
					(Some(_), _) => {}
					(None, ename) => {
					    if *name == ename {
						ss = 0.5;
					    }
					}
				    }
				}
				(Some(WildcardOrName::Name(ns)), Some(WildcardOrName::Name(name))) => {
				    match (n.to_name(d).get_nsuri(), n.to_name(d).get_localname()) {
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
			    }
			}
			_ => {}
		    }
		});
	    preserve.iter()
		.for_each(|t| {
		    match t {
			NodeTest::Kind(KindTest::AnyKindTest) |
			NodeTest::Kind(KindTest::ElementTest) => ps = -0.5,
			NodeTest::Name(nt) => {
			    match (nt.ns.as_ref(), nt.name.as_ref()) {
				(None, Some(WildcardOrName::Name(name))) => {
				    match (n.to_name(d).get_nsuri(), n.to_name(d).get_localname()) {
					(Some(_), _) => {}
					(None, ename) => {
					    if *name == ename {
						ps = 0.5;
					    }
					}
				    }
				}
				(Some(WildcardOrName::Name(ns)), Some(WildcardOrName::Name(name))) => {
				    match (n.to_name(d).get_nsuri(), n.to_name(d).get_localname()) {
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
			    }
			}
			_ => {}
		    }
		});
	    let mut cit = n.child_iter();
	    loop {
		match cit.next(d) {
		    Some(m) => {
			strip_whitespace_node(
			    m,
			    d,
			    cpi,
			    strip.clone(),  // TODO: borrow instead
			    preserve.clone(), // TODO: borrow instead
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
			    }
			)
		    }
		    None => break,
		}
	    }
	}
	NodeType::Text => {
	    if n.to_string(d).trim().is_empty() && !keep {
		n.remove(d).expect("unable to remove text node");
	    }
	}
	_ => {}
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn xslt_literal_text() {
	let mut sc = StaticContext::new_with_xslt_builtins();

	let src = Tree::try_from("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("unable to parse XML");
	let isrc = Rc::new(Item::Node(src.get_doc_node()));

	let mut style = Tree::try_from("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'>Found the document</xsl:template>
</xsl:stylesheet>").expect("unable to parse XML");

	// Setup dynamic context with result document
	let mut ev = from_document(
            &mut style,
	    &mut sc,
	    None,
	    |s| Tree::try_from(s.as_str()),
	)
            .expect("failed to compile stylesheet");
	ev.set_doc(&src);
	eprintln!("Evaluator templates:");
	ev.dump_templates();

	// Prime the stylesheet evaluation by finding the template for the document root
	// and making the document root the initial context
	eprintln!("find match");
	let t = ev.find_match(&isrc).expect("unable to find match");
	assert!(t.len() >= 1);

	let mut rd = Tree::new();
	eprintln!("evaluate");
	let seq = ev.evaluate(Some(vec![Rc::clone(&isrc)]), Some(0), &t, &mut rd).expect("evaluation failed");

	assert_eq!(seq.to_string(Some(&rd)), "Found the document")
    }
}
