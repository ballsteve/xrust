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
pub use sequence::Sequence;

mod rox_adaptor;

//mod xpath;
//pub use xpath::parse;

//mod evaluate;
//pub use evaluate::{DynamicContext, cons_literal};

#[cfg(test)]
mod tests {
    use super::*;

    // Create an Item
    #[test]
    fn item_create_string() {
        Value::String(String::from("item"));
	assert!(true);
    }

    // Create a sequence
    #[test]
    fn seq_add_one_item() {
        let s: Sequence = vec![Box::new(Value::String(String::from("item")))];
	assert_eq!(s.stringvalue(), "item");
    }

    // Create a sequence
    #[test]
    fn seq_add_two_items() {
        let s: Sequence = vec![Box::new(Value::String(String::from("item"))),
          Box::new(Value::Double(1.234))];
	assert_eq!(s.stringvalue(), "item1.234");
    }

    // Construct a literal singleton sequence
    //#[test]
    //fn eval_literal() {
      //let d = DynamicContext {
        //context_item: None,
      //};
      //let seq = cons_literal(&d, None, Some(Item::Value(Value::Integer(456)))).expect("unable to construct literal");
      //if seq.len() == 1 {
        //match seq[0] {
	  //Item::Value(Value::Integer(v)) => assert_eq!(v, 456),
	  //_ => panic!("item is not a literal integer value")
	//}
      //} else {
        //panic!("sequence is not a singleton")
      //}
    //}
}
