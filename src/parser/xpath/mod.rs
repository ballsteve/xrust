//! # xrust::parser::xpath
//!
//! An XPath parser using the xrust parser combinator that produces a xrust transformation.

mod flwr;
mod support;
pub(crate) mod nodetests;
mod logic;
mod compare;
mod strings;
mod numbers;
mod nodes;
mod types;
mod functions;
mod expressions;
pub(crate) mod literals;
mod context;
pub(crate) mod variables;
pub(crate) mod predicates;

use crate::parser::{ParseError, ParseInput, ParseResult, ParserState};
use crate::parser::combinators::map::map;
use crate::parser::combinators::alt::alt4;
use crate::parser::combinators::list::separated_list1;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::tuple3;
use crate::parser::combinators::whitespace::xpwhitespace;
use crate::parser::xpath::flwr::{if_expr, for_expr, let_expr};
use crate::parser::xpath::logic::or_expr;
use crate::parser::xpath::support::noop;

use crate::xdmerror;
use crate::item::Node;
use crate::transform::Transform;

pub fn parse<N: Node>(
    input: &str,
) -> Result<Transform<N>, xdmerror::Error> {
    let state = ParserState::new(None, None);
    match xpath_expr((input, state)) {
        Ok((_, x)) => Result::Ok(x),
        Err(err) => {
            match err {
                ParseError::Combinator => Result::Err(xdmerror::Error {
                    kind: xdmerror::ErrorKind::ParseError,
                    message: "Unrecoverable parser error.".to_string(),
                }),
                ParseError::NotWellFormed => Result::Err(xdmerror::Error {
                    kind: xdmerror::ErrorKind::ParseError,
                    message: "Unrecognised extra characters.".to_string(),
                }),
                ParseError::MissingNameSpace => Result::Err(xdmerror::Error {
                    kind: xdmerror::ErrorKind::ParseError,
                    message: "Missing namespace declaration.".to_string(),
                }),
                ParseError::Notimplemented => Result::Err(xdmerror::Error {
                    kind: xdmerror::ErrorKind::ParseError,
                    message: "Unimplemented feature.".to_string(),
                }),
                _ => Err(xdmerror::Error {
                    kind: xdmerror::ErrorKind::Unknown,
                    message: "Unknown error".to_string(),
                }),
            }
        }
    }
}

fn xpath_expr<N: Node>(input: ParseInput) -> ParseResult<Transform<N>> {
    match expr::<N>()(input) {
        Err(err) => Err(err),
        Ok(((input1, state1), e)) => {
            //Check nothing remaining in iterator, nothing after the end of the root node.
            if input1.is_empty() {
                Ok(((input1, state1), e))
            } else {
                Err(ParseError::NotWellFormed)
            }
        }
    }
}
// Implementation note: cannot use opaque type because XPath expressions are recursive, and Rust *really* doesn't like recursive opaque types. Dynamic trait objects aren't ideal, but compiling XPath expressions is a one-off operation so that shouldn't cause a major performance issue.
// Implementation note 2: since XPath is recursive, must lazily evaluate arguments to avoid stack overflow.
fn expr<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Transform<N>> + 'a {
    Box::new(map(
        separated_list1(
            map(tuple3(xpwhitespace(), tag(","), xpwhitespace()), |_| ()),
            expr_single::<N>(),
        ),
        |mut v| {
            if v.len() == 1 {
                v.pop().unwrap()
            } else {
                Transform::SequenceItems(v)
            }
        },
    ))
}

pub(crate) fn expr_wrapper<N: Node>(
    b: bool,
) -> impl Fn(ParseInput) -> ParseResult<Transform<N>> {
    move |input| {
        if b {
            expr::<N>()(input)
        } else {
            noop::<N>()(input)
        }
    }
}

// ExprSingle ::= ForExpr | LetExpr | QuantifiedExpr | IfExpr | OrExpr
fn expr_single<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Transform<N>> + 'a {
    alt4(
        or_expr::<N>(),
        let_expr::<N>(),
        for_expr::<N>(),
        if_expr::<N>(),
    )
}

pub(crate) fn expr_single_wrapper<N: Node>(
    b: bool,
) -> impl Fn(ParseInput) -> ParseResult<Transform<N>>
{
    move |input| {
        if b {
            expr_single::<N>()(input)
        } else {
            noop::<N>()(input)
        }
    }
}
