use crate::item::Node;
use crate::parser::{ParseError, ParseInput};
use crate::qname::Interner;

pub fn many0<'a, 'i, P, R, I: Interner + 'i, N: Node>(
    parser: P,
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Vec<R>), ParseError>
where
    P: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R), ParseError>,
{
    //TODO ERROR IF ANY ERROR OTHER THAN COMBINATOR RETURNED.
    move |mut input| {
        let mut result = Vec::new();

        while let Ok((input2, next_item)) = parser(input.clone()) {
            result.push(next_item);
            input = input2;
        }

        Ok((input, result))
    }
}

///This is a special combinator, it will reset namespaces on the parser state between iterations
///It is only intended for use when parsing the children of an element node.
pub fn many0nsreset<'a, 'i, P, R, I: Interner + 'i, N: Node>(
    parser: P,
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Vec<R>), ParseError>
where
    P: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R), ParseError>,
{
    //TODO ERROR IF ANY ERROR OTHER THAN COMBINATOR RETURNED.

    move |(mut input, mut state)| {
        let mut result = Vec::new();
        let namespaces = state.namespace.clone();

        while let Ok(((input2, mut state2), next_item)) = parser((input, state.clone())) {
            result.push(next_item);
            input = input2;
            state2.namespace = namespaces.clone();
            state = state2;
        }

        Ok(((input, state), result))
    }
}

pub fn many1<'a, 'i, P, R, I: Interner + 'i, N: Node>(
    parser: P,
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Vec<R>), ParseError>
where
    P: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R), ParseError>,
{
    //TODO ERROR IF ANY ERROR OTHER THAN COMBINATOR RETURNED.
    move |mut input| {
        let mut result = Vec::new();

        match parser(input) {
            Err(err) => Err(err),
            Ok((input1, result1)) => {
                input = input1;
                result.push(result1);
                while let Ok((input2, next_item)) = parser(input.clone()) {
                    input = input2;
                    result.push(next_item);
                }
                Ok((input, result))
            }
        }
    }
}
