use crate::item::{Node, NodeType};
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_until;
use crate::parser::xml::dtd::extsubset::extsubset;
use crate::parser::xml::element::content;
use crate::parser::{ParseError, ParseInput};
use crate::value::Value;
use std::rc::Rc;

// Reference ::= EntityRef | CharRef
// \Its important to note, we pre-populate the standard char references in the DTD.
pub(crate) fn reference<N: Node>()
-> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, Vec<N>), ParseError> {
    move |(input, state)| {
        let e = delimited(tag("&"), take_until(";"), tag(";"))((input, state.clone()));
        match e {
            Err(e) => Err(e),
            Ok(((input1, mut state1), entitykey)) => {
                match entitykey.as_str() {
                    "amp" => Ok((
                        (input1, state1),
                        vec![
                            state
                                .doc
                                .clone()
                                .unwrap()
                                .new_text(Rc::new(Value::from("&".to_string())))
                                .expect("unable to create text node"),
                        ],
                    )),
                    "gt" => Ok((
                        (input1, state1),
                        vec![
                            state
                                .doc
                                .clone()
                                .unwrap()
                                .new_text(Rc::new(Value::from(">".to_string())))
                                .expect("unable to create text node"),
                        ],
                    )),
                    "lt" => Ok((
                        (input1, state1),
                        vec![
                            state
                                .doc
                                .clone()
                                .unwrap()
                                .new_text(Rc::new(Value::from("<".to_string())))
                                .expect("unable to create text node"),
                        ],
                    )),
                    "quot" => Ok((
                        (input1, state1),
                        vec![
                            state
                                .doc
                                .clone()
                                .unwrap()
                                .new_text(Rc::new(Value::from("\"".to_string())))
                                .expect("unable to create text node"),
                        ],
                    )),
                    "apos" => Ok((
                        (input1, state1),
                        vec![
                            state
                                .doc
                                .clone()
                                .unwrap()
                                .new_text(Rc::new(Value::from("'".to_string())))
                                .expect("unable to create text node"),
                        ],
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
                                                Err(ParseError::NotWellFormed(outstr.to_string()))
                                            } else {
                                                Ok(((input1, state1), nodes))
                                            }
                                        }
                                        Err(_) => Err(ParseError::NotWellFormed(e2)),
                                    }
                                }
                            }
                            None => {
                                /* Check if any unparsed DTDs, if so parse and try again. */
                                match state1.ext_entities_to_parse.pop() {
                                    None => Err(ParseError::MissingGenEntity {
                                        col: state1.currentcol,
                                        row: state1.currentrow,
                                    }),
                                    Some(sid) => {
                                        match state1.clone().resolve(state1.docloc.clone(), sid) {
                                            Err(_) => Err(ParseError::ExtDTDLoadError),
                                            Ok(s) => {
                                                match extsubset()((s.as_str(), state1)) {
                                                    Err(e) => Err(e),
                                                    Ok(((_, state2), _)) => {
                                                        match state2
                                                            .clone()
                                                            .dtd
                                                            .generalentities
                                                            .get(&entitykey as &str)
                                                        {
                                                            Some((entval, _)) => {
                                                                if state2.currententitydepth
                                                                    >= state2.maxentitydepth
                                                                {
                                                                    //attempting to exceed expansion depth
                                                                    Err(ParseError::EntityDepth {
                                                                        col: state2.currentcol,
                                                                        row: state2.currentrow,
                                                                    })
                                                                } else {
                                                                    //Parse the entity, using the parserstate which has information on namespaces
                                                                    let mut tempstate =
                                                                        state2.clone();
                                                                    tempstate.currententitydepth +=
                                                                        1;

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
                                                                                Err(ParseError::NotWellFormed(outstr.to_string()))
                                                                            } else {
                                                                                Ok(((input1, state2), nodes))
                                                                            }
                                                                        }
                                                                        Err(_) => Err(ParseError::NotWellFormed(e2)),
                                                                    }
                                                                }
                                                            }
                                                            None => {
                                                                Err(ParseError::MissingGenEntity {
                                                                    col: state2.currentcol,
                                                                    row: state2.currentrow,
                                                                })
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn textreference<N: Node>()
-> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, String), ParseError> {
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
                                                Err(ParseError::NotWellFormed(outstr.to_string()))
                                            } else {
                                                let mut res = vec![];
                                                for rn in nodes {
                                                    match rn.node_type() {
                                                        NodeType::Text => res.push(rn.to_string()),
                                                        _ => {
                                                            return Err(ParseError::NotWellFormed(
                                                                String::from("not a text node"),
                                                            ));
                                                        }
                                                    }
                                                }
                                                Ok(((input1, state1), res.concat()))
                                            }
                                        }
                                        Err(_) => Err(ParseError::NotWellFormed(e2)),
                                    }
                                }
                            }
                            None => Err(ParseError::MissingGenEntity {
                                col: state1.currentcol,
                                row: state1.currentrow,
                            }),
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
