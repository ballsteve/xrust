use crate::item::Node;
use crate::parser::combinators::alt::{alt2, alt3};
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::many::many1;
use crate::parser::combinators::map::{map, map_ver};
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::{take_until, take_while};
use crate::parser::combinators::wellformed::{wellformed, wellformed_ver};
use crate::parser::common::{is_char10, is_char11, is_unrestricted_char11};
use crate::parser::{ParseError, ParseInput, StaticState};
use qualname::{NamespacePrefix, NamespaceUri};
use std::str::FromStr;

// CharData ::= [^<&]* - (']]>')
pub(crate) fn chardata<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), ss| {
        map(
            many1(alt3(
                map_ver(
                    wellformed_ver(
                        chardata_cdata(),
                        |s| !s.contains(|c: char| !is_char10(&c)), //XML 1.0
                        |s| !s.contains(|c: char| !is_unrestricted_char11(&c)), //XML 1.1
                        "invalid character",
                    ),
                    |s: String| s.replace("\r\n", "\n").replace("\r", "\n"),
                    |s: String| {
                        s.replace("\r\n", "\n")
                            .replace("\r\u{85}", "\n")
                            .replace("\u{85}", "\n")
                            .replace("\u{2028}", "\n")
                            .replace("\r", "\n")
                    },
                ),
                map(
                    wellformed_ver(
                        chardata_unicode_codepoint(),
                        is_char10, //XML 1.0
                        is_char11,
                        "invalid character",
                    ), //XML 1.1
                    |c| c.to_string(),
                ),
                map_ver(
                    wellformed_ver(
                        chardata_literal(),
                        |s| !s.contains("]]>") && !s.contains(|c: char| !is_char10(&c)), //XML 1.0
                        |s| {
                            !s.contains("]]>") && !s.contains(|c: char| !is_unrestricted_char11(&c))
                        }, //XML 1.1
                        "processing instruction contains invalid character",
                    ),
                    |s: String| s.replace("\r\n", "\n").replace("\r", "\n"),
                    |s: String| {
                        s.replace("\r\n", "\n")
                            .replace("\r\u{85}", "\n")
                            .replace("\u{85}", "\n")
                            .replace("\u{2028}", "\n")
                            .replace("\r", "\n")
                    },
                ),
                // |s| { !s.contains("]]>") && !s.contains(|c: char| !is_char11(&c)) }, // XML 1.1
            )),
            |v| v.concat(),
        )((input, state), ss)
    }
}

fn chardata_cdata<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    delimited(tag("<![CDATA["), take_until("]]>"), tag("]]>"))
}

pub(crate) fn chardata_escapes<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |input, ss| match chardata_unicode_codepoint()(input.clone(), ss) {
        Ok((inp, s)) => Ok((inp, s.to_string())),
        Err(e) => Err(e),
    }
}

pub(crate) fn chardata_unicode_codepoint<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, char), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
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
            },
            "invalid character in codepoint",
        ),
        |value| std::char::from_u32(value).unwrap(),
    )
}

fn parse_hex<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, u32), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |input, ss| match take_while(|c: char| c.is_ascii_hexdigit())(input, ss) {
        Ok((input1, hex)) => match u32::from_str_radix(&hex, 16) {
            Ok(r) => Ok((input1, r)),
            Err(_) => Err(ParseError::NotWellFormed(hex)),
        },
        Err(e) => Err(e),
    }
}
fn parse_decimal<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, u32), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |input, ss| match take_while(|c: char| c.is_ascii_digit())(input, ss) {
        Ok((input1, dec)) => match u32::from_str(&dec) {
            Ok(r) => Ok((input1, r)),
            Err(_) => Err(ParseError::NotWellFormed(dec)),
        },
        Err(e) => Err(e),
    }
}

fn chardata_literal<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    take_while(|c| c != '<' && c != '&')
}
