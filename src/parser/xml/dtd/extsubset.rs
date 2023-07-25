use crate::parser::{ParseError, ParseInput, ParseResult};
use crate::parser::combinators::alt::{alt10, alt11, alt8};
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_until;
use crate::parser::combinators::tuple::{tuple2, tuple7};
use crate::parser::combinators::whitespace::{whitespace0, whitespace1};
use crate::parser::xml::dtd::attlistdecl::attlistdecl;
use crate::parser::xml::dtd::conditionals::{ignoresect, includesect};
use crate::parser::xml::dtd::elementdecl::elementdecl;
use crate::parser::xml::dtd::gedecl::gedecl;
use crate::parser::xml::dtd::notation::ndatadecl;
use crate::parser::xml::dtd::pedecl::pedecl;
use crate::parser::xml::dtd::pereference::pereference;
use crate::parser::xml::dtd::textdecl::textdecl;
use crate::parser::xml::misc::{comment, processing_instruction};
use crate::parser::xml::xmldecl::xmldecl;

pub(crate) fn extsubset() -> impl Fn(ParseInput) -> ParseResult<()> {
    move |(input, state)| {
        if state.standalone == true {
            Ok(((input, state),()))
        } else {
            match tuple2(
                opt(textdecl()),
                extsubsetdecl()
            )((input, state)){
                Ok(((input2, state2), (_, _))) => {
                    if !input2.is_empty(){
                        println!("{:?}",input2);
                        println!("NWF");
                        Err(ParseError::NotWellFormed)
                    } else {
                        Ok(((input2, state2), ()))
                    }
                }
                Err(e) => {Err(e)}
            }
        }
    }
}


pub(crate) fn extsubsetdecl() -> impl Fn(ParseInput) -> ParseResult<Vec<()>> {
    many0(alt11(
        includesect(),
        ignoresect(),
        elementdecl(),
        attlistdecl(),
        pedecl(),
        gedecl(),
        ndatadecl(),
        whitespace1(),
        map(comment(), |_| ()),
        map(processing_instruction(), |_| ()),
        pereference()
    ))
}

