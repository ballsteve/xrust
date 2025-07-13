use crate::item::Node;
use crate::parser::{ParseError, ParseInput, ParserState};
use crate::qname::Interner;

pub fn map<'a, 'i, P, F, A, B, I: Interner + 'i, N: Node>(
    parser: P,
    map_fn: F,
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, B), ParseError>
//-> impl Fn(ParseInput<'a, 'i, I, N>)-> Result<(String, usize, B), usize>
where
    P: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    F: Fn(A) -> B,
{
    move |input| match parser(input) {
        Ok((input2, result)) => Ok((input2, map_fn(result))),
        Err(err) => Err(err),
    }
}

pub fn map_ver<'a, 'i, P, F, G, A, B, I: Interner + 'i, N: Node>(
    parser: P,
    map_fn10: F,
    map_fn11: G,
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, B), ParseError>
//-> impl Fn(ParseInput<'a, 'i, I, N>)-> Result<(String, usize, B), usize>
where
    P: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    F: Fn(A) -> B,
    G: Fn(A) -> B,
{
    move |input| match parser(input) {
        Ok(((input2, state2), result)) => {
            if state2.xmlversion == "1.1" {
                Ok(((input2, state2), map_fn11(result)))
            } else {
                Ok(((input2, state2), map_fn10(result)))
            }
        }
        Err(err) => Err(err),
    }
}

pub fn map_with_state<'a, 'i, P, F, A, B, I: Interner + 'i, N: Node>(
    parser: P,
    map_fn: F,
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, B), ParseError>
//-> impl Fn(ParseInput<'a, 'i, I, N>)-> Result<(String, usize, B), usize>
where
    P: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    F: Fn(A, ParserState<'i, I, N>) -> B,
{
    move |input| match parser(input) {
        Ok((input2, result)) => Ok((input2.clone(), map_fn(result, input2.1))),
        Err(err) => Err(err),
    }
}
