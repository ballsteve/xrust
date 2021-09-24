/*! Document and Node implementation using XDMTree

```rust
# use std::rc::Rc;
# use std::cell::RefCell;
# use xrust::item::{Item, Document, Sequence, SequenceTrait};
# use xrust::evaluate::{StaticContext, DynamicContext, evaluate};
# use xrust::xpath::parse;
# use xrust::xslt::from_document;
# use petgraph::graph::Graph;
# use xrust::xdmgraph::{XDMTree, XDMTreeNode, from};

// First create a XDMTreeNode from the source XML
let src = from("<MyTest>This is the source document</MyTest>").expect("unable to parse source XML");
// Make this an [Item]
let isrc = Rc::new(Item::Document(Rc::new(src.get_doc())));

// Parse the XSL stylesheet
let style = from("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'>It works!</xsl:template>
</xsl:stylesheet>").expect("unable to parse XSL stylesheet");
let istyle = Rc::new(style.get_doc());

// Setup dynamic context with result document
let sc = StaticContext::new_with_xslt_builtins();
let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
XDMTreeNode::new(rd.clone());
let dc = from_document(istyle, &rd, &sc).expect("failed to compile stylesheet");

// Prime the stylesheet evaluation by finding the template for the document root
// and making the document root the initial context
let t = dc.find_match(&isrc);

// Now evaluate the Sequence Constructor
// with the source document as the initial context
let seq = evaluate(&dc, Some(vec![isrc]), Some(0), &t).expect("evaluation failed");

assert_eq!(seq.to_string(), "It works!");
```

*/

use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;
use petgraph::graph::Graph;
use crate::xdmgraph::{XDMTree, XDMTreeNode, NodeType as TreeNodeType, from};
use crate::item::*;
use crate::evaluate::*;
use crate::xpath::*;
use crate::xslt::*;
use crate::xdmerror::*;

impl Document for XDMTree {
  fn to_string(&self) -> String {
    get_doc_node(self).to_string()
  }
  fn to_xml(&self) -> String {
    get_doc_node(self).to_xml()
  }
  fn to_json(&self) -> String {
    // TODO
    "".to_string()
  }
  fn children(&self) -> Vec<Rc<dyn Node>> {
    let mut it = get_doc_node(self).child_iter();
    let mut result: Vec<Rc<dyn Node>> = vec![];
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
    // TODO: handle prologue, epilogue
    let mut it = get_doc_node(self).child_iter();
    let mut result = vec![];
    loop {
      match it.next() {
        Some(n) => {
	  result.push(Rc::new(n))
	}
	None => break,
      }
    }
    if result.len() == 1 {
      Some(result[0].clone())
    } else {
      None
    }
  }
  fn new_element(&self, name: &str, _ns: Option<&str>) -> Result<Rc<dyn Node>, Error> {
    // TODO: namespaces
    Ok(Rc::new(get_doc_node(self).new_element(QualifiedName::new(None, None, name.to_string()))))
  }
  fn new_text(&self, c: &str) -> Result<Rc<dyn Node>, Error> {
    Ok(Rc::new(get_doc_node(self).new_value(Value::String(c.to_string()))))
  }
  fn set_root_element(&mut self, r: &dyn Any) -> Result<(), Error> {
    let n: &XDMTreeNode = match r.downcast_ref::<XDMTreeNode>() {
      Some(m) => m,
      None => return Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "root element must be a XDMTreeNode".to_string()}),
    };
    // TODO: If the document already has a root element then remove it
    get_doc_node(self).append_child(n.clone());
    Ok(())
  }
}

impl Node for XDMTreeNode {
  fn as_any(&self) -> &dyn Any {
    self
  }
  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn to_string(&self) -> String {
    match self.get_doc().borrow()[self.get_index()] {
      TreeNodeType::Document => {
	self.get_first_child()
	  .map_or("".to_string(), |n| n.to_string())
      }
      TreeNodeType::Element(_) => {
      	let mut result = String::new();
	self.child_iter().for_each(
          |c| {
	    result.push_str(c.to_string().as_str());
	  }
        );
	result
      }
      TreeNodeType::Text(ref t) => {
        t.to_string()
      }
      TreeNodeType::Attribute(ref v) => {
        v.to_string()
      }
    }
  }
  fn to_xml(&self) -> String {
    self.to_xml_int()
  }
  fn to_json(&self) -> String {
    // TODO
    "".to_string()
  }

  // A non-empty node is always true
  fn to_bool(&self) -> bool {
    true
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

  fn to_name(&self) -> QualifiedName {
    match self.get_doc().borrow()[self.get_index()] {
      TreeNodeType::Document => {
        QualifiedName::new(None, None, "".to_string())
      }
      TreeNodeType::Element(ref e) => {
      	// TODO: namespaces
	e.name.clone()
      }
      TreeNodeType::Text(_) => {
        QualifiedName::new(None, None, "".to_string())
      }
      TreeNodeType::Attribute(_) => {
        self.get_name()
      }
    }
  }

  fn doc(&self) -> Option<Rc<dyn Document>> {
    Some(Rc::new(self.get_doc()))
  }

  fn node_type(&self) -> NodeType {
    match self.get_doc().borrow()[self.get_index()] {
      TreeNodeType::Document => NodeType::Document,
      TreeNodeType::Element(_) => NodeType::Element,
      TreeNodeType::Text(_) => NodeType::Text,
      TreeNodeType::Attribute(_) => NodeType::Attribute,
    }
  }

  fn parent(&self) -> Option<Rc<dyn Node>> {
    match self.ancestor_iter().nth(0) {
      Some(p) => {
        Some(Rc::new(p))
      }
      None => None
    }
  }

  // TODO: redo as iterators
  fn ancestors(&self) -> Vec<Rc<dyn Node>> {
    //self.ancestor_iter().map(|n| Rc::new(n)).collect()
    let mut it = self.ancestor_iter();
    let mut result: Vec<Rc<dyn Node>> = vec![];
    loop {
      match it.next() {
        Some(a) => {
	  result.push(Rc::new(a))
	}
	None => break,
      }
    }
    result
  }
  fn children(&self) -> Vec<Rc<dyn Node>> {
    //self.child_iter().map(|n| Rc::new(n)).collect()
    let mut it = self.child_iter();
    let mut result: Vec<Rc<dyn Node>> = vec![];
    loop {
      match it.next() {
        Some(a) => {
	  result.push(Rc::new(a))
	}
	None => break,
      }
    }
    result
  }
  fn following_siblings(&self) -> Vec<Rc<dyn Node>> {
    //self.sibling_iter().map(|n| Rc::new(n)).collect()
    let mut it = self.sibling_iter();
    let mut result: Vec<Rc<dyn Node>> = vec![];
    loop {
      match it.next() {
        Some(a) => {
	  result.push(Rc::new(a))
	}
	None => break,
      }
    }
    result
  }
  fn preceding_siblings(&self) -> Vec<Rc<dyn Node>> {
    //self.preceding_sibling_iter().map(|n| Rc::new(n)).collect()
    let mut it = self.preceding_sibling_iter();
    let mut result: Vec<Rc<dyn Node>> = vec![];
    loop {
      match it.next() {
        Some(a) => {
	  result.push(Rc::new(a))
	}
	None => break,
      }
    }
    result
  }

  fn descendants(&self) -> Vec<Rc<dyn Node>> {
    match self.get_doc().borrow()[self.get_index()] {
      TreeNodeType::Document => {
        match self.get_first_child() {
	  Some(r) => {
	    let mut v: Vec<Rc<dyn Node>> = vec![Rc::new(r.clone())];
	    let mut d = r.descendants();
	    v.append(&mut d);
	    v
	  }
	  None => vec![],
	}
      }
      TreeNodeType::Element(_) => {
	self.child_iter()
	  .fold(
	    vec![],
	    |mut a, n| {
	      let mut w = n.descendants();
	      a.push(Rc::new(n));
	      a.append(&mut w);
	      a
	    }
	  )
      }
      _ => {
        vec![]
      }
    }
  }
  fn get_following_sibling(&self) -> Option<Rc<dyn Node>> {
    //self.sibling_iter().nth(0).map(|n| Rc::new(n))
    match self.sibling_iter().nth(0) {
      Some(n) => {
        Some(Rc::new(n))
      }
      None => None,
    }
  }
  fn get_preceding_sibling(&self) -> Option<Rc<dyn Node>> {
    //self.preceding_sibling_iter().nth(0).map(|n| Rc::new(n))
    match self.preceding_sibling_iter().nth(0) {
      Some(n) => {
        Some(Rc::new(n))
      }
      None => None,
    }
  }

  // TODO
  fn attribute(&self, name: &str) -> Option<String> {
    self.get_attribute(QualifiedName::new(None, None, name.to_string()))
      .map(|v| v.to_string())
  }

  fn is_element(&self) -> bool {
    match self.get_doc().borrow()[self.get_index()] {
      TreeNodeType::Element(_) => true,
      TreeNodeType::Document |
      TreeNodeType::Attribute(_) |
      TreeNodeType::Text(_) => false,
    }
  }

  fn add_child(&self, c: &dyn Any) -> Result<(), Error> {
    let e = match c.downcast_ref::<XDMTreeNode>() {
      Some(d) => d,
      None => return Result::Err(Error{kind: ErrorKind::DynamicAbsent, message: "child node must be a XDMTreeNode".to_string()}),
    };
    Ok(self.append_child(e.clone()))
  }
  fn add_text_child(&self, t: String) -> Result<(), Error> {
    let t = self.new_value(Value::String(t));
    self.append_child(t);
    Ok(())
  }
  fn remove(&self) -> Result<(), Error> {
    self.remove_node();
    Ok(())
  }
}

fn get_doc_node(g: &XDMTree) -> XDMTreeNode {
    let h = g.borrow();
    let r = match h.node_indices()
      .find(|i| match h[*i] {
          TreeNodeType::Document => true,
          _ => false,
        }) {
      Some(r) => {
        r
      }
      None => {
        panic!("no document node")
      }
    };
    XDMTreeNode::new_node(g.clone(), r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doc() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(t.clone());
      Item::Document(Rc::new(t));
    }

    #[test]
    fn node() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r);
      Item::Node(Rc::new(d));
    }

    #[test]
    fn node_xml() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r);
      let i = Item::Node(Rc::new(d));

      assert!(i.to_xml() == "<Test/>" || i.to_xml() == "<Test></Test>")
    }

    #[test]
    fn node_str() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      let u = d.new_value(Value::String("this is a test".to_string()));
      r.append_child(u);
      d.append_child(r);
      let i = Item::Node(Rc::new(d));

      assert_eq!(i.to_string(), "this is a test")
    }

    #[test]
    fn doc_name() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      let u = d.new_value(Value::String("this is a test".to_string()));
      r.append_child(u);
      d.append_child(r);
      let i = Item::Node(Rc::new(d));

      assert_eq!(i.to_name().get_localname(), "")
    }
    #[test]
    fn element_name() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      let u = d.new_value(Value::String("this is a test".to_string()));
      r.append_child(u);
      d.append_child(r.clone());
      let i = Item::Node(Rc::new(r));

      assert_eq!(i.to_name().get_localname(), "Test")
    }

    #[test]
    fn new_element() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let n = t.new_element("Data", None).expect("unable to create element");
      r.add_child(n.as_any()).expect("unable to add child");

      let e = Item::Document(Rc::new(t));

      assert_eq!(e.to_xml(), "<Test><Data></Data></Test>");
    }

    #[test]
    fn new_value() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let n = t.new_element("Data", None).expect("unable to create element");
      r.add_child(n.as_any()).expect("unable to add child");
      n.add_text_child("this is a test".to_string()).expect("unable to add text");

      let e = Item::Document(Rc::new(t));

      assert_eq!(e.to_xml(), "<Test><Data>this is a test</Data></Test>");
    }

    #[test]
    fn descend() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      r.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c1.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c2.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      c3.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("this is a test".to_string()).expect("unable to add text");

      assert_eq!(r.descendants().len(), 5);
    }

    #[test]
    fn ascend() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      r.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c1.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c2.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      c3.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("this is a test".to_string()).expect("unable to add text");

      assert_eq!(c4.ancestors().len(), 4);
    }

    #[test]
    fn siblings() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      r.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      r.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      r.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      r.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      assert_eq!(c1.following_siblings().len(), 3);
    }

    #[test]
    fn preceding_siblings() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      r.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      r.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      r.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      r.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      assert_eq!(c4.preceding_siblings().len(), 3);
    }

    // Evaluation tests

    #[test]
    fn eval_root() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      r.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c1.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c2.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      c3.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("this is a test".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
      // XPath == /
      let cons = vec![Constructor::Root];
      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(c4))]), Some(0), &cons)
        .expect("evaluation failed");

      if e.len() == 1 {
        assert_eq!(e[0].to_xml(), "<Test><Child1><Child2><Child3><Child4>this is a test</Child4></Child3></Child2></Child1></Test>")
      } else {
        panic!("sequence is not a singleton")
      }
    }

    #[test]
    fn eval_child_all() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      r.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      r.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      r.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      r.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
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

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(Rc::new(r)))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 4)
    }

    #[test]
    fn eval_self_pos() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      r.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      r.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      r.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      r.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
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

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(Rc::new(r)))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 1);
      assert_eq!(e[0].to_name().get_localname(), "Test");
    }

    #[test]
    fn eval_self_neg() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      r.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      r.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      r.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      r.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");
      let uv = c1.children();

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
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

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(uv[0].clone()))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 0);
    }

    #[test]
    fn eval_parent_any() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      r.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      r.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      r.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      r.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
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

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(c1))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 1);
      assert_eq!(e[0].to_name().get_localname(), "Test");
    }

    #[test]
    fn eval_descendant_1() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      r.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      r.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      r.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      r.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
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

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(Rc::new(r)))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 4);
    }

    #[test]
    fn eval_descendantorself_1() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      r.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      r.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      r.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      r.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
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

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(Rc::new(r)))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 5);
    }

    #[test]
    fn eval_ancestor_1() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      r.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      c1.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      c2.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      c3.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
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

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(c4))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 4);
    }

    #[test]
    fn eval_ancestororself_1() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      r.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      c1.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      c2.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      c3.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
      // XPath == ancestor-or-self::*
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

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(c4))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 5);
    }

    #[test]
    fn eval_followingsibling_1() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      r.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      r.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      r.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      r.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
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

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(c1))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 3);
    }

    #[test]
    fn eval_precedingsibling_1() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      r.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      r.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      r.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      r.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
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

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(c4))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 3);
    }

    #[test]
    fn eval_following_1() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      l1.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child1", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      l1.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      l2.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      l2.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
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

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(c2))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 3);
    }

    #[test]
    fn eval_preceding_1() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      l1.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      l1.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      l2.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      l2.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
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

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(c4))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 4);
    }

    #[test]
    fn eval_path_1() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      l1.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      l1.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      l2.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      l2.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
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

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(c4))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 2);
      assert_eq!(e[0].to_name().get_localname(), "Level1");
      assert_eq!(e[1].to_name().get_localname(), "Level2");
    }

    #[test]
    fn eval_nametest_pos() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      l1.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      l1.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      l2.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      l2.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
      // XPath == /child::Test
      let cons = vec![
	  Constructor::Path(
	    vec![
	      vec![Constructor::Root],
              vec![Constructor::Step(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Test".to_string()))})}, vec![])],
            ]
	  )
	];

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(Rc::new(r)))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 1);
      assert_eq!(e[0].to_name().get_localname(), "Test");
    }
    #[test]
    fn eval_nametest_neg() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      l1.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      l1.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      l2.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      l2.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
      // XPath == /child::Foo
      let cons = vec![
	  Constructor::Path(
	    vec![
	      vec![Constructor::Root],
              vec![Constructor::Step(NodeMatch{axis: Axis::Child, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name("Foo".to_string()))})}, vec![])],
            ]
	  )
	];

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(Rc::new(r)))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 0);
    }

    #[test]
    fn eval_kindtest_element() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      r.add_text_child("i1".to_string()).expect("unable to add text");
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      r.add_text_child("i2".to_string()).expect("unable to add text");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      r.add_text_child("i3".to_string()).expect("unable to add text");
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      l1.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      l1.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      l2.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      l2.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
      // XPath == child::element()
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Child,
	      nodetest: NodeTest::Kind(KindTest::ElementTest)
	    },
	    vec![]
	  )
	];

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(Rc::new(r)))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 2);
    }
    #[test]
    fn eval_kindtest_text() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      r.add_text_child("i1".to_string()).expect("unable to add text");
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      r.add_text_child("i2".to_string()).expect("unable to add text");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      r.add_text_child("i3".to_string()).expect("unable to add text");
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      l1.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      l1.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      l2.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      l2.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
      // XPath == child::text()
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Child,
	      nodetest: NodeTest::Kind(KindTest::TextTest)
	    },
	    vec![]
	  )
	];

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(Rc::new(r)))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 3);
    }
    #[test]
    fn eval_kindtest_any() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      r.add_text_child("i1".to_string()).expect("unable to add text");
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      r.add_text_child("i2".to_string()).expect("unable to add text");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      r.add_text_child("i3".to_string()).expect("unable to add text");
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      l1.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      l1.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      l2.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      l2.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
      // XPath == child::node()
      let cons = vec![
	  Constructor::Step(
	    NodeMatch{
	      axis: Axis::Child,
	      nodetest: NodeTest::Kind(KindTest::AnyKindTest)
	    },
	    vec![]
	  )
	];

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(Rc::new(r)))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 5);
    }

    #[test]
    fn eval_predicate_pos() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      r.add_text_child("i1".to_string()).expect("unable to add text");
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      r.add_text_child("i2".to_string()).expect("unable to add text");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      r.add_text_child("i3".to_string()).expect("unable to add text");
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      l1.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      l1.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      l2.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      l2.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
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

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(Rc::new(r)))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 1);
    }
    #[test]
    fn eval_predicate_neg() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      r.add_text_child("i1".to_string()).expect("unable to add text");
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      r.add_text_child("i2".to_string()).expect("unable to add text");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      r.add_text_child("i3".to_string()).expect("unable to add text");
      let c1 = t.new_element("Child1", None).expect("unable to create element");
      c1.add_text_child("one".to_string()).expect("unable to add text");
      l1.add_child(c1.as_any()).expect("unable to add child");
      let c2 = t.new_element("Child2", None).expect("unable to create element");
      c2.add_text_child("two".to_string()).expect("unable to add text");
      l1.add_child(c2.as_any()).expect("unable to add child");
      let c3 = t.new_element("Child3", None).expect("unable to create element");
      c3.add_text_child("three".to_string()).expect("unable to add text");
      l2.add_child(c3.as_any()).expect("unable to add child");
      let c4 = t.new_element("Child4", None).expect("unable to create element");
      l2.add_child(c4.as_any()).expect("unable to add child");
      c4.add_text_child("four".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
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

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(Rc::new(r)))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 0);
    }

    // Node-related Functions

    #[test]
    fn eval_fncall_localname() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      r.add_text_child("i1".to_string()).expect("unable to add text");
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      r.add_text_child("i2".to_string()).expect("unable to add text");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      r.add_text_child("i3".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
      // XPath == /Test[foo]
      let cons = vec![
	  Constructor::FunctionCall(
            Function::new("local-name".to_string(), vec![], Some(func_localname)),
	    vec![]
      	  )
	];

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(Rc::new(r)))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 1);
      assert_eq!(e[0].to_string(), "Test");
    }

    #[test]
    fn eval_fncall_name() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      r.add_text_child("i1".to_string()).expect("unable to add text");
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      r.add_text_child("i2".to_string()).expect("unable to add text");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      r.add_text_child("i3".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
      // XPath == /Test[foo]
      let cons = vec![
	  Constructor::FunctionCall(
            Function::new("name".to_string(), vec![], Some(func_name)),
	    vec![]
      	  )
	];

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(Rc::new(r)))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 1);
      assert_eq!(e[0].to_string(), "Test");
    }
    // TODO: test qualified name

    // Patterns

    #[test]
    fn pattern_1_pos() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      r.add_text_child("i1".to_string()).expect("unable to add text");
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      r.add_text_child("i2".to_string()).expect("unable to add text");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      r.add_text_child("i3".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
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

      assert_eq!(item_matches(&dc, &p, &Rc::new(Item::Node(Rc::new(r)))), true);
    }
    // TODO: matching a text node should return false

    #[test]
    fn pattern_2_pos() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      r.add_text_child("i1".to_string()).expect("unable to add text");
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      r.add_text_child("i2".to_string()).expect("unable to add text");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      r.add_text_child("i3".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
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

      assert_eq!(item_matches(&dc, &p, &Rc::new(Item::Node(Rc::new(r)))), true);
    }
    #[test]
    fn pattern_2_neg() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      r.add_text_child("i1".to_string()).expect("unable to add text");
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      r.add_text_child("i2".to_string()).expect("unable to add text");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      r.add_text_child("i3".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
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

      assert_eq!(item_matches(&dc, &p, &Rc::new(Item::Node(Rc::new(r)))), false);
    }

    #[test]
    fn pattern_3_pos() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      r.add_text_child("i1".to_string()).expect("unable to add text");
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      r.add_text_child("i2".to_string()).expect("unable to add text");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      r.add_text_child("i3".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
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

      assert_eq!(item_matches(&dc, &p, &Rc::new(Item::Node(l2))), true);
    }

    #[test]
    fn pattern_4_pos() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.append_child(r.clone());
      r.add_text_child("i1".to_string()).expect("unable to add text");
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      r.add_text_child("i2".to_string()).expect("unable to add text");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      r.add_text_child("i3".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
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

      assert_eq!(item_matches(&dc, &p, &Rc::new(Item::Node(l2))), true);
    }
    #[test]
    fn pattern_4_neg() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let x = d.new_element(QualifiedName::new(None, None, "Root".to_string()));
      d.append_child(x.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      x.add_child(r.as_any()).expect("unable to add child");
      r.add_text_child("i1".to_string()).expect("unable to add text");
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      r.add_text_child("i2".to_string()).expect("unable to add text");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      r.add_text_child("i3".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
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

      assert_eq!(item_matches(&dc, &p, &Rc::new(Item::Node(l2))), false);
    }

    #[test]
    fn pattern_5_pos() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let x = d.new_element(QualifiedName::new(None, None, "Root".to_string()));
      d.append_child(x.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      x.add_child(r.as_any()).expect("unable to add child");
      r.add_text_child("i1".to_string()).expect("unable to add text");
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      r.add_text_child("i2".to_string()).expect("unable to add text");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      r.add_text_child("i3".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
      // XPath == /
      let cons = vec![Constructor::Path(
	    vec![
	      vec![Constructor::Root],
            ]
	  )];
      let p = to_pattern(cons).expect("unable to reverse expression");

      assert_eq!(item_matches(&dc, &p, &Rc::new(Item::Document(Rc::new(t)))), true);
    }
    #[test]
    fn pattern_5_neg() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let x = d.new_element(QualifiedName::new(None, None, "Root".to_string()));
      d.append_child(x.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      x.add_child(r.as_any()).expect("unable to add child");
      r.add_text_child("i1".to_string()).expect("unable to add text");
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      r.add_text_child("i2".to_string()).expect("unable to add text");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      r.add_text_child("i3".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));
      // XPath == /
      let cons = vec![Constructor::Path(
	    vec![
	      vec![Constructor::Root],
            ]
	  )];
      let p = to_pattern(cons).expect("unable to reverse expression");

      assert_eq!(item_matches(&dc, &p, &Rc::new(Item::Node(l2))), false);
    }

    // Literal result elements

    #[test]
    fn literal_element_1() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.add_child(r.as_any()).expect("unable to add child");
      r.add_text_child("i1".to_string()).expect("unable to add text");
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      r.add_text_child("i2".to_string()).expect("unable to add text");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      r.add_text_child("i3".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));

      let cons = vec![
        Constructor::LiteralElement("New".to_string(), "".to_string(), "".to_string(), vec![]),
      ];

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(Rc::new(r)))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 1);
      assert!(
        e[0].to_xml() == "<New/>" ||
        e[0].to_xml() == "<New></New>"
      );
    }

    #[test]
    fn literal_element_2() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.add_child(r.as_any()).expect("unable to add child");
      r.add_text_child("i1".to_string()).expect("unable to add text");
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      r.add_text_child("i2".to_string()).expect("unable to add text");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      r.add_text_child("i3".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));

      let cons = vec![
        Constructor::LiteralElement("New".to_string(), "".to_string(), "".to_string(),
	  vec![
	    Constructor::LiteralElement("Level1".to_string(), "".to_string(), "".to_string(),
	      vec![
	        Constructor::Literal(Value::String("Test text".to_string())),
	      ]
	    )
	  ]
	),
      ];

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(Rc::new(r)))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 1);
      assert_eq!(e[0].to_xml(), "<New><Level1>Test text</Level1></New>");
    }

    #[test]
    fn literal_element_3() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.add_child(r.as_any()).expect("unable to add child");
      r.add_text_child("i1".to_string()).expect("unable to add text");
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      r.add_text_child("i2".to_string()).expect("unable to add text");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      r.add_text_child("i3".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));

      let cons = vec![
        Constructor::LiteralElement("New".to_string(), "".to_string(), "".to_string(),
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

      let e = evaluate(&dc, Some(vec![Rc::new(Item::Node(Rc::new(r)))]), Some(0), &cons)
        .expect("evaluation failed");

      assert_eq!(e.len(), 1);
      assert_eq!(e[0].to_xml(), "<New><Level1>one</Level1><Level1>two</Level1></New>");
    }

    // Templates

    #[test]
    fn template_1() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.add_child(r.as_any()).expect("unable to add child");
      r.add_text_child("i1".to_string()).expect("unable to add text");
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      r.add_text_child("i2".to_string()).expect("unable to add text");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      r.add_text_child("i3".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let mut dc = DynamicContext::new(Some(&rd));

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
      dc.add_template(p, cons2);

      let s = Rc::new(Item::Node(Rc::new(r)));
      let u = dc.find_match(&s);
      assert_eq!(u.len(), 1);

      let e = evaluate(&dc, Some(vec![s]), Some(0), &u)
        .expect("evaluation failed");

      assert_eq!(e.len(), 1);
      assert_eq!(e[0].to_string(), "I found a matching template");
    }

    #[test]
    fn template_2() {
      let t: XDMTree = Rc::new(RefCell::new(Graph::new()));
      let d = XDMTreeNode::new(t.clone());
      let r = d.new_element(QualifiedName::new(None, None, "Test".to_string()));
      d.add_child(r.as_any()).expect("unable to add child");
      r.add_text_child("i1".to_string()).expect("unable to add text");
      let l1 = t.new_element("Level1", None).expect("unable to create element");
      r.add_child(l1.as_any()).expect("unable to add child");
      r.add_text_child("i2".to_string()).expect("unable to add text");
      let l2 = t.new_element("Level2", None).expect("unable to create element");
      r.add_child(l2.as_any()).expect("unable to add child");
      r.add_text_child("i3".to_string()).expect("unable to add text");
      let l3 = t.new_element("Level3", None).expect("unable to create element");
      r.add_child(l3.as_any()).expect("unable to add child");
      r.add_text_child("i4".to_string()).expect("unable to add text");

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let mut dc = DynamicContext::new(Some(&rd));

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

      let s = Rc::new(Item::Node(Rc::new(r)));
      let u = dc.find_match(&s);
      assert_eq!(u.len(), 1);

      let e = evaluate(&dc, Some(vec![s]), Some(0), &u)
        .expect("evaluation failed");

      assert_eq!(e.len(), 2);
      assert_eq!(e[0].to_string(), "I found a Level2");
      assert_eq!(e[1].to_string(), "I found a Level3");
    }

    // for-each, for-each-group

    #[test]
    fn foreach_1() {
      let d = from("<Test><Level2></Level2><Level3></Level3></Test>").expect("unable to parse XML");
      let r = d.children().iter().nth(0).unwrap().clone();

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

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));

      let seq = evaluate(&dc, Some(vec![Rc::new(Item::Node(r))]), Some(0), &cons).expect("evaluation failed");

      assert_eq!(seq.len(), 2);
      assert_eq!(seq[0].to_xml(), "<Group>a group</Group>");
      assert_eq!(seq[1].to_xml(), "<Group>a group</Group>");
    }

    #[test]
    fn foreach_2() {
      let d = from("<Test><Level1>one</Level1><Level2>two</Level2><Level3>one</Level3><Level4>two</Level4></Test>").expect("unable to parse XML");
      let r = d.children().iter().nth(0).unwrap().clone();

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

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));

      let seq = evaluate(&dc, Some(vec![Rc::new(Item::Node(r))]), Some(0), &cons).expect("evaluation failed");

      assert_eq!(seq.len(), 2);
      assert_eq!(seq[0].to_xml(), "<Group>a group</Group>");
      assert_eq!(seq[1].to_xml(), "<Group>a group</Group>");
    }

    #[test]
    fn foreach_3() {
      let d = from("<Test><Level1>one</Level1><Level2>one</Level2><Level3>two</Level3><Level4>three</Level4></Test>").expect("unable to parse XML");
      let r = d.children().iter().nth(0).unwrap().clone();

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

      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = DynamicContext::new(Some(&rd));

      let seq = evaluate(&dc, Some(vec![Rc::new(Item::Node(r))]), Some(0), &cons).expect("evaluation failed");

      assert_eq!(seq.len(), 3);
      assert_eq!(seq[0].to_xml(), "<Group>one2</Group>");
      assert_eq!(seq[1].to_xml(), "<Group>two1</Group>");
      assert_eq!(seq[2].to_xml(), "<Group>three1</Group>");
    }

    // XPath tests

    #[test]
    fn xpath_root() {
      let d = from("<Level1><Level2>one</Level2><Level2>two</Level2><Level2>three</Level2></Level1>").expect("unable to parse XML");
      let r = d.children().iter().nth(0).unwrap().clone();

      let dc = DynamicContext::new(None);
      let cons = parse("/").expect("unable to parse XPath \"/\"");

      let seq = evaluate(&dc, Some(vec![Rc::new(Item::Node(r))]), Some(0), &cons).expect("evaluation failed");

      assert_eq!(seq.len(), 1);
      assert_eq!(seq[0].to_xml(), "<Level1><Level2>one</Level2><Level2>two</Level2><Level2>three</Level2></Level1>");
    }

    // XSLT tests

    #[test]
    fn xslt_literal_text() {
      let sc = StaticContext::new_with_xslt_builtins();

      let src = from("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("unable to parse XML");
      let isrc = Rc::new(Item::Document(Rc::new(src.get_doc())));

      let style = from("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'>Found the document</xsl:template>
</xsl:stylesheet>").expect("unable to parse XML");
      let istyle = Rc::new(style.get_doc());

      // Setup dynamic context with result document
      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = from_document(istyle, &rd, &sc).expect("failed to compile stylesheet");

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let t = dc.find_match(&isrc);
      assert!(t.len() >= 1);

      let seq = evaluate(&dc, Some(vec![isrc]), Some(0), &t).expect("evaluation failed");

      assert_eq!(seq.to_string(), "Found the document")
    }

    #[test]
    fn xslt_apply_templates_1() {
      let sc = StaticContext::new_with_xslt_builtins();

      let src = from("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("unable to parse XML");
      let isrc = Rc::new(Item::Document(Rc::new(src.get_doc())));

      let style = from("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::*'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::text()'>found text</xsl:template>
</xsl:stylesheet>").expect("unable to parse XML");
      let istyle = Rc::new(style.get_doc());

      // Setup dynamic context with result document
      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = from_document(istyle, &rd, &sc).expect("failed to compile stylesheet");

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let t = dc.find_match(&isrc);
      assert!(t.len() >= 1);

      let seq = evaluate(&dc, Some(vec![isrc]), Some(0), &t).expect("evaluation failed");

      assert_eq!(seq.to_string(), "found textfound text")
    }

    #[test]
    fn xslt_apply_templates_2() {
      let sc = StaticContext::new_with_xslt_builtins();

      let src = from("<Test>one<Level1/>two<Level1/>three<Level1/>four<Level1/></Test>").expect("unable to parse XML");
      let isrc = Rc::new(Item::Document(Rc::new(src.get_doc())));

      let style = from("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:apply-templates select='child::text()'/></xsl:template>
  <xsl:template match='child::Level1'>found Level1 element</xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>").expect("unable to parse XML");
      let istyle = Rc::new(style.get_doc());

      // Setup dynamic context with result document
      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = from_document(istyle, &rd, &sc).expect("failed to compile stylesheet");

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let t = dc.find_match(&isrc);
      assert!(t.len() >= 1);

      let seq = evaluate(&dc, Some(vec![isrc]), Some(0), &t).expect("evaluation failed");

      assert_eq!(seq.to_string(), "onetwothreefour")
    }

    #[test]
    fn xslt_sequence_1() {
      let sc = StaticContext::new_with_xslt_builtins();

      let src = from("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("unable to parse XML");
      let isrc = Rc::new(Item::Document(Rc::new(src.get_doc())));

      let style = from("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::*'><xsl:sequence select='count(child::*)'/></xsl:template>
</xsl:stylesheet>").expect("unable to parse XML");
      let istyle = Rc::new(style.get_doc());

      // Setup dynamic context with result document
      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = from_document(istyle, &rd, &sc).expect("failed to compile stylesheet");

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let t = dc.find_match(&isrc);
      assert!(t.len() >= 1);

      let seq = evaluate(&dc, Some(vec![isrc]), Some(0), &t).expect("evaluation failed");

      assert_eq!(seq.to_string(), "2")
    }

    #[test]
    fn xslt_sequence_2() {
      let sc = StaticContext::new_with_xslt_builtins();

      let src = from("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("unable to parse XML");
      let isrc = Rc::new(Item::Document(Rc::new(src.get_doc())));

      let style = from("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::*'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>").expect("unable to parse XML");
      let istyle = Rc::new(style.get_doc());

      // Setup dynamic context with result document
      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = from_document(istyle, &rd, &sc).expect("failed to compile stylesheet");

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let t = dc.find_match(&isrc);
      assert!(t.len() >= 1);

      let seq = evaluate(&dc, Some(vec![isrc]), Some(0), &t).expect("evaluation failed");

      assert_eq!(seq.to_string(), "onetwo")
    }

    #[test]
    fn xslt_sequence_3() {
      let sc = StaticContext::new_with_xslt_builtins();

      let src = from("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("unable to parse XML");
      let isrc = Rc::new(Item::Document(Rc::new(src.get_doc())));

      let style = from("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::*'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::text()'>X<xsl:sequence select='.'/>Y</xsl:template>
</xsl:stylesheet>").expect("unable to parse XML");
      let istyle = Rc::new(style.get_doc());

      // Setup dynamic context with result document
      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = from_document(istyle, &rd, &sc).expect("failed to compile stylesheet");

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let t = dc.find_match(&isrc);
      assert!(t.len() >= 1);

      let seq = evaluate(&dc, Some(vec![isrc]), Some(0), &t).expect("evaluation failed");

      assert_eq!(seq.to_string(), "XoneYXtwoY")
    }

    #[test]
    fn xslt_literal_result_element_1() {
      let sc = StaticContext::new_with_xslt_builtins();

      let src = from("<Test><Level1>one</Level1><Level1>two</Level1></Test>").expect("unable to parse XML");
      let isrc = Rc::new(Item::Document(Rc::new(src.get_doc())));

      let style = from("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><MyTest><xsl:apply-templates/></MyTest></xsl:template>
  <xsl:template match='child::Level1'><MyLevel1><xsl:apply-templates/></MyLevel1></xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>").expect("unable to parse XML");
      let istyle = Rc::new(style.get_doc());

      // Setup dynamic context with result document
      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = from_document(istyle, &rd, &sc).expect("failed to compile stylesheet");

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let t = dc.find_match(&isrc);
      assert!(t.len() >= 1);

      let seq = evaluate(&dc, Some(vec![isrc]), Some(0), &t).expect("evaluation failed");

      assert_eq!(seq.to_xml(), "<MyTest><MyLevel1>one</MyLevel1><MyLevel1>two</MyLevel1></MyTest>")
    }

    #[test]
    fn xslt_if_1() {
      let sc = StaticContext::new_with_xslt_builtins();

      let src = from("<Test><Level1>one</Level1><Level1>two</Level1><Level1><text/></Level1></Test>")
        .expect("unable to parse XML");
      let isrc = Rc::new(Item::Document(Rc::new(src.get_doc())));

      let style = from("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Level1'><xsl:if test='child::text()'>has text</xsl:if><xsl:if test='not(child::text())'>no text</xsl:if></xsl:template>
</xsl:stylesheet>").expect("unable to parse XML");
      let istyle = Rc::new(style.get_doc());

      // Setup dynamic context with result document
      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = from_document(istyle, &rd, &sc).expect("failed to compile stylesheet");

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let t = dc.find_match(&isrc);
      assert!(t.len() >= 1);

      let seq = evaluate(&dc, Some(vec![isrc]), Some(0), &t).expect("evaluation failed");

      assert_eq!(seq.to_xml(), "has texthas textno text")
    }

    #[test]
    fn xslt_choose_1() {
      let sc = StaticContext::new_with_xslt_builtins();

      let src = from("<Test><Level1>one</Level1><Level1>two</Level1><Level1><text/></Level1></Test>")
        .expect("unable to parse XML");
      let isrc = Rc::new(Item::Document(Rc::new(src.get_doc())));

      let style = from("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Level1'><xsl:choose><xsl:when test='child::text()'>has text</xsl:when><xsl:otherwise>no text</xsl:otherwise></xsl:choose></xsl:template>
</xsl:stylesheet>").expect("unable to parse XML");
      let istyle = Rc::new(style.get_doc());

      // Setup dynamic context with result document
      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = from_document(istyle, &rd, &sc).expect("failed to compile stylesheet");

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let t = dc.find_match(&isrc);
      assert!(t.len() >= 1);

      let seq = evaluate(&dc, Some(vec![isrc]), Some(0), &t).expect("evaluation failed");

      assert_eq!(seq.to_xml(), "has texthas textno text")
    }

    #[test]
    fn xslt_foreach_1() {
      let sc = StaticContext::new_with_xslt_builtins();

      let src = from("<Test><Level1>one</Level1><Level1>two</Level1></Test>")
        .expect("unable to parse XML");
      let isrc = Rc::new(Item::Document(Rc::new(src.get_doc())));

      let style = from("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:for-each select='child::*'><group><xsl:apply-templates/></group></xsl:for-each></xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>").expect("unable to parse XML");
      let istyle = Rc::new(style.get_doc());

      // Setup dynamic context with result document
      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = from_document(istyle, &rd, &sc).expect("failed to compile stylesheet");

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let t = dc.find_match(&isrc);
      assert!(t.len() >= 1);

      let seq = evaluate(&dc, Some(vec![isrc]), Some(0), &t).expect("evaluation failed");

      assert_eq!(seq.to_xml(), "<group>one</group><group>two</group>")
    }

    #[test]
    fn xslt_foreach_2() {
      let sc = StaticContext::new_with_xslt_builtins();

      let src = from("<Test><Level1>one</Level1><Level2>two</Level2><Level3>one</Level3><Level4>two</Level4></Test>")
        .expect("unable to parse XML");
      let isrc = Rc::new(Item::Document(Rc::new(src.get_doc())));

      let style = from("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:for-each-group select='child::*' group-by='.'><group><xsl:apply-templates/></group></xsl:for-each-group></xsl:template>
  <xsl:template match='child::text()'>a group</xsl:template>
</xsl:stylesheet>").expect("unable to parse XML");
      let istyle = Rc::new(style.get_doc());

      // Setup dynamic context with result document
      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = from_document(istyle, &rd, &sc).expect("failed to compile stylesheet");

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let t = dc.find_match(&isrc);
      assert!(t.len() >= 1);

      let seq = evaluate(&dc, Some(vec![isrc]), Some(0), &t).expect("evaluation failed");

      assert_eq!(seq.to_xml(), "<group>a group</group><group>a group</group>")
    }

    #[test]
    fn xslt_foreach_3() {
      let sc = StaticContext::new_with_xslt_builtins();

      let src = from("<Test><Level1>one</Level1><Level2>two</Level2><Level3>one</Level3><Level4>two</Level4></Test>")
        .expect("unable to parse XML");
      let isrc = Rc::new(Item::Document(Rc::new(src.get_doc())));

      let style = from("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:for-each-group select='child::*' group-by='.'><group><xsl:sequence select='current-grouping-key()'/></group></xsl:for-each-group></xsl:template>
</xsl:stylesheet>").expect("unable to parse XML");
      let istyle = Rc::new(style.get_doc());

      // Setup dynamic context with result document
      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = from_document(istyle, &rd, &sc).expect("failed to compile stylesheet");

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let t = dc.find_match(&isrc);
      assert!(t.len() >= 1);

      let seq = evaluate(&dc, Some(vec![isrc]), Some(0), &t).expect("evaluation failed");

      // NB. the order that the groups appear in is undefined
      assert!(
        seq.to_xml() == "<group>one</group><group>two</group>" ||
      	seq.to_xml() == "<group>two</group><group>one</group>"
      )
    }

    #[test]
    fn xslt_foreach_4() {
      let sc = StaticContext::new_with_xslt_builtins();

      let src = from("<Test><Level1>one</Level1><Level2>two</Level2><Level3>one</Level3><Level4>two</Level4><Level5>one</Level5></Test>")
        .expect("unable to parse XML");
      let isrc = Rc::new(Item::Document(Rc::new(src.get_doc())));

      let style = from("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:for-each-group select='child::*' group-by='.'><group><key><xsl:sequence select='current-grouping-key()'/></key><members><xsl:sequence select='count(current-group())'/></members></group></xsl:for-each-group></xsl:template>
</xsl:stylesheet>").expect("unable to parse XML");
      let istyle = Rc::new(style.get_doc());

      // Setup dynamic context with result document
      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = from_document(istyle, &rd, &sc).expect("failed to compile stylesheet");

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let t = dc.find_match(&isrc);
      assert!(t.len() >= 1);

      let seq = evaluate(&dc, Some(vec![isrc]), Some(0), &t).expect("evaluation failed");

      // NB. the order that the groups appear in is undefined
      assert!(
        seq.to_xml() == "<group><key>one</key><members>3</members></group><group><key>two</key><members>2</members></group>" ||
      	seq.to_xml() == "<group><key>two</key><members>2</members></group><group><key>one</key><members>3</members></group>"
      )
    }

    #[test]
    fn xslt_foreach_adj() {
      let sc = StaticContext::new_with_xslt_builtins();

      let src = from("<Test><Level1>one</Level1><Level2>one</Level2><Level3>two</Level3><Level4>two</Level4><Level5>one</Level5></Test>")
        .expect("unable to parse XML");
      let isrc = Rc::new(Item::Document(Rc::new(src.get_doc())));

      let style = from("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='/'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Test'><xsl:for-each-group select='child::*' group-adjacent='.'><group><key><xsl:sequence select='current-grouping-key()'/></key><members><xsl:sequence select='count(current-group())'/></members></group></xsl:for-each-group></xsl:template>
</xsl:stylesheet>").expect("unable to parse XML");
      let istyle = Rc::new(style.get_doc());

      // Setup dynamic context with result document
      let rd: XDMTree = Rc::new(RefCell::new(Graph::new()));
      XDMTreeNode::new(rd.clone());
      let dc = from_document(istyle, &rd, &sc).expect("failed to compile stylesheet");

      // Prime the stylesheet evaluation by finding the template for the document root
      // and making the document root the initial context
      let t = dc.find_match(&isrc);
      assert!(t.len() >= 1);

      let seq = evaluate(&dc, Some(vec![isrc]), Some(0), &t).expect("evaluation failed");

      // NB. the order that the groups appear in is undefined
      assert!(
        seq.to_xml() == "<group><key>one</key><members>2</members></group><group><key>two</key><members>2</members></group><group><key>one</key><members>1</members></group>" ||
      	seq.to_xml() == "<group><key>two</key><members>2</members></group><group><key>one</key><members>3</members></group>"
      )
    }
}
