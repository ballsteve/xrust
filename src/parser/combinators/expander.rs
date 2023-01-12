use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_until;
use crate::parser::{ParseInput, ParseError, ParseResult};
use crate::intmuttree::DTDDecl;

pub(crate) fn genentityexpander() -> impl Fn(ParseInput) -> ParseResult<String> + 'static {
    move |input| {
        let e = delimited(tag("&"), take_until(";"), tag(";"))(input);

        match e {
            Err(usize) => Err(usize),
            Ok((mut input1, entitykey)) => {
                match input1.dtd.generalentities.get(&entitykey as &str) {
                    Some(DTDDecl::GeneralEntity(_, v)) => {
                        if input1.currententitydepth >= input1.maxentitydepth {
                            //attempting to exceed expansion depth
                            Err(ParseError::EntityDepth {col:input1.currentcol, row:input1.currentrow})
                        } else {
                            for ch in v.chars().rev() {
                                input1.entityfeed.push(ch);
                            }
                            input1.currententitydepth += 1;
                            Ok((input1, "".to_string()))
                        }
                    },
                    None => Err(ParseError::MissingGenEntity{col:input1.currentcol, row:input1.currentrow}),
                    _ => Err(ParseError::Unknown{col:input1.currentcol, row:input1.currentrow}),
                }
            }
        }
    }
}

pub(crate) fn paramentityexpander() -> impl Fn(ParseInput) -> ParseResult<String> + 'static {
    move |input| {
        let e = delimited(tag("%"), take_until(";"), tag(";"))(input);

        match e {
            Err(err) => Err(err),
            Ok((mut input1, entitykey)) => {
                match input1.dtd.paramentities.get(&entitykey as &str) {
                    Some(DTDDecl::ParamEntity(_, v)) => {
                        if input1.currententitydepth >= input1.maxentitydepth {
                            //attempting to exceed expansion depth
                                Err(ParseError::EntityDepth{col:input1.currentcol, row:input1.currentrow})
                        } else {
                            for ch in v.chars().rev() {
                                input1.entityfeed.push(ch);
                            }
                            input1.currententitydepth += 1;
                            Ok((input1, "".to_string()))
                        }
                    },
                    None => Err(ParseError::MissingParamEntity{col:input1.currentcol, row:input1.currentrow}),
                    _ => Err(ParseError::Unknown{col:input1.currentcol, row:input1.currentrow}),
                }
            }
        }
    }
}
