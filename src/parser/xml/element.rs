use crate::item::NodeType;
use crate::parser::combinators::alt::{alt2, alt4};
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::{tuple10, tuple2, tuple5};
use crate::parser::combinators::wellformed::wellformed;
use crate::parser::combinators::whitespace::whitespace0;
use crate::parser::xml::attribute::attributes;
use crate::parser::xml::chardata::chardata;
use crate::parser::xml::misc::{comment, processing_instruction};
use crate::parser::xml::qname::qualname;
use crate::parser::xml::reference::reference;
use crate::parser::{ParseError, ParseInput, ParseResult};
use crate::trees::intmuttree::{NodeBuilder, RNode};
use crate::{Node, Value};

// Element ::= EmptyElemTag | STag content ETag
pub(crate) fn element() -> impl Fn(ParseInput) -> ParseResult<RNode> {
    move |input| alt2(emptyelem(), taggedelem())(input)
}

// EmptyElemTag ::= '<' Name (Attribute)* '/>'
fn emptyelem() -> impl Fn(ParseInput) -> ParseResult<RNode> {
    move |input| {
        match tuple5(
            tag("<"),
            wellformed(qualname(), |qn| {
                qn.get_prefix() != Some("xmlns".to_string())
            }),
            attributes(), //many0(attribute),
            whitespace0(),
            tag("/>"),
        )(input)
        {
            Ok(((input1, mut state1), (_, n, av, _, _))) => {
                let e = NodeBuilder::new(NodeType::Element).name(n.clone()).build();
                match state1.namespace.pop() {
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
                                Some(nsuri) => {
                                    /* In XML 1.1, you cannot set a namespace alias to empty and then use it. */
                                    if ns_to_check != *"xmlns"
                                        && nsuri.is_empty()
                                        && state1.xmlversion == "1.1"
                                    {
                                        return Err(ParseError::NotWellFormed);
                                    }
                                    e.set_nsuri(nsuri.clone())
                                }
                            }
                        }
                    }
                };
                av.iter()
                    .for_each(|b| e.add_attribute(b.clone()).expect("unable to add attribute"));
                Ok(((input1, state1), e))
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
        match wellformed(
            tuple10(
                tag("<"),
                wellformed(qualname(), |qn| {
                    qn.get_prefix() != Some("xmlns".to_string())
                }),
                attributes(), //many0(attribute),
                whitespace0(),
                tag(">"),
                content(),
                tag("</"),
                wellformed(qualname(), |qn| {
                    qn.get_prefix() != Some("xmlns".to_string())
                }),
                whitespace0(),
                tag(">"),
            ),
            |(_, n, _a, _, _, _c, _, e, _, _)| n.to_string() == e.to_string(),
        )(input)
        {
            Ok(((input1, mut state1), (_, n, av, _, _, c, _, _, _, _))) => {
                let mut e = NodeBuilder::new(NodeType::Element).name(n.clone()).build();
                match state1.namespace.pop() {
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
                Ok(((input1, state1), e))
            }
            Err(err) => Err(err),
        }
    }
}

// content ::= CharData? ((element | Reference | CDSect | PI | Comment) CharData?)*
pub(crate) fn content() -> impl Fn(ParseInput) -> ParseResult<Vec<RNode>> {
    map(
        tuple2(
            opt(chardata()),
            many0(tuple2(
                alt4(
                    map(processing_instruction(), |e| vec![e]),
                    map(comment(), |e| vec![e]),
                    map(element(), |e| vec![e]),
                    reference(),
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
                    for x in w {
                        match x.node_type() {
                            NodeType::Text => notex.push(x.to_string()),
                            _ => {
                                if !notex.is_empty() {
                                    new.push(
                                        NodeBuilder::new(NodeType::Text)
                                            .value(Value::String(notex.concat()))
                                            .build(),
                                    );
                                    notex.clear();
                                }
                                new.push(x);
                            }
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
