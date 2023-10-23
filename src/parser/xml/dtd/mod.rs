mod intsubset;
mod attlistdecl;
mod elementdecl;
mod enumerated;
mod gedecl;
mod misc;
mod notation;
mod pedecl;
pub(crate) mod extsubset;
mod textdecl;
pub(crate) mod pereference;
mod conditionals;

use crate::parser::combinators::alt::alt2;
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::{take_until, take_while};
use crate::parser::combinators::tuple::{tuple3, tuple5, tuple8};
use crate::parser::combinators::whitespace::{whitespace0, whitespace1};
use crate::parser::common::{is_pubid_char, is_pubid_charwithapos};
use crate::parser::xml::dtd::intsubset::intsubset;
use crate::parser::xml::qname::name;
use crate::parser::{ParseError, ParseInput, ParseResult};
use crate::parser::xml::dtd::extsubset::extsubset;
use crate::parser::xml::dtd::textdecl::textdecl;
use crate::parser::xml::reference::reference;

pub(crate) fn doctypedecl() -> impl Fn(ParseInput) -> ParseResult<()> {
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
        Ok(((input1, state1), (_, _, _n, _, _, _, _inss, _))) => {
            /*  We're doing nothing with the below, just evaluating the external entity to check its well formed */
            let exdtd = state1.ext_entities_to_parse.clone().pop();
            match exdtd {
                None => {}
                Some(s) => {
                    match state1.clone().resolve(state1.docloc.clone(), s) {
                        Err(_) => {
                            return Err(ParseError::ExtDTDLoadError)
                        }
                        Ok(s) => {
                            match extsubset()
                                ((s.as_str(), state1.clone())){
                                Err(e) => { return Err(e)}
                                _ => {}
                            }
                        }
                    }
                }
            }
            /*
             Same again, with Internal subset */
            for (k, (v, _)) in state1.clone().dtd.generalentities {
                if v != "<".to_string(){ /* A single < on its own will generate an error if used, but doesn't actually generate a not well formed error! */
                    match reference()((["&".to_string(), k, ";".to_string()].join("").as_str(), state1.clone())){
                        Err(ParseError::NotWellFormed) => { return Err(ParseError::NotWellFormed)}
                        _ => {}
                    }
                }
            }
            Ok(((input1, state1), ()))
        }
        Err(err) => Err(err),
    }
}

fn externalid() -> impl Fn(ParseInput) -> ParseResult<()> {
    move |(input, state)| {
        match alt2(
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
                        delimited(tag("'"), take_while(|c| is_pubid_char(&c)), tag("'")),
                        delimited(
                            tag("\""),
                            take_while(|c| is_pubid_charwithapos(&c)),
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
        )((input, state))
        {
            Err(e) => Err(e),
            Ok(((input2, mut state2), (sid, _))) => {
                if !state2.currentlyexternal{
                    state2.ext_entities_to_parse.push(sid);
                    Ok(((input2, state2), ()))
                } else {
                    match state2.clone().resolve(state2.docloc.clone(), sid.clone()) {
                        Err(_) => {
                            Err(ParseError::ExtDTDLoadError)
                        }
                        Ok(s) => {
                            match extsubset()
                                ((s.as_str(), state2)){
                                Err(e) => {Err(e)}
                                Ok(((_, state3), _)) => {
                                    Ok(((input2, state3), ()))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn textexternalid() -> impl Fn(ParseInput) -> ParseResult<String> {
    move |(input, state)| {
        match alt2(
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
                    whitespace1(),
                    alt2(
                        delimited(
                            tag("'"),
                            take_while(|c| is_pubid_char(&c)),
                            tag("'")),
                        delimited(
                            tag("\""),
                            take_while(|c| is_pubid_charwithapos(&c)),
                            tag("\""),
                        ),
                    ), //PubidLiteral TODO validate chars here (PubidChar from spec).
                    whitespace1(),
                    alt2(
                        delimited(tag("'"), take_until("'"), tag("'")),
                        delimited(tag("\""), take_until("\""), tag("\"")),
                    ), //SystemLiteral
                ),
                |(_, _, pid, _, sid)| {
                    (sid, Some(pid))
                },
            ),
        )((input, state))
        {
            Err(e) => Err(e),
            Ok(((input2, state2), (sid, _pid))) => {
                match state2.clone().resolve(state2.docloc.clone(), sid) {
                    Err(_) => {
                        Err(ParseError::ExtDTDLoadError)
                    }
                    Ok(s) => {
                        match opt(textdecl())((s.replace("\r\n", "\n").replace('\r', "\n").as_str(), state2.clone())){
                            Err(_) => {Ok(((input2, state2), s))},
                            Ok(((i3, _), _)) => {
                                Ok(((input2, state2), i3.to_string()))
                            }
                        }
                    }
                }
            }
        }
    }
}
