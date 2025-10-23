use crate::item::Node;
use crate::parser::{ParseError, ParseInput, StaticState};
use qualname::{NamespacePrefix, NamespaceUri};

pub(crate) fn opt<'a, P1, R1, N: Node, L>(
    parser1: P1,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, Option<R1>), ParseError>
where
    P1: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, R1), ParseError>,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |input, ss| match parser1(input.clone(), ss) {
        Ok((input1, result1)) => Ok((input1, Some(result1))),
        Err(ParseError::Combinator(_)) => Ok((input, None)),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::combinators::opt::opt;
    use crate::parser::combinators::tag::tag;
    use crate::parser::{ParseError, ParserState, StaticStateBuilder};
    use crate::trees::nullo::Nullo;
    use qualname::NamespaceUri;

    #[test]
    fn parser_opt_test1() {
        let testdoc = "<doc>";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = opt(tag("<"));
        assert_eq!(
            Ok((("doc>", ParserState::new()), Some(()))),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }

    #[test]
    fn parser_opt_test2() {
        let testdoc = "<doc>";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = opt(tag(">"));
        assert_eq!(
            Ok((("<doc>", ParserState::new()), None)),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }
}
