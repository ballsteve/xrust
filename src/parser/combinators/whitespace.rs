use crate::parser::combinators::alt::alt4;
use crate::parser::combinators::many::{many0, many1};
use crate::parser::combinators::map::map;
use crate::parser::combinators::tag::tag;
use crate::parser::{ParseInput, ParseResult};

pub(crate) fn whitespace0() -> impl Fn(ParseInput) -> ParseResult<()> {
    map(
        many0(alt4(tag(" "), tag("\t"), tag("\r"), tag("\n"))),
        |_| (),
    )
}

pub(crate) fn whitespace1() -> impl Fn(ParseInput) -> ParseResult<()> {
    map(
        many1(alt4(tag(" "), tag("\t"), tag("\r"), tag("\n"))),
        |_| (),
    )
}
