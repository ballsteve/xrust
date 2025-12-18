use crate::item::Node;
use crate::parser::{ParseError, ParseInput, StaticState};
use qualname::{NamespacePrefix, NamespaceUri};

pub(crate) fn wellformed<'a, P, F, A, N: Node, L>(
    parser: P,
    validate_fn: F,
    reason: &str,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>
where
    P: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    F: Fn(&A) -> bool,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |input, ss| match parser(input, ss) {
        Ok(((input2, state2), result)) => {
            if validate_fn(&result) {
                Ok(((input2, state2), result))
            } else {
                Err(ParseError::NotWellFormed(reason.to_string()))
            }
        }
        Err(err) => Err(err),
    }
}
pub(crate) fn wellformed_ver<'a, P, F10, F11, A, N: Node, L>(
    parser: P,
    validate_fn10: F10,
    validate_fn11: F11,
    reason: &str,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>
where
    P: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    F10: Fn(&A) -> bool,
    F11: Fn(&A) -> bool,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    /*
       Some well formed constraints (specifically character checks) are dependant on XML versions.
       This just selects the constraint based on the version in the state.
    */
    move |input, ss| match parser(input, ss) {
        Ok(((input2, state2), result)) => {
            if state2.xmlversion == "1.1" {
                if validate_fn11(&result) {
                    Ok(((input2, state2), result))
                } else {
                    Err(ParseError::NotWellFormed(format!(
                        "{} - \"{}\"",
                        reason, input2
                    )))
                }
            } else if validate_fn10(&result) {
                Ok(((input2, state2), result))
            } else {
                Err(ParseError::NotWellFormed(format!(
                    "{} - \"{}\"",
                    reason, input2
                )))
            }
        }
        Err(err) => Err(err),
    }
}
