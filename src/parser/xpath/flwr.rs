//! XPath FLWR expressions.

use crate::item::Node;
use crate::parser::combinators::list::separated_list1;
use crate::parser::combinators::map::map;
use crate::parser::combinators::pair::pair;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::{tuple10, tuple3, tuple5, tuple6};
use crate::parser::combinators::whitespace::xpwhitespace;
//use crate::parser::combinators::debug::inspect;
use crate::parser::xpath::nodetests::qualname_test;
use crate::parser::xpath::support::get_nt_localname;
use crate::parser::xpath::{expr_single_wrapper, expr_wrapper};
use crate::parser::{ParseError, ParseInput};
use crate::transform::Transform;

// IfExpr ::= 'if' '(' Expr ')' 'then' ExprSingle 'else' ExprSingle
pub(crate) fn if_expr<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a> {
    Box::new(map(
        pair(
            // need tuple15
            tuple10(
                tag("if"),
                xpwhitespace(),
                tag("("),
                xpwhitespace(),
                expr_wrapper::<N>(true),
                xpwhitespace(),
                tag(")"),
                xpwhitespace(),
                tag("then"),
                xpwhitespace(),
            ),
            tuple5(
                expr_single_wrapper::<N>(true),
                xpwhitespace(),
                tag("else"),
                xpwhitespace(),
                expr_single_wrapper::<N>(true),
            ),
        ),
        |((_, _, _, _, i, _, _, _, _, _), (t, _, _, _, e))| {
            Transform::Switch(vec![(i, t)], Box::new(e))
        },
    ))
}

// ForExpr ::= SimpleForClause 'return' ExprSingle
pub(crate) fn for_expr<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a> {
    Box::new(map(
        tuple3(
            simple_for_clause::<N>(),
            tuple3(xpwhitespace(), tag("return"), xpwhitespace()),
            expr_single_wrapper::<N>(true),
        ),
        |(f, _, e)| Transform::Loop(f, Box::new(e)), // tc_loop does not yet support multiple variable bindings
    ))
}

// SimpleForClause ::= 'for' SimpleForBinding (',' SimpleForBinding)*
// SimpleForBinding ::= '$' VarName 'in' ExprSingle
fn simple_for_clause<'a, N: Node + 'a>() -> Box<
    dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Vec<(String, Transform<N>)>), ParseError> + 'a,
> {
    Box::new(map(
        tuple3(
            tag("for"),
            xpwhitespace(),
            separated_list1(
                map(tuple3(xpwhitespace(), tag(","), xpwhitespace()), |_| ()),
                map(
                    tuple6(
                        tag("$"),
                        qualname_test(),
                        xpwhitespace(),
                        tag("in"),
                        xpwhitespace(),
                        expr_single_wrapper::<N>(true),
                    ),
                    |(_, qn, _, _, _, e)| (get_nt_localname(&qn), e),
                ),
            ),
        ),
        |(_, _, v)| v,
    ))
}

// LetExpr ::= SimpleLetClause 'return' ExprSingle
pub(crate) fn let_expr<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a> {
    Box::new(map(
        tuple3(
            simple_let_clause::<N>(),
            tuple3(xpwhitespace(), tag("return"), xpwhitespace()),
            expr_single_wrapper::<N>(true),
        ),
        |(mut v, _, e)| {
            let (qn, f) = v.pop().unwrap();
            let mut result = Transform::VariableDeclaration(qn, Box::new(f), Box::new(e));
            loop {
                if v.is_empty() {
                    break;
                } else {
                    let (qn, f) = v.pop().unwrap();
                    let inter = Transform::VariableDeclaration(qn, Box::new(f), Box::new(result));
                    result = inter;
                }
            }
            result
        },
    ))
}

// SimpleLetClause ::= 'let' SimpleLetBinding (',' SimpleLetBinding)*
// SimpleLetBinding ::= '$' VarName ':=' ExprSingle
// TODO: handle multiple bindings
fn simple_let_clause<'a, N: Node + 'a>() -> Box<
    dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Vec<(String, Transform<N>)>), ParseError> + 'a,
> {
    Box::new(map(
        tuple3(
            tag("let"),
            xpwhitespace(),
            separated_list1(
                map(tuple3(xpwhitespace(), tag(","), xpwhitespace()), |_| ()),
                map(
                    tuple6(
                        tag("$"),
                        qualname_test(),
                        xpwhitespace(),
                        tag(":="),
                        xpwhitespace(),
                        expr_single_wrapper::<N>(true),
                    ),
                    |(_, qn, _, _, _, e)| (get_nt_localname(&qn), e),
                ),
            ),
        ),
        |(_, _, v)| v,
    ))
}
