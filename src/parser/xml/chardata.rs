use crate::parser::combinators::alt::{alt2, alt3};
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::many::many1;
use crate::parser::combinators::map::map;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::{take_until, take_while};
use crate::parser::combinators::wellformed::{wellformed, wellformed_ver};
use crate::parser::common::{is_char10, is_char11, is_unrestricted_char11};
use crate::parser::{ParseError, ParseInput, ParseResult};
use std::str::FromStr;

// CharData ::= [^<&]* - (']]>')
pub(crate) fn chardata() -> impl Fn(ParseInput) -> ParseResult<String> {
    map(
        many1(
            alt3(
                wellformed_ver(
                    chardata_cdata(),
                    |s| !s.contains(|c: char| !is_char10(&c)), //XML 1.0
                    |s| !s.contains(|c: char| !is_unrestricted_char11(&c)), //XML 1.1
                ),
                map(wellformed_ver(
                    chardata_unicode_codepoint(),
                    is_char10, //XML 1.0
                    is_char11) //XML 1.1
                ,|c|c.to_string()),
                wellformed_ver(
                    chardata_literal(),
                    |s| !s.contains("]]>") && !s.contains(|c: char| !is_char10(&c)), //XML 1.0
                    |s| !s.contains("]]>") && !s.contains(|c: char| !is_unrestricted_char11(&c)), //XML 1.1
                ),

               // |s| { !s.contains("]]>") && !s.contains(|c: char| !is_char11(&c)) }, // XML 1.1
            )
        )
        ,|v| v.concat()
    )
}

fn chardata_cdata() -> impl Fn(ParseInput) -> ParseResult<String> {
    delimited(tag("<![CDATA["), take_until("]]>"), tag("]]>"))
}

pub(crate) fn chardata_escapes() -> impl Fn(ParseInput) -> ParseResult<String> {
    move |input| match chardata_unicode_codepoint()(input.clone()) {
        Ok((inp, s)) => Ok((inp, s.to_string())),
        Err(e) => Err(e),
    }
}

pub(crate) fn chardata_unicode_codepoint() -> impl Fn(ParseInput) -> ParseResult<char> {
    map(
        wellformed(
            alt2(
                delimited(tag("&#x"), parse_hex(), tag(";")),
                delimited(tag("&#"), parse_decimal(), tag(";")),
            ),
            |value| {
                std::char::from_u32(*value).is_some()
                //match std::char::from_u32(*value) {
                //    None => false,
                //    _ => true
                //}
            }
        ),
        |value| std::char::from_u32(value).unwrap(),
    )
}

fn parse_hex() -> impl Fn(ParseInput) -> ParseResult<u32> {
    move |input| match take_while(|c: char| c.is_ascii_hexdigit())(input) {
        Ok((input1, hex)) => match u32::from_str_radix(&hex, 16) {
            Ok(r) => {
                Ok((input1, r))
            },
            Err(_) => Err(ParseError::NotWellFormed),
        },
        Err(e) => Err(e),
    }
}
fn parse_decimal() -> impl Fn(ParseInput) -> ParseResult<u32> {
    move |input| match take_while(|c: char| c.is_ascii_digit())(input) {
        Ok((input1, dec)) => match u32::from_str(&dec) {
            Ok(r) => Ok((input1, r)),
            Err(_) => Err(ParseError::NotWellFormed),
        },
        Err(e) => Err(e),
    }
}

fn chardata_literal() -> impl Fn(ParseInput) -> ParseResult<String> {
    take_while(|c| c != '<' && c != '&')
}
