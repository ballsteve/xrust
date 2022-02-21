/*! ## An XSLT compiler

Compile an XSLT stylesheet into a [Sequence] [Constructor].

Once the stylesheet has been compiled, it may then be evaluated by the evaluation module.

*/

use std::rc::Rc;
use crate::xdmerror::*;
use crate::qname::*;
use crate::item::*;
use crate::evaluate::*;
use crate::xpath::*;

const XSLTNS: &str = "http://www.w3.org/1999/XSL/Transform";

/// Compiles a [Document] into a Sequence Constructor.
/// NB. Due to whitespace stripping, this is destructive of the stylesheet.
pub fn from_document<'a>(
  d: Rc<dyn Document>,
  resultdoc: &'a dyn Document,
  sc: &StaticContext
) -> Result<DynamicContext<'a>, Error> {
    let mut dc = DynamicContext::new(Some(resultdoc));

    // Check that this is a valid XSLT stylesheet
    let root = match d.get_root_element() {
      Some(r) => r,
      None => return Result::Err(Error{kind: ErrorKind::TypeError, message: "document does not have stylesheet element".to_string()}),
    };
    if !(root.to_name().get_nsuri_ref() == Some(XSLTNS) &&
        (root.to_name().get_localname() == "stylesheet" ||
         root.to_name().get_localname() == "transform")) {
      return Result::Err(Error{kind: ErrorKind::TypeError, message: "not an XSLT stylesheet".to_string()})
    }
    // TODO: check version attribute

    // Strip whitespace from the stylesheet
    strip_whitespace(d,
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
    dc.add_builtin_template(bi1pat, bi1bod, None, -1.0);
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
    dc.add_builtin_template(bi2pat, bi2bod, None, -1.0);
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
    dc.add_builtin_template(bi3pat, bi3bod, None, -1.0);

    // Setup the serialization of the primary result document
    for o in root.children().iter()
      .filter(|c| c.is_element() &&
		  c.to_name().get_nsuri_ref() == Some(XSLTNS) &&
		  c.to_name().get_localname() == "output") {
      match o.get_attribute(&QualifiedName::new(None, None, "indent".to_string())) {
        Some(i) => {
	  let b: bool = match i.to_string().as_str() {
	    "yes" |
	    "true" |
	    "1" => true,
	    _ => false,
	  };

      	  let mut od = OutputDefinition::new();
      	  od.set_indent(b);
      	  dc.set_output_definition(od);
	}
	None => {}
      };
    };

    // Iterate over children, looking for templates
    // * compile match pattern
    // * compile content into sequence constructor
    // * register template in dynamic context
    // TODO: Don't Panic
    for t in root.children().iter()
      //.inspect(|x| println!("checking {} node", x.node_type().to_string()))
      .filter(|c| c.is_element() &&
                  c.to_name().get_nsuri_ref() == Some(XSLTNS) &&
		  c.to_name().get_localname() == "template")
    {
      match t.get_attribute(&QualifiedName::new(None, None, "match".to_string())) {
          Some(m) => {
	    let n = m.clone().to_string();
	    let a = parse(&n).expect("failed to parse match expression");
	    let mut pat = to_pattern(a).expect("failed to compile match pattern");
	    let mut body = t.children().iter()
	      .map(|d| to_constructor(d.clone()).expect("failed to compile sequence constructor"))
	      .collect();
	    static_analysis(&mut pat, &sc);
	    static_analysis(&mut body, &sc);
	    // Determine the priority of the template
	    let prio;
	    match t.get_attribute(&QualifiedName::new(None, None, "priority".to_string())) {
	      Some(pr) => prio = pr.to_string().parse::<f64>().unwrap(), // TODO: better error handling
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
	    dc.add_template(pat, body, None, prio);
	  }
	  None => {
	    return Result::Err(Error{kind: ErrorKind::TypeError, message: "template does not have a match attribute".to_string()})
	  }
      }
    };
    Ok(dc)
}

/// Compile a node in a template to a sequence constructor
fn to_constructor(n: Rc<dyn Node>) -> Result<Constructor, Error> {
  match n.node_type() {
    NodeType::Text => {
      Ok(Constructor::Literal(Value::String(n.to_string())))
    }
    NodeType::Element => {
      match (n.to_name().get_nsuri_ref(), n.to_name().get_localname().as_str()) {
        (Some(XSLTNS), "text") => {
	  Ok(Constructor::Literal(Value::String(n.to_string())))
	}
	(Some(XSLTNS), "apply-templates") => {
	  match n.get_attribute(&QualifiedName::new(None, None, "select".to_string())) {
	    Some(sel) => {
	      Ok(Constructor::ApplyTemplates(
	        parse(&sel.to_string())?
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
	  match n.get_attribute(&QualifiedName::new(None, None, "select".to_string())) {
	    Some(s) => {
	      let cons = parse(&s.to_string())?;
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
	  match n.get_attribute(&QualifiedName::new(None, None, "test".to_string())) {
	    Some(t) => {
	      Ok(
	        // TODO: Don't Panic
		Constructor::Switch(
		  vec![
		    parse(&t.to_string())?,
		    n.children().iter()
		      .map(|d| to_constructor(d.clone()).expect("failed to compile test content"))
		      .collect()
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
	  // TODO: Don't Panic
	  n.children().iter()
	    .for_each(|m| {
	        // look for when elements
	      	// then find an otherwise
	      	// fail on anything else (apart from whitespace, comments, PIs)
		match m.node_type() {
		  NodeType::Element => {
      		    match (m.to_name().get_nsuri_ref(), m.to_name().get_localname().as_str()) {
        	      (Some(XSLTNS), "when") => {
		        if otherwise.len() == 0 {
			  match m.get_attribute(&QualifiedName::new(None, None, "test".to_string())) {
			    Some(t) => {
			      when.push(
		    	        parse(&t.to_string()).expect("failed to compile test attribute")
			      );
			      when.push(
		    	        m.children().iter()
		      		  .map(|d| to_constructor(d.clone()).expect("failed to compile when content"))
		      		  .collect()
			      );
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
			  otherwise = m.children().iter()
		      	    .map(|d| to_constructor(d.clone()).expect("failed to compile otherwise content"))
		      	    .collect()
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
	    });

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
	  match n.get_attribute(&QualifiedName::new(None, None, "select".to_string())) {
	    Some(s) => {
	      // TODO: Don't Panic
	      Ok(
	        Constructor::ForEach(
		  parse(&s.to_string())?,
		  n.children().iter()
		    .map(|d| to_constructor(d.clone()).expect("failed to compile for-each content"))
		    .collect(),
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
	  match n.get_attribute(&QualifiedName::new(None, None, "select".to_string())) {
	    Some(s) => {
	      match (n.get_attribute(&QualifiedName::new(None, None, "group-by".to_string())),
	        n.get_attribute(&QualifiedName::new(None, None, "group-adjacent".to_string())),
		n.get_attribute(&QualifiedName::new(None, None, "group-starting-with".to_string())),
		n.get_attribute(&QualifiedName::new(None, None, "group-ending-with".to_string()))) {
	        (Some(by), None, None, None) => {
		  // TODO: Don't Panic
		  Ok(
	            Constructor::ForEach(
		      parse(&s.to_string())?,
		      n.children().iter()
		        .map(|d| to_constructor(d.clone()).expect("failed to compile for-each content"))
		    	.collect(),
		      Some(Grouping::By(parse(&by.to_string()).expect("failed to compile group-by attribute"))),
		    )
	      	  )
		}
	        (None, Some(adj), None, None) => {
		  // TODO: Don't Panic
		  Ok(
	            Constructor::ForEach(
		      parse(&s.to_string())?,
		      n.children().iter()
		        .map(|d| to_constructor(d.clone()).expect("failed to compile for-each content"))
		    	.collect(),
		      Some(Grouping::Adjacent(parse(&adj.to_string())?)),
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
	  // TODO: Don't Panic
	  Ok(Constructor::Copy(
	    vec![],
	    // The content of this element is a template for the content of the new item
	    n.children().iter()
	      .map(|d| to_constructor(d.clone()).expect("failed to compile sequence constructor"))
	      .collect(),
	  ))
	}
	(Some(XSLTNS), "copy-of") => {
	  match n.get_attribute(&QualifiedName::new(None, None, "select".to_string())) {
	    Some(s) => {
	      Ok(Constructor::DeepCopy(
	        parse(&s.to_string())?,
	      ))
	    }
	    None => {
	      return Result::Err(Error{kind: ErrorKind::TypeError, message: "missing select attribute".to_string()})
	    }
	  }
	}
	(Some(XSLTNS), "attribute") => {
	  match n.get_attribute(&QualifiedName::new(None, None, "name".to_string())) {
	    Some(m) => {
      	      // TODO: Don't Panic
	      Ok(Constructor::LiteralAttribute(
	        QualifiedName::new(None, None, m.to_string()),
		n.children().iter()
		  .map(|d| to_constructor(d.clone()).expect("failed to compile attribute content"))
		  .collect(),
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
	  // TODO: Don't Panic
	  n.attributes().iter()
	      .for_each(|d| content.push(to_constructor(d.clone()).expect("failed to compile sequence constructor")));
	  // TODO: Don't Panic
	  n.children().iter()
	      .for_each(|d| content.push(to_constructor(d.clone()).expect("failed to compile sequence constructor")));
	  Ok(Constructor::LiteralElement(
	    QualifiedName::new(None, None, a.to_string()),
	    content
	  ))
	}
      }
    }
    NodeType::Attribute => {
      // Get value as a Value
      Ok(Constructor::LiteralAttribute(n.to_name(), vec![Constructor::Literal(Value::String(n.to_string()))]))
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
  d: Rc<dyn Document>,
  cpi: bool, // strip comments and PIs?
  strip: Vec<NodeTest>,
  preserve: Vec<NodeTest>,
) {
  let c = d.children();
  if c.len() == 1 {
    strip_whitespace_node(
      c[0].clone(),
      cpi,
      strip,
      preserve,
      true
    );
  }
}

/// Strip whitespace nodes from a XDM [Document].
/// This function operates under the direction of the xsl:strip-space and xsl:preserve-space directives in a XSLT stylesheet.
pub fn strip_source_document(
  src: Rc<dyn Document>,
  style: Rc<dyn Document>
) {
  // Find strip-space element, if any, and use it to construct a vector of NodeTests.
  // Ditto for preserve-space.
  let ss: Vec<NodeTest> = style.get_root_element().unwrap()	// this should be the xsl:stylesheet element
    .children().iter()
    .filter(|e| match (e.node_type(), e.to_name().get_nsuri_ref(), e.to_name().get_localname().as_str()) {
      (NodeType::Element, Some(XSLTNS), "strip-space") => true,
      _ => false,
    })
    .fold(vec![], |mut s, e| {
      match e.get_attribute(&QualifiedName::new(None, None, "elements".to_string())) {
        Some(v) => {
	  // TODO: Don't Panic
	  v.to_string().split_whitespace()
	    .for_each(|t| {
	      s.push(NodeTest::from(t).expect("not a NodeTest"))
	    })
	}
	None => {}	// should return an error
      };
      s
    });
  let ps: Vec<NodeTest> = style.get_root_element().unwrap()	// this should be the xsl:stylesheet element
    .children().iter()
    .filter(|e| match (e.node_type(), e.to_name().get_nsuri_ref(), e.to_name().get_localname().as_str()) {
      (NodeType::Element, Some(XSLTNS), "preserve-space") => true,
      _ => false,
    })
    .fold(vec![], |mut s, e| {
      match e.get_attribute(&QualifiedName::new(None, None, "elements".to_string())) {
        Some(v) => {
	  // TODO: Don't Panic
	  v.to_string().split_whitespace()
	    .for_each(|t| {
	      s.push(NodeTest::from(t).expect("not a NodeTest"))
	    })
	}
	None => {}	// should return an error
      }
      s
    });

  strip_whitespace(src, false, ss, ps);
}

// TODO: the rules for stripping/preserving are a lot more complex
// TODO: Return Result so that errors can be propagated
fn strip_whitespace_node(
  n: Rc<dyn Node>,
  cpi: bool, // strip comments and PIs?
  strip: Vec<NodeTest>,
  preserve: Vec<NodeTest>,
  keep: bool
) {
  match n.node_type() {
    NodeType::Comment |
    NodeType::ProcessingInstruction => {
      if cpi {
        n.remove().expect("unable to remove text node");
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
		  match (n.to_name().get_nsuri(), n.to_name().get_localname()) {
		    (Some(_), _) => {}
		    (None, ename) => {
		      if *name == ename {
		        ss = 0.5;
		      }
		    }
		  }
		}
	        (Some(WildcardOrName::Name(ns)), Some(WildcardOrName::Name(name))) => {
		  match (n.to_name().get_nsuri(), n.to_name().get_localname()) {
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
		  match (n.to_name().get_nsuri(), n.to_name().get_localname()) {
		    (Some(_), _) => {}
		    (None, ename) => {
		      if *name == ename {
		        ps = 0.5;
		      }
		    }
		  }
		}
	        (Some(WildcardOrName::Name(ns)), Some(WildcardOrName::Name(name))) => {
		  match (n.to_name().get_nsuri(), n.to_name().get_localname()) {
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
      n.children().iter()
        .for_each(|m| {
	  strip_whitespace_node(
	    m.clone(),
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
	});
    }
    NodeType::Text => {
      if n.to_string().trim().is_empty() && !keep {
        n.remove().expect("unable to remove text node");
      }
    }
    _ => {}
  }
}

