use crate::parser::{ParseInput, ParseResult};

pub fn many0<P, R>(parser: P) -> impl Fn(ParseInput) -> ParseResult<Vec<R>>
where
    P: Fn(ParseInput) -> ParseResult<R>,
{
    //TODO ERROR IF ANY ERROR OTHER THAN COMBINATOR RETURNED.
    move |mut input| {
        let mut result = Vec::new();

        while let Ok((input2, next_item)) = parser(input.clone()) {
            result.push(next_item);
            input = input2;
        }

        Ok((input, result))
    }
}

pub fn many1<P, R>(parser: P) -> impl Fn(ParseInput) -> ParseResult<Vec<R>>
where
    P: Fn(ParseInput) -> ParseResult<R>,
{
    //TODO ERROR IF ANY ERROR OTHER THAN COMBINATOR RETURNED.
    move |mut input| {
        let mut result = Vec::new();

        match parser(input) {
            Err(err) => Err(err),
            Ok((input1, result1)) => {
                input = input1;
                result.push(result1);
                while let Ok((input2, next_item)) = parser(input.clone()) {
                    input = input2;
                    result.push(next_item);
                }
                Ok((input, result))
            }
        }
    }
}
