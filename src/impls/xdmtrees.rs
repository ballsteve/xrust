/// Driver for trees

use std::rc::Rc;
use std::any::Any;
use std::collections::HashMap;
use trees::{Tree, RcNode};
use crate::item::{Item, Sequence, SequenceTrait, Document, Node, NodeType, Value, QualifiedName};
use crate::xdmerror::*;
use crate::evaluate::*;
//use crate::xpath::parse;
//use crate::xslt::*;

#[derive(Clone)]
pub struct NodeContent {
  node_type: NodeType,
  local_name: Option<String>,
  prefix: Option<String>,
  nsuri: Option<String>,
  attributes: HashMap<String, Value>,
  content: Option<String>,
}

impl NodeContent {
  pub fn new_document() -> NodeContent {
    NodeContent{
      node_type: NodeType::Document,
      local_name: None,
      prefix: None,
      nsuri: None,
      content: None,
      attributes: HashMap::new(),
    }
  }
  pub fn new_element(
    local_name: String,
    prefix: Option<String>,
    nsuri: Option<String>
  ) -> NodeContent {
    NodeContent{
      node_type: NodeType::Element,
      local_name: Some(local_name),
      prefix: prefix,
      nsuri: nsuri,
      content: None,
      attributes: HashMap::new(),
    }
  }
  pub fn new_text(c: String) -> NodeContent {
    NodeContent{
      node_type: NodeType::Text,
      local_name: None,
      prefix: None,
      nsuri: None,
      content: Some(c),
      attributes: HashMap::new(),
    }
  }
  // TODO: PI, comment
  pub fn get_nodetype(&self) -> NodeType {
    self.node_type
  }
  pub fn get_local_name(&self) -> String {
    self.local_name.as_ref().map_or("".to_string(), |s| s.clone())
  }
  pub fn get_namespace(&self) -> Option<String> {
    self.nsuri.as_ref().map_or(None, |s| Some(s.clone()))
  }
  pub fn get_prefix(&self) -> Option<String> {
    self.prefix.as_ref().map_or(None, |s| Some(s.clone()))
  }
  pub fn get_content(&self) -> String {
    self.content.as_ref().map_or("".to_string(), |s| s.clone())
  }
}

// Only the 'root' node of the Tree is the Document for XPath

fn find_root(s: &RcNode<NodeContent>) -> RcNode<NodeContent> {
  if s.is_root() {
    s.clone()
  } else {
    let mut r = s.clone();
    loop {
      match r.parent() {
        Some(p) => {
	  r = p.clone();
	}
	None => break,
      }
    }
    r
  }
}

impl Document for RcNode<NodeContent> {
  fn to_string(&self) -> String {
    to_string(&find_root(self))
  }
  fn to_xml(&self) -> String {
    to_xml(&find_root(self))
  }
  fn to_json(&self) -> String {
    to_json(&find_root(self))
  }
  fn to_int(&self) -> Result<i64, Error> {
    to_int(&find_root(self))
  }
  fn to_double(&self) -> f64 {
    f64::NAN
  }
  fn children(&self) -> Vec<Rc<dyn Node>> {
    let mut result: Vec<Rc<dyn Node>> = vec![];
    let mut it = find_root(self)
        .iter_rc();
    loop {
      match it.next() {
        Some(n) => {
	  result.push(Rc::new(n))
	}
	None => break,
      }
    }
    result
  }
  fn get_root_element(&self) -> Option<Rc<dyn Node>> {
    Some(Rc::new(find_root(self)))
  }

  fn new_element(&self, name: &str, _ns: Option<&str>) -> Result<Rc<dyn Node>, Error> {
    //Result::Err(Error{kind: ErrorKind::Unknown, message: "unable to create element".to_string()})
    Ok(Rc::new(RcNode::from(Tree::new(NodeContent::new_element(name.to_string(), None, None)))))
  }
  fn new_text(&self, _c: &str) -> Result<Rc<dyn Node>, Error> {
    Result::Err(Error{kind: ErrorKind::Unknown, message: "unable to create text node".to_string()})
  }
  // The parameter must be a RcNode
  fn set_root_element(&mut self, r: &dyn Any) -> Result<(), Error> {
    let n: &RcNode<NodeContent> = match r.downcast_ref::<RcNode<NodeContent>>() {
      Some(m) => m,
      None => return Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "root element must be a trees RcNode".to_string()}),
    };
    let s = find_root(self);
    if s.has_no_child() {
      s.push_back(unsafe{n.clone().into_tree()})
    } else {
      loop {
        match s.pop_front() {
	  Some(_) => {}
	  None => break,
	}
      }
      s.push_back(unsafe{n.clone().into_tree()})
    }
    Ok(())
  }
}

impl Node for RcNode<NodeContent> {
  fn as_any(&self) -> &dyn Any {
    self
  }
  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn to_name(&self) -> QualifiedName {
    let n = self.data();
    match n.get_nodetype() {
      NodeType::Element => {
        match n.get_namespace() {
      	  Some(ns) => {
            QualifiedName::new(Some(ns), Some(n.get_prefix().unwrap()), n.get_local_name())
      	  }
      	  None => QualifiedName::new(None, None, n.get_local_name())
	}
      }
      _ => QualifiedName::new(None, None, "".to_string())
    }
  }

  fn to_string(&self) -> String {
    to_string(self)
  }

  fn to_xml(&self) -> String {
    to_xml(self)
  }

  fn to_json(&self) -> String {
    to_json(self)
  }

  fn doc(&self) -> Option<Rc<dyn Document>> {
    Some(Rc::new(find_root(self)))
  }

  fn parent(&self) -> Option<Rc<dyn Node>> {
    match self.parent() {
      Some(p) => Some(Rc::new(p)),
      None => None,
    }
  }

  fn ancestors(&self) -> Vec<Rc<dyn Node>> {
    find_ancestors(self.clone())
  }

  fn children(&self) -> Vec<Rc<dyn Node>> {
    let mut result: Vec<Rc<dyn Node>> = vec![];
    let mut it = self.iter_rc();
    loop {
      match it.next() {
        Some(n) => {
	  result.push(Rc::new(n))
	}
	None => break,
      }
    }
    result
  }

  fn descendants(&self) -> Vec<Rc<dyn Node>> {
    //TreeWalk::from(self).iter().collect()
    // TODO
    vec![]
  }

  fn get_following_sibling(&self) -> Option<Rc<dyn Node>> {
    // TODO
    //match self.get_next_sib() {
      //Some(n) => Some(Rc::new(n)),
      //None => None,
    //}
    None
  }
  fn following_siblings(&self) -> Vec<Rc<dyn Node>> {
    find_following_siblings(self.clone())
  }

  fn get_preceding_sibling(&self) -> Option<Rc<dyn Node>> {
    // TODO
    //match self.get_prev_sib() {
      //Some(n) => Some(Rc::new(n)),
      //None => None,
    //}
    None
  }
  fn preceding_siblings(&self) -> Vec<Rc<dyn Node>> {
    find_preceding_siblings(self.clone())
  }

  fn to_int(&self) -> Result<i64, Error> {
    to_int(self)
  }
  fn to_double(&self) -> f64 {
    match to_string(self).parse::<f64>() {
      Ok(f) => f,
      Result::Err(_) => f64::NAN
    }
  }

  fn attribute(&self, _name: &str) -> Option<String> {
    // TODO
    None
  }

  fn node_type(&self) -> NodeType {
    self.data().get_nodetype()
  }

  fn is_element(&self) -> bool {
    match self.data().get_nodetype() {
      NodeType::Element => true,
      _ => false,
    }
  }

  fn add_child(&self, c: &dyn Any) -> Result<(), Error>{
    let e = match c.downcast_ref::<RcNode<NodeContent>>() {
      Some(d) => d,
      None => return Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "child node must be a trees Node".to_string()}),
    };
    self.push_back(unsafe{e.clone().into_tree()});
    Ok(())
  }
  fn add_text_child(&self, t: String) -> Result<(), Error> {
    self.push_back(
      Tree::new(NodeContent::new_text(t))
    );
    Ok(())
  }
}

fn to_int(s: &RcNode<NodeContent>) -> Result<i64, Error> {
    match to_string(s).parse::<i64>() {
      Ok(i) => Ok(i),
      Result::Err(e) => Result::Err(Error{kind: ErrorKind::Unknown, message: e.to_string()}),
    }
}

fn to_string(s: &RcNode<NodeContent>) -> String {
    let d = s.data();
    match d.get_nodetype() {
      NodeType::Document => {
        if s.has_no_child() {
	  String::new()
	} else {
	  s.iter_rc().fold(String::new(), |s,c| s + to_string(&c).as_str())
	}
      }
      NodeType::Element => {
        if s.has_no_child() {
	  "".to_string()
	} else {
	  s.iter_rc().fold(String::new(), |s,c| s + to_string(&c).as_str())
	}
      }
      NodeType::Text => {
        String::from(d.get_content())
      }
      NodeType::Attribute |
      NodeType::Comment |
      NodeType::ProcessingInstruction => {
        String::from(d.get_content())
      }
      _ => {
	"".to_string()
      }
    }
}

fn to_xml(s: &RcNode<NodeContent>) -> String {
    let d = s.data();

    match d.get_nodetype() {
      NodeType::Document => {
        if s.has_no_child() {
	  String::new()
	} else {
	  s.iter_rc().fold(String::new(), |s,c| s + to_xml(&c).as_str())
	}
      }
      NodeType::Element => {
        if s.has_no_child() {
	  format!("<{}/>", d.get_local_name()) // TODO: namespace
	} else {
	  // TODO: attributes
	  format!("<{}>{}</{}>", d.get_local_name(), s.iter_rc().fold(String::new(), |s,c| s + to_xml(&c).as_str()), d.get_local_name())
	}
      }
      NodeType::Text => {
        String::from(d.get_content())
      }
      NodeType::Attribute => {
        let mut r = String::new();
        r.push_str(d.get_local_name().as_str());
        r.push_str("='");
        r.push_str(d.get_content().as_str());
        r.push_str("'");
        // TODO: delimiters, escaping
	r
      }
      NodeType::Comment => {
        let mut r = String::new();
        r.push_str("<!--");
        r.push_str(d.get_content().as_str());
        r.push_str("-->");
	r
      }
      NodeType::ProcessingInstruction => {
        let mut r = String::new();
        r.push_str("<?");
        r.push_str(d.get_local_name().as_str());
        r.push_str(" ");
        r.push_str(d.get_content().as_str());
        r.push_str("?>");
	r
      }
      _ => {
	"".to_string()
      }
    }
}

fn to_json(s: &RcNode<NodeContent>) -> String {
    let d = s.data();

    match d.get_nodetype() {
      NodeType::Document => {
        if s.has_no_child() {
	  "{}".to_string()
	} else {
	  let mut r = String::from("{");
	  r.push_str(s.iter_rc().fold(String::new(), |s,c| s + to_json(&c).as_str()).as_str());
	  r.push('}');
	  r
	}
      }
      NodeType::Element => {
        if s.has_no_child() {
	  format!("\"{}\": \"\"", d.get_local_name())
	} else {
	  format!("\"{}\": {}\n", d.get_local_name(), s.iter_rc().fold(String::new(), |s,c| s + to_json(&c).as_str()))
	}
      }
      NodeType::Text => {
        format!("\"{}\"", String::from(d.get_content()))
      }
      NodeType::Attribute => {
        "".to_string()
      }
      NodeType::Comment => {
        "".to_string()
      }
      NodeType::ProcessingInstruction => {
        "".to_string()
      }
      _ => {
	"".to_string()
      }
    }
}

fn find_ancestors(n: RcNode<NodeContent>) -> Vec<Rc<dyn Node>> {
  match n.parent() {
    Some(p) => {
      let mut anc = find_ancestors(p.clone());
      anc.insert(0, Rc::new(p.clone()));
      anc
    }
    None => vec![]
  }
}

fn find_following_siblings(_n: RcNode<NodeContent>) -> Vec<Rc<dyn Node>> {
  vec![]
  // TODO
  //match n.get_next_sib() {
    //Some(p) => {
      //let mut anc = find_following_siblings(p.clone());
      //anc.insert(0, Rc::new(p.clone()));
      //anc
    //}
    //None => vec![]
  //}
}
fn find_preceding_siblings(_n: RcNode<NodeContent>) -> Vec<Rc<dyn Node>> {
  vec![]
  // TODO
  //match n.get_prev_sib() {
    //Some(p) => {
      //let mut anc = find_preceding_siblings(p.clone());
      //anc.insert(0, Rc::new(p.clone()));
      //anc
    //}
    //None => vec![]
  //}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_document() {
        RcNode::from(Tree::new(NodeContent::new_document()));
        assert!(true)
    }
    #[test]
    fn node_element() {
        let d = RcNode::from(Tree::new(NodeContent::new_document()));
        let e = Tree::new(NodeContent::new_element("Test".to_string(), None, None));
	d.push_front(e);
        assert_eq!(to_string(&d), "");
        assert_eq!(to_xml(&d), "<Test/>");
        assert_eq!(to_json(&d), "{\"Test\": \"\"}")
    }
    #[test]
    fn node_text() {
        let d = RcNode::from(Tree::new(NodeContent::new_document()));
        let mut e = Tree::new(NodeContent::new_element("Test".to_string(), None, None));
        let t = Tree::new(NodeContent::new_text("Test text".to_string()));
	e.push_front(t);
	d.push_front(e);
        assert_eq!(to_string(&d), "Test text");
        assert_eq!(to_xml(&d), "<Test>Test text</Test>")
    }
    #[test]
    fn item_node_to_string() {
        let d = RcNode::from(Tree::new(NodeContent::new_document()));
        let mut e = Tree::new(NodeContent::new_element("Test".to_string(), None, None));
        let t = Tree::new(NodeContent::new_text("Test text".to_string()));
	e.push_front(t);
	d.push_front(e);
	let i = Item::Node(Rc::new(d));
        assert_eq!(i.to_string(), "Test text");
        assert_eq!(i.to_xml(), "<Test>Test text</Test>")
    }
    #[test]
    fn node_root() {
        let mut d = RcNode::from(Tree::new(NodeContent::new_document()));
	let n = d.new_element("Test", None).expect("unable to create element");
	d.set_root_element(n.as_any()).expect("unable to set root element");
	assert_eq!(Item::Document(Rc::new(d)).to_xml(), "<Test/>")
    }
    #[test]
    fn node_add_child() {
        let mut d = RcNode::from(Tree::new(NodeContent::new_document()));
	let n = d.new_element("Test", None).expect("unable to create element");
	d.set_root_element(n.as_any()).expect("unable to set root element");
	let m = d.new_element("Data", None).expect("unable to create Data element");
	n.add_child(m.as_any()).expect("unable to add child");
	assert_eq!(Item::Document(Rc::new(d)).to_xml(), "<Test><Data/></Test>")
    }
    #[test]
    fn node_add_text() {
        let mut d = RcNode::from(Tree::new(NodeContent::new_document()));
	let n = d.new_element("Test", None).expect("unable to create element");
	d.set_root_element(n.as_any()).expect("unable to set root element");
	let m = d.new_element("Data", None).expect("unable to create Data element");
	n.add_child(m.as_any()).expect("unable to add child");
	m.add_text_child("this is a test".to_string()).expect("unable to add text");
	assert_eq!(Item::Document(Rc::new(d)).to_xml(), "<Test><Data>this is a test</Data></Test>")
    }
    #[test]
    fn node_name() {
        let mut d = RcNode::from(Tree::new(NodeContent::new_document()));
	let n = d.new_element("Test", None).expect("unable to create element");
	d.set_root_element(n.as_any()).expect("unable to set root element");
	let m = d.new_element("Data", None).expect("unable to create Data element");
	n.add_child(m.as_any()).expect("unable to add child");
	m.add_text_child("this is a test".to_string()).expect("unable to add text");
	assert_eq!(Item::Node(n).to_name().get_localname(), "Test");
	assert_eq!(Item::Node(m).to_name().get_localname(), "Data");
    }
    #[test]
    fn node_child() {
        let mut d = RcNode::from(Tree::new(NodeContent::new_document()));
	let n = d.new_element("Test", None).expect("unable to create element");
	d.set_root_element(n.as_any()).expect("unable to set root element");
	let m = d.new_element("Data", None).expect("unable to create Data element");
	n.add_child(m.as_any()).expect("unable to add child");
	m.add_text_child("this is a test".to_string()).expect("unable to add text");
	let c = n.children();
	assert_eq!(c.len(), 1);
	assert_eq!(c[0].to_name().get_localname(), "Data");
    }
    #[test]
    fn node_parent() {
        let mut d = RcNode::from(Tree::new(NodeContent::new_document()));
	let n = d.new_element("Test", None).expect("unable to create element");
	d.set_root_element(n.as_any()).expect("unable to set root element");
	let m = d.new_element("Data", None).expect("unable to create Data element");
	n.add_child(m.as_any()).expect("unable to add child");
	m.add_text_child("this is a test".to_string()).expect("unable to add text");
	let p = m.parent().unwrap();
	assert_eq!(p.to_name().get_localname(), "Test");
    }

    // XPath evaluation tests

    #[test]
    fn xpath_root() {
        let mut d = RcNode::from(Tree::new(NodeContent::new_document()));
	let n = d.new_element("Test", None).expect("unable to create element");
	d.set_root_element(n.as_any()).expect("unable to set root element");
	let m = d.new_element("Data", None).expect("unable to create Data element");
	n.add_child(m.as_any()).expect("unable to add child");
	m.add_text_child("this is a test".to_string()).expect("unable to add text");

	// XPath == /
	let cons = vec![Constructor::Root];

	let dc = DynamicContext::new();
	let e: Sequence = evaluate(&dc, Some(vec![Rc::new(Item::Node(m))]), Some(0), &cons).expect("evaluation failed");

	assert_eq!(e.len(), 1);
	assert_eq!(e.to_xml(), "<Test><Data>this is a test</Data></Test>");
    }
}

