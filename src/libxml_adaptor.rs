//! # libxml adaptor for xrust
//!
//! Implement the TDocument, TNode trait for libxml

use crate::item::*;
use libxml::*;

impl TDocument for libxml::Document {
  pub fn new() -> Self {
    libxml::Document::new().expect("unable to create libxml document")
  }
  /// Get the string value of the document
  /// TODO: check that 'node_to_string' computes this correctly
  pub fn to_string(&self) -> String {
    match self.get_root_element() {
      Some(n) => {
        self.node_to_string(n)
      }
      None => "".to_string()
    }
  }
  /// libxml only provides access to the root element, not the prologue or epilogue
  pub fn get_children(&self) -> Vec<TNode> {
    vec![self.get_root_element().unwrap()]
  }
}

impl TNode for libxml::Node {
  pub fn new(d: &Document) -> Self {
    libxml::Node::new(name: "", None, d)
  }
  pub fn get_name(&self) -> String {
    self.get_name()
  }
  pub fn get_value(&self) -> String {
    self.get_content()
  }
  /// TODO: check if this returns the string value
  pub fn to_string(&self) -> String {
    self.get_content()
  }
  pub fn get_document(&self) -> TDocument {
  }
  pub fn get_parent(&self) -> Self {
    self.get_parent().unwrap()
  }
  pub fn get_children(&self) -> Vec<Self> {
    self.get_child_nodes()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn doc_item() {
    let i = Item::TDocument(libxml::parse_string("<Test>a test document</Test>").expect("failed to parse XML"));
    assert_eq!(i.to_string(), "a test document")
  }
}
