use std::cmp::Ordering;

use crate::parser::combinators::alt::alt4;
use crate::parser::combinators::many::{many0, many1};
use crate::parser::combinators::map::map;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::tuple3;
use crate::parser::{ParseError, ParseInput, ParseResult};

pub(crate) fn whitespace0() -> impl Fn(ParseInput) -> ParseResult<()> {
    //TODO add support for xml:space
    map(
        many0(alt4(tag(" "), tag("\t"), tag("\r"), tag("\n"))),
        |_| (),
    )
}

pub(crate) fn whitespace1() -> impl Fn(ParseInput) -> ParseResult<()> {
    //TODO add support for xml:space
    map(
        many1(alt4(tag(" "), tag("\t"), tag("\r"), tag("\n"))),
        |_| (),
    )
}

pub(crate) fn xpwhitespace() -> impl Fn(ParseInput) -> ParseResult<()> {
    map(
        tuple3(
            whitespace0(),
            take_until_balanced("(:", ":)"),
            whitespace0(),
        ),
        |_| (),
    )
}

/// Parse nested input.
///
/// Inspired by 'take_until_unbalanced' from parse_hyperlinks crate.
/// We can't use the parse_hyperlinks version since it only takes character delimiters.
/// Also, this function does not need to consider escaped brackets.
/// The function assumes that the open and close delimiters are the same length.
///
/// This function consumes the delimiters.
/// The start delimiter must be the first token in the input. Finding this sets the bracket count to 1.
/// After that there are 4 scenarios:
///
/// * The close delimiter is not found. This is an error.
/// * There is no open delimiter. In this case, consume up to and including the close delimiter. If the bracket count is 1 then return Ok, otherwise error.
/// * There is an open delimiter. If the open occurs after the close, then consume up to and including the close delimiter. If the bracket count is 1 then return Ok, otherwise error.
/// * The open delimiter occurs before the close. In this case, increment the bracket count and continue after the open delimiter.
fn take_until_balanced(
    open: &'static str,
    close: &'static str,
) -> impl Fn(ParseInput) -> ParseResult<()> {
    move |(input, state)| {
        let mut pos = 0;
        let mut counter = 0;
        let mut bracket_counter = 0;

        loop {
            counter += 1;
            if counter > 1000 {
                return Err(ParseError::EntityDepth {
                    row: 0,
                    col: counter,
                });
            }
            match (input[pos..].find(&open), input[pos..].find(&close)) {
                (Some(0), _) => {
                    bracket_counter += 1;
                    pos += open.len();
                    //let _: Vec<_> = (&mut input).take(open.len()).collect();
                    match (input[pos..].find(&open), input[pos..].find(&close)) {
                        (_, None) => {
                            // Scenario 1
                            return Err(ParseError::Unbalanced);
                        }
                        (Some(o), Some(c)) => {
                            // Scenario 3/4
                            if o > c {
                                // Scenario 3
                                if bracket_counter == 1 {
                                    //let _: Vec<_> = (&mut input).take(c + close.len()).collect();
                                    pos += c + close.len();
                                    return Ok(((&input[pos..], state), ()));
                                } else {
                                    return Err(ParseError::Unbalanced);
                                }
                            } else {
                                // Scenario 4
                                bracket_counter += 1;
                                //let _: Vec<_> = (&mut input).take(o + open.len()).collect();
                                pos += o + close.len();
                            }
                        }
                        (_, Some(c)) => {
                            // Scenario 2
                            match bracket_counter.cmp(&1) {
                                Ordering::Greater => {
                                    bracket_counter -= 1;
                                    //let _: Vec<_> = (&mut input).take(c + close.len()).collect();
                                    pos += c + close.len();
                                }
                                Ordering::Equal => {
                                    //let _: Vec<_> = (&mut input).take(c + close.len()).collect();
                                    pos += c + close.len();
                                    return Ok(((&input[pos..], state), ()));
                                }
                                Ordering::Less => {
                                    return Err(ParseError::Unbalanced);
                                }
                            }
                        }
                    }
                }
                (None, Some(c)) => {
                    // Scenario 2
                    match bracket_counter.cmp(&1) {
                        Ordering::Greater => {
                            bracket_counter -= 1;
                            //let _: Vec<_> = (&mut input).take(c + close.len()).collect();
                            pos += c + close.len();
                        }
                        Ordering::Equal => {
                            //let _: Vec<_> = (&mut input).take(c + close.len()).collect();
                            pos += c + close.len();
                            return Ok(((&input[pos..], state), ()));
                        }
                        Ordering::Less => {
                            return Err(ParseError::Unbalanced);
                        }
                    }
                }
                _ => return Ok(((&input[pos..], state), ())),
            }
        }
    }
}
