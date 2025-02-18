//! Logic expressions in XPath.

use crate::item::Node;
use crate::parser::combinators::list::separated_list1;
use crate::parser::combinators::map::map;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::tuple3;
use crate::parser::combinators::whitespace::xpwhitespace;
use crate::parser::xpath::compare::comparison_expr;
use crate::parser::{ParseError, ParseInput};
use crate::transform::Transform;

// OrExpr ::= AndExpr ('or' AndExpr)*
pub(crate) fn or_expr<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a> {
    Box::new(map(
        separated_list1(
            map(tuple3(xpwhitespace(), tag("or"), xpwhitespace()), |_| ()),
            and_expr::<N>(),
        ),
        |mut v| {
            if v.len() == 1 {
                v.pop().unwrap()
            } else {
                Transform::Or(v)
            }
        },
    ))
}

// AndExpr ::= ComparisonExpr ('and' ComparisonExpr)*
fn and_expr<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a> {
    Box::new(map(
        separated_list1(
            map(tuple3(xpwhitespace(), tag("and"), xpwhitespace()), |_| ()),
            comparison_expr::<N>(),
        ),
        |mut v| {
            if v.len() == 1 {
                v.pop().unwrap()
            } else {
                Transform::And(v)
            }
        },
    ))
}
