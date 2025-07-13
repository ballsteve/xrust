use crate::item::Node;
use crate::parser::{ParseError, ParseInput};
use crate::qname::Interner;

pub(crate) fn value<'a, 'i, P1, R1, V: Clone, I: Interner + 'i, N: Node>(
    parser1: P1,
    val: V,
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, V), ParseError>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R1), ParseError>,
{
    move |input| match parser1(input) {
        Ok((input1, _)) => Ok((input1, val.clone())),
        Err(err) => Err(err),
    }
}
