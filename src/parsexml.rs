//! # xdm::parsexml
//!
//! A parser for XML, as a nom parser combinator.
//! XML 1.1, see <https://www.w3.org/TR/xml11/>
//!
//! This is a very simple, minimalist parser of XML. It excludes:
//!	DTDs (and therefore entities)

extern crate nom;

use crate::parsecommon::*;
use crate::qname::*;
use crate::value::Value;
use crate::xdmerror::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while_m_n},
    character::complete::{char, digit1, hex_digit1, multispace0, multispace1, none_of},
    combinator::{map, map_opt, opt, recognize, value, verify},
    multi::{many0, many1},
    sequence::delimited,
    sequence::tuple,
    IResult,
};
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::str::FromStr;

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
    pub xmldecl: Option<XMLdecl>,
}

impl XMLDocument {
    /// Expand general entities in the document
    pub fn expand(&mut self) -> Result<(), Error> {
        let mut ent: HashMap<QualifiedName, Vec<XMLNode>> = HashMap::new();

        // Process the entity declarations to get the definition of each entity
        for p in &self.prologue {
            if let XMLNode::DTD(d) = p {
                let DTDDecl::GeneralEntity(n, c) = d;
                let (rest, e) = content(c.as_str())
                    .map_err(|e| Error::new(ErrorKind::Unknown, e.to_string()))?;
                if rest.len() != 0 {
                    return Result::Err(Error::new(
                        ErrorKind::Unknown,
                        format!("unable to parse general entity \"{}\"", n.to_string()),
                    ));
                }
                match ent.insert(n.clone(), e) {
                    Some(_) => {
                        return Result::Err(Error::new(
                            ErrorKind::Unknown,
                            format!("general entity \"{}\" already defined", n.to_string()),
                        ))
                    }
                    None => {}
                }
            }
        }

        // Now search for references and replace them with their content
        // This naieve implementation copies the entire document...
        // TODO: a better implementation that mutates the current document
        let mut new: Vec<XMLNode> = vec![];
        for e in &self.content {
            let mut a = expand_node(e, &ent);
            new.append(&mut a);
        }
        self.content = new;

        Ok(())
    }
}

fn expand_node(n: &XMLNode, ent: &HashMap<QualifiedName, Vec<XMLNode>>) -> Vec<XMLNode> {
    match n {
        XMLNode::Reference(qn) => {
            ent.get(&qn).map_or(vec![], |x| x.clone()) // TODO: an undefined entity name is an error
        }
        XMLNode::Element(qn, attr, content) => {
            let mut attrs: Vec<XMLNode> = vec![];
            for a in attr {
                let mut b = expand_node(a, ent);
                attrs.append(&mut b);
            }
            let mut newcontent: Vec<XMLNode> = vec![];
            for c in content {
                let mut d = expand_node(c, ent);
                newcontent.append(&mut d);
            }
            vec![XMLNode::Element(qn.clone(), attrs, newcontent)]
        }
        XMLNode::Attribute(qn, v) => {
            // TODO: expand attribute value
            vec![XMLNode::Attribute(qn.clone(), v.clone())]
        }
        XMLNode::Text(t) => {
            vec![XMLNode::Text(t.clone())]
        }
        XMLNode::PI(_, _) | XMLNode::Comment(_) | XMLNode::DTD(_) => vec![], // TODO
    }
}

impl TryFrom<&str> for XMLDocument {
    type Error = Error;
    fn try_from(e: &str) -> Result<Self, Self::Error> {
        let e = trim_whitespace(e);
        match document(&e) {
            Ok((rest, value)) => {
                if rest == "" {
                    Result::Ok(value)
                } else {
                    Result::Err(Error {
                        kind: ErrorKind::Unknown,
                        message: String::from(format!(
                            "extra characters after expression: \"{}\"",
                            rest
                        )),
                    })
                }
            }
            Err(nom::Err::Error(c)) => Result::Err(Error {
                kind: ErrorKind::Unknown,
                message: format!("parser error: {:?}", c),
            }),
            Err(nom::Err::Incomplete(_)) => Result::Err(Error {
                kind: ErrorKind::Unknown,
                message: String::from("incomplete input"),
            }),
            Err(nom::Err::Failure(_)) => Result::Err(Error {
                kind: ErrorKind::Unknown,
                message: String::from("unrecoverable parser error"),
            }),
        }
    }
}
impl TryFrom<String> for XMLDocument {
    type Error = Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        XMLDocument::try_from(s.as_str())
    }
}

#[derive(Clone, PartialEq)]
pub enum XMLNode {
    Element(QualifiedName, Vec<XMLNode>, Vec<XMLNode>), // Element name, attributes, content
    Attribute(QualifiedName, Value),
    Text(Value),
    PI(String, Value),
    Comment(Value),           // Comment value is a string
    DTD(DTDDecl),             // These only occur in the prologue
    Reference(QualifiedName), // General entity reference. These need to be resolved before presentation to the application
}

#[derive(PartialEq)]
pub struct XMLdecl {
    version: String,
    encoding: Option<String>,
    standalone: Option<String>,
}

/// DTD declarations.
/// Only general entities are supported, so far.
/// TODO: element, attribute declarations
#[derive(Clone, PartialEq)]
pub enum DTDDecl {
    GeneralEntity(QualifiedName, String),
}

// document ::= ( prolog element misc*)
fn document(input: &str) -> IResult<&str, XMLDocument> {
    map(tuple((opt(prolog), element, opt(misc))), |(p, e, m)| {
        let pr = p.unwrap_or((None, vec![]));

        XMLDocument {
            content: vec![e],
            epilogue: m.unwrap_or(vec![]),
            xmldecl: pr.0,
            prologue: pr.1,
        }
    })(input)
}

// prolog ::= XMLDecl misc* (doctypedecl Misc*)?
fn prolog(input: &str) -> IResult<&str, (Option<XMLdecl>, Vec<XMLNode>)> {
    map(tuple((opt(xmldecl), opt(doctypedecl))), |(x, dtd)| {
        (x, dtd.map_or(vec![], |d| d))
    })(input)
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
                    delimited_string,
                )),
                |(_, _, _, _, v)| v,
            ),
            multispace0,
            opt(map(
                tuple((
                    tag("encoding"),
                    multispace0,
                    tag("="),
                    multispace0,
                    delimited_string,
                )),
                |(_, _, _, _, e)| e,
            )),
            multispace0,
            opt(map(
                tuple((
                    tag("standalone"),
                    multispace0,
                    tag("="),
                    multispace0,
                    delimited_string,
                )),
                |(_, _, _, _, s)| s,
            )),
            multispace0,
            tag("?>"),
        )),
        |(_, _, ver, _, enc, _, sta, _, _)| XMLdecl {
            version: ver,
            encoding: enc,
            standalone: sta,
        },
    )(input)
}

fn doctypedecl(input: &str) -> IResult<&str, Vec<XMLNode>> {
    map(
        tuple((
            tag("<!DOCTYPE"),
            multispace1,
            qualname,
            map(opt(map(tuple((multispace1, externalid)), |e| e)), |e| e),
            multispace0,
            opt(map(
                tuple((
                    tag("["),
                    multispace0,
                    intsubset,
                    multispace0,
                    tag("]"),
                    multispace0,
                )),
                |(_, _, i, _, _, _)| i,
            )),
            tag(">"),
        )),
        |(_, _, _n, _extid, _, intss, _)| {
            // TODO: the name must match the document element
            intss.map_or(vec![], |i| i)
        },
    )(input)
}

// TODO: parameter entities
// intSubset ::= (markupdecl | DeclSep)*
// markupdecl ::= elementdecl | AttlistDecl | EntityDecl | NotationDecl | PI | Comment
fn intsubset(input: &str) -> IResult<&str, Vec<XMLNode>> {
    many0(alt((entitydecl, processing_instruction, comment)))(input)
}

// EntityDecl ::= GEDecl | PEDecl
// TODO: support parameter entities
fn entitydecl(input: &str) -> IResult<&str, XMLNode> {
    // TODO: handle quotes properly
    map(
        tuple((
            tag("<!ENTITY"),
            multispace1,
            qualname,
            multispace1,
            entityvalue,
            multispace0,
            tag(">"),
        )),
        |(_, _, n, _, v, _, _)| XMLNode::DTD(DTDDecl::GeneralEntity(n, v)),
    )(input)
}

fn entityvalue(input: &str) -> IResult<&str, String> {
    alt((entityvalue_single, entityvalue_double))(input)
}
// TODO: parameter entity references
fn entityvalue_single(input: &str) -> IResult<&str, String> {
    map(
        delimited(
            char('\''),
            recognize(many0(alt((
                map(recognize(reference), |r| String::from(r)),
                map(many1(none_of("'&")), |v| v.iter().collect::<String>()),
            )))),
            char('\''),
        ),
        |e| String::from(e),
    )(input)
}
fn entityvalue_double(input: &str) -> IResult<&str, String> {
    map(
        delimited(
            char('"'),
            recognize(many0(alt((
                map(recognize(reference), |r| String::from(r)),
                map(many1(none_of("\"&")), |v| v.iter().collect::<String>()),
            )))),
            char('"'),
        ),
        |e| String::from(e),
    )(input)
}

fn externalid(input: &str) -> IResult<&str, Vec<XMLNode>> {
    map(tag("not yet implemented"), |_| {
        vec![XMLNode::Text(Value::String(
            "external ID not yet implemented".to_string(),
        ))]
    })(input)
}

// Element ::= EmptyElemTag | STag content ETag
fn element(input: &str) -> IResult<&str, XMLNode> {
    map(alt((emptyelem, taggedelem)), |e| {
        // TODO: Check for namespace declarations, and resolve URIs in the node tree under 'e'
        e
    })(input)
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
        },
    )(input)
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
        |(_, n, a, _, _)| XMLNode::Element(n, a, vec![]),
    )(input)
}

fn attributes(input: &str) -> IResult<&str, Vec<XMLNode>> {
    //this is just a wrapper around the attribute function, that checks for duplicates.
    verify(many0(attribute), |v: &[XMLNode]| {
        let attrs = v.clone();
        let uniqueattrs: HashSet<_> = attrs
            .iter()
            .map(|xmlnode| match xmlnode {
                XMLNode::Attribute(q, _) => q.to_string(),
                _ => "".to_string(),
            })
            .collect();
        if &v.len() == &uniqueattrs.len() {
            true
        } else {
            false
        }
    })(input)
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
        |(_, n, _, _, _, s)| XMLNode::Attribute(n, Value::String(s)),
    )(input)
}
fn delimited_string(input: &str) -> IResult<&str, String> {
    alt((string_single, string_double))(input)
}
fn string_single(input: &str) -> IResult<&str, String> {
    delimited(
        char('\''),
        map(many0(none_of("'")), |v| v.iter().collect::<String>()),
        char('\''),
    )(input)
}
fn string_double(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        map(many0(none_of("\"")), |v| v.iter().collect::<String>()),
        char('"'),
    )(input)
}

// content ::= CharData? ((element | Reference | CDSect | PI | Comment) CharData?)*
fn content(input: &str) -> IResult<&str, Vec<XMLNode>> {
    map(
        tuple((
            opt(chardata),
            many0(tuple((
                alt((
                    element,
                    reference,
                    // TODO: CData Section
                    processing_instruction,
                    comment,
                )),
                opt(chardata),
            ))),
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
        },
    )(input)
}

// Reference ::= EntityRef | CharRef
fn reference(input: &str) -> IResult<&str, XMLNode> {
    alt((entityref, charref))(input)
}
fn entityref(input: &str) -> IResult<&str, XMLNode> {
    map(tuple((char('&'), qualname, char(';'))), |(_, n, _)| {
        XMLNode::Reference(n)
    })(input)
}
fn charref(input: &str) -> IResult<&str, XMLNode> {
    alt((charref_octal, charref_hex))(input)
}
fn charref_octal(input: &str) -> IResult<&str, XMLNode> {
    map(
        tuple((char('&'), char('#'), digit1, char(';'))),
        |(_, _, n, _)| {
            let u = match u32::from_str_radix(n, 8) {
                Ok(c) => c,
                Err(_) => 0, // TODO: pass back error to nom
            };
            match std::char::from_u32(u) {
                Some(c) => XMLNode::Text(Value::from(c.to_string())),
                None => {
                    //make_error(input, NomErrorKind::OctDigit)
                    XMLNode::Text(Value::from(""))
                }
            }
        },
    )(input)
}
fn charref_hex(input: &str) -> IResult<&str, XMLNode> {
    map(
        tuple((char('&'), char('#'), char('x'), hex_digit1, char(';'))),
        |(_, _, _, n, _)| {
            let u = match u32::from_str_radix(n, 16) {
                Ok(c) => c,
                Err(_) => 0, // TODO: pass back error to nom
            };
            match std::char::from_u32(u) {
                Some(c) => XMLNode::Text(Value::from(c.to_string())),
                None => {
                    //make_error(input, NomErrorKind::OctDigit)
                    XMLNode::Text(Value::from(""))
                }
            }
        },
    )(input)
}

// PI ::= '<?' PITarget (char* - '?>') '?>'
fn processing_instruction(input: &str) -> IResult<&str, XMLNode> {
    map(
        delimited(
            tag("<?"),
            tuple((multispace0, name, multispace0, take_until("?>"))),
            tag("?>"),
        ),
        |(_, n, _, v)| XMLNode::PI(String::from(n), Value::String(v.to_string())),
    )(input)
}

// Comment ::= '<!--' (char* - '--') '-->'
fn comment(input: &str) -> IResult<&str, XMLNode> {
    map(
        delimited(tag("<!--"), take_until("--"), tag("-->")),
        |v: &str| XMLNode::Comment(Value::String(v.to_string())),
    )(input)
}

// Misc ::= Comment | PI | S
fn misc(input: &str) -> IResult<&str, Vec<XMLNode>> {
    map(tag("not yet implemented"), |_| {
        //vec![Node::new(NodeType::Comment).set_value("not yet implemented".to_string())]
        vec![]
    })(input)
}

// CharData ::= [^<&]* - (']]>')
fn chardata(input: &str) -> IResult<&str, String> {
    map(
        many1(alt((chardata_cdata, chardata_escapes, chardata_literal))),
        |v| v.join(""),
    )(input)
}

fn chardata_cdata(input: &str) -> IResult<&str, String> {
    map(
        delimited(tag("<![CDATA["), take_until("]]>"), tag("]]>")),
        |cd: &str| cd.to_string(),
    )(input)
}

fn chardata_escapes(input: &str) -> IResult<&str, String> {
    alt((
        chardata_unicode_codepoint,
        value(">".to_string(), tag("&gt;")),
        value("<".to_string(), tag("&lt;")),
        value("&".to_string(), tag("&amp;")),
        value("\"".to_string(), tag("&quot;")),
        value("\'".to_string(), tag("&apos;")),
    ))(input)
}

fn chardata_unicode_codepoint(input: &str) -> IResult<&str, String> {
    let parse_hex = map(
        take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit()),
        |hex| u32::from_str_radix(hex, 16),
    );

    let parse_decimal = map(take_while_m_n(1, 6, |c: char| c.is_ascii_digit()), |dec| {
        u32::from_str(dec)
    });

    map_opt(
        alt((
            delimited(tag("&#x"), parse_hex, tag(";")),
            delimited(tag("&#"), parse_decimal, tag(";")),
        )),
        |value| Option::from(std::char::from_u32(value.unwrap()).unwrap().to_string()),
    )(input)
}

fn chardata_literal(input: &str) -> IResult<&str, String> {
    map(
        verify(many1(none_of("<&")), |v: &[char]| {
            // chardata cannot contain ]]>
            let cd_end = &[']', ']', '>'][..];
            let mut w = v.clone();
            while !w.is_empty() {
                if w.starts_with(cd_end) {
                    return false;
                }
                if !is_char(&w[0]) {
                    return false;
                }
                w = &w[1..];
            }
            true
        }),
        |c| c.iter().collect::<String>(),
    )(input)
}

// QualifiedName
fn qualname(input: &str) -> IResult<&str, QualifiedName> {
    alt((prefixed_name, unprefixed_name))(input)
}
fn unprefixed_name(input: &str) -> IResult<&str, QualifiedName> {
    map(ncname, |localpart| {
        QualifiedName::new(None, None, String::from(localpart))
    })(input)
}
fn prefixed_name(input: &str) -> IResult<&str, QualifiedName> {
    map(
        tuple((ncname, tag(":"), ncname)),
        |(prefix, _, localpart)| {
            QualifiedName::new(None, Some(String::from(prefix)), String::from(localpart))
        },
    )(input)
}

fn trim_whitespace(s: &str) -> String {
    let s = s.replace("", "<!--  -->");
    let s = s.replace("", "<!--  -->");
    let re = regex::Regex::new(r"^[\s]+(.*?)[\s]*$").unwrap();
    let result = re.replace_all(&s, "$1");

    result.to_owned().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let doc = XMLDocument::try_from("<Test/>").expect("failed to parse XML \"<Test/>\"");
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
    fn preceeding_and_trailing_whitespace() {
        let doc =
            XMLDocument::try_from("   <Test/> \n \r  ").expect("failed to parse XML \"<Test/>\"");
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
        let doc =
            XMLDocument::try_from("<Test></Test>").expect("failed to parse XML \"<Test></Test>\"");
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
        let doc = XMLDocument::try_from("<Test>Foobar</Test>")
            .expect("failed to parse XML \"<Test>Foobar</Test>\"");
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
        let doc = XMLDocument::try_from("<Test><Foo>bar</Foo></Test>")
            .expect("failed to parse XML \"<Test><Foo>bar</Foo></Test>\"");
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
    fn ref_pos() {
        let doc = XMLDocument::try_from("<Test>&foo;</Test>")
            .expect("failed to parse XML \"<Test>&foo;</Test>\"");
        assert_eq!(doc.prologue.len(), 0);
        assert_eq!(doc.epilogue.len(), 0);
        assert_eq!(doc.content.len(), 1);
    }
    #[test]
    #[should_panic]
    fn ref_neg_1() {
        // Missing ;
        let doc = XMLDocument::try_from("<Test>&foo</Test>")
            .expect("failed to parse XML \"<Test>&foo</Test>\"");
        assert_eq!(doc.prologue.len(), 0);
        assert_eq!(doc.epilogue.len(), 0);
        assert_eq!(doc.content.len(), 0);
    }
    #[test]
    fn ref_neg_2() {
        // space
        let doc = XMLDocument::try_from("<Test>& foo;</Test>");
        assert!(doc.is_err());
    }

    #[test]
    fn char_ref_oct() {
        let doc = XMLDocument::try_from("<Test>&#65;</Test>")
            .expect("failed to parse XML \"<Test>&#65;</Test>\"");
        assert_eq!(doc.prologue.len(), 0);
        assert_eq!(doc.epilogue.len(), 0);
        assert_eq!(doc.content.len(), 1);
        match &doc.content[0] {
            XMLNode::Element(n, a, c) => {
                assert_eq!(n.get_localname(), "Test");
                assert_eq!(a.len(), 0);
                assert_eq!(c.len(), 1);
                match &c[0] {
                    XMLNode::Text(t) => {
                        assert_eq!(t.to_string(), "A")
                    }
                    _ => panic!("document element content is not text"),
                }
            }
            _ => panic!("document element is not \"Test\""),
        }
    }
    #[test]
    fn char_ref_hex() {
        let doc = XMLDocument::try_from("<Test>&#x03c7;</Test>")
            .expect("failed to parse XML \"<Test>&#x03c7;</Test>\"");
        assert_eq!(doc.prologue.len(), 0);
        assert_eq!(doc.epilogue.len(), 0);
        assert_eq!(doc.content.len(), 1);
        match &doc.content[0] {
            XMLNode::Element(n, a, c) => {
                assert_eq!(n.get_localname(), "Test");
                assert_eq!(a.len(), 0);
                assert_eq!(c.len(), 1);
                match &c[0] {
                    XMLNode::Text(t) => {
                        assert_eq!(t.to_string(), "\u{03c7}")
                    }
                    _ => panic!("document element content is not text"),
                }
            }
            _ => panic!("document element is not \"Test\""),
        }
    }

    #[test]
    fn mixed() {
        let doc = XMLDocument::try_from("<Test>i1<Foo>bar</Foo>i2</Test>")
            .expect("failed to parse XML \"<Test>i1<Foo>bar</Foo>i2</Test>\"");
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
                    _ => panic!("first mixed element content is not text"),
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
                    _ => panic!("third mixed element content is not text"),
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
        let result = XMLDocument::try_from(doc).expect("failed to parse XML \"<doc><![CDATA[<doc<!DOCTYPE&a%b&#c]] >] ]> ]]]><![CDATA[]]><![CDATA[<![CDATA[]]></doc>\"");
        assert_eq!(result.prologue.len(), 0);
        assert_eq!(result.epilogue.len(), 0);
        assert_eq!(result.content.len(), 1);
        match &result.content[0] {
            XMLNode::Element(n, a, c) => {
                assert_eq!(n.get_localname(), "doc");
                assert_eq!(a.len(), 0);
                assert_eq!(c.len(), 1);
                match &c[0] {
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
        let result = XMLDocument::try_from(doc)
            .expect("failed to parse XML \"<?xml version=\"1.0\" encoding=\"UTF-8\"?><doc/>\"");
        assert_eq!(result.prologue.len(), 0);
        assert_eq!(result.epilogue.len(), 0);
        assert_eq!(result.content.len(), 1);
        match result.xmldecl {
            None => {
                panic!("XML Declaration not parsed")
            }
            Some(XMLdecl {
                version,
                encoding,
                standalone,
            }) => {
                assert_eq!(version, "1.0");
                assert_eq!(encoding, Some("UTF-8".to_string()));
                assert_eq!(standalone, None);
            }
        }
    }

    #[test]
    fn general_entity_1() {
        let doc = r#"<?xml version="1.0" encoding="UTF-8"?><!DOCTYPE doc [<!ENTITY general 'entity'>]><doc>&general;</doc>"#;
        let result = XMLDocument::try_from(doc).expect("failed to parse XML \"<?xml version=\"1.0\" encoding=\"UTF-8\"?><doc>&general;</doc>\"");
        assert_eq!(result.prologue.len(), 1);
        assert_eq!(result.epilogue.len(), 0);
        assert_eq!(result.content.len(), 1);
        match result.xmldecl {
            None => {
                panic!("XML Declaration not parsed")
            }
            Some(XMLdecl {
                version,
                encoding,
                standalone,
            }) => {
                assert_eq!(version, "1.0");
                assert_eq!(encoding, Some("UTF-8".to_string()));
                assert_eq!(standalone, None);
            }
        }
        match &result.prologue[0] {
            XMLNode::DTD(DTDDecl::GeneralEntity(n, v)) => {
                assert_eq!(n.to_string(), "general");
                assert_eq!(v, "entity");
            }
            _ => {
                panic!("prologue contains something other than a general entity declaration")
            }
        }
        match &result.content[0] {
            XMLNode::Element(n, a, c) => {
                assert_eq!(n.get_localname(), "doc");
                assert_eq!(a.len(), 0);
                assert_eq!(c.len(), 1);
                match &c[0] {
                    XMLNode::Reference(e) => {
                        assert_eq!(e.to_string(), "general")
                    }
                    _ => {
                        panic!("failed to find text")
                    }
                }
            }
            _ => {
                panic!("root is not an element node")
            }
        }
    }

    #[test]
    fn general_entity_2() {
        let doc = r#"<?xml version="1.0" encoding="UTF-8"?><!DOCTYPE doc [<!ENTITY general '<expansion>entity</expansion>'>]><doc>&general;</doc>"#;
        let mut result = XMLDocument::try_from(doc).expect("failed to parse XML \"<?xml version=\"1.0\" encoding=\"UTF-8\"?><doc>&general;</doc>\"");
        result.expand().expect("unable to expand entities");
        assert_eq!(result.prologue.len(), 1);
        assert_eq!(result.epilogue.len(), 0);
        assert_eq!(result.content.len(), 1);
        match result.xmldecl {
            None => {
                panic!("XML Declaration not parsed")
            }
            Some(XMLdecl {
                version,
                encoding,
                standalone,
            }) => {
                assert_eq!(version, "1.0");
                assert_eq!(encoding, Some("UTF-8".to_string()));
                assert_eq!(standalone, None);
            }
        }
        match &result.content[0] {
            XMLNode::Element(n, a, c) => {
                assert_eq!(n.get_localname(), "doc");
                assert_eq!(a.len(), 0);
                assert_eq!(c.len(), 1);
                match &c[0] {
                    XMLNode::Element(m, b, d) => {
                        assert_eq!(m.get_localname(), "expansion");
                        assert_eq!(b.len(), 0);
                        assert_eq!(d.len(), 1);
                        match &d[0] {
                            XMLNode::Text(e) => {
                                assert_eq!(e.to_string(), "entity")
                            }
                            _ => {
                                panic!("failed to find text")
                            }
                        }
                    }
                    _ => {
                        panic!("failed to find \"expansion\" element")
                    }
                }
            }
            _ => {
                panic!("root is not an element node")
            }
        }
    }
}
