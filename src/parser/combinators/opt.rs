use crate::parser::{ParseError, ParseInput, ParseResult};

pub(crate) fn opt<P1, R1>(parser1: P1) -> impl Fn(ParseInput) -> ParseResult<Option<R1>>
where
    P1: Fn(ParseInput) -> ParseResult<R1>,
{
    move |input| match parser1(input.clone()) {
        Ok((input1, result1)) => Ok((input1, Some(result1))),
        Err(ParseError::Combinator) => Ok((input, None)),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::combinators::opt::opt;
    use crate::parser::combinators::tag::tag;
    use crate::parser::ParserState;

    #[test]
    fn parser_opt_test1() {
        let testdoc = "<doc>";
        let teststate = ParserState::new(None);
        let parse_doc = opt(tag("<"));
        assert_eq!(Ok((("doc>", ParserState::new(None)),   Some(()))), parse_doc((testdoc, teststate)));
    }

    #[test]
    fn parser_opt_test2() {
        let testdoc = "<doc>";
        let teststate = ParserState::new(None);
        let parse_doc = opt(tag(">"));
        assert_eq!(Ok((("<doc>", ParserState::new(None)),   None)), parse_doc((testdoc, teststate)));
    }
}
