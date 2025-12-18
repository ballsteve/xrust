use crate::item::Node;
use crate::parser::{ParseError, ParseInput, StaticState};
use qualname::{NamespacePrefix, NamespaceUri};

pub(crate) fn delimited<'a, P1, P2, P3, R1, R2, R3, N: Node, L>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, R2), ParseError>
where
    P1: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, R1), ParseError>,
    P2: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, R2), ParseError>,
    P3: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, R3), ParseError>,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |input, ss| match parser1(input, ss) {
        Ok((input1, _)) => match parser2(input1, ss) {
            Ok((input2, result2)) => match parser3(input2, ss) {
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
    use qualname::NamespaceUri;

    use crate::parser::combinators::delimited::delimited;
    use crate::parser::combinators::tag::tag;
    use crate::parser::{ParseError, ParserState, StaticStateBuilder};
    use crate::trees::nullo::Nullo;

    #[test]
    fn parser_delimited_test1() {
        let testdoc = "<doc>";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = delimited(tag("<"), tag("doc"), tag(">"));

        assert_eq!(
            Ok((("", ParserState::new()), ())),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }
}
