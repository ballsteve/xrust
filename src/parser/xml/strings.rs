use crate::parser::{ParseInput, ParseResult};
use crate::parser::combinators::alt::{alt2, alt3};
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_while;
use crate::parser::xml::chardata::chardata_escapes;
use crate::parser::xml::chardata::chardata_unicode_codepoint;

pub(crate) fn delimited_string() -> impl Fn(ParseInput) -> ParseResult<String> {
    alt2(string_single(), string_double())
}
fn string_single() -> impl Fn(ParseInput) -> ParseResult<String> {
    delimited(
        tag("\'"),
        map(
            many0(alt3(
                chardata_escapes(),
                chardata_unicode_codepoint(),
                take_while(|c| !"&\'<".contains(c)),
            )),
            |v| v.concat(),
        ),
        tag("\'"),
    )
}
fn string_double() -> impl Fn(ParseInput) -> ParseResult<String> {
    delimited(
        tag("\""),
        map(
            many0(alt2(
                chardata_escapes(),
                take_while(|c| !"&\"<".contains(c)),
            )),
            |v| v.concat(),
        ),
        tag("\""),
    )
}
