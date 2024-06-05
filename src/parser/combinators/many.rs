use crate::item::Node;
use crate::parser::{ParseError, ParseInput};

pub fn many0<P, R, N: Node>(
    parser: P,
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, Vec<R>), ParseError>
where
    P: Fn(ParseInput<N>) -> Result<(ParseInput<N>, R), ParseError>,
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

pub fn many1<P, R, N: Node>(
    parser: P,
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, Vec<R>), ParseError>
where
    P: Fn(ParseInput<N>) -> Result<(ParseInput<N>, R), ParseError>,
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
