//! # xdm::evaluate
//!
//! Evaluate a sequence constructor.

use crate::xdmerror::*;
use crate::item::*;
//use crate::xpath::parse;

pub struct DynamicContext {
  pub current_item: Option<Item>, // in some circumstances there is no current item
}

type SequenceConstructorFunc = fn(Option<Item>, &DynamicContext) -> Result<Vec<Item>, Error>;

pub struct SequenceConstructor {
  pub func: SequenceConstructorFunc,
  pub data: Option<Item>,
}

pub fn cons_literal(i: Option<Item>, _d: &DynamicContext) -> Result<Vec<Item>, Error> {
  match i {
    Some(j) => {
      let mut seq = Vec::new();
      seq.push(j.clone());
      Ok(seq)
    }
    None => Result::Err(Error{kind: ErrorKind::Unknown, message: format!("no item supplied")}),
  }
}

pub fn cons_current_item(_i: Option<Item>, d: &DynamicContext) -> Result<Vec<Item>, Error> {
  match &d.current_item {
    Some(c) => {
      let mut seq = Vec::new();
      seq.push(c.clone());
      Ok(seq)
    }
    None => Result::Err(Error{kind: ErrorKind::Unknown, message: format!("no current item")}),
  }
}

pub fn eval(cons: Vec<SequenceConstructor>, ctxt: DynamicContext) -> Result<Vec<Item>, Error> {
  let mut ret = Vec::new();

  for i in cons {
    let seq = (i.func)(i.data, &ctxt).expect("evaluation failed");
    for j in seq {
      ret.push(j);
    }
  }

  Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Just evaluate

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

    #[test]
    fn eval_current_item() {
      let d = DynamicContext {
        current_item: Some(Item::Value(Value::Integer(123))),
      };
      let seq = cons_current_item(None, &d).expect("unable to construct current_item");
      if seq.len() == 1 {
        match seq[0] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 123),
	  _ => panic!("item is not a literal integer value")
	}
      } else {
        panic!("sequence is not a singleton")
      }
    }

    // Sequence constructor

    #[test]
    fn cons_singleton_literal_sequence() {
      let d = DynamicContext {
        current_item: None,
      };
      let mut c = Vec::new();
      c.push(SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::Integer(1)))});
      // should result in singleton sequence integer item 1
      let s = eval(c, d).expect("failed to evaluate sequence constructor");
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 1),
	  _ => panic!("item is not a literal integer value")
	}
      } else {
        panic!("not a singleton sequence")
      }
    }

    // Parse then evaluate

//    #[test]
//    fn empty_sequence() {
//      let d = DynamicContext {
//        current_item: Some(Item::Value(Value::Integer(123))),
//      }
//      eval(parse("()"), d); // should result in empty sequence
//    }
//
//    #[test]
//    fn singleton_literal_sequence() {
//      let d = DynamicContext {
//        current_item: Some(Item::Value(Value::Integer(123))),
//      }
//      eval(parse("1"), d); // should result in singleton sequence integer item 1
//    }
//
//    #[test]
//    fn literal_sequence() {
//      let d = DynamicContext {
//        current_item: Some(Item::Value(Value::Integer(123))),
//      }
//      eval(parse("1, 'abc', 2"), d); // should result in 3 item sequence
//    }
//
//    #[test]
//    fn literal_sequence() {
//      let d = DynamicContext {
//        current_item: Some(Item::Value(Value::Integer(123))),
//      }
//      eval(parse("."), d); // should result in singleton sequence integer item 123
//    }
//
//    #[test]
//    fn literal_sequence() {
//      let d = DynamicContext {
//        current_item: Some(Item::Value(Value::Integer(123))),
//      }
//      eval(parse("'abc', ., 456"), d); // should result in the sequence ('abc', 123, 456)
//    }
//
//    #[test]
//    fn literal_sequence() {
//      let d = DynamicContext {
//        current_item: Some(Item::Value(Value::Integer(123))),
//      }
//      eval(parse(".,."), d); // should result in the sequence (123, 123)
//    }
}

