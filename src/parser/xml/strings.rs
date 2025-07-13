use crate::item::Node;
use crate::parser::combinators::alt::{alt2, alt3};
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_while;
use crate::parser::xml::chardata::chardata_escapes;
use crate::parser::xml::chardata::chardata_unicode_codepoint;
use crate::parser::{ParseError, ParseInput};
use crate::qname::Interner;

pub(crate) fn delimited_string<'a, 'i, I: Interner + 'i, N: Node>(
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, String), ParseError> {
    alt2(string_single(), string_double())
}
fn string_single<'a, 'i, I: Interner + 'i, N: Node>(
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, String), ParseError> {
    delimited(
        tag("\'"),
        map(
            many0(alt3(
                chardata_escapes(),
                map(chardata_unicode_codepoint(), |c| c.to_string()),
                take_while(|c| !"&\'<".contains(c)),
            )),
            |v| v.concat(),
        ),
        tag("\'"),
    )
}
fn string_double<'a, 'i, I: Interner + 'i, N: Node>(
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, String), ParseError> {
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
