use crate::parser::combinators::alt::alt9;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::whitespace::whitespace1;
use crate::parser::xml::dtd::attlistdecl::attlistdecl;
use crate::parser::xml::dtd::elementdecl::elementdecl;
use crate::parser::xml::dtd::gedecl::gedecl;
use crate::parser::xml::dtd::notation::ndatadecl;
use crate::parser::xml::dtd::pedecl::pedecl;
use crate::parser::xml::dtd::pereference::pereference;
use crate::parser::xml::misc::comment;
use crate::parser::xml::misc::processing_instruction;
use crate::parser::{ParseError, ParseInput};
use crate::item::Node;

pub(crate) fn intsubset<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, Vec<()>), ParseError> {
    many0(alt9(
        elementdecl(),
        attlistdecl(),
        pedecl(),
        gedecl(),
        ndatadecl(),
        whitespace1(),
        map(comment(), |_| ()),
        map(processing_instruction(), |_| ()),
        pereference(),
    ))
}
