use crate::parser::{ParseError, ParseInput, ParseResult};

pub(crate) fn validate<P, F, A>(parser: P, validate_fn: F) -> impl Fn(ParseInput) -> ParseResult<A>
where
    P: Fn(ParseInput) -> ParseResult<A>,
    F: Fn(&A) -> bool,
{
    move |input| match parser(input) {
        Ok((input2, result)) => {
            if validate_fn(&result) {
                Ok((input2, result))
            } else {
                Err(ParseError::Validation {
                    col: input2.currentcol,
                    row: input2.currentrow,
                })
            }
        }
        Err(err) => Err(err),
    }
}
