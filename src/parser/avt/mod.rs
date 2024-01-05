/*!
Parse an Attribute Value Template.

See XSL Transformations v3.0 5.6.1.
 */

use crate::item::{Item, Node};
use crate::parser::combinators::alt::alt2;
use crate::parser::combinators::many::{many0, many1};
use crate::parser::combinators::map::map;
use crate::parser::{ParseError, ParseInput, ParseResult, ParserState};
use crate::value::Value;
use crate::xdmerror::{Error, ErrorKind};
use std::rc::Rc;
//use crate::parser::combinators::debug::inspect;
use crate::parser::xpath::expr;
use crate::parser::xpath::support::none_of;
use crate::transform::Transform;

/// AVT ::= text* "{" xpath "}" text*
pub fn parse<N: Node>(input: &str) -> Result<Transform<N>, Error> {
    let state = ParserState::new(None, None);
    match avt_expr((input, state)) {
        Ok((_, x)) => Ok(x),
        Err(err) => match err {
            ParseError::Combinator => Result::Err(Error::new(
                ErrorKind::ParseError,
                "Unrecoverable parser error.".to_string(),
            )),
            ParseError::NotWellFormed => Result::Err(Error::new(
                ErrorKind::ParseError,
                "Unrecognised extra characters.".to_string(),
            )),
            ParseError::Notimplemented => Result::Err(Error::new(
                ErrorKind::ParseError,
                "Unimplemented feature.".to_string(),
            )),
            _ => Err(Error::new(ErrorKind::Unknown, "Unknown error".to_string())),
        },
    }
}

fn avt_expr<N: Node>(input: ParseInput) -> ParseResult<Transform<N>> {
    match avt::<N>()(input) {
        Err(err) => Err(err),
        Ok(((input1, state1), e)) => {
            //Check nothing remaining in iterator, nothing after the end of the AVT.
            if input1.is_empty() {
                Ok(((input1, state1), e))
            } else {
                Err(ParseError::NotWellFormed)
            }
        }
    }
}

fn avt<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        many0(alt2(
            map(many1(none_of("{")), |v| {
                Transform::Literal(Item::Value(Rc::new(Value::from(
                    v.iter().collect::<String>(),
                ))))
            }),
            braced_expr(),
        )),
        |mut v| {
            if v.len() == 1 {
                v.pop().unwrap()
            } else {
                Transform::SequenceItems(v)
            }
        },
    ))
}

/// A XPath expression in the AVT. Braces do not nest.
fn braced_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    // Can't use combinator directly, since the close brace will be unexpected.
    // Instead, extract the string up to the close brace, then feed that to the combinator.
    //    Box::new(map(
    //        tuple3(
    //            inspect("brace-open", tag("{")),
    //            inspect("expr",expr()),
    //            inspect("brace-close", tag("}")),
    //        ),
    //        |(_, e, _)| e
    //    ))
    Box::new(move |(input, state)| match input.get(0..1) {
        Some("{") => match input.find("}") {
            None => Err(ParseError::Combinator),
            Some(ind) => match expr()((input.get(1..ind).unwrap(), state.clone())) {
                Ok((_, result)) => Ok(((input.get(ind..).map_or("", |r| r), state), result)),
                Err(e) => Err(e),
            },
        },
        _ => Err(ParseError::Combinator),
    })
}
