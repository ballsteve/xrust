use crate::parser::{ParseInput, ParseResult};

pub(crate) fn take_until(s: &'static str) -> impl Fn(ParseInput) -> ParseResult<String> {
    move |(input, index)| {
        let mut chars = s.chars();
        match chars.clone().count() {
            1 => take_until1(chars.next().unwrap())((input, index)),
            2 => take_until2(chars.next().unwrap(), chars.next().unwrap())((input, index)),
            3 => take_until3(
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
            )((input, index)),
            _ => Err(index),
        }
    }
}

fn take_until1(ch: char) -> impl Fn(ParseInput) -> ParseResult<String> {
    move |(mut input, index)| match input.clone().position(|c| c == ch) {
        None => Err(index),
        Some(pos) => {
            let res = (&mut input).take(pos).collect::<String>();
            Ok((input, index + pos, res))
        }
    }
}

fn take_until2(ch1: char, ch2: char) -> impl Fn(ParseInput) -> ParseResult<String> {
    move |(mut input, index)| {
        let mut i = input.clone().peekable();
        let mut search = 0;
        loop {
            let nextchar = i.next();
            search += 1;
            match nextchar {
                None => return Err(index),
                Some(c) => {
                    if c == ch1 {
                        match i.peek() {
                            None => return Err(index),
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
        Ok((input, index + search - 1, res))
    }
}

fn take_until3(ch1: char, ch2: char, ch3: char) -> impl Fn(ParseInput) -> ParseResult<String> {
    move |(mut input, index)| {
        let mut i = input.clone().peekable();
        let mut search = 0;
        match i.next() {
            None => return Err(index),
            Some(c) => {
                let mut prevchar = c;
                search += 1;
                loop {
                    let nextchar = i.next();
                    search += 1;
                    match nextchar {
                        None => return Err(index),
                        Some(c) => {
                            if (prevchar == ch1) && (c == ch2) {
                                match i.peek() {
                                    None => return Err(index),
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
        Ok((input, index + search - 2, res))
    }
}

pub(crate) fn take_while<F>(condition: F) -> impl Fn(ParseInput) -> ParseResult<String>
//TODO REPLACE WITH ORDINARY TAKE_WHILE
where
    F: Fn(char) -> bool,
{
    move |(mut input, index)| match input.clone().position(|c| !condition(c)) {
        None => Err(index),
        Some(0) => Err(index),
        Some(pos) => {
            let res = (&mut input).take(pos).collect::<String>();
            Ok((input, index + pos, res))
        }
    }
    /*
    move |(mut input, index)| {
        let mut res = (&mut input).take_while(|c| condition(*c)).collect::<String>();
        if !res.is_empty() {
            Ok((input, index + (&mut res).chars().count(), res))
        } else{
            Err(index)
        }
    }

     */
}

pub(crate) fn take_while_m_n<F>(
    min: usize,
    max: usize,
    condition: F,
) -> impl Fn(ParseInput) -> ParseResult<String>
where
    F: Fn(char) -> bool,
{
    move |(mut input, index)| match input.clone().position(|c| !condition(c)) {
        None => Err(index),
        Some(pos) => {
            if pos >= min {
                if pos > max {
                    let res = (&mut input).take(max).collect::<String>();
                    Ok((input, index + max, res))
                } else {
                    let res = (&mut input).take(pos).collect::<String>();
                    Ok((input, index + pos, res))
                }
            } else {
                Err(index)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::combinators::take::take_until;
    use crate::parser::Parserinput;

    #[test]
    fn parser_take_until1_test1() {
        let testdoc = Parserinput::new("<doc>");
        let parse_doc = take_until(">");
        assert_eq!(
            Ok((Parserinput::new("<doc>"), 4, "<doc".to_string())),
            parse_doc((testdoc, 0))
        );
    }

    #[test]
    fn parser_take_until2_test1() {
        let testdoc = Parserinput::new("<doc>");
        let parse_doc = take_until("oc");
        assert_eq!(
            Ok((Parserinput::new("<doc>"), 2, "<d".to_string())),
            parse_doc((testdoc, 0))
        );
    }

    #[test]
    fn parser_take_until3_test1() {
        let testdoc = Parserinput::new("<doc>");
        let parse_doc = take_until("doc");
        assert_eq!(
            Ok((Parserinput::new("<doc>"), 1, "<".to_string())),
            parse_doc((testdoc, 0))
        );
    }
}
