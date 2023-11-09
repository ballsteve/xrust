//! Functions that manipulate the context.

use crate::item::Node;
use crate::transform::Transform;
use crate::parser::{ParseInput, ParseResult};
use crate::parser::combinators::map::map;
use crate::parser::combinators::tag::tag;

// ContextItemExpr ::= '.'
pub(crate) fn context_item<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Transform<N>> + 'a {
    map(tag("."), |_| Transform::ContextItem)
}
