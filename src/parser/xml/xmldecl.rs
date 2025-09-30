use crate::item::Node;
use crate::parser::combinators::alt::alt2;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::{take_one, take_while};
use crate::parser::combinators::tuple::{tuple2, tuple3, tuple5, tuple6, tuple8};
use crate::parser::combinators::wellformed::wellformed;
use crate::parser::combinators::whitespace::{whitespace0, whitespace1};
use crate::parser::xml::strings::delimited_string;
use crate::parser::{ParseError, ParseInput};
use crate::xmldecl::XMLDecl;

fn xmldeclversion<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, String), ParseError>
{
    move |input| match tuple5(
        tag("version"),
        whitespace0(),
        tag("="),
        whitespace0(),
        delimited_string(),
    )(input)
    {
        Ok((input1, (_, _, _, _, v))) => {
            if v.parse::<f64>().is_ok() {
                if v == *"1.1" {
                    Ok((input1, v))
                } else if v.starts_with("1.") {
                    Ok((input1, "1.0".to_string()))
                } else {
                    Err(ParseError::Notimplemented)
                }
            } else {
                Err(ParseError::NotWellFormed(v))
            }
        }
        Err(err) => Err(err),
    }
}

fn xmldeclstandalone<N: Node>()
-> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, String), ParseError> {
    move |(input, state)| match map(
        wellformed(
            tuple6(
                whitespace1(),
                tag("standalone"),
                whitespace0(),
                tag("="),
                whitespace0(),
                delimited_string(),
            ),
            |(_, _, _, _, _, s)| ["yes".to_string(), "no".to_string()].contains(s),
        ),
        |(_, _, _, _, _, s)| s,
    )((input, state))
    {
        Err(e) => Err(e),
        Ok(((input2, mut state2), sta)) => {
            if &sta == "yes" {
                state2.standalone = true;
            }
            Ok(((input2, state2), sta))
        }
    }
}

pub(crate) fn encodingdecl<N: Node>()
-> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, String), ParseError> {
    map(
        tuple6(
            whitespace1(),
            tag("encoding"),
            whitespace0(),
            tag("="),
            whitespace0(),
            //delimited_string(),
            alt2(
                tuple3(
                    tag("'"),
                    map(
                        tuple2(
                            wellformed(take_one(), |c| is_encname_startchar(*c)),
                            take_while(is_encname_char),
                        ),
                        |(s, r)| [s.to_string(), r].concat(),
                    ),
                    tag("'"),
                ),
                tuple3(
                    tag("\""),
                    map(
                        tuple2(
                            wellformed(take_one(), |c| is_encname_startchar(*c)),
                            take_while(is_encname_char),
                        ),
                        |(s, r)| [s.to_string(), r].concat(),
                    ),
                    tag("\""),
                ),
            ),
        ),
        |(_, _, _, _, _, (_, e, _))| e,
    )
}

pub(crate) fn xmldecl<N: Node>()
-> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, XMLDecl), ParseError> {
    move |(input, state)| match tuple8(
        tag("<?xml"),
        whitespace1(),
        xmldeclversion(),
        opt(encodingdecl()),
        opt(xmldeclstandalone()),
        whitespace0(),
        tag("?>"),
        whitespace0(),
    )((input, state))
    {
        Ok(((input1, mut state1), (_, _, ver, enc, sta, _, _, _))) => {
            state1.xmlversion.clone_from(&ver);
            let res = XMLDecl {
                version: ver,
                encoding: enc,
                standalone: sta,
            };
            Ok(((input1, state1), res))
        }
        Err(e) => Err(e),
    }
}

pub(crate) fn is_encname_char(ch: char) -> bool {
    matches!(ch,
          'a'..='z'
        | 'A'..='Z'
        | '0'..='9'
        | '-'
        | '_'
        | '.'
    )
}

pub(crate) fn is_encname_startchar(ch: char) -> bool {
    ch.is_ascii_alphabetic()
}
