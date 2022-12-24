use crate::parser::{ParseInput, ParseError, ParseResult};

pub(crate) fn none_of(chars: &str) -> impl Fn(ParseInput) -> ParseResult<char> + '_ {
    move |mut input| match input.next() {
        Some(next) => {
            if chars.contains(next) {
                Err(ParseError::Combinator)
            } else {
                Ok((input,  next))
            }
        }
        None => Err(ParseError::Combinator),
    }
}
