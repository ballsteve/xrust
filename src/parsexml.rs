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
  branch::alt,
  character::complete::{char, multispace0, multispace1, none_of,},
  sequence::tuple,
  multi::{many0, many1},
  combinator::{map, opt},
  bytes::complete::{tag, take_until},
  sequence::delimited,
};
use crate::item::*;
use crate::xdmerror::*;
use crate::parsecommon::*;

// document ::= ( prolog element misc*)
fn document(input: &str) -> IResult<&str, Node> {
  map (
    tuple((
      opt(prolog),
      element,
      many0(misc),
    )),
    |(p, e, m)| {
      let d = Node::new(NodeType::Document);
      let mut c: Vec<Node> = Vec::new();
      p.map(|q| for i in q {c.push(i);});
      c.push(e);
      for i in m {
        for j in i {
          c.push(j);
	}
      }
      d.set_content(c)
    }
  )
  (input)
}

// prolog ::= XMLDecl misc* (doctypedecl Misc*)?
fn prolog(input: &str) -> IResult<&str, Vec<Node>> {
  map(
    tag("not yet implemented"),
    |_| {
      vec![Node::new(NodeType::ProcessingInstruction).set_name("xml".to_string()).set_value("not yet implemented".to_string())]
    }
  )
  (input)
}

// Element ::= EmptyElemTag | STag content ETag
fn element(input: &str) -> IResult<&str, Node> {
  map(
    alt((
      emptyelem,
      taggedelem,
    )),
    |e| {
      e
    }
  )
  (input)
}

// STag ::= '<' Name (Attribute)* '>'
// ETag ::= '</' Name '>'
// NB. Names must match
fn taggedelem(input: &str) -> IResult<&str, Node> {
  map(
    tuple((
      tag("<"),
      multispace0,
      name,
      many0(attribute),
      multispace0,
      tag(">"),
      content,
      tag("</"),
      multispace0,
      name,
      multispace0,
      tag(">"),
    )),
    |(_, _, n, a, _, _, c, _, _, _e, _, _)| {
      // TODO: check that the start tag name and end tag name match (n == e)
      Node::new(NodeType::Element).set_name(n.to_string()).set_attributes(a).set_content(c)
    }
  )
  (input)
}

// EmptyElemTag ::= '<' Name (Attribute)* '/>'
fn emptyelem(input: &str) -> IResult<&str, Node> {
  map(
    tuple((
      tag("<"),
      multispace0,
      name,
      many0(attribute),
      multispace0,
      tag("/>"),
    )),
    |(_, _, n, a, _, _)| {
      Node::new(NodeType::Element).set_name(n.to_string()).set_attributes(a)
    }
  )
  (input)
}

// Attribute ::= Name '=' AttValue
fn attribute(input: &str) -> IResult<&str, Node> {
  map(
    tuple((
      multispace1,
      name,
      multispace0,
      tag("="),
      multispace0,
      delimited_string,
    )),
    |(_, n, _, _, _, s)| {
      Node::new(NodeType::Attribute).set_name(n.to_string()).set_value(s)
    }
  )
  (input)
}
fn delimited_string(input: &str) -> IResult<&str, String> {
  alt((
    string_single,
    string_double,
  ))
  (input)
}
fn string_single(input: &str) -> IResult<&str, String> {
  delimited(
    char('\''),
    map(
      many0(none_of("'")),
      |v| v.iter().collect::<String>()
    ),
    char('\''),
  )
  (input)
}
fn string_double(input: &str) -> IResult<&str, String> {
  delimited(
    char('"'),
    map(
      many0(none_of("\"")),
      |v| v.iter().collect::<String>()
    ),
    char('"'),
  )
  (input)
}

// content ::= CharData? ((element | Reference | CDSect | PI | Comment) CharData?)*
fn content(input: &str) -> IResult<&str, Vec<Node>> {
  map(
    tuple((
      opt(chardata),
      many0(
        alt((
          element,
	  reference,
	  // TODO: CData Section
	  processing_instruction,
	  comment,
        )),
      ),
      opt(chardata),
    )),
    |(c, v, d)| {
      let mut new: Vec<Node> = Vec::new();
      if c.is_some() {
        new.push(Node::new(NodeType::Text).set_value(c.unwrap()));
      }
      if v.len() != 0 {
        for w in v {
          new.push(w);
	}
      }
      if d.is_some() {
        new.push(Node::new(NodeType::Text).set_value(d.unwrap()));
      }
      new
    }
  )
  (input)
}

// Reference ::= EntityRef | CharRef
// TODO
fn reference(input: &str) -> IResult<&str, Node> {
  map(
    tag("not yet implemented"),
    |_| {
      Node::new(NodeType::Text).set_value("not yet implemented".to_string())
    }
  )
  (input)
}

// PI ::= '<?' PITarget (char* - '?>') '?>'
fn processing_instruction(input: &str) -> IResult<&str, Node> {
  map(
    delimited(
      tag("<?"),
      tuple((
        multispace0,
	name,
	multispace0,
	take_until("?>"),
      )),
      tag("?>"),
    ),
    |(_, n, _, v)| {
      Node::new(NodeType::ProcessingInstruction).set_name(n.to_string()).set_value(v.to_string())
    }
  )
  (input)
}

// Comment ::= '<!--' (char* - '--') '-->'
fn comment(input: &str) -> IResult<&str, Node> {
  map(
    delimited(
      tag("<!--"),
      take_until("--"),
      tag("-->"),
    ),
    |v: &str| {
      Node::new(NodeType::Comment).set_value(v.to_string())
    }
  )
  (input)
}

// Misc ::= Comment | PI | S
fn misc(input: &str) -> IResult<&str, Vec<Node>> {
  map(
    tag("not yet implemented"),
    |_| {
      vec![Node::new(NodeType::Comment).set_value("not yet implemented".to_string())]
    }
  )
  (input)
}

// CharData ::= [^<&]* - (']]>')
fn chardata(input: &str) -> IResult<&str, String> {
  map(
    many1(none_of("<&")),
    |v| {
      v.iter().collect::<String>()
    }
  )
  (input)
}

pub fn parse(e: &str) -> Result<Node, Error> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let doc = parse("<Test/>").expect("failed to parse XML \"<Test/>\"");
	match doc.nodetype() {
	  NodeType::Document => {
	    if doc.content().unwrap().len() == 1 {
	      let c = doc.content().unwrap();
	      match c[0].nodetype() {
		NodeType::Element => {
		  assert_eq!(c[0].name(), "Test")
		}
		_ => panic!("document does not contain root element")
	      }
	    } else {
	      panic!("document does not have single element content")
	    }
	  }
	  _ => {
	    panic!("not a Document node")
	  }
	}
    }

    #[test]
    fn root_element() {
        let doc = parse("<Test></Test>").expect("failed to parse XML \"<Test></Test>\"");
	match doc.nodetype() {
	  NodeType::Document => {
	    if doc.content().unwrap().len() == 1 {
	      let c = doc.content().unwrap();
	      match c[0].nodetype() {
		NodeType::Element => {
		  assert_eq!(c[0].name(), "Test")
		}
		_ => panic!("document does not contain root element")
	      }
	    } else {
	      panic!("document does not have single element content")
	    }
	  }
	  _ => {
	    panic!("not a Document node")
	  }
	}
    }

    #[test]
    fn root_element_text() {
        let doc = parse("<Test>Foobar</Test>").expect("failed to parse XML \"<Test>Foobar</Test>\"");
	match doc.nodetype() {
	  NodeType::Document => {
	    if doc.content().unwrap().len() == 1 {
	      let c = doc.content().unwrap();
	      match c[0].nodetype() {
		NodeType::Element => {
		  assert_eq!(c[0].name(), "Test");
		  let t = c[0].content().unwrap();
		  if t.len() == 1 {
		    match t[0].nodetype() {
		      NodeType::Text => {
		        assert_eq!(t[0].value(), "Foobar")
		      }
		      _ => {
		        panic!("root element does not contain a text child node")
		      }
		    }
		  } else {
		    panic!("root element does not contain exactly 1 child node")
		  }
		}
		_ => panic!("document does not contain root element")
	      }
	    } else {
	      panic!("document does not have single element content")
	    }
	  }
	  _ => {
	    panic!("not a Document node")
	  }
	}
    }

    #[test]
    fn nested() {
        let doc = parse("<Test><Foo>bar</Foo></Test>").expect("failed to parse XML \"<Test><Foo>bar</Foo></Test>\"");
	match doc.nodetype() {
	  NodeType::Document => {
	    if doc.content().unwrap().len() == 1 {
	      let c = doc.content().unwrap();
	      match c[0].nodetype() {
		NodeType::Element => {
		  assert_eq!(c[0].name(), "Test");
		  let t = c[0].content().unwrap();
		  if t.len() == 1 {
		    match t[0].nodetype() {
		      NodeType::Element => {
		        assert_eq!(t[0].name(), "Foo");
			let u = t[0].content().unwrap();
			if u.len() == 1 {
		    	  match u[0].nodetype() {
		      	    NodeType::Text => {
		              assert_eq!(u[0].value(), "bar");
			    }
			    _ => {
			      panic!("nested element text child does not have correct value")
			    }
			  }
			} else {
		    	  panic!("nested element does not contain exactly 1 child node")
			}
		      }
		      _ => {
		        panic!("root element does not contain an element child")
		      }
		    }
		  } else {
		    panic!("root element does not contain exactly 1 child node")
		  }
		}
		_ => panic!("document does not contain root element")
	      }
	    } else {
	      panic!("document does not have single element content")
	    }
	  }
	  _ => {
	    panic!("not a Document node")
	  }
	}
    }
}
