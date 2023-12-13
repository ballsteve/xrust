//! Functions that produce numbers.

use crate::item::Node;
use crate::parser::combinators::alt::{alt2, alt4};
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::pair::pair;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::{tuple2, tuple3};
use crate::parser::combinators::whitespace::xpwhitespace;
use crate::parser::xpath::nodes::{path_expr, union_expr};
use crate::parser::{ParseInput, ParseResult};
use crate::transform::{ArithmeticOperand, ArithmeticOperator, Transform};

// RangeExpr ::= AdditiveExpr ( 'to' AdditiveExpr)?
pub(crate) fn range_expr<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        pair(
            additive_expr::<N>(),
            opt(tuple2(
                tuple3(xpwhitespace(), tag("to"), xpwhitespace()),
                additive_expr::<N>(),
            )),
        ),
        |(v, o)| match o {
            None => v,
            Some((_, u)) => Transform::Range(Box::new(v), Box::new(u)),
        },
    ))
}

// AdditiveExpr ::= MultiplicativeExpr ( ('+' | '-') MultiplicativeExpr)*
fn additive_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        pair(
            multiplicative_expr::<N>(),
            many0(tuple2(
                alt2(
                    map(
                        tuple3(
                            xpwhitespace(),
                            map(tag("+"), |_| ArithmeticOperator::Add),
                            xpwhitespace(),
                        ),
                        |(_, x, _)| x,
                    ),
                    map(
                        tuple3(
                            xpwhitespace(),
                            map(tag("-"), |_| ArithmeticOperator::Subtract),
                            xpwhitespace(),
                        ),
                        |(_, x, _)| x,
                    ),
                ),
                multiplicative_expr::<N>(),
            )),
        ),
        |(mut a, b)| {
            if b.is_empty() {
                if a.len() == 1 {
                    let c: ArithmeticOperand<N> = a.pop().unwrap();
                    c.operand
                } else {
                    Transform::Arithmetic(a)
                }
            } else {
                let mut e: Vec<ArithmeticOperand<N>> = b
                    .iter()
                    .map(|(c, d)| {
                        ArithmeticOperand::new(c.clone(), Transform::Arithmetic(d.clone()))
                    })
                    .collect();
                a.append(&mut e);

                Transform::Arithmetic(a)
            }
        },
    ))
}

// MultiplicativeExpr ::= UnionExpr ( ('*' | 'div' | 'idiv' | 'mod') UnionExpr)*
fn multiplicative_expr<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Vec<ArithmeticOperand<N>>> + 'a> {
    Box::new(map(
        pair(
            union_expr::<N>(),
            many0(tuple2(
                alt4(
                    tuple3(xpwhitespace(), map(tag("*"), |_| "*"), xpwhitespace()),
                    tuple3(xpwhitespace(), map(tag("div"), |_| "div"), xpwhitespace()),
                    tuple3(xpwhitespace(), map(tag("idiv"), |_| "idiv"), xpwhitespace()),
                    tuple3(xpwhitespace(), map(tag("mod"), |_| "mod"), xpwhitespace()),
                ),
                union_expr::<N>(),
            )),
        ),
        |(a, b)| {
            if b.is_empty() {
                vec![ArithmeticOperand::new(ArithmeticOperator::Noop, a)]
            } else {
                // The arguments to the constructor are the items to be summed
                // These are pair-wise items: first is the operator,
                // second is the combinator for the value
                let mut r: Vec<ArithmeticOperand<N>> = Vec::new();

                r.push(ArithmeticOperand::new(ArithmeticOperator::Noop, a));

                for ((_, c, _), d) in b {
                    r.push(ArithmeticOperand::new(ArithmeticOperator::from(c), d))
                }
                r
            }
        },
    ))
}

// UnaryExpr ::= ('-' | '+')* ValueExpr
pub(crate) fn unary_expr<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        pair(many0(alt2(tag("-"), tag("+"))), value_expr::<N>()),
        |(u, v)| {
            if u.is_empty() {
                v
            } else {
                Transform::NotImplemented("unary_expr".to_string())
            }
        },
    ))
}

// ValueExpr (SBox<dyneMapExpr) ::= PathExpr ('!' PathExpr)*
fn value_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        pair(path_expr::<N>(), many0(tuple2(tag("!"), path_expr::<N>()))),
        |(u, v)| {
            if v.is_empty() {
                u
            } else {
                Transform::NotImplemented("value_expr".to_string())
            }
        },
    ))
}
