use crate::item::Node;
use crate::parser::{ParseError, ParseInput};

pub fn map<P, F, A, B, N: Node>(parser: P, map_fn: F) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, B), ParseError>
//-> impl Fn(ParseInput<N>)-> Result<(String, usize, B), usize>
where
    P: Fn(ParseInput<N>) -> Result<(ParseInput<N>, A), ParseError>,
    F: Fn(A) -> B,
{
    move |input| match parser(input) {
        Ok((input2, result)) => Ok((input2, map_fn(result))),
        Err(err) => Err(err),
    }
}
