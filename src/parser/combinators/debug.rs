use crate::item::Node;
use crate::parser::{ParseError, ParseInput, StaticState};
use qualname::{NamespacePrefix, NamespaceUri};

/// Emits a message to stderr from within the parser combinator. This can be useful for debugging.
#[allow(dead_code)]
pub fn inspect<'a, P1, A, N: Node, L>(
    msg: &'a str,
    parser: P1,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError> + 'a
where
    P1: Fn(ParseInput<N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError> + 'a,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), ss| {
        eprintln!(
            "inspect pre: {} - input: \"{}\"",
            msg,
            input.chars().take(80).collect::<String>()
        );
        let result = parser((input, state.clone()), ss);
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
