//! Support for predicates

use crate::item::Node;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::tuple3;
use crate::parser::combinators::whitespace::xpwhitespace;
use crate::parser::{ParseError, ParseInput};
use crate::qname::Interner;
use crate::transform::Transform;
//use crate::parser::combinators::debug::inspect;
use crate::parser::xpath::expr_wrapper;

// PredicateList ::= Predicate*
pub(crate) fn predicate_list<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(map(many0(predicate::<I, N>()), |v| Transform::Compose(v)))
}

// Predicate ::= "[" expr "]"
fn predicate<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(map(
        tuple3(
            map(tuple3(xpwhitespace(), tag("["), xpwhitespace()), |_| ()),
            expr_wrapper::<I, N>(true),
            map(tuple3(xpwhitespace(), tag("]"), xpwhitespace()), |_| ()),
        ),
        |(_, e, _)| Transform::Filter(Box::new(e)),
    ))
}
