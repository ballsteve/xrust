use crate::item::Node;
use crate::parser::combinators::alt::alt3;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::{take_until, take_until_either_or};
use crate::parser::combinators::tuple::{tuple2, tuple3, tuple5};
use crate::parser::combinators::value::value;
use crate::parser::combinators::whitespace::whitespace0;
use crate::parser::xml::dtd::extsubset::extsubsetdecl;
use crate::parser::xml::dtd::pereference::petextreference;
use crate::parser::{ParseError, ParseInput};

pub(crate) fn conditionalsect<N: Node>(
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, ()), ParseError> {
    move |(input, state)| match tuple5(
        tag("<!["),
        whitespace0(),
        alt3(
            petextreference(),
            value(tag("INCLUDE"), "INCLUDE".to_string()),
            value(tag("IGNORE"), "IGNORE".to_string()),
        ),
        whitespace0(),
        tag("["),
    )((input, state))
    {
        Ok(((input2, state2), (_, _, ii, _, _))) => match ii.as_str() {
            "INCLUDE" => includesect()((input2, state2)),
            "IGNORE" => ignoresect()((input2, state2)),
            _ => Err(ParseError::Combinator),
        },
        Err(e) => Err(e),
    }
}

pub(crate) fn includesect<N: Node>(
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, ()), ParseError> {
    move |(input, state)| match tuple2(extsubsetdecl(), tag("]]>"))((input, state)) {
        Ok(((input2, state2), (_, _))) => Ok(((input2, state2), ())),
        Err(e) => Err(e),
    }
}

pub(crate) fn ignoresect<N: Node>(
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, ()), ParseError> {
    move |(input, state)| match tuple2(ignoresectcontents(), tag("]]>"))((input, state.clone())) {
        Ok(((input2, _), (_, _))) => Ok(((input2, state), ())),
        Err(e) => Err(e),
    }
}

fn ignoresectcontents<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, ()), ParseError>
{
    move |(input, state)| match tuple2(
        many0(tuple2(
            take_until_either_or("<![", "]]>"),
            tuple3(tag("<!["), ignoresectcontents(), tag("]]>")),
        )),
        take_until("]]>"),
    )((input, state.clone()))
    {
        Ok(((input2, _), (_, _))) => Ok(((input2, state), ())),
        Err(e) => Err(e),
    }
}
