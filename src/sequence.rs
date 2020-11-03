//! # xdm::sequence
//!
//! A Sequence is an ordered collection of zero or more Items.
//! Sequences do not nest.

//use crate::xdmerror::{Error, ErrorKind};
use crate::item::*;

pub fn add_item(s: &mut Vec<Item>, i: Item) {
    s.push(i);
}

// TODO: define and access the item-separator parameter
pub fn stringvalue(s: &Vec<Item>) -> String {
    // Find the string value of each item in the sequence
    // and concatenate

    s.iter().map(|i| i.stringvalue()).collect()
}

// Determine the effective boolean value of a sequence.
// See XPath 2.4.3.
pub fn effective_boolean_value(s: Vec<Item>) -> bool {
  if s.len() == 0 {
    false
  } else {
    // If the first item is a node then true
    match &s[0] {
      Item::Value(Value::Boolean(b)) => *b == true,
      Item::Value(Value::String(t)) => {
        //t.is_empty()
	t.as_str().len() != 0
      },
      Item::Value(Value::Double(n)) => *n != 0.0,
      Item::Value(Value::Integer(i)) => *i != 0,
      _ => false
    }
  }
}

// TODO: atomization is low priority for now
// Atomize each value in a sequence.
// Returns a new sequence with the atomized values.
// ? Should this modify the original sequence?
//pub fn atomize(s: Vec<Item>) -> Vec<Item> {
    // TODO: use an atomize method for the item
    //s.iter().map(|i| i.clone()).collect()

//    let mut r = Vec::new();
//
//    for i in &s {
//        r.push(i.clone());
//    }
//
//    r
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seq_add_item_new() {
        let mut s: Vec<Item> = Vec::new();
	add_item(&mut s, Item::Value(Value::Int(1)));
	assert_eq!(s.len(), 1)
    }

    #[test]
    fn seq_stringvalue() {
        let mut s: Vec<Item> = Vec::new();
	add_item(&mut s, Item::Value(Value::Int(1)));
	add_item(&mut s, Item::Value(Value::String(String::from("second item"))));
	assert_eq!(stringvalue(&s), "1second item")
    }

    #[test]
    fn ebv_empty() {
        let s: Vec<Item> = Vec::new();
	assert_eq!(effective_boolean_value(s), false)
    }

    #[test]
    fn ebv_int_0() {
        let s: Vec<Item> = vec![Item::Value(Value::Integer(0))];
	assert_eq!(effective_boolean_value(s), false)
    }
    #[test]
    fn ebv_int_1() {
        let s: Vec<Item> = vec![Item::Value(Value::Integer(10))];
	assert_eq!(effective_boolean_value(s), true)
    }
    #[test]
    fn ebv_double_0() {
        let s: Vec<Item> = vec![Item::Value(Value::Double(0.0))];
	assert_eq!(effective_boolean_value(s), false)
    }
    #[test]
    fn ebv_double_1() {
        let s: Vec<Item> = vec![Item::Value(Value::Double(0.01))];
	assert_eq!(effective_boolean_value(s), true)
    }
    #[test]
    fn ebv_string_0() {
        let s: Vec<Item> = vec![Item::Value(Value::String(String::from("")))];
	assert_eq!(effective_boolean_value(s), false)
    }
    #[test]
    fn ebv_string_1() {
        let s: Vec<Item> = vec![Item::Value(Value::String(String::from("false")))];
	assert_eq!(effective_boolean_value(s), true)
    }
}

