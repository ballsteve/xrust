use crate::intmuttree::RNode;
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_until;
use crate::parser::{ParseError, ParseInput, ParseResult};
use crate::parser::xml::dtd::extsubset::extsubsetdecl;

pub(crate) fn pereference() -> impl Fn(ParseInput) -> ParseResult<()> {
    move |(input, state)| {
        let e = delimited(tag("%"), take_until(";"), tag(";"))((input, state));
        match e {
            Err(e) => Err(e),
            Ok(((input1, state1), entitykey)) => {
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

                            /*
                            We want to reuse the "Content" combinator to parse the entity, but
                            that function parses everything up until the closing tag of an XML element.
                            The fix? We append a < character and the parser will stop as if its hit that
                            closing tag. Then we check that that closing tag is all that remained on the parsing.
                             */
                            let mut e2 = entval.clone();

                            match extsubsetdecl()((e2.as_str(), tempstate)) {
                                Ok(((outstr, _), _)) => {
                                    if !outstr.is_empty() {
                                        Err(ParseError::NotWellFormed)
                                    } else {
                                        Ok(((input1, state1), ()))
                                    }
                                }
                                Err(e) => Err(ParseError::NotWellFormed),
                            }
                        }
                    }
                    None => Err(ParseError::MissingParamEntity {
                        col: state1.currentcol,
                        row: state1.currentrow,
                    }),
                    _ => Err(ParseError::Unknown {
                        col: state1.currentcol,
                        row: state1.currentrow,
                    }),
                }
            }
        }
    }
}
