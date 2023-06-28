use crate::parser::{ParseError, ParseInput, ParseResult};

pub(crate) fn take_until(s: &'static str) -> impl Fn(ParseInput) -> ParseResult<String> {
    move |input| {
        let mut chars = s.chars();
        match chars.clone().count() {
            1 => take_until1(chars.next().unwrap())(input),
            2 => take_until2(chars.next().unwrap(), chars.next().unwrap())(input),
            3 => take_until3(
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
            )(input),
            _ => Err(ParseError::Notimplemented),
        }
    }
}

/// Take all characters up to the search character.
/// If the search character is not found, then if there is no more input return ParseError::Combinator (i.e. no match).
/// Otherwise return the remaining input.
fn take_until1(ch: char) -> impl Fn(ParseInput) -> ParseResult<String> {
    move |mut input| match input.clone().position(|c| c == ch) {
        None => match input.clone().peekable().peek() {
            Some(_) => {
                let res = (&mut input).collect::<String>();
                Ok((input, res))
            }
            None => Err(ParseError::Combinator),
        },
        Some(pos) => {
            let res = (&mut input).take(pos).collect::<String>();
            Ok((input, res))
        }
    }
}

fn take_until2(ch1: char, ch2: char) -> impl Fn(ParseInput) -> ParseResult<String> {
    move |mut input| {
        let mut i = input.clone().peekable();
        let mut search = 0;
        loop {
            let nextchar = i.next();
            search += 1;
            match nextchar {
                None => return Err(ParseError::Combinator),
                Some(c) => {
                    if c == ch1 {
                        match i.peek() {
                            None => return Err(ParseError::Combinator),
                            Some(p) => {
                                if p == &ch2 {
                                    //search += 1;
                                    break; //search
                                }
                            }
                        }
                    }
                }
            }
        }
        let res = (&mut input).take(search - 1).collect::<String>();
        Ok((input, res))
    }
}

fn take_until3(ch1: char, ch2: char, ch3: char) -> impl Fn(ParseInput) -> ParseResult<String> {
    move |mut input| {
        let mut i = input.clone().peekable();
        let mut search = 0;
        match i.next() {
            None => return Err(ParseError::Combinator),
            Some(c) => {
                let mut prevchar = c;
                search += 1;
                loop {
                    let nextchar = i.next();
                    search += 1;
                    match nextchar {
                        None => return Err(ParseError::Combinator),
                        Some(c) => {
                            if (prevchar == ch1) && (c == ch2) {
                                match i.peek() {
                                    None => return Err(ParseError::Combinator),
                                    Some(p) => {
                                        if p == &ch3 {
                                            //search += 2;
                                            break;
                                        }
                                    }
                                }
                            } else {
                                prevchar = c;
                            }
                        }
                    }
                }
            }
        }
        let res = (&mut input).take(search - 2).collect();
        Ok((input, res))
    }
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
    move |mut input| match input.clone().position(|c| !condition(c)) {
        None => match input.clone().peekable().peek() {
            Some(_) => {
                let res = (&mut input).collect::<String>();
                Ok((input, res))
            }
            None => Err(ParseError::Combinator),
        },
        Some(0) => Err(ParseError::Combinator),
        Some(pos) => {
            let res = (&mut input).take(pos).collect::<String>();
            Ok((input, res))
        }
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
    move |mut input| match input.clone().position(|c| !condition(c)) {
        None => match input.clone().peekable().peek() {
            Some(_) => {
                let res = (&mut input).take(max).collect::<String>();
                Ok((input, res))
            }
            None => Err(ParseError::Combinator),
        },
        Some(pos) => {
            if pos >= min {
                if pos > max {
                    let res = (&mut input).take(max).collect::<String>();
                    Ok((input, res))
                } else {
                    let res = (&mut input).take(pos).collect::<String>();
                    Ok((input, res))
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
    use crate::parser::ParseInput;

    #[test]
    fn parser_take_until1_test1() {
        let testdoc = ParseInput::new("<doc>");
        let parse_doc = take_until(">");
        assert_eq!(
            Ok((ParseInput::new("<doc>"), "<doc".to_string())),
            parse_doc(testdoc)
        );
    }
    #[test]
    fn parser_take_until1_test2() {
        let testdoc = ParseInput::new("<document");
        let parse_doc = take_until(">");
        assert_eq!(
            Ok((ParseInput::new("<document"), "<document".to_string())),
            parse_doc(testdoc)
        );
    }

    #[test]
    fn parser_take_until2_test1() {
        let testdoc = ParseInput::new("<doc>");
        let parse_doc = take_until("oc");
        assert_eq!(
            Ok((ParseInput::new("<doc>"), "<d".to_string())),
            parse_doc(testdoc)
        );
    }

    #[test]
    fn parser_take_until3_test1() {
        let testdoc = ParseInput::new("<doc>");
        let parse_doc = take_until("doc");
        assert_eq!(
            Ok((ParseInput::new("<doc>"), "<".to_string())),
            parse_doc(testdoc)
        );
    }

    #[test]
    fn parser_take_while_test1() {
        let testdoc = ParseInput::new("<doc>");
        let parse_doc = take_while(|c| c != '>');
        assert_eq!(
            Ok((ParseInput::new("<doc>"), "<doc".to_string())),
            parse_doc(testdoc)
        );
    }
    #[test]
    fn parser_take_while_test2() {
        let testdoc = ParseInput::new("<document");
        let parse_doc = take_while(|c| c != '>');
        assert_eq!(
            Ok((ParseInput::new("<document"), "<document".to_string())),
            parse_doc(testdoc)
        );
    }
}
