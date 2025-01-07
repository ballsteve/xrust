use crate::item::Node;
use crate::parser::{ParseError, ParseInput};

/// Emits a message to stderr from within the parser combinator. This can be useful for debugging.
#[allow(dead_code)]
pub fn inspect<'a, P1, A, N: Node>(
    msg: &'a str,
    parser: P1,
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, A), ParseError> + 'a
where
    P1: Fn(ParseInput<N>) -> Result<(ParseInput<N>, A), ParseError> + 'a,
{
    move |(input, state)| {
        eprintln!(
            "inspect pre: {} - input: \"{}\"",
            msg,
            input.chars().take(80).collect::<String>()
        );
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
                .chars()
                .take(80)
                .collect::<String>()
        );
        result
    }
}
