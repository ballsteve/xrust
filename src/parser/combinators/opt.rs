use crate::item::Node;
use crate::parser::{ParseError, ParseInput};
use crate::qname::Interner;

pub(crate) fn opt<'a, 'i, P1, R1, I: Interner + 'i, N: Node>(
    parser1: P1,
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Option<R1>), ParseError>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R1), ParseError>,
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
        let teststate: ParserState<Nullo> = ParserState::new(None, None, None);
        let parse_doc = opt(tag("<"));
        assert_eq!(
            Ok((("doc>", ParserState::new(None, None, None)), Some(()))),
            parse_doc((testdoc, teststate))
        );
    }

    #[test]
    fn parser_opt_test2() {
        let testdoc = "<doc>";
        let teststate: ParserState<Nullo> = ParserState::new(None, None, None);
        let parse_doc = opt(tag(">"));
        assert_eq!(
            Ok((("<doc>", ParserState::new(None, None, None)), None)),
            parse_doc((testdoc, teststate))
        );
    }
}
