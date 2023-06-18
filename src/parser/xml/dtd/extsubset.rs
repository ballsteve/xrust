use crate::parser::{ParseInput, ParseResult};
use crate::parser::combinators::alt::alt8;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tuple::tuple2;
use crate::parser::combinators::whitespace::whitespace1;
use crate::parser::xml::dtd::attlistdecl::attlistdecl;
use crate::parser::xml::dtd::elementdecl::elementdecl;
use crate::parser::xml::dtd::gedecl::gedecl;
use crate::parser::xml::dtd::notation::ndatadecl;
use crate::parser::xml::dtd::pedecl::pedecl;
use crate::parser::xml::misc::{comment, processing_instruction};
use crate::parser::xml::xmldecl::xmldecl;

pub(crate) fn extsubset() -> impl Fn(ParseInput) -> ParseResult<Vec<()>> {
    move |(input, state)| {

    }

    tuple2(
    opt(xmldecl()),
    extsubsetdecl()
    )(input, state);
    Ok(())
}


pub(crate) fn extsubsetdecl() -> impl Fn(ParseInput) -> ParseResult<Vec<()>> {
    many0(alt8(
        elementdecl(),
        attlistdecl(),
        pedecl(),
        gedecl(),
        ndatadecl(),
        whitespace1(),
        map(comment(), |_| ()),
        map(processing_instruction(), |_| ()),
    ))
}