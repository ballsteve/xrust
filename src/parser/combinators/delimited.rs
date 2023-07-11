use crate::parser::{ParseInput, ParseResult};

pub(crate) fn delimited<P1, P2, P3, R1, R2, R3>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
) -> impl Fn(ParseInput) -> ParseResult<R2>
where
    P1: Fn(ParseInput) -> ParseResult<R1>,
    P2: Fn(ParseInput) -> ParseResult<R2>,
    P3: Fn(ParseInput) -> ParseResult<R3>,
{
    move |input| match parser1(input) {
        Ok((input1, _)) => match parser2(input1) {
            Ok((input2, result2)) => match parser3(input2) {
                Ok((input3, _)) => Ok((input3, result2)),
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::combinators::delimited::delimited;
    use crate::parser::combinators::tag::tag;
    use crate::parser::ParserState;

    #[test]
    fn parser_delimited_test1() {
        let testdoc = "<doc>";
        let teststate = ParserState::new(None, None);
        let parse_doc = delimited(tag("<"), tag("doc"), tag(">"));

        assert_eq!(
            Ok((("", ParserState::new(None, None)), ())),
            parse_doc((testdoc, teststate))
        );
    }
}
