use crate::parser::{ParseError, ParseInput, ParseResult};

pub(crate) fn separated_list0<P1, P2, R1>(
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
                    eprintln!("sep0: didn't find any more seps, returning");
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

        //        eprintln!("sep1: find first element");
        match f(input.clone()) {
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                res.push(o);
                input = i1;
            }
        }

        //        eprintln!("sep1: got first element");
        loop {
            match sep(input.clone()) {
                Err(ParseError::Combinator) => {
                    //                    eprintln!("sep1 return due to end of input 1");
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
                            //                            eprintln!("sep1: return due to end of input 2");
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
    use crate::parser::ParseInput;

    #[test]
    fn parser_separated_list0_test1() {
        let testdoc = ParseInput::new("b");
        let parse_doc = separated_list0(tag(","), map(tag("a"), |_| "a"));
        let (remainder, result) = parse_doc(testdoc).expect("parse failed");
        assert_eq!(remainder, ParseInput::new("b"));
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn parser_separated_list0_test2() {
        let testdoc = ParseInput::new("a");
        let parse_doc = separated_list0(tag(","), map(tag("a"), |_| "a"));
        let (remainder, result) = parse_doc(testdoc).expect("parse failed");
        assert_eq!(remainder, ParseInput::new(""));
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "a");
    }

    #[test]
    fn parser_separated_list0_test3() {
        let testdoc = ParseInput::new("a,b,c,d");
        let parse_doc = separated_list1(
            tag(","),
            alt4(
                map(tag("a"), |_| "1"),
                map(tag("b"), |_| "2"),
                map(tag("c"), |_| "3"),
                map(tag("d"), |_| "4"),
            ),
        );
        let (remainder, result) = parse_doc(testdoc).expect("parse failed");
        assert_eq!(remainder, ParseInput::new(""));
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], "1");
        assert_eq!(result[1], "2");
        assert_eq!(result[2], "3");
        assert_eq!(result[3], "4");
    }

    #[test]
    fn parser_separated_list1_test1() {
        let testdoc = ParseInput::new("a");
        let parse_doc = separated_list1(tag(","), map(tag("a"), |_| "a"));
        let (remainder, result) = parse_doc(testdoc).expect("parse failed");
        assert_eq!(remainder, ParseInput::new(""));
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "a");
    }

    #[test]
    fn parser_separated_list1_test2() {
        let testdoc = ParseInput::new("a,b,c,d");
        let parse_doc = separated_list1(
            tag(","),
            alt4(
                map(tag("a"), |_| "1"),
                map(tag("b"), |_| "2"),
                map(tag("c"), |_| "3"),
                map(tag("d"), |_| "4"),
            ),
        );
        let (remainder, result) = parse_doc(testdoc).expect("parse failed");
        assert_eq!(remainder, ParseInput::new(""));
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], "1");
        assert_eq!(result[1], "2");
        assert_eq!(result[2], "3");
        assert_eq!(result[3], "4");
    }
}
