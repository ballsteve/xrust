/*!
Parse an Attribute Value Template.

See XSL Transformations v3.0 5.6.1.
 */

use crate::item::{Item, Node};
use crate::parser::combinators::alt::alt2;
use crate::parser::combinators::many::{many0, many1};
use crate::parser::combinators::map::map;
use crate::parser::{
    ParseError, ParseInput, ParserState, ParserStateBuilder, StaticState, StaticStateBuilder,
};
use crate::value::Value;
use crate::xdmerror::{Error, ErrorKind};
use std::rc::Rc;
//use crate::parser::combinators::debug::inspect;
use crate::parser::combinators::support::none_of;
use crate::parser::xpath::expr;
use crate::transform::Transform;
use qualname::{NamespacePrefix, NamespaceUri};

/// AVT ::= text* "{" xpath "}" text*
/// A [Node] is required to resolve in-scope XML Namespaces
pub fn parse<N: Node>(input: &str, n: Option<N>) -> Result<Transform<N>, Error> {
    let state = n.map_or_else(
        || ParserState::new(),
        |m| ParserStateBuilder::new().doc(m).build(),
    );
    let mut static_state = StaticStateBuilder::new()
        .namespace(|_| {
            NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
        })
        .build();
    match avt_expr((input, state), &mut static_state) {
        Ok((_, x)) => Ok(x),
        Err(err) => match err {
            ParseError::Combinator(f) => Result::Err(Error::new(
                ErrorKind::ParseError,
                format!("Unrecoverable parser error ({})", f),
            )),
            ParseError::NotWellFormed(e) => Result::Err(Error::new(
                ErrorKind::ParseError,
                format!("Unrecognised extra characters: \"{}\"", e),
            )),
            ParseError::Notimplemented => Result::Err(Error::new(
                ErrorKind::ParseError,
                "Unimplemented feature.".to_string(),
            )),
            _ => Err(Error::new(ErrorKind::Unknown, "Unknown error".to_string())),
        },
    }
}

fn avt_expr<'a, N: Node + 'a, L>(
    input: ParseInput<'a, N>,
    ss: &mut StaticState<L>,
) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    match avt::<N, L>()(input, ss) {
        Err(err) => Err(err),
        Ok(((input1, state1), e)) => {
            //Check nothing remaining in iterator, nothing after the end of the AVT.
            if input1.is_empty() {
                Ok(((input1, state1), e))
            } else {
                Err(ParseError::NotWellFormed(format!(
                    "unexpected extra characters: \"{}\"",
                    input1
                )))
            }
        }
    }
}

fn avt<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
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
fn braced_expr<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
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
    Box::new(move |(input, state), ss| {
        match input.get(0..1) {
            Some("{") => match input.find('}') {
                None => Err(ParseError::Combinator(String::from("no closing brace"))),
                Some(ind) => match expr()((input.get(1..ind).unwrap(), state.clone()), ss) {
                    Ok((_, result)) => {
                        // Successful parse of expression
                        // Must also consume the close brace
                        Ok(((input.get((ind + 1)..).map_or("", |r| r), state), result))
                    }
                    Err(e) => Err(e),
                },
            },
            _ => Err(ParseError::Combinator(String::from(
                "closing brace not found",
            ))),
        }
    })
}
