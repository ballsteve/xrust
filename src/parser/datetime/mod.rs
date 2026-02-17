//! # parsepicture
//!
//! A parser for XPath format picture strings, as a parser combinator.
//!
//! This implementation is a quick-and-dirty translation to strftime format.
//!
//! TODO: presentation modifiers, and width modifiers

use crate::item::Node;
use crate::parser::combinators::alt::alt4;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::support::none_of;
use crate::parser::combinators::tag::anychar;
use crate::parser::combinators::tuple::{tuple2, tuple6};
use crate::parser::{ParseError, ParseInput, ParserState, StaticState, StaticStateBuilder};
use crate::xdmerror::*;
use qualname::{NamespacePrefix, NamespaceUri};

// This implementation translates an XPath picture string to a strftime format

#[allow(dead_code)]
fn picture<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    map(
        many0(alt4(open_escape(), close_escape(), literal(), marker())),
        |v| v.iter().cloned().collect::<String>(),
    )
}

#[allow(dead_code)]
fn literal<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    map(none_of("[]"), String::from)
}

#[allow(dead_code)]
fn marker<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    map(
        tuple6(
            anychar('['),
            none_of("]"),
            opt(none_of(",]")),
            opt(none_of(",]")),
            opt(tuple2(anychar(','), none_of("]"))),
            anychar(']'),
        ),
        |(_, c, _p1, _p2, _w, _)| {
            match c {
                "Y" => String::from("%Y"),
                "M" => String::from("%m"),
                "D" => String::from("%d"),
                "d" => String::from("%j"),
                "F" => String::from("%A"),
                "W" => String::from("%U"),
                "w" => String::from(""), // not supported
                "H" => String::from("%H"),
                "h" => String::from("%I"),
                "P" => String::from("%P"),
                "m" => String::from("%M"),
                "s" => String::from("%S"),
                "f" => String::from("%f"),
                "Z" => String::from("%Z"),
                "z" => String::from("%:z"), // partial support
                "C" => String::from(""),    // not supported
                "E" => String::from(""),    // not supported
                _ => String::from(""),      // error
            }
        },
    )
}

#[allow(dead_code)]
fn open_escape<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    map(tuple2(anychar('['), anychar('[')), |_| String::from("["))
}
#[allow(dead_code)]
fn close_escape<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    map(tuple2(anychar(']'), anychar(']')), |_| String::from("]"))
}

pub fn parse<N: Node>(e: &str) -> Result<String, Error> {
    let state: ParserState<N> = ParserState::new();
    let mut static_state = StaticStateBuilder::new()
        .namespace(|_| {
            NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
        })
        .build();
    match picture()((e, state), &mut static_state) {
        Ok(((rem, _), value)) => {
            if rem.is_empty() {
                Ok(value)
            } else {
                Err(Error::new(
                    ErrorKind::Unknown,
                    format!("extra characters after expression: \"{}\"", rem),
                ))
            }
        }
        Err(_) => Err(Error::new(
            ErrorKind::ParseError,
            String::from("unable to parse picture"),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trees::nullo::Nullo;

    #[test]
    fn picture_empty() {
        let pic = parse::<Nullo>("").expect("failed to parse picture \"\"");
        assert_eq!(pic, "");
    }

    #[test]
    fn picture_date() {
        let pic = parse::<Nullo>("[D] [M] [Y]").expect("failed to parse picture \"[D] [M] [Y]\"");
        assert_eq!(pic, "%d %m %Y");
    }

    #[test]
    fn picture_time() {
        let pic = parse::<Nullo>("Hr [h][P] Mins [m] secs [s],[f]")
            .expect("failed to parse picture \"Hr [h][P] Mins [m] secs [s],[f]\"");
        assert_eq!(pic, "Hr %I%P Mins %M secs %S,%f");
    }

    #[test]
    fn picture_datetime() {
        let pic = parse::<Nullo>("[D]/[M]/[Y] [H]:[m]:[s]")
            .expect("failed to parse picture \"[D]/[M]/[Y] [H]:[m]:[s]\"");
        assert_eq!(pic, "%d/%m/%Y %H:%M:%S");
    }

    #[test]
    fn picture_escapes() {
        let pic = parse::<Nullo>("[[[D]/[M]/[Y]]] [[[H]:[m]:[s]]]")
            .expect("failed to parse picture \"[D]/[M]/[Y] [H]:[m]:[s]\"");
        assert_eq!(pic, "[%d/%m/%Y] [%H:%M:%S]");
    }
}
