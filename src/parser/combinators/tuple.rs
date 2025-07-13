use crate::item::Node;
use crate::parser::{ParseError, ParseInput};
use crate::qname::Interner;

pub fn tuple2<'a, 'i, P1, P2, R1, R2, I: Interner + 'i, N: Node>(
    parser1: P1,
    parser2: P2,
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, (R1, R2)), ParseError>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R1), ParseError>,
    P2: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R2), ParseError>,
{
    move |input| match parser1(input) {
        Ok((input1, result1)) => match parser2(input1) {
            Ok((input2, result2)) => Ok((input2, (result1, result2))),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

pub fn tuple3<'a, 'i, P1, P2, P3, R1, R2, R3, I: Interner + 'i, N: Node>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, (R1, R2, R3)), ParseError>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R1), ParseError>,
    P2: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R2), ParseError>,
    P3: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R3), ParseError>,
{
    move |input| match parser1(input) {
        Ok((input1, result1)) => match parser2(input1) {
            Ok((input2, result2)) => match parser3(input2) {
                Ok((input3, result3)) => Ok((input3, (result1, result2, result3))),
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

pub fn tuple4<'a, 'i, P1, P2, P3, P4, R1, R2, R3, R4, I: Interner + 'i, N: Node>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, (R1, R2, R3, R4)), ParseError>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R1), ParseError>,
    P2: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R2), ParseError>,
    P3: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R3), ParseError>,
    P4: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R4), ParseError>,
{
    move |input| match parser1(input) {
        Ok((input1, result1)) => match parser2(input1) {
            Ok((input2, result2)) => match parser3(input2) {
                Ok((input3, result3)) => match parser4(input3) {
                    Ok((input4, result4)) => Ok((input4, (result1, result2, result3, result4))),
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

pub(crate) fn tuple5<'a, 'i, P1, P2, P3, P4, P5, R1, R2, R3, R4, R5, I: Interner + 'i, N: Node>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
) -> impl Fn(
    ParseInput<'a, 'i, I, N>,
) -> Result<(ParseInput<'a, 'i, I, N>, (R1, R2, R3, R4, R5)), ParseError>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R1), ParseError>,
    P2: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R2), ParseError>,
    P3: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R3), ParseError>,
    P4: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R4), ParseError>,
    P5: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R5), ParseError>,
{
    move |input| match parser1(input) {
        Ok((input1, result1)) => match parser2(input1) {
            Ok((input2, result2)) => match parser3(input2) {
                Ok((input3, result3)) => match parser4(input3) {
                    Ok((input4, result4)) => match parser5(input4) {
                        Ok((input5, result5)) => {
                            Ok((input5, (result1, result2, result3, result4, result5)))
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
    }
}

pub(crate) fn tuple6<
    'a,
    'i,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    I: Interner + 'i,
    N: Node,
>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
    parser6: P6,
) -> impl Fn(
    ParseInput<'a, 'i, I, N>,
) -> Result<(ParseInput<'a, 'i, I, N>, (R1, R2, R3, R4, R5, R6)), ParseError>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R1), ParseError>,
    P2: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R2), ParseError>,
    P3: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R3), ParseError>,
    P4: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R4), ParseError>,
    P5: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R5), ParseError>,
    P6: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R6), ParseError>,
{
    move |input| match parser1(input) {
        Ok((input1, result1)) => match parser2(input1) {
            Ok((input2, result2)) => match parser3(input2) {
                Ok((input3, result3)) => match parser4(input3) {
                    Ok((input4, result4)) => match parser5(input4) {
                        Ok((input5, result5)) => match parser6(input5) {
                            Ok((input6, result6)) => Ok((
                                input6,
                                (result1, result2, result3, result4, result5, result6),
                            )),
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

pub(crate) fn tuple7<
    'a,
    'i,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    I: Interner + 'i,
    N: Node,
>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
    parser6: P6,
    parser7: P7,
) -> impl Fn(
    ParseInput<'a, 'i, I, N>,
) -> Result<(ParseInput<'a, 'i, I, N>, (R1, R2, R3, R4, R5, R6, R7)), ParseError>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R1), ParseError>,
    P2: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R2), ParseError>,
    P3: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R3), ParseError>,
    P4: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R4), ParseError>,
    P5: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R5), ParseError>,
    P6: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R6), ParseError>,
    P7: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R7), ParseError>,
{
    move |input| match parser1(input) {
        Ok((input1, result1)) => match parser2(input1) {
            Ok((input2, result2)) => match parser3(input2) {
                Ok((input3, result3)) => match parser4(input3) {
                    Ok((input4, result4)) => match parser5(input4) {
                        Ok((input5, result5)) => match parser6(input5) {
                            Ok((input6, result6)) => match parser7(input6) {
                                Ok((input7, result7)) => Ok((
                                    input7,
                                    (
                                        result1, result2, result3, result4, result5, result6,
                                        result7,
                                    ),
                                )),
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
pub(crate) fn tuple8<
    'a,
    'i,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    I: Interner + 'i,
    N: Node,
>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
    parser6: P6,
    parser7: P7,
    parser8: P8,
) -> impl Fn(
    ParseInput<'a, 'i, I, N>,
) -> Result<(ParseInput<'a, 'i, I, N>, (R1, R2, R3, R4, R5, R6, R7, R8)), ParseError>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R1), ParseError>,
    P2: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R2), ParseError>,
    P3: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R3), ParseError>,
    P4: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R4), ParseError>,
    P5: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R5), ParseError>,
    P6: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R6), ParseError>,
    P7: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R7), ParseError>,
    P8: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R8), ParseError>,
{
    move |input| match parser1(input) {
        Ok((input1, result1)) => match parser2(input1) {
            Ok((input2, result2)) => match parser3(input2) {
                Ok((input3, result3)) => match parser4(input3) {
                    Ok((input4, result4)) => match parser5(input4) {
                        Ok((input5, result5)) => match parser6(input5) {
                            Ok((input6, result6)) => match parser7(input6) {
                                Ok((input7, result7)) => match parser8(input7) {
                                    Ok((input8, result8)) => Ok((
                                        input8,
                                        (
                                            result1, result2, result3, result4, result5, result6,
                                            result7, result8,
                                        ),
                                    )),
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
pub(crate) fn tuple9<
    'a,
    'i,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    I: Interner + 'i,
    N: Node,
>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
    parser6: P6,
    parser7: P7,
    parser8: P8,
    parser9: P9,
) -> impl Fn(
    ParseInput<'a, 'i, I, N>,
) -> Result<
    (
        ParseInput<'a, 'i, I, N>,
        (R1, R2, R3, R4, R5, R6, R7, R8, R9),
    ),
    ParseError,
>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R1), ParseError>,
    P2: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R2), ParseError>,
    P3: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R3), ParseError>,
    P4: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R4), ParseError>,
    P5: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R5), ParseError>,
    P6: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R6), ParseError>,
    P7: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R7), ParseError>,
    P8: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R8), ParseError>,
    P9: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R9), ParseError>,
{
    move |input| match parser1(input) {
        Ok((input1, result1)) => match parser2(input1) {
            Ok((input2, result2)) => match parser3(input2) {
                Ok((input3, result3)) => match parser4(input3) {
                    Ok((input4, result4)) => match parser5(input4) {
                        Ok((input5, result5)) => match parser6(input5) {
                            Ok((input6, result6)) => match parser7(input6) {
                                Ok((input7, result7)) => match parser8(input7) {
                                    Ok((input8, result8)) => match parser9(input8) {
                                        Ok((input9, result9)) => Ok((
                                            input9,
                                            (
                                                result1, result2, result3, result4, result5,
                                                result6, result7, result8, result9,
                                            ),
                                        )),
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

#[allow(clippy::too_many_arguments)]
pub(crate) fn tuple10<
    'a,
    'i,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
    P10,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    I: Interner + 'i,
    N: Node,
>(
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
) -> impl Fn(
    ParseInput<'a, 'i, I, N>,
) -> Result<
    (
        ParseInput<'a, 'i, I, N>,
        (R1, R2, R3, R4, R5, R6, R7, R8, R9, R10),
    ),
    ParseError,
>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R1), ParseError>,
    P2: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R2), ParseError>,
    P3: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R3), ParseError>,
    P4: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R4), ParseError>,
    P5: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R5), ParseError>,
    P6: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R6), ParseError>,
    P7: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R7), ParseError>,
    P8: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R8), ParseError>,
    P9: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R9), ParseError>,
    P10: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R10), ParseError>,
{
    move |input| match parser1(input) {
        Ok((input1, result1)) => match parser2(input1) {
            Ok((input2, result2)) => match parser3(input2) {
                Ok((input3, result3)) => match parser4(input3) {
                    Ok((input4, result4)) => match parser5(input4) {
                        Ok((input5, result5)) => match parser6(input5) {
                            Ok((input6, result6)) => match parser7(input6) {
                                Ok((input7, result7)) => match parser8(input7) {
                                    Ok((input8, result8)) => match parser9(input8) {
                                        Ok((input9, result9)) => match parser10(input9) {
                                            Ok((input10, result10)) => Ok((
                                                input10,
                                                (
                                                    result1, result2, result3, result4, result5,
                                                    result6, result7, result8, result9, result10,
                                                ),
                                            )),
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

#[cfg(test)]
mod tests {
    use crate::parser::combinators::tag::tag;
    use crate::parser::combinators::tuple::tuple3;
    use crate::parser::ParserState;
    use crate::qname::LocalInternment;
    use crate::trees::nullo::Nullo;

    #[test]
    fn parser_tuple3_test1() {
        let intern = LocalInternment::new();
        let testdoc = "<doc>";
        let teststate: ParserState<Nullo> = ParserState::new(None, None, None, &intern);
        let parse_doc = tuple3(tag("<"), tag("doc"), tag(">"));
        assert_eq!(
            Ok((
                ("", ParserState::new(None, None, None, &intern)),
                ((), (), ())
            )),
            parse_doc((testdoc, teststate))
        );
    }
}
