use crate::item::Node;
use crate::parser::combinators::alt::{alt2, alt3, alt4};
use crate::parser::combinators::many::{many0, many1};
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_while;
use crate::parser::combinators::tuple::{tuple2, tuple4, tuple5, tuple6};
use crate::parser::combinators::value::value;
use crate::parser::combinators::whitespace::whitespace0;
use crate::parser::common::is_namechar;
use crate::parser::xml::dtd::pereference::petextreference;
use crate::parser::xml::qname::name;
use crate::parser::{ParseError, ParseInput};

pub(crate) fn nmtoken<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, String), ParseError>
{
    map(many1(take_while(|c| is_namechar(&c))), |x| x.join(""))
}

pub(crate) fn contentspec<N: Node>(
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, String), ParseError> {
    alt4(
        value(tag("EMPTY"), "EMPTY".to_string()),
        value(tag("ANY"), "ANY".to_string()),
        mixed(),
        children(),
    )
}

//Mixed	   ::=   	'(' S? '#PCDATA' (S? '|' S? Name)* S? ')*' | '(' S? '#PCDATA' S? ')'
pub(crate) fn mixed<N: Node>(
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, String), ParseError> {
    alt2(
        map(
            tuple6(
                tag("("),
                whitespace0(),
                tag("#PCDATA"),
                many0(tuple4(
                    whitespace0(),
                    tag("|"),
                    whitespace0(),
                    alt2(petextreference(), name()),
                )),
                whitespace0(),
                tag(")*"),
            ),
            |_x| "".to_string(),
        ),
        map(
            tuple5(
                tag("("),
                whitespace0(),
                tag("#PCDATA"),
                whitespace0(),
                tag(")"),
            ),
            |_x| "".to_string(),
        ),
    )
}

// children	   ::=   	(choice | seq) ('?' | '*' | '+')?
pub(crate) fn children<N: Node>(
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, String), ParseError> {
    map(
        tuple2(
            alt3(petextreference(), choice(), seq()),
            opt(alt3(tag("?"), tag("*"), tag("+"))),
        ),
        |_x| "".to_string(),
    )
}

// cp	   ::=   	(Name | choice | seq) ('?' | '*' | '+')?
fn cp<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, String), ParseError> {
    move |input| {
        map(
            tuple2(
                alt4(petextreference(), name(), choice(), seq()),
                opt(alt3(tag("?"), tag("*"), tag("+"))),
            ),
            |_x| "".to_string(),
        )(input)
    }
}
//choice	   ::=   	'(' S? cp ( S? '|' S? cp )+ S? ')'
fn choice<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, String), ParseError> {
    move |input| {
        map(
            tuple6(
                tag("("),
                whitespace0(),
                cp(),
                many0(alt2(
                    map(petextreference(), |x| ((), (), (), x)),
                    tuple4(whitespace0(), tag("|"), whitespace0(), cp()),
                )),
                whitespace0(),
                tag(")"),
            ),
            |_x| "".to_string(),
        )(input)
    }
}

//seq	   ::=   	'(' S? cp ( S? ',' S? cp )* S? ')'
fn seq<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, String), ParseError> {
    map(
        tuple6(
            tag("("),
            whitespace0(),
            cp(),
            many0(tuple4(whitespace0(), tag(","), whitespace0(), cp())),
            whitespace0(),
            tag(")"),
        ),
        |_x| "".to_string(),
    )
}
