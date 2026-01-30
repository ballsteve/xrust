use crate::Node;
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
use crate::parser::{ParseError, ParseInput, StaticState};
use qualname::{NamespacePrefix, NamespaceUri};

pub(crate) fn externalid<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, ()), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), ss| {
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
        )((input, state), ss)
        {
            Err(e) => Err(e),
            Ok(((input2, mut state2), (sid, _))) => {
                if !state2.currentlyexternal {
                    state2.ext_entities_to_parse.push(sid);
                    Ok(((input2, state2), ()))
                } else {
                    match ss.resolve(state2.docloc.clone(), sid) {
                        Err(_) => Err(ParseError::ExtDTDLoadError),
                        Ok(s) => match extsubset()((s.as_str(), state2), ss) {
                            Err(e) => Err(e),
                            Ok(((_, state3), _)) => Ok(((input2, state3), ())),
                        },
                    }
                }
            }
        }
    }
}

pub(crate) fn textexternalid<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), ss| {
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
        )((input, state), ss)
        {
            Err(e) => Err(e),
            Ok(((input2, state2), (sid, _pid))) => match ss.resolve(state2.docloc.clone(), sid) {
                Err(_) => Err(ParseError::ExtDTDLoadError),
                Ok(s) => {
                    if state2.xmlversion == "1.1" {
                        s.replace("\r\n", "\n")
                            .replace("\r\u{85}", "\n")
                            .replace("\u{85}", "\n")
                            .replace("\u{2028}", "\n")
                            .replace("\r", "\n")
                    } else {
                        s.replace("\r\n", "\n").replace('\r', "\n")
                    };
                    match opt(textdecl())((s.as_str(), state2.clone()), ss) {
                        Err(_) => Ok(((input2, state2), s.clone())),
                        Ok(((i3, _), _)) => Ok(((input2, state2), i3.to_string())),
                    }
                }
            },
        }
    }
}
