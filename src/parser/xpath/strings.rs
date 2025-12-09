//! Functions that produce strings.

use crate::item::Node;
use crate::parser::combinators::list::separated_list1;
use crate::parser::combinators::map::map;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::tuple3;
use crate::parser::combinators::whitespace::xpwhitespace;
use crate::parser::xpath::numbers::range_expr;
use crate::parser::{ParseError, ParseInput, StaticState};
use crate::transform::Transform;
use qualname::{NamespacePrefix, NamespaceUri};

// StringConcatExpr ::= RangeExpr ( '||' RangeExpr)*
pub(crate) fn stringconcat_expr<'a, N: Node + 'a, L>() -> Box<
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
        separated_list1(
            map(tuple3(xpwhitespace(), tag("||"), xpwhitespace()), |_| ()),
            range_expr::<N, L>(),
        ),
        |mut v| {
            if v.len() == 1 {
                v.pop().unwrap()
            } else {
                Transform::Concat(v)
            }
        },
    ))
}
