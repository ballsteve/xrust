use crate::parser::{ParseError, ParseInput, ParseResult};

pub fn separated_list0<P1, P2, R1>(
    sep: P1,
    f: P2,
) -> impl Fn(ParseInput) -> ParseResult<Vec<R1>>
where
    P1: Fn(ParseInput) -> ParseResult<()>,
    P2: Fn(ParseInput) -> ParseResult<R1>,
{
    move |mut input| {
        let mut res = Vec::new();

        match f(input.clone()) {
            Err(_e) => {
                return Ok((input, res));
            }
            Ok((i1, o)) => {
                res.push(o);
                input = i1;
            }
        }

        loop {
            match sep(input.clone()) {
                Err(ParseError::Combinator) => {
                    return Ok((input, res));
                }
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    // infinite loop check: the parser must always consume
                    // SRB: not sure if this check is necessary with this parser, since input is an iterator
                    //		    if i1.input_len() == len {
                    //			return Err(ParseError::Combinator);
                    //		    }

                    match f(i1) {
                        Err(ParseError::Combinator) => return Ok((input, res)),
                        Err(e) => return Err(e),
                        Ok((i2, o)) => {
                            res.push(o);
                            input = i2;
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn separated_list1<P1, P2, R1>(
    sep: P1,
    f: P2,
) -> impl Fn(ParseInput) -> ParseResult<Vec<R1>>
where
    P1: Fn(ParseInput) -> ParseResult<()>,
    P2: Fn(ParseInput) -> ParseResult<R1>,
{
    move |mut input| {
        let mut res = Vec::new();

        match f(input.clone()) {
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                res.push(o);
                input = i1;
            }
        }

        loop {
            match sep(input.clone()) {
                Err(ParseError::Combinator) => {
                    return Ok((input, res));
                }
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    // infinite loop check: the parser must always consume
                    // SRB: not sure if this check is necessary with this parser, since input is an iterator
                    //		    if i1.input_len() == len {
                    //			return Err(ParseError::Combinator);
                    //		    }

                    match f(i1) {
                        Err(ParseError::Combinator) => {
                            return Ok((input, res));
                        }
                        Err(e) => return Err(e),
                        Ok((i2, o)) => {
                            res.push(o);
                            input = i2;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::combinators::alt::alt4;
    use crate::parser::combinators::list::{separated_list0, separated_list1};
    use crate::parser::combinators::map::map;
    use crate::parser::combinators::tag::tag;
    use crate::parser::ParserState;

    #[test]
    fn parser_separated_list0_test1() {
        let testdoc = "b";
        let teststate = ParserState::new(None, None);
        let parse_doc = separated_list0(tag(","), map(tag("a"), |_| "a"));

        assert_eq!(
            Ok((("b", ParserState::new(None, None)), vec![])),
            parse_doc((testdoc, teststate))
        );
    }

    #[test]
    fn parser_separated_list0_test2() {
        let testdoc = "a";
        let teststate = ParserState::new(None, None);
        let parse_doc = separated_list0(tag(","), map(tag("a"), |_| "a"));

        assert_eq!(
            Ok((("", ParserState::new(None, None)), vec!["a"])),
            parse_doc((testdoc, teststate))
        );
    }

    #[test]
    fn parser_separated_list0_test3() {
        let testdoc = "a,b,c,d";
        let teststate = ParserState::new(None, None);
        let parse_doc = separated_list1(
            tag(","),
            alt4(
                map(tag("a"), |_| "1"),
                map(tag("b"), |_| "2"),
                map(tag("c"), |_| "3"),
                map(tag("d"), |_| "4"),
            ),
        );

        assert_eq!(
            Ok((("", ParserState::new(None, None)), vec!["1", "2", "3", "4"])),
            parse_doc((testdoc, teststate))
        );
    }

    #[test]
    fn parser_separated_list1_test1() {
        let testdoc = "a";
        let teststate = ParserState::new(None, None);
        let parse_doc = separated_list1(tag(","), map(tag("a"), |_| "a"));

        assert_eq!(
            Ok((("", ParserState::new(None, None)), vec!["a"])),
            parse_doc((testdoc, teststate))
        );
    }

    #[test]
    fn parser_separated_list1_test2() {
        let testdoc = "a,b,c,d";
        let teststate = ParserState::new(None, None);
        let parse_doc = separated_list1(
            tag(","),
            alt4(
                map(tag("a"), |_| "1"),
                map(tag("b"), |_| "2"),
                map(tag("c"), |_| "3"),
                map(tag("d"), |_| "4"),
            ),
        );

        assert_eq!(
            Ok((("", ParserState::new(None, None)), vec!["1", "2", "3", "4"])),
            parse_doc((testdoc, teststate))
        );
    }
}
