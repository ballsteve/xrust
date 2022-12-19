use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_until;
use crate::parser::{ParseInput, ParseResult};
use crate::intmuttree::DTDDecl;

pub(crate) fn genentityexpander() -> impl Fn(ParseInput) -> ParseResult<String> + 'static {
    move |(input, index)| {
        let e = delimited(tag("&"), take_until(";"), tag(";"))((input, index));

        match e {
            Err(usize) => Err(usize),
            Ok((mut input1, _, entitykey)) => {
                match input1.dtd.generalentities.get(&entitykey as &str) {
                    Some(DTDDecl::GeneralEntity(_, v)) => {
                        if input1.currententitydepth >= input1.maxentitydepth {
                            //attempting to exceed expansion depth
                            Err(index)
                        } else {
                            for ch in v.chars().rev() {
                                input1.entityfeed.push(ch);
                            }
                            input1.currententitydepth += 1;
                            Ok((input1, index, "".to_string()))
                        }
                    }
                    _ => Err(index),
                }
            }
        }
    }
}

pub(crate) fn paramentityexpander() -> impl Fn(ParseInput) -> ParseResult<String> + 'static {
    move |(input, index)| {
        let e = delimited(tag("%"), take_until(";"), tag(";"))((input, index));

        match e {
            Err(usize) => Err(usize),
            Ok((mut input1, _, entitykey)) => {
                match input1.dtd.paramentities.get(&entitykey as &str) {
                    Some(DTDDecl::ParamEntity(_, v)) => {
                        if input1.currententitydepth >= input1.maxentitydepth {
                            //attempting to exceed expansion depth
                            Err(index)
                        } else {
                            for ch in v.chars().rev() {
                                input1.entityfeed.push(ch);
                            }
                            input1.currententitydepth += 1;
                            Ok((input1, index, "".to_string()))
                        }
                    }
                    _ => Err(index),
                }
            }
        }
    }
}
