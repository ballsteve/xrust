//! # xdm::construct
//!
//! A sequence constructor.

//use std::rc::Rc;

use crate::item::*;
use crate::sequence::*;

// Constructor for one item
// The constructor for a sequence is a vector of this struct
pub struct Constructor {
    item: Item, // data for the evaluator
    name: String, // TODO: make this a QName
    nodetest: String, // TODO: make this a QName
    evaluator: fn(&Item, String) -> Vec<Item>, // the function to call at evaluate time to construct the item
}

// Constructor function for a literal item
// data is the static input to the constructor
// name is the node name to use

// This implementation is pretty simple: just return a singleton sequence with the supplied item
fn construct_literal(&data: Item, name: String) -> Vec<Item> {
    let mut s = Vec::new();
    s.push(data.clone());
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_empty_sequence() {
    }

    #[test]
    fn make_singleton_sequence() {
    }

    #[test]
    fn make_multi_sequence() {
    }
}

