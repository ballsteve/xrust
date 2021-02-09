//! # roxmltree adpator
//!
//! Uses roxmltree in XPath

use crate::xdmerror::*;
use roxmltree::{Node, Document};
use crate::item::Item;

impl<'a> Item<'a> for Document<'a> {
  fn stringvalue(&self) -> String {
    // TODO: this is incorrect for element nodes
    // need to find all text node descendants
    let t = self.root_element().first_child().expect("unable to find first child element").text();
    match t {
      Some(s) => String::from(s),
      None => String::from("")
    }
  }

  fn to_bool(&self) -> bool {
    // TODO
    false
  }

  fn doc(&self) -> Result<Box<dyn Item<'a> + 'a>, Error> {
    Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("document cannot refer to itself")})
  }
  fn parent(&self) -> Result<Box<dyn Item<'a> + 'a>, Error> {
    Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("document doesn't have a parent")})
  }
  fn children(&self) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
    // TODO: getting error "cannot infer an appropriate lifetime for autoref due to conflicting requirements"
    //let t: Node = self.root_element().first_child().expect("unable to find first child element");
    //Ok(vec![Box::new(t)])
    Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not yet implemented")})
  }
}

impl<'a, 'b: 'a> Item<'a> for Node<'a, 'b> {
  fn stringvalue(&self) -> String {
    // TODO: this is incorrect for element nodes
    // need to find all text node descendants
    match self.text() {
      Some(s) => String::from(s),
      None => String::from("")
    }
  }

  fn to_bool(&self) -> bool {
    // TODO
    false
  }

  fn doc(&self) -> Result<Box<dyn Item<'a> + 'a>, Error> {
    // TODO: fix error - "the trait bound `&roxmltree::Document<'_>: item::Item` is not satisfied"
    //Ok(Box::new(self.document()))
    Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not yet implemented")})
  }
  fn parent(&self) -> Result<Box<dyn Item<'a> + 'a>, Error> {
    match self.parent() {
      Some(s) => Ok(Box::new(s)),
      None => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("unable to get parent")})
    }
  }
  fn children(&self) -> Result<Vec<Box<dyn Item<'a> + 'a>>, Error> {
    //Ok(self.children().map(|n| Box::new(n)).collect())
    let mut r: Vec<Box<dyn Item<'a> + 'a>> = Vec::new();
    for c in self.children() {
      r.push(Box::new(c))
    }
    Ok(r)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rox_doc_stringvalue() {
    let doc = roxmltree::Document::parse("<test>foobar</test>").expect("unable to parse XML");
    let n = doc.root_element().first_child().expect("unable to find first child element");
    assert_eq!(n.stringvalue(), "foobar")
  }

  #[test]
  fn rox_node_stringvalue() {
    let doc = roxmltree::Document::parse("<test><one>foobar</one><two>barfoo</two></test>").expect("unable to parse XML");
    let n = doc.root_element().first_child().expect("unable to find first child element");
    assert_eq!(n.stringvalue(), "foobar")
  }
}


