//! # xdm
//!
//! A Rust implementation of the XQuery and XPath Data Model 3.1.
//! https://www.w3.org/TR/xpath-datamodel-31/
//!
//! NB. This library is independent of XML and JSON. This is so that
//! documents in either format (or other formats) can be loaded
//! into xdm and operated upon by XPath, XQuery or XSLT.

//! An Item is a Node, Function or Atomic Value.
//! A Sequence is an ordered collection of zero or more Items.
//! Sequences do not nest.

mod xdmerror;
pub use xdmerror::{Error, ErrorKind};

mod item;
pub use item::{Item, Value};

mod sequence;
pub use sequence::{add_item, stringvalue};

mod xpath;
pub use xpath::parse;

mod evaluate;
pub use evaluate::{DynamicContext, cons_literal};

#[cfg(test)]
mod tests {
    use super::*;

    // Create an Item
    #[test]
    fn item_create_string() {
        Item::Value(Value::String(String::from("item")));
	assert!(true);
    }

    // Create a sequence
    #[test]
    fn seq_add_one_item() {
        let mut s: Vec<Item> = Vec::new();
        add_item(&mut s, Item::Value(Value::String(String::from("item"))));
	assert_eq!(stringvalue(s), "item");
    }

    // Create a sequence
    #[test]
    fn seq_add_two_items() {
        let mut s: Vec<Item> = Vec::new();
        add_item(&mut s, Item::Value(Value::String(String::from("item"))));
        add_item(&mut s, Item::Value(Value::Double(1.234)));
	assert_eq!(stringvalue(s), "item1.234");
    }

    // Construct a literal singleton sequence
    #[test]
    fn eval_literal() {
      let d = DynamicContext {
        current_item: None,
      };
      let seq = cons_literal(Some(Item::Value(Value::Integer(456))), &d).expect("unable to construct literal");
      if seq.len() == 1 {
        match seq[0] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 456),
	  _ => panic!("item is not a literal integer value")
	}
      } else {
        panic!("sequence is not a singleton")
      }
    }

    // Parse an XPath
    // TODO: don't have sequences yet
    //#[test]
    //fn xpath_parse_empty() {
        //let e = parse("()").expect("failed to parse expression \"()\"");
    //}
}
