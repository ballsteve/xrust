use crate::item::Node;
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_until;
use crate::parser::xml::dtd::extsubset::extsubsetdecl;
use crate::parser::{ParseError, ParseInput, StaticState};
use qualname::{NamespacePrefix, NamespaceUri};

pub(crate) fn pereference<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, ()), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), ss| {
        let e = delimited(tag("%"), take_until(";"), tag(";"))((input, state), ss);
        match e {
            Err(e) => Err(e),
            Ok(((input1, state1), entitykey)) => {
                //match state1.currentlyexternal {
                //    /* Are we in an external DTD? Param entities not allowed anywhere else. */
                //    false => Err(ParseError::NotWellFormed),
                //    true => {
                match state1.clone().dtd.paramentities.get(&entitykey as &str) {
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

                            let e2 = entval.clone();
                            match extsubsetdecl()((e2.as_str(), tempstate), ss) {
                                Ok(((outstr, _), _)) => {
                                    if !outstr.is_empty() {
                                        Err(ParseError::NotWellFormed(outstr.to_string()))
                                    } else {
                                        Ok(((input1, state1), ()))
                                    }
                                }
                                Err(_) => Err(ParseError::NotWellFormed(e2.clone())),
                            }
                        }
                    }
                    None => Err(ParseError::MissingParamEntity {
                        col: state1.currentcol,
                        row: state1.currentrow,
                    }),
                }
                //    }
                //}
            }
        }
    }
}

pub(crate) fn petextreference<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), ss| {
        let e = delimited(tag("%"), take_until(";"), tag(";"))((input, state), ss);
        match e {
            Err(e) => Err(e),
            Ok(((input1, state1), entitykey)) => {
                match state1.currentlyexternal {
                    /* Are we in an external DTD? Param entities not allowed anywhere else. */
                    false => Err(ParseError::NotWellFormed(String::from(
                        "parameter entity not allowed outside of external DTD",
                    ))),
                    true => {
                        match state1.clone().dtd.paramentities.get(&entitykey as &str) {
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
                                    Ok(((input1, state1), entval.clone()))
                                }
                            }
                            None => Err(ParseError::MissingParamEntity {
                                col: state1.currentcol,
                                row: state1.currentrow,
                            }),
                        }
                    }
                }
            }
        }
    }
}
