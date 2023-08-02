use crate::parser::{ParseError, ParseInput, ParseResult};

pub(crate) fn tag(expected: &str) -> impl Fn(ParseInput) -> ParseResult<()> + '_ {
    move |(input, state)|{
        match input.get(0..expected.len()) {
            None => Err(ParseError::Combinator),
            Some(chars) => {
                if chars == expected {
                    Ok(((&input[expected.len()..], state), ()))
                } else {
                    Err(ParseError::Combinator)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::combinators::tag::tag;
    use crate::parser::{ParseError, ParserState};

    #[test]
    fn parser_tag_test1() {
        let testdoc = "<doc>";
        let teststate = ParserState::new(None, None);
        let parse_doc = tag("<");
        assert_eq!(
            Ok((("doc>", ParserState::new(None, None)), ())),
            parse_doc((testdoc, teststate))
        );
    }

    #[test]
    fn parser_tag_test2() {
        let testdoc = "<doc>";
        let teststate = ParserState::new(None, None);
        let parse_doc = tag(">");
        assert_eq!(Err(ParseError::Combinator), parse_doc((testdoc, teststate)));
    }

    #[test]
    fn parser_tag_test3() {
        let testdoc = "<?ProcessingInstruction?>";
        let teststate = ParserState::new(None, None);
        let parse_doc = tag("<?");
        assert_eq!(
            Ok((
                ("ProcessingInstruction?>", ParserState::new(None, None)),
                ()
            )),
            parse_doc((testdoc, teststate))
        );
    }
}
