use crate::parser::combinators::alt::{alt2, alt3};
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::{take_until, take_while};
use crate::parser::combinators::tuple::{tuple3, tuple4, tuple5, tuple7, tuple8};
use crate::parser::combinators::wellformed::wellformed;
use crate::parser::combinators::whitespace::{whitespace0, whitespace1};
use crate::parser::common::{is_pubid_char, is_pubid_charwithapos};
use crate::parser::xml::qname::{name, qualname};
use crate::parser::{ParseInput, ParseResult};
use crate::trees::intmuttree::DTDDecl;

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

pub(crate) fn notationpublicid() -> impl Fn(ParseInput) -> ParseResult<String> {
    alt3(
        map(
            tuple3(
                tag("SYSTEM"),
                whitespace0(),
                alt2(
                    delimited(tag("'"), take_until("'"), tag("'")),
                    delimited(tag("\""), take_until("\""), tag("\"")),
                ), //SystemLiteral
            ),
            |(_, _, sid)| sid, //(sid, None),
        ),
        map(
            tuple5(
                tag("PUBLIC"),
                whitespace0(),
                alt2(
                    delimited(tag("'"), take_while(|c| is_pubid_char(&c)), tag("'")),
                    delimited(
                        tag("\""),
                        take_while(|c| is_pubid_charwithapos(&c)),
                        tag("\""),
                    ),
                ), //PubidLiteral TODO validate chars here (PubidChar from spec).
                whitespace1(),
                alt2(
                    delimited(tag("'"), take_until("'"), tag("'")),
                    delimited(tag("\""), take_until("\""), tag("\"")),
                ), //SystemLiteral
            ),
            |(_, _, _pid, _, sid)| sid,
        ), //(sid, Some(pid)),
        map(
            tuple3(
                tag("PUBLIC"),
                whitespace0(),
                alt2(
                    delimited(tag("'"), take_while(|c| is_pubid_char(&c)), tag("'")),
                    delimited(
                        tag("\""),
                        take_while(|c| is_pubid_charwithapos(&c)),
                        tag("\""),
                    ),
                ),
            ),
            |_| "".to_string(),
        ),
    )
}

pub(crate) fn ndatadecl() -> impl Fn(ParseInput) -> ParseResult<()> {
    move |input| match tuple7(
        tag("<!NOTATION"),
        whitespace1(),
        wellformed(qualname(), |n| !n.to_string().contains(':')),
        whitespace1(),
        notationpublicid(),
        //contentspec(), //take_until(">"), //contentspec - TODO Build out.
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
