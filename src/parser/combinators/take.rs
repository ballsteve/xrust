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

pub(crate) fn take_until_either_or(s1: &'static str, s2: &'static str) -> impl Fn(ParseInput) -> ParseResult<String> {
    move |(input, state)| {
        let r1 = input.find(s1);
        let r2 = input.find(s2);
        match (r1, r2) {
            (Some(i1), Some(i2)) => {
                Ok(((&input[i1.min(i2)..], state), input[0..i1.min(i2)].to_string()))
            },
            (Some(i1), None) =>{
                Ok(((&input[i1..], state), input[0..i1].to_string()))
            },
            (None, Some(i2)) =>{
                Ok(((&input[i2..], state), input[0..i2].to_string()))
            },
            (None, None) => {
                Err(ParseError::Combinator)
            }
        }
    }
}

pub(crate) fn take_until_either_or_min1(s1: &'static str, s2: &'static str) -> impl Fn(ParseInput) -> ParseResult<String> {
    move |(input, state)| {
        let r1 = input.find(s1);
        let r2 = input.find(s2);
        match (r1, r2) {
            (Some(i1), Some(i2)) => {
                if i1 == 0 || i2 == 0 {
                    Err(ParseError::Combinator)
                } else {
                    Ok(((&input[i1.min(i2)..], state), input[0..i1.min(i2)].to_string()))
                }
            },
            (Some(i1), None) =>{
                if i1 == 0 {
                    Err(ParseError::Combinator)
                } else {
                    Ok(((&input[i1..], state), input[0..i1].to_string()))
                }
            },
            (None, Some(i2)) =>{
                if i2 == 0 {
                    Err(ParseError::Combinator)
                } else {
                    Ok(((&input[i2..], state), input[0..i2].to_string()))
                }
            },
            (None, None) => {
                Err(ParseError::Combinator)
            }
        }
    }
}


pub(crate) fn take_until_end() -> impl Fn(ParseInput) -> ParseResult<String> {
    move |(input, state)| Ok((("", state), input.to_string()))
}

/// Take characters from the input while the condition is true.
/// If there is no character that fails the condition,
/// then if the input is empty returns ParseError::Combinator (i.e. no match),
/// otherwise returns the input.
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

/// Take characters from the input while the condition is true.
/// If there is no character that fails the condition,
/// then if the input is empty returns ParseError::Combinator (i.e. no match),
/// otherwise returns the input.
pub(crate) fn take_while_m_n<F>(
    min: usize,
    max: usize,
    condition: F,
) -> impl Fn(ParseInput) -> ParseResult<String>
    where
        F: Fn(char) -> bool,
{
    move |(input, state)| {
        match input.find(|c| !condition(c)) {
            None => {
                if input.is_empty() {
                    Err(ParseError::Combinator)
                } else {
                    Ok(((&input[max..], state), input[0..max].to_string()))
                }
            },
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
}

#[cfg(test)]
mod tests {
    use crate::parser::combinators::take::{take_until, take_until_either_or, take_while, take_while_m_n};
    use crate::parser::{ParseError, ParserState};

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
    fn parser_take_until1_test2() {
        let testdoc = "<document";
        let teststate = ParserState::new(None, None);
        let parse_doc = take_until(">");
        assert_eq!(
            Ok((("<document", ParserState::new(None, None)), "<document".to_string())),
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

    #[test]
    fn parser_take_until_either_or1() {
        let testdoc = "ABCDEFGH";
        let teststate = ParserState::new(None, None);
        let parse_doc = take_until_either_or("DE","FG");
        assert_eq!(
            Ok((
                ("DEFGH", ParserState::new(None, None)),
                "ABC".to_string()
            )),
            parse_doc((testdoc, teststate))
        );
    }

    #[test]
    fn parser_take_until_either_or2() {
        let testdoc = "ABCDEFGH";
        let teststate = ParserState::new(None, None);
        let parse_doc = take_until_either_or("AA","BB");
        assert_eq!(
            Err(ParseError::Combinator),
            parse_doc((testdoc, teststate))
        );
    }

    #[test]
    fn parser_take_until_either_or3() {
        let testdoc = "ABCDEFGH";
        let teststate = ParserState::new(None, None);
        let parse_doc = take_until_either_or("EF","FF");
        assert_eq!(
            Ok((
                ("EFGH", ParserState::new(None, None)),
                "ABCD".to_string()
            )),
            parse_doc((testdoc, teststate))
        );
    }

    #[test]
    fn parser_take_until_either_or4() {
        let testdoc = "ABCDEFGH";
        let teststate = ParserState::new(None, None);
        let parse_doc = take_until_either_or("ABD","GH");
        assert_eq!(
            Ok((
                ("GH", ParserState::new(None, None)),
                "ABCDEF".to_string()
            )),
            parse_doc((testdoc, teststate))
        );
    }

    #[test]
    fn parser_take_until_either_or5() {
        let testdoc = "ABCDEFGH";
        let teststate = ParserState::new(None, None);
        let parse_doc = take_until_either_or("BC","BC");
        assert_eq!(
            Ok((
                ("BCDEFGH", ParserState::new(None, None)),
                "A".to_string()
            )),
            parse_doc((testdoc, teststate))
        );
    }

    #[test]
    fn parser_take_while_m_n_1() {
        let testdoc = "ABCDEFGH";
        let teststate = ParserState::new(None, None);
        let parse_doc = take_while_m_n(2,4, |c| c.is_uppercase());
        assert_eq!(
            Ok((
                ("EFGH", ParserState::new(None, None)),
                "ABCD".to_string()
            )),
            parse_doc((testdoc, teststate))
        );
    }
    #[test]
    fn parser_take_while_m_n_2() {
        let testdoc = "ABCDEFGH";
        let teststate = ParserState::new(None, None);
        let parse_doc = take_while_m_n(2,4, |c| c.is_lowercase());
        assert_eq!(
            Err(ParseError::Combinator),
            parse_doc((testdoc, teststate))
        );
    }
}
