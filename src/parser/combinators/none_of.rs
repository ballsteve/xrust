use crate::parser::{ParseInput, ParseResult};

pub(crate) fn none_of(chars: &str) -> impl Fn(ParseInput) -> ParseResult<char> + '_ {
    move |(mut input, index)| match input.next() {
        Some(next) => {
            if chars.contains(next) {
                Err(index)
            } else {
                Ok((input, index + 1, next))
            }
        }
        _ => Err(index),
    }
}
