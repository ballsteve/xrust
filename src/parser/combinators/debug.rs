use crate::parser::{ParseError, ParseInput, ParseResult};

pub(crate) fn inspect<'a, P1, A>(
    msg: &'a str,
    parser: P1,
) -> impl Fn(ParseInput) -> ParseResult<A> + '_
where
    P1: Fn(ParseInput) -> ParseResult<A> + 'a,
{
    move |input| {
        //eprintln!("inspect pre: {} - input: \"{}\"", msg, input);
        let result = parser(input);
        //eprintln!("inspect post: {}", msg);
        result
    }
}
