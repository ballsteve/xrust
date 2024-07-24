use crate::item::Node;
use crate::parser::{ParseError, ParseInput};

pub(crate) fn value<P1, R1, V: Clone, N: Node>(
    parser1: P1,
    val: V,
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, V), ParseError>
where
    P1: Fn(ParseInput<N>) -> Result<(ParseInput<N>, R1), ParseError>,
{
    move |input| match parser1(input) {
        Ok((input1, _)) => Ok((input1, val.clone())),
        Err(err) => Err(err),
    }
}
