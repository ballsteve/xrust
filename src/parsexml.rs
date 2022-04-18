//! # xdm::parsexml
//!
//! A parser for XML, as a nom parser combinator.
//! XML 1.1, see https://www.w3.org/TR/xml11/
//!
//! This is a very simple, minimalist parser of XML. It excludes:
//!	DTDs (and therefore entities)


extern crate nom;

use std::collections::HashSet;
use std::str::FromStr;
use nom:: {
  IResult,
  branch::alt,
  character::complete::{char, multispace0, multispace1, none_of,},
  sequence::tuple,
  multi::{many0, many1},
  combinator::{map, map_opt, opt, value,verify},
  bytes::complete::{tag, take_until, take_while_m_n},
  sequence::delimited,
};
use crate::qname::*;
use crate::item::*;
use crate::parsecommon::*;
use crate::xdmerror::*;

// nom doesn't pass additional parameters, only the input,
// so this is a two-pass process.
// First, use nom to tokenize and parse the input.
// Second, use the internal structure returned by the parser
// to build the document structure.

// This structure allows multiple root elements.
// An XML document will only be well-formed if there is exactly one element.
// However, external general entities may have more than one element.
#[derive(PartialEq)]
pub struct XMLDocument {
  pub prologue: Vec<XMLNode>,
  pub content: Vec<XMLNode>,
  pub epilogue: Vec<XMLNode>,
  pub xmldecl: Option<XMLdecl>
}

#[derive(Clone, PartialEq)]
pub enum XMLNode {
  Element(QualifiedName, Vec<XMLNode>, Vec<XMLNode>), // Element name, attributes, content
  Attribute(QualifiedName, Value),
  Text(Value),
  PI(String, Value),
  Comment(Value), // Comment value is a string
}

#[derive(PartialEq)]
pub struct XMLdecl {
    version: String,
    encoding: Option<String>,
    standalone: Option<String>
}

// document ::= ( prolog element misc*)
fn document(input: &str) -> IResult<&str, XMLDocument> {
  map (
    tuple((
      opt(prolog),
      element,
      opt(misc),
    )),
    |(p, e, m)| {
      let pr = p.unwrap_or((None, vec![]));

      XMLDocument {
	    content: vec![e],
	    epilogue: m.unwrap_or(vec![]),
        xmldecl: pr.0,
        prologue: pr.1
      }
    }
  )
  (input)
}

// prolog ::= XMLDecl misc* (doctypedecl Misc*)?
fn prolog(input: &str) -> IResult<&str, (Option<XMLdecl>, Vec<XMLNode>)> {
    map(
        tuple((
            opt(xmldecl),
            opt(doctypedecl)
            )),
        |(x, _dtd)| (x, vec![])
    )(input)
}

fn xmldecl(input: &str) -> IResult<&str, XMLdecl> {
    map(
        tuple((
            tag("<?xml"),
            multispace0,
            map(
                tuple((
                    tag("version"),
                    multispace0,
                    tag("="),
                    multispace0,
                    delimited_string
                )), | (_,_,_,_,v) | v
            ),
            multispace0,
            opt(
                map(
                tuple((
                    tag("encoding"),
                    multispace0,
                    tag("="),
                    multispace0,
                    delimited_string
                )), | (_,_,_,_,e) | e
            )),
            multispace0,
            opt(
                map(
                tuple((
                    tag("standalone"),
                    multispace0,
                    tag("="),
                    multispace0,
                    delimited_string
                )), | (_,_,_,_,s) | s
            )),
            multispace0,
            tag("?>")
        )),
        |(_,_, ver, _, enc,_,sta,_,_)| {
            XMLdecl{
                version: ver,
                encoding: enc,
                standalone: sta
            }
        }
    )(input)
}

fn doctypedecl(input: &str) -> IResult<&str, Vec<XMLNode>> {
    map(
        tag("not yet implemented"),
        |_| {
            vec![]
        }
    )
        (input)
}


// Element ::= EmptyElemTag | STag content ETag
fn element(input: &str) -> IResult<&str, XMLNode> {
  map(
    alt((
      emptyelem,
      taggedelem,
    )),
    |e| {
      // TODO: Check for namespace declarations, and resolve URIs in the node tree under 'e'
      e
    }
  )
  (input)
}

// STag ::= '<' Name (Attribute)* '>'
// ETag ::= '</' Name '>'
// NB. Names must match
fn taggedelem(input: &str) -> IResult<&str, XMLNode> {
  map(
    tuple((
      tag("<"),
      qualname,
      attributes, //many0(attribute),
      multispace0,
      tag(">"),
      content,
      tag("</"),
      qualname,
      multispace0,
      tag(">"),
    )),
    |(_, n, a, _, _, c, _, _e, _, _)| {
      // TODO: check that the start tag name and end tag name match (n == e)
      XMLNode::Element(n, a, c)
    }
  )
  (input)
}

// EmptyElemTag ::= '<' Name (Attribute)* '/>'
fn emptyelem(input: &str) -> IResult<&str, XMLNode> {
  map(
    tuple((
      tag("<"),
      qualname,
      attributes, //many0(attribute),
      multispace0,
      tag("/>"),
    )),
    |(_, n, a, _, _)| {
      XMLNode::Element(n, a, vec![])
    }
  )
  (input)
}

fn attributes(input: &str) -> IResult<&str, Vec<XMLNode>> {
    //this is just a wrapper around the attribute function, that checks for duplicates.
    verify(many0(attribute),
           |v: &[XMLNode]|
               {
                   let attrs = v.clone();
                   let uniqueattrs: HashSet<_> = attrs.iter()
                       .map(
                           |xmlnode|
                               match xmlnode {
                                   XMLNode::Attribute(q, _) => {q.to_string()}
                                   _ => "".to_string()
                               }
                       )
                       .collect();
                   if &v.len() == &uniqueattrs.len(){
                       true
                   } else {
                       false
                   }
               }
    )(input)
}

// Attribute ::= Name '=' AttValue
fn attribute(input: &str) -> IResult<&str, XMLNode> {
  map(
    tuple((
      multispace1,
      qualname,
      multispace0,
      tag("="),
      multispace0,
      delimited_string,
    )),
    |(_, n, _, _, _, s)| {
      XMLNode::Attribute(n, Value::String(s))
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
fn content(input: &str) -> IResult<&str, Vec<XMLNode>> {
  map(
    tuple((
      opt(chardata),
      many0(
        tuple((
	  alt((
            element,
	    reference,
	    // TODO: CData Section
	    processing_instruction,
	    comment,
          )),
      	  opt(chardata),
	))
      ),
    )),
    |(c, v)| {
      let mut new: Vec<XMLNode> = Vec::new();
      if c.is_some() {
        new.push(XMLNode::Text(Value::String(c.unwrap())));
      }
      if v.len() != 0 {
        for (w, d) in v {
          new.push(w);
      	  if d.is_some() {
            new.push(XMLNode::Text(Value::String(d.unwrap())));
      	  }
	}
      }
      new
    }
  )
  (input)
}

// Reference ::= EntityRef | CharRef
// TODO
fn reference(input: &str) -> IResult<&str, XMLNode> {
  map(
    tag("not yet implemented"),
    |_| {
      XMLNode::Text(Value::String("not yet implemented".to_string()))
    }
  )
  (input)
}

// PI ::= '<?' PITarget (char* - '?>') '?>'
fn processing_instruction(input: &str) -> IResult<&str, XMLNode> {
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
      XMLNode::PI(String::from(n), Value::String(v.to_string()))
    }
  )
  (input)
}

// Comment ::= '<!--' (char* - '--') '-->'
fn comment(input: &str) -> IResult<&str, XMLNode> {
  map(
    delimited(
      tag("<!--"),
      take_until("--"),
      tag("-->"),
    ),
    |v: &str| {
      XMLNode::Comment(Value::String(v.to_string()))
    }
  )
  (input)
}

// Misc ::= Comment | PI | S
fn misc(input: &str) -> IResult<&str, Vec<XMLNode>> {
  map(
    tag("not yet implemented"),
    |_| {
      //vec![Node::new(NodeType::Comment).set_value("not yet implemented".to_string())]
      vec![]
    }
  )
  (input)
}

// CharData ::= [^<&]* - (']]>')
fn chardata(input: &str) -> IResult<&str, String> {
    map(
        many1(
            alt((
                chardata_cdata,
                chardata_escapes,
                chardata_literal
            ))
        ),
        |v| {
                v.join("")
              }
        )(input)
}

fn chardata_cdata(input: &str) -> IResult<&str, String> {
    map(
    delimited(
        tag("<![CDATA["),take_until("]]>"),tag("]]>")
        ),
        |cd: &str| cd.to_string()
    )(input)
}

fn chardata_escapes(input: &str) -> IResult<&str, String> {
    alt((
        chardata_unicode_codepoint,
        value(">".to_string(),tag("&gt;")),
        value("<".to_string(),tag("&lt;")),
        value("&".to_string(),tag("&amp;")),
        value("\"".to_string(),tag("&quot;")),
        value("\'".to_string(),tag("&apos;")),
    ))(input)
}

fn chardata_unicode_codepoint(input: &str) -> IResult<&str, String> {
    let parse_hex =
        map (
        take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit()),
            |hex| {
                u32::from_str_radix(hex, 16)
            }
        );

    let parse_decimal =
        map (
            take_while_m_n(1, 6, |c: char| c.is_ascii_digit()),
            |dec| {
                u32::from_str(dec)
            }
        );

    map_opt(
    alt((
        delimited(tag("&#x"),parse_hex,tag(";")),
        delimited(tag("&#"),parse_decimal,tag(";")),
        )),
    |value| Option::from(std::char::from_u32(value.unwrap()).unwrap().to_string())
    )(input)
}

fn chardata_literal(input: &str) -> IResult<&str, String> {

    map(
    verify(many1(none_of("<&")),
           |v: &[char]|
               {
                   // chardata cannot contain ]]>
                   let cd_end = &[']',']','>'][..];
                   let mut w = v.clone();
                   while !w.is_empty() {
                       if w.starts_with(cd_end) { return false; }
                       if !is_char(&w[0]) {return false;}
                       w = &w[1..];
                   }
                   true
               }
    ),
        |c| c.iter().collect::<String>()
    )(input)
}


// QualifiedName
fn qualname(input: &str) -> IResult<&str, QualifiedName> {
  alt((
    prefixed_name,
    unprefixed_name,
  ))
  (input)
}
fn unprefixed_name(input: &str) -> IResult<&str, QualifiedName> {
  map (
    ncname,
    |localpart| {
      QualifiedName::new(None, None, String::from(localpart))
    }
  )
  (input)
}
fn prefixed_name(input: &str) -> IResult<&str, QualifiedName> {
  map (
    tuple((
      ncname,
      tag(":"),
      ncname
    )),
    |(prefix, _, localpart)| {
      QualifiedName::new(None, Some(String::from(prefix)), String::from(localpart))
    }
  )
  (input)
}

pub fn parse(e: &str) -> Result<XMLDocument, Error> {
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
        assert_eq!(doc.prologue.len(), 0);
        assert_eq!(doc.epilogue.len(), 0);
        assert_eq!(doc.content.len(), 1);
        match &doc.content[0] {
            XMLNode::Element(n, a, c) => {
                assert_eq!(n.get_localname(), "Test");
                assert_eq!(a.len(), 0);
                assert_eq!(c.len(), 0);
            }
            _ => {
                panic!("root is not an element node")
            }
        }
    }

    #[test]
    fn root_element() {
        let doc = parse("<Test></Test>").expect("failed to parse XML \"<Test></Test>\"");
        assert_eq!(doc.prologue.len(), 0);
        assert_eq!(doc.epilogue.len(), 0);
        assert_eq!(doc.content.len(), 1);
        match &doc.content[0] {
            XMLNode::Element(n, a, c) => {
                assert_eq!(n.get_localname(), "Test");
                assert_eq!(a.len(), 0);
                assert_eq!(c.len(), 0);
            }
            _ => {
                panic!("root is not an element node")
            }
        }
    }

    #[test]
    fn root_element_text() {
        let doc = parse("<Test>Foobar</Test>").expect("failed to parse XML \"<Test>Foobar</Test>\"");
        assert_eq!(doc.prologue.len(), 0);
        assert_eq!(doc.epilogue.len(), 0);
        assert_eq!(doc.content.len(), 1);
        match &doc.content[0] {
            XMLNode::Element(n, a, c) => {
                assert_eq!(n.get_localname(), "Test");
                assert_eq!(a.len(), 0);
                assert_eq!(c.len(), 1);
                match &c[0] {
                    XMLNode::Text(v) => {
                        assert_eq!(v.to_string(), "Foobar")
                    }
                    _ => panic!("root element content is not text"),
                }
            }
            _ => {
                panic!("root is not an element node")
            }
        }
    }

    #[test]
    fn nested() {
        let doc = parse("<Test><Foo>bar</Foo></Test>").expect("failed to parse XML \"<Test><Foo>bar</Foo></Test>\"");
        assert_eq!(doc.prologue.len(), 0);
        assert_eq!(doc.epilogue.len(), 0);
        assert_eq!(doc.content.len(), 1);
        match &doc.content[0] {
            XMLNode::Element(n, a, c) => {
                assert_eq!(n.get_localname(), "Test");
                assert_eq!(a.len(), 0);
                assert_eq!(c.len(), 1);
                match &c[0] {
                    XMLNode::Element(m, b, d) => {
                        assert_eq!(m.get_localname(), "Foo");
                        assert_eq!(b.len(), 0);
                        assert_eq!(d.len(), 1);
                        match &d[0] {
                            XMLNode::Text(w) => {
                                assert_eq!(w.to_string(), "bar")
                            }
                            _ => panic!("child element content is not text"),
                        }
                    }
                    _ => panic!("child element is not an element"),
                }
            }
            _ => {
                panic!("root is not an element node")
            }
        }
    }

    #[test]
    fn mixed() {
        let doc = parse("<Test>i1<Foo>bar</Foo>i2</Test>").expect("failed to parse XML \"<Test>i1<Foo>bar</Foo>i2</Test>\"");
        assert_eq!(doc.prologue.len(), 0);
        assert_eq!(doc.epilogue.len(), 0);
        assert_eq!(doc.content.len(), 1);
        match &doc.content[0] {
            XMLNode::Element(n, a, c) => {
                assert_eq!(n.get_localname(), "Test");
                assert_eq!(a.len(), 0);
                assert_eq!(c.len(), 3);
                match &c[0] {
                    XMLNode::Text(y) => {
                        assert_eq!(y.to_string(), "i1")
                    }
                    _ => panic!("first mixed element content is not text")
                };
                match &c[1] {
                    XMLNode::Element(m, b, d) => {
                        assert_eq!(m.get_localname(), "Foo");
                        assert_eq!(b.len(), 0);
                        assert_eq!(d.len(), 1);
                        match &d[0] {
                            XMLNode::Text(w) => {
                                assert_eq!(w.to_string(), "bar")
                            }
                            _ => panic!("child element content is not text"),
                        }
                    }
                    _ => panic!("child element is not an element"),
                };
                match &c[2] {
                    XMLNode::Text(z) => {
                        assert_eq!(z.to_string(), "i2")
                    }
                    _ => panic!("third mixed element content is not text")
                };
            }
            _ => {
                panic!("root is not an element node")
            }
        }
    }

    #[test]
    fn cdata() {
        let doc = "<doc><![CDATA[<doc<!DOCTYPE&a%b&#c]] >] ]> ]]]><![CDATA[]]><![CDATA[<![CDATA[]]></doc>";
        let result = parse(doc).expect("failed to parse XML \"<doc><![CDATA[<doc<!DOCTYPE&a%b&#c]] >] ]> ]]]><![CDATA[]]><![CDATA[<![CDATA[]]></doc>\"");
        assert_eq!(result.prologue.len(), 0);
        assert_eq!(result.epilogue.len(), 0);
        assert_eq!(result.content.len(), 1);
        match &result.content[0] {
            XMLNode::Element(n, a, c) => {
                assert_eq!(n.get_localname(), "doc");
                assert_eq!(a.len(), 0);
                assert_eq!(c.len(), 1);
                match &c[0]{
                    XMLNode::Text(t) => {
                        assert_eq!(t.to_string(), "<doc<!DOCTYPE&a%b&#c]] >] ]> ]<![CDATA[");
                    }
                    _ => {
                        panic!("element content is not text")
                    }
                }
            }
            _ => {
                panic!("root is not an element node")
            }
        }
    }

    #[test]
    fn xmldeclaration() {
        let doc = r#"<?xml version="1.0" encoding="UTF-8"?><doc/>"#;
        let result = parse(doc).expect("failed to parse XML \"<?xml version=\"1.0\" encoding=\"UTF-8\"?><doc/>\"");
        assert_eq!(result.prologue.len(), 0);
        assert_eq!(result.epilogue.len(), 0);
        assert_eq!(result.content.len(), 1);
        match result.xmldecl {
            None => {panic!("XML Declaration not parsed")},
            Some(XMLdecl { version, encoding, standalone  }) => {
                assert_eq!(version, "1.0");
                assert_eq!(encoding, Some("UTF-8".to_string()));
                assert_eq!(standalone, None);
            }
        }
    }
}
