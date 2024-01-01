//! Support for predicates

use crate::item::Node;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::tuple3;
use crate::parser::combinators::whitespace::xpwhitespace;
use crate::parser::{ParseError, ParseInput};
use crate::transform::Transform;
//use crate::parser::combinators::debug::inspect;
use crate::parser::xpath::expr_wrapper;

// PredicateList ::= Predicate*
pub(crate) fn predicate_list<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a> {
    Box::new(map(many0(predicate::<N>()), |v| Transform::Compose(v)))
}

// Predicate ::= "[" expr "]"
fn predicate<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a> {
    Box::new(map(
        tuple3(
            map(tuple3(xpwhitespace(), tag("["), xpwhitespace()), |_| ()),
            expr_wrapper::<N>(true),
            map(tuple3(xpwhitespace(), tag("]"), xpwhitespace()), |_| ()),
        ),
        |(_, e, _)| Transform::Filter(Box::new(e)),
    ))
}
