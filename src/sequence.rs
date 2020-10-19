//! # xdm::sequence
//!
//! A Sequence is an ordered collection of zero or more Items.
//! Sequences do not nest.

//use std::rc::Rc;

//use crate::xdmerror::{Error, ErrorKind};
use crate::item::*;

//type Sequence = Vec<Rc<Item>>;

pub fn add_item(s: &mut Vec<Item>, i: Item) {
    s.push(i);
}

// TODO: define and access the item-separator parameter
pub fn stringvalue(s: Vec<Item>) -> String {
    // Find the string value of each item in the sequence
    // and concatenate

    s.iter().map(|i| i.stringvalue()).collect()
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
	assert_eq!(stringvalue(s), "1second item")
    }
}

