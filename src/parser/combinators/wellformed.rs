use crate::parser::{ParseError, ParseInput, ParseResult};

pub(crate) fn wellformed<P, F, A>(
    parser: P,
    validate_fn: F,
) -> impl Fn(ParseInput) -> ParseResult<A>
where
    P: Fn(ParseInput) -> ParseResult<A>,
    F: Fn(&A) -> bool,
{
    move |input| match parser(input) {
        Ok(((input2, state2), result)) => {
            if validate_fn(&result) {
                Ok(((input2, state2), result))
            } else {
                Err(ParseError::NotWellFormed)
            }
        }
        Err(err) => Err(err),
    }
}
