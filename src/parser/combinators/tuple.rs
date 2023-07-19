use crate::parser::{ParseInput, ParseResult};

pub(crate) fn tuple2<P1, P2, R1, R2>(
    parser1: P1,
    parser2: P2,
) -> impl Fn(ParseInput) -> ParseResult<(R1, R2)>
where
    P1: Fn(ParseInput) -> ParseResult<R1>,
    P2: Fn(ParseInput) -> ParseResult<R2>,
{
    move |input| match parser1(input) {
        Ok((input1, result1)) => match parser2(input1) {
            Ok((input2, result2)) => Ok((input2, (result1, result2))),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

pub(crate) fn tuple3<P1, P2, P3, R1, R2, R3>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
) -> impl Fn(ParseInput) -> ParseResult<(R1, R2, R3)>
where
    P1: Fn(ParseInput) -> ParseResult<R1>,
    P2: Fn(ParseInput) -> ParseResult<R2>,
    P3: Fn(ParseInput) -> ParseResult<R3>,
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

pub(crate) fn tuple4<P1, P2, P3, P4, R1, R2, R3, R4>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
) -> impl Fn(ParseInput) -> ParseResult<(R1, R2, R3, R4)>
where
    P1: Fn(ParseInput) -> ParseResult<R1>,
    P2: Fn(ParseInput) -> ParseResult<R2>,
    P3: Fn(ParseInput) -> ParseResult<R3>,
    P4: Fn(ParseInput) -> ParseResult<R4>,
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

pub(crate) fn tuple5<P1, P2, P3, P4, P5, R1, R2, R3, R4, R5>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
) -> impl Fn(ParseInput) -> ParseResult<(R1, R2, R3, R4, R5)>
where
    P1: Fn(ParseInput) -> ParseResult<R1>,
    P2: Fn(ParseInput) -> ParseResult<R2>,
    P3: Fn(ParseInput) -> ParseResult<R3>,
    P4: Fn(ParseInput) -> ParseResult<R4>,
    P5: Fn(ParseInput) -> ParseResult<R5>,
{
    move |mut input| {
        input.stack_push(format!("tuple5 - input=\"{}\"", input));
        match parser1(input) {
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
}

pub(crate) fn tuple6<P1, P2, P3, P4, P5, P6, R1, R2, R3, R4, R5, R6>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
    parser6: P6,
) -> impl Fn(ParseInput) -> ParseResult<(R1, R2, R3, R4, R5, R6)>
where
    P1: Fn(ParseInput) -> ParseResult<R1>,
    P2: Fn(ParseInput) -> ParseResult<R2>,
    P3: Fn(ParseInput) -> ParseResult<R3>,
    P4: Fn(ParseInput) -> ParseResult<R4>,
    P5: Fn(ParseInput) -> ParseResult<R5>,
    P6: Fn(ParseInput) -> ParseResult<R6>,
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

pub(crate) fn tuple7<P1, P2, P3, P4, P5, P6, P7, R1, R2, R3, R4, R5, R6, R7>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
    parser6: P6,
    parser7: P7,
) -> impl Fn(ParseInput) -> ParseResult<(R1, R2, R3, R4, R5, R6, R7)>
where
    P1: Fn(ParseInput) -> ParseResult<R1>,
    P2: Fn(ParseInput) -> ParseResult<R2>,
    P3: Fn(ParseInput) -> ParseResult<R3>,
    P4: Fn(ParseInput) -> ParseResult<R4>,
    P5: Fn(ParseInput) -> ParseResult<R5>,
    P6: Fn(ParseInput) -> ParseResult<R6>,
    P7: Fn(ParseInput) -> ParseResult<R7>,
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
pub(crate) fn tuple8<P1, P2, P3, P4, P5, P6, P7, P8, R1, R2, R3, R4, R5, R6, R7, R8>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
    parser6: P6,
    parser7: P7,
    parser8: P8,
) -> impl Fn(ParseInput) -> ParseResult<(R1, R2, R3, R4, R5, R6, R7, R8)>
where
    P1: Fn(ParseInput) -> ParseResult<R1>,
    P2: Fn(ParseInput) -> ParseResult<R2>,
    P3: Fn(ParseInput) -> ParseResult<R3>,
    P4: Fn(ParseInput) -> ParseResult<R4>,
    P5: Fn(ParseInput) -> ParseResult<R5>,
    P6: Fn(ParseInput) -> ParseResult<R6>,
    P7: Fn(ParseInput) -> ParseResult<R7>,
    P8: Fn(ParseInput) -> ParseResult<R8>,
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
pub(crate) fn tuple9<P1, P2, P3, P4, P5, P6, P7, P8, P9, R1, R2, R3, R4, R5, R6, R7, R8, R9>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
    parser4: P4,
    parser5: P5,
    parser6: P6,
    parser7: P7,
    parser8: P8,
    parser9: P9,
) -> impl Fn(ParseInput) -> ParseResult<(R1, R2, R3, R4, R5, R6, R7, R8, R9)>
where
    P1: Fn(ParseInput) -> ParseResult<R1>,
    P2: Fn(ParseInput) -> ParseResult<R2>,
    P3: Fn(ParseInput) -> ParseResult<R3>,
    P4: Fn(ParseInput) -> ParseResult<R4>,
    P5: Fn(ParseInput) -> ParseResult<R5>,
    P6: Fn(ParseInput) -> ParseResult<R6>,
    P7: Fn(ParseInput) -> ParseResult<R7>,
    P8: Fn(ParseInput) -> ParseResult<R8>,
    P9: Fn(ParseInput) -> ParseResult<R9>,
{
    move |mut input| {
        input.stack_push(format!("tuple9 - input=\"{}\"", input));
        match parser1(input) {
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
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn tuple10<
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
) -> impl Fn(ParseInput) -> ParseResult<(R1, R2, R3, R4, R5, R6, R7, R8, R9, R10)>
where
    P1: Fn(ParseInput) -> ParseResult<R1>,
    P2: Fn(ParseInput) -> ParseResult<R2>,
    P3: Fn(ParseInput) -> ParseResult<R3>,
    P4: Fn(ParseInput) -> ParseResult<R4>,
    P5: Fn(ParseInput) -> ParseResult<R5>,
    P6: Fn(ParseInput) -> ParseResult<R6>,
    P7: Fn(ParseInput) -> ParseResult<R7>,
    P8: Fn(ParseInput) -> ParseResult<R8>,
    P9: Fn(ParseInput) -> ParseResult<R9>,
    P10: Fn(ParseInput) -> ParseResult<R10>,
{
    move |mut input| {
        input.stack_push(format!("tuple10 - input=\"{}\"", input));
        match parser1(input) {
            Ok((input1, result1)) => {
                eprintln!("parser1 OK, try parser 2");
                match parser2(input1) {
                    Ok((input2, result2)) => {
                        eprintln!("parser2 OK, try parser 3");
                        match parser3(input2) {
                            Ok((input3, result3)) => {
                                eprintln!("parser3 OK, try parser 4");
                                match parser4(input3) {
                                    Ok((input4, result4)) => {
                                        eprintln!("parser4 OK, try parser 5");
                                        match parser5(input4) {
                                            Ok((input5, result5)) => {
                                                eprintln!("parser5 OK, try parser 6");
                                                match parser6(input5) {
                                                    Ok((input6, result6)) => {
                                                        eprintln!("parser6 OK, try parser 7");
                                                        match parser7(input6) {
                                                            Ok((input7, result7)) => {
                                                                eprintln!(
                                                                    "parser7 OK, try parser 8"
                                                                );
                                                                match parser8(input7) {
                                                                    Ok((input8, result8)) => {
                                                                        eprintln!("parser8 OK, try parser 9");
                                                                        match parser9(input8) {
                                                                            Ok((
                                                                                input9,
                                                                                result9,
                                                                            )) => {
                                                                                eprintln!("parser9 OK, try parser 10");
                                                                                match parser10(input9) {
										    Ok((input10, result10)) => Ok((
											input10,
											(
											    result1, result2, result3, result4, result5,
											    result6, result7, result8, result9, result10,
											),
										    )),
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
            Err(err) => {
                eprintln!("parser1 failed");
                Err(err)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::combinators::tag::tag;
    use crate::parser::combinators::tuple::tuple3;
    use crate::parser::ParseInput;

    #[test]
    fn parser_delimited_test1() {
        let testdoc = ParseInput::new("<doc>");
        let parse_doc = tuple3(tag("<"), tag("doc"), tag(">"));
        assert_eq!(
            Ok((ParseInput::new("<doc>"), ((), (), ()))),
            parse_doc(testdoc)
        );
    }
}
