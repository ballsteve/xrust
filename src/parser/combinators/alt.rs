use crate::parser::{ParseError, ParseInput, ParseResult};

pub(crate) fn alt2<P1, P2, A>(parser1: P1, parser2: P2) -> impl Fn(ParseInput) -> ParseResult<A>
where
    P1: Fn(ParseInput) -> ParseResult<A>,
    P2: Fn(ParseInput) -> ParseResult<A>,
{
    move |(input, state)| match parser1((input, state.clone())) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator) => match parser2((input, state)) {
            Ok(parse_result2) => Ok(parse_result2),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

pub(crate) fn alt3<P1, P2, P3, A>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
) -> impl Fn(ParseInput) -> ParseResult<A>
where
    P1: Fn(ParseInput) -> ParseResult<A>,
    P2: Fn(ParseInput) -> ParseResult<A>,
    P3: Fn(ParseInput) -> ParseResult<A>,
{
    move |(input, state)| match parser1((input, state.clone())) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator) => match parser2((input, state.clone())) {
            Ok(parse_result2) => Ok(parse_result2),
            Err(ParseError::Combinator) => match parser3((input, state)) {
                Ok(parse_result3) => Ok(parse_result3),
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

pub(crate) fn alt4<P1, P2, P3, P4, A>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
) -> impl Fn(ParseInput) -> ParseResult<A>
where
    P1: Fn(ParseInput) -> ParseResult<A>,
    P2: Fn(ParseInput) -> ParseResult<A>,
    P3: Fn(ParseInput) -> ParseResult<A>,
    P4: Fn(ParseInput) -> ParseResult<A>,
{
    move |(input, state)| match parser1((input, state.clone())) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator) => match parser2((input, state.clone())) {
            Ok(parse_result2) => Ok(parse_result2),
            Err(ParseError::Combinator) => match parser3((input, state.clone())) {
                Ok(parse_result3) => Ok(parse_result3),
                Err(ParseError::Combinator) => match parser4((input, state)) {
                    Ok(parse_result4) => Ok(parse_result4),
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

/*
pub(crate) fn alt5<P1, P2, P3, P4, P5, A>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
) -> impl Fn(ParseInput) -> ParseResult<A>
where
    P1: Fn(ParseInput) -> ParseResult<A>,
    P2: Fn(ParseInput) -> ParseResult<A>,
    P3: Fn(ParseInput) -> ParseResult<A>,
    P4: Fn(ParseInput) -> ParseResult<A>,
    P5: Fn(ParseInput) -> ParseResult<A>,
{
    move |(input, state)| match parser1((input, state.clone())) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator) => match parser2((input, state.clone())) {
            Ok(parse_result2) => Ok(parse_result2),
            Err(ParseError::Combinator) => match parser3((input, state.clone())) {
                Ok(parse_result3) => Ok(parse_result3),
                Err(ParseError::Combinator) => match parser4((input, state.clone())) {
                    Ok(parse_result4) => Ok(parse_result4),
                    Err(ParseError::Combinator) => match parser5(input, state) {
                        Ok(parse_result5) => Ok(parse_result5),
                        Err(err) => Err(err),
                    },
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

pub(crate) fn alt6<P1, P2, P3, P4, P5, P6, A>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
    parser6: P6,
) -> impl Fn(ParseInput) -> ParseResult<A>
where
    P1: Fn(ParseInput) -> ParseResult<A>,
    P2: Fn(ParseInput) -> ParseResult<A>,
    P3: Fn(ParseInput) -> ParseResult<A>,
    P4: Fn(ParseInput) -> ParseResult<A>,
    P5: Fn(ParseInput) -> ParseResult<A>,
    P6: Fn(ParseInput) -> ParseResult<A>,
{
    move |(input, state)| match parser1((input, state.clone())) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator) => match parser2((input, state.clone())) {
            Ok(parse_result2) => Ok(parse_result2),
            Err(ParseError::Combinator) => match parser3((input, state.clone())) {
                Ok(parse_result3) => Ok(parse_result3),
                Err(ParseError::Combinator) => match parser4((input, state.clone())) {
                    Ok(parse_result4) => Ok(parse_result4),
                    Err(ParseError::Combinator) => match parser5((input, state.clone())) {
                        Ok(parse_result5) => Ok(parse_result5),
                        Err(ParseError::Combinator) => match parser6((input, state.clone())) {
                            Ok(parse_result6) => Ok(parse_result6),
                            Err(err) => Err(err),
                        },
                        Err(err) => Err(err),
                    },
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}
 */

pub(crate) fn alt7<P1, P2, P3, P4, P5, P6, P7, A>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
    parser6: P6,
    parser7: P7,
) -> impl Fn(ParseInput) -> ParseResult<A>
where
    P1: Fn(ParseInput) -> ParseResult<A>,
    P2: Fn(ParseInput) -> ParseResult<A>,
    P3: Fn(ParseInput) -> ParseResult<A>,
    P4: Fn(ParseInput) -> ParseResult<A>,
    P5: Fn(ParseInput) -> ParseResult<A>,
    P6: Fn(ParseInput) -> ParseResult<A>,
    P7: Fn(ParseInput) -> ParseResult<A>,
{
    move |(input, state)| match parser1((input, state.clone())) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator) => match parser2((input, state.clone())) {
            Ok(parse_result2) => Ok(parse_result2),
            Err(ParseError::Combinator) => match parser3((input, state.clone())) {
                Ok(parse_result3) => Ok(parse_result3),
                Err(ParseError::Combinator) => match parser4((input, state.clone())) {
                    Ok(parse_result4) => Ok(parse_result4),
                    Err(ParseError::Combinator) => match parser5((input, state.clone())) {
                        Ok(parse_result5) => Ok(parse_result5),
                        Err(ParseError::Combinator) => match parser6((input, state.clone())) {
                            Ok(parse_result6) => Ok(parse_result6),
                            Err(ParseError::Combinator) => match parser7((input, state)) {
                                Ok(parse_result7) => Ok(parse_result7),
                                Err(err) => Err(err),
                            },
                            Err(err) => Err(err),
                        },
                        Err(err) => Err(err),
                    },
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn alt8<P1, P2, P3, P4, P5, P6, P7, P8, A>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
    parser6: P6,
    parser7: P7,
    parser8: P8,
) -> impl Fn(ParseInput) -> ParseResult<A>
where
    P1: Fn(ParseInput) -> ParseResult<A>,
    P2: Fn(ParseInput) -> ParseResult<A>,
    P3: Fn(ParseInput) -> ParseResult<A>,
    P4: Fn(ParseInput) -> ParseResult<A>,
    P5: Fn(ParseInput) -> ParseResult<A>,
    P6: Fn(ParseInput) -> ParseResult<A>,
    P7: Fn(ParseInput) -> ParseResult<A>,
    P8: Fn(ParseInput) -> ParseResult<A>,
{
    move |(input, state)| match parser1((input, state.clone())) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator) => match parser2((input, state.clone())) {
            Ok(parse_result2) => Ok(parse_result2),
            Err(ParseError::Combinator) => match parser3((input, state.clone())) {
                Ok(parse_result3) => Ok(parse_result3),
                Err(ParseError::Combinator) => match parser4((input, state.clone())) {
                    Ok(parse_result4) => Ok(parse_result4),
                    Err(ParseError::Combinator) => match parser5((input, state.clone())) {
                        Ok(parse_result5) => Ok(parse_result5),
                        Err(ParseError::Combinator) => match parser6((input, state.clone())) {
                            Ok(parse_result6) => Ok(parse_result6),
                            Err(ParseError::Combinator) => match parser7((input, state.clone())) {
                                Ok(parse_result7) => Ok(parse_result7),
                                Err(ParseError::Combinator) => match parser8((input, state)) {
                                    Ok(parse_result8) => Ok(parse_result8),
                                    Err(err) => Err(err),
                                },
                                Err(err) => Err(err),
                            },
                            Err(err) => Err(err),
                        },
                        Err(err) => Err(err),
                    },
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn alt11<P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, A>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
    parser6: P6,
    parser7: P7,
    parser8: P8,
    parser9: P9,
    parser10: P10,
    parser11: P11
) -> impl Fn(ParseInput) -> ParseResult<A>
    where
        P1: Fn(ParseInput) -> ParseResult<A>,
        P2: Fn(ParseInput) -> ParseResult<A>,
        P3: Fn(ParseInput) -> ParseResult<A>,
        P4: Fn(ParseInput) -> ParseResult<A>,
        P5: Fn(ParseInput) -> ParseResult<A>,
        P6: Fn(ParseInput) -> ParseResult<A>,
        P7: Fn(ParseInput) -> ParseResult<A>,
        P8: Fn(ParseInput) -> ParseResult<A>,
        P9: Fn(ParseInput) -> ParseResult<A>,
        P10: Fn(ParseInput) -> ParseResult<A>,
        P11: Fn(ParseInput) -> ParseResult<A>,
{
    move |(input, state)| match parser1((input, state.clone())) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator) => match parser2((input, state.clone())) {
            Ok(parse_result2) => Ok(parse_result2),
            Err(ParseError::Combinator) => match parser3((input, state.clone())) {
                Ok(parse_result3) => Ok(parse_result3),
                Err(ParseError::Combinator) => match parser4((input, state.clone())) {
                    Ok(parse_result4) => Ok(parse_result4),
                    Err(ParseError::Combinator) => match parser5((input, state.clone())) {
                        Ok(parse_result5) => Ok(parse_result5),
                        Err(ParseError::Combinator) => match parser6((input, state.clone())) {
                            Ok(parse_result6) => Ok(parse_result6),
                            Err(ParseError::Combinator) => match parser7((input, state.clone())) {
                                Ok(parse_result7) => Ok(parse_result7),
                                Err(ParseError::Combinator) => match parser8((input, state.clone())) {
                                    Ok(parse_result8) => Ok(parse_result8),
                                    Err(ParseError::Combinator) => match parser9((input, state.clone())) {
                                        Ok(parse_result9) => Ok(parse_result9),
                                        Err(ParseError::Combinator) => match parser10((input, state.clone())) {
                                            Ok(parse_result10) => Ok(parse_result10),
                                            Err(ParseError::Combinator) => match parser11((input, state)) {
                                                Ok(parse_result11) => Ok(parse_result11),
                                                Err(err) => Err(err),
                                            },
                                            Err(err) => Err(err),
                                        },
                                        Err(err) => Err(err),
                                    },
                                    Err(err) => Err(err),
                                },
                                Err(err) => Err(err),
                            },
                            Err(err) => Err(err),
                        },
                        Err(err) => Err(err),
                    },
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}
