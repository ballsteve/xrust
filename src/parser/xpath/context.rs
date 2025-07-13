//! Functions that manipulate the context.

use crate::item::Node;
use crate::parser::combinators::map::map;
use crate::parser::combinators::tag::tag;
use crate::parser::{ParseError, ParseInput};
use crate::qname::Interner;
use crate::transform::Transform;

// ContextItemExpr ::= '.'
pub(crate) fn context_item<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(map(tag("."), |_| Transform::ContextItem))
}
