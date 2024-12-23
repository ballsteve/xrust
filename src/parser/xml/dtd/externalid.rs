use crate::Node;
use crate::parser::{ParseError, ParseInput};
use crate::parser::combinators::alt::alt2;
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::{take_until, take_while};
use crate::parser::combinators::tuple::{tuple3, tuple5};
use crate::parser::combinators::whitespace::{whitespace0, whitespace1};
use crate::parser::common::{is_pubid_char, is_pubid_charwithapos};
use crate::parser::xml::dtd::extsubset::extsubset;
use crate::parser::xml::dtd::textdecl::textdecl;

pub(crate) fn externalid<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, ()), ParseError> {
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
                if !state2.currentlyexternal {
                    state2.ext_entities_to_parse.push(sid);
                    Ok(((input2, state2), ()))
                } else {
                    match state2.clone().resolve(state2.docloc.clone(), sid) {
                        Err(_) => Err(ParseError::ExtDTDLoadError),
                        Ok(s) => match extsubset()((s.as_str(), state2)) {
                            Err(e) => Err(e),
                            Ok(((_, state3), _)) => Ok(((input2, state3), ())),
                        },
                    }
                }
            }
        }
    }
}

pub(crate) fn textexternalid<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, String), ParseError>
{
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
            Ok(((input2, state2), (sid, _pid))) => {
                match state2.clone().resolve(state2.docloc.clone(), sid) {
                    Err(_) => Err(ParseError::ExtDTDLoadError),
                    Ok(s) => {
                        match opt(textdecl())((
                            s.replace("\r\n", "\n").replace('\r', "\n").as_str(),
                            state2.clone(),
                        )) {
                            Err(_) => Ok(((input2, state2), s)),
                            Ok(((i3, _), _)) => Ok(((input2, state2), i3.to_string())),
                        }
                    }
                }
            }
        }
    }
}
