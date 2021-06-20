/*! ## An XSLT compiler

Compile an XSLT stylesheet into a [Sequence] [Constructor].

Once the stylesheet has been compiled, it may then be evaluated by the evaluation module.

```rust
# use std::rc::Rc;
# use xrust::xdmerror::*;
# use xrust::item::*;
# use roxmltree::{Document, Node};
# use xrust::evaluate::*;
# use xrust::xpath::*;
# use xrust::xslt::*;

// We're going to need to statically analyze the sequence constructor later on
let sc = StaticContext::new_with_builtins();

// This is the source document for the transformation
let source = roxmltree::Document::parse("<Test>Check, one, two</Test>")
  .expect("failed to parse source document");

// This is the stylesheet document
let style = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::Test'><html><body><xsl:apply-templates/></body></html></xsl:template>
  <xsl:template match='child::text()'><p><xsl:sequence select='.'/></p></xsl:template>
</xsl:stylesheet>")
  .expect("failed to parse stylesheet document");

// Now compile the stylesheet
let dc = from_xnode(&style, &sc).expect("failed to compile stylesheet");

// The source document root node is the initial context.
// Find the template that matches it,
// and use that to start the transformation
let item = Rc::new(Item::XNode(source.root().first_child().unwrap()));
let mut template = dc.find_match(&item);

// Now evaluate the stylesheet
let sequence = evaluate(&dc, Some(vec![item.clone()]), Some(0), &template)
  .expect("stylesheet evaluation failed");

assert_eq!(sequence.to_xml(), "<html><body><p>Check, one, two</p></body></html>")
```

*/

use std::rc::Rc;
use crate::xdmerror::*;
use crate::item::*;
use roxmltree::{Document, Node};
use crate::evaluate::*;
use crate::xpath::*;

const XSLTNS: &str = "http://www.w3.org/1999/XSL/Transform";

/// Compiles an XML document into a Sequence Constructor.
///
/// This function takes a [roxmltree::Document](https://docs.rs/roxmltree/0.14.1/roxmltree/struct.Document.html) as its argument.
///
/// If the stylesheet creates any elements in the result tree, they are created as [trees](https://crates.io/crates/trees) Nodes.
pub fn from_xnode<'a>(d: &'a Document<'a>, sc: &'a StaticContext<'a>) -> Result<DynamicContext<'a>, Error> {
    let mut dc = DynamicContext::new();

    // Check that this is a valid XSLT stylesheet
    let root = match d.root().first_child() {
      Some(r) => r,
      None => return Result::Err(Error{kind: ErrorKind::TypeError, message: "document does not have stylesheet element".to_string()}),
    };
    if !(root.tag_name().namespace() == Some(XSLTNS) &&
        (root.tag_name().name() == "stylesheet" ||
         root.tag_name().name() == "transform")) {
      return Result::Err(Error{kind: ErrorKind::TypeError, message: "not an XSLT stylesheet".to_string()})
    }
    // TODO: check version attribute

    // Strip/preserve whitespace
    // TODO

    // Iterate over children, looking for templates
    // * compile match pattern
    // * compile content into sequence constructor
    // * register template in dynamic context
    for t in root.children()
      .filter(|c| c.is_element() &&
                  c.tag_name().namespace() == Some(XSLTNS) &&
		  c.tag_name().name() == "template") {
      match t.attribute("match") {
          Some(m) => {
	    let a = parse(m.clone()).expect("failed to parse match expression");
	    let mut pat = to_pattern(a).expect("failed to compile match pattern");
	    let mut body = t.children()
	      .map(|d| to_constructor(d).expect("failed to compile sequence constructor"))
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
fn to_constructor<'a, 'input>(n: Node<'a, 'input>) -> Result<Constructor<'a>, Error> {
  match n.node_type() {
    roxmltree::NodeType::Text => {
      match n.text() {
        Some(t) => {
	  Ok(Constructor::Literal(Value::String(t.to_string())))
	}
	None => {
	  // Shouldn't get here
	  Ok(Constructor::Literal(Value::String("".to_string())))
	}
      }
    }
    roxmltree::NodeType::Element => {
      match (n.tag_name().namespace(), n.tag_name().name()) {
        (Some(XSLTNS), "apply-templates") => {
	  match n.attribute("select") {
	    Some(sel) => {
	      Ok(Constructor::ApplyTemplates(
	        parse(sel.clone()).expect("failed to compile select attribute")
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
	      let cons = parse(s.clone()).expect("failed to compile select attribute");
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
		    parse(t.clone()).expect("failed to compile test attribute"),
		    n.children()
		      .map(|d| to_constructor(d).expect("failed to compile test content"))
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
	  let mut itr = n.children();

	  loop {
	    match itr.next() {
	      Some(m) => {
	        // look for when elements
	      	// then find an otherwise
	      	// fail on anything else (apart from whitespace, comments, PIs)
		match m.node_type() {
		  roxmltree::NodeType::Element => {
      		    match (m.tag_name().namespace(), m.tag_name().name()) {
        	      (Some(XSLTNS), "when") => {
		        if otherwise.len() == 0 {
			  match m.attribute("test") {
			    Some(t) => {
			      when.push(
		    	        parse(t.clone()).expect("failed to compile test attribute")
			      );
			      when.push(
		    	        m.children()
		      		  .map(|d| to_constructor(d).expect("failed to compile when content"))
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
			  otherwise = m.children()
		      	    .map(|d| to_constructor(d).expect("failed to compile otherwise content"))
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
		  roxmltree::NodeType::Text => {
		    if !n.text().unwrap().trim().is_empty() {
		      return Result::Err(Error{kind: ErrorKind::TypeError, message: "invalid text content in choose element".to_string()})
		    }
		  }
		  roxmltree::NodeType::Comment |
		  roxmltree::NodeType::PI => {}
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
		  parse(s.clone()).expect("failed to compile select attribute"),
		  n.children()
		    .map(|d| to_constructor(d).expect("failed to compile for-each content"))
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
		      parse(s.clone()).expect("failed to compile select attribute"),
		      n.children()
		        .map(|d| to_constructor(d).expect("failed to compile for-each content"))
		    	.collect(),
		      Some(Grouping::By(parse(by.clone()).expect("failed to compile group-by attribute"))),
		    )
	      	  )
		}
	        (None, Some(adj), None, None) => {
		  Ok(
	            Constructor::ForEach(
		      parse(s.clone()).expect("failed to compile select attribute"),
		      n.children()
		        .map(|d| to_constructor(d).expect("failed to compile for-each content"))
		    	.collect(),
		      Some(Grouping::Adjacent(parse(adj.clone()).expect("failed to compile group-adjacent attribute"))),
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
	  Ok(Constructor::LiteralElement(a.to_string(), "".to_string(), "".to_string(),
	    n.children()
	      .map(|d| to_constructor(d).expect("failed to compile sequence constructor"))
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn literal_text() {
    let sc = StaticContext::new_with_xslt_builtins();
    let instxml = roxmltree::Document::parse("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::*'>Found an element</xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml, &sc).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let t = dc.find_match(&i);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    assert_eq!(seq.to_string(), "Found an element")
  }

  #[test]
  fn apply_templates_1() {
    let sc = StaticContext::new_with_xslt_builtins();
    let instxml = roxmltree::Document::parse("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::*'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::text()'>found text</xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml, &sc).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let t = dc.find_match(&i);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    assert_eq!(seq.to_string(), "found textfound text")
  }
  #[test]
  fn apply_templates_2() {
    let sc = StaticContext::new_with_xslt_builtins();
    let instxml = roxmltree::Document::parse("<Test>one<Level1/>two<Level1/>three<Level1/>four<Level1/></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::Test'><xsl:apply-templates select='child::text()'/></xsl:template>
  <xsl:template match='child::Level1'>found Level1 element</xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml, &sc).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let t = dc.find_match(&i);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    assert_eq!(seq.to_string(), "onetwothreefour")
  }

  #[test]
  fn sequence_1() {
    let sc = StaticContext::new_with_xslt_builtins();
    let instxml = roxmltree::Document::parse("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::*'><xsl:sequence select='count(child::*)'/></xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml, &sc).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let t = dc.find_match(&i);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    assert_eq!(seq.to_string(), "2")
  }
  #[test]
  fn sequence_2() {
    let sc = StaticContext::new_with_xslt_builtins();
    let instxml = roxmltree::Document::parse("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::*'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml, &sc).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let t = dc.find_match(&i);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    assert_eq!(seq.to_string(), "onetwo")
  }
  #[test]
  fn sequence_3() {
    let sc = StaticContext::new_with_xslt_builtins();
    let instxml = roxmltree::Document::parse("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::*'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::text()'>X<xsl:sequence select='.'/>Y</xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml, &sc).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let t = dc.find_match(&i);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    assert_eq!(seq.to_string(), "XoneYXtwoY")
  }

  #[test]
  fn literal_result_element_1() {
    let sc = StaticContext::new_with_xslt_builtins();
    let instxml = roxmltree::Document::parse("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::Test'><MyTest><xsl:apply-templates/></MyTest></xsl:template>
  <xsl:template match='child::Level1'><MyLevel1><xsl:apply-templates/></MyLevel1></xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml, &sc).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let t = dc.find_match(&i);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    assert_eq!(seq.to_xml(), "<MyTest><MyLevel1>one</MyLevel1><MyLevel1>two</MyLevel1></MyTest>")
  }

  #[test]
  fn if_1() {
    let sc = StaticContext::new_with_xslt_builtins();
    let instxml = roxmltree::Document::parse("<Test><Level1>one</Level1><Level1>two</Level1><Level1/></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::Test'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Level1'><xsl:if test='child::text()'>has text</xsl:if><xsl:if test='not(child::text())'>no text</xsl:if></xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml, &sc).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let t = dc.find_match(&i);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    assert_eq!(seq.to_xml(), "has texthas textno text")
  }

  #[test]
  fn choose_1() {
    let sc = StaticContext::new_with_xslt_builtins();
    let instxml = roxmltree::Document::parse("<Test><Level1>one</Level1><Level1>two</Level1><Level1/></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::Test'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Level1'><xsl:choose><xsl:when test='child::text()'>has text</xsl:when><xsl:otherwise>no text</xsl:otherwise></xsl:choose></xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml, &sc).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let t = dc.find_match(&i);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    assert_eq!(seq.to_xml(), "has texthas textno text")
  }

  #[test]
  fn foreach_1() {
    let sc = StaticContext::new_with_xslt_builtins();
    let instxml = roxmltree::Document::parse("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::Test'><xsl:for-each select='child::*'><group><xsl:apply-templates/></group></xsl:for-each></xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml, &sc).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let t = dc.find_match(&i);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    assert_eq!(seq.to_xml(), "<group>one</group><group>two</group>")
  }

  #[test]
  fn foreach_2() {
    let sc = StaticContext::new_with_xslt_builtins();
    let instxml = roxmltree::Document::parse("<Test><Level1>one</Level1><Level2>two</Level2><Level3>one</Level3><Level4>two</Level4></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::Test'><xsl:for-each-group select='child::*' group-by='.'><group><xsl:apply-templates/></group></xsl:for-each-group></xsl:template>
  <xsl:template match='child::text()'>a group</xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml, &sc).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let t = dc.find_match(&i);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    assert_eq!(seq.to_xml(), "<group>a group</group><group>a group</group>")
  }

  #[test]
  fn foreach_3() {
    let sc = StaticContext::new_with_xslt_builtins();
    let instxml = roxmltree::Document::parse("<Test><Level1>one</Level1><Level2>two</Level2><Level3>one</Level3><Level4>two</Level4></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::Test'><xsl:for-each-group select='child::*' group-by='.'><group><xsl:sequence select='current-grouping-key()'/></group></xsl:for-each-group></xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml, &sc).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let t = dc.find_match(&i);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    // NB. the order that the groups appear in is undefined
    assert!(
      seq.to_xml() == "<group>one</group><group>two</group>" ||
      seq.to_xml() == "<group>two</group><group>one</group>"
    )
  }

  #[test]
  fn foreach_4() {
    let sc = StaticContext::new_with_xslt_builtins();
    let instxml = roxmltree::Document::parse("<Test><Level1>one</Level1><Level2>two</Level2><Level3>one</Level3><Level4>two</Level4><Level5>one</Level5></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::Test'><xsl:for-each-group select='child::*' group-by='.'><group><key><xsl:sequence select='current-grouping-key()'/></key><members><xsl:sequence select='count(current-group())'/></members></group></xsl:for-each-group></xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml, &sc).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let t = dc.find_match(&i);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    // NB. the order that the groups appear in is undefined
    assert!(
      seq.to_xml() == "<group><key>one</key><members>3</members></group><group><key>two</key><members>2</members></group>" ||
      seq.to_xml() == "<group><key>two</key><members>2</members></group><group><key>one</key><members>3</members></group>"
    )
  }

  #[test]
  fn foreach_adj() {
    let sc = StaticContext::new_with_xslt_builtins();
    let instxml = roxmltree::Document::parse("<Test><Level1>one</Level1><Level2>one</Level2><Level3>two</Level3><Level4>two</Level4><Level5>one</Level5></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::Test'><xsl:for-each-group select='child::*' group-adjacent='.'><group><key><xsl:sequence select='current-grouping-key()'/></key><members><xsl:sequence select='count(current-group())'/></members></group></xsl:for-each-group></xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml, &sc).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let t = dc.find_match(&i);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    // NB. the order that the groups appear in is undefined
    assert!(
      seq.to_xml() == "<group><key>one</key><members>2</members></group><group><key>two</key><members>2</members></group><group><key>one</key><members>1</members></group>" ||
      seq.to_xml() == "<group><key>two</key><members>2</members></group><group><key>one</key><members>3</members></group>"
    )
  }
}
