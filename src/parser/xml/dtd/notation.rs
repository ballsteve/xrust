use crate::intmuttree::DTDDecl;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_until;
use crate::parser::combinators::tuple::{tuple4, tuple7, tuple8};
use crate::parser::combinators::whitespace::{whitespace0, whitespace1};
use crate::parser::xml::qname::{name, qualname};
use crate::parser::{ParseInput, ParseResult};
use crate::parser::combinators::wellformed::wellformed;

//NotationType ::= 'NOTATION' S '(' S? Name (S? '|' S? Name)* S? ')'
pub(crate) fn notationtype() -> impl Fn(ParseInput) -> ParseResult<()> {
    map(
        tuple8(
            tag("NOTATION"),
            whitespace1(),
            tag("("),
            whitespace0(),
            name(),
            many0(tuple4(whitespace0(), tag("|"), whitespace0(), name())),
            whitespace0(),
            tag(")"),
        ),
        |_x| (),
    )
}

pub(crate) fn ndatadecl() -> impl Fn(ParseInput) -> ParseResult<()> {
    move |input| match tuple7(
        tag("<!NOTATION"),
        whitespace1(),
        wellformed(qualname(),|n| !n.to_string().contains(":") ),
        whitespace1(),
        take_until(">"), //contentspec - TODO Build out.
        whitespace0(),
        tag(">"),
    )(input)
    {
        Ok(((input2, mut state2), (_, _, n, _, s, _, _))) => {
            state2
                .dtd
                .notations
                .insert(n.to_string(), DTDDecl::Notation(n, s));
            Ok(((input2, state2), ()))
        }
        Err(err) => Err(err),
    }
}
