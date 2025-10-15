use crate::item::Node;
use crate::parser::{ParseError, ParseInput, StaticState};
use qualname::{NamespacePrefix, NamespaceUri};

pub fn many0<'a, P, R, N: Node, L>(
    parser: P,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, Vec<R>), ParseError>
where
    P: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, R), ParseError>,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    //TODO ERROR IF ANY ERROR OTHER THAN COMBINATOR RETURNED.
    move |mut input, ss| {
        let mut result = Vec::new();

        while let Ok((input2, next_item)) = parser(input.clone(), ss) {
            result.push(next_item);
            input = input2;
        }

        Ok((input, result))
    }
}

/// This is a special combinator, it will reset namespaces on the parser state between iterations
/// It is only intended for use when parsing the children of an element node.
/// TODO: consider the performance of copying the NamespaceMap for every iteration. Perhaps copy-on-write?
pub fn many0nsreset<'a, P, R, N: Node, L>(
    parser: P,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, Vec<R>), ParseError>
where
    P: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, R), ParseError>,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    //TODO ERROR IF ANY ERROR OTHER THAN COMBINATOR RETURNED.

    move |(mut input, state), ss| {
        let mut result = Vec::new();
        //let namespaces = state.in_scope_namespaces.clone();

        while let Ok(((input2, _state2), next_item)) = parser((input, state.clone()), ss) {
            result.push(next_item);
            input = input2;
            //ss.in_scope_namespaces = namespaces.clone();
        }

        Ok(((input, state), result))
    }
}

pub fn many1<'a, P, R, N: Node, L>(
    parser: P,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, Vec<R>), ParseError>
where
    P: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, R), ParseError>,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    //TODO ERROR IF ANY ERROR OTHER THAN COMBINATOR RETURNED.
    move |mut input, ss| {
        let mut result = Vec::new();

        match parser(input, ss) {
            Err(err) => Err(err),
            Ok((input1, result1)) => {
                input = input1;
                result.push(result1);
                while let Ok((input2, next_item)) = parser(input.clone(), ss) {
                    input = input2;
                    result.push(next_item);
                }
                Ok((input, result))
            }
        }
    }
}
