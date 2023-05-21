use std::str::FromStr;
use crate::parser::combinators::alt::{alt2, alt3};
use crate::parser::combinators::many::many1;
use crate::parser::combinators::map::map;
use crate::parser::combinators::validate::validate;
use crate::parser::common::is_char;
use crate::parser::{ParseInput, ParseResult, ParseError};
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::{take_until, take_while};

// CharData ::= [^<&]* - (']]>')
pub(crate) fn chardata() -> impl Fn(ParseInput) -> ParseResult<String> {
    validate(
        map(
            many1(alt3(
                chardata_cdata(),
                chardata_escapes(),
                chardata_literal(),
            )),
            |v|
                {
                    v.concat()
                }
        ),
        |s| !s.contains(|c: char| !is_char(&c)),
    )
}

fn chardata_cdata() -> impl Fn(ParseInput) -> ParseResult<String> {
    delimited(tag("<![CDATA["), take_until("]]>"), tag("]]>"))
}

pub(crate) fn chardata_escapes() -> impl Fn(ParseInput) -> ParseResult<String> {
    move |input| match chardata_unicode_codepoint()(input.clone()) {
        Ok((inp, s)) => Ok((inp, s)),
        Err(e) => match delimited(tag("&"), take_until(";"), tag(";"))(input) {
            Ok((inp, rstr)) => match rstr.as_str() {
                "gt" => Ok((inp, ">".to_string())),
                "lt" => Ok((inp, "<".to_string())),
                "amp" => Ok((inp, "&".to_string())),
                "quot" => Ok((inp, "\"".to_string())),
                "apos" => Ok((inp, "\'".to_string())),
                _ => Err(e),
            },
            Err(e) => Err(e),
        },
    }
}

pub(crate) fn chardata_unicode_codepoint() -> impl Fn(ParseInput) -> ParseResult<String> {
    map(
        alt2(
            delimited(tag("&#x"), parse_hex(), tag(";")),
            delimited(tag("&#"), parse_decimal(), tag(";")),
        ),
        |value| std::char::from_u32(value).unwrap().to_string(),
    )
}
fn parse_hex() -> impl Fn(ParseInput) -> ParseResult<u32> {
    move |input| {
        match take_while(|c: char| c.is_ascii_hexdigit())(input){
            Ok((input1, hex)) => {
                match u32::from_str_radix(&hex, 16){
                    Ok(r) => Ok((input1, r)),
                    Err(_) => Err(ParseError::NotWellFormed)
                }
            }
            Err(e) => Err(e)
        }
    }
}
fn parse_decimal() -> impl Fn(ParseInput) -> ParseResult<u32> {
    move |input| {
        match take_while(|c: char| c.is_ascii_digit())(input){
            Ok((input1, dec)) => {
                match u32::from_str(&dec){
                    Ok(r) => Ok((input1, r)),
                    Err(_) => Err(ParseError::NotWellFormed)
                }
            }
            Err(e) => Err(e)
        }
    }
}

fn chardata_literal() -> impl Fn(ParseInput) -> ParseResult<String> {
    validate(take_while(|c| c != '<' && c != '&'), |s| {
        !s.contains("]]>") && !s.contains(|c: char| !is_char(&c))
    })
}