use crate::intmuttree::{NodeBuilder, RNode};
use crate::item::NodeType;
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_until;
use crate::parser::xml::element::content;
use crate::parser::ParseError::NotWellFormed;
use crate::parser::{ParseError, ParseInput, ParseResult};
use crate::{Node, Value};

// Reference ::= EntityRef | CharRef
// \Its important to note, we pre-populate the standard char references in the DTD.
pub(crate) fn reference() -> impl Fn(ParseInput) -> ParseResult<Vec<RNode>> {
    move |(input, state)| {
        let e = delimited(tag("&"), take_until(";"), tag(";"))((input, state));
        match e {
            Err(e) => Err(e),
            Ok(((input1, state1), entitykey)) => {
                match entitykey.as_str() {
                    "amp" => Ok((
                        (input1, state1),
                        vec![NodeBuilder::new(NodeType::Text)
                            .value(Value::String("&".to_string()))
                            .build()],
                    )),
                    "gt" => Ok((
                        (input1, state1),
                        vec![NodeBuilder::new(NodeType::Text)
                            .value(Value::String(">".to_string()))
                            .build()],
                    )),
                    "lt" => Ok((
                        (input1, state1),
                        vec![NodeBuilder::new(NodeType::Text)
                            .value(Value::String("<".to_string()))
                            .build()],
                    )),
                    "quot" => Ok((
                        (input1, state1),
                        vec![NodeBuilder::new(NodeType::Text)
                            .value(Value::String("\"".to_string()))
                            .build()],
                    )),
                    "apos" => Ok((
                        (input1, state1),
                        vec![NodeBuilder::new(NodeType::Text)
                            .value(Value::String("'".to_string()))
                            .build()],
                    )),
                    _ => {
                        match state1.clone().dtd.generalentities.get(&entitykey as &str) {
                            Some((entval, _)) => {
                                if state1.currententitydepth >= state1.maxentitydepth {
                                    //attempting to exceed expansion depth
                                    Err(ParseError::EntityDepth {
                                        col: state1.currentcol,
                                        row: state1.currentrow,
                                    })
                                } else {
                                    //Parse the entity, using the parserstate which has information on namespaces
                                    let mut tempstate = state1.clone();
                                    tempstate.currententitydepth += 1;

                                    /*
                                    We want to reuse the "Content" combinator to parse the entity, but
                                    that function parses everything up until the closing tag of an XML element.
                                    The fix? We append a < character and the parser will stop as if its hit that
                                    closing tag. Then we check that that closing tag is all that remained on the parsing.
                                     */
                                    let mut e2 = entval.clone();
                                    e2.push('<');

                                    match content()((e2.as_str(), tempstate)) {
                                        Ok(((outstr, _), nodes)) => {
                                            if outstr != "<" {
                                                Err(NotWellFormed)
                                            } else {
                                                Ok(((input1, state1), nodes))
                                            }
                                        }
                                        Err(_) => Err(NotWellFormed),
                                    }
                                }
                            }
                            None => Err(ParseError::MissingGenEntity {
                                col: state1.currentcol,
                                row: state1.currentrow,
                            })
                        }
                        //} else {
                        //    Err(ParseError::Combinator)
                        //}
                    }
                }
            }
        }
    }
}

pub(crate) fn textreference() -> impl Fn(ParseInput) -> ParseResult<String> {
    move |(input, state)| {
        let e = delimited(tag("&"), take_until(";"), tag(";"))((input, state));
        match e {
            Err(e) => Err(e),
            Ok(((input1, state1), entitykey)) => {
                //if !["lt", "gt", "apos", "amp", "quot"].contains(&entitykey.as_str()){
                match entitykey.as_str() {
                    "amp" => Ok(((input1, state1), "&".to_string())),
                    "gt" => Ok(((input1, state1), ">".to_string())),
                    "lt" => Ok(((input1, state1), "<".to_string())),
                    "quot" => Ok(((input1, state1), "\"".to_string())),
                    "apos" => Ok(((input1, state1), "'".to_string())),
                    _ => {
                        match state1.clone().dtd.generalentities.get(&entitykey as &str) {
                            Some((entval, _)) => {
                                if state1.currententitydepth >= state1.maxentitydepth {
                                    //attempting to exceed expansion depth
                                    Err(ParseError::EntityDepth {
                                        col: state1.currentcol,
                                        row: state1.currentrow,
                                    })
                                } else {
                                    //Parse the entity, using the parserstate which has information on namespaces
                                    let mut tempstate = state1.clone();
                                    tempstate.currententitydepth += 1;

                                    /*
                                    We want to reuse the "Content" combinator to parse the entity, but
                                    that function parses everything up until the closing tag of an XML element.
                                    The fix? We append a < character and the parser will stop as if its hit that
                                    closing tag. Then we check that that closing tag is all that remained on the parsing.
                                     */
                                    let mut e2 = entval.clone();
                                    e2.push('<');

                                    match content()((e2.as_str(), tempstate)) {
                                        Ok(((outstr, _), nodes)) => {
                                            if outstr != "<" {
                                                Err(NotWellFormed)
                                            } else {
                                                let mut res = vec![];
                                                for rn in nodes {
                                                    match rn.node_type() {
                                                        NodeType::Text => res.push(rn.to_string()),
                                                        _ => return Err(NotWellFormed),
                                                    }
                                                }
                                                Ok(((input1, state1), res.concat()))
                                            }
                                        }
                                        Err(_) => Err(NotWellFormed),
                                    }
                                }
                            }
                            None => Err(ParseError::MissingGenEntity {
                                col: state1.currentcol,
                                row: state1.currentrow,
                            })
                        }
                        //} else {
                        //    Err(ParseError::Combinator)
                        //}
                    }
                }
            }
        }
    }
}
