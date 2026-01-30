//! Functions that manipulate the context.

use crate::item::Node;
use crate::parser::combinators::map::map;
use crate::parser::combinators::tag::tag;
use crate::parser::{ParseError, ParseInput, StaticState};
use crate::transform::Transform;
use qualname::{NamespacePrefix, NamespaceUri};

// ContextItemExpr ::= '.'
pub(crate) fn context_item<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(tag("."), |_| Transform::ContextItem))
}
