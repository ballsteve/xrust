use crate::item::Node;
use crate::parser::{ParseError, ParseInput, StaticState};
use qualname::{NamespacePrefix, NamespaceUri};

pub(crate) fn value<'a, P1, R1, V: Clone, N: Node, L>(
    parser1: P1,
    val: V,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, V), ParseError>
where
    P1: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, R1), ParseError>,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |input, ss| match parser1(input, ss) {
        Ok((input1, _)) => Ok((input1, val.clone())),
        Err(err) => Err(err),
    }
}
