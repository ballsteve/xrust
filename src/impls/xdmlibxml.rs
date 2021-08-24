/// Driver for libxml

use std::rc::Rc;
use std::any::Any;
use crate::item::{Item, Sequence, SequenceTrait, Document, Node, NodeType, Value, QualifiedName};
use crate::xdmerror::*;
use crate::evaluate::*;
use crate::xpath::parse;
use crate::xslt::*;
use libxml::tree::{NodeType as libxmlNodeType, Document as libxmlDocument, Node as libxmlNode, set_node_rc_guard};
use libxml::parser::Parser;

// In order to get a mutable reference to a node to create new content,
// the add_child, etc, methods clone the reference they are passed in order to then
// make it mutable. However, this increments the strong count.
fn init() {
  set_node_rc_guard(4);
}

impl Document for libxml::tree::Document {
  fn to_string(&self) -> String {
    match self.get_root_element() {
      Some(n) => n.to_string(),
      None => "".to_string(),
    }
  }
  fn to_xml(&self) -> String {
    match self.get_root_element() {
      Some(n) => n.to_xml(),
      None => "".to_string(),
    }
  }
  fn to_json(&self) -> String {
    match self.get_root_element() {
      Some(n) => n.to_json(),
      None => "".to_string(),
    }
  }
  fn to_int(&self) -> Result<i64, Error> {
    match self.get_root_element() {
      Some(e) => e.to_int(),
      None => Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "no root element".to_string()}),
    }
  }
  fn to_double(&self) -> f64 {
    f64::NAN
  }
  fn children(&self) -> Vec<Rc<dyn Node>> {
    // libxml does not currently expose the prologue and epilogue
    match self.get_root_element() {
      Some(e) => vec![Rc::new(e)],
      None => vec![],
    }
  }
  fn get_root_element(&self) -> Option<Rc<dyn Node>> {
    println!("libxml::Document get_root_element");
    match self.get_root_element() {
      Some(e) => Some(Rc::new(e)),
      None => None,
    }
  }

  fn new_element(&self, name: &str, _ns: Option<&str>) -> Result<Rc<dyn Node>, Error> {
    // TODO: namespace
    Ok(Rc::new(libxmlNode::new(name, None, self).expect("unable to create libxml node")))
  }
  // The parameter must be a libxmlNode
  fn set_root_element(&mut self, r: &dyn Any) -> Result<(), Error> {
    let n: &libxmlNode = match r.downcast_ref::<libxmlNode>() {
      Some(m) => m,
      None => return Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "root element must be a libxml Node".to_string()}),
    };
    libxmlDocument::set_root_element(self, n);
    Ok(())
  }
}

impl Node for libxml::tree::Node {
  fn as_any(&self) -> &dyn Any {
    self
  }
  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
  fn to_name(&self) -> QualifiedName {
    match self.get_namespace() {
      Some(ns) => {
        QualifiedName::new(Some(ns.get_href()), Some(ns.get_prefix()), self.get_name())
      }
      None => QualifiedName::new(None, None, self.get_name())
    }
  }

  fn to_string(&self) -> String {
    match self.get_type() {
      Some(libxmlNodeType::ElementNode) => {
        self
	  .get_child_nodes()
	  .iter()
	  .fold(String::new(), |s,c| s + c.to_string().as_str())
        }
      Some(libxmlNodeType::TextNode) |
      Some(libxmlNodeType::CommentNode) |
      Some(libxmlNodeType::PiNode) => {
        self.get_content()
      }
      _ => "".to_string(),
    }
  }

  fn to_xml(&self) -> String {
    match self.get_type() {
      Some(libxmlNodeType::ElementNode) => {
	// TODO: attributes
	format!("<{}>{}</{}>",
	  self.get_name(),
	  self.get_child_nodes()
	    .iter()
	    .fold(String::new(), |s,c| s + c.to_xml().as_str()),
	  self.get_name()
	)
      }
      Some(libxmlNodeType::TextNode) => {
        //println!("to_xml(): text \"{}\"", self.get_content());
	self.get_content()
      }
      Some(libxmlNodeType::CommentNode) => {
	let mut r = String::new();
        r.push_str("<!--");
        r.push_str(self.get_content().as_str());
        r.push_str("-->");
	r
      }
      Some(libxmlNodeType::PiNode) => {
        let mut r = String::new();
        r.push_str("<?");
        r.push_str(self.get_name().as_str());
        r.push_str(" ");
        r.push_str(self.get_content().as_str());
        r.push_str("?>");
	r
      }
      _ => "".to_string(),
    }
  }

  fn to_json(&self) -> String {
    match self.get_type() {
      Some(libxmlNodeType::ElementNode) => {
	// TODO: attributes
	format!("\"{}\": {}",
	  self.get_name(),
	  self.get_child_nodes()
	    .iter()
	    .fold(String::new(), |s,c| s + c.to_json().as_str())
	)
      }
      Some(libxmlNodeType::TextNode) => {
        format!("\"{}\"", self.get_content())
      }
      _ => {
        "".to_string()
      }
    }
  }
  /// Find the xmlDocPtr from the xmlNodePtr. Probably unsafe.
  fn doc(&self) -> Option<Rc<dyn Document>> {
    None
    // TODO Some(Rc::new(self.get_docref()))
  }

  fn parent(&self) -> Option<Rc<dyn Node>> {
    match self.get_parent() {
      Some(p) => {
	match p.as_any().downcast_ref::<libxmlNode>().unwrap().get_type() {
	  Some(libxml::tree::nodetype::NodeType::DocumentNode) => None,
	  Some(_) => Some(Rc::new(p)),
	  None => panic!("unable to determine type of libxml node"),
	}
      }
      None => None,
    }
  }
  fn ancestors(&self) -> Vec<Rc<dyn Node>> {
    find_ancestors(self.clone())
  }

  fn children(&self) -> Vec<Rc<dyn Node>> {
    let mut ret: Vec<Rc<dyn Node>> = Vec::new();
    for c in self.get_child_nodes() {
      ret.push(Rc::new(c));
    }
    ret
  }
  fn descendants(&self) -> Vec<Rc<dyn Node>> {
    let mut ret: Vec<Rc<dyn Node>> = Vec::new();
    for c in self.get_child_nodes() {
      ret.push(Rc::new(c.clone()));
      let mut d = c.descendants();
      ret.append(&mut d);
    }
    ret
  }
  fn get_following_sibling(&self) -> Option<Rc<dyn Node>> {
    match self.get_next_sibling() {
      Some(n) => Some(Rc::new(n)),
      None => None,
    }
  }
  fn following_siblings(&self) -> Vec<Rc<dyn Node>> {
    find_following_siblings(self.clone())
  }
  fn get_preceding_sibling(&self) -> Option<Rc<dyn Node>> {
    match self.get_prev_sibling() {
      Some(n) => Some(Rc::new(n)),
      None => None,
    }
  }
  fn preceding_siblings(&self) -> Vec<Rc<dyn Node>> {
    find_preceding_siblings(self.clone())
  }

  fn to_int(&self) -> Result<i64, Error> {
    match self.to_string().parse::<i64>() {
      Ok(i) => Ok(i),
      Result::Err(e) => Result::Err(Error{kind: ErrorKind::Unknown, message: e.to_string()}),
    }
  }
  fn to_double(&self) -> f64 {
    match self.to_string().parse::<f64>() {
      Ok(f) => f,
      Result::Err(_) => f64::NAN
    }
  }

  fn attribute(&self, name: &str) -> Option<String> {
    self.get_attribute(name)
  }

  fn node_type(&self) -> NodeType {
    match self.get_type() {
      Some(libxmlNodeType::ElementNode) => NodeType::Element,
      Some(libxmlNodeType::AttributeNode) => NodeType::Attribute,
      Some(libxmlNodeType::TextNode) => NodeType::Text,
      Some(libxmlNodeType::CDataSectionNode) => NodeType::Text,
      Some(libxmlNodeType::EntityRefNode) => NodeType::Unknown,
      Some(libxmlNodeType::EntityNode) => NodeType::Unknown,
      Some(libxmlNodeType::PiNode) => NodeType::ProcessingInstruction,
      Some(libxmlNodeType::CommentNode) => NodeType::Comment,
      Some(libxmlNodeType::DocumentNode) => NodeType::Document,
      Some(libxmlNodeType::DocumentTypeNode) => NodeType::Unknown,
      Some(libxmlNodeType::DocumentFragNode) => NodeType::Unknown,
      Some(libxmlNodeType::NotationNode) => NodeType::Unknown,
      Some(libxmlNodeType::HtmlDocumentNode) => NodeType::Unknown,
      Some(libxmlNodeType::DTDNode) => NodeType::Unknown,
      Some(libxmlNodeType::ElementDecl) => NodeType::Unknown,
      Some(libxmlNodeType::AttributeDecl) => NodeType::Unknown,
      Some(libxmlNodeType::EntityDecl) => NodeType::Unknown,
      Some(libxmlNodeType::NamespaceDecl) => NodeType::Unknown,
      Some(libxmlNodeType::XIncludeStart) => NodeType::Unknown,
      Some(libxmlNodeType::XIncludeEnd) => NodeType::Unknown,
      Some(libxmlNodeType::DOCBDocumentNode) => NodeType::Unknown,
      None => NodeType::Unknown,
    }
  }
  fn is_element(&self) -> bool {
    self.is_element_node()
  }

  //fn add_child(&mut self, c: &mut dyn Any) -> Result<(), Error>{
  fn add_child(&self, c: &dyn Any) -> Result<(), Error>{
    let mut o = self.clone();
    let e = match c.downcast_ref::<libxmlNode>() {
      Some(d) => d,
      None => return Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "child node must be a libxml Node".to_string()}),
    };
    let mut f = e.clone();
    match libxmlNode::add_child(&mut o, &mut f) {
      Ok(_) => Ok(()),
      Result::Err(g) => {
        println!("libxml add_child failed");
	Result::Err(Error{kind: ErrorKind::Unknown, message: g.to_string()})
      }
    }
  }
  fn add_text_child(&self, t: String) -> Result<(), Error> {
    let o = self.clone();
    let doc = libxmlDocument::new().expect("unable to create libxml document");
    let mut n = libxmlNode::new_text(t.as_str(), &doc).expect("unable to create text node");

    match o.add_child(&mut n) {
      Ok(_) => Ok(()),
      Result::Err(e) => Result::Err(Error{kind: ErrorKind::Unknown, message: e.to_string()}),
    }
  }
}

fn find_ancestors(n: libxmlNode) -> Vec<Rc<dyn Node>> {
  match n.get_parent() {
    Some(p) => {
      let mut anc = find_ancestors(p.clone());
      anc.insert(0, Rc::new(p.clone()));
      anc
    }
    None => vec![]
  }
}
fn find_following_siblings(n: libxmlNode) -> Vec<Rc<dyn Node>> {
  match n.get_next_sibling() {
    Some(p) => {
      let mut anc = find_following_siblings(p.clone());
      anc.insert(0, Rc::new(p.clone()));
      anc
    }
    None => vec![]
  }
}
fn find_preceding_siblings(n: libxmlNode) -> Vec<Rc<dyn Node>> {
  match n.get_prev_sibling() {
    Some(p) => {
      let mut anc = find_preceding_siblings(p.clone());
      anc.insert(0, Rc::new(p.clone()));
      anc
    }
    None => vec![]
  }
}

fn nodetype_to_string(nt: Option<libxml::tree::NodeType>) -> &'static str {
  match nt {
    Some(libxml::tree::nodetype::NodeType::ElementNode) => "ElementNode",
    Some(libxml::tree::nodetype::NodeType::AttributeNode) => "AttributeNode",
    Some(libxml::tree::nodetype::NodeType::TextNode) => "TextNode",
    Some(libxml::tree::nodetype::NodeType::CDataSectionNode) => "CDataSectionNode",
    Some(libxml::tree::nodetype::NodeType::EntityRefNode) => "EntityRefNode",
    Some(libxml::tree::nodetype::NodeType::EntityNode) => "EntityNode",
    Some(libxml::tree::nodetype::NodeType::PiNode) => "PiNode",
    Some(libxml::tree::nodetype::NodeType::CommentNode) => "CommentNode",
    Some(libxml::tree::nodetype::NodeType::DocumentNode) => "DocumentNode",
    Some(libxml::tree::nodetype::NodeType::DocumentTypeNode) => "DocumentTypeNode",
    Some(libxml::tree::nodetype::NodeType::DocumentFragNode) => "DocumentFragNode",
    Some(libxml::tree::nodetype::NodeType::NotationNode) => "NotationNode",
    Some(libxml::tree::nodetype::NodeType::HtmlDocumentNode) => "HtmlDocumentNode",
    Some(libxml::tree::nodetype::NodeType::DTDNode) => "DTDNode",
    Some(libxml::tree::nodetype::NodeType::ElementDecl) => "ElementDecl",
    Some(libxml::tree::nodetype::NodeType::AttributeDecl) => "AttributeDecl",
    Some(libxml::tree::nodetype::NodeType::EntityDecl) => "EntityDecl",
    Some(libxml::tree::nodetype::NodeType::NamespaceDecl) => "NamespaceDecl",
    Some(libxml::tree::nodetype::NodeType::XIncludeStart) => "XIncludeStart",
    Some(libxml::tree::nodetype::NodeType::XIncludeEnd) => "XIncludeEnd",
    Some(libxml::tree::nodetype::NodeType::DOCBDocumentNode) => "DOCBDocumentNode",
    None => "--None--",
    //_ => "unknown",
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nt_to_str() {
      assert_eq!(nodetype_to_string(None), "--None--")
    }

    #[test]
    fn ldoc() {
      let p = Parser::default();
      let doc = p.parse_string("<Test>a test document</Test>").expect("failed to parse XML");
      let i = Item::Document(Rc::new(doc));
      assert_eq!(i.to_string(), "a test document")
    }
    // The code must not let the libxml::Document be dropped.
    // Any libxml::Node references will become invalid,
    // but this is not caught by the Rust borrow checker(!)
    #[test]
    fn lnode() {
      let ps = Parser::default();
      let doc = ps.parse_string("<Test>a test document</Test>")
          .expect("failed to parse XML");
      let r = match doc.get_root_element() {
	  Some(n) => {
	    n
	  }
	  None => panic!("unable to find root element"),
      };

      let i = Item::Node(Rc::new(r));
      assert_eq!(i.to_name().get_localname(), "Test");
      assert_eq!(i.to_string(), "a test document")
    }

    #[test]
    fn attr_pos() {
      let ps = Parser::default();
      let doc = ps.parse_string("<Test status='test'>a test document</Test>")
          .expect("failed to parse XML");
      let r = match doc.get_root_element() {
	  Some(n) => {
	    n
	  }
	  None => panic!("unable to find root element"),
      };

      assert_eq!(r.attribute("status"), Some("test".to_string()));
    }
    #[test]
    fn attr_neg() {
      let ps = Parser::default();
      let doc = ps.parse_string("<Test status='test'>a test document</Test>")
          .expect("failed to parse XML");
      let r = match doc.get_root_element() {
	  Some(n) => {
	    n
	  }
	  None => panic!("unable to find root element"),
      };

      assert_eq!(r.attribute("role"), None);
    }

    #[test]
    fn node_create() {
      init();
      let mut doc = libxmlDocument::new().expect("unable to create libxml document");
      let r = doc.new_element("Test", None).expect("unable to create libxml element");
      Document::set_root_element(&mut doc, r.as_any()).expect("unable to set root element");
      let n = doc.new_element("Data", None).expect("unable to create libxml element");
      r.add_child(n.as_any()).expect("unable to add child element");
      let di = Item::Document(Rc::new(doc));

      assert_eq!(di.to_xml(), "<Test><Data></Data></Test>")
    }

    #[test]
    fn add_text() {
      init();
      let mut doc = libxmlDocument::new().expect("unable to create libxml document");
      let r = doc.new_element("Test", None).expect("unable to create libxml element");
      Document::set_root_element(&mut doc, r.as_any()).expect("unable to set root element");
      r.add_text_child("this is a test".to_string()).expect("unable to add child element");
      let di = Item::Document(Rc::new(doc));

      assert_eq!(di.to_xml(), "<Test>this is a test</Test>")
    }

    #[test]
    fn descend() {
      let ps = Parser::default();
      let doc = ps.parse_string("<Test><a><c><e/></c></a><b><d><f/></d></b></Test>")
          .expect("failed to parse XML");
      let r = match doc.get_root_element() {
	  Some(n) => {
	    n
	  }
	  None => panic!("unable to find root element"),
      };

      assert_eq!(r.descendants().len(), 6);
    }

//    #[test]
//    fn ascend() {
//      let ps = Parser::default();
//      let doc = ps.parse_string("<Test><a><c><e/></c></a><b><d><f/></d></b></Test>")
//          .expect("failed to parse XML");
//      let r = doc.get_root_element().unwrap()
//	  .get_first_element_child().unwrap()
//	  .get_first_element_child().unwrap()
//	  .get_first_element_child().unwrap();
//      let mut iter = r.ancestor_iter();
//      assert_eq!(iter.next().unwrap().to_name().get_localname(), "c");
//      assert_eq!(iter.next().unwrap().to_name().get_localname(), "a");
//      assert_eq!(iter.next().unwrap().to_name().get_localname(), "Test");
//      match iter.next() {
//        None => assert!(true),
//	Some(_) => assert!(false),
//      }
//    }

//    #[test]
//    fn following_sib_iter() {
//      let ps = Parser::default();
//      let doc = ps.parse_string("<Test><a/><b/><c/><d/><e/></Test>")
//          .expect("failed to parse XML");
//      let r = doc.get_root_element().unwrap()
//	  .get_first_element_child().unwrap();
//      let mut iter = r.following_sibling_iter();
//
//      assert_eq!(iter.next().unwrap().to_name().get_localname(), "b");
//      assert_eq!(iter.next().unwrap().to_name().get_localname(), "c");
//      assert_eq!(iter.next().unwrap().to_name().get_localname(), "d");
//      assert_eq!(iter.next().unwrap().to_name().get_localname(), "e");
//      match iter.next() {
//        None => assert!(true),
//	Some(_) => assert!(false),
//      }
//    }
//    #[test]
//    fn preceding_sib_iter() {
//      let ps = Parser::default();
//      let doc = ps.parse_string("<Test><a/><b/><c/><d/><e/></Test>")
//          .expect("failed to parse XML");
//      let r = doc.get_root_element().unwrap()
//	  .get_last_element_child().unwrap();
//      let mut iter = r.preceding_sibling_iter();
//
//      assert_eq!(iter.next().unwrap().to_name().get_localname(), "d");
//      assert_eq!(iter.next().unwrap().to_name().get_localname(), "c");
//      assert_eq!(iter.next().unwrap().to_name().get_localname(), "b");
//      assert_eq!(iter.next().unwrap().to_name().get_localname(), "a");
//      match iter.next() {
//        None => assert!(true),
//	Some(_) => assert!(false),
//      }
//    }

    // Evaluation tests

    #[test]
    fn node_root() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test>a test document</Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Item::Node(Rc::new(n.get_first_child().unwrap()));

      // XPath == /
      let cons = vec![Constructor::Root];

      let e = evaluate(&dc, Some(vec![Rc::new(i)]), Some(0), &cons).expect("evaluation failed");
      if e.len() == 1 {
        assert_eq!(e[0].to_xml(), "<Test>a test document</Test>")
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn node_child_all() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test>foo<a>a</a>some text<b>b</b>bar</Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let idoc = Item::Document(rgdoc);
      let i = Rc::new(Item::Node(idoc.get_root_element().unwrap()));

      // XPath == child::*
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Child,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];

      let e = evaluate(&dc, Some(vec![i]), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() == 2 {
        assert_eq!(e[0].to_name().get_localname(), "a");
        assert_eq!(e[1].to_name().get_localname(), "b");
      } else {
        panic!("sequence does not have 2 items: \"{}\"", e.len())
      }
    }

    #[test]
    fn node_self_pos() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test>foo<a>a</a>some text<b>b</b>bar</Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let idoc = Item::Document(rgdoc);
      let i = Rc::new(Item::Node(idoc.get_root_element().unwrap()));

      // XPath == self::*
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Selfaxis,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];
      let e = evaluate(&dc, Some(vec![i]), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() == 1 {
        assert_eq!(e[0].to_name().get_localname(), "Test")
      } else {
        panic!("sequence is not a singleton")
      }
    }
    #[test]
    fn node_self_neg() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test>foo<a>a</a>some text<b>b</b>bar</Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Item::Node(Rc::new(n.get_first_child().unwrap()));

      // XPath == self::*
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Selfaxis,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];
      // The first child of the root element is a text node, "foo", so this should evaluate to an empty sequence
      let e = evaluate(&dc, Some(vec![Rc::new(i)]), Some(0), &cons)
        .expect("evaluation failed");
      assert_eq!(e.len(), 0)
    }

    #[test]
    fn node_parent_any() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test>foo<a>a</a>some text<b>b</b>bar</Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Item::Node(Rc::new(n.get_first_child().unwrap()));

      // XPath == parent::*
      let cons = vec![Constructor::Step(
	  NodeMatch{
	    axis: Axis::Parent,
	    nodetest: NodeTest::Name(NameTest{
	      ns: None,
	      prefix: None,
	      name: Some(WildcardOrName::Wildcard)
	    })
	  },
	  vec![]
	)];

      let e = evaluate(&dc, Some(vec![Rc::new(i)]), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() == 1 {
        assert_eq!(e[0].to_name().get_localname(), "Test")
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn node_descendant_1() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Item::Node(Rc::new(n.get_first_child().unwrap()));

      // XPath == descendant::*
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Descendant,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];
      let e = evaluate(&dc, Some(vec![Rc::new(i)]), Some(0), &cons)
        .expect("evaluation failed");
      assert_eq!(e.len(), 6);
      assert_eq!(e[1].to_xml(), "<level3>1 1 1</level3>")
    }
    #[test]
    fn node_descendantorself_1() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Item::Node(Rc::new(n.get_first_child().unwrap()));

      // XPath == descendant-or-self::*
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::DescendantOrSelf,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];
      let e = evaluate(&dc, Some(vec![Rc::new(i)]), Some(0), &cons)
        .expect("evaluation failed");
      assert_eq!(e.len(), 7);
      assert_eq!(e[2].to_xml(), "<level3>1 1 1</level3>")
    }

    #[test]
    fn node_ancestor_1() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Item::Node(Rc::new(n.get_first_child().unwrap().get_first_child().unwrap().get_first_child().unwrap()));

      // XPath == ancestor::*
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Ancestor,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];
      let e = evaluate(&dc, Some(vec![Rc::new(i)]), Some(0), &cons)
        .expect("evaluation failed");
      assert_eq!(e.len(), 3);
    }
    #[test]
    fn node_ancestororself_1() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Item::Node(Rc::new(n.get_first_child().unwrap().get_first_child().unwrap().get_first_child().unwrap()));

      // XPath == ancestor::*
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::AncestorOrSelf,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];
      let e = evaluate(&dc, Some(vec![Rc::new(i)]), Some(0), &cons)
        .expect("evaluation failed");
      assert_eq!(e.len(), 4);
    }

    #[test]
    fn node_followingsibling_1() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Item::Node(Rc::new(n.get_first_child().unwrap().get_first_child().unwrap().get_first_child().unwrap()));

      // XPath == following-sibling::*
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::FollowingSibling,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];
      let e = evaluate(&dc, Some(vec![Rc::new(i)]), Some(0), &cons)
        .expect("evaluation failed");
      assert_eq!(e.len(), 1);
      assert_eq!(e[0].to_xml(), "<level3>1 1 2</level3>");
    }

    #[test]
    fn node_precedingsibling_1() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Item::Node(Rc::new(n.get_first_child().unwrap().get_first_child().unwrap().get_last_child().unwrap()));
      println!("i=\"{}\"", i.to_xml());

      // XPath == preceding-sibling::*
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::PrecedingSibling,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];
      let e = evaluate(&dc, Some(vec![Rc::new(i)]), Some(0), &cons)
        .expect("evaluation failed");
      println!("e=\"{}\" len {}", e.to_xml(), e.len());
      assert_eq!(e.len(), 1);
      assert_eq!(e[0].to_xml(), "<level3>1 1 1</level3>");
    }

    #[test]
    fn node_following_1() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Item::Node(Rc::new(n.get_first_child().unwrap().get_first_child().unwrap().get_last_child().unwrap()));
      println!("i=\"{}\"", i.to_xml());

      // XPath == follow::*
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Following,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];
      let e = evaluate(&dc, Some(vec![Rc::new(i)]), Some(0), &cons)
        .expect("evaluation failed");
      println!("e=\"{}\"", e.to_xml());
      assert_eq!(e.len(), 4);
      assert_eq!(e[0].to_xml(), "<level2><level3>1 2 1</level3><level3>1 2 2</level3></level2>");
      assert_eq!(e[1].to_xml(), "<level3>1 2 1</level3>");
      assert_eq!(e[2].to_xml(), "<level3>1 2 2</level3>");
      assert_eq!(e[3].to_xml(), "<level1>not me</level1>");
    }

    #[test]
    fn node_preceding_1() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Item::Node(Rc::new(n.get_last_child().unwrap()));
      println!("i=\"{}\"", i.to_xml());

      // XPath == preceding::*
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Preceding,
	      nodetest: NodeTest::Name(NameTest{
	        ns: None,
		prefix: None,
		name: Some(WildcardOrName::Wildcard)
	      })
	    },
	    vec![]
	  )
	];
      let e = evaluate(&dc, Some(vec![Rc::new(i)]), Some(0), &cons)
        .expect("evaluation failed");
      assert_eq!(e.len(), 7);
      assert_eq!(e[0].to_name().get_localname(), "level1");
      assert_eq!(e[1].to_name().get_localname(), "level2");
      assert_eq!(e[2].to_name().get_localname(), "level3");
      assert_eq!(e[2].to_xml(), "<level3>1 1 1</level3>");
    }

    #[test]
    fn node_path() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Level1><Level2>one</Level2><Level2>two</Level2><Level2>three</Level2></Level1>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let s = vec![Rc::new(Item::Document(Rc::clone(&rgdoc)))];

      // XPath == /child::*/child::*
      let cons = vec![
	  Constructor::Path(
	    vec![
	      vec![Constructor::Root],
              vec![Constructor::Step(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})}, vec![])],
              vec![Constructor::Step(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})}, vec![])],
            ]
	  )
	];
      let e = evaluate(&dc, Some(s), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() == 3 {
        assert_eq!(e[0].to_xml(), "<Level2>one</Level2>");
        assert_eq!(e[1].to_xml(), "<Level2>two</Level2>");
        assert_eq!(e[2].to_xml(), "<Level2>three</Level2>");
      } else {
        panic!("sequence does not have 3 items: \"{}\"", e.len())
      }
    }

    #[test]
    fn node_nametest_pos() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test/>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let s = vec![Rc::new(Item::Document(Rc::clone(&rgdoc)))];

      // XPath == /child::Test
      let cons = vec![
	  Constructor::Path(
	    vec![
	      vec![Constructor::Root],
              vec![Constructor::Step(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Test".to_string()))})}, vec![])],
            ]
	  )
	];
      let e = evaluate(&dc, Some(s), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() == 1 {
        assert!(
	  e[0].to_xml() == "<Test/>" ||
	  e[0].to_xml() == "<Test></Test>"
	);
      } else {
        panic!("sequence does not have 1 item: \"{}\"", e.len())
      }
    }
    #[test]
    fn node_nametest_neg() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test/>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let s = vec![Rc::new(Item::Document(Rc::clone(&rgdoc)))];

      // XPath == /child::Foo
      let cons = vec![
	  Constructor::Path(
	    vec![
	      vec![Constructor::Root],
              vec![Constructor::Step(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Foo".to_string()))})}, vec![])],
            ]
	  )
	];
      let e = evaluate(&dc, Some(s), Some(0), &cons)
        .expect("evaluation failed");
      println!("e=\"{}\" len {}", e.to_xml(), e.len());
      assert_eq!(e.len(), 0);
    }

    // Kind Tests
    #[test]
    fn kind_element_1() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1>1<level2/>2<level2/>3<level2/>4<level2/>5<level2/>6<level2/>7</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let s = vec![Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap())))];

      // XPath == /child::element()
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Child,
	      nodetest: NodeTest::Kind(KindTest::ElementTest)
	    },
	    vec![]
	  )
	];
      let e = evaluate(&dc, Some(s), Some(0), &cons)
        .expect("evaluation failed");
      assert_eq!(e.len(), 6);
      assert_eq!(e[0].to_name().get_localname(), "level2");
      assert_eq!(e[1].to_name().get_localname(), "level2");
      assert_eq!(e[2].to_name().get_localname(), "level2");
      assert_eq!(e[3].to_name().get_localname(), "level2");
      assert_eq!(e[4].to_name().get_localname(), "level2");
      assert_eq!(e[5].to_name().get_localname(), "level2");
    }

    #[test]
    fn kind_text_1() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1>1<level2/>2<level2/>3<level2/>4<level2/>5<level2/>6<level2/>7</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let s = vec![Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap())))];

      // XPath == /child::text()
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Child,
	      nodetest: NodeTest::Kind(KindTest::TextTest)
	    },
	    vec![]
	  )
	];
      let e = evaluate(&dc, Some(s), Some(0), &cons)
        .expect("evaluation failed");
      assert_eq!(e.len(), 7);
      assert_eq!(e[0].to_string(), "1");
      assert_eq!(e[1].to_string(), "2");
      assert_eq!(e[2].to_string(), "3");
      assert_eq!(e[3].to_string(), "4");
      assert_eq!(e[4].to_string(), "5");
      assert_eq!(e[5].to_string(), "6");
      assert_eq!(e[6].to_string(), "7");
    }

    #[test]
    fn kind_any_1() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1>1<level2/>2<level2/>3<level2/>4<level2/>5<level2/>6<level2/>7</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let s = vec![Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap())))];

      // XPath == /child::node()
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Child,
	      nodetest: NodeTest::Kind(KindTest::AnyKindTest)
	    },
	    vec![]
	  )
	];
      let e = evaluate(&dc, Some(s), Some(0), &cons)
        .expect("evaluation failed");
      assert_eq!(e.len(), 13);
      assert_eq!(e[0].to_string(), "1");
      assert_eq!(e[1].to_name().get_localname(), "level2");
      assert_eq!(e[2].to_string(), "2");
      assert_eq!(e[3].to_name().get_localname(), "level2");
      assert_eq!(e[4].to_string(), "3");
      assert_eq!(e[5].to_name().get_localname(), "level2");
      assert_eq!(e[6].to_string(), "4");
      assert_eq!(e[7].to_name().get_localname(), "level2");
      assert_eq!(e[8].to_string(), "5");
      assert_eq!(e[9].to_name().get_localname(), "level2");
      assert_eq!(e[10].to_string(), "6");
      assert_eq!(e[11].to_name().get_localname(), "level2");
      assert_eq!(e[12].to_string(), "7");
    }

    // Predicates

    #[test]
    fn predicate_pos() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><Level2></Level2></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let s = vec![Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap())))];

      // XPath == /Test[Level2]
      let cons = vec![
	  Constructor::Path(
	    vec![
	      vec![Constructor::Root],
              vec![Constructor::Step(
	        NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Test".to_string()))})},
		vec![vec![Constructor::Step(
	          NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Level2".to_string()))})},
		  vec![]
		)]]
	      )],
            ]
	  )
	];
      let e = evaluate(&dc, Some(s), Some(0), &cons)
        .expect("evaluation failed");
      if e.len() == 1 {
        assert!(
	  e[0].to_xml() == "<Test><Level2/></Test>" ||
	  e[0].to_xml() == "<Test><Level2></Level2></Test>"
	)
      } else {
        panic!("sequence does not have 1 item: \"{}\"", e.len())
      }
    }
    #[test]
    fn predicate_neg() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><Level2></Level2></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let t = vec![Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap())))];

      // XPath == /Test[foo]
      let cons = vec![
	  Constructor::Path(
	    vec![
	      vec![Constructor::Root],
              vec![Constructor::Step(
	        NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Test".to_string()))})},
		vec![vec![Constructor::Step(
	          NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("foo".to_string()))})},
		  vec![]
		)]]
	      )],
            ]
	  )
	];
      let e = evaluate(&dc, Some(t), Some(0), &cons)
        .expect("evaluation failed");
      println!("seq=\"{}\" len={}", e.to_xml(), e.len());
      assert_eq!(e.len(), 0);
    }

    // Node-related Functions

    #[test]
    fn function_call_local_name() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><Level2></Level2></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let idoc = Item::Document(rgdoc);
      let s = vec![Rc::new(Item::Node(idoc.get_root_element().unwrap()))];

      // XPath == local-name()
      let c = Constructor::FunctionCall(
        Function::new("local-name".to_string(), vec![], Some(func_localname)),
	vec![]
      );
      let vc = vec![c];
      let r = evaluate(&dc, Some(s), Some(0), &vc).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      assert_eq!(r[0].to_string(), "Test")
    }
    #[test]
    fn function_call_name() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><Level2></Level2></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let idoc = Item::Document(rgdoc);
      let s = vec![Rc::new(Item::Node(idoc.get_root_element().unwrap()))];

      // XPath == name()
      let c = Constructor::FunctionCall(
        Function::new("name".to_string(), vec![], Some(func_name)),
	vec![]
      );
      let vc = vec![c];
      let r = evaluate(&dc, Some(s), Some(0), &vc).expect("evaluation failed");
      assert_eq!(r.len(), 1);
      assert_eq!(r[0].to_string(), "Test")
    }

    // Patterns

    #[test]
    fn pattern_1_pos() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><Level2></Level2></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap())));

      // XPath == *
      let cons = vec![Constructor::Path(
	    vec![
              vec![Constructor::Step(
	        NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})},
		vec![]
	      )],
            ]
	  )];
      let p = to_pattern(cons).expect("unable to reverse expression");
      let dc = DynamicContext::new();
      assert_eq!(item_matches(&dc, &p, &i), true);
    }
    // TODO: matching a text node should return false
    #[test]
    fn pattern_2_pos() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><Level2></Level2></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let idoc = Item::Document(rgdoc);
      let i = Rc::new(Item::Node(idoc.get_root_element().unwrap()));

      // XPath == child::Test
      let cons = vec![Constructor::Path(
	    vec![
              vec![Constructor::Step(
	        NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Test".to_string()))})},
		vec![]
	      )],
            ]
	  )];
      let p = to_pattern(cons).expect("unable to reverse expression");
      let dc = DynamicContext::new();
      assert_eq!(item_matches(&dc, &p, &i), true);
    }
    #[test]
    fn pattern_2_neg() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><Level2></Level2></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let i = Rc::new(Item::Document(Rc::clone(&rgdoc)));

      // XPath == child::Level2
      let cons = vec![Constructor::Path(
	    vec![
              vec![Constructor::Step(
	        NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Level2".to_string()))})},
		vec![]
	      )],
            ]
	  )];
      let p = to_pattern(cons).expect("unable to reverse expression");
      let dc = DynamicContext::new();
      assert_eq!(item_matches(&dc, &p, &i), false);
    }
    #[test]
    fn pattern_3_pos() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><Level2></Level2></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap())));

      // XPath == child::Test/child::Level2
      let cons = vec![Constructor::Path(
	    vec![
              vec![Constructor::Step(
	        NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Test".to_string()))})},
		vec![]
	      )],
              vec![Constructor::Step(
	        NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Level2".to_string()))})},
		vec![]
	      )],
            ]
	  )];
      let p = to_pattern(cons).expect("unable to reverse expression");
      let dc = DynamicContext::new();
      assert_eq!(item_matches(&dc, &p, &i), true);
    }
    #[test]
    fn pattern_4_pos() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><Level2></Level2></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap())));

      // XPath == /child::Test/child::Level2
      let cons = vec![Constructor::Path(
	    vec![
	      vec![Constructor::Root],
              vec![Constructor::Step(
	        NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Test".to_string()))})},
		vec![]
	      )],
              vec![Constructor::Step(
	        NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Level2".to_string()))})},
		vec![]
	      )],
            ]
	  )];
      let p = to_pattern(cons).expect("unable to reverse expression");
      assert_eq!(item_matches(&dc, &p, &i), true);
    }
    #[test]
    fn pattern_4_neg() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Root><Test><Level2></Level2></Test></Root>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap().get_first_child().unwrap())));

      // XPath == /child::Test/child::Level2
      let cons = vec![Constructor::Path(
	    vec![
	      vec![Constructor::Root],
              vec![Constructor::Step(
	        NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Test".to_string()))})},
		vec![]
	      )],
              vec![Constructor::Step(
	        NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Level2".to_string()))})},
		vec![]
	      )],
            ]
	  )];
      let p = to_pattern(cons).expect("unable to reverse expression");
      assert_eq!(item_matches(&dc, &p, &i), false);
    }
    #[test]
    fn pattern_5_pos() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><Level2></Level2></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let i = Rc::new(Item::Document(rgdoc));

      // XPath == /
      let cons = vec![Constructor::Path(
	    vec![
	      vec![Constructor::Root],
            ]
	  )];
      let p = to_pattern(cons).expect("unable to reverse expression");
      assert_eq!(item_matches(&dc, &p, &i), true);
    }
    #[test]
    fn pattern_5_neg() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Root><Test><Level2></Level2></Test></Root>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap().get_first_child().unwrap())));

      // XPath == /
      let cons = vec![Constructor::Path(
	    vec![
	      vec![Constructor::Root],
            ]
	  )];
      let p = to_pattern(cons).expect("unable to reverse expression");
      assert_eq!(item_matches(&dc, &p, &i), false);
    }

    /// Literal result elements

    #[test]
    fn literal_element_1() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><Level2></Level2></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));

      let cons = vec![
        Constructor::LiteralElement("Test".to_string(), "".to_string(), "".to_string(), vec![]),
      ];
      let seq = evaluate(&dc, None, None, &cons).expect("evaluation failed");
      assert_eq!(seq.len(), 1);
      assert!(
        seq[0].to_xml() == "<Test/>" ||
        seq[0].to_xml() == "<Test></Test>"
      );
    }
    #[test]
    fn literal_element_2() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><Level2></Level2></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));

      let cons = vec![
        Constructor::LiteralElement("Test".to_string(), "".to_string(), "".to_string(),
	  vec![
	    Constructor::LiteralElement("Level1".to_string(), "".to_string(), "".to_string(),
	      vec![
	        Constructor::Literal(Value::String("Test text".to_string())),
	      ]
	    )
	  ]
	),
      ];
      let seq = evaluate(&dc, None, None, &cons).expect("evaluation failed");
      assert_eq!(seq.len(), 1);
      assert_eq!(seq[0].to_xml(), "<Test><Level1>Test text</Level1></Test>")
    }
    #[test]
    fn literal_element_3() {
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><Level2></Level2></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));

      let cons = vec![
        Constructor::LiteralElement("Test".to_string(), "".to_string(), "".to_string(),
	  vec![
	    Constructor::LiteralElement("Level1".to_string(), "".to_string(), "".to_string(),
	      vec![
	        Constructor::Literal(Value::String("one".to_string())),
	      ]
	    ),
	    Constructor::LiteralElement("Level1".to_string(), "".to_string(), "".to_string(),
	      vec![
	        Constructor::Literal(Value::String("two".to_string())),
	      ]
	    ),
	  ]
	),
      ];
      let seq = evaluate(&dc, None, None, &cons).expect("evaluation failed");
      assert_eq!(seq.len(), 1);
      assert_eq!(seq[0].to_xml(), "<Test><Level1>one</Level1><Level1>two</Level1></Test>")
    }

    /// Templates

    #[test]
    fn template_1() {
      let p = Parser::default();
      let doc = p.parse_string("<Test><Level2></Level2></Test>").expect("failed to parse XML");
      let i = Rc::new(Item::Node(Rc::new(doc.get_root_element().unwrap())));

      // This constructor is "child::Test"
      let cons1 = vec![Constructor::Path(
	    vec![
              vec![Constructor::Step(
	        NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Test".to_string()))})},
		vec![]
	      )],
            ]
	  )];
      let p = to_pattern(cons1).expect("unable to convert to pattern");
      let cons2 = vec![
        Constructor::Literal(Value::String("I found a matching template".to_string())),
      ];
      let mut dc = DynamicContext::new();
      dc.add_template(p, cons2);
      let t = dc.find_match(&i);
      assert_eq!(t.len(), 1);
      let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("evaluation failed");
      assert_eq!(seq.len(), 1);
      assert_eq!(seq[0].to_string(), "I found a matching template")
    }
    #[test]
    fn template_2() {
      let p = Parser::default();
      let doc = p.parse_string("<Test><Level2></Level2><Level3></Level3></Test>").expect("failed to parse XML");
      let i = Rc::new(Item::Node(Rc::new(doc.get_root_element().unwrap())));

      let mut dc = DynamicContext::new();

      // This constructor is "child::Test"
      let cons1 = vec![Constructor::Path(
	    vec![
              vec![Constructor::Step(
	        NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Test".to_string()))})},
		vec![]
	      )],
            ]
	  )];
      let pat1 = to_pattern(cons1).expect("unable to convert to pattern");
      // The constructor for the select expression is "child::*"
      let body1 = vec![
        Constructor::ApplyTemplates(
              vec![Constructor::Step(
	        NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})},
		vec![]
	      )],
	),
      ];
      dc.add_template(pat1, body1);

      // This constructor is "child::Level2"
      let cons2 = vec![Constructor::Path(
	    vec![
              vec![Constructor::Step(
	        NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Level2".to_string()))})},
		vec![]
	      )],
            ]
	  )];
      let pat2 = to_pattern(cons2).expect("unable to convert to pattern");
      let body2 = vec![
        Constructor::Literal(Value::String("I found a Level2".to_string())),
      ];
      dc.add_template(pat2, body2);

      // This constructor is "child::Level3"
      let cons3 = vec![Constructor::Path(
	    vec![
              vec![Constructor::Step(
	        NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Level3".to_string()))})},
		vec![]
	      )],
            ]
	  )];
      let pat3 = to_pattern(cons3).expect("unable to convert to pattern");
      let body3 = vec![
        Constructor::Literal(Value::String("I found a Level3".to_string())),
      ];
      dc.add_template(pat3, body3);

      let t = dc.find_match(&i);
      assert_eq!(t.len(), 1);
      let seq: Sequence = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("evaluation failed");
      println!("seq=\"{}\"", seq.to_string());
      //println!("seq=\"{}\"", seq.to_xml());
      assert_eq!(seq.len(), 2);
      assert_eq!(seq[0].to_string(), "I found a Level2");
      assert_eq!(seq[1].to_string(), "I found a Level3");
    }

    // for-each, for-each-group

    #[test]
    fn foreach_1() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><Level2></Level2><Level3></Level3></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let idoc = Item::Document(rgdoc);
      let i = Rc::new(Item::Node(idoc.get_root_element().unwrap()));

      let cons = vec![
        Constructor::ForEach(
	  vec![
	    Constructor::Step(
	      NodeMatch{
	        axis: Axis::Child,
	      	nodetest: NodeTest::Kind(KindTest::AnyKindTest)
	      },
	      vec![]
	    ),
	  ],
	  vec![
	    Constructor::LiteralElement("Group".to_string(), "".to_string(), "".to_string(),
	      vec![
	        Constructor::Literal(Value::String("a group".to_string())),
	      ]
	    ),
	  ],
	  None,
	),
      ];
      let seq = evaluate(&dc, Some(vec![i]), Some(0), &cons).expect("evaluation failed");
      assert_eq!(seq.len(), 2);
      assert_eq!(seq[0].to_xml(), "<Group>a group</Group>");
      assert_eq!(seq[1].to_xml(), "<Group>a group</Group>");
    }

    #[test]
    fn foreach_2() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><Level1>one</Level1><Level2>two</Level2><Level3>one</Level3><Level4>two</Level4></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let idoc = Item::Document(rgdoc);
      let i = Rc::new(Item::Node(idoc.get_root_element().unwrap()));

      let cons = vec![
        Constructor::ForEach(
	  vec![
	    Constructor::Step(
	      NodeMatch{
	        axis: Axis::Child,
	      	nodetest: NodeTest::Kind(KindTest::AnyKindTest)
	      },
	      vec![]
	    ),
	  ],
	  vec![
	    Constructor::LiteralElement("Group".to_string(), "".to_string(), "".to_string(),
	      vec![
	        Constructor::Literal(Value::String("a group".to_string())),
	      ]
	    ),
	  ],
	  Some(Grouping::By(
	    vec![Constructor::ContextItem],
	  )),
	),
      ];
      let seq = evaluate(&dc, Some(vec![i]), Some(0), &cons).expect("evaluation failed");
      assert_eq!(seq.len(), 2);
      assert_eq!(seq[0].to_xml(), "<Group>a group</Group>");
      assert_eq!(seq[1].to_xml(), "<Group>a group</Group>");
    }
    #[test]
    fn foreach_3() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><Level1>one</Level1><Level2>one</Level2><Level3>two</Level3><Level4>three</Level4></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let idoc = Item::Document(rgdoc);
      let i = Rc::new(Item::Node(idoc.get_root_element().unwrap()));

      let cons = vec![
        Constructor::ForEach(
	  vec![
	    Constructor::Step(
	      NodeMatch{
	        axis: Axis::Child,
	      	nodetest: NodeTest::Kind(KindTest::AnyKindTest)
	      },
	      vec![]
	    ),
	  ],
	  vec![
	    Constructor::LiteralElement("Group".to_string(), "".to_string(), "".to_string(),
	      vec![
	        Constructor::FunctionCall(
		  Function::new("current-grouping-key".to_string(), vec![], Some(func_current_grouping_key)),
		  vec![],
		),
	        Constructor::FunctionCall(
		  Function::new("count".to_string(), vec![], Some(func_count)),
		  vec![vec![
		    Constructor::FunctionCall(
		      Function::new("current-group".to_string(), vec![], Some(func_current_group)),
		      vec![],
		    ),
		  ]],
		),
	      ]
	    ),
	  ],
	  Some(Grouping::Adjacent(
	    vec![Constructor::ContextItem],
	  )),
	),
      ];
      let seq = evaluate(&dc, Some(vec![i]), Some(0), &cons).expect("evaluation failed");
      assert_eq!(seq.len(), 3);
      assert_eq!(seq[0].to_xml(), "<Group>one2</Group>");
      assert_eq!(seq[1].to_xml(), "<Group>two1</Group>");
      assert_eq!(seq[2].to_xml(), "<Group>three1</Group>");
    }

    // XPath tests

    #[test]
    fn xpath_root() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Level1><Level2>one</Level2><Level2>two</Level2><Level2>three</Level2></Level1>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let idoc = Item::Document(rgdoc);
      let i = Rc::new(Item::Node(idoc.get_root_element().unwrap()));

      let c = parse("/").expect("unable to parse XPath \"/\"");

      let e = evaluate(&dc, Some(vec![i]), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 1);
      assert_eq!(e[0].to_xml(), "<Level1><Level2>one</Level2><Level2>two</Level2><Level2>three</Level2></Level1>");
    }

    #[test]
    fn xpath_step_1() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Level1><Level2>one</Level2><Level2>two</Level2><Level2>three</Level2></Level1>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let idoc = Item::Document(rgdoc);
      let i = Rc::new(Item::Node(idoc.get_root_element().unwrap()));

      let c = parse("/child::*").expect("failed to parse expression \"/child::*\"");

      let e = evaluate(&dc, Some(vec![i]), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 1);
      assert_eq!(e[0].to_xml(), "<Level1><Level2>one</Level2><Level2>two</Level2><Level2>three</Level2></Level1>");
    }
    #[test]
    fn xpath_step_nodetest_pos() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Level1><Level2>one</Level2><Level2>two</Level2><Level2>three</Level2></Level1>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let idoc = Item::Document(rgdoc);
      let i = Rc::new(Item::Node(idoc.get_root_element().unwrap()));

      let c = parse("/child::Level1").expect("failed to parse expression \"/child::Level1\"");

      let e = evaluate(&dc, Some(vec![i]), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 1);
      assert_eq!(e[0].to_name().get_localname(), "Level1");
    }
    #[test]
    fn xpath_step_nodetest_neg() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Level1><Level2>one</Level2><Level2>two</Level2><Level2>three</Level2></Level1>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let idoc = Item::Document(rgdoc);
      let i = Rc::new(Item::Node(idoc.get_root_element().unwrap()));

      let c = parse("/child::Test").expect("failed to parse expression \"/child::Test\"");

      let e = evaluate(&dc, Some(vec![i]), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 0);
    }
    #[test]
    fn xnode_step_2() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Level1><Level2>one</Level2><Level2>two</Level2><Level2>three</Level2></Level1>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let idoc = Item::Document(rgdoc);
      let i = Rc::new(Item::Node(idoc.get_root_element().unwrap()));

      let c = parse("/child::*/child::*").expect("failed to parse expression \"/child::a/child::b\"");

      let e = evaluate(&dc, Some(vec![i]), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 3);
      assert_eq!(e[0].to_xml(), "<Level2>one</Level2>");
      assert_eq!(e[1].to_xml(), "<Level2>two</Level2>");
      assert_eq!(e[2].to_xml(), "<Level2>three</Level2>");
    }
    #[test]
    fn xpath_descendant_1() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap())));

      let c = parse("descendant::*").expect("failed to parse expression \"descendant::*\"");

      let e = evaluate(&dc, Some(vec![i]), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 6);
      assert_eq!(e[1].to_xml(), "<level3>1 1 1</level3>")
    }
    #[test]
    fn xpath_descendantorself_1() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap())));

      let c = parse("descendant-or-self::*").expect("failed to parse expression \"descendant-or-self::*\"");
      let e = evaluate(&dc, Some(vec![i]), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 7);
      assert_eq!(e[2].to_xml(), "<level3>1 1 1</level3>")
    }
    #[test]
    fn xpath_ancestor_1() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap().get_first_child().unwrap().get_first_child().unwrap().get_first_child().unwrap())));

      let c = parse("ancestor::*").expect("failed to parse expression \"ancestor::*\"");
      let e = evaluate(&dc, Some(vec![i]), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 4);
    }
    #[test]
    fn xpath_ancestororself_1() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap().get_first_child().unwrap().get_first_child().unwrap().get_first_child().unwrap())));

      let c = parse("ancestor-or-self::*").expect("failed to parse expression \"ancestor-or-self::*\"");
      let e = evaluate(&dc, Some(vec![i]), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 4);
    }
    #[test]
    fn xpath_followingsibling_1() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap().get_first_child().unwrap().get_first_child().unwrap())));

      let c = parse("following-sibling::*").expect("failed to parse expression \"following-sibling::*\"");
      let e = evaluate(&dc, Some(vec![i]), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 1);
      assert_eq!(e.to_xml(), "<level3>1 1 2</level3>");
    }
    #[test]
    fn xpath_precedingsibling_1() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap().get_first_child().unwrap().get_last_child().unwrap())));

      let c = parse("preceding-sibling::*").expect("failed to parse expression \"preceding-sibling::*\"");
      let e = evaluate(&dc, Some(vec![i]), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 1);
      assert_eq!(e.to_xml(), "<level3>1 1 1</level3>");
    }
    #[test]
    fn xpath_following_1() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap().get_first_child().unwrap().get_last_child().unwrap())));

      let c = parse("following::*").expect("failed to parse expression \"following::*\"");
      let e = evaluate(&dc, Some(vec![i]), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 4);
      assert_eq!(e.to_xml(), "<level2><level3>1 2 1</level3><level3>1 2 2</level3></level2><level3>1 2 1</level3><level3>1 2 2</level3><level1>not me</level1>");
    }
    #[test]
    fn xpath_preceding_1() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap().get_first_child().unwrap().get_last_child().unwrap())));

      let c = parse("preceding::*").expect("failed to parse expression \"preceding::*\"");
      let e = evaluate(&dc, Some(vec![i]), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 1);
      assert_eq!(e[0].to_xml(), "<level3>1 1 1</level3>");
    }

    #[test]
    fn parse_eval_predicate_pos() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><a><b/></a><a><c/></a></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let i = Rc::new(Item::Document(rgdoc));

      let e = parse("/child::*/child::*[child::b]").expect("failed to parse expression \"//child::*/child::*[child::b]\"");
      let s = evaluate(&dc, Some(vec![i]), Some(0), &e).expect("evaluation failed");
      assert_eq!(s.len(), 1);
      assert!(
        s.to_xml() == "<a><b/></a>" ||
        s.to_xml() == "<a><b></b></a>"
      )
    }
    #[test]
    fn parse_eval_predicate_neg() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><a><b/></a><a><c/></a></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let idoc = Item::Document(rgdoc);
      let i = Rc::new(Item::Node(idoc.get_root_element().unwrap()));

      let e = parse("/child::*[child::b]").expect("failed to parse expression \"/child::*[child::b]\"");
      let s = evaluate(&dc, Some(vec![i]), Some(0), &e).expect("evaluation failed");
      assert_eq!(s.len(), 0)
    }

    #[test]
    fn parse_eval_fncall_position() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><a><b/></a><a><c/></a></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let idoc = Item::Document(rgdoc);
      let i = Rc::new(Item::Node(idoc.get_root_element().unwrap()));

      let mut e = parse("/child::*/child::*[position() eq 1]").expect("failed to parse expression \"/child::*/child::*[position() eq 1]\"");

      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);

      let s = evaluate(&dc, Some(vec![i]), Some(0), &e).expect("evaluation failed");
      assert!(
        s.to_xml() == "<a><b/></a>" ||
        s.to_xml() == "<a><b></b></a>"
      )
    }
    #[test]
    fn parse_eval_fncall_last() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><a><b/></a><a><c/></a><a><d/></a></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let idoc = Item::Document(rgdoc);
      let i = Rc::new(Item::Node(idoc.get_root_element().unwrap()));

      let mut e = parse("/child::*/child::*[position() eq last()]").expect("failed to parse expression \"/child::*/child::*[position() eq last()]\"");

      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);

      let s = evaluate(&dc, Some(vec![i]), Some(0), &e).expect("evaluation failed");
      assert!(
        s.to_xml() == "<a><d/></a>" ||
	s.to_xml() == "<a><d></d></a>"
      )
    }
    #[test]
    fn parse_eval_fncall_count() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><a><b/></a><a><c/></a><a><d/></a></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let idoc = Item::Document(rgdoc);
      let i = Rc::new(Item::Node(idoc.get_root_element().unwrap()));

      let mut e = parse("count(/child::*/child::*)").expect("failed to parse expression \"count(/child::*/child::*)\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);

      let s = evaluate(&dc, Some(vec![i]), Some(0), &e).expect("evaluation failed");
      assert_eq!(s.to_string(), "3")
    }
    #[test]
    fn parse_eval_fncall_localname() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><a><b/></a><a><c/></a><a><d/></a></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let idoc = Item::Document(rgdoc);
      let i = Rc::new(Item::Node(idoc.get_root_element().unwrap()));

      let mut e = parse("local-name()").expect("failed to parse expression \"local-name()\"");

      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);

      let s = evaluate(&dc, Some(vec![i]), Some(0), &e).expect("evaluation failed");
      assert_eq!(s.to_string(), "Test")
    }
    #[test]
    fn parse_eval_fncall_name() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><a><b/></a><a><c/></a><a><d/></a></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let idoc = Item::Document(rgdoc);
      let i = Rc::new(Item::Node(idoc.get_root_element().unwrap()));

      let mut e = parse("name()").expect("failed to parse expression \"name()\"");

      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);

      let s = evaluate(&dc, Some(vec![i]), Some(0), &e).expect("evaluation failed");
      assert_eq!(s.to_string(), "Test")
    }

    // Kind Tests
    #[test]
    fn xpath_kind_element_1() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1>1<level2/>2<level2/>3<level2/>4<level2/>5<level2/>6<level2/>7</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap())));

      let cons = parse("child::element()").expect("failed to parse element kind expression");

      let e = evaluate(&dc, Some(vec![i]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 6);
      assert_eq!(e[0].to_name().get_localname(), "level2");
      assert_eq!(e[1].to_name().get_localname(), "level2");
      assert_eq!(e[2].to_name().get_localname(), "level2");
      assert_eq!(e[3].to_name().get_localname(), "level2");
      assert_eq!(e[4].to_name().get_localname(), "level2");
      assert_eq!(e[5].to_name().get_localname(), "level2");
    }
    #[test]
    fn xpath_kind_text_1() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1>1<level2/>2<level2/>3<level2/>4<level2/>5<level2/>6<level2/>7</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap())));

      let cons = parse("child::text()").expect("failed to parse text kind expression");

      let e = evaluate(&dc, Some(vec![i]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 7);
      assert_eq!(e[0].to_string(), "1");
      assert_eq!(e[1].to_string(), "2");
      assert_eq!(e[2].to_string(), "3");
      assert_eq!(e[3].to_string(), "4");
      assert_eq!(e[4].to_string(), "5");
      assert_eq!(e[5].to_string(), "6");
      assert_eq!(e[6].to_string(), "7");
    }
    #[test]
    fn xpath_kind_any_1() {
      init();
      let mut dc = DynamicContext::new();
      let p = Parser::default();
      let doc = p.parse_string("<Test><level1>1<level2/>2<level2/>3<level2/>4<level2/>5<level2/>6<level2/>7</level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(doc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));
      let r = (*rgdoc).get_root_element().unwrap();
      let s: &dyn Any = (*r).as_any();
      let n: &libxmlNode = match s.downcast_ref::<libxmlNode>() {
        Some(m) => m,
	None => panic!("root element must be a libxml Node"),
      };
      let i = Rc::new(Item::Node(Rc::new(n.get_first_child().unwrap())));

      let cons = parse("child::node()").expect("failed to parse text kind expression");

      let e = evaluate(&dc, Some(vec![i]), Some(0), &cons)
        .expect("evaluation failed");
      assert_eq!(e.len(), 13);
      assert_eq!(e[0].to_string(), "1");
      assert_eq!(e[1].to_name().get_localname(), "level2");
      assert_eq!(e[2].to_string(), "2");
      assert_eq!(e[3].to_name().get_localname(), "level2");
      assert_eq!(e[4].to_string(), "3");
      assert_eq!(e[5].to_name().get_localname(), "level2");
      assert_eq!(e[6].to_string(), "4");
      assert_eq!(e[7].to_name().get_localname(), "level2");
      assert_eq!(e[8].to_string(), "5");
      assert_eq!(e[9].to_name().get_localname(), "level2");
      assert_eq!(e[10].to_string(), "6");
      assert_eq!(e[11].to_name().get_localname(), "level2");
      assert_eq!(e[12].to_string(), "7");
    }

    // XSLT tests

    #[test]
    fn xslt_literal_text() {
      init();
      let sc = StaticContext::new_with_xslt_builtins();
      let instp = Parser::default();
      let instdoc = instp.parse_string("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(instdoc) as Rc<dyn Document>;

      let stylep = Parser::default();
      let styledoc = stylep.parse_string("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'>Found the document</xsl:template>
</xsl:stylesheet>").expect("failed to parse XML");
      let mut dc = from_document(Rc::new(styledoc), &sc).expect("failed to compile stylesheet");
      dc.set_doc(Rc::clone(&rgdoc));

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let i = Rc::new(Item::Document(rgdoc));
      let t = dc.find_match(&i);
      let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

      assert_eq!(seq.to_string(), "Found the document")
    }

    #[test]
    fn xslt_apply_templates_1() {
      init();
      let sc = StaticContext::new_with_xslt_builtins();
      let instp = Parser::default();
      let instdoc = instp.parse_string("<Test><Level1>one</Level1><Level2>two</Level2></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(instdoc) as Rc<dyn Document>;

      let stylep = Parser::default();
      let styledoc = stylep.parse_string("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::*'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::text()'>found text</xsl:template>
</xsl:stylesheet>").expect("failed to parse XML");
      let mut dc = from_document(Rc::new(styledoc), &sc).expect("failed to compile stylesheet");
      dc.dump_templates();
      dc.set_doc(Rc::clone(&rgdoc));

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let i = Rc::new(Item::Document(rgdoc));
      let t = dc.find_match(&i);
      let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

      assert_eq!(seq.to_string(), "found textfound text")
    }

    #[test]
    fn xslt_apply_templates_2() {
      init();
      let mut dc = DynamicContext::new();
      let sc = StaticContext::new_with_xslt_builtins();
      let instp = Parser::default();
      let instdoc = instp.parse_string("<Test>one<Level1/>two<Level1/>three<Level1/>four<Level1/></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(instdoc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));

      let stylep = Parser::default();
      let styledoc = stylep.parse_string("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:apply-templates select='child::text()'/></xsl:template>
  <xsl:template match='child::Level1'>found Level1 element</xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>").expect("failed to parse XML");
      let dc = from_document(Rc::new(styledoc), &sc).expect("failed to compile stylesheet");

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let i = Rc::new(Item::Document(rgdoc));
      let t = dc.find_match(&i);
      let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

      assert_eq!(seq.to_string(), "onetwothreefour")
    }

    #[test]
    fn xslt_sequence_1() {
      init();
      let mut dc = DynamicContext::new();
      let sc = StaticContext::new_with_xslt_builtins();
      let instp = Parser::default();
      let instdoc = instp.parse_string("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(instdoc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));

      let stylep = Parser::default();
      let styledoc = stylep.parse_string("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::*'><xsl:sequence select='count(child::*)'/></xsl:template>
</xsl:stylesheet>").expect("failed to parse XML");
      let dc = from_document(Rc::new(styledoc), &sc).expect("failed to compile stylesheet");

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let i = Rc::new(Item::Document(rgdoc));
      let t = dc.find_match(&i);
      let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

      assert_eq!(seq.to_string(), "2")
    }

    #[test]
    fn xslt_sequence_2() {
      init();
      let mut dc = DynamicContext::new();
      let sc = StaticContext::new_with_xslt_builtins();
      let instp = Parser::default();
      let instdoc = instp.parse_string("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(instdoc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));

      let stylep = Parser::default();
      let styledoc = stylep.parse_string("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::*'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>").expect("failed to parse XML");
      let dc = from_document(Rc::new(styledoc), &sc).expect("failed to compile stylesheet");

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let i = Rc::new(Item::Document(rgdoc));
      let t = dc.find_match(&i);
      let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

      assert_eq!(seq.to_string(), "onetwo")
    }

    #[test]
    fn xslt_sequence_3() {
      init();
      let mut dc = DynamicContext::new();
      let sc = StaticContext::new_with_xslt_builtins();
      let instp = Parser::default();
      let instdoc = instp.parse_string("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(instdoc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));

      let stylep = Parser::default();
      let styledoc = stylep.parse_string("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::*'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::text()'>X<xsl:sequence select='.'/>Y</xsl:template>
</xsl:stylesheet>").expect("failed to parse XML");
      let dc = from_document(Rc::new(styledoc), &sc).expect("failed to compile stylesheet");

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let i = Rc::new(Item::Document(rgdoc));
      let t = dc.find_match(&i);
      let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

      assert_eq!(seq.to_string(), "XoneYXtwoY")
    }

    #[test]
    fn xslt_literal_result_element_1() {
      init();
      let sc = StaticContext::new_with_xslt_builtins();
      let instp = Parser::default();
      let instdoc = instp.parse_string("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(instdoc) as Rc<dyn Document>;

      let stylep = Parser::default();
      let styledoc = stylep.parse_string("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><MyTest><xsl:apply-templates/></MyTest></xsl:template>
  <xsl:template match='child::Level1'><MyLevel1><xsl:apply-templates/></MyLevel1></xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>").expect("failed to parse XML");
      let mut dc = from_document(Rc::new(styledoc), &sc).expect("failed to compile stylesheet");
      dc.set_doc(Rc::clone(&rgdoc));

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let i = Rc::new(Item::Document(rgdoc));
      let t = dc.find_match(&i);
      let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

      assert_eq!(seq.to_xml(), "<MyTest><MyLevel1>one</MyLevel1><MyLevel1>two</MyLevel1></MyTest>")
    }

    #[test]
    fn xslt_if_1() {
      init();
      let sc = StaticContext::new_with_xslt_builtins();
      let instp = Parser::default();
      let instdoc = instp.parse_string("<Test><Level1>one</Level1><Level1>two</Level1><Level1><text/></Level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(instdoc) as Rc<dyn Document>;

      let stylep = Parser::default();
      let styledoc = stylep.parse_string("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Level1'><xsl:if test='child::text()'>has text</xsl:if><xsl:if test='not(child::text())'>no text</xsl:if></xsl:template>
</xsl:stylesheet>").expect("failed to parse XML");
      let mut dc = from_document(Rc::new(styledoc), &sc).expect("failed to compile stylesheet");
      dc.set_doc(Rc::clone(&rgdoc));

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let i = Rc::new(Item::Document(rgdoc));
      println!("find match for source document");
      let t = dc.find_match(&i);
      println!("evaluate seq cons of length {}", t.len());
      let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

      assert_eq!(seq.to_xml(), "has texthas textno text")
    }

    #[test]
    fn xslt_choose_1() {
      init();
      let mut dc = DynamicContext::new();
      let sc = StaticContext::new_with_xslt_builtins();
      let instp = Parser::default();
      let instdoc = instp.parse_string("<Test><Level1>one</Level1><Level1>two</Level1><Level1><text/></Level1></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(instdoc) as Rc<dyn Document>;
      dc.set_doc(Rc::clone(&rgdoc));

      let stylep = Parser::default();
      let styledoc = stylep.parse_string("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Level1'><xsl:choose><xsl:when test='child::text()'>has text</xsl:when><xsl:otherwise>no text</xsl:otherwise></xsl:choose></xsl:template>
</xsl:stylesheet>").expect("failed to parse XML");
      let dc = from_document(Rc::new(styledoc), &sc).expect("failed to compile stylesheet");

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let i = Rc::new(Item::Document(rgdoc));
      let t = dc.find_match(&i);
      let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

      assert_eq!(seq.to_xml(), "has texthas textno text")
    }

// this test is throwing a memory error
//    #[test]
//    fn xslt_foreach_1() {
//      init();
//      let sc = StaticContext::new_with_xslt_builtins();
//      let instp = Parser::default();
//      let instdoc = instp.parse_string("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("failed to parse XML");
//      let rgdoc = Rc::new(instdoc) as Rc<dyn Document>;
//
//      let stylep = Parser::default();
//      let styledoc = stylep.parse_string("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
//  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
//  <xsl:template match='child::Test'><xsl:for-each select='child::*'><group><xsl:apply-templates/></group></xsl:for-each></xsl:template>
//  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
//</xsl:stylesheet>").expect("failed to parse XML");
//      let mut dc = from_document(Rc::new(styledoc), &sc).expect("failed to compile stylesheet");
//      dc.set_doc(Rc::clone(&rgdoc));
//
      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
//      let i = Rc::new(Item::Document(rgdoc));
//      let t = dc.find_match(&i);
//      let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");
//
//      assert_eq!(seq.to_xml(), "<group>one</group><group>two</group>")
//    }

    #[test]
    fn xslt_foreach_2() {
      init();
      let sc = StaticContext::new_with_xslt_builtins();
      let instp = Parser::default();
      let instdoc = instp.parse_string("<Test><Level1>one</Level1><Level2>two</Level2><Level3>one</Level3><Level4>two</Level4></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(instdoc) as Rc<dyn Document>;

      let stylep = Parser::default();
      let styledoc = stylep.parse_string("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:for-each-group select='child::*' group-by='.'><group><xsl:apply-templates/></group></xsl:for-each-group></xsl:template>
  <xsl:template match='child::text()'>a group</xsl:template>
</xsl:stylesheet>").expect("failed to parse XML");
      let mut dc = from_document(Rc::new(styledoc), &sc).expect("failed to compile stylesheet");
      dc.set_doc(Rc::clone(&rgdoc));

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let i = Rc::new(Item::Document(rgdoc));
      let t = dc.find_match(&i);
      let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

      assert_eq!(seq.to_xml(), "<group>a group</group><group>a group</group>")
    }

    #[test]
    fn xslt_foreach_3() {
      init();
      let sc = StaticContext::new_with_xslt_builtins();
      let instp = Parser::default();
      let instdoc = instp.parse_string("<Test><Level1>one</Level1><Level2>two</Level2><Level3>one</Level3><Level4>two</Level4></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(instdoc) as Rc<dyn Document>;

      let stylep = Parser::default();
      let styledoc = stylep.parse_string("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:for-each-group select='child::*' group-by='.'><group><xsl:sequence select='current-grouping-key()'/></group></xsl:for-each-group></xsl:template>
</xsl:stylesheet>").expect("failed to parse XML");
      let mut dc = from_document(Rc::new(styledoc), &sc).expect("failed to compile stylesheet");
      dc.set_doc(Rc::clone(&rgdoc));

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let i = Rc::new(Item::Document(rgdoc));
      let t = dc.find_match(&i);
      let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

      // NB. the order that the groups appear in is undefined
      assert!(
        seq.to_xml() == "<group>one</group><group>two</group>" ||
      	seq.to_xml() == "<group>two</group><group>one</group>"
      )
    }

    #[test]
    fn xslt_foreach_4() {
      init();
      let sc = StaticContext::new_with_xslt_builtins();
      let instp = Parser::default();
      let instdoc = instp.parse_string("<Test><Level1>one</Level1><Level2>two</Level2><Level3>one</Level3><Level4>two</Level4><Level5>one</Level5></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(instdoc) as Rc<dyn Document>;

      let stylep = Parser::default();
      let styledoc = stylep.parse_string("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:for-each-group select='child::*' group-by='.'><group><key><xsl:sequence select='current-grouping-key()'/></key><members><xsl:sequence select='count(current-group())'/></members></group></xsl:for-each-group></xsl:template>
</xsl:stylesheet>").expect("failed to parse XML");
      let mut dc = from_document(Rc::new(styledoc), &sc).expect("failed to compile stylesheet");
      dc.set_doc(Rc::clone(&rgdoc));

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let i = Rc::new(Item::Document(rgdoc));
      let t = dc.find_match(&i);
      let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

      // NB. the order that the groups appear in is undefined
      assert!(
        seq.to_xml() == "<group><key>one</key><members>3</members></group><group><key>two</key><members>2</members></group>" ||
      	seq.to_xml() == "<group><key>two</key><members>2</members></group><group><key>one</key><members>3</members></group>"
      )
    }

  #[test]
  fn xslt_foreach_adj() {
      init();
      let sc = StaticContext::new_with_xslt_builtins();
      let instp = Parser::default();
      let instdoc = instp.parse_string("<Test><Level1>one</Level1><Level2>one</Level2><Level3>two</Level3><Level4>two</Level4><Level5>one</Level5></Test>").expect("failed to parse XML");
      let rgdoc = Rc::new(instdoc) as Rc<dyn Document>;

      let stylep = Parser::default();
      let styledoc = stylep.parse_string("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:for-each-group select='child::*' group-adjacent='.'><group><key><xsl:sequence select='current-grouping-key()'/></key><members><xsl:sequence select='count(current-group())'/></members></group></xsl:for-each-group></xsl:template>
</xsl:stylesheet>").expect("failed to parse XML");
      let mut dc = from_document(Rc::new(styledoc), &sc).expect("failed to compile stylesheet");
      dc.set_doc(Rc::clone(&rgdoc));

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let i = Rc::new(Item::Document(rgdoc));
      let t = dc.find_match(&i);
      let seq = evaluate(&dc, Some(vec![i.clone()]), Some(0), &t).expect("failed to evaluate stylesheet");

      // NB. the order that the groups appear in is undefined
      assert!(
        seq.to_xml() == "<group><key>one</key><members>2</members></group><group><key>two</key><members>2</members></group><group><key>one</key><members>1</members></group>" ||
      	seq.to_xml() == "<group><key>two</key><members>2</members></group><group><key>one</key><members>3</members></group>"
      )
    }
}

