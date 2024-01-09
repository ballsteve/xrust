use crate::item::Node;
use crate::parser::{ParseError, ParseInput};

pub(crate) fn wellformed<P, F, A, N: Node>(
    parser: P,
    validate_fn: F,
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, A), ParseError>
where
    P: Fn(ParseInput<N>) -> Result<(ParseInput<N>, A), ParseError>,
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
pub(crate) fn wellformed_ver<P, F10, F11, A, N: Node>(
    parser: P,
    validate_fn10: F10,
    validate_fn11: F11,
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, A), ParseError>
where
    P: Fn(ParseInput<N>) -> Result<(ParseInput<N>, A), ParseError>,
    F10: Fn(&A) -> bool,
    F11: Fn(&A) -> bool,
{
    /*
       Some well formed constraints (specifically character checks) are dependant on XML versions.
       This just selects the constrain based on the version in the state.
    */
    move |input| match parser(input) {
        Ok(((input2, state2), result)) => {
            if state2.xmlversion == "1.1" {
                if validate_fn11(&result) {
                    Ok(((input2, state2), result))
                } else {
                    Err(ParseError::NotWellFormed)
                }
            } else if validate_fn10(&result) {
                Ok(((input2, state2), result))
            } else {
                Err(ParseError::NotWellFormed)
            }
        }
        Err(err) => Err(err),
    }
}
