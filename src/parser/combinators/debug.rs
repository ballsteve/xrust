use crate::parser::{ParseError, ParseInput, ParseResult};
use std::fmt::Debug;

/// Emits a message to stderr from within the parser combinator. This can be useful for debugging.
#[allow(dead_code)]
pub(crate) fn inspect<'a, P1, A>(
    msg: &'a str,
    parser: P1,
) -> impl Fn(ParseInput) -> ParseResult<A> + '_
where
    P1: Fn(ParseInput) -> ParseResult<A> + 'a,
{
    move |(input, state)| {
        eprintln!("inspect pre: {} - input: \"{}\"", msg, input);
        let result = parser((input, state.clone()));
        let errmsg = format!(
            "error: {:?}",
            result
                .as_ref()
                .map_or_else(|e| e, |_| &ParseError::Notimplemented)
        );
        eprintln!(
            "inspect post: {} - input is now \"{}\"",
            msg,
            result
                .as_ref()
                .map_or_else(|_| errmsg, |((r, _), _)| r.to_string())
        );
        result
    }
}
