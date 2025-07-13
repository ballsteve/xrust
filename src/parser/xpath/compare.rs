//! Functions that produce comparisons.

use crate::item::Node;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::pair::pair;
use crate::parser::combinators::tag::anytag;
use crate::parser::combinators::tuple::tuple3;
use crate::parser::combinators::whitespace::xpwhitespace;
use crate::parser::xpath::strings::stringconcat_expr;
use crate::parser::{ParseError, ParseInput};
use crate::qname::Interner;
use crate::transform::Transform;
use crate::value::Operator;

// ComparisonExpr ::= StringConcatExpr ( (ValueComp | GeneralComp | NodeComp) StringConcatExpr)?
pub(crate) fn comparison_expr<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(map(
        pair(
            stringconcat_expr::<I, N>(),
            opt(pair(
                tuple3(
                    xpwhitespace(),
                    anytag(vec![
                        "=", "!=", "<", "<=", "<<", ">", ">=", ">>", "eq", "ne", "lt", "le", "gt",
                        "ge", "is",
                    ]),
                    xpwhitespace(),
                ),
                stringconcat_expr::<I, N>(),
            )),
        ),
        |(v, o)| match o {
            None => v,
            Some(((_, b, _), t)) => {
                match b.as_str() {
                    "=" | "!=" | "<" | "<=" | ">" | ">=" => {
                        Transform::GeneralComparison(Operator::from(b), Box::new(v), Box::new(t))
                    }
                    "eq" | "ne" | "lt" | "le" | "gt" | "ge" | "is" | "<<" | ">>" => {
                        Transform::ValueComparison(Operator::from(b), Box::new(v), Box::new(t))
                    }
                    _ => Transform::Empty, // error
                }
            }
        },
    ))
}
