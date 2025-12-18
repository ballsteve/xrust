//! General productions for XPath expressions.

use crate::item::Node;
use crate::parser::combinators::alt::{alt2, alt5};
use crate::parser::combinators::map::map;
use crate::parser::{ParseError, ParseInput, StaticState};
//use crate::parser::combinators::debug::inspect;
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::pair::pair;
use crate::parser::combinators::tag::tag;
use crate::parser::xpath::context::context_item;
use crate::parser::xpath::expr_wrapper;
use crate::parser::xpath::functions::function_call;
use crate::parser::xpath::literals::literal;
use crate::parser::xpath::predicates::predicate_list;
use crate::parser::xpath::variables::variable_reference;
use crate::transform::Transform;
use qualname::{NamespacePrefix, NamespaceUri};

// PostfixExpr ::= PrimaryExpr (Predicate | ArgumentList | Lookup)*
// TODO: arg list, lookup
pub(crate) fn postfix_expr<'a, N: Node + 'a, L>() -> Box<
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
        pair(primary_expr::<N, L>(), predicate_list()),
        |(pr, pl)| Transform::Compose(vec![pr, pl]),
    ))
}

// PrimaryExpr ::= Literal | VarRef | ParenthesizedExpr | ContextItemExpr | FunctionCall | FunctionItemExpr | MapConstructor | ArrayConstructor | UnaryLookup
// TODO: finish this parser
fn primary_expr<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(alt5(
        literal::<N, L>(),
        parenthesized_expr::<N, L>(),
        function_call::<N, L>(),
        variable_reference::<N, L>(),
        context_item::<N, L>(),
    ))
}

// ParenthesizedExpr ::= '(' Expr? ')'
pub(crate) fn parenthesized_expr<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(alt2(
        parenthesized_expr_empty::<N, L>(),
        parenthesized_expr_nonempty::<N, L>(),
    ))
}
fn parenthesized_expr_empty<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(tag("()"), |_| Transform::Empty))
}
fn parenthesized_expr_nonempty<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(delimited(
        tag("("),
        map(expr_wrapper::<N, L>(true), |e| e),
        tag(")"),
    ))
}
