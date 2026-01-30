use crate::item::Node;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::tuple7;
use crate::parser::combinators::whitespace::{whitespace0, whitespace1};
use crate::parser::xml::dtd::misc::contentspec;
use crate::parser::xml::qname::qualname_to_parts;
use crate::parser::{ParseError, ParseInput, StaticState};
use qualname::{NamespacePrefix, NamespaceUri};

//elementdecl	   ::=   	'<!ELEMENT' S Name S contentspec S? '>'
pub(crate) fn elementdecl<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, ()), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |input, ss| match tuple7(
        tag("<!ELEMENT"),
        whitespace1(),
        qualname_to_parts(),
        whitespace1(),
        contentspec(), //contentspec - TODO Build out.
        whitespace0(),
        tag(">"),
    )(input, ss)
    {
        Ok(((input2, mut state2), (_, _, n, _, s, _, _))) => {
            state2.dtd.elements.insert(n, s);
            Ok(((input2, state2), ()))
        }
        Err(err) => Err(err),
    }
}
