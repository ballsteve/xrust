use crate::item::Node;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::tuple7;
use crate::parser::combinators::whitespace::{whitespace0, whitespace1};
use crate::parser::xml::dtd::misc::contentspec;
use crate::parser::xml::qname::qualname;
use crate::parser::{ParseError, ParseInput};
use crate::qname::Interner;

//elementdecl	   ::=   	'<!ELEMENT' S Name S contentspec S? '>'
pub(crate) fn elementdecl<'a, 'i, I: Interner, N: Node>(
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, ()), ParseError> {
    move |input| match tuple7(
        tag("<!ELEMENT"),
        whitespace1(),
        qualname(),
        whitespace1(),
        contentspec(), //contentspec - TODO Build out.
        whitespace0(),
        tag(">"),
    )(input)
    {
        Ok(((input2, mut state2), (_, _, n, _, s, _, _))) => {
            state2.dtd.elements.insert(n, s);
            Ok(((input2, state2), ()))
        }
        Err(err) => Err(err),
    }
}
