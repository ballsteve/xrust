//! # xdm::parsexml
//!
//! A parser for XML, as a nom parser combinator.
//! XML 1.1, see <https://www.w3.org/TR/xml11/>
//!
//! This is a very simple, minimalist parser of XML. It excludes:
//!	DTDs (and therefore entities)
//!
//! The parser produces a document tree as an [ADoc]; a tree structure that is mutable, but not fully navigable.

extern crate nom;

use crate::intmuttree::{
    DTDDecl, Document, DocumentBuilder, NodeBuilder, RNode, XMLDecl, XMLDeclBuilder,
};
use crate::item::{Node as ItemNode, NodeType};
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

// For backward compatibility
pub type XMLDocument = Document;

// nom doesn't pass additional parameters, only the input,
// so this is a two-pass process.

impl TryFrom<&str> for Document {
    type Error = Error;
    fn try_from(e: &str) -> Result<Self, Self::Error> {
        match document(e) {
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
impl TryFrom<String> for Document {
    type Error = Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Document::try_from(s.as_str())
    }
}

// document ::= ( prolog element misc*)
fn document(input: &str) -> IResult<&str, Document> {
    map(tuple((opt(prolog), element, opt(misc))), |(p, e, m)| {
        let pr = p.unwrap_or((None, vec![]));

        // TODO: resolve XML Namespace declarations during the parsing phase. To do that, the parser needs to be able to build state and carry that forward through the parsing process
        let mut ns: HashMap<String, String> = HashMap::new();
        resolve_namespaces(e.clone(), &mut ns).expect("unable to resolve namespaces");

        let mut a = DocumentBuilder::new()
            .prologue(pr.1)
            .content(vec![e])
            .epilogue(m.unwrap_or(vec![]))
            .build();
        pr.0.map(|x| a.set_xmldecl(x));
        a
    })(input)
}

// Resolve XML Namespace declarations and qualified names
fn resolve_namespaces(e: RNode, ns: &mut HashMap<String, String>) -> Result<(), Error> {
    e.attribute_iter()
        .filter(|b| b.name().get_prefix().map_or(false, |p| p == "xmlns"))
        .for_each(|b| {
            ns.insert(b.name().get_localname(), b.value().to_string());
        });
    if e.node_type() == NodeType::Element {
        if let Some(p) = e.name().get_prefix() {
            match ns.get(&p) {
                Some(u) => e.set_nsuri(u.clone()),
                None => {
                    return Result::Err(Error::new(
                        ErrorKind::Unknown,
                        format!("XML Namespace URI not found for prefix \"{}\"", p),
                    ))
                }
            }
        }
    }
    e.child_iter()
	.try_for_each(|c| {
	    resolve_namespaces(c, ns)?;
	    Ok::<(), Error>(())
	})?;
    Ok(())
}

// prolog ::= XMLDecl misc* (doctypedecl Misc*)?
fn prolog(input: &str) -> IResult<&str, (Option<XMLDecl>, Vec<RNode>)> {
    map(tuple((opt(xmldecl), opt(doctypedecl))), |(x, dtd)| {
        (x, dtd.map_or(vec![], |d| d))
    })(input)
}

fn xmldecl(input: &str) -> IResult<&str, XMLDecl> {
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
        |(_, _, ver, _, enc, _, sta, _, _)| {
            let mut x = XMLDeclBuilder::new().version(ver).build();
            enc.map(|e| x.set_encoding(e));
            sta.map(|s| x.set_standalone(s));
            x
        },
    )(input)
}

fn doctypedecl(input: &str) -> IResult<&str, Vec<RNode>> {
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
fn intsubset(input: &str) -> IResult<&str, Vec<RNode>> {
    many0(alt((entitydecl, processing_instruction, comment)))(input)
}

// EntityDecl ::= GEDecl | PEDecl
// TODO: support parameter entities
fn entitydecl(input: &str) -> IResult<&str, RNode> {
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
        |(_, _, n, _, v, _, _)| {
            NodeBuilder::new(NodeType::Unknown)
                .dtd(DTDDecl::GeneralEntity(n, v))
                .build()
        },
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

fn externalid(input: &str) -> IResult<&str, Vec<RNode>> {
    map(tag("not yet implemented"), |_| {
        vec![NodeBuilder::new(NodeType::Text)
            .value(Value::String("external ID not yet implemented".to_string()))
            .build()]
    })(input)
}

// Element ::= EmptyElemTag | STag content ETag
fn element(input: &str) -> IResult<&str, RNode> {
    map(alt((emptyelem, taggedelem)), |e| {
        // TODO: Check for namespace declarations, and resolve URIs in the node tree under 'e'
        e
    })(input)
}

// STag ::= '<' Name (Attribute)* '>'
// ETag ::= '</' Name '>'
// NB. Names must match
fn taggedelem(input: &str) -> IResult<&str, RNode> {
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
        |(_, n, av, _, _, c, _, _e, _, _)| {
            // TODO: check that the start tag name and end tag name match (n == e)
            let mut a = NodeBuilder::new(NodeType::Element).name(n).build();
            av.iter()
                .for_each(|b| a.add_attribute(b.clone()).expect("unable to add attribute"));
            c.iter().for_each(|d| {
                a.push(d.clone()).expect("unable to add node");
            });
            a
        },
    )(input)
}

// EmptyElemTag ::= '<' Name (Attribute)* '/>'
fn emptyelem(input: &str) -> IResult<&str, RNode> {
    map(
        tuple((
            tag("<"),
            qualname,
            attributes, //many0(attribute),
            multispace0,
            tag("/>"),
        )),
        |(_, n, av, _, _)| {
            let e = NodeBuilder::new(NodeType::Element).name(n).build();
            av.iter()
                .for_each(|b| e.add_attribute(b.clone()).expect("unable to add attribute"));
            e
        },
    )(input)
}

fn attributes(input: &str) -> IResult<&str, Vec<RNode>> {
    //this is just a wrapper around the attribute function, that checks for duplicates.
    verify(many0(attribute), |v: &[RNode]| {
        let attrs = v.clone();
        let uniqueattrs: HashSet<_> = attrs
            .iter()
            .map(|xmlnode| match xmlnode.node_type() {
                NodeType::Attribute => xmlnode.name().to_string(),
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
fn attribute(input: &str) -> IResult<&str, RNode> {
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
            NodeBuilder::new(NodeType::Attribute)
                .name(n)
                .value(Value::String(s))
                .build()
        },
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
pub(crate) fn content(input: &str) -> IResult<&str, Vec<RNode>> {
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
            let mut new: Vec<RNode> = Vec::new();
            if c.is_some() {
                new.push(
                    NodeBuilder::new(NodeType::Text)
                        .value(Value::String(c.unwrap()))
                        .build(),
                );
            }
            if v.len() != 0 {
                for (w, d) in v {
                    new.push(w);
                    if d.is_some() {
                        new.push(
                            NodeBuilder::new(NodeType::Text)
                                .value(Value::String(d.unwrap()))
                                .build(),
                        );
                    }
                }
            }
            new
        },
    )(input)
}

// Reference ::= EntityRef | CharRef
fn reference(input: &str) -> IResult<&str, RNode> {
    alt((entityref, charref))(input)
}
fn entityref(input: &str) -> IResult<&str, RNode> {
    map(tuple((char('&'), qualname, char(';'))), |(_, n, _)| {
        NodeBuilder::new(NodeType::Unknown).reference(n).build()
    })(input)
}
fn charref(input: &str) -> IResult<&str, RNode> {
    alt((charref_octal, charref_hex))(input)
}
fn charref_octal(input: &str) -> IResult<&str, RNode> {
    map(
        tuple((char('&'), char('#'), digit1, char(';'))),
        |(_, _, n, _)| {
            let u = match u32::from_str_radix(n, 8) {
                Ok(c) => c,
                Err(_) => 0, // TODO: pass back error to nom
            };
            match std::char::from_u32(u) {
                Some(c) => NodeBuilder::new(NodeType::Text)
                    .value(Value::from(c.to_string()))
                    .build(),
                None => {
                    //make_error(input, NomErrorKind::OctDigit)
                    NodeBuilder::new(NodeType::Text)
                        .value(Value::from(""))
                        .build()
                }
            }
        },
    )(input)
}
fn charref_hex(input: &str) -> IResult<&str, RNode> {
    map(
        tuple((char('&'), char('#'), char('x'), hex_digit1, char(';'))),
        |(_, _, _, n, _)| {
            let u = match u32::from_str_radix(n, 16) {
                Ok(c) => c,
                Err(_) => 0, // TODO: pass back error to nom
            };
            match std::char::from_u32(u) {
                Some(c) => NodeBuilder::new(NodeType::Text)
                    .value(Value::from(c.to_string()))
                    .build(),
                None => {
                    //make_error(input, NomErrorKind::OctDigit)
                    NodeBuilder::new(NodeType::Text)
                        .value(Value::from(""))
                        .build()
                }
            }
        },
    )(input)
}

// PI ::= '<?' PITarget (char* - '?>') '?>'
fn processing_instruction(input: &str) -> IResult<&str, RNode> {
    map(
        delimited(
            tag("<?"),
            tuple((multispace0, name, multispace0, take_until("?>"))),
            tag("?>"),
        ),
        |(_, n, _, v)| {
            NodeBuilder::new(NodeType::ProcessingInstruction)
                .pi_name(String::from(n))
                .value(Value::String(v.to_string()))
                .build()
        },
    )(input)
}

// Comment ::= '<!--' (char* - '--') '-->'
fn comment(input: &str) -> IResult<&str, RNode> {
    map(
        delimited(tag("<!--"), take_until("--"), tag("-->")),
        |v: &str| {
            NodeBuilder::new(NodeType::Comment)
                .value(Value::String(v.to_string()))
                .build()
        },
    )(input)
}

// Misc ::= Comment | PI | S
fn misc(input: &str) -> IResult<&str, Vec<RNode>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let doc = Document::try_from("<Test/>").expect("failed to parse XML \"<Test/>\"");
        assert_eq!(doc.prologue.len(), 0);
        assert_eq!(doc.epilogue.len(), 0);
        assert_eq!(doc.content.len(), 1);
        assert_eq!(doc.content[0].node_type(), NodeType::Element);
        assert_eq!(doc.content[0].name().get_localname(), "Test");
        match doc.content[0].child_iter().next() {
            None => {}
            _ => panic!("unexpected child node"),
        }
    }

    #[test]
    fn root_element() {
        let doc =
            Document::try_from("<Test></Test>").expect("failed to parse XML \"<Test></Test>\"");
        assert_eq!(doc.prologue.len(), 0);
        assert_eq!(doc.epilogue.len(), 0);
        assert_eq!(doc.content.len(), 1);
        assert_eq!(doc.content[0].node_type(), NodeType::Element);
        assert_eq!(doc.content[0].name().get_localname(), "Test");
        match doc.content[0].child_iter().next() {
            None => {}
            _ => panic!("unexpected child node"),
        }
    }

    #[test]
    fn root_element_text() {
        let doc = Document::try_from("<Test>Foobar</Test>")
            .expect("failed to parse XML \"<Test>Foobar</Test>\"");
        assert_eq!(doc.prologue.len(), 0);
        assert_eq!(doc.epilogue.len(), 0);
        assert_eq!(doc.content.len(), 1);
        assert_eq!(doc.content[0].node_type(), NodeType::Element);
        assert_eq!(doc.content[0].name().get_localname(), "Test");
        let mut it = doc.content[0].child_iter();
        match it.next() {
            Some(c) => {
                assert_eq!(c.node_type(), NodeType::Text);
                assert_eq!(c.value().to_string(), "Foobar");
            }
            _ => {
                panic!("no text child")
            }
        }
    }

    #[test]
    fn nested() {
        let doc = Document::try_from("<Test><Foo>bar</Foo></Test>")
            .expect("failed to parse XML \"<Test><Foo>bar</Foo></Test>\"");
        assert_eq!(doc.prologue.len(), 0);
        assert_eq!(doc.epilogue.len(), 0);
        assert_eq!(doc.content.len(), 1);
        assert_eq!(doc.content[0].node_type(), NodeType::Element);
        assert_eq!(doc.content[0].name().get_localname(), "Test");
        let mut it1 = doc.content[0].child_iter();
        match it1.next() {
            Some(c) => {
                assert_eq!(c.node_type(), NodeType::Element);
                assert_eq!(c.name().get_localname(), "Foo");
                let mut it2 = c.child_iter();
                match it2.next() {
                    Some(d) => {
                        assert_eq!(d.node_type(), NodeType::Text);
                        assert_eq!(d.value().to_string(), "bar");
                    }
                    None => {
                        panic!("no element grandchild")
                    }
                }
            }
            _ => {
                panic!("no element child")
            }
        }
    }

    #[test]
    fn ns_1() {
        let doc = Document::try_from("<a:Test xmlns:a='urn:test'><a:Foo>bar</a:Foo></a:Test>")
            .expect("failed to parse XML \"<a:Test xmlns:a='urn:test'><a:Foo>bar</a:Foo></a:Test>\"");
        assert_eq!(doc.prologue.len(), 0);
        assert_eq!(doc.epilogue.len(), 0);
        assert_eq!(doc.content.len(), 1);
        assert_eq!(doc.content[0].node_type(), NodeType::Element);
        assert_eq!(doc.content[0].name().get_localname(), "Test");
        assert_eq!(doc.content[0].name().get_prefix(), Some(String::from("a")));
        assert_eq!(doc.content[0].name().get_nsuri(), Some(String::from("urn:test")));
        let mut it1 = doc.content[0].child_iter();
        match it1.next() {
            Some(c) => {
                assert_eq!(c.node_type(), NodeType::Element);
                assert_eq!(c.name().get_localname(), "Foo");
                assert_eq!(c.name().get_prefix(), Some(String::from("a")));
                assert_eq!(c.name().get_nsuri(), Some(String::from("urn:test")));
                let mut it2 = c.child_iter();
                match it2.next() {
                    Some(d) => {
                        assert_eq!(d.node_type(), NodeType::Text);
                        assert_eq!(d.value().to_string(), "bar");
                    }
                    None => {
                        panic!("no element grandchild")
                    }
                }
            }
            _ => {
                panic!("no element child")
            }
        }
    }

    #[test]
    fn ref_pos() {
        let doc = Document::try_from("<Test>&foo;</Test>")
            .expect("failed to parse XML \"<Test>&foo;</Test>\"");
        assert_eq!(doc.prologue.len(), 0);
        assert_eq!(doc.epilogue.len(), 0);
        assert_eq!(doc.content.len(), 1);
    }
    #[test]
    #[should_panic]
    fn ref_neg_1() {
        // Missing ;
        let doc = Document::try_from("<Test>&foo</Test>")
            .expect("failed to parse XML \"<Test>&foo</Test>\"");
        assert_eq!(doc.prologue.len(), 0);
        assert_eq!(doc.epilogue.len(), 0);
        assert_eq!(doc.content.len(), 0);
    }
    #[test]
    fn ref_neg_2() {
        // space
        let doc = Document::try_from("<Test>& foo;</Test>");
        assert!(doc.is_err());
    }

    #[test]
    fn char_ref_oct() {
        let doc = Document::try_from("<Test>&#65;</Test>")
            .expect("failed to parse XML \"<Test>&#65;</Test>\"");
        assert_eq!(doc.prologue.len(), 0);
        assert_eq!(doc.epilogue.len(), 0);
        assert_eq!(doc.content.len(), 1);
        assert_eq!(doc.content[0].node_type(), NodeType::Element);
        assert_eq!(doc.content[0].name().get_localname(), "Test");
        let mut it = doc.content[0].child_iter();
        match it.next() {
            Some(c) => {
                assert_eq!(c.node_type(), NodeType::Text);
                assert_eq!(c.value().to_string(), "A");
            }
            _ => panic!("no child node"),
        }
    }
    #[test]
    fn char_ref_hex() {
        let doc = Document::try_from("<Test>&#x03c7;</Test>")
            .expect("failed to parse XML \"<Test>&#x03c7;</Test>\"");
        assert_eq!(doc.prologue.len(), 0);
        assert_eq!(doc.epilogue.len(), 0);
        assert_eq!(doc.content.len(), 1);
        assert_eq!(doc.content[0].node_type(), NodeType::Element);
        assert_eq!(doc.content[0].name().get_localname(), "Test");
        let mut it = doc.content[0].child_iter();
        match it.next() {
            Some(c) => {
                assert_eq!(c.node_type(), NodeType::Text);
                assert_eq!(c.value().to_string(), "\u{03c7}");
            }
            _ => panic!("no child node"),
        }
    }

    #[test]
    fn mixed() {
        let doc = Document::try_from("<Test>i1<Foo>bar</Foo>i2</Test>")
            .expect("failed to parse XML \"<Test>i1<Foo>bar</Foo>i2</Test>\"");
        assert_eq!(doc.prologue.len(), 0);
        assert_eq!(doc.epilogue.len(), 0);
        assert_eq!(doc.content.len(), 1);
        assert_eq!(doc.content[0].node_type(), NodeType::Element);
        assert_eq!(doc.content[0].name().get_localname(), "Test");
        let mut it = doc.content[0].child_iter();
        match it.next() {
            Some(c) => {
                assert_eq!(c.node_type(), NodeType::Text);
                assert_eq!(c.value().to_string(), "i1");
                match it.next() {
                    Some(d) => {
                        assert_eq!(d.node_type(), NodeType::Element);
                        assert_eq!(d.name().get_localname(), "Foo");
                        match it.next() {
                            Some(e) => {
                                assert_eq!(e.node_type(), NodeType::Text);
                                assert_eq!(e.value().to_string(), "i2");
                            }
                            None => panic!("no third mixed node"),
                        }
                    }
                    None => panic!("no second mixed node"),
                }
            }
            _ => panic!("no child node"),
        }
    }

    #[test]
    fn cdata() {
        let doc = "<doc><![CDATA[<doc<!DOCTYPE&a%b&#c]] >] ]> ]]]><![CDATA[]]><![CDATA[<![CDATA[]]></doc>";
        let result = Document::try_from(doc).expect("failed to parse XML \"<doc><![CDATA[<doc<!DOCTYPE&a%b&#c]] >] ]> ]]]><![CDATA[]]><![CDATA[<![CDATA[]]></doc>\"");
        assert_eq!(result.prologue.len(), 0);
        assert_eq!(result.epilogue.len(), 0);
        assert_eq!(result.content.len(), 1);
        assert_eq!(result.content[0].node_type(), NodeType::Element);
        assert_eq!(result.content[0].name().get_localname(), "doc");
        let mut it = result.content[0].child_iter();
        match it.next() {
            Some(c) => {
                assert_eq!(c.node_type(), NodeType::Text);
                assert_eq!(
                    c.value().to_string(),
                    "<doc<!DOCTYPE&a%b&#c]] >] ]> ]<![CDATA["
                );
            }
            _ => {
                panic!("no text child node")
            }
        }
    }

    #[test]
    fn attrs_1() {
        let doc = "<doc mode='testing'></doc>";
        let result =
            Document::try_from(doc).expect("failed to parse XML \"<doc mode='testing'></doc>\"");
        assert_eq!(result.prologue.len(), 0);
        assert_eq!(result.epilogue.len(), 0);
        assert_eq!(result.content.len(), 1);
        assert_eq!(result.content[0].node_type(), NodeType::Element);
        assert_eq!(result.content[0].name().get_localname(), "doc");
        let mut it = result.content[0].attribute_iter();
        match it.next() {
            Some(a) => {
                assert_eq!(a.node_type(), NodeType::Attribute);
                assert_eq!(a.name().to_string(), "mode");
                assert_eq!(a.value().to_string(), "testing");
            }
            _ => {
                panic!("no attribute node")
            }
        }
    }

    //    #[test]
    //    fn xmldeclaration() {
    //        let doc = r#"<?xml version="1.0" encoding="UTF-8"?><doc/>"#;
    //        let result = ADoc::try_from(doc).expect("failed to parse XML \"<?xml version=\"1.0\" encoding=\"UTF-8\"?><doc/>\"");
    //        assert_eq!(result.prologue.len(), 0);
    //        assert_eq!(result.epilogue.len(), 0);
    //        assert_eq!(result.content.len(), 1);
    //        match result.get_xmldecl() {
    //            None => {panic!("XML Declaration not parsed")},
    //            Some(x) => {
    //                assert_eq!(x.to_string(), String::from("<?xml version=\"1.0\" encoding=\"UTF-8\""));
    //            }
    //        }
    //    }

    //    #[test]
    //    fn general_entity_1() {
    //        let doc = r#"<?xml version="1.0" encoding="UTF-8"?><!DOCTYPE doc [<!ENTITY general 'entity'>]><doc>&general;</doc>"#;
    //        let result = ADoc::try_from(doc).expect("failed to parse XML \"<?xml version=\"1.0\" encoding=\"UTF-8\"?><doc>&general;</doc>\"");
    //        assert_eq!(result.prologue.len(), 1);
    //        assert_eq!(result.epilogue.len(), 0);
    //        assert_eq!(result.content.len(), 1);
    //        match result.get_xmldecl() {
    //            None => {panic!("XML Declaration not parsed")},
    //            Some(x) => {
    //                assert_eq!(x.to_string(), String::from("<?xml version=\"1.0\" encoding=\"UTF-8\""));
    //            }
    //        }
    //	match &result.prologue[0] {
    //	    ANode::DTD(DTDDecl::GeneralEntity(n, v)) => {
    //		assert_eq!(n.to_string(), "general");
    //		assert_eq!(v, "entity");
    //	    }
    //	    _ => {
    //		panic!("prologue contains something other than a general entity declaration")
    //	    }
    //	}
    //	match &result.content[0] {
    //	    ANode::Element(n, a, c) => {
    //		assert_eq!(n.get_localname(), "doc");
    //		assert_eq!(a.len(), 0);
    //		assert_eq!(c.len(), 1);
    //		match &c[0] {
    //		    ANode::Reference(e) => {
    //			assert_eq!(e.to_string(), "general")
    //		    }
    //		    _ => {
    //			panic!("failed to find text")
    //		    }
    //		}
    //	    }
    //	    _ => {
    //		panic!("root is not an element node")
    //	    }
    //	}
    //    }

    //    #[test]
    //    fn general_entity_2() {
    //        let doc = r#"<?xml version="1.0" encoding="UTF-8"?><!DOCTYPE doc [<!ENTITY general '<expansion>entity</expansion>'>]><doc>&general;</doc>"#;
    //        let result = ADoc::try_from(doc).expect("failed to parse XML \"<?xml version=\"1.0\" encoding=\"UTF-8\"?><doc>&general;</doc>\"");
    //	result.expand().expect("unable to expand entities");
    //        assert_eq!(result.prologue.len(), 1);
    //        assert_eq!(result.epilogue.len(), 0);
    //        assert_eq!(result.content.len(), 1);
    //        match result.get_xmldecl() {
    //            None => {panic!("XML Declaration not parsed")},
    //            Some(x) => {
    //                assert_eq!(x.to_string(), String::from("<?xml version=\"1.0\" encoding=\"UTF-8\""));
    //            }
    //        }
    //	match &result.content[0] {
    //	    ANode::Element(n, a, c) => {
    //		assert_eq!(n.get_localname(), "doc");
    //		assert_eq!(a.len(), 0);
    //		assert_eq!(c.len(), 1);
    //		match &c[0] {
    //		    ANode::Element(m, b, d) => {
    //			assert_eq!(m.get_localname(), "expansion");
    //			assert_eq!(b.len(), 0);
    //			assert_eq!(d.len(), 1);
    //			match &d[0] {
    //			    ANode::Text(e) => {
    //				assert_eq!(e.to_string(), "entity")
    //			    }
    //			    _ => {
    //				panic!("failed to find text")
    //			    }
    //			}
    //		    }
    //		    _ => {
    //			panic!("failed to find \"expansion\" element")
    //		    }
    //		}
    //	    }
    //	    _ => {
    //		panic!("root is not an element node")
    //	    }
    //	}
    //    }
}
