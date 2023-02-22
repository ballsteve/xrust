use crate::parser::{ParseError, ParseInput, ParseResult};

pub(crate) fn tag(expected: &str) -> impl Fn(ParseInput) -> ParseResult<()> + '_ {
    move |mut input| {
        let tagchars = expected.chars();
        for tchar in tagchars {
            match input.next() {
                None => return Err(ParseError::Combinator),
                Some(char) => {
                    if char != tchar {
                        return Err(ParseError::Combinator);
                    }
                }
            }
        }
        Ok((input, ()))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::combinators::tag::tag;
    use crate::parser::ParseError;
    use crate::parser::ParseInput;

    #[test]
    fn parser_tag_test1() {
        let testdoc = ParseInput::new("<doc>");
        let parse_doc = tag("<");
        assert_eq!(Ok((ParseInput::new("<doc>"), ())), parse_doc(testdoc));
    }

    #[test]
    fn parser_tag_test2() {
        let testdoc = ParseInput::new("<doc>");
        let parse_doc = tag(">");
        assert_eq!(Err(ParseError::Combinator), parse_doc(testdoc));
    }
}
