//! # xdm::parsexml
//!
//! A parser for XML, as a nom parser combinator.
//! XML 1.1, see https://www.w3.org/TR/xml11/
//!
//! This is a very simple, minimalist parser of XML. It excludes:
//!	XML declaration
//!	DTDs (and therefore entities)
//!	CDATA sections

extern crate nom;
use nom:: {
  IResult,
  sequence::tuple,
  multi::many0,
  combinator::{map, opt},
  bytes::complete::tag,
};
use crate::item::*;
use crate::xdmerror::*;
use crate::parsecommon::*;

// document ::= ( prolog element misc*)
fn document(input: &str) -> IResult<&str, Item> {
  map (
    tuple((
      opt(prolog),
      element,
      many0(misc),
    )),
    |(p, e, m)| {
      let mut d = Node::new(NodeType::Document);
      let mut c: Vec<Item> = Vec::new();
      p.map(|q| for i in q {c.push(i);});
      c.push(e);
      for i in m {
        c.push(i);
      }
      d.set_content(c);
      Item::Node(d)
    }
  )
  (input)
}

// prolog ::= XMLDecl misc* (doctypedecl Misc*)?
fn prolog(input: &str) -> IResult<&str, Vec<Item>> {
  map(
    tag("not yet implemented"),
    |_| {
      vec![Item::Value(Value::String("not yet implemented"))]
    }
  )
  (input)
}

// Element ::= EmptyElemTag | STag content ETag
fn element(input: &str) -> IResult<&str, Item> {
  map(
    tag("not yet implemented"),
    |_| {
      Item::Value(Value::String("not yet implemented"))
    }
  )
  (input)
}

// Misc ::= Comment | PI | S
fn misc(input: &str) -> IResult<&str, Item> {
  map(
    tag("not yet implemented"),
    |_| {
      Item::Value(Value::String("not yet implemented"))
    }
  )
  (input)
}

pub fn parse(e: &str) -> Result<Item, Error> {
  match document(e) {
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
