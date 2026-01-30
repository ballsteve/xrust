use crate::item::Node;
use crate::parser::{ParseError, ParseInput, StaticState};
use qualname::{NamespacePrefix, NamespaceUri};

pub(crate) fn pair<'a, P1, P2, A, B, N: Node, L>(
    parser1: P1,
    parser2: P2,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, (A, B)), ParseError>
where
    P1: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P2: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, B), ParseError>,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |input, ss| match parser1(input, ss) {
        Ok((input1, parse1_result)) => match parser2(input1, ss) {
            Ok((input2, parse2_result)) => Ok((input2, (parse1_result, parse2_result))),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}
