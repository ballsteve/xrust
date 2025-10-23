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
    move |mut input, ss| {
        let mut result = Vec::new();

        loop {
            match parser(input.clone(), ss) {
                Ok((input2, next_item)) => {
                    result.push(next_item);
                    input = input2;
                }
                Err(ParseError::Combinator(_)) => break,
                Err(e) => return Err(e),
            }
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
        let mut new_state = state.clone();
        //let namespaces = state.in_scope_namespaces.clone();

        eprintln!("many0nsreset: input==\"{}\"", input);
        while let Ok(((input2, state2), next_item)) = parser((input, state.clone()), ss) {
            eprintln!("got item, input2==\"{}\"", input2);
            result.push(next_item);
            input = input2;
            new_state = state2;
            eprintln!("many0nsreset: input now==\"{}\"", input);
            //ss.in_scope_namespaces = namespaces.clone();
        }

        eprintln!("many0nsreset: end input==\"{}\"", input);
        Ok(((input, new_state), result))
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
