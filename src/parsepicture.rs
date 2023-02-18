//! # parsepicture
//!
//! A parser for XPath format picture strings, as a nom parser combinator.
//!
//! This implementation is a quick-and-dirty translation to strftime format.
//!
//! TODO: presentation modifiers, and width modifiers

extern crate nom;
use crate::xdmerror::*;
use nom::{
    branch::alt,
    character::complete::{char, none_of},
    combinator::{map, opt},
    multi::many0,
    sequence::tuple,
    IResult,
};

// This implementation translates an XPath picture string to a strftime format

#[allow(dead_code)]
fn picture(input: &str) -> IResult<&str, String> {
    map(
        many0(alt((open_escape, close_escape, literal, marker))),
        |v| v.iter().cloned().collect::<String>(),
    )(input)
}

#[allow(dead_code)]
fn literal(input: &str) -> IResult<&str, String> {
    map(none_of("[]"), String::from)(input)
}

#[allow(dead_code)]
fn marker(input: &str) -> IResult<&str, String> {
    map(
        tuple((
            char('['),
            none_of("]"),
            opt(none_of(",]")),
            opt(none_of(",]")),
            opt(tuple((char(','), none_of("]")))),
            char(']'),
        )),
        |(_, c, _p1, _p2, _w, _)| {
            match c {
                'Y' => String::from("%Y"),
                'M' => String::from("%m"),
                'D' => String::from("%d"),
                'd' => String::from("%j"),
                'F' => String::from("%A"),
                'W' => String::from("%U"),
                'w' => String::from(""), // not supported
                'H' => String::from("%H"),
                'h' => String::from("%I"),
                'P' => String::from("%P"),
                'm' => String::from("%M"),
                's' => String::from("%S"),
                'f' => String::from("%f"),
                'Z' => String::from("%Z"),
                'z' => String::from("%:z"), // partial support
                'C' => String::from(""),    // not supported
                'E' => String::from(""),    // not supported
                _ => String::from(""),      // error
            }
        },
    )(input)
}

#[allow(dead_code)]
fn open_escape(input: &str) -> IResult<&str, String> {
    map(tuple((char('['), char('['))), |_| String::from("["))(input)
}
#[allow(dead_code)]
fn close_escape(input: &str) -> IResult<&str, String> {
    map(tuple((char(']'), char(']'))), |_| String::from("]"))(input)
}

pub fn parse(e: &str) -> Result<String, Error> {
    match picture(e) {
        Ok((rest, value)) => {
            if rest.is_empty() {
                Result::Ok(value)
            } else {
                Result::Err(Error {
                    kind: ErrorKind::Unknown,
                    message: format!("extra characters after expression: \"{}\"", rest),
                })
            }
        }
        Err(nom::Err::Error(c)) => Result::Err(Error {
            kind: ErrorKind::Unknown,
            message: format!("parser error: {:?}", c),
        }),
        Err(nom::Err::Incomplete(_)) => Result::Err(Error {
            kind: ErrorKind::Unknown,
            message: String::from("incomplete input"),
        }),
        Err(nom::Err::Failure(_)) => Result::Err(Error {
            kind: ErrorKind::Unknown,
            message: String::from("unrecoverable parser error"),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picture_empty() {
        let pic = parse("").expect("failed to parse picture \"\"");
        assert_eq!(pic, "");
    }

    #[test]
    fn picture_date() {
        let pic = parse("[D] [M] [Y]").expect("failed to parse picture \"[D] [M] [Y]\"");
        assert_eq!(pic, "%d %m %Y");
    }

    #[test]
    fn picture_time() {
        let pic = parse("Hr [h][P] Mins [m] secs [s],[f]")
            .expect("failed to parse picture \"Hr [h][P] Mins [m] secs [s],[f]\"");
        assert_eq!(pic, "Hr %I%P Mins %M secs %S,%f");
    }

    #[test]
    fn picture_datetime() {
        let pic = parse("[D]/[M]/[Y] [H]:[m]:[s]")
            .expect("failed to parse picture \"[D]/[M]/[Y] [H]:[m]:[s]\"");
        assert_eq!(pic, "%d/%m/%Y %H:%M:%S");
    }

    #[test]
    fn picture_escapes() {
        let pic = parse("[[[D]/[M]/[Y]]] [[[H]:[m]:[s]]]")
            .expect("failed to parse picture \"[D]/[M]/[Y] [H]:[m]:[s]\"");
        assert_eq!(pic, "[%d/%m/%Y] [%H:%M:%S]");
    }
}
