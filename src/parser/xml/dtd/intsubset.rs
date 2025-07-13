use crate::item::Node;
use crate::parser::combinators::alt::alt9;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::whitespace::whitespace1;
use crate::parser::xml::dtd::attlistdecl::attlistdecl;
use crate::parser::xml::dtd::elementdecl::elementdecl;
use crate::parser::xml::dtd::gedecl::gedecl;
use crate::parser::xml::dtd::notation::notation_decl;
use crate::parser::xml::dtd::pedecl::pedecl;
use crate::parser::xml::dtd::pereference::pereference;
use crate::parser::xml::misc::comment;
use crate::parser::xml::misc::processing_instruction;
use crate::parser::{ParseError, ParseInput};
use crate::qname::Interner;

pub(crate) fn intsubset<'a, 'i, I: Interner + 'i, N: Node>(
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Vec<()>), ParseError> {
    many0(alt9(
        elementdecl(),
        attlistdecl(),
        pedecl(),
        gedecl(),
        notation_decl(),
        whitespace1(),
        map(comment(), |_| ()),
        map(processing_instruction(), |_| ()),
        pereference(),
    ))
}
