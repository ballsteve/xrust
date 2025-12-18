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
        match input.find(|c: char| !c.is_ascii_digit()) {
            Some(0) => Err(ParseError::Combinator(String::from("digit0: no digits"))),
            Some(pos) => {
                //let result = (&mut input).take(pos).collect::<String>();
                Ok(((&input[pos..], state), input[..pos].to_string()))
            }
            None => {
                if input.is_empty() {
                    Err(ParseError::Combinator(String::from("digit0: no input")))
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
        if input.starts_with(|c: char| c.is_ascii_digit()) {
            match input.find(|c: char| !c.is_ascii_digit()) {
                Some(0) => Ok(((&input[1..], state), input[..1].to_string())),
                Some(pos) => Ok(((&input[pos..], state), input[..pos].to_string())),
                None => Ok((("", state), input.to_string())),
            }
        } else {
            Err(ParseError::Combinator(String::from("digit1: no digits")))
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
            Err(ParseError::Combinator(String::from("none_of: no input")))
        } else {
            let a = input.chars().next().unwrap();
            match s.find(a) {
                Some(_) => Err(ParseError::Combinator(String::from(
                    "none_of: found characters",
                ))),
                None => Ok(((&input[1..], state), a)),
            }
        }
    }
}
