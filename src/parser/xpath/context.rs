//! Functions that manipulate the context.

use crate::item::Node;
use crate::parser::combinators::map::map;
use crate::parser::combinators::tag::tag;
use crate::parser::{ParseError, ParseInput};
use crate::transform::Transform;

// ContextItemExpr ::= '.'
pub(crate) fn context_item<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a> {
    Box::new(map(tag("."), |_| Transform::ContextItem))
}
