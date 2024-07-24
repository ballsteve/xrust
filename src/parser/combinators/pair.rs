use crate::item::Node;
use crate::parser::{ParseError, ParseInput};

pub(crate) fn pair<P1, P2, A, B, N: Node>(
    parser1: P1,
    parser2: P2,
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, (A, B)), ParseError>
where
    P1: Fn(ParseInput<N>) -> Result<(ParseInput<N>, A), ParseError>,
    P2: Fn(ParseInput<N>) -> Result<(ParseInput<N>, B), ParseError>,
{
    move |input| match parser1(input) {
        Ok((input1, parse1_result)) => match parser2(input1) {
            Ok((input2, parse2_result)) => Ok((input2, (parse1_result, parse2_result))),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}
