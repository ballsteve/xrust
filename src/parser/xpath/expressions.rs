//! General productions for XPath expressions.

use crate::item::Node;
use crate::parser::combinators::alt::{alt2, alt5};
use crate::parser::combinators::map::map;
use crate::parser::{ParseError, ParseInput};
use crate::qname::Interner;
//use crate::parser::combinators::debug::inspect;
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::tag::tag;
use crate::parser::xpath::context::context_item;
use crate::parser::xpath::expr_wrapper;
use crate::parser::xpath::functions::function_call;
use crate::parser::xpath::literals::literal;
use crate::parser::xpath::variables::variable_reference;
use crate::transform::Transform;

// PostfixExpr ::= PrimaryExpr (Predicate | ArgumentList | Lookup)*
// TODO: predicates, arg list, lookup
pub(crate) fn postfix_expr<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(primary_expr::<I, N>())
}

// PrimaryExpr ::= Literal | VarRef | ParenthesizedExpr | ContextItemExpr | FunctionCall | FunctionItemExpr | MapConstructor | ArrayConstructor | UnaryLookup
// TODO: finish this parser
fn primary_expr<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(alt5(
        literal::<I, N>(),
        parenthesized_expr::<I, N>(),
        function_call::<I, N>(),
        variable_reference::<I, N>(),
        context_item::<I, N>(),
    ))
}

// ParenthesizedExpr ::= '(' Expr? ')'
pub(crate) fn parenthesized_expr<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(alt2(
        parenthesized_expr_empty::<I, N>(),
        parenthesized_expr_nonempty::<I, N>(),
    ))
}
fn parenthesized_expr_empty<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(map(tag("()"), |_| Transform::Empty))
}
fn parenthesized_expr_nonempty<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(delimited(
        tag("("),
        map(expr_wrapper::<I, N>(true), |e| e),
        tag(")"),
    ))
}
