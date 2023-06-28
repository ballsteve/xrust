//! # xrust::xpath
//!
//! An XPath parser using the xrust parser combinator that produces a xrust transformation combinator.

use std::rc::Rc;

use crate::evaluate::{
    ArithmeticOperator, Axis, KindTest, NameTest, NodeMatch, NodeTest, WildcardOrName,
};
use crate::item::{Item, Node};
use crate::parsecommon::*;
use crate::transcomb::{
    compose, context, empty, general_comparison, literal as tc_literal, root, step, tc_and,
    tc_concat, tc_or, tc_range, tc_sequence, value_comparison, Combinator, Context, TransResult,
};
use crate::value::Value;
use crate::value::*;
use crate::xdmerror::*;
use rust_decimal::Decimal;
use std::cmp::Ordering;
use std::str::FromStr;

use crate::parser::combinators::alt::{alt2, alt3};
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::list::separated_list1;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::pair::pair;
use crate::parser::combinators::tag::{anychar, tag};
use crate::parser::combinators::tuple::{tuple2, tuple3, tuple4};
use crate::parser::combinators::whitespace::whitespace0;
use crate::parser::common::ncname;
use crate::parser::{ParseError, ParseInput, ParseResult};

pub fn parse<'a, N: Node>(e: &'a str) -> Result<Combinator<'a, N>, Error> {
    if e == "" {
        Ok(empty())
    } else {
        let input = ParseInput::new(e);
        match expr(input) {
            Ok((mut rem, f)) => {
                eprintln!("parse: remainder == {}", rem);
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
fn expr<'a, N: Node + 'a>(input: ParseInput) -> ParseResult<Combinator<'a, N>> {
    map(
        separated_list1(
            map(tuple3(whitespace0(), tag(","), whitespace0()), |_| ()),
            expr_single(),
        ),
        |v| tc_sequence(v),
    )(input)
}

// ExprSingle ::= ForExpr | LetExpr | QuantifiedExpr | IfExpr | OrExpr
fn expr_single<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    alt3(or_expr(), absolutepath_expr(), relativepath_expr())
    //    alt4(
    //	or_expr(),
    //	let_expr(),
    //	for_expr(),
    //	if_expr(),
    //    )
}

// OrExpr ::= AndExpr ('or' AndExpr)*
fn or_expr<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    map(
        separated_list1(
            map(tuple3(whitespace0(), tag("or"), whitespace0()), |_| ()),
            and_expr(),
        ),
        |v| {
            if v.len() == 1 {
                // TODO: This is inefficient, but Rust is not allowing v[0]
                tc_sequence(v)
            } else {
                tc_or(v)
            }
        },
    )
}

// AndExpr ::= ComparisonExpr ('and' ComparisonExpr)*
fn and_expr<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    map(
        separated_list1(
            map(tuple3(whitespace0(), tag("and"), whitespace0()), |_| ()),
            comparison_expr(),
        ),
        |v| {
            if v.len() == 1 {
                // TODO: This is inefficient, but Rust is not allowing v[0]
                tc_sequence(v)
            } else {
                tc_and(v)
            }
        },
    )
}

// ComparisonExpr ::= StringConcatExpr ( (ValueComp | GeneralComp | NodeComp) StringConcatExpr)?
fn comparison_expr<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    map(
        pair(
            stringconcat_expr(),
            opt(pair(
                tuple3(
                    whitespace0(),
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
                    whitespace0(),
                ),
                stringconcat_expr(),
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
    )
}

// StringConcatExpr ::= RangeExpr ( '||' RangeExpr)*
fn stringconcat_expr<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    map(
        separated_list1(
            map(tuple3(whitespace0(), tag("||"), whitespace0()), |_| ()),
            range_expr(),
        ),
        |v| {
            if v.len() == 1 {
                // TODO: rust doesn't like v[0], see above
                tc_sequence(v)
            } else {
                tc_concat(v)
            }
        },
    )
}

// RangeExpr ::= AdditiveExpr ( 'to' AdditiveExpr)?
fn range_expr<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    map(
        pair(
            additive_expr(),
            opt(tuple2(
                tuple3(whitespace0(), tag("to"), whitespace0()),
                additive_expr(),
            )),
        ),
        |(v, o)| match o {
            None => v,
            Some((_, u)) => tc_range(v, u),
        },
    )
}

// AdditiveExpr ::= MultiplicativeExpr ( ('+' | '-') MultiplicativeExpr)*
fn additive_expr<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    // just a paths and literals for now
    alt3(
        absolutedescendant_expr(),
        absolutepath_expr(),
        relativepath_expr(),
    )
}

// ('//' RelativePathExpr?)
fn absolutedescendant_expr<'a, N: Node + 'a>(
) -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    map(pair(tag("//"), relativepath_expr()), |(_, r)| {
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
    })
}

// ('/' RelativePathExpr?)
fn absolutepath_expr<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    map(pair(tag("/"), opt(relativepath_expr())), |(_, r)| match r {
        Some(a) => compose(vec![root(), a]),
        None => root(),
    })
}

// RelativePathExpr ::= StepExpr (('/' | '//') StepExpr)*
fn relativepath_expr<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    map(
        pair(
            step_expr(),
            //	    many0(tuple2(
            //		alt(
            //		    tuple3(whitespace0(), tag("//"), whitespace0()),
            //		    tuple3(whitespace0(), tag("/"), whitespace0()),
            //		)
            //	    ))
            many0(tuple2(
                tuple3(whitespace0(), tag("/"), whitespace0()),
                step_expr(),
            )),
        ),
        |(a, b)| {
            if b.is_empty() {
                a
            } else {
                let mut r = Vec::new();
                r.push(a);
                for (_, c) in b {
                    r.push(c)
                }
                compose(r)
            }
        },
    )
}

// StepExpr ::= PostfixExpr | AxisStep
fn step_expr<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    alt2(postfix_expr(), axisstep())
}

// PostfixExpr ::= PrimaryExpr (Predicate | ArgumentList | Lookup)*
// TODO: predicates, arg list, lookup
fn postfix_expr<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    primary_expr()
}

// PrimaryExpr ::= Literal | VarRef | ParenthesizedExpr | ContextItemExpr | FunctionCall | FunctionItemExpr | MapConstructor | ArrayConstructor | UnaryLookup
// TODO: finish this parser
fn primary_expr<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    alt2(
        literal(),
        context_item(),
        //	parenthesized_expr(),
        //	function_call(),
        //	variable_reference(),
    )
}

// ContextItemExpr ::= '.'
fn context_item<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    map(tag("."), |_| context())
}

// Literal ::= NumericLiteral | StringLiteral
fn literal<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    alt2(numeric_literal(), string_literal())
}

// NumericLiteral ::= IntegerLiteral | DecimalLiteral | DoubleLiteral
fn numeric_literal<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    alt3(double_literal(), decimal_literal(), integer_literal())
}
// IntegerLiteral ::= Digits
fn integer_literal<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    map(digit1(), |s: String| {
        let n = s.parse::<i64>().unwrap();
        tc_literal(Rc::new(Item::Value(Value::Integer(n))))
    })
}
// DecimalLiteral ::= ('.' Digits) | (Digits '.' [0-9]*)
// Construct a double, but if that fails fall back to decimal
fn decimal_literal<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    alt2(decimal_literal_frac(), decimal_literal_comp())
}
fn decimal_literal_frac<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>>
{
    map(pair(tag("."), digit1()), |(_, mut f)| {
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
    })
}
fn decimal_literal_comp<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>>
{
    map(tuple3(digit1(), tag("."), digit0()), |(w, _, f)| {
        let s = format!("{}.{}", w, f);
        let n = s.parse::<f64>();
        let i = match n {
            Ok(m) => Value::Double(m),
            Err(_) => Value::Decimal(Decimal::from_str(&s).unwrap()),
        };
        tc_literal(Rc::new(Item::Value(i)))
    })
}

// DoubleLiteral ::= (('.' Digits) | (Digits ('.' [0-9]*)?)) [eE] [+-]? Digits
// Construct a double
fn double_literal<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    alt2(double_literal_frac(), double_literal_comp())
}

fn double_literal_frac<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>>
{
    map(
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
    )
}
fn double_literal_comp<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>>
{
    map(
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
    )
}

// StringLiteral ::= double- or single-quote delimited with double-delimiter escape
fn string_literal_double<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>>
{
    map(
        delimited(
            anychar('"'),
            map(many0(alt2(map(tag("\"\""), |_| '"'), none_of("\""))), |v| {
                v.iter().collect::<String>()
            }),
            anychar('"'),
        ),
        |s| tc_literal(Rc::new(Item::Value(Value::from(s)))),
    )
}
fn string_literal_single<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>>
{
    map(
        delimited(
            anychar('\''),
            map(many0(alt2(map(tag("''"), |_| '\''), none_of("'"))), |v| {
                v.iter().collect::<String>()
            }),
            anychar('\''),
        ),
        |s| tc_literal(Rc::new(Item::Value(Value::from(s)))),
    )
}
fn string_literal<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    alt2(string_literal_double(), string_literal_single())
}

/// Return zero or more digits from the input stream.
fn digit0() -> impl Fn(ParseInput) -> ParseResult<String> {
    move |mut input| {
        let result = (&mut input)
            .take_while(|c| *c >= '0' && *c <= '9')
            .collect::<String>();
        Ok((input, result))
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
                        let mut result = (&mut input).collect::<String>();
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
fn axisstep<'a, N: Node + 'a>() -> impl Fn(ParseInput) -> ParseResult<Combinator<'a, N>> {
    map(pair(forwardaxis(), nodetest()), |(a, n)| {
        step(NodeMatch {
            axis: Axis::from(a),
            nodetest: n,
        })
    })
}

// ForwardAxis ::= ('child' | 'descendant' | 'attribute' | 'self' | 'descendant-or-self' | 'following-sibling' | 'following' | 'namespace') '::'
fn forwardaxis() -> impl Fn(ParseInput) -> ParseResult<&'static str> {
    map(
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
    )
}

fn axis_child() -> impl Fn(ParseInput) -> ParseResult<&'static str> {
    map(tag("child"), |c| {
        eprintln!("found child axis");
        "child"
    })
}
fn axis_self() -> impl Fn(ParseInput) -> ParseResult<&'static str> {
    map(tag("self"), |c| "self")
}

// NodeTest ::= KindTest | NameTest
// NameTest ::= EQName | Wildcard
fn nodetest() -> impl Fn(ParseInput) -> ParseResult<NodeTest> {
    // shortcut: just unprefixed_name
    map(ncname(), |n| {
        NodeTest::Name(NameTest {
            ns: None,
            prefix: None,
            name: Some(WildcardOrName::Name(String::from(n))),
        })
    })
}

fn noop() {
    //    literal(Rc::new(Item::Value(Value::from("found expression"))))
}
