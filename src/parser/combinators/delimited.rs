use crate::item::Node;
use crate::parser::{ParseError, ParseInput};
use crate::qname::Interner;

pub(crate) fn delimited<'a, 'i, P1, P2, P3, R1, R2, R3, I: Interner + 'i, N: Node>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R2), ParseError>
where
    P1: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R1), ParseError>,
    P2: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R2), ParseError>,
    P3: Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, R3), ParseError>,
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
    use crate::trees::nullo::Nullo;

    #[test]
    fn parser_delimited_test1() {
        let testdoc = "<doc>";
        let teststate: ParserState<Nullo> = ParserState::new(None, None, None);
        let parse_doc = delimited(tag("<"), tag("doc"), tag(">"));

        assert_eq!(
            Ok((("", ParserState::new(None, None, None)), ())),
            parse_doc((testdoc, teststate))
        );
    }
}
