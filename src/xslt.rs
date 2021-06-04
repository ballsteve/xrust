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
let mut sc = StaticContext::new_with_builtins();

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
let dc = from_xnode(&style).expect("failed to compile stylesheet");

// The source document root node is the initial context.
// Find the template that matches it,
// and use that to start the transformation
let item = Rc::new(Item::XNode(source.root().first_child().unwrap()));
let mut template = dc.find_match(&item);
static_analysis(&mut template, &mut sc);

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
  fn apply_templates_1() {
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
  #[test]
  fn apply_templates_2() {
    let instxml = roxmltree::Document::parse("<Test>one<Level1/>two<Level1/>three<Level1/>four<Level1/></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::Test'><xsl:apply-templates select='child::text()'/></xsl:template>
  <xsl:template match='child::Level1'>found Level1 element</xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let t = dc.find_match(&i);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    assert_eq!(seq.to_string(), "onetwothreefour")
  }

  #[test]
  fn sequence_1() {
    let mut sc = StaticContext::new_with_builtins();
    let instxml = roxmltree::Document::parse("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::*'><xsl:sequence select='count(child::*)'/></xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let mut t = dc.find_match(&i);
    static_analysis(&mut t, &mut sc);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    assert_eq!(seq.to_string(), "2")
  }
  #[test]
  fn sequence_2() {
    let mut sc = StaticContext::new_with_builtins();
    let instxml = roxmltree::Document::parse("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::*'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let mut t = dc.find_match(&i);
    static_analysis(&mut t, &mut sc);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    assert_eq!(seq.to_string(), "onetwo")
  }
  #[test]
  fn sequence_3() {
    let mut sc = StaticContext::new_with_builtins();
    let instxml = roxmltree::Document::parse("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::*'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::text()'>X<xsl:sequence select='.'/>Y</xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let mut t = dc.find_match(&i);
    static_analysis(&mut t, &mut sc);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    assert_eq!(seq.to_string(), "XoneYXtwoY")
  }

  #[test]
  fn literal_result_element_1() {
    let mut sc = StaticContext::new_with_builtins();
    let instxml = roxmltree::Document::parse("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("failed to parse instance XML document");
    let stylexml = roxmltree::Document::parse("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::Test'><MyTest><xsl:apply-templates/></MyTest></xsl:template>
  <xsl:template match='child::Level1'><MyLevel1><xsl:apply-templates/></MyLevel1></xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>").expect("failed to parse XSL stylesheet");
    let dc = from_xnode(&stylexml).expect("failed to compile stylesheet");

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let i = Rc::new(Item::XNode(instxml.root().first_child().unwrap()));
    let mut t = dc.find_match(&i);
    static_analysis(&mut t, &mut sc);
    let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

    assert_eq!(seq.to_xml(), "<MyTest><MyLevel1>one</MyLevel1><MyLevel1>two</MyLevel1></MyTest>")
  }
}
