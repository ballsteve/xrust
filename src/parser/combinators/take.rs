use crate::item::Node;
use crate::parser::{ParseError, ParseInput, StaticState};
use qualname::{NamespacePrefix, NamespaceUri};

pub(crate) fn take_one<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, char), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), _ss| {
        let c = input.chars().next();
        match c {
            None => Err(ParseError::Combinator(String::from("take_one: no input"))),
            Some(ind) => Ok(((&input[ind.len_utf8()..], state), ind)),
        }
    }
}

pub(crate) fn take_until<'a, N: Node, L>(
    s: &'static str,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), _ss| match input.find(s) {
        None => Err(ParseError::Combinator(String::from("take_until: no input"))),
        Some(ind) => Ok(((&input[ind..], state), input[0..ind].to_string())),
    }
}

pub(crate) fn take_until_either_or<'a, N: Node, L>(
    s1: &'static str,
    s2: &'static str,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), _ss| {
        let r1 = input.find(s1);
        let r2 = input.find(s2);
        match (r1, r2) {
            (Some(i1), Some(i2)) => Ok((
                (&input[i1.min(i2)..], state),
                input[0..i1.min(i2)].to_string(),
            )),
            (Some(i1), None) => Ok(((&input[i1..], state), input[0..i1].to_string())),
            (None, Some(i2)) => Ok(((&input[i2..], state), input[0..i2].to_string())),
            (None, None) => Err(ParseError::Combinator(String::from(
                "take_until_either_or: no input",
            ))),
        }
    }
}

pub(crate) fn take_until_either_or_min1<'a, N: Node, L>(
    s1: &'static str,
    s2: &'static str,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), _ss| {
        let r1 = input.find(s1);
        let r2 = input.find(s2);
        match (r1, r2) {
            (Some(i1), Some(i2)) => {
                if i1 == 0 || i2 == 0 {
                    Err(ParseError::Combinator(String::from(
                        "take_until_either_or_min: no input",
                    )))
                } else {
                    Ok((
                        (&input[i1.min(i2)..], state),
                        input[0..i1.min(i2)].to_string(),
                    ))
                }
            }
            (Some(i1), None) => {
                if i1 == 0 {
                    Err(ParseError::Combinator(String::from(
                        "take_until_either_or_min: unable to find second term",
                    )))
                } else {
                    Ok(((&input[i1..], state), input[0..i1].to_string()))
                }
            }
            (None, Some(i2)) => {
                if i2 == 0 {
                    Err(ParseError::Combinator(String::from(
                        "take_until_either_or_min: unable to find first term",
                    )))
                } else {
                    Ok(((&input[i2..], state), input[0..i2].to_string()))
                }
            }
            (None, None) => Err(ParseError::Combinator(String::from(
                "take_until_either_or_min: no input",
            ))),
        }
    }
}

pub(crate) fn take_until_end<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), _ss| Ok((("", state), input.to_string()))
}

/// Take characters from the input while the condition is true.
/// If there is no character that fails the condition,
/// then if the input is empty returns ParseError::Combinator (i.e. no match),
/// otherwise returns the input.
pub(crate) fn take_while<'a, F, N: Node, L>(
    condition: F,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
//TODO REPLACE WITH ORDINARY TAKE_WHILE
where
    F: Fn(char) -> bool,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), _ss| match input.find(|c| !condition(c)) {
        None => {
            if input.is_empty() {
                Err(ParseError::Combinator(String::from("take_while: no input")))
            } else {
                Ok((("", state), input.to_string()))
            }
        }
        Some(0) => Err(ParseError::Combinator(String::from(
            "take_while: term not found",
        ))),
        Some(pos) => Ok(((&input[pos..], state), input[0..pos].to_string())),
    }
}

/// Take characters from the input while the condition is true.
/// If there is no character that fails the condition,
/// then if the input is empty returns ParseError::Combinator (i.e. no match),
/// otherwise returns the input.
#[allow(dead_code)]
pub(crate) fn take_while_m_n<'a, F, N: Node, L>(
    min: usize,
    max: usize,
    condition: F,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    F: Fn(char) -> bool,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), _ss| match input.find(|c| !condition(c)) {
        None => {
            if input.is_empty() {
                Err(ParseError::Combinator(String::from(
                    "take_while_m_n: no input",
                )))
            } else {
                Ok(((&input[max..], state), input[0..max].to_string()))
            }
        }
        Some(pos) => {
            if pos >= min {
                if pos > max {
                    Ok(((&input[max..], state), input[0..max].to_string()))
                } else {
                    Ok(((&input[pos..], state), input[0..pos].to_string()))
                }
            } else {
                Err(ParseError::Combinator(String::from(
                    "take_while_m_n: term not found",
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::combinators::take::{
        take_until, take_until_either_or, take_while, take_while_m_n,
    };
    use crate::parser::{ParseError, ParserState, StaticStateBuilder};
    use crate::trees::nullo::Nullo;
    use qualname::NamespaceUri;

    #[test]
    fn parser_take_until_test1() {
        let testdoc = "<doc>";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = take_until(">");
        assert_eq!(
            Ok(((">", ParserState::new()), "<doc".to_string())),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }
    #[test]
    fn parser_take_until_test2() {
        let testdoc = "<document";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = take_until(">");
        assert_eq!(
            Err(ParseError::Combinator(String::from("test"))),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }

    #[test]
    fn parser_take_until_test3() {
        let testdoc = "<doc>";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = take_until("oc");
        assert_eq!(
            Ok((("oc>", ParserState::new()), "<d".to_string())),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }

    #[test]
    fn parser_take_until_test4() {
        let testdoc = "<doc>";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = take_until("doc");
        assert_eq!(
            Ok((("doc>", ParserState::new()), "<".to_string())),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }

    #[test]
    fn parser_take_while_test1() {
        let testdoc = "AAAAABCCCCC";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = take_while(|c| c != 'B');
        assert_eq!(
            Ok((("BCCCCC", ParserState::new()), "AAAAA".to_string())),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }

    #[test]
    fn parser_take_while_test2() {
        let testdoc = "ABCDEFGH";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = take_while(|c| c != 'B' && c != 'C');
        assert_eq!(
            Ok((("BCDEFGH", ParserState::new()), "A".to_string())),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }

    #[test]
    fn parser_take_while_test3() {
        let testdoc = "v1\"></doc>";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = take_while(|c| c != '&' && c != '"');
        assert_eq!(
            Ok((("\"></doc>", ParserState::new()), "v1".to_string())),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }

    #[test]
    fn parser_take_until_either_or1() {
        let testdoc = "ABCDEFGH";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = take_until_either_or("DE", "FG");
        assert_eq!(
            Ok((("DEFGH", ParserState::new()), "ABC".to_string())),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }

    #[test]
    fn parser_take_until_either_or2() {
        let testdoc = "ABCDEFGH";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = take_until_either_or("AA", "BB");
        assert_eq!(
            Err(ParseError::Combinator(String::from("test"))),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }

    #[test]
    fn parser_take_until_either_or3() {
        let testdoc = "ABCDEFGH";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = take_until_either_or("EF", "FF");
        assert_eq!(
            Ok((("EFGH", ParserState::new()), "ABCD".to_string())),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }

    #[test]
    fn parser_take_until_either_or4() {
        let testdoc = "ABCDEFGH";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = take_until_either_or("ABD", "GH");
        assert_eq!(
            Ok((("GH", ParserState::new()), "ABCDEF".to_string())),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }

    #[test]
    fn parser_take_until_either_or5() {
        let testdoc = "ABCDEFGH";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = take_until_either_or("BC", "BC");
        assert_eq!(
            Ok((("BCDEFGH", ParserState::new()), "A".to_string())),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }

    #[test]
    fn parser_take_while_m_n_1() {
        let testdoc = "ABCDEFGH";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = take_while_m_n(2, 4, |c| c.is_uppercase());
        assert_eq!(
            Ok((("EFGH", ParserState::new()), "ABCD".to_string())),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }
    #[test]
    fn parser_take_while_m_n_2() {
        let testdoc = "ABCDEFGH";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = take_while_m_n(2, 4, |c| c.is_lowercase());
        assert_eq!(
            Err(ParseError::Combinator(String::from("test"))),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }
}
