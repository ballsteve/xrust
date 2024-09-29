use crate::xmldecl::AttType;
use crate::item::Node;
use crate::parser::combinators::alt::alt2;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::{tuple4, tuple6};
use crate::parser::combinators::whitespace::whitespace0;
use crate::parser::xml::dtd::misc::nmtoken;
use crate::parser::xml::dtd::notation::notationtype;
use crate::parser::{ParseError, ParseInput};

//EnumeratedType ::= NotationType | Enumeration
pub(crate) fn enumeratedtype<N: Node>(
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, AttType), ParseError> {
    alt2(notationtype(), enumeration())
}

//Enumeration ::= '(' S? Nmtoken (S? '|' S? Nmtoken)* S? ')'
fn enumeration<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, AttType), ParseError> {
    map(
        tuple6(
            tag("("),
            whitespace0(),
            nmtoken(),
            many0(
                map(
                    tuple4(whitespace0(), tag("|"), whitespace0(), nmtoken()),
                    |(_,_,_,nmt)| {
                        nmt
                    }
                )
            ),
            whitespace0(),
            tag(")"),
        ),
        |(_,_,nm, mut nms,_,_)|{
            nms.push(nm);
            AttType::ENUMERATION(nms)
        }
    )
}
