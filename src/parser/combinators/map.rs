use crate::parser::{ParseInput, ParseResult};

pub(crate) fn map<P, F, A, B>(parser: P, map_fn: F) -> impl Fn(ParseInput) -> ParseResult<B>
//-> impl Fn(ParseInput)-> Result<(String, usize, B), usize>
where
    P: Fn(ParseInput) -> ParseResult<A>,
    F: Fn(A) -> B,
{
    move |mut input| {
        match parser(input) {
            Ok((input2, result)) => Ok((input2, map_fn(result))),
            Err(err) => Err(err),
        }
    }
}
