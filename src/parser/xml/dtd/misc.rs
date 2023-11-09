use crate::parser::combinators::alt::{alt2, alt3, alt4};
use crate::parser::combinators::many::{many0, many1};
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_while;
use crate::parser::combinators::tuple::{tuple2, tuple4, tuple5, tuple6};
use crate::parser::combinators::whitespace::whitespace0;
use crate::parser::common::is_namechar;
use crate::parser::xml::qname::name;
use crate::parser::{ParseInput, ParseResult};
use crate::parser::combinators::value::value;
use crate::parser::xml::dtd::pereference::petextreference;

pub(crate) fn nmtoken() -> impl Fn(ParseInput) -> ParseResult<()> {
    map(many1(take_while(|c| is_namechar(&c))), |_x| ())
}

pub(crate) fn contentspec() -> impl Fn(ParseInput) -> ParseResult<String> {
    alt4(
        value(tag("EMPTY"), "EMPTY".to_string()),
        value(tag("ANY"), "ANY".to_string()),
        mixed(),
        children(),
    )
}

//Mixed	   ::=   	'(' S? '#PCDATA' (S? '|' S? Name)* S? ')*' | '(' S? '#PCDATA' S? ')'
pub(crate) fn mixed() -> impl Fn(ParseInput) -> ParseResult<String> {
    alt2(
        map(
            tuple6(
                tag("("),
                whitespace0(),
                tag("#PCDATA"),
                many0(
                    tuple4(
                        whitespace0(),
                        tag("|"),
                        whitespace0(),
                        alt2(
                            petextreference(),
                            name()
                        )
                    )
                ),
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
pub(crate) fn children() -> impl Fn(ParseInput) -> ParseResult<String> {
    map(
        tuple2(
            alt3(
                petextreference(),
                choice(),
                seq()
            ),
            opt(alt3(tag("?"), tag("*"), tag("+"))),
        ),
        |_x| "".to_string(),
    )
}

// cp	   ::=   	(Name | choice | seq) ('?' | '*' | '+')?
fn cp() -> impl Fn(ParseInput) -> ParseResult<String> {
    move |input| {
        map(
            tuple2(
                alt4(
                    petextreference(),
                    name(),
                    choice(),
                    seq()
                ),
                opt(alt3(tag("?"), tag("*"), tag("+"))),
            ),
            |_x| "".to_string(),
        )(input)
    }
}
//choice	   ::=   	'(' S? cp ( S? '|' S? cp )+ S? ')'
fn choice() -> impl Fn(ParseInput) -> ParseResult<String> {
    move |input| {
        map(
            tuple6(
                tag("("),
                whitespace0(),
                cp(),
                many0(
                    alt2(
                        map(petextreference(), |x| ((),(),(),x)),
                        tuple4(whitespace0(), tag("|"), whitespace0(), cp())
                    )
                ),
                whitespace0(),
                tag(")"),
            ),
            |_x| "".to_string(),
        )(input)
    }
}

//seq	   ::=   	'(' S? cp ( S? ',' S? cp )* S? ')'
fn seq() -> impl Fn(ParseInput) -> ParseResult<String> {
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
