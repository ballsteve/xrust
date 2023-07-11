use crate::parser::{ParseError, ParseInput, ParseResult};

pub(crate) fn take_one() -> impl Fn(ParseInput) -> ParseResult<char> {
    move |(input, state)| {
        let c = input.chars().next();
        match c {
            None => Err(ParseError::Combinator),
            Some(ind) => Ok(((&input[ind.len_utf8()..], state), ind)),
        }
    }
}

pub(crate) fn take_until(s: &'static str) -> impl Fn(ParseInput) -> ParseResult<String> {
    move |(input, state)| match input.find(s) {
        None => Err(ParseError::Combinator),
        Some(ind) => Ok(((&input[ind..], state), input[0..ind].to_string())),
    }
}

pub(crate) fn take_until_end() -> impl Fn(ParseInput) -> ParseResult<String> {
    move |(input, state)| Ok((("", state), input.to_string()))
}

pub(crate) fn take_while<F>(condition: F) -> impl Fn(ParseInput) -> ParseResult<String>
//TODO REPLACE WITH ORDINARY TAKE_WHILE
where
    F: Fn(char) -> bool,
{
    move |(input, state)| match input.find(|c| !condition(c)) {
        None => Err(ParseError::Combinator),
        Some(0) => Err(ParseError::Combinator),
        Some(pos) => Ok(((&input[pos..], state), input[0..pos].to_string())),
    }
}

pub(crate) fn take_while_m_n<F>(
    min: usize,
    max: usize,
    condition: F,
) -> impl Fn(ParseInput) -> ParseResult<String>
where
    F: Fn(char) -> bool,
{
    move |(input, state)| match input.find(|c| !condition(c)) {
        None => Err(ParseError::Combinator),
        Some(pos) => {
            if pos >= min {
                if pos > max {
                    Ok(((&input[max..], state), input[0..max].to_string()))
                } else {
                    Ok(((&input[pos..], state), input[0..pos].to_string()))
                }
            } else {
                Err(ParseError::Combinator)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::combinators::take::{take_until, take_while};
    use crate::parser::ParserState;

    #[test]
    fn parser_take_until_test1() {
        let testdoc = "<doc>";
        let teststate = ParserState::new(None, None);
        let parse_doc = take_until(">");
        assert_eq!(
            Ok(((">", ParserState::new(None, None)), "<doc".to_string())),
            parse_doc((testdoc, teststate))
        );
    }

    #[test]
    fn parser_take_until_test2() {
        let testdoc = "<doc>";
        let teststate = ParserState::new(None, None);
        let parse_doc = take_until("oc");
        assert_eq!(
            Ok((("oc>", ParserState::new(None, None)), "<d".to_string())),
            parse_doc((testdoc, teststate))
        );
    }

    #[test]
    fn parser_take_until_test3() {
        let testdoc = "<doc>";
        let teststate = ParserState::new(None, None);
        let parse_doc = take_until("doc");
        assert_eq!(
            Ok((("doc>", ParserState::new(None, None)), "<".to_string())),
            parse_doc((testdoc, teststate))
        );
    }

    #[test]
    fn parser_take_while_test1() {
        let testdoc = "AAAAABCCCCC";
        let teststate = ParserState::new(None, None);
        let parse_doc = take_while(|c| c != 'B');
        assert_eq!(
            Ok((
                ("BCCCCC", ParserState::new(None, None)),
                "AAAAA".to_string()
            )),
            parse_doc((testdoc, teststate))
        );
    }

    #[test]
    fn parser_take_while_test2() {
        let testdoc = "ABCDEFGH";
        let teststate = ParserState::new(None, None);
        let parse_doc = take_while(|c| c != 'B' && c != 'C');
        assert_eq!(
            Ok((("BCDEFGH", ParserState::new(None, None)), "A".to_string())),
            parse_doc((testdoc, teststate))
        );
    }

    #[test]
    fn parser_take_while_test3() {
        let testdoc = "v1\"></doc>";
        let teststate = ParserState::new(None, None);
        let parse_doc = take_while(|c| c != '&' && c != '"');
        assert_eq!(
            Ok((
                ("\"></doc>", ParserState::new(None, None)),
                "v1".to_string()
            )),
            parse_doc((testdoc, teststate))
        );
    }
}
