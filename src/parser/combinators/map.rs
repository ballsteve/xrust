use crate::item::Node;
use crate::parser::{ParseError, ParseInput, ParserState, StaticState};
use qualname::{NamespacePrefix, NamespaceUri};

pub fn map<'a, P, F, A, B, N: Node, L>(
    parser: P,
    map_fn: F,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, B), ParseError>
//-> impl Fn(ParseInput<N>)-> Result<(String, usize, B), usize>
where
    P: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    F: Fn(A) -> B,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |input, ss| parser(input, ss).map(|(input2, result)| (input2, map_fn(result)))
}

pub fn map_ver<'a, P, F, G, A, B, N: Node, L>(
    parser: P,
    map_fn10: F,
    map_fn11: G,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, B), ParseError>
//-> impl Fn(ParseInput<N>)-> Result<(String, usize, B), usize>
where
    P: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    F: Fn(A) -> B,
    G: Fn(A) -> B,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |input, ss| {
        parser(input, ss).map(|((input2, state2), result)| {
            if state2.xmlversion == "1.1" {
                ((input2, state2), map_fn11(result))
            } else {
                ((input2, state2), map_fn10(result))
            }
        })
    }
}

pub fn map_with_state<'a, P, F, A, B, N: Node, L>(
    parser: P,
    map_fn: F,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, B), ParseError>
//-> impl Fn(ParseInput<N>)-> Result<(String, usize, B), usize>
where
    P: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    F: Fn(A, ParserState<N>, &mut StaticState<L>) -> B,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |input, ss| match parser(input, ss) {
        Ok((input2, result)) => Ok(((input2.0, input2.1.clone()), map_fn(result, input2.1, ss))),
        Err(err) => Err(err),
    }
}
