use crate::item::Node;
use crate::parser::combinators::alt::alt10;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tuple::tuple3;
use crate::parser::combinators::whitespace::whitespace1;
use crate::parser::xml::dtd::attlistdecl::attlistdecl;
use crate::parser::xml::dtd::conditionals::conditionalsect;
use crate::parser::xml::dtd::elementdecl::elementdecl;
use crate::parser::xml::dtd::gedecl::gedecl;
use crate::parser::xml::dtd::notation::notation_decl;
use crate::parser::xml::dtd::pedecl::pedecl;
use crate::parser::xml::dtd::pereference::pereference;
use crate::parser::xml::dtd::textdecl::textdecl;
use crate::parser::xml::misc::{comment, processing_instruction};
use crate::parser::xml::utf8bom;
use crate::parser::{ParseError, ParseInput, StaticState};
use qualname::{NamespacePrefix, NamespaceUri};

pub(crate) fn extsubset<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, ()), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, mut state), ss| {
        if state.standalone {
            Ok(((input, state), ()))
        } else {
            state.currentlyexternal = true;
            match tuple3(opt(utf8bom()), opt(textdecl()), extsubsetdecl())((input, state), ss) {
                Ok(((input2, mut state2), (_, td, _))) => {
                    if !input2.is_empty() {
                        Err(ParseError::NotWellFormed(format!(
                            "unexpected text in external subset \"{}\"",
                            input2.to_string()
                        )))
                    } else if td.is_some_and(|xd| xd.encoding.is_none()) {
                        Err(ParseError::ExtDTDLoadError)
                    } else {
                        state2.currentlyexternal = false;
                        Ok(((input2, state2), ()))
                    }
                }
                Err(e) => Err(e),
            }
        }
    }
}

pub(crate) fn extsubsetdecl<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, Vec<()>), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    many0(alt10(
        conditionalsect(),
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
