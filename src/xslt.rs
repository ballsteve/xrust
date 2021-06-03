//! # An XSLT compiler
//!
//! Compile an XSLT stylesheet into a sequence constructor.

use std::rc::Rc;
use crate::xdmerror::*;
use crate::item::*;
use roxmltree::{Document, Node};
use crate::evaluate::*;
use crate::xpath::*;

const XSLTNS: &str = "http://www.w3.org/1999/XSL/Transform";

/// Compiles an XML document into a Sequence Constructor.
pub fn from_xnode<'a>(d: &'a Document<'a>) -> Result<DynamicContext<'a>, Error> {
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
	    let pat = to_pattern(a).expect("failed to compile match pattern");
	    let body = t.children()
	      .map(|d| to_constructor(d).expect("failed to compile sequence constructor"))
	      .collect();
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
fn to_constructor<'a>(n: Node) -> Result<Constructor<'a>, Error> {
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
	  // TODO: select attribute
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
	_ => {
	  Ok(Constructor::NotImplemented("literal element"))
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
    let instxml = roxmltree::Document::parse("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::*'>Found an element</xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let t = dc.find_match(&i);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    assert_eq!(seq.to_string(), "Found an element")
  }

  #[test]
  fn apply_templates() {
    let instxml = roxmltree::Document::parse("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::*'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::text()'>found text</xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let t = dc.find_match(&i);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    assert_eq!(seq.to_string(), "found textfound text")
  }
}
