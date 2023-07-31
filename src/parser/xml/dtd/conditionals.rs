use crate::parser::{ParseInput, ParseResult};
use crate::parser::combinators::many::many0;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::{take_until, take_until_either_or};
use crate::parser::combinators::tuple::{tuple2, tuple3, tuple7};
use crate::parser::combinators::whitespace::whitespace0;
use crate::parser::xml::dtd::extsubset::extsubsetdecl;

pub(crate) fn includesect() -> impl Fn(ParseInput) -> ParseResult<()> {
    move |(input, state)| {
        match tuple7(
            tag("<!["),
            whitespace0(),
            tag("INCLUDE"),
            whitespace0(),
            tag("["),
            extsubsetdecl(),
            tag("]]>"),
        )((input, state)) {
            Ok(((input2, state2), (_, _, _, _, _, _, _))) => {
                Ok(((input2, state2), ()))
            }
            Err(e) => { Err(e) }
        }
    }
}

pub(crate) fn ignoresect() -> impl Fn(ParseInput) -> ParseResult<()> {
    move |(input, state)| {
        match tuple7(
            tag("<!["),
            whitespace0(),
            tag("IGNORE"),
            whitespace0(),
            tag("["),
            ignoresectcontents(),
            //take_until("]]>"),
            tag("]]>"),
        )((input, state.clone())) {
            Ok(((input2, _), (_, _, _, _, _, _, _))) => {
                Ok(((input2, state), ()))
            }
            Err(e) => { Err(e) }
        }
    }
}


fn ignoresectcontents() -> impl Fn(ParseInput) -> ParseResult<()> {
    move |(input, state)| {
        match tuple2(
        many0(
            tuple2(
                take_until_either_or("<![", "]]>"),
                tuple3(
                    tag("<!["),
                    ignoresectcontents(),
                    tag("]]>")
                )
            )
        ),
        take_until("]]>"))((input, state.clone())){
            Ok(((input2, _), (_,_))) => {
                Ok(((input2, state),()))
            }
            Err(e) => { Err(e) }
        }
    }
}

