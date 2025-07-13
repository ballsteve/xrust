use crate::item::Node;
use crate::parser::{ParseError, ParseInput};
use crate::qname::Interner;

pub fn alt2<'a, 'i, P1, P2, A, I: Interner + 'i, N: Node>(
    parser1: P1,
    parser2: P2,
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P2: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
{
    move |(input, state)| match parser1((input, state.clone())) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator) | Err(ParseError::NotWellFormed(_)) => {
            match parser2((input, state)) {
                Ok(parse_result2) => Ok(parse_result2),
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

pub fn alt3<'a, 'i, P1, P2, P3, A, I: Interner + 'i, N: Node>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P2: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P3: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
{
    move |(input, state)| match parser1((input, state.clone())) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator) | Err(ParseError::NotWellFormed(_)) => {
            match parser2((input, state.clone())) {
                Ok(parse_result2) => Ok(parse_result2),
                Err(ParseError::Combinator) | Err(ParseError::NotWellFormed(_)) => {
                    match parser3((input, state)) {
                        Ok(parse_result3) => Ok(parse_result3),
                        Err(err) => Err(err),
                    }
                }
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

pub fn alt4<'a, 'i, P1, P2, P3, P4, A, I: Interner + 'i, N: Node>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P2: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P3: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P4: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
{
    move |(input, state)| match parser1((input, state.clone())) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator) | Err(ParseError::NotWellFormed(_)) => {
            match parser2((input, state.clone())) {
                Ok(parse_result2) => Ok(parse_result2),
                Err(ParseError::Combinator) | Err(ParseError::NotWellFormed(_)) => {
                    match parser3((input, state.clone())) {
                        Ok(parse_result3) => Ok(parse_result3),
                        Err(ParseError::Combinator) | Err(ParseError::NotWellFormed(_)) => {
                            match parser4((input, state)) {
                                Ok(parse_result4) => Ok(parse_result4),
                                Err(err) => Err(err),
                            }
                        }
                        Err(err) => Err(err),
                    }
                }
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

pub(crate) fn alt5<'a, 'i, P1, P2, P3, P4, P5, A, I: Interner + 'i, N: Node>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P2: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P3: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P4: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P5: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
{
    move |(input, state)| match parser1((input, state.clone())) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator) | Err(ParseError::NotWellFormed(_)) => {
            match parser2((input, state.clone())) {
                Ok(parse_result2) => Ok(parse_result2),
                Err(ParseError::Combinator) | Err(ParseError::NotWellFormed(_)) => {
                    match parser3((input, state.clone())) {
                        Ok(parse_result3) => Ok(parse_result3),
                        Err(ParseError::Combinator) | Err(ParseError::NotWellFormed(_)) => {
                            match parser4((input, state.clone())) {
                                Ok(parse_result4) => Ok(parse_result4),
                                Err(ParseError::Combinator) | Err(ParseError::NotWellFormed(_)) => {
                                    match parser5((input, state)) {
                                        Ok(parse_result5) => Ok(parse_result5),
                                        Err(err) => Err(err),
                                    }
                                }
                                Err(err) => Err(err),
                            }
                        }
                        Err(err) => Err(err),
                    }
                }
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

pub(crate) fn alt6<'a, 'i, P1, P2, P3, P4, P5, P6, A, I: Interner + 'i, N: Node>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
    parser6: P6,
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P2: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P3: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P4: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P5: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P6: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
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

#[allow(dead_code)]
pub(crate) fn alt7<'a, 'i, P1, P2, P3, P4, P5, P6, P7, A, I: Interner + 'i, N: Node>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
    parser6: P6,
    parser7: P7,
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P2: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P3: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P4: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P5: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P6: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P7: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
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

/*
#[allow(clippy::too_many_arguments)]
pub(crate) fn alt8<'a, 'i, P1, P2, P3, P4, P5, P6, P7, P8, A, I: Interner, N: Node>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
    parser6: P6,
    parser7: P7,
    parser8: P8,
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P2: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P3: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P4: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P5: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P6: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P7: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P8: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
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
 */

#[allow(clippy::too_many_arguments)]
pub(crate) fn alt9<'a, 'i, P1, P2, P3, P4, P5, P6, P7, P8, P9, A, I: Interner + 'i, N: Node>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
    parser6: P6,
    parser7: P7,
    parser8: P8,
    parser9: P9,
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P2: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P3: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P4: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P5: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P6: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P7: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P8: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P9: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
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
                                Err(ParseError::Combinator) => {
                                    match parser8((input, state.clone())) {
                                        Ok(parse_result8) => Ok(parse_result8),
                                        Err(ParseError::Combinator) => {
                                            match parser9((input, state)) {
                                                Ok(parse_result9) => Ok(parse_result9),
                                                Err(err) => Err(err),
                                            }
                                        }
                                        Err(err) => Err(err),
                                    }
                                }
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
pub(crate) fn alt10<'a, 'i, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, A, I: Interner + 'i, N: Node>(
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
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P2: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P3: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P4: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P5: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P6: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P7: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P8: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P9: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
    P10: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
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
                                Err(ParseError::Combinator) => {
                                    match parser8((input, state.clone())) {
                                        Ok(parse_result8) => Ok(parse_result8),
                                        Err(ParseError::Combinator) => {
                                            match parser9((input, state.clone())) {
                                                Ok(parse_result9) => Ok(parse_result9),
                                                Err(ParseError::Combinator) => {
                                                    match parser10((input, state)) {
                                                        Ok(parse_result10) => Ok(parse_result10),
                                                        Err(err) => Err(err),
                                                    }
                                                }
                                                Err(err) => Err(err),
                                            }
                                        }
                                        Err(err) => Err(err),
                                    }
                                }
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

/*
#[allow(clippy::too_many_arguments)]
pub(crate) fn alt11<'a, 'i, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, A, I: Interner, N: Node>(
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
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>
    where
        P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
        P2: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
        P3: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
        P4: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
        P5: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
        P6: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
        P7: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
        P8: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
        P9: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
        P10: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
        P11: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, A), ParseError>,
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
 */
