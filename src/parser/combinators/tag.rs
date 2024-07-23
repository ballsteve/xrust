use crate::item::Node;
use crate::parser::{ParseError, ParseInput};

pub fn tag<N: Node>(
    expected: &str,
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, ()), ParseError> + '_ {
    move |(input, state)| match input.get(0..expected.len()) {
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

/// Return the longest possible of one of the given tags.
/// If there are multiple tags of the same length, the first one that matches will be returned.
pub(crate) fn anytag<N: Node>(
    s: Vec<&str>,
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, String), ParseError> + '_ {
    move |(input, state)| {
        // NB. this algorithm could probably be optimised
        let u = s.iter().fold("", |result, t| {
            if t.len() > result.len() {
                // Since this tag is longer, it is a candidate
                match input.get(0..t.len()) {
                    None => result,
                    Some(chars) => {
                        if chars == *t {
                            t
                        } else {
                            result
                        }
                    }
                }
            } else {
                result
            }
        });
        if u == "" {
            Err(ParseError::Combinator)
        } else {
            Ok(((&input[u.len()..], state), u.to_string()))
        }
    }
}

pub(crate) fn anychar<N: Node>(
    expected: char,
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, ()), ParseError> {
    move |(input, state)| {
        if input.starts_with(expected) {
            Ok(((&input[1..], state), ()))
        } else {
            Err(ParseError::Combinator)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::combinators::tag::{anychar, anytag, tag};
    use crate::parser::{ParseError, ParserState};
    use crate::trees::nullo::Nullo;

    #[test]
    fn parser_tag_test1() {
        let testdoc = "<doc>";
        let teststate: ParserState<Nullo> = ParserState::new(None, None);
        let parse_doc = tag("<");
        assert_eq!(
            Ok((("doc>", ParserState::new(None, None)), ())),
            parse_doc((testdoc, teststate))
        );
    }

    #[test]
    fn parser_tag_test2() {
        let testdoc = "<doc>";
        let teststate: ParserState<Nullo> = ParserState::new(None, None);
        let parse_doc = tag(">");
        assert_eq!(Err(ParseError::Combinator), parse_doc((testdoc, teststate)));
    }

    #[test]
    fn parser_tag_test3() {
        let testdoc = "<?ProcessingInstruction?>";
        let teststate: ParserState<Nullo> = ParserState::new(None, None);
        let parse_doc = tag("<?");
        assert_eq!(
            Ok((
                (
                    "ProcessingInstruction?>",
                    ParserState::new(None, None)
                ),
                ()
            )),
            parse_doc((testdoc, teststate))
        );
    }

    #[test]
    fn parser_char_test1() {
        let testdoc = "<doc>";
        let teststate: ParserState<Nullo> = ParserState::new(None, None);
        let parse_doc = anychar('<');
        assert_eq!(
            Ok((("doc>", ParserState::new(None, None)), ())),
            parse_doc((testdoc, teststate))
        )
    }
    #[test]
    fn parser_char_test2() {
        let testdoc = "<doc>";
        let teststate: ParserState<Nullo> = ParserState::new(None, None);
        let parse_doc = anychar('>');
        assert_eq!(Err(ParseError::Combinator), parse_doc((testdoc, teststate)))
    }
    #[test]
    fn parser_anytag_test1() {
        let testdoc = "<doc>";
        let teststate: ParserState<Nullo> = ParserState::new(None, None);
        let parse_doc = anytag(vec![">", ">=", "<=", "<"]);
        assert_eq!(
            Ok((
                ("doc>", ParserState::new(None, None)),
                "<".to_string()
            )),
            parse_doc((testdoc, teststate))
        )
    }
    #[test]
    fn parser_anytag_test2() {
        let testdoc = "<=>";
        let teststate: ParserState<Nullo> = ParserState::new(None, None);
        let parse_doc = anytag(vec![">", ">=", "<=", "<"]);
        assert_eq!(
            Ok(((">", ParserState::new(None, None)), "<=".to_string())),
            parse_doc((testdoc, teststate))
        )
    }
}
