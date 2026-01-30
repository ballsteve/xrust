use crate::item::Node;
use crate::parser::{ParseError, ParseInput, StaticState};
use qualname::{NamespacePrefix, NamespaceUri};

pub fn alt2<'a, P1, P2, A, N: Node, L>(
    parser1: P1,
    parser2: P2,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>
where
    P1: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P2: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), ss| match parser1((input, state.clone()), ss) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator(_)) | Err(ParseError::NotWellFormed(_)) => {
            parser2((input, state), ss)
        }
        Err(err) => Err(err),
    }
}

pub fn alt3<'a, P1, P2, P3, A, N: Node, L>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>
where
    P1: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P2: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P3: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), ss| match parser1((input, state.clone()), ss) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator(_)) | Err(ParseError::NotWellFormed(_)) => {
            match parser2((input, state.clone()), ss) {
                Ok(parse_result2) => Ok(parse_result2),
                Err(ParseError::Combinator(_)) | Err(ParseError::NotWellFormed(_)) => {
                    match parser3((input, state), ss) {
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

pub fn alt4<'a, P1, P2, P3, P4, A, N: Node, L>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>
where
    P1: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P2: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P3: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P4: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), ss| match parser1((input, state.clone()), ss) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator(_)) | Err(ParseError::NotWellFormed(_)) => {
            match parser2((input, state.clone()), ss) {
                Ok(parse_result2) => Ok(parse_result2),
                Err(ParseError::Combinator(_)) | Err(ParseError::NotWellFormed(_)) => {
                    match parser3((input, state.clone()), ss) {
                        Ok(parse_result3) => Ok(parse_result3),
                        Err(ParseError::Combinator(_)) | Err(ParseError::NotWellFormed(_)) => {
                            match parser4((input, state), ss) {
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

pub(crate) fn alt5<'a, P1, P2, P3, P4, P5, A, N: Node, L>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>
where
    P1: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P2: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P3: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P4: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P5: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), ss| match parser1((input, state.clone()), ss) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator(_)) | Err(ParseError::NotWellFormed(_)) => {
            match parser2((input, state.clone()), ss) {
                Ok(parse_result2) => Ok(parse_result2),
                Err(ParseError::Combinator(_)) | Err(ParseError::NotWellFormed(_)) => {
                    match parser3((input, state.clone()), ss) {
                        Ok(parse_result3) => Ok(parse_result3),
                        Err(ParseError::Combinator(_)) | Err(ParseError::NotWellFormed(_)) => {
                            match parser4((input, state.clone()), ss) {
                                Ok(parse_result4) => Ok(parse_result4),
                                Err(ParseError::Combinator(_))
                                | Err(ParseError::NotWellFormed(_)) => {
                                    match parser5((input, state), ss) {
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

pub(crate) fn alt6<'a, P1, P2, P3, P4, P5, P6, A, N: Node, L>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
    parser6: P6,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>
where
    P1: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P2: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P3: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P4: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P5: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P6: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), ss| match parser1((input, state.clone()), ss) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator(_)) => match parser2((input, state.clone()), ss) {
            Ok(parse_result2) => Ok(parse_result2),
            Err(ParseError::Combinator(_)) => match parser3((input, state.clone()), ss) {
                Ok(parse_result3) => Ok(parse_result3),
                Err(ParseError::Combinator(_)) => match parser4((input, state.clone()), ss) {
                    Ok(parse_result4) => Ok(parse_result4),
                    Err(ParseError::Combinator(_)) => match parser5((input, state.clone()), ss) {
                        Ok(parse_result5) => Ok(parse_result5),
                        Err(ParseError::Combinator(_)) => match parser6((input, state.clone()), ss)
                        {
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
pub(crate) fn alt7<'a, P1, P2, P3, P4, P5, P6, P7, A, N: Node, L>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
    parser6: P6,
    parser7: P7,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>
where
    P1: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P2: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P3: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P4: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P5: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P6: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P7: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), ss| match parser1((input, state.clone()), ss) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator(_)) => match parser2((input, state.clone()), ss) {
            Ok(parse_result2) => Ok(parse_result2),
            Err(ParseError::Combinator(_)) => match parser3((input, state.clone()), ss) {
                Ok(parse_result3) => Ok(parse_result3),
                Err(ParseError::Combinator(_)) => match parser4((input, state.clone()), ss) {
                    Ok(parse_result4) => Ok(parse_result4),
                    Err(ParseError::Combinator(_)) => match parser5((input, state.clone()), ss) {
                        Ok(parse_result5) => Ok(parse_result5),
                        Err(ParseError::Combinator(_)) => match parser6((input, state.clone()), ss)
                        {
                            Ok(parse_result6) => Ok(parse_result6),
                            Err(ParseError::Combinator(_)) => match parser7((input, state), ss) {
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
pub(crate) fn alt8<'a, P1, P2, P3, P4, P5, P6, P7, P8, A, N: Node>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
    parser6: P6,
    parser7: P7,
    parser8: P8,
) -> impl Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>
where
    P1: Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P2: Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P3: Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P4: Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P5: Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P6: Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P7: Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P8: Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>,
{
    move |(input, state)| match parser1((input, state.clone())) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator(_)) => match parser2((input, state.clone())) {
            Ok(parse_result2) => Ok(parse_result2),
            Err(ParseError::Combinator(_)) => match parser3((input, state.clone())) {
                Ok(parse_result3) => Ok(parse_result3),
                Err(ParseError::Combinator(_)) => match parser4((input, state.clone())) {
                    Ok(parse_result4) => Ok(parse_result4),
                    Err(ParseError::Combinator(_)) => match parser5((input, state.clone())) {
                        Ok(parse_result5) => Ok(parse_result5),
                        Err(ParseError::Combinator(_)) => match parser6((input, state.clone())) {
                            Ok(parse_result6) => Ok(parse_result6),
                            Err(ParseError::Combinator(_)) => match parser7((input, state.clone())) {
                                Ok(parse_result7) => Ok(parse_result7),
                                Err(ParseError::Combinator(_)) => match parser8((input, state)) {
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
pub(crate) fn alt9<'a, P1, P2, P3, P4, P5, P6, P7, P8, P9, A, N: Node, L>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
    parser6: P6,
    parser7: P7,
    parser8: P8,
    parser9: P9,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>
where
    P1: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P2: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P3: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P4: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P5: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P6: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P7: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P8: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P9: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), ss| match parser1((input, state.clone()), ss) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator(_)) => match parser2((input, state.clone()), ss) {
            Ok(parse_result2) => Ok(parse_result2),
            Err(ParseError::Combinator(_)) => match parser3((input, state.clone()), ss) {
                Ok(parse_result3) => Ok(parse_result3),
                Err(ParseError::Combinator(_)) => match parser4((input, state.clone()), ss) {
                    Ok(parse_result4) => Ok(parse_result4),
                    Err(ParseError::Combinator(_)) => match parser5((input, state.clone()), ss) {
                        Ok(parse_result5) => Ok(parse_result5),
                        Err(ParseError::Combinator(_)) => match parser6((input, state.clone()), ss)
                        {
                            Ok(parse_result6) => Ok(parse_result6),
                            Err(ParseError::Combinator(_)) => {
                                match parser7((input, state.clone()), ss) {
                                    Ok(parse_result7) => Ok(parse_result7),
                                    Err(ParseError::Combinator(_)) => {
                                        match parser8((input, state.clone()), ss) {
                                            Ok(parse_result8) => Ok(parse_result8),
                                            Err(ParseError::Combinator(_)) => {
                                                match parser9((input, state), ss) {
                                                    Ok(parse_result9) => Ok(parse_result9),
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
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn alt10<'a, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, A, N: Node, L>(
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
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>
where
    P1: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P2: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P3: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P4: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P5: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P6: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P7: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P8: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P9: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    P10: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError>,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), ss| match parser1((input, state.clone()), ss) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator(_)) => match parser2((input, state.clone()), ss) {
            Ok(parse_result2) => Ok(parse_result2),
            Err(ParseError::Combinator(_)) => match parser3((input, state.clone()), ss) {
                Ok(parse_result3) => Ok(parse_result3),
                Err(ParseError::Combinator(_)) => match parser4((input, state.clone()), ss) {
                    Ok(parse_result4) => Ok(parse_result4),
                    Err(ParseError::Combinator(_)) => match parser5((input, state.clone()), ss) {
                        Ok(parse_result5) => Ok(parse_result5),
                        Err(ParseError::Combinator(_)) => match parser6((input, state.clone()), ss)
                        {
                            Ok(parse_result6) => Ok(parse_result6),
                            Err(ParseError::Combinator(_)) => {
                                match parser7((input, state.clone()), ss) {
                                    Ok(parse_result7) => Ok(parse_result7),
                                    Err(ParseError::Combinator(_)) => {
                                        match parser8((input, state.clone()), ss) {
                                            Ok(parse_result8) => Ok(parse_result8),
                                            Err(ParseError::Combinator(_)) => {
                                                match parser9((input, state.clone()), ss) {
                                                    Ok(parse_result9) => Ok(parse_result9),
                                                    Err(ParseError::Combinator(_)) => {
                                                        match parser10((input, state), ss) {
                                                            Ok(parse_result10) => {
                                                                Ok(parse_result10)
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
pub(crate) fn alt11<'a, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, A, N: Node>(
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
) -> impl Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>
    where
        P1: Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>,
        P2: Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>,
        P3: Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>,
        P4: Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>,
        P5: Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>,
        P6: Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>,
        P7: Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>,
        P8: Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>,
        P9: Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>,
        P10: Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>,
        P11: Fn(ParseInput<'a, N>) -> Result<(ParseInput<'a, N>, A), ParseError>,
{
    move |(input, state)| match parser1((input, state.clone())) {
        Ok(parse_result) => Ok(parse_result),
        Err(ParseError::Combinator(_)) => match parser2((input, state.clone())) {
            Ok(parse_result2) => Ok(parse_result2),
            Err(ParseError::Combinator(_)) => match parser3((input, state.clone())) {
                Ok(parse_result3) => Ok(parse_result3),
                Err(ParseError::Combinator(_)) => match parser4((input, state.clone())) {
                    Ok(parse_result4) => Ok(parse_result4),
                    Err(ParseError::Combinator(_)) => match parser5((input, state.clone())) {
                        Ok(parse_result5) => Ok(parse_result5),
                        Err(ParseError::Combinator(_)) => match parser6((input, state.clone())) {
                            Ok(parse_result6) => Ok(parse_result6),
                            Err(ParseError::Combinator(_)) => match parser7((input, state.clone())) {
                                Ok(parse_result7) => Ok(parse_result7),
                                Err(ParseError::Combinator(_)) => match parser8((input, state.clone())) {
                                    Ok(parse_result8) => Ok(parse_result8),
                                    Err(ParseError::Combinator(_)) => match parser9((input, state.clone())) {
                                        Ok(parse_result9) => Ok(parse_result9),
                                        Err(ParseError::Combinator(_)) => match parser10((input, state.clone())) {
                                            Ok(parse_result10) => Ok(parse_result10),
                                            Err(ParseError::Combinator(_)) => match parser11((input, state)) {
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
