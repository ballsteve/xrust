use crate::item::Node;
use crate::parser::{ParseError, ParseInput};

pub(crate) fn opt<P1, R1, N: Node>(
    parser1: P1,
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, Option<R1>), ParseError>
where
    P1: Fn(ParseInput<N>) -> Result<(ParseInput<N>, R1), ParseError>,
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
    use crate::trees::nullo::Nullo;

    #[test]
    fn parser_opt_test1() {
        let testdoc = "<doc>";
        let teststate: ParserState<Nullo> = ParserState::new(None, None);
        let parse_doc = opt(tag("<"));
        assert_eq!(
            Ok((("doc>", ParserState::new(None, None)), Some(()))),
            parse_doc((testdoc, teststate))
        );
    }

    #[test]
    fn parser_opt_test2() {
        let testdoc = "<doc>";
        let teststate: ParserState<Nullo> = ParserState::new(None, None);
        let parse_doc = opt(tag(">"));
        assert_eq!(
            Ok((("<doc>", ParserState::new(None, None)), None)),
            parse_doc((testdoc, teststate))
        );
    }
}
