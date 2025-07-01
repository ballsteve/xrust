//! Support for predicates

use crate::item::Node;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::tuple3;
use crate::parser::combinators::whitespace::xpwhitespace;
use crate::parser::{ParseError, ParseInput, StaticState};
use crate::transform::Transform;
//use crate::parser::combinators::debug::inspect;
use crate::parser::xpath::expr_wrapper;
use qualname::{NamespacePrefix, NamespaceUri};

// PredicateList ::= Predicate*
pub(crate) fn predicate_list<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(many0(predicate::<N, L>()), |v| Transform::Compose(v)))
}

// Predicate ::= "[" expr "]"
fn predicate<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(
        tuple3(
            map(tuple3(xpwhitespace(), tag("["), xpwhitespace()), |_| ()),
            expr_wrapper::<N, L>(true),
            map(tuple3(xpwhitespace(), tag("]"), xpwhitespace()), |_| ()),
        ),
        |(_, e, _)| Transform::Filter(Box::new(e)),
    ))
}
