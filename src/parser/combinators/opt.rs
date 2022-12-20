use crate::parser::{ParseInput, ParseResult};

pub(crate) fn opt<P1, R1>(parser1: P1) -> impl Fn(ParseInput) -> ParseResult<Option<R1>>
where
    P1: Fn(ParseInput) -> ParseResult<R1>,
{
    move |(input, index)| match parser1((input.clone(), index)) {
        Ok((input1, index1, result1)) => Ok((input1, index1, Some(result1))),
        Err(_) => Ok((input, index, None)),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::combinators::opt::opt;
    use crate::parser::combinators::tag::tag;
    use crate::parser::Parserinput;

    #[test]
    fn parser_opt_test1() {
        let testdoc = Parserinput::new("<doc>");
        let parse_doc = opt(tag("<"));
        assert_eq!(
            Ok((Parserinput::new("<doc>"), 1, Some(()))),
            parse_doc((testdoc, 0))
        );
    }

    #[test]
    fn parser_opt_test2() {
        let testdoc = Parserinput::new("<doc>");
        let parse_doc = opt(tag(">"));
        assert_eq!(
            Ok((Parserinput::new("<doc>"), 0, None)),
            parse_doc((testdoc, 0))
        );
    }
}
