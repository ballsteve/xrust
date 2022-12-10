use crate::parser::{ParseInput, ParseResult};

pub(crate) fn tag(expected: &str) -> impl Fn(ParseInput) -> ParseResult<()> + '_ {
    move |(mut input, index)| {
        let tagchars = expected.chars();
        let mut cnt = 0;
        for tchar in tagchars {
            match input.next() {
                None => return Err(index),
                Some(char) => {
                    if char == tchar {
                        cnt += 1
                    } else {
                        return Err(index);
                    }
                }
            }
        }
        Ok((input, index + cnt, ()))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::combinators::tag::tag;
    use crate::parser::Parserinput;

    #[test]
    fn parser_tag_test1() {
        let testdoc = Parserinput::new("<doc>");
        let parse_doc = tag("<");
        assert_eq!(
            Ok((Parserinput::new("<doc>"), 1, ())),
            parse_doc((testdoc, 0))
        );
    }

    #[test]
    fn parser_tag_test2() {
        let testdoc = Parserinput::new("<doc>");
        let parse_doc = tag(">");
        assert_eq!(Err(0), parse_doc((testdoc, 0)));
    }
}
