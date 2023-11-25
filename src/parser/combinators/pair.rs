use crate::parser::{ParseInput, ParseResult};

pub(crate) fn pair<P1, P2, A, B>(
    parser1: P1,
    parser2: P2,
) -> impl Fn(ParseInput) -> ParseResult<(A, B)>
where
    P1: Fn(ParseInput) -> ParseResult<A>,
    P2: Fn(ParseInput) -> ParseResult<B>,
{
    move |input| match parser1(input) {
        Ok((input1, parse1_result)) => match parser2(input1) {
            Ok((input2, parse2_result)) => Ok((input2, (parse1_result, parse2_result))),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}