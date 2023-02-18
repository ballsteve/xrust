//! # xdm::parsexml
//!
//! A parser for XML, as a parser combinator.
//! XML 1.0, see <https://www.w3.org/TR/xml/>
//! XML 1.0 namespaces, see <http://www.w3.org/TR/xml-names/>
//! XML 1.1, see <https://www.w3.org/TR/xml11/>
//! XML 1.1 namespaces, see <http://www.w3.org/TR/xml-names11/>
//!

//extern crate nom;

use crate::parser::common::{
    is_char, is_namechar, is_pubid_char, is_pubid_charwithapos, name, ncname,
};
use crate::qname::*;
use crate::xdmerror::*;
use std::collections::HashMap;
use std::str::FromStr;

use crate::parser::combinators::alt::{alt2, alt3, alt4, alt7, alt8};
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::expander::{genentityexpander, paramentityexpander};
use crate::parser::combinators::many::{many0, many1};
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::{take_until, take_while, take_while_m_n};
use crate::parser::combinators::tuple::{
    tuple10, tuple2, tuple3, tuple4, tuple5, tuple6, tuple7, tuple8, tuple9,
};
use crate::parser::combinators::validate::validate;
use crate::parser::combinators::value::value;
use crate::parser::combinators::whitespace::{whitespace0, whitespace1};
use crate::parser::{ParseError, ParseInput, ParseResult};

use crate::intmuttree::{DTDDecl, Document, DocumentBuilder, NodeBuilder, RNode, XMLDecl};
use crate::item::{Node as ItemNode, NodeType};
use crate::value::Value;

// nom doesn't pass additional parameters, only the input,
// so this is a two-pass process.
// First, use nom to tokenize and parse the input.
// Second, use the internal structure returned by the parser
// to build the document structure.

// For backward compatibility
pub type XMLDocument = Document;

pub fn parse(e: String) -> Result<XMLDocument, Error> {
    let input = ParseInput::new(e.as_str());
    match document(input) {
        Ok((_, xmldoc)) => Result::Ok(xmldoc),
        Err(err) => {
            match err {
                ParseError::Combinator => Result::Err(Error {
                    kind: ErrorKind::ParseError,
                    message: "Unrecoverable parser error.".to_string(),
                }),
                /*
                ParseError::InvalidChar { row, col } => {
                    Result::Err(Error {
                        kind: ErrorKind::ParseError,
                        message: "Invalid character in document.".to_string(),
                    })
                }
                 */
                ParseError::MissingGenEntity { .. } => Result::Err(Error {
                    kind: ErrorKind::ParseError,
                    message: "Missing Gen Entity.".to_string(),
                }),
                ParseError::MissingParamEntity { .. } => Result::Err(Error {
                    kind: ErrorKind::ParseError,
                    message: "Missing Param Entity.".to_string(),
                }),
                ParseError::EntityDepth { .. } => Result::Err(Error {
                    kind: ErrorKind::ParseError,
                    message: "Entity depth limit exceeded".to_string(),
                }),
                ParseError::Validation { .. } => Result::Err(Error {
                    kind: ErrorKind::ParseError,
                    message: "Validation error.".to_string(),
                }),
                ParseError::Unknown { .. } => Result::Err(Error {
                    kind: ErrorKind::ParseError,
                    message: "Unknown error.".to_string(),
                }),
                ParseError::MissingNameSpace => Result::Err(Error {
                    kind: ErrorKind::ParseError,
                    message: "Missing namespace declaration.".to_string(),
                }),
                ParseError::NotWellFormed => Result::Err(Error {
                    kind: ErrorKind::ParseError,
                    message: "XML document not well formed.".to_string(),
                }),
                ParseError::Notimplemented => Result::Err(Error {
                    kind: ErrorKind::ParseError,
                    message: "Unimplemented feature.".to_string(),
                }),
            }
        }
    }
}

fn document(input: ParseInput) -> ParseResult<XMLDocument> {
    match tuple3(opt(prolog()), element(), opt(misc()))(input) {
        Err(err) => Err(err),
        Ok((mut input1, (p, e, m))) => {
            //Check nothing remaining in iterator, nothing after the end of the root node.
            match input1.next() {
                Some(_) => Err(ParseError::NotWellFormed),
                None => {
                    let pr = p.unwrap_or((None, vec![]));

                    let mut a = DocumentBuilder::new()
                        .prologue(pr.1)
                        .content(vec![e])
                        .epilogue(m.unwrap_or_default())
                        .build();
                    if let Some(x) = pr.0 {
                        a.set_xmldecl(x)
                    };
                    Ok((input1, a))
                }
            }
        }
    }
}

// prolog ::= XMLDecl misc* (doctypedecl Misc*)?
fn prolog() -> impl Fn(ParseInput) -> ParseResult<(Option<XMLDecl>, Vec<RNode>)> {
    map(
        tuple4(opt(xmldecl()), misc(), opt(doctypedecl()), misc()),
        |(xmld, mut m1, _dtd, mut m2)| {
            m1.append(&mut m2);
            (xmld, m1)
        },
    )
}

fn xmldeclversion() -> impl Fn(ParseInput) -> ParseResult<String> {
    move |input| match tuple5(
        tag("version"),
        whitespace0(),
        tag("="),
        whitespace0(),
        delimited_string(),
    )(input)
    {
        Ok((input1, (_, _, _, _, v))) => {
            if v == *"1.1" {
                Ok((input1, v))
            } else if v.starts_with("1.") {
                Ok((input1, "1.0".to_string()))
            } else {
                Err(ParseError::Notimplemented)
            }
        }
        Err(err) => Err(err),
    }
}

fn xmldeclstandalone() -> impl Fn(ParseInput) -> ParseResult<String> {
    map(
        validate(
            tuple6(
                whitespace1(),
                tag("standalone"),
                whitespace0(),
                tag("="),
                whitespace0(),
                delimited_string(),
            ),
            |(_, _, _, _, _, s)| vec!["yes".to_string(), "no".to_string()].contains(s),
        ),
        |(_, _, _, _, _, s)| s,
    )
}

fn xmldecl() -> impl Fn(ParseInput) -> ParseResult<XMLDecl> {
    map(
        tuple8(
            tag("<?xml"),
            whitespace1(),
            xmldeclversion(),
            opt(map(
                tuple6(
                    whitespace1(),
                    tag("encoding"),
                    whitespace0(),
                    tag("="),
                    whitespace0(),
                    delimited_string(),
                ),
                |(_, _, _, _, _, e)| e,
            )),
            opt(xmldeclstandalone()),
            whitespace0(),
            tag("?>"),
            whitespace0(),
        ),
        |(_, _, ver, enc, sta, _, _, _)| XMLDecl {
            version: ver,
            encoding: enc,
            standalone: sta,
        },
    )
}

fn doctypedecl() -> impl Fn(ParseInput) -> ParseResult<()> {
    move |input| match tuple8(
        tag("<!DOCTYPE"),
        whitespace1(),
        name(),
        whitespace1(),
        opt(externalid()),
        whitespace0(),
        opt(delimited(tag("["), intsubset(), tag("]"))),
        tag(">"),
    )(input)
    {
        Ok((d, (_, _, _n, _, _e, _, _inss, _))) => Ok((d, ())),
        Err(err) => Err(err),
    }
}

fn externalid() -> impl Fn(ParseInput) -> ParseResult<(String, Option<String>)> {
    alt2(
        map(
            tuple3(
                tag("SYSTEM"),
                whitespace0(),
                alt2(
                    delimited(tag("'"), take_until("'"), tag("'")),
                    delimited(tag("\""), take_until("\""), tag("\"")),
                ), //SystemLiteral
            ),
            |(_, _, sid)| (sid, None),
        ),
        map(
            tuple5(
                tag("PUBLIC"),
                whitespace0(),
                alt2(
                    delimited(tag("'"), take_while(|c| !is_pubid_char(&c)), tag("'")),
                    delimited(
                        tag("\""),
                        take_while(|c| !is_pubid_charwithapos(&c)),
                        tag("\""),
                    ),
                ), //PubidLiteral TODO validate chars here (PubidChar from spec).
                whitespace1(),
                alt2(
                    delimited(tag("'"), take_until("'"), tag("'")),
                    delimited(tag("\""), take_until("\""), tag("\"")),
                ), //SystemLiteral
            ),
            |(_, _, pid, _, sid)| (sid, Some(pid)),
        ),
    )
}

fn intsubset() -> impl Fn(ParseInput) -> ParseResult<Vec<()>> {
    many0(alt8(
        elementdecl(),
        attlistdecl(),
        pedecl(),
        gedecl(),
        ndatadecl(),
        whitespace1(),
        map(comment(), |_| ()),
        map(processing_instruction(), |_| ()),
    ))
}

//elementdecl	   ::=   	'<!ELEMENT' S Name S contentspec S? '>'
fn elementdecl() -> impl Fn(ParseInput) -> ParseResult<()> {
    move |input| match tuple7(
        tag("<!ELEMENT"),
        whitespace1(),
        qualname(),
        whitespace1(),
        contentspec(), //contentspec - TODO Build out.
        whitespace0(),
        tag(">"),
    )(input)
    {
        Ok((mut d, (_, _, n, _, s, _, _))) => {
            d.dtd.elements.insert(n.to_string(), DTDDecl::Element(n, s));
            Ok((d, ()))
        }
        Err(err) => Err(err),
    }
}
fn contentspec() -> impl Fn(ParseInput) -> ParseResult<String> {
    alt4(
        value(tag("EMPTY"), "EMPTY".to_string()),
        value(tag("ANY"), "ANY".to_string()),
        mixed(),
        children(),
    )
}

//AttlistDecl ::= '<!ATTLIST' S Name AttDef* S? '>'
fn attlistdecl() -> impl Fn(ParseInput) -> ParseResult<()> {
    move |input| match tuple6(
        tag("<!ATTLIST"),
        whitespace1(),
        qualname(),
        many0(attdef()),
        whitespace0(),
        tag(">"),
    )(input)
    {
        Ok((mut d, (_, _, n, _, _, _))) => {
            d.dtd
                .attlists
                .insert(n.to_string(), DTDDecl::Attlist(n, "".to_string()));
            Ok((d, ()))
        }
        Err(err) => Err(err),
    }
}

//AttDef ::= S Name S AttType S DefaultDecl
fn attdef() -> impl Fn(ParseInput) -> ParseResult<String> {
    map(
        tuple6(
            whitespace1(),
            name(),
            whitespace1(),
            atttype(),
            whitespace1(),
            defaultdecl(),
        ),
        |_x| "".to_string(),
    )
}

//AttType ::= StringType | TokenizedType | EnumeratedType
fn atttype() -> impl Fn(ParseInput) -> ParseResult<()> {
    alt3(
        tag("CDATA"), //Stringtype
        alt7(
            //tokenizedtype
            tag("ID"),
            tag("IDREF"),
            tag("IDREFS"),
            tag("ENTITY"),
            tag("ENTITIES"),
            tag("NMTOKENS"),
            tag("NMTOKEN"),
        ),
        enumeratedtype(),
    )
}

//EnumeratedType ::= NotationType | Enumeration
fn enumeratedtype() -> impl Fn(ParseInput) -> ParseResult<()> {
    alt2(notationtype(), enumeration())
}

//NotationType ::= 'NOTATION' S '(' S? Name (S? '|' S? Name)* S? ')'
fn notationtype() -> impl Fn(ParseInput) -> ParseResult<()> {
    map(
        tuple8(
            tag("NOTATION"),
            whitespace1(),
            tag("("),
            whitespace0(),
            name(),
            many0(tuple4(whitespace0(), tag("|"), whitespace0(), name())),
            whitespace0(),
            tag(")"),
        ),
        |_x| (),
    )
}

//Enumeration ::= '(' S? Nmtoken (S? '|' S? Nmtoken)* S? ')'
fn enumeration() -> impl Fn(ParseInput) -> ParseResult<()> {
    map(
        tuple6(
            tag("("),
            whitespace0(),
            nmtoken(),
            many0(tuple4(whitespace0(), tag("|"), whitespace0(), nmtoken())),
            whitespace0(),
            tag(")"),
        ),
        |_x| (),
    )
}

fn nmtoken() -> impl Fn(ParseInput) -> ParseResult<()> {
    map(many1(take_while(|c| is_namechar(&c))), |_x| ())
}

//DefaultDecl ::= '#REQUIRED' | '#IMPLIED' | (('#FIXED' S)? AttValue)
fn defaultdecl() -> impl Fn(ParseInput) -> ParseResult<()> {
    map(
        alt3(
            value(tag("#REQUIRED"), "#REQUIRED".to_string()),
            value(tag("#IMPLIED"), "#IMPLIED".to_string()),
            map(
                tuple2(
                    opt(tuple2(
                        value(tag("#FIXED"), "#FIXED".to_string()),
                        whitespace1(),
                    )),
                    attvalue(),
                ),
                |(x, y)| match x {
                    None => y,
                    Some((mut f, _)) => {
                        f.push_str(&y);
                        f
                    }
                },
            ),
        ),
        |_x| (),
    )
}

//AttValue ::= '"' ([^<&"] | Reference)* '"' | "'" ([^<&'] | Reference)* "'"
fn attvalue() -> impl Fn(ParseInput) -> ParseResult<String> {
    alt2(
        delimited(
            tag("\'"),
            map(
                many0(alt3(
                    take_while(|c| !"&\'<".contains(c)),
                    genentityexpander(),
                    paramentityexpander(),
                )),
                |v| v.join(""),
            ),
            tag("\'"),
        ),
        delimited(
            tag("\""),
            map(
                many0(alt3(
                    take_while(|c| !"&\"<".contains(c)),
                    genentityexpander(),
                    paramentityexpander(),
                )),
                |v| v.join(""),
            ),
            tag("\""),
        ),
    )
}

fn pedecl() -> impl Fn(ParseInput) -> ParseResult<()> {
    move |input| match validate(
        tuple9(
            tag("<!ENTITY"),
            whitespace1(),
            tag("%"),
            whitespace1(),
            qualname(),
            whitespace1(),
            alt2(
                delimited(tag("'"), take_until("'"), tag("'")),
                delimited(tag("\""), take_until("\""), tag("\"")),
            ),
            whitespace0(),
            tag(">"),
        ),
        |(_, _, _, _, _, _, s, _, _)| !s.contains(|c: char| !is_char(&c)),
    )(input)
    {
        Ok((mut d, (_, _, _, _, n, _, s, _, _))) => {
            d.dtd.paramentities.insert(
                n.to_string(),
                DTDDecl::ParamEntity(n, s.replace("&#60;", "<")),
            );
            Ok((d, ()))
        }
        Err(err) => Err(err),
    }
}

fn gedecl() -> impl Fn(ParseInput) -> ParseResult<()> {
    move |input| match validate(
        tuple7(
            tag("<!ENTITY"),
            whitespace1(),
            qualname(),
            whitespace1(),
            alt2(
                delimited(tag("'"), take_until("'"), tag("'")),
                delimited(tag("\""), take_until("\""), tag("\"")),
            ),
            whitespace0(),
            tag(">"),
        ),
        |(_, _, _, _, s, _, _)| !s.contains(|c: char| !is_char(&c)),
    )(input)
    {
        Ok((mut d, (_, _, n, _, s, _, _))) => {
            d.dtd.generalentities.insert(
                n.to_string(),
                DTDDecl::GeneralEntity(n, s.replace("&#60;", "<")),
            );
            Ok((d, ()))
        }
        Err(err) => Err(err),
    }
}
fn ndatadecl() -> impl Fn(ParseInput) -> ParseResult<()> {
    move |input| match tuple7(
        tag("<!NOTATION"),
        whitespace1(),
        qualname(),
        whitespace1(),
        take_until(">"), //contentspec - TODO Build out.
        whitespace0(),
        tag(">"),
    )(input)
    {
        Ok((mut d, (_, _, n, _, s, _, _))) => {
            d.dtd
                .notations
                .insert(n.to_string(), DTDDecl::Notation(n, s));
            Ok((d, ()))
        }
        Err(err) => Err(err),
    }
}

//Mixed	   ::=   	'(' S? '#PCDATA' (S? '|' S? Name)* S? ')*' | '(' S? '#PCDATA' S? ')'
fn mixed() -> impl Fn(ParseInput) -> ParseResult<String> {
    alt2(
        map(
            tuple6(
                tag("("),
                whitespace0(),
                tag("#PCDATA"),
                many0(tuple4(whitespace0(), tag("|"), whitespace0(), name())),
                whitespace0(),
                tag(")*"),
            ),
            |_x| "".to_string(),
        ),
        map(
            tuple5(
                tag("("),
                whitespace0(),
                tag("#PCDATA"),
                whitespace0(),
                tag(")"),
            ),
            |_x| "".to_string(),
        ),
    )
}

// children	   ::=   	(choice | seq) ('?' | '*' | '+')?
fn children() -> impl Fn(ParseInput) -> ParseResult<String> {
    map(
        tuple2(
            alt2(choice(), seq()),
            opt(alt3(tag("?"), tag("*"), tag("+"))),
        ),
        |_x| "".to_string(),
    )
}

// cp	   ::=   	(Name | choice | seq) ('?' | '*' | '+')?
fn cp() -> impl Fn(ParseInput) -> ParseResult<String> {
    move |input| {
        map(
            tuple2(
                alt3(name(), choice(), seq()),
                opt(alt3(tag("?"), tag("*"), tag("+"))),
            ),
            |_x| "".to_string(),
        )(input)
    }
}
//choice	   ::=   	'(' S? cp ( S? '|' S? cp )+ S? ')'
fn choice() -> impl Fn(ParseInput) -> ParseResult<String> {
    move |input| {
        map(
            tuple6(
                tag("("),
                whitespace0(),
                cp(),
                many0(tuple4(whitespace0(), tag("|"), whitespace0(), cp())),
                whitespace0(),
                tag(")"),
            ),
            |_x| "".to_string(),
        )(input)
    }
}

//seq	   ::=   	'(' S? cp ( S? ',' S? cp )* S? ')'
fn seq() -> impl Fn(ParseInput) -> ParseResult<String> {
    map(
        tuple6(
            tag("("),
            whitespace0(),
            cp(),
            many0(tuple4(whitespace0(), tag(","), whitespace0(), cp())),
            whitespace0(),
            tag(")"),
        ),
        |_x| "".to_string(),
    )
}

// Element ::= EmptyElemTag | STag content ETag
fn element() -> impl Fn(ParseInput) -> ParseResult<RNode> {
    move |input|
        //map(
        alt2(
            emptyelem(),
            taggedelem(),
        )
            //,|e| {
            // TODO: Check for namespace declarations, and resolve URIs in the node tree under 'e'
//            e
//        }
            //)
            (input)
}

// EmptyElemTag ::= '<' Name (Attribute)* '/>'
fn emptyelem() -> impl Fn(ParseInput) -> ParseResult<RNode> {
    move |input| {
        match tuple5(
            tag("<"),
            qualname(),
            attributes(), //many0(attribute),
            whitespace0(),
            tag("/>"),
        )(input)
        {
            Ok((mut input1, (_, n, av, _, _))) => {
                let e = NodeBuilder::new(NodeType::Element).name(n.clone()).build();
                match input1.namespace.pop() {
                    None => {
                        //No namespace to assign.
                    }
                    Some(ns) => {
                        let ns_to_check = n.get_prefix().unwrap_or_else(|| "xmlns".to_string());
                        if ns_to_check == *"xml" {
                            e.set_nsuri("http://www.w3.org/XML/1998/namespace".to_string())
                        } else {
                            match ns.get(&*ns_to_check) {
                                None => {
                                    if ns_to_check != *"xmlns" {
                                        return Err(ParseError::MissingNameSpace);
                                    }
                                }
                                Some(nsuri) => e.set_nsuri(nsuri.clone()),
                            }
                        }
                    }
                };
                av.iter()
                    .for_each(|b| e.add_attribute(b.clone()).expect("unable to add attribute"));
                Ok((input1, e))
            }
            Err(err) => Err(err),
        }
    }
}

// STag ::= '<' Name (Attribute)* '>'
// ETag ::= '</' Name '>'
// NB. Names must match
fn taggedelem() -> impl Fn(ParseInput) -> ParseResult<RNode> {
    move |input| {
        match validate(
            tuple10(
                tag("<"),
                qualname(),
                attributes(), //many0(attribute),
                whitespace0(),
                tag(">"),
                content(),
                tag("</"),
                qualname(),
                whitespace0(),
                tag(">"),
            ),
            |(_, n, _a, _, _, _c, _, e, _, _)| n.to_string() == e.to_string(),
        )(input)
        {
            Ok((mut input1, (_, n, av, _, _, c, _, _, _, _))) => {
                let mut e = NodeBuilder::new(NodeType::Element).name(n.clone()).build();
                match input1.namespace.pop() {
                    None => {
                        //No namespace to assign.
                    }
                    Some(ns) => {
                        let ns_to_check = n.get_prefix().unwrap_or_else(|| "xmlns".to_string());
                        if ns_to_check == *"xml" {
                            e.set_nsuri("http://www.w3.org/XML/1998/namespace".to_string())
                        } else {
                            match ns.get(&*ns_to_check) {
                                None => {
                                    if ns_to_check != *"xmlns" {
                                        return Err(ParseError::MissingNameSpace);
                                    }
                                }
                                Some(nsuri) => e.set_nsuri(nsuri.clone()),
                            }
                        }
                    }
                };
                av.iter()
                    .for_each(|b| e.add_attribute(b.clone()).expect("unable to add attribute"));
                c.iter().for_each(|d| {
                    e.push(d.clone()).expect("unable to add node");
                });
                Ok((input1, e))
            }
            Err(err) => Err(err),
        }
        /*
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
        )
        */
    }
}

// QualifiedName
fn qualname() -> impl Fn(ParseInput) -> ParseResult<QualifiedName> {
    alt2(prefixed_name(), unprefixed_name())
}
fn unprefixed_name() -> impl Fn(ParseInput) -> ParseResult<QualifiedName> {
    map(ncname(), |localpart| {
        QualifiedName::new(None, None, localpart)
    })
}
fn prefixed_name() -> impl Fn(ParseInput) -> ParseResult<QualifiedName> {
    map(
        tuple3(ncname(), tag(":"), ncname()),
        |(prefix, _, localpart)| QualifiedName::new(None, Some(prefix), localpart),
    )
}

fn attributes() -> impl Fn(ParseInput) -> ParseResult<Vec<RNode>> {
    move |input| match many0(attribute())(input) {
        Ok((mut input1, nodes)) => {
            let n: HashMap<String, String> = HashMap::new();
            let mut namespaces = input1.namespace.last().unwrap_or(&n).clone();
            for node in nodes.clone() {
                //Return error if someone attempts to redefine namespaces.
                if (node.name().get_prefix() == Some("xmlns".to_string()))
                    && (node.name().get_localname() == *"xmlns")
                {
                    return Err(ParseError::NotWellFormed);
                }
                //xml prefix must always be set to http://www.w3.org/XML/1998/namespace
                if (node.name().get_prefix() == Some("xmlns".to_string()))
                    && (node.name().get_localname() == *"xml")
                    && (node.to_string() != *"http://www.w3.org/XML/1998/namespace")
                {
                    return Err(ParseError::NotWellFormed);
                }

                if (node.name().get_prefix() == Some("xmlns".to_string()))
                    || (node.name().get_localname() == *"xmlns")
                {
                    namespaces.insert(node.name().get_localname(), node.to_string());
                };

                //Check if the xml:space attribute is present and if so, does it have
                //"Preserved" or "Default" as its value. We'll actually handle in a future release.
                if node.name().get_prefix() == Some("xml".to_string())
                    && node.name().get_localname() == *"space"
                    && !(node.to_string() == "Default" || node.to_string() == "Preserve")
                {
                    return Err(ParseError::Validation {
                        row: input1.currentrow,
                        col: input1.currentcol,
                    });
                }
            }
            input1.namespace.push(namespaces.clone());
            //Why loop through the nodes a second time? XML attributes are no in any order, so the
            //namespace declaration can happen after the attribute if it has a namespace prefix.
            let mut resnodes = vec![];
            for node in nodes {
                if node.name().get_prefix() != Some("xmlns".to_string())
                    && node.name().get_localname() != *"xmlns"
                {
                    if let Some(ns) = node.name().get_prefix() {
                        if ns == *"xml" {
                            node.set_nsuri("http://www.w3.org/XML/1998/namespace".to_string())
                        } else {
                            match namespaces.get(&*ns) {
                                None => return Err(ParseError::MissingNameSpace),
                                Some(nsuri) => node.set_nsuri(nsuri.clone()),
                            }
                        }
                    }
                    resnodes.push(node);
                }
            }
            Ok((input1, resnodes))
        }
        Err(err) => Err(err),
    }
}
// Attribute ::= Name '=' AttValue
fn attribute() -> impl Fn(ParseInput) -> ParseResult<RNode> {
    map(
        tuple6(
            whitespace1(),
            qualname(),
            whitespace0(),
            tag("="),
            whitespace0(),
            delimited_string(),
        ),
        |(_, n, _, _, _, s)| {
            NodeBuilder::new(NodeType::Attribute)
                .name(n)
                .value(Value::String(s))
                .build()
        },
    )
}
fn delimited_string() -> impl Fn(ParseInput) -> ParseResult<String> {
    alt2(string_single(), string_double())
}
fn string_single() -> impl Fn(ParseInput) -> ParseResult<String> {
    delimited(
        tag("\'"),
        map(
            many0(alt3(
                chardata_escapes(),
                chardata_unicode_codepoint(),
                take_while(|c| !"&\'<".contains(c)),
            )),
            |v| v.concat(),
        ),
        tag("\'"),
    )
}
fn string_double() -> impl Fn(ParseInput) -> ParseResult<String> {
    delimited(
        tag("\""),
        map(
            many0(alt2(
                chardata_escapes(),
                take_while(|c| !"&\"<".contains(c)),
            )),
            |v| v.concat(),
        ),
        tag("\""),
    )
}

// content ::= CharData? ((element | Reference | CDSect | PI | Comment) CharData?)*
fn content() -> impl Fn(ParseInput) -> ParseResult<Vec<RNode>> {
    map(
        tuple2(
            opt(chardata()),
            many0(tuple2(
                alt4(
                    element(),
                    reference(),
                    // TODO: CData Section
                    processing_instruction(),
                    comment(),
                ),
                opt(chardata()),
            )),
        ),
        |(c, v)| {
            let mut new: Vec<RNode> = Vec::new();
            let mut notex: Vec<String> = Vec::new();

            if let Some(..) = c {
                notex.push(c.unwrap());
            }

            if !v.is_empty() {
                for (w, d) in v {
                    match w.node_type() {
                        NodeType::Text => notex.push(w.to_string()),
                        _ => {
                            if !notex.is_empty() {
                                new.push(
                                    NodeBuilder::new(NodeType::Text)
                                        .value(Value::String(notex.concat()))
                                        .build(),
                                );
                                notex.clear();
                            }
                            new.push(w);
                        }
                    }
                    if let Some(..) = d {
                        notex.push(d.unwrap())
                    }
                }
            }
            if !notex.is_empty() {
                new.push(
                    NodeBuilder::new(NodeType::Text)
                        .value(Value::String(notex.concat()))
                        .build(),
                );
            }
            new
        },
    )
}

// Reference ::= EntityRef | CharRef
// TODO
fn reference() -> impl Fn(ParseInput) -> ParseResult<RNode> {
    map(genentityexpander(), |_| {
        NodeBuilder::new(NodeType::Text)
            .value(Value::from(""))
            .build()
    })
}

// PI ::= '<?' PITarget (char* - '?>') '?>'
fn processing_instruction() -> impl Fn(ParseInput) -> ParseResult<RNode> {
    validate(
        map(
            tuple5(
                tag("<?"),
                name(),
                opt(tuple2(whitespace1(), take_until("?>"))),
                whitespace0(),
                tag("?>"),
            ),
            |(_, n, vt, _, _)| match vt {
                None => NodeBuilder::new(NodeType::ProcessingInstruction)
                    .pi_name(n)
                    .value(Value::String("".to_string()))
                    .build(),
                Some((_, v)) => NodeBuilder::new(NodeType::ProcessingInstruction)
                    .pi_name(n)
                    .value(Value::String(v))
                    .build(),
            },
        ),
        |v| match v.node_type() {
            NodeType::ProcessingInstruction => {
                if v.to_string().contains(|c: char| !is_char(&c)) {
                    false
                } else {
                    v.pi_name().unwrap().to_lowercase() != *"xml"
                    /*
                    match v.name(){
                        QualifiedName {nsuri, prefix, localname} => {
                            localname.to_lowercase() != "xml"
                        }
                    }
                     */
                }
            }
            _ => false,
        },
    )
}

// Comment ::= '<!--' (char* - '--') '-->'
fn comment() -> impl Fn(ParseInput) -> ParseResult<RNode> {
    validate(
        map(
            delimited(tag("<!--"), take_until("--"), tag("-->")),
            |v: String| {
                NodeBuilder::new(NodeType::Comment)
                    .value(Value::String(v))
                    .build()
            },
        ),
        |v| match v.node_type() {
            NodeType::Comment => !v.to_string().contains(|c: char| !is_char(&c)),
            _ => false,
        },
    )
}

// Misc ::= Comment | PI | S
fn misc() -> impl Fn(ParseInput) -> ParseResult<Vec<RNode>> {
    map(
        tuple2(
            many0(map(
                alt2(
                    tuple2(whitespace0(), comment()),
                    tuple2(whitespace0(), processing_instruction()),
                ),
                |(_ws, xn)| xn,
            )),
            whitespace0(),
        ),
        |(v, _)| v,
    )
}

// CharData ::= [^<&]* - (']]>')
fn chardata() -> impl Fn(ParseInput) -> ParseResult<String> {
    validate(
        map(
            many1(alt3(
                chardata_cdata(),
                chardata_escapes(),
                chardata_literal(),
            )),
            |v| v.concat(),
        ),
        |s| !s.contains(|c: char| !is_char(&c)),
    )
}

fn chardata_cdata() -> impl Fn(ParseInput) -> ParseResult<String> {
    delimited(tag("<![CDATA["), take_until("]]>"), tag("]]>"))
}

fn chardata_escapes() -> impl Fn(ParseInput) -> ParseResult<String> {
    move |input| match chardata_unicode_codepoint()(input.clone()) {
        Ok((inp, s)) => Ok((inp, s)),
        Err(e) => match delimited(tag("&"), take_until(";"), tag(";"))(input) {
            Ok((inp, rstr)) => match rstr.as_str() {
                "gt" => Ok((inp, ">".to_string())),
                "lt" => Ok((inp, "<".to_string())),
                "amp" => Ok((inp, "&".to_string())),
                "quot" => Ok((inp, "\"".to_string())),
                "apos" => Ok((inp, "\'".to_string())),
                _ => Err(e),
            },
            Err(e) => Err(e),
        },
    }
}

fn chardata_unicode_codepoint() -> impl Fn(ParseInput) -> ParseResult<String> {
    map(
        alt2(
            delimited(tag("&#x"), parse_hex(), tag(";")),
            delimited(tag("&#"), parse_decimal(), tag(";")),
        ),
        |value| std::char::from_u32(value).unwrap().to_string(),
    )
}
fn parse_hex() -> impl Fn(ParseInput) -> ParseResult<u32> {
    map(
        take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit()),
        |hex| u32::from_str_radix(&hex, 16).unwrap(),
    )
}
fn parse_decimal() -> impl Fn(ParseInput) -> ParseResult<u32> {
    map(take_while_m_n(1, 6, |c: char| c.is_ascii_digit()), |dec| {
        u32::from_str(&dec).unwrap()
    })
}

fn chardata_literal() -> impl Fn(ParseInput) -> ParseResult<String> {
    validate(take_while(|c| c != '<' && c != '&'), |s| {
        !s.contains("]]>") && !s.contains(|c: char| !is_char(&c))
    })
}
