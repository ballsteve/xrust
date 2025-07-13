use crate::item::Node;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::{tuple2, tuple5, tuple6};
use crate::parser::combinators::whitespace::{whitespace0, whitespace1};
use crate::parser::xml::strings::delimited_string;
use crate::parser::xml::xmldecl::encodingdecl;
use crate::parser::{ParseError, ParseInput};
use crate::qname::Interner;
use crate::xmldecl::XMLDecl;

fn xmldeclversion<'a, 'i, I: Interner, N: Node>(
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, String), ParseError> {
    move |(input, state)| match tuple5(
        tag("version"),
        whitespace0(),
        tag("="),
        whitespace0(),
        delimited_string(),
    )((input, state))
    {
        Ok(((input1, state1), (_, _, _, _, v))) => {
            if v == *"1.1" {
                if state1.xmlversion == "1.0" {
                    return Err(ParseError::NotWellFormed(String::from("version mismatch")));
                }
                Ok(((input1, state1), v))
            } else if v.starts_with("1.") {
                Ok(((input1, state1), "1.0".to_string()))
            } else {
                Err(ParseError::Notimplemented)
            }
        }
        Err(err) => Err(err),
    }
}

pub(crate) fn textdecl<'a, 'i, I: Interner + 'i, N: Node>(
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, XMLDecl), ParseError> {
    //This is NOT the same as the XML declaration in XML documents.
    //There is no standalone, and the version is optional.
    map(
        tuple6(
            tag("<?xml"),
            opt(tuple2(whitespace1(), xmldeclversion())),
            encodingdecl(),
            whitespace0(),
            tag("?>"),
            whitespace0(),
        ),
        |(_, ver, enc, _, _, _)| {
            if ver == Some(((), "1.1".to_string())) {
                XMLDecl {
                    version: "1.1".to_string(),
                    encoding: Some(enc),
                    standalone: None,
                }
            } else {
                XMLDecl {
                    version: "1.0".to_string(),
                    encoding: Some(enc),
                    standalone: None,
                }
            }
        },
    )
}
