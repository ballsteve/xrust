//! # xdm::sequence
//!
//! A Sequence is an ordered collection of zero or more Items.
//! Sequences do not nest.

use crate::xdmerror::*;
use crate::item::*;

pub type Sequence<'a> = Vec<Item<'a>>;

impl<'a> Sequence<'a> {
    fn to_string(&self) -> String {
        // Find the string value of each item in the sequence
        // and concatenate
        // TODO: define and access the item-separator parameter
        self.iter().map(|i| i.to_string()).collect()
    }
    fn to_bool(&self) -> bool {
      if self.len() == 0 {
        false
      } else {
        self[0].to_bool()
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
        let s: Sequence = vec![Item::Value(Value::Int(1))];
	assert_eq!(s.len(), 1)
    }

    #[test]
    fn seq_stringvalue() {
        let s: Sequence = vec![Item::Value(Value::Int(1)),
	  Item::Value(Value::String(String::from("second item")))];
	assert_eq!(s.to_string(), "1second item")
    }

    #[test]
    fn ebv_empty() {
        let s: Sequence = Vec::new();
	assert_eq!(s.to_bool(), false)
    }

    #[test]
    fn ebv_int_0() {
        let s: Sequence = vec![Item::Value(Value::Integer(0))];
	assert_eq!(s.to_bool(), false)
    }
    #[test]
    fn ebv_int_1() {
        let s: Sequence = vec![Item::Value(Value::Integer(10))];
	assert_eq!(s.to_bool(), true)
    }
    #[test]
    fn ebv_double_0() {
        let s: Sequence = vec![Item::Value(Value::Double(0.0))];
	assert_eq!(s.to_bool(), false)
    }
    #[test]
    fn ebv_double_1() {
        let s: Sequence = vec![Item::Value(Value::Double(0.01))];
	assert_eq!(s.to_bool(), true)
    }
    #[test]
    fn ebv_string_0() {
        let s: Sequence = vec![Item::Value(Value::String(String::from("")))];
	assert_eq!(s.to_bool(), false)
    }
    #[test]
    fn ebv_string_1() {
        let s: Sequence = vec![Item::Value(Value::String(String::from("false")))];
	assert_eq!(s.to_bool(), true)
    }
}

