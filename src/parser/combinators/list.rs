use crate::item::Node;
use crate::parser::{ParseError, ParseInput, StaticState};
use qualname::{NamespacePrefix, NamespaceUri};

pub fn separated_list0<'a, P1, P2, R1, N: Node, L>(
    sep: P1,
    f: P2,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, Vec<R1>), ParseError>
where
    P1: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, ()), ParseError>,
    P2: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, R1), ParseError>,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |mut input, ss| {
        let mut res = Vec::new();

        match f(input.clone(), ss) {
            Err(_e) => {
                return Ok((input, res));
            }
            Ok((i1, o)) => {
                res.push(o);
                input = i1;
            }
        }

        loop {
            match sep(input.clone(), ss) {
                Err(ParseError::Combinator) => {
                    return Ok((input, res));
                }
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    // infinite loop check: the parser must always consume
                    // SRB: not sure if this check is necessary with this parser, since input is an iterator
                    //		    if i1.input_len() == len {
                    //			return Err(ParseError::Combinator);
                    //		    }

                    match f(i1, ss) {
                        Err(ParseError::Combinator) => return Ok((input, res)),
                        Err(e) => return Err(e),
                        Ok((i2, o)) => {
                            res.push(o);
                            input = i2;
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn separated_list1<'a, P1, P2, R1, N: Node, L>(
    sep: P1,
    f: P2,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, Vec<R1>), ParseError>
where
    P1: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, ()), ParseError>,
    P2: Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, R1), ParseError>,
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |mut input, ss| {
        let mut res = Vec::new();

        match f(input.clone(), ss) {
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                res.push(o);
                input = i1;
            }
        }

        loop {
            match sep(input.clone(), ss) {
                Err(ParseError::Combinator) => {
                    return Ok((input, res));
                }
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    // infinite loop check: the parser must always consume
                    // SRB: not sure if this check is necessary with this parser, since input is an iterator
                    //		    if i1.input_len() == len {
                    //			return Err(ParseError::Combinator);
                    //		    }

                    match f(i1, ss) {
                        Err(ParseError::Combinator) => {
                            return Ok((input, res));
                        }
                        Err(e) => return Err(e),
                        Ok((i2, o)) => {
                            res.push(o);
                            input = i2;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::combinators::alt::alt4;
    use crate::parser::combinators::list::{separated_list0, separated_list1};
    use crate::parser::combinators::map::map;
    use crate::parser::combinators::tag::tag;
    use crate::parser::{ParseError, ParserState, StaticStateBuilder};
    use crate::trees::nullo::Nullo;
    use qualname::NamespaceUri;

    #[test]
    fn parser_separated_list0_test1() {
        let testdoc = "b";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = separated_list0(tag(","), map(tag("a"), |_| "a"));

        assert_eq!(
            Ok((("b", ParserState::new()), vec![])),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }

    #[test]
    fn parser_separated_list0_test2() {
        let testdoc = "a";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = separated_list0(tag(","), map(tag("a"), |_| "a"));

        assert_eq!(
            Ok((("", ParserState::new()), vec!["a"])),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }

    #[test]
    fn parser_separated_list0_test3() {
        let testdoc = "a,b,c,d";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = separated_list1(
            tag(","),
            alt4(
                map(tag("a"), |_| "1"),
                map(tag("b"), |_| "2"),
                map(tag("c"), |_| "3"),
                map(tag("d"), |_| "4"),
            ),
        );

        assert_eq!(
            Ok((("", ParserState::new()), vec!["1", "2", "3", "4"])),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }

    #[test]
    fn parser_separated_list1_test1() {
        let testdoc = "a";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = separated_list1(tag(","), map(tag("a"), |_| "a"));

        assert_eq!(
            Ok((("", ParserState::new()), vec!["a"])),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }

    #[test]
    fn parser_separated_list1_test2() {
        let testdoc = "a,b,c,d";
        let teststate: ParserState<Nullo> = ParserState::new();
        let mut static_state = StaticStateBuilder::new()
            .namespace(|_| {
                NamespaceUri::try_from("urn:xrust").map_err(|_| ParseError::MissingNameSpace)
            })
            .build();
        let parse_doc = separated_list1(
            tag(","),
            alt4(
                map(tag("a"), |_| "1"),
                map(tag("b"), |_| "2"),
                map(tag("c"), |_| "3"),
                map(tag("d"), |_| "4"),
            ),
        );

        assert_eq!(
            Ok((("", ParserState::new()), vec!["1", "2", "3", "4"])),
            parse_doc((testdoc, teststate), &mut static_state)
        );
    }
}
