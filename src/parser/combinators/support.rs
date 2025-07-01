//! Supporting functions.

use crate::item::Node;
use crate::parser::{ParseError, ParseInput, StaticState};
use qualname::{NamespacePrefix, NamespaceUri};

/// Return zero or more digits from the input stream. Be careful not to consume non-digit input.
pub(crate) fn digit0<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), _ss| {
        match input.find(|c| !('0'..='9').contains(&c)) {
            Some(0) => Err(ParseError::Combinator),
            Some(pos) => {
                //let result = (&mut input).take(pos).collect::<String>();
                Ok(((&input[pos..], state), input[..pos].to_string()))
            }
            None => {
                if input.is_empty() {
                    Err(ParseError::Combinator)
                } else {
                    Ok((("", state), input.to_string()))
                }
            }
        }
    }
}
/// Return one or more digits from the input stream.
pub(crate) fn digit1<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), _ss| {
        if input.starts_with(|c| ('0'..='9').contains(&c)) {
            match input.find(|c| !('0'..='9').contains(&c)) {
                Some(0) => Ok(((&input[1..], state), input[..1].to_string())),
                Some(pos) => Ok(((&input[pos..], state), input[..pos].to_string())),
                None => Ok((("", state), input.to_string())),
            }
        } else {
            Err(ParseError::Combinator)
        }
    }
}

/// Return the next character if it is not from the given set
pub(crate) fn none_of<'a, N: Node, L>(
    s: &str,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, char), ParseError> + '_
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), _ss| {
        if input.is_empty() {
            Err(ParseError::Combinator)
        } else {
            let a = input.chars().next().unwrap();
            match s.find(|b| a == b) {
                Some(_) => Err(ParseError::Combinator),
                None => Ok(((&input[1..], state), a)),
            }
        }
    }
}
