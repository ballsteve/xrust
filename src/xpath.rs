//! # xdm::xpath
//!
//! An XPath parser as a nom parser combinator.

extern crate nom;
use nom:: {
  IResult,
  character::complete::*,
  branch::alt,
  character::complete::{char, none_of},
  sequence::{delimited, tuple},
  multi::{many0, separated_nonempty_list},
  combinator::map,
  bytes::complete::tag,
};
use crate::item::*;
use crate::xdmerror::*;
use crate::evaluate::{SequenceConstructor, cons_literal, cons_context_item};

// Expr ::= ExprSingle (',' ExprSingle)* ;
fn expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  separated_nonempty_list(
    tuple((multispace0, tag(","), multispace0)),
    primary_expr
  )
  (input)
}

// PrimaryExpr ::= Literal | VarRef | ParenthesizedExpr | ContextItemExpr | FunctionCall | FunctionItemExpr | MapConstructor | ArrayConstructor | UnaryLookup
fn primary_expr(input: &str) -> IResult<&str, SequenceConstructor> {
  alt((
    literal,
    context_item
  ))
  (input)
}

// Literal ::= NumericLiteral | StringLiteral
fn literal(input: &str) -> IResult<&str, SequenceConstructor> {
  alt((
    numeric_literal ,
    string_literal
  ))
  (input)
}

// NumericLiteral ::= IntegerLiteral | DecimalLiteral | DoubleLiteral
// TODO: decimal and double
fn numeric_literal(input: &str) -> IResult<&str, SequenceConstructor> {
  map(digit1, |s: &str| {
    let n = s.parse::<i64>().unwrap();
    SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::Integer(n)))}
  })
  (input)
}

// StringLiteral ::= double- or single-quote delimited with double-delimiter escape
fn string_literal_double(input: &str) -> IResult<&str, String> {
  delimited(
    char('"'),
    map(
      many0(
        alt((
          map(
            tag("\"\""),
            |_| '"'
	  )
	  ,
	  none_of("\"")
        ))
      ),
      |v| v.iter().collect::<String>()
    ),
    char('"')
  )
  (input)
}
fn string_literal_single(input: &str) -> IResult<&str, String> {
  delimited(
    char('\''),
    map(
      many0(
        alt((
          map(
            tag("''"),
            |_| '\''
	  )
          ,
	  none_of("'")
        ))
      ),
      |v| v.iter().collect::<String>()
    ),
    char('\'')
  )
  (input)
}
fn string_literal(input: &str) -> IResult<&str, SequenceConstructor> {
  map(
    alt((
      string_literal_double ,
      string_literal_single
    )),
    |s| SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::String(s)))}
  )
  (input)
}
fn context_item(input: &str) -> IResult<&str, SequenceConstructor> {
  map(
    tag("."),
    |_| SequenceConstructor{func: cons_context_item, data: None}
  )
  (input)
}

pub fn parse(e: &str) -> Result<Vec<SequenceConstructor>, Error> {
  match expr(e) {
    Ok((rest, value)) => {
      if rest == "" {
        Result::Ok(value)
      } else {
        Result::Err(Error{kind: ErrorKind::Unknown, message: String::from(format!("extra characters after expression: \"{}\"", rest))})
      }
    },
    Err(nom::Err::Error(c)) => Result::Err(Error{kind: ErrorKind::Unknown, message: format!("parser error: {:?}", c)}),
    Err(nom::Err::Incomplete(_)) => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("incomplete input")}),
    Err(nom::Err::Failure(_)) => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("unrecoverable parser error")}),
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Parses to a singleton integer sequence constructor
    #[test]
    fn nomxpath_parse_int() {
        let e = parse("1").expect("failed to parse expression \"1\"");
	if e.len() == 1 {
	  match e[0] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::Integer(v)))} => assert_eq!(v, 1),
	    _ => panic!("item is not a literal integer value constructor")
	  }
	} else {
	  panic!("sequence is not a singleton")
	}
    }
    //#[test]
    //fn nomxpath_parse_double() {
        //assert_eq!(parse("2.0").unwrap(), Item::Value(Value::Double(2.0)));
    //}
    // Parses to a singleton string
    #[test]
    fn nomxpath_parse_string_apos() {
        let e = parse("'abc'").expect("failed to parse expression \"'abc'\"");
	if e.len() == 1 {
	  match &e[0] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::String(v)))} => assert_eq!(v, "abc"),
	    _ => panic!("item is not a literal string value")
	  }
	} else {
	  panic!("sequence is not a singleton")
	}
    }
    // Parses to a singleton string
    #[test]
    fn nomxpath_parse_string_apos_esc() {
        let e = parse("'abc''def'").expect("failed to parse expression \"'abc''def'\"");
	if e.len() == 1 {
	  match &e[0] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::String(v)))} => assert_eq!(v, "abc'def"),
	    _ => panic!("item is not a literal string value")
	  }
	} else {
	  panic!("sequence is not a singleton")
	}
    }
    // Parses to a singleton string
    #[test]
    fn nomxpath_parse_string_quot() {
        let e = parse(r#""abc""#).expect("failed to parse expression \"\"abc\"\"");
	if e.len() == 1 {
	  match &e[0] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::String(v)))} => assert_eq!(v, "abc"),
	    _ => panic!("item is not a literal string value")
	  }
	} else {
	  panic!("sequence is not a singleton")
	}
    }
    // Parses to a singleton string
    #[test]
    fn nomxpath_parse_string_quot_esc() {
        let e = parse(r#""abc""def""#).expect("failed to parse expression \"\"abc\"\"def\"\"");
	if e.len() == 1 {
	  match &e[0] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::String(v)))} => assert_eq!(v, r#"abc"def"#),
	    _ => panic!("item is not a literal string value")
	  }
	} else {
	  panic!("sequence is not a singleton")
	}
    }
    #[test]
    fn nomxpath_parse_literal_sequence() {
        let e = parse("1,'abc',2").expect("failed to parse \"1,'abc',2\"");
	if e.len() == 3 {
	  match &e[0] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::Integer(v)))} => assert_eq!(*v, 1),
	    _ => panic!("item 0 is not a literal integer value")
	  }
	  match &e[1] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::String(v)))} => assert_eq!(v, r#"abc"#),
	    _ => panic!("item 1 is not a literal string value")
	  }
	  match &e[2] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::Integer(v)))} => assert_eq!(*v, 2),
	    _ => panic!("item 2 is not a literal integer value")
	  }
	} else {
	  panic!("sequence does not have 3 items")
	}
    }
    #[test]
    fn nomxpath_parse_literal_seq_ws() {
        let e = parse("1 , 'abc', 2").expect("failed to parse \"1 , 'abc', 2\"");
	if e.len() == 3 {
	  match &e[0] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::Integer(v)))} => assert_eq!(*v, 1),
	    _ => panic!("item 0 is not a literal integer value")
	  }
	  match &e[1] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::String(v)))} => assert_eq!(v, r#"abc"#),
	    _ => panic!("item 1 is not a literal string value")
	  }
	  match &e[2] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::Integer(v)))} => assert_eq!(*v, 2),
	    _ => panic!("item 2 is not a literal integer value")
	  }
	} else {
	  panic!("sequence does not have 3 items")
	}
    }

    // Parses to a singleton context item sequence constructor
    #[test]
    fn nomxpath_parse_context_item() {
        let e = parse(".").expect("failed to parse expression \".\"");
	if e.len() == 1 {
	  let s = &e[0].data;
	  match s {
	    None => assert!(true),
	    _ => panic!("item is not a context item constructor")
	  }
	} else {
	  panic!("sequence is not a singleton")
	}
    }

    //#[test]
    //fn nomxpath_parse_empty() {
        //assert_eq!(parse("()").unwrap(), ());
    //}
    //#[test]
    //fn nomxpath_parse_singleton_int() {
        //assert_eq!(parse("(1)").unwrap(), 1);
    //}

}

