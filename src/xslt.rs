/*! ## An XSLT compiler

Compile an XSLT stylesheet into a [Sequence] [Constructor].

Once the stylesheet has been compiled, it may then be evaluated by the evaluation module.

```rust
# use std::rc::Rc;
# use xrust::xdmerror::*;
# use xrust::item::{Item, Document};
# use xrust::evaluate::*;
# use xrust::xpath::*;
# use xrust::xslt::*;
# use libxml::tree::{NodeType as libxmlNodeType, Document as libxmlDocument, Node as libxmlNode, set_node_rc_guard};
# use libxml::parser::Parser;

# set_node_rc_guard(4);

// We're going to need to statically analyze the sequence constructor later on
let sc = StaticContext::new_with_builtins();

// This is the source document for the transformation
let psrc = Parser::default();
let doc = psrc.parse_string("<Test>Check, one, two</Test>")
  .expect("failed to parse source document");
let srcdoc = Rc::new(doc) as Rc<dyn Document>;

// This is the stylesheet document
let psty = Parser::default();
let style = psty.parse_string("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::Test'><html><body><xsl:apply-templates/></body></html></xsl:template>
  <xsl:template match='child::text()'><p><xsl:sequence select='.'/></p></xsl:template>
</xsl:stylesheet>")
  .expect("failed to parse source document");

// Now compile the stylesheet
let dc = from_document(style, &sc).expect("failed to compile stylesheet");

// The source document is the initial context.
// Find the template that matches it,
// and use that to start the transformation
let item = Rc::new(Item::Document(srcdoc));
let mut template = dc.find_match(&item);

// Now evaluate the stylesheet
let sequence = evaluate(&dc, Some(vec![item]), Some(0), &template)
  .expect("stylesheet evaluation failed");

assert_eq!(sequence.to_xml(), "<html><body><p>Check, one, two</p></body></html>")
```

*/

use std::rc::Rc;
use crate::xdmerror::*;
use crate::item::*;
use crate::evaluate::*;
use crate::xpath::*;

const XSLTNS: &str = "http://www.w3.org/1999/XSL/Transform";

/// Compiles a [Document] into a Sequence Constructor.
///
/// If the stylesheet creates any elements in the result tree, they are created using the [DynamicContext]'s [Document] [Node] creation method.
pub fn from_document<'a>(d: Rc<dyn Document>, sc: &'a StaticContext<'a>) -> Result<DynamicContext<'a>, Error> {
    let mut dc = DynamicContext::new();

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

    // Strip/preserve whitespace
    // TODO

    // Iterate over children, looking for templates
    // * compile match pattern
    // * compile content into sequence constructor
    // * register template in dynamic context
    for t in root.children().iter()
      .filter(|c| c.is_element() &&
                  c.to_name().get_nsuri_ref() == Some(XSLTNS) &&
		  c.to_name().get_localname() == "template") {
      match t.attribute("match") {
          Some(m) => {
	    let n = m.clone();
	    let a = parse(&n).expect("failed to parse match expression");
	    let mut pat = to_pattern(a).expect("failed to compile match pattern");
	    let mut body = t.children().iter()
	      .map(|d| to_constructor(d.clone()).expect("failed to compile sequence constructor"))
	      .collect();
	    static_analysis(&mut pat, &sc);
	    static_analysis(&mut body, &sc);
	    dc.add_template(pat, body);
	  }
	  None => {
	    return Result::Err(Error{kind: ErrorKind::TypeError, message: "template does not have a match attribute".to_string()})
	  }
      }
    };
    Ok(dc)
}

/// Compile a node in a template to a sequence constructor
fn to_constructor<'a>(n: Rc<dyn Node>) -> Result<Constructor<'a>, Error> {
  match n.node_type() {
    NodeType::Text => {
      Ok(Constructor::Literal(Value::String(n.to_string())))
    }
    NodeType::Element => {
      match (n.to_name().get_nsuri_ref(), n.to_name().get_localname().as_str()) {
        (Some(XSLTNS), "apply-templates") => {
	  match n.attribute("select") {
	    Some(sel) => {
	      Ok(Constructor::ApplyTemplates(
	        parse(&sel).expect("failed to compile select attribute")
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
	  match n.attribute("select") {
	    Some(s) => {
	      let cons = parse(&s).expect("failed to compile select attribute");
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
	  match n.attribute("test") {
	    Some(t) => {
	      Ok(
	        Constructor::Switch(
		  vec![
		    parse(&t).expect("failed to compile test attribute"),
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
	  let mut itr = n.children().iter();

	  loop {
	    match itr.next() {
	      Some(m) => {
	        // look for when elements
	      	// then find an otherwise
	      	// fail on anything else (apart from whitespace, comments, PIs)
		match m.node_type() {
		  NodeType::Element => {
      		    match (m.to_name().get_nsuri_ref(), m.to_name().get_localname().as_str()) {
        	      (Some(XSLTNS), "when") => {
		        if otherwise.len() == 0 {
			  match m.attribute("test") {
			    Some(t) => {
			      when.push(
		    	        parse(&t).expect("failed to compile test attribute")
			      );
			      when.push(
		    	        m.children().iter()
		      		  .map(|d| to_constructor(d.clone()).expect("failed to compile when content"))
		      		  .collect()
			      );
			    }
	    		    None => {
	      		      return Result::Err(Error{kind: ErrorKind::TypeError, message: "missing test attribute".to_string()})
	    		    }
			  }
			} else {
			  return Result::Err(Error{kind: ErrorKind::TypeError, message: "invalid content in choose element: when follows otherwise".to_string()})
			}
		      }
        	      (Some(XSLTNS), "otherwise") => {
		        if when.len() != 0 {
			  otherwise = m.children().iter()
		      	    .map(|d| to_constructor(d.clone()).expect("failed to compile otherwise content"))
		      	    .collect()
			} else {
			  return Result::Err(Error{kind: ErrorKind::TypeError, message: "invalid content in choose element: no when elements".to_string()})
			}
		      }
		      _ => {
			 return Result::Err(Error{kind: ErrorKind::TypeError, message: "invalid element content in choose element".to_string()})
		      }
		    }
		  }
		  NodeType::Text => {
		    if !n.to_string().trim().is_empty() {
		      return Result::Err(Error{kind: ErrorKind::TypeError, message: "invalid text content in choose element".to_string()})
		    }
		  }
		  NodeType::Comment |
		  NodeType::ProcessingInstruction => {}
		  _ => return Result::Err(Error{kind: ErrorKind::TypeError, message: "invalid content in choose element".to_string()})
		}
	      }
	      None => break,
	    }
	  }

	  Ok(
	    Constructor::Switch(
	      when,
	      otherwise,
	    )
	  )
	}
	(Some(XSLTNS), "for-each") => {
	  match n.attribute("select") {
	    Some(s) => {
	      Ok(
	        Constructor::ForEach(
		  parse(&s).expect("failed to compile select attribute"),
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
	  match n.attribute("select") {
	    Some(s) => {
	      match (n.attribute("group-by"), n.attribute("group-adjacent"), n.attribute("group-starting-with"), n.attribute("group-ending-with")) {
	        (Some(by), None, None, None) => {
		  Ok(
	            Constructor::ForEach(
		      parse(&s).expect("failed to compile select attribute"),
		      n.children().iter()
		        .map(|d| to_constructor(d.clone()).expect("failed to compile for-each content"))
		    	.collect(),
		      Some(Grouping::By(parse(&by).expect("failed to compile group-by attribute"))),
		    )
	      	  )
		}
	        (None, Some(adj), None, None) => {
		  Ok(
	            Constructor::ForEach(
		      parse(&s).expect("failed to compile select attribute"),
		      n.children().iter()
		        .map(|d| to_constructor(d.clone()).expect("failed to compile for-each content"))
		    	.collect(),
		      Some(Grouping::Adjacent(parse(&adj).expect("failed to compile group-adjacent attribute"))),
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
	(Some(XSLTNS), _) => {
	  Ok(Constructor::NotImplemented("unsupported XSL element"))
	}
	(_, a) => {
	  // TODO: Handle qualified element name
	  Ok(Constructor::LiteralElement(a.to_string(), "".to_string(), "".to_string(),
	    n.children().iter()
	      .map(|d| to_constructor(d.clone()).expect("failed to compile sequence constructor"))
	      .collect(),
	  ))
	}
      }
    }
    _ => {
      // TODO: literal elements, etc, pretty much everything in the XSLT spec
      Ok(Constructor::NotImplemented("other template content"))
    }
  }
}
