use crate::parser::{ParseInput, ParseResult};

pub(crate) fn many0<P, R>(parser: P) -> impl Fn(ParseInput) -> ParseResult<Vec<R>>
where
    P: Fn(ParseInput) -> ParseResult<R>,
{
    move |(mut input, mut index)| {
        let mut result = Vec::new();

        while let Ok((input2, next_index, next_item)) = parser((input.clone(), index)) {
            index = next_index;
            result.push(next_item);
            input = input2;
        }

        Ok((input, index, result))
    }
}

pub(crate) fn many1<P, R>(parser: P) -> impl Fn(ParseInput) -> ParseResult<Vec<R>>
where
    P: Fn(ParseInput) -> ParseResult<R>,
{
    move |(mut input, mut index)| {
        let mut result = Vec::new();

        match parser((input, index)) {
            Err(err) => Err(err),
            Ok((input1, index1, result1)) => {
                input = input1;
                index = index1;
                result.push(result1);
                while let Ok((input2, index2, next_item)) = parser((input.clone(), index)) {
                    input = input2;
                    index = index2;
                    result.push(next_item);
                }
                Ok((input, index, result))
            }
        }
    }
}
