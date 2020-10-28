//! # xdm::evaluate
//!
//! Evaluate a sequence constructor.

use crate::xdmerror::*;
use crate::item::*;
use crate::xpath::parse;

pub struct DynamicContext {
  pub context_item: Option<Item>, // in some circumstances there is no context item
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
      seq.push(j.clone()); // pass a reference?
      Ok(seq)
    }
    None => Result::Err(Error{kind: ErrorKind::Unknown, message: format!("no item supplied")}),
  }
}

pub fn cons_context_item(_i: Option<Item>, d: &DynamicContext) -> Result<Vec<Item>, Error> {
  match &d.context_item {
    Some(c) => {
      let mut seq = Vec::new();
      seq.push(c.clone()); // TODO: pass a reference, rather than cloning
      Ok(seq)
    }
    None => Result::Err(Error{kind: ErrorKind::Unknown, message: format!("no context item")}),
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
        context_item: None,
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
    fn eval_context_item() {
      let d = DynamicContext {
        context_item: Some(Item::Value(Value::Integer(123))),
      };
      let seq = cons_context_item(None, &d).expect("unable to construct context_item");
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
        context_item: None,
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

    #[test]
    fn cons_sequence() {
      let d = DynamicContext {
        context_item: Some(Item::Value(Value::Integer(123))),
      };
      let mut c = Vec::new();
      c.push(SequenceConstructor{func: cons_context_item, data: None});
      c.push(SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::Integer(456)))});
      // should result in sequence of length 2
      let s = eval(c, d).expect("failed to evaluate sequence constructor");
      if s.len() == 2 {
        match s[0] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 123),
	  _ => panic!("item is not a literal integer value")
	}
        match s[1] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 456),
	  _ => panic!("item is not a literal integer value")
	}
      } else {
        panic!("not a sequence of two items")
      }
    }

    // Parse then evaluate

    #[test]
    fn empty_sequence() {
      let d = DynamicContext {
        context_item: Some(Item::Value(Value::Integer(123))),
      };
      let s = eval(parse("()").expect("failed to parse expression \"()\""), d).expect("failed to evaluate expression \"()\""); // should result in empty sequence
      assert_eq!(s.len(), 0)
    }

    #[test]
    fn parse_singleton_literal_sequence() {
      let d = DynamicContext {
        context_item: Some(Item::Value(Value::Integer(123))),
      };
      let s = eval(parse("1").expect("failed to parse expression \"1\""), d).expect("failed to evaluate expression \"1\""); // should result in singleton sequence integer item 1
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 1),
	  _ => panic!("item is not a literal integer value")
	}
      } else {
        panic!("not a singleton sequence")
      }
    }

    #[test]
    fn parse_singleton_context_item_sequence() {
      let d = DynamicContext {
        context_item: Some(Item::Value(Value::Integer(123))),
      };
      let s = eval(parse(".").expect("failed to parse expression \".\""), d).expect("failed to evaluate expression \".\""); // should result in singleton sequence integer item 123
      if s.len() == 1 {
        match s[0] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 123),
	  _ => panic!("item is not a literal integer value")
	}
      } else {
        panic!("not a singleton sequence")
      }
    }

    #[test]
    fn parse_literal_sequence() {
      let d = DynamicContext {
        context_item: Some(Item::Value(Value::Integer(123))),
      };
      let s = eval(parse("1, 'abc', 2").expect("failed to parse expression \"1, 'abc', 2\""), d).expect("failed to evaluate expression \"1, 'abc', 2\""); // should result in 3 item sequence
      if s.len() == 3 {
        match s[0] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 1),
	  _ => panic!("item is not a literal integer value")
	}
        match &s[1] {
	  Item::Value(Value::String(v)) => assert_eq!(v, "abc"),
	  _ => panic!("item is not a literal string value")
	}
        match s[2] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 2),
	  _ => panic!("item is not a literal integer value")
	}
      } else {
        panic!("sequence does not have 3 items")
      }
    }

    #[test]
    fn parse_literal_sequence_with_context_item() {
      let d = DynamicContext {
        context_item: Some(Item::Value(Value::Integer(123))),
      };
      let s = eval(parse("'abc', ., 456").expect("failed to parse expression \"'abc', ., 456\""), d).expect("failed to evaluate expression \"'abc', ., 456\""); // should result in the sequence ('abc', 123, 456)
      if s.len() == 3 {
        match s[1] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 123),
	  _ => panic!("item is not a literal integer value")
	}
        match &s[0] {
	  Item::Value(Value::String(v)) => assert_eq!(v, "abc"),
	  _ => panic!("item is not a literal string value")
	}
        match s[2] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 456),
	  _ => panic!("item is not a literal integer value")
	}
      } else {
        panic!("sequence does not have 3 items")
      }
    }

    #[test]
    fn parse_multi_context_item_sequence() {
      let d = DynamicContext {
        context_item: Some(Item::Value(Value::Integer(123))),
      };
      let s = eval(parse(".,.").expect("failed to parse expression \".,.\""), d).expect("failed to evaluate expression \".,.\""); // should result in the sequence (123, 123)
      if s.len() == 2 {
        match s[0] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 123),
	  _ => panic!("item is not a literal integer value")
	}
        match s[1] {
	  Item::Value(Value::Integer(v)) => assert_eq!(v, 123),
	  _ => panic!("item is not a literal integer value")
	}
      } else {
        panic!("sequence does not have 2 items")
      }
    }
}

