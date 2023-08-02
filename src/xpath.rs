//! # xrust::xpath
//!
//! An XPath parser using the xrust parser combinator that produces a xrust transformation combinator.

use std::cmp::Ordering;
use std::rc::Rc;

use crate::evaluate::{ArithmeticOperator, Axis, NameTest, NodeMatch, NodeTest, WildcardOrName};
use crate::item::{Item, Node};
use crate::transcomb::{
    arithmetic, boolean, ceiling, compose, contains, context, current_date, current_date_time,
    current_group, current_grouping_key, current_time, declare_variable, empty, floor, format_date,
    format_date_time, format_time, general_comparison, last, literal as tc_literal, local_name,
    name, normalize_space, not, not_implemented, number, position, reference_variable, root, round,
    starts_with, step, string, substring, substring_after, substring_before, sum, switch, tc_and,
    tc_concat, tc_count, tc_false, tc_loop, tc_or, tc_range, tc_sequence, tc_true, translate,
    value_comparison, Combinator, Context, TransResult,
};
use crate::value::Value;
use crate::value::*;
use crate::xdmerror::*;
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::parser::combinators::alt::{alt2, alt3, alt4, alt5};
use crate::parser::combinators::debug::inspect;
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::list::{separated_list0, separated_list1};
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::pair::pair;
use crate::parser::combinators::tag::{anychar, tag};
use crate::parser::combinators::tuple::{tuple10, tuple2, tuple3, tuple4, tuple5, tuple6};
use crate::parser::combinators::whitespace::whitespace0;
use crate::parser::common::ncname;
use crate::parser::{ParseError, ParseInput, ParseResult};

pub fn parse<'a, N: Node, F>(e: &'a str) -> Result<Combinator<'a, N>, Error>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    if e == "" {
        Ok(empty())
    } else {
        let mut input = ParseInput::new(e);
        //input.set_limit(100);
        match xpath_expr::<N, F>(input) {
            Ok((rem, f)) => {
                if rem.clone().peekable().peek().is_some() {
                    Err(Error::new(
                        ErrorKind::Unknown,
                        format!("extra characters found: \"{}\"", rem),
                    ))
                } else {
                    Ok(f)
                }
            }
            Err(err) => Err(Error::new(ErrorKind::Unknown, format!("{:?}", err))),
        }
    }
}

// Expr ::= ExprSingle (',' ExprSingle)* ;
fn xpath_expr<'a, N: Node + 'a, F>(input: ParseInput) -> ParseResult<Combinator<'a, N>>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    expr::<N, F>()(input)
}

// Implementation note: cannot use opaque type because XPath expressions are recursive, and Rust *really* doesn't like recursive opaque types. Dynamic trait objects aren't ideal, but compiling XPath expressions is a one-off operation so that shouldn't cause a major performance issue.
// Implementation note 2: since XPath is recursice, must lazily evaluate arguments to avoid stack overflow.
fn expr<'a, N: Node + 'a, F>() -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        inspect("expr", separated_list1(
            map(tuple3(xpwhitespace(), tag(","), xpwhitespace()), |_| ()),
            expr_single::<N, F>(),
        )),
        |v| tc_sequence(v),
    ))
}

fn expr_wrapper<'a, N: Node + 'a, F>(
    b: bool,
) -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    move |input| {
        if b {
            expr::<N, F>()(input)
        } else {
            noop()(input)
        }
    }
}

// ExprSingle ::= ForExpr | LetExpr | QuantifiedExpr | IfExpr | OrExpr
fn expr_single<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(inspect(
        "expr_single",
        alt4(
            or_expr::<N, F>(),
            let_expr::<N, F>(),
            for_expr::<N, F>(),
            if_expr::<N, F>(),
        ),
    ))
}

fn expr_single_wrapper<'a, N: Node + 'a, F>(
    b: bool,
) -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    move |input| {
        if b {
            expr_single::<N, F>()(input)
        } else {
            noop()(input)
        }
    }
}

// IfExpr ::= 'if' '(' Expr ')' 'then' ExprSingle 'else' ExprSingle
fn if_expr<'a, N: Node + 'a, F>() -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        pair(
            // need tuple15
            tuple10(
                tag("if"),
                xpwhitespace(),
                tag("("),
                xpwhitespace(),
                expr_wrapper::<N, F>(true),
                xpwhitespace(),
                tag(")"),
                xpwhitespace(),
                tag("then"),
                xpwhitespace(),
            ),
            tuple5(
                expr_single_wrapper::<N, F>(true),
                xpwhitespace(),
                tag("else"),
                xpwhitespace(),
                expr_single_wrapper::<N, F>(true),
            ),
        ),
        |((_, _, _, _, i, _, _, _, _, _), (t, _, _, _, e))| switch(vec![(i, t)], e),
    ))
}

// ForExpr ::= SimpleForClause 'return' ExprSingle
fn for_expr<'a, N: Node + 'a, F>() -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        tuple3(
            simple_for_clause::<N, F>(),
            tuple3(xpwhitespace(), tag("return"), xpwhitespace()),
            expr_single_wrapper::<N, F>(true),
        ),
        |(mut f, _, e)| tc_loop(f.pop().unwrap(), e), // tc_loop does not yet support multiple variable bindings
    ))
}

// SimpleForClause ::= 'for' SimpleForBinding (',' SimpleForBinding)*
// SimpleForBinding ::= '$' VarName 'in' ExprSingle
fn simple_for_clause<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Vec<(String, Combinator<'a, N>)>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        tuple3(
            tag("for"),
            xpwhitespace(),
            separated_list1(
                map(tuple3(xpwhitespace(), tag(","), xpwhitespace()), |_| ()),
                map(
                    tuple6(
                        tag("$"),
                        qname(),
                        xpwhitespace(),
                        tag("in"),
                        xpwhitespace(),
                        expr_single_wrapper::<N, F>(true),
                    ),
                    |(_, qn, _, _, _, e)| (get_nt_localname(&qn), e),
                ),
            ),
        ),
        |(_, _, v)| v,
    ))
}

// LetExpr ::= SimpleLetClause 'return' ExprSingle
fn let_expr<'a, N: Node + 'a, F>() -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        tuple3(
            simple_let_clause::<N, F>(),
            tuple3(xpwhitespace(), tag("return"), xpwhitespace()),
            expr_single_wrapper::<N, F>(true),
        ),
        |(mut v, _, e)| {
            let (qn, f) = v.pop().unwrap();
            let mut result = declare_variable(qn, f, e);
            loop {
                if v.is_empty() {
                    break;
                } else {
                    let (qn, f) = v.pop().unwrap();
                    let inter = declare_variable(qn, f, result);
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
fn simple_let_clause<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Vec<(String, Combinator<'a, N>)>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        tuple3(
            tag("let"),
            xpwhitespace(),
            separated_list1(
                map(tuple3(xpwhitespace(), tag(","), xpwhitespace()), |_| ()),
                map(
                    tuple6(
                        tag("$"),
                        qname(),
                        xpwhitespace(),
                        tag(":="),
                        xpwhitespace(),
                        expr_single_wrapper::<N, F>(true),
                    ),
                    |(_, qn, _, _, _, e)| (get_nt_localname(&qn), e),
                ),
            ),
        ),
        |(_, _, v)| v,
    ))
}

fn get_nt_localname(nt: &NodeTest) -> String {
    match nt {
        NodeTest::Name(NameTest {
            name: Some(WildcardOrName::Name(localpart)),
            ns: None,
            prefix: None,
        }) => localpart.to_string(),
        _ => String::from("invalid qname"),
    }
}

// OrExpr ::= AndExpr ('or' AndExpr)*
fn or_expr<'a, N: Node + 'a, F>() -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        inspect("or_expr", separated_list1(
            map(tuple3(xpwhitespace(), tag("or"), xpwhitespace()), |_| ()),
            inspect("or_expr: and", and_expr::<N, F>()),
        )),
        |mut v| {
            if v.len() == 1 {
		v.pop().unwrap()
            } else {
                tc_or(v)
            }
        },
    ))
}

// AndExpr ::= ComparisonExpr ('and' ComparisonExpr)*
fn and_expr<'a, N: Node + 'a, F>() -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        inspect("and_expr", separated_list1(
            map(tuple3(xpwhitespace(), tag("and"), xpwhitespace()), |_| ()),
            comparison_expr::<N, F>(),
        )),
        |v| {
            if v.len() == 1 {
                // TODO: This is inefficient, but Rust is not allowing v[0]
                tc_sequence(v)
            } else {
                tc_and(v)
            }
        },
    ))
}

// ComparisonExpr ::= StringConcatExpr ( (ValueComp | GeneralComp | NodeComp) StringConcatExpr)?
fn comparison_expr<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        pair(
            stringconcat_expr::<N, F>(),
            opt(pair(
                tuple3(
                    xpwhitespace(),
                    anytag(vec![
                        (vec!["=", ""], false),
                        (vec!["!", "="], false),
                        (vec!["<", "=<"], true),
                        (vec![">", "=>"], true),
                        (vec!["e", "q"], false),
                        (vec!["n", "e"], false),
                        (vec!["l", "te"], false),
                        (vec!["g", "te"], false),
                        (vec!["i", "s"], false),
                    ]),
                    xpwhitespace(),
                ),
                stringconcat_expr::<N, F>(),
            )),
        ),
        |(v, o)| match o {
            None => v,
            Some(((_, b, _), t)) => {
                match b.as_str() {
                    "=" | "!=" | "<" | "<=" | ">" | ">=" => {
                        general_comparison(Operator::from(b), v, t)
                    }
                    "eq" | "ne" | "lt" | "le" | "gt" | "ge" | "is" | "<<" | ">>" => {
                        value_comparison(Operator::from(b), v, t)
                    }
                    _ => empty(), // error
                }
            }
        },
    ))
}

// StringConcatExpr ::= RangeExpr ( '||' RangeExpr)*
fn stringconcat_expr<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        inspect("stringconcat", separated_list1(
            map(tuple3(xpwhitespace(), tag("||"), xpwhitespace()), |_| ()),
            range_expr::<N, F>(),
        )),
        |v| {
            if v.len() == 1 {
                // TODO: rust doesn't like v[0], see above
                tc_sequence(v)
            } else {
                tc_concat(v)
            }
        },
    ))
}

// RangeExpr ::= AdditiveExpr ( 'to' AdditiveExpr)?
fn range_expr<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        pair(
            additive_expr::<N, F>(),
            opt(tuple2(
                tuple3(xpwhitespace(), tag("to"), xpwhitespace()),
                additive_expr::<N, F>(),
            )),
        ),
        |(v, o)| match o {
            None => v,
            Some((_, u)) => tc_range(v, u),
        },
    ))
}

// AdditiveExpr ::= MultiplicativeExpr ( ('+' | '-') MultiplicativeExpr)*
fn additive_expr<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        pair(
            multiplicative_expr::<N, F>(),
            many0(tuple2(
                alt2(
                    tuple3(xpwhitespace(), map(tag("+"), |_| "+"), xpwhitespace()),
                    tuple3(xpwhitespace(), map(tag("-"), |_| "-"), xpwhitespace()),
                ),
                multiplicative_expr::<N, F>(),
            )),
        ),
        |(mut a, b)| {
            if b.is_empty() {
                if a.len() == 1 {
                    // TODO: see above
                    let mut r = Vec::new();
                    for (_, c) in a {
                        r.push(c);
                    }
                    tc_sequence(r)
                } else {
                    arithmetic(a)
                }
            } else {
                // The arguments to the constructor are the items to be summed
                // These are pair-wise items: first is the operator,
                // second is the combinator for the value
                let mut r: Vec<(ArithmeticOperator, Combinator<N>)> = Vec::new();

                r.append(&mut a);

                for ((_, c, _), d) in b {
                    r.push((ArithmeticOperator::from(c), arithmetic(d)))
                }
                arithmetic(r)
            }
        },
    ))
}

// MultiplicativeExpr ::= UnionExpr ( ('*' | 'div' | 'idiv' | 'mod') UnionExpr)*
fn multiplicative_expr<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Vec<(ArithmeticOperator, Combinator<'a, N>)>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        pair(
            union_expr::<N, F>(),
            many0(tuple2(
                alt4(
                    tuple3(xpwhitespace(), map(tag("*"), |_| "*"), xpwhitespace()),
                    tuple3(xpwhitespace(), map(tag("div"), |_| "div"), xpwhitespace()),
                    tuple3(xpwhitespace(), map(tag("idiv"), |_| "idiv"), xpwhitespace()),
                    tuple3(xpwhitespace(), map(tag("mod"), |_| "mod"), xpwhitespace()),
                ),
                union_expr::<N, F>(),
            )),
        ),
        |(a, b)| {
            if b.is_empty() {
                vec![(ArithmeticOperator::Noop, a)]
            } else {
                // The arguments to the constructor are the items to be summed
                // These are pair-wise items: first is the operator,
                // second is the combinator for the value
                let mut r: Vec<(ArithmeticOperator, Combinator<N>)> = Vec::new();

                r.push((ArithmeticOperator::Noop, a));

                for ((_, c, _), d) in b {
                    r.push((ArithmeticOperator::from(c), d))
                }
                r
            }
        },
    ))
}

// UnionExpr ::= IntersectExceptExpr ( ('union' | '|') IntersectExceptExpr)*
fn union_expr<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        inspect("union", separated_list1(
            map(
                tuple3(xpwhitespace(), alt2(tag("union"), tag("|")), xpwhitespace()),
                |_| (),
            ),
            intersectexcept_expr::<N, F>(),
        )),
        |v| {
            if v.len() == 1 {
                // TODO: see above
                tc_sequence(v)
            } else {
                not_implemented("union_expr".to_string())
            }
        },
    ))
}

// IntersectExceptExpr ::= InstanceOfExpr ( ('intersect' | 'except') InstanceOfExpr)*
fn intersectexcept_expr<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        pair(
            instanceof_expr::<N, F>(),
            many0(tuple2(
                tuple3(
                    xpwhitespace(),
                    alt2(tag("intersect"), tag("except")),
                    xpwhitespace(),
                ),
                instanceof_expr::<N, F>(),
            )),
        ),
        |(v, o)| {
            if o.is_empty() {
                v
            } else {
                not_implemented("intersectexcept_expr".to_string())
            }
        },
    ))
}

// InstanceOfExpr ::= TreatExpr ( 'instance' 'of' SequenceType)?
fn instanceof_expr<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        pair(
            treat_expr::<N, F>(),
            opt(tuple6(
                xpwhitespace(),
                tag("instance"),
                xpwhitespace(),
                tag("of"),
                xpwhitespace(),
                sequencetype_expr::<N>(),
            )),
        ),
        |(v, o)| {
            if o.is_none() {
                v
            } else {
                not_implemented("instanceof_expr".to_string())
            }
        },
    ))
}

// SequenceType ::= ( 'empty-sequence' '(' ')' | (ItemType OccurrenceIndicator?)
// TODO: Box<dynement this parser fully
fn sequencetype_expr<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a> {
    Box::new(map(tag("empty-sequence()"), |_| {
        not_implemented("sequencetype_expr".to_string())
    }))
}

// TreatExpr ::= CastableExpr ( 'treat' 'as' SequenceType)?
fn treat_expr<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        pair(
            castable_expr::<N, F>(),
            opt(tuple6(
                xpwhitespace(),
                tag("treat"),
                xpwhitespace(),
                tag("as"),
                xpwhitespace(),
                sequencetype_expr::<N>(),
            )),
        ),
        |(v, o)| {
            if o.is_none() {
                v
            } else {
                not_implemented("treat_expr".to_string())
            }
        },
    ))
}

// CastableExpr ::= CastExpr ( 'castable' 'as' SingleType)?
fn castable_expr<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        pair(
            cast_expr::<N, F>(),
            opt(tuple6(
                xpwhitespace(),
                tag("castable"),
                xpwhitespace(),
                tag("as"),
                xpwhitespace(),
                singletype_expr::<N>(),
            )),
        ),
        |(v, o)| {
            if o.is_none() {
                v
            } else {
                not_implemented("castable_expr".to_string())
            }
        },
    ))
}

// SingleType ::= SimpleTypeName '?'?
// SimpleTypeName ::= TypeName
// TypeName ::= EQName
// EQName ::= QName | URIQualifiedName
// URIQualifiedName ::= BracedURILiteral NCName
// QName ::= PrefixedName | UnprefixedName
// PrefixedName ::= Prefix ':' LocalPart
// UnprefixedName ::= LocalPart
// Prefix ::= NCName
// LocalPart ::= NCName
// NCName ::= Name - (Char* ':' Char*)
// Char ::= #x9 | #xA |#xD | [#x20-#xD7FF] | [#xE000-#xFFFD | [#x10000-#x10FFFF]
// TODO: Box<dynement this parser fully
fn singletype_expr<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a> {
    Box::new(map(pair(qname(), tag("?")), |_| {
        not_implemented("singletype_expr".to_string())
    }))
}

// CastExpr ::= ArrowExpr ( 'cast' 'as' SingleType)?
fn cast_expr<'a, N: Node + 'a, F>() -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        pair(
            arrow_expr::<N, F>(),
            opt(tuple6(
                xpwhitespace(),
                tag("cast"),
                xpwhitespace(),
                tag("as"),
                xpwhitespace(),
                singletype_expr::<N>(),
            )),
        ),
        |(v, o)| {
            if o.is_none() {
                v
            } else {
                not_implemented("cast_expr".to_string())
            }
        },
    ))
}

// ArrowExpr ::= UnaryExpr ( '=>' ArrowFunctionSpecifier ArgumentList)*
fn arrow_expr<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        pair(
            unary_expr::<N, F>(),
            opt(tuple6(
                xpwhitespace(),
                tag("=>"),
                xpwhitespace(),
                arrowfunctionspecifier::<N, F>(),
                xpwhitespace(),
                opt(argumentlist::<N>()),
            )),
        ),
        |(v, o)| {
            if o.is_none() {
                v
            } else {
                not_implemented("arrow_expr".to_string())
            }
        },
    ))
}

// ArrowFunctionSpecifier ::= EQName | VarRef | ParenthesizedExpr
// TODO: finish this parser with EQName and VarRef
fn arrowfunctionspecifier<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        alt2(qname_expr::<N>(), parenthesized_expr::<N, F>()),
        |_| not_implemented("arrowfunctionspecifier".to_string()),
    ))
}
fn qname_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
{
    Box::new(map(qname(), |q| match q {
        NodeTest::Name(NameTest {
            name: Some(WildcardOrName::Name(localpart)),
            ns: None,
            prefix: None,
        }) => tc_literal(Rc::new(Item::Value(Value::from(localpart)))),
        _ => tc_literal(Rc::new(Item::Value(Value::from("invalid qname")))),
    }))
}

// ArgumentList ::= '(' (Argument (',' Argument)*)? ')'
// TODO: finish this parser with actual arguments
fn argumentlist<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
{
    Box::new(map(tag("()"), |_| {
        not_implemented("argumentlist".to_string())
    }))
}

// UnaryExpr ::= ('-' | '+')* ValueExpr
fn unary_expr<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        pair(many0(alt2(tag("-"), tag("+"))), value_expr::<N, F>()),
        |(u, v)| {
            if u.is_empty() {
                v
            } else {
                not_implemented("unary_expr".to_string())
            }
        },
    ))
}

// ValueExpr (SBox<dyneMapExpr) ::= PathExpr ('!' PathExpr)*
fn value_expr<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(inspect("value_expr", map(
        pair(
            path_expr::<N, F>(),
            many0(tuple2(tag("!"), path_expr::<N, F>())),
        ),
        |(u, v)| {
            if v.is_empty() {
                u
            } else {
                not_implemented("value_expr".to_string())
            }
        },
    )))
}

// PathExpr ::= ('/' RelativePathExpr?) | ('//' RelativePathExpr) | RelativePathExpr
fn path_expr<'a, N: Node + 'a, F>() -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(inspect("path_expr", alt3(
        absolutedescendant_expr::<N, F>(),
        absolutepath_expr::<N, F>(),
        relativepath_expr::<N, F>(),
    )))
}

// ('//' RelativePathExpr?)
fn absolutedescendant_expr<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        pair(tag("//"), relativepath_expr::<N, F>()),
        |(_, r)| {
            compose(vec![
                step(NodeMatch {
                    axis: Axis::DescendantOrSelfOrRoot,
                    nodetest: NodeTest::Name(NameTest {
                        ns: None,
                        prefix: None,
                        name: Some(WildcardOrName::Wildcard),
                    }),
                }),
                r,
            ])
        },
    ))
}

// ('/' RelativePathExpr?)
fn absolutepath_expr<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        pair(tag("/"), opt(relativepath_expr::<N, F>())),
        |(_, r)| match r {
            Some(a) => compose(vec![root(), a]),
            None => root(),
        },
    ))
}

// RelativePathExpr ::= StepExpr (('/' | '//') StepExpr)*
fn relativepath_expr<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        pair(
            inspect("relpath: looking for first step", step_expr::<N, F>()),
            many0(tuple2(
                inspect("relpath: looking for tag separator", alt2(
                    map(tuple3(xpwhitespace(), tag("//"), xpwhitespace()), |_| "//"),
                    map(tuple3(xpwhitespace(), tag("/"), xpwhitespace()), |_| "/"),
                )),
                inspect("relpath: looking for remaining steps", step_expr::<N, F>()),
            )),
        ),
        |(a, b)| {
            if b.is_empty() {
                a
            } else {
                let mut r = Vec::new();
                r.push(a);
                for (s, c) in b {
                    match s {
                        "/" => r.push(c),
                        "//" => {
                            // Insert a descendant-or-self::* step
                            r.push(step(NodeMatch {
                                axis: Axis::DescendantOrSelf,
                                nodetest: NodeTest::Name(NameTest {
                                    ns: None,
                                    prefix: None,
                                    name: Some(WildcardOrName::Wildcard),
                                }),
                            }));
                            r.push(c)
                        }
                        _ => panic!("unexpected"),
                    }
                }
                compose(r)
            }
        },
    ))
}

// StepExpr ::= PostfixExpr | AxisStep
fn step_expr<'a, N: Node + 'a, F>() -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(alt2(postfix_expr::<N, F>(), axisstep()))
}

// PostfixExpr ::= PrimaryExpr (Predicate | ArgumentList | Lookup)*
// TODO: predicates, arg list, lookup
fn postfix_expr<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(primary_expr::<N, F>())
}

// PrimaryExpr ::= Literal | VarRef | ParenthesizedExpr | ContextItemExpr | FunctionCall | FunctionItemExpr | MapConstructor | ArrayConstructor | UnaryLookup
// TODO: finish this parser
fn primary_expr<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(inspect(
        "primary_expr",
        alt5(
            inspect("literal", literal()),
            context_item(),
            parenthesized_expr::<N, F>(),
            inspect("function_call", function_call::<N, F>()),
            variable_reference(),
        ),
    ))
}

// FunctionCall ::= EQName ArgumentList
fn function_call<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        pair(qname(), arglist::<N, F>()),
        |(qn, mut a)| match qn {
            NodeTest::Name(NameTest {
                name: Some(WildcardOrName::Name(localpart)),
                ns: None,
                prefix: None,
            }) => match localpart.as_str() {
                "position" => position(),
                "last" => last(),
                "count" => {
                    if a.len() == 0 {
                        tc_count::<F, N>(None)
                    } else if a.len() == 1 {
                        tc_count(Some(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        empty()
                    }
                }
                "local-name" => {
                    if a.len() == 0 {
                        local_name::<F, N>(None)
                    } else if a.len() == 1 {
                        local_name(Some(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        empty()
                    }
                }
                "name" => {
                    if a.len() == 0 {
                        name::<F, N>(None)
                    } else if a.len() == 1 {
                        name(Some(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        empty()
                    }
                }
                "string" => {
                    if a.len() == 1 {
                        string(a.pop().unwrap())
                    } else {
                        // Too many arguments
                        empty()
                    }
                }
                "concat" => tc_concat(a),
                "starts-with" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        starts_with(c, b)
                    } else {
                        // Incorrect arguments
                        empty()
                    }
                }
                "contains" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        contains(c, b)
                    } else {
                        // Incorrect arguments
                        empty()
                    }
                }
                "substring" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        substring(c, b, None)
                    } else if a.len() == 3 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        let d = a.pop().unwrap();
                        substring(d, c, Some(b))
                    } else {
                        // Wrong number of arguments
                        empty()
                    }
                }
                "substring-before" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        substring_before(c, b)
                    } else {
                        // Incorrect arguments
                        empty()
                    }
                }
                "substring-after" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        substring_after(c, b)
                    } else {
                        // Incorrect arguments
                        empty()
                    }
                }
                "normalize-space" => {
                    if a.len() == 0 {
                        normalize_space::<F, N>(None)
                    } else if a.len() == 1 {
                        normalize_space(Some(a.pop().unwrap()))
                    } else {
                        // Wrong number of arguments
                        empty()
                    }
                }
                "translate" => {
                    if a.len() == 3 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        let d = a.pop().unwrap();
                        translate(d, c, b)
                    } else {
                        // Wrong number of arguments
                        empty()
                    }
                }
                "boolean" => {
                    if a.len() == 1 {
                        boolean(a.pop().unwrap())
                    } else {
                        // Too many arguments
                        empty()
                    }
                }
                "not" => {
                    if a.len() == 1 {
                        not(a.pop().unwrap())
                    } else {
                        // Too many arguments
                        empty()
                    }
                }
                "true" => {
                    if a.len() == 0 {
                        tc_true()
                    } else {
                        // Too many arguments
                        empty()
                    }
                }
                "false" => {
                    if a.len() == 0 {
                        tc_false()
                    } else {
                        // Too many arguments
                        empty()
                    }
                }
                "number" => {
                    if a.len() == 1 {
                        number(a.pop().unwrap())
                    } else {
                        // Too many arguments
                        empty()
                    }
                }
                "sum" => {
                    if a.len() == 1 {
                        sum(a.pop().unwrap())
                    } else {
                        // Too many arguments
                        empty()
                    }
                }
                "floor" => {
                    if a.len() == 1 {
                        floor(a.pop().unwrap())
                    } else {
                        // Too many arguments
                        empty()
                    }
                }
                "ceiling" => {
                    eprintln!("ceiling function");
                    if a.len() == 1 {
                        ceiling(a.pop().unwrap())
                    } else {
                        // Too many arguments
                        empty()
                    }
                }
                "round" => {
                    if a.len() == 1 {
                        let b = a.pop().unwrap();
                        round(b, None)
                    } else if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        round(c, Some(b))
                    } else {
                        // Wrong number of arguments
                        empty()
                    }
                }
                "current-date-time" => {
                    if a.len() == 0 {
                        current_date_time()
                    } else {
                        // Too many arguments
                        empty()
                    }
                }
                "current-date" => {
                    if a.len() == 0 {
                        current_date()
                    } else {
                        // Too many arguments
                        empty()
                    }
                }
                "current-time" => {
                    if a.len() == 0 {
                        current_time()
                    } else {
                        // Too many arguments
                        empty()
                    }
                }
                "format-date-time" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        format_date_time(c, b, None, None, None)
                    } else if a.len() == 5 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        let d = a.pop().unwrap();
                        let e = a.pop().unwrap();
                        let f = a.pop().unwrap();
                        format_date_time(f, e, Some(d), Some(c), Some(b))
                    } else {
                        // Too many arguments
                        empty()
                    }
                }
                "format-date" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        format_date(c, b, None, None, None)
                    } else if a.len() == 5 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        let d = a.pop().unwrap();
                        let e = a.pop().unwrap();
                        let f = a.pop().unwrap();
                        format_date(f, e, Some(d), Some(c), Some(b))
                    } else {
                        // Too many arguments
                        empty()
                    }
                }
                "format-time" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        format_time(c, b, None, None, None)
                    } else if a.len() == 5 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        let d = a.pop().unwrap();
                        let e = a.pop().unwrap();
                        let f = a.pop().unwrap();
                        format_time(f, e, Some(d), Some(c), Some(b))
                    } else {
                        // Too many arguments
                        empty()
                    }
                }
                "current-group" => {
                    if a.len() == 0 {
                        current_group()
                    } else {
                        // Too many arguments
                        empty()
                    }
                }
                "current-grouping-key" => {
                    if a.len() == 0 {
                        current_grouping_key()
                    } else {
                        // Too many arguments
                        empty()
                    }
                }
                _ => empty(), // TODO: user-defined functions
            },
            _ => empty(),
        },
    ))
}

// ArgumentList ::= '(' (Argument (',' Argument)*)? ')'
fn arglist<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Vec<Combinator<'a, N>>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        inspect(
            "arglist",
            tuple3(
                inspect("arglist open", tag("(")),
                inspect(
                    "sep0",
                    separated_list0(
                        map(tuple3(xpwhitespace(), tag(","), xpwhitespace()), |_| ()),
                        argument::<N, F>(),
                    ),
                ),
                inspect("arglist closed", tag(")")),
            ),
        ),
        |(_, a, _)| a,
    ))
}

// Argument ::= ExprSingle | ArgumentPlaceHolder
// TODO: ArgumentPlaceHolder
fn argument<'a, N: Node + 'a, F>() -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(inspect("argument", expr_single_wrapper::<N, F>(true)))
}

// VarRef ::= '$' VarName
fn variable_reference<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a> {
    Box::new(map(pair(tag("$"), qname()), |(_, qn)| {
        reference_variable(get_nt_localname(&qn))
    }))
}

// ParenthesizedExpr ::= '(' Expr? ')'
fn parenthesized_expr<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(alt2(
        parenthesized_expr_empty(),
        parenthesized_expr_nonempty::<N, F>(),
    ))
}
fn parenthesized_expr_empty<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a> {
    Box::new(map(tag("()"), |_| empty()))
}
fn parenthesized_expr_nonempty<'a, N: Node + 'a, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
where
    F: Fn(&mut Context<'a, N>) -> TransResult<'a, N> + 'a,
{
    Box::new(delimited(
        tag("("),
        map(expr_wrapper::<N, F>(true), |e| e),
        tag(")"),
    ))
}

// ContextItemExpr ::= '.'
fn context_item<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a>
{
    Box::new(map(tag("."), |_| context()))
}

// Literal ::= NumericLiteral | StringLiteral
fn literal<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a> {
    Box::new(alt2(numeric_literal(), string_literal()))
}

// NumericLiteral ::= IntegerLiteral | DecimalLiteral | DoubleLiteral
fn numeric_literal<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a> {
    Box::new(alt3(double_literal(), decimal_literal(), integer_literal()))
}
// IntegerLiteral ::= Digits
fn integer_literal<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a> {
    Box::new(map(digit1(), |s: String| {
        let n = s.parse::<i64>().unwrap();
        tc_literal(Rc::new(Item::Value(Value::Integer(n))))
    }))
}
// DecimalLiteral ::= ('.' Digits) | (Digits '.' [0-9]*)
// Construct a double, but if that fails fall back to decimal
fn decimal_literal<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a> {
    Box::new(alt2(decimal_literal_frac(), decimal_literal_comp()))
}
fn decimal_literal_frac<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a> {
    Box::new(map(pair(tag("."), digit1()), |(_, mut f)| {
        f.insert(0, '.');
        let n = f.parse::<f64>();
        let i = match n {
            Ok(m) => Value::Double(m),
            Err(_) => {
                f.insert_str(0, "0");
                Value::Decimal(Decimal::from_str(&f).unwrap())
            }
        };
        tc_literal(Rc::new(Item::Value(i)))
    }))
}
fn decimal_literal_comp<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a> {
    Box::new(inspect(
        "decimal",
        map(tuple3(digit1(), tag("."), digit0()), |(w, _, f)| {
            let s = format!("{}.{}", w, f);
            let n = s.parse::<f64>();
            let i = match n {
                Ok(m) => Value::Double(m),
                Err(_) => Value::Decimal(Decimal::from_str(&s).unwrap()),
            };
            eprintln!("decimal found \"{}\"", i);
            tc_literal(Rc::new(Item::Value(i)))
        }),
    ))
}

// DoubleLiteral ::= (('.' Digits) | (Digits ('.' [0-9]*)?)) [eE] [+-]? Digits
// Construct a double
fn double_literal<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a> {
    Box::new(alt2(double_literal_frac(), double_literal_comp()))
}

fn double_literal_frac<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a> {
    Box::new(map(
        tuple4(
            pair(tag("."), digit1()),
            alt2(tag("e"), tag("E")),
            opt(alt2(map(tag("+"), |_| "+"), map(tag("-"), |_| "-"))),
            digit1(),
        ),
        |((_, f), _, s, e)| {
            let n = format!("0.{}e{}{}", f, s.unwrap_or(""), e).parse::<f64>();
            let i = match n {
                Ok(m) => Value::Double(m),
                Err(_) => panic!("unable to convert to double"),
            };
            tc_literal(Rc::new(Item::Value(Value::from(i))))
        },
    ))
}
fn double_literal_comp<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a> {
    Box::new(map(
        tuple4(
            tuple3(digit1(), tag("."), digit1()),
            alt2(tag("e"), tag("E")),
            opt(alt2(map(tag("+"), |_| "+"), map(tag("-"), |_| "-"))),
            digit1(),
        ),
        |((c, _, f), _, s, e)| {
            let n = format!("{}.{}e{}{}", c, f, s.unwrap_or(""), e).parse::<f64>();
            let i = match n {
                Ok(m) => Value::Double(m),
                Err(_) => panic!("unable to convert to double"),
            };
            tc_literal(Rc::new(Item::Value(Value::from(i))))
        },
    ))
}

// StringLiteral ::= double- or single-quote delimited with double-delimiter escape
fn string_literal_double<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a> {
    Box::new(map(
        delimited(
            anychar('"'),
            map(many0(alt2(map(tag("\"\""), |_| '"'), none_of("\""))), |v| {
                v.iter().collect::<String>()
            }),
            anychar('"'),
        ),
        |s| tc_literal(Rc::new(Item::Value(Value::from(s)))),
    ))
}
fn string_literal_single<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a> {
    Box::new(map(
        delimited(
            anychar('\''),
            map(many0(alt2(map(tag("''"), |_| '\''), none_of("'"))), |v| {
                v.iter().collect::<String>()
            }),
            anychar('\''),
        ),
        |s| tc_literal(Rc::new(Item::Value(Value::from(s)))),
    ))
}
fn string_literal<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a> {
    Box::new(alt2(string_literal_double(), string_literal_single()))
}

/// Return zero or more digits from the input stream. Be careful not to consume non-digit input.
fn digit0() -> impl Fn(ParseInput) -> ParseResult<String> {
    move |mut input| {
        eprintln!("digit0: input \"{}\"", input);
        match input.clone().position(|c| !(c >= '0' && c <= '9')) {
            Some(0) => {
                eprintln!("digit0: non-digit at pos 0");
                Err(ParseError::Combinator)
            }
            Some(pos) => {
                let result = (&mut input).take(pos).collect::<String>();
                eprintln!("digit0: got digits \"{}\" input now \"{}\"", result, input);
                Ok((input, result))
            }
            None => {
                eprintln!("digit0: no non-digits");
                match input.clone().peekable().peek() {
                    Some(_) => {
                        let result = (&mut input).collect::<String>();
                        Ok((input, result))
                    }
                    _ => Err(ParseError::Combinator),
                }
            }
        }
    }
}
/// Return one or more digits from the input stream.
fn digit1() -> impl Fn(ParseInput) -> ParseResult<String> {
    move |mut input| match input.clone().peekable().peek() {
        Some(a) => match a {
            '0'..='9' => {
                // Take more digits out of the input stream,
                // but be careful not to take anything else
                match input.clone().position(|c| !(c >= '0' && c <= '9')) {
                    Some(0) => {
                        input.next();
                        Ok((input, a.to_string()))
                    }
                    Some(pos) => {
                        let result = (&mut input).take(pos).collect::<String>();
                        Ok((input, result))
                    }
                    None => {
                        let result = (&mut input).collect::<String>();
                        Ok((input, result))
                    }
                }
            }
            _ => Err(ParseError::Combinator),
        },
        None => Err(ParseError::Combinator),
    }
}

/// Return the next character if it is not from the given set
fn none_of(s: &str) -> impl Fn(ParseInput) -> ParseResult<char> + '_ {
    move |mut input| match input.clone().peekable().peek() {
        Some(a) => match s.chars().position(|b| *a == b) {
            Some(_) => Err(ParseError::Combinator),
            None => {
                let result = (&mut input).next().unwrap();
                Ok((input, result))
            }
        },
        None => Err(ParseError::Combinator),
    }
}

/// Return the longest possible of one of the given tags
fn anytag(s: Vec<(Vec<&str>, bool)>) -> impl Fn(ParseInput) -> ParseResult<String> + '_ {
    move |mut input| {
        let mut candidate: Option<(Vec<&str>, bool)> = None;
        match input.clone().peekable().peek() {
            Some(ch) => {
                s.iter().for_each(|t| {
                    let (u, v) = t;
                    if u[0].chars().nth(0).unwrap() == *ch {
                        candidate = Some(t.clone());
                    }
                });
                if candidate.is_none() {
                    Err(ParseError::Combinator)
                } else {
                    (&mut input).next();
                    match input.clone().peekable().peek() {
                        Some(ch) => {
                            let d = candidate.unwrap().0;
                            if (&d)[1].contains(*ch) {
                                (&mut input).next();
                                Ok((input, format!("{}{}", d[0], ch)))
                            } else {
                                Err(ParseError::Combinator)
                            }
                        }
                        None => {
                            let d = candidate.unwrap();
                            if (&d).1 {
                                Ok((input, d.0[0].to_string()))
                            } else {
                                Err(ParseError::Combinator)
                            }
                        }
                    }
                }
            }
            None => Err(ParseError::Combinator),
        }
    }
}

// AxisStep ::= (ReverseStep | ForwardStep) PredicateList
fn axisstep<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a> {
    Box::new(map(pair(forwardaxis(), nodetest()), |(a, n)| {
        step(NodeMatch {
            axis: Axis::from(a),
            nodetest: n,
        })
    }))
}

// ForwardAxis ::= ('child' | 'descendant' | 'attribute' | 'self' | 'descendant-or-self' | 'following-sibling' | 'following' | 'namespace') '::'
fn forwardaxis() -> Box<dyn Fn(ParseInput) -> ParseResult<&'static str>> {
    Box::new(map(
        //    alt8(
        pair(
            alt2(
                axis_child(),
                //	axis_descendant_or_self(),
                //	axis_descendant(),
                //	axis_attribute(),
                axis_self(),
                //	axis_following_sibling(),
                //	axis_following(),
                //	axis_namespace(),
            ),
            tag("::"),
        ),
        |(a, _)| a,
    ))
}

fn axis_child() -> Box<dyn Fn(ParseInput) -> ParseResult<&'static str>> {
    Box::new(map(tag("child"), |c| "child"))
}
fn axis_self() -> Box<dyn Fn(ParseInput) -> ParseResult<&'static str>> {
    Box::new(map(tag("self"), |c| "self"))
}

// NodeTest ::= KindTest | NameTest
// NameTest ::= EQName | Wildcard
fn nodetest() -> Box<dyn Fn(ParseInput) -> ParseResult<NodeTest>> {
    //    Box::new(alt2(kindtest(), nametest()))
    Box::new(nametest())
}

// KindTest ::= DocumentTest | ElementTest | AttributeTest | SchemaElementTest | SchemaAttributeTest | PITest | CommentTest | TextTest | NamespaceNodeTest | AnyKindTest
//fn kindtest() -> Box<dyn Fn(ParseInput) -> ParseResult<NodeTest>> {
//    Box::new(map(tag("not implemented"), |_| not_implemented))
//}

// NameTest ::= EQName | Wildcard
// TODO: allow EQName rather than QName
fn nametest() -> Box<dyn Fn(ParseInput) -> ParseResult<NodeTest>> {
    Box::new(alt2(qname(), wildcard()))
}

// Wildcard ::= '*' | (NCName ':*') | ('*:' NCName) | (BracedURILiteral '*')
// TODO: more specific wildcards
fn wildcard() -> Box<dyn Fn(ParseInput) -> ParseResult<NodeTest>> {
    Box::new(map(tag("*"), |_| {
        NodeTest::Name(NameTest {
            ns: Some(WildcardOrName::Wildcard),
            prefix: None,
            name: Some(WildcardOrName::Wildcard),
        })
    }))
}

fn qname() -> Box<dyn Fn(ParseInput) -> ParseResult<NodeTest>> {
    Box::new(alt2(prefixed_name(), unprefixed_name()))
}
fn unprefixed_name() -> Box<dyn Fn(ParseInput) -> ParseResult<NodeTest>> {
    Box::new(map(ncname(), |localpart| {
        NodeTest::Name(NameTest {
            ns: None,
            prefix: None,
            name: Some(WildcardOrName::Name(String::from(localpart))),
        })
    }))
}
fn prefixed_name() -> Box<dyn Fn(ParseInput) -> ParseResult<NodeTest>> {
    Box::new(map(
        tuple3(ncname(), tag(":"), ncname()),
        |(prefix, _, localpart)| {
            NodeTest::Name(NameTest {
                ns: None,
                prefix: Some(String::from(prefix)),
                name: Some(WildcardOrName::Name(String::from(localpart))),
            })
        },
    ))
}

fn xpwhitespace() -> Box<dyn Fn(ParseInput) -> ParseResult<()>> {
    Box::new(inspect(
        "xpwhitespace",
        map(
            tuple3(
                whitespace0(),
                take_until_balanced("(:", ":)"),
                whitespace0(),
            ),
            |_| (),
        ),
    ))
}
/*
fn xpwhitespace_wrapper(
    b: bool,
) -> impl Fn(ParseInput) -> ParseResult<()>
{
    move |input| {
        if b {
            xpwhitespace()(input)
        } else {
            Ok((input, ()))
        }
    }
}
 */

/// Parse nested input.
///
/// Inspired by 'take_until_unbalanced' from parse_hyperlinks crate.
/// We can't use the parse_hyperlinks version since it only takes character delimiters.
/// Also, this function does not need to consider escaped brackets.
///
/// This function consumes the delimiters.
/// The start delimiter must be the first token in the input. Finding this sets the bracket count to 1. After that there are 4 scenarios:
///
/// * The close delimiter is not found. This is an error.
/// * There is no open delimiter. In this case, consume up to and including the close delimiter. If the bracket count is 1 then return Ok, otherwise error.
/// * There is an open delimiter. If the open occurs after the close, then consume up to and including the close delimiter. If the bracket count is 1 then return Ok, otherwise error.
/// * The open delimiter occurs before the close. In this case, increment the bracket count and continue after the open delimiter.
fn take_until_balanced(
    open: &'static str,
    close: &'static str,
) -> Box<dyn Fn(ParseInput) -> ParseResult<()>> {
    Box::new(move |mut input: ParseInput| {
        let mut counter = 0;
        let mut bracket_counter = 0;

        // Assume the open and close phrases are the same length
        loop {
//            eprintln!(
//                "TUB: looking for open in \"{}\", bc=={}",
//                input, bracket_counter
//            );
            counter += 1;
            if counter > 1000 {
//                eprintln!("TUB: too many loops");
                return Err(ParseError::Unknown { row: 0, col: 0 });
            }
            match (input.as_str().find(open), input.as_str().find(close)) {
                (Some(0), _) => {
//                    eprintln!("TUB: found open, bracket counter=={}", bracket_counter);
                    bracket_counter += 1;
                    let _: Vec<_> = (&mut input).take(open.len()).collect();
//                    eprintln!("TUB: input now \"{}\"", input);
                    match (input.as_str().find(&open), input.as_str().find(&close)) {
                        (_, None) => {
                            // Scenario 1
//                            eprintln!("TUB: scenario 1");
                            return Err(ParseError::Unbalanced);
                        }
                        (Some(o), Some(c)) => {
                            // Scenario 3/4
                            if o > c {
                                // Scenario 3
//                                eprintln!("TUB: scenario 3");
                                if bracket_counter == 1 {
                                    let _: Vec<_> = (&mut input).take(c + close.len()).collect();
//                                    eprintln!("TUB: returning, input now \"{}\"", input);
                                    return Ok((input, ()));
                                } else {
                                    return Err(ParseError::Unbalanced);
                                }
                            } else {
                                // Scenario 4
//                                eprintln!("TUB: scenario 4");
                                bracket_counter += 1;
                                let _: Vec<_> = (&mut input).take(o + open.len()).collect();
//                                eprintln!("TUB: input now \"{}\"", input);
                            }
                        }
                        (_, Some(c)) => {
                            // Scenario 2
//                            eprintln!("TUB: scenario 2");
                            match bracket_counter.cmp(&1) {
                                Ordering::Greater => {
                                    bracket_counter -= 1;
                                    let _: Vec<_> = (&mut input).take(c + close.len()).collect();
                                }
                                Ordering::Equal => {
                                    let _: Vec<_> = (&mut input).take(c + close.len()).collect();
//                                    eprintln!("TUB: returning, input now \"{}\"", input);
                                    return Ok((input, ()));
                                }
                                Ordering::Less => {
                                    return Err(ParseError::Unbalanced);
                                }
                            }
                        }
                        _ => {
//                            eprintln!("TUB: unhandled scenario")
                        }
                    }
                }
                (None, Some(c)) => {
                    // Scenario 2
//                    eprintln!("TUB: scenario 2/2");
                    match bracket_counter.cmp(&1) {
                        Ordering::Greater => {
                            bracket_counter -= 1;
                            let _: Vec<_> = (&mut input).take(c + close.len()).collect();
                        }
                        Ordering::Equal => {
                            let _: Vec<_> = (&mut input).take(c + close.len()).collect();
//                            eprintln!("TUB: returning, input now \"{}\"", input);
                            return Ok((input, ()));
                        }
                        Ordering::Less => {
                            return Err(ParseError::Unbalanced);
                        }
                    }
                }
                _ => return Ok((input, ())),
            }
        }
    })
}

fn noop<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Combinator<'a, N>> + 'a> {
    Box::new(move |_| Err(ParseError::Combinator))
}
