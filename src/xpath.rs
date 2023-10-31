//! # xrust::xpath
//!
//! An XPath parser using the xrust parser combinator that produces a xrust transformation combinator.

use std::rc::Rc;
use std::cmp::Ordering;
use std::str::FromStr;

use crate::xdmerror::*;
use crate::value::{Value, Operator};
use crate::item::Node;
use crate::transform::{Transform,
                       NodeMatch, NodeTest,
                       NameTest, WildcardOrName,
                       ArithmeticOperator, ArithmeticOperand,
Axis};
use rust_decimal::Decimal;
use crate::Item;

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

pub fn expression<N: Node>(e: &str) -> Result<Transform<N>, Error> {
    if e == "" {
        Ok(Transform::Empty)
    } else {
        let input = ParseInput::new(e);
        match xpath_expr::<N>(input) {
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
fn xpath_expr<N: Node>(input: ParseInput) -> ParseResult<Transform<N>> {
    expr::<N>()(input)
}

// Implementation note: cannot use opaque type because XPath expressions are recursive, and Rust *really* doesn't like recursive opaque types. Dynamic trait objects aren't ideal, but compiling XPath expressions is a one-off operation so that shouldn't cause a major performance issue.
// Implementation note 2: since XPath is recursive, must lazily evaluate arguments to avoid stack overflow.
fn expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        separated_list1(
            map(tuple3(xpwhitespace(), tag(","), xpwhitespace()), |_| ()),
            expr_single::<N>(),
        ),
        |v| Transform::SequenceItems(v),
    ))
}

fn expr_wrapper<N: Node>(
    b: bool,
) -> impl Fn(ParseInput) -> ParseResult<Transform<N>> {
    move |input| {
        if b {
            expr::<N>()(input)
        } else {
            noop::<N>()(input)
        }
    }
}

// ExprSingle ::= ForExpr | LetExpr | QuantifiedExpr | IfExpr | OrExpr
fn expr_single<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(inspect(
        "expr_single",
        alt4(
            or_expr::<N>(),
            let_expr::<N>(),
            for_expr::<N>(),
            if_expr::<N>(),
        ),
    ))
}

fn expr_single_wrapper<N: Node>(
    b: bool,
) -> impl Fn(ParseInput) -> ParseResult<Transform<N>>
{
    move |input| {
        if b {
            expr_single::<N>()(input)
        } else {
            noop::<N>()(input)
        }
    }
}

// IfExpr ::= 'if' '(' Expr ')' 'then' ExprSingle 'else' ExprSingle
fn if_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
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
        |((_, _, _, _, i, _, _, _, _, _), (t, _, _, _, e))| Transform::Switch(vec![(i, t)], Box::new(e)),
    ))
}

// ForExpr ::= SimpleForClause 'return' ExprSingle
fn for_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
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
fn simple_for_clause<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Vec<(String, Transform<N>)>> + 'a> {
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
fn let_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>>+ 'a> {
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
fn simple_let_clause<'a, N: Node + 'a>()-> Box<dyn Fn(ParseInput) -> ParseResult<Vec<(String, Transform<N>)>> + 'a> {
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
                        expr_single_wrapper::<N>(true),
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
fn or_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
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
fn and_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
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

// ComparisonExpr ::= StringConcatExpr ( (ValueComp | GeneralComp | NodeComp) StringConcatExpr)?
fn comparison_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        pair(
            stringconcat_expr::<N>(),
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
                stringconcat_expr::<N>(),
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

// StringConcatExpr ::= RangeExpr ( '||' RangeExpr)*
fn stringconcat_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        inspect(
            "stringconcat",
            separated_list1(
                map(tuple3(xpwhitespace(), tag("||"), xpwhitespace()), |_| ()),
                range_expr::<N>(),
            ),
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

// RangeExpr ::= AdditiveExpr ( 'to' AdditiveExpr)?
fn range_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
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
                    map(tuple3(xpwhitespace(), map(tag("+"), |_| ArithmeticOperator::Add), xpwhitespace()), |(_, x, _)| x),
                    map(tuple3(xpwhitespace(), map(tag("-"), |_| ArithmeticOperator::Subtract), xpwhitespace()), |(_, x, _)| x),
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
                let mut e: Vec<ArithmeticOperand<N>> = b.iter().map(|(c, d)| ArithmeticOperand::new(c.clone(), Transform::Arithmetic(d.clone()))).collect();
                a.append(&mut e);

                Transform::Arithmetic(a)
            }
        },
    ))
}

// MultiplicativeExpr ::= UnionExpr ( ('*' | 'div' | 'idiv' | 'mod') UnionExpr)*
fn multiplicative_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Vec<ArithmeticOperand<N>>> + 'a> {
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

// UnionExpr ::= IntersectExceptExpr ( ('union' | '|') IntersectExceptExpr)*
fn union_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        inspect(
            "union",
            separated_list1(
                map(
                    tuple3(xpwhitespace(), alt2(tag("union"), tag("|")), xpwhitespace()),
                    |_| (),
                ),
                intersectexcept_expr::<N>(),
            ),
        ),
        |mut v| {
            if v.len() == 1 {
                v.pop().unwrap()
            } else {
                Transform::NotImplemented("union_expr".to_string())
            }
        },
    ))
}

// IntersectExceptExpr ::= InstanceOfExpr ( ('intersect' | 'except') InstanceOfExpr)*
fn intersectexcept_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        pair(
            instanceof_expr::<N>(),
            many0(tuple2(
                tuple3(
                    xpwhitespace(),
                    alt2(tag("intersect"), tag("except")),
                    xpwhitespace(),
                ),
                instanceof_expr::<N>(),
            )),
        ),
        |(v, o)| {
            if o.is_empty() {
                v
            } else {
                Transform::NotImplemented("intersectexcept_expr".to_string())
            }
        },
    ))
}

// InstanceOfExpr ::= TreatExpr ( 'instance' 'of' SequenceType)?
fn instanceof_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        pair(
            treat_expr::<N>(),
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
                Transform::NotImplemented("instanceof_expr".to_string())
            }
        },
    ))
}

// SequenceType ::= ( 'empty-sequence' '(' ')' | (ItemType OccurrenceIndicator?)
// TODO: implement this parser fully
fn sequencetype_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(tag("empty-sequence()"), |_| {
        Transform::NotImplemented("sequencetype_expr".to_string())
    }))
}

// TreatExpr ::= CastableExpr ( 'treat' 'as' SequenceType)?
fn treat_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        pair(
            castable_expr::<N>(),
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
                Transform::NotImplemented("treat_expr".to_string())
            }
        },
    ))
}

// CastableExpr ::= CastExpr ( 'castable' 'as' SingleType)?
fn castable_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        pair(
            cast_expr::<N>(),
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
                Transform::NotImplemented("castable_expr".to_string())
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
// TODO: implement this parser fully
fn singletype_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(pair(qname(), tag("?")), |_| {
        Transform::NotImplemented("singletype_expr".to_string())
    }))
}

// CastExpr ::= ArrowExpr ( 'cast' 'as' SingleType)?
fn cast_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        pair(
            arrow_expr::<N>(),
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
                Transform::NotImplemented("cast_expr".to_string())
            }
        },
    ))
}

// ArrowExpr ::= UnaryExpr ( '=>' ArrowFunctionSpecifier ArgumentList)*
fn arrow_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        pair(
            unary_expr::<N>(),
            opt(tuple6(
                xpwhitespace(),
                tag("=>"),
                xpwhitespace(),
                arrowfunctionspecifier::<N>(),
                xpwhitespace(),
                opt(argumentlist::<N>()),
            )),
        ),
        |(v, o)| {
            if o.is_none() {
                v
            } else {
                Transform::NotImplemented("arrow_expr".to_string())
            }
        },
    ))
}

// ArrowFunctionSpecifier ::= EQName | VarRef | ParenthesizedExpr
// TODO: finish this parser with EQName and VarRef
fn arrowfunctionspecifier<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        alt2(qname_expr(), parenthesized_expr::<N>()),
        |_| Transform::NotImplemented("arrowfunctionspecifier".to_string()),
    ))
}
fn qname_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(qname(), |q| match q {
        NodeTest::Name(NameTest {
            name: Some(WildcardOrName::Name(localpart)),
            ns: None,
            prefix: None,
        }) => Transform::Literal(Rc::new(Item::Value(Value::from(localpart)))),
        _ => Transform::Literal(Rc::new(Item::Value(Value::from("invalid qname")))),
    }))
}

// ArgumentList ::= '(' (Argument (',' Argument)*)? ')'
// TODO: finish this parser with actual arguments
fn argumentlist<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(tag("()"), |_| {
        Transform::NotImplemented("argumentlist".to_string())
    }))
}

// UnaryExpr ::= ('-' | '+')* ValueExpr
fn unary_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
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
    Box::new(inspect(
        "value_expr",
        map(
            pair(
                path_expr::<N>(),
                many0(tuple2(tag("!"), path_expr::<N>())),
            ),
            |(u, v)| {
                if v.is_empty() {
                    u
                } else {
                    Transform::NotImplemented("value_expr".to_string())
                }
            },
        ),
    ))
}

// PathExpr ::= ('/' RelativePathExpr?) | ('//' RelativePathExpr) | RelativePathExpr
fn path_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(alt3(
        absolutedescendant_expr::<N>(),
        absolutepath_expr::<N>(),
        relativepath_expr::<N>(),
    ))
}

// ('//' RelativePathExpr?)
fn absolutedescendant_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        pair(tag("//"), relativepath_expr::<N>()),
        |(_, r)| {
            Transform::Compose(vec![
                Transform::Step(NodeMatch {
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
fn absolutepath_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        pair(tag("/"), opt(relativepath_expr::<N>())),
        |(_, r)| match r {
            Some(a) => Transform::Compose(vec![Transform::Root, a]),
            None => Transform::Root,
        },
    ))
}

// RelativePathExpr ::= StepExpr (('/' | '//') StepExpr)*
fn relativepath_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        pair(
            step_expr::<N>(),
            many0(tuple2(
                alt2(
                    map(tuple3(xpwhitespace(), tag("//"), xpwhitespace()), |_| "//"),
                    map(tuple3(xpwhitespace(), tag("/"), xpwhitespace()), |_| "/"),
                ),
                step_expr::<N>(),
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
                            r.push(Transform::Step(NodeMatch {
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
                Transform::Compose(r)
            }
        },
    ))
}

// StepExpr ::= PostfixExpr | AxisStep
fn step_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(alt2(postfix_expr::<N>(), axisstep::<N>()))
}

// PostfixExpr ::= PrimaryExpr (Predicate | ArgumentList | Lookup)*
// TODO: predicates, arg list, lookup
fn postfix_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(primary_expr::<N>())
}

// PrimaryExpr ::= Literal | VarRef | ParenthesizedExpr | ContextItemExpr | FunctionCall | FunctionItemExpr | MapConstructor | ArrayConstructor | UnaryLookup
// TODO: finish this parser
fn primary_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(inspect(
        "primary_expr",
        alt5(
            literal::<N>(),
            context_item::<N>(),
            parenthesized_expr::<N>(),
            function_call::<N>(),
            variable_reference::<N>(),
        ),
    ))
}

// FunctionCall ::= EQName ArgumentList
fn function_call<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        pair(qname(), arglist::<N>()),
        |(qn, mut a)| match qn {
            NodeTest::Name(NameTest {
                name: Some(WildcardOrName::Name(localpart)),
                ns: None,
                prefix: None,
            }) => match localpart.as_str() {
                "position" => Transform::Position,
                "last" => Transform::Last,
                "count" => {
                    if a.len() == 0 {
                        Transform::Count(None)
                    } else if a.len() == 1 {
                        Transform::Count(Some(Box::new(a.pop().unwrap())))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "local-name" => {
                    if a.len() == 0 {
                        Transform::LocalName(None)
                    } else if a.len() == 1 {
                        Transform::LocalName(Some(Box::new(a.pop().unwrap())))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "name" => {
                    if a.len() == 0 {
                        Transform::Name(None)
                    } else if a.len() == 1 {
                        Transform::Name(Some(Box::new(a.pop().unwrap())))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "string" => {
                    if a.len() == 1 {
                        Transform::String(Box::new(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "concat" => Transform::Concat(a),
                "starts-with" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        Transform::StartsWith(Box::new(c), Box::new(b))
                    } else {
                        // Incorrect arguments
                        Transform::Error(ErrorKind::ParseError, String::from("incorrect arguments"))
                    }
                }
                "contains" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        Transform::Contains(Box::new(c), Box::new(b))
                    } else {
                        // Incorrect arguments
                        Transform::Error(ErrorKind::ParseError, String::from("incorrect arguments"))
                    }
                }
                "substring" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        Transform::Substring(Box::new(c), Box::new(b), None)
                    } else if a.len() == 3 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        let d = a.pop().unwrap();
                        Transform::Substring(Box::new(d), Box::new(c), Some(Box::new(b)))
                    } else {
                        // Wrong number of arguments
                        Transform::Error(ErrorKind::ParseError, String::from("wrong number of arguments"))
                    }
                }
                "substring-before" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        Transform::SubstringBefore(Box::new(c), Box::new(b))
                    } else {
                        // Incorrect arguments
                        Transform::Error(ErrorKind::ParseError, String::from("incorrect arguments"))
                    }
                }
                "substring-after" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        Transform::SubstringAfter(Box::new(c), Box::new(b))
                    } else {
                        // Incorrect arguments
                        Transform::Error(ErrorKind::ParseError, String::from("incorrect arguments"))
                    }
                }
                "normalize-space" => {
                    if a.len() == 0 {
                        Transform::NormalizeSpace(None)
                    } else if a.len() == 1 {
                        Transform::NormalizeSpace(Some(Box::new(a.pop().unwrap())))
                    } else {
                        // Wrong number of arguments
                        Transform::Error(ErrorKind::ParseError, String::from("wrong number of arguments"))
                    }
                }
                "translate" => {
                    if a.len() == 3 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        let d = a.pop().unwrap();
                        Transform::Translate(Box::new(d), Box::new(c), Box::new(b))
                    } else {
                        // Wrong number of arguments
                        Transform::Error(ErrorKind::ParseError, String::from("wrong number of arguments"))
                    }
                }
                "boolean" => {
                    if a.len() == 1 {
                        Transform::Boolean(Box::new(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "not" => {
                    if a.len() == 1 {
                        Transform::Not(Box::new(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "true" => {
                    if a.len() == 0 {
                        Transform::True
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "false" => {
                    if a.len() == 0 {
                        Transform::False
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "number" => {
                    if a.len() == 1 {
                        Transform::Number(Box::new(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "sum" => {
                    if a.len() == 1 {
                        Transform::Sum(Box::new(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "floor" => {
                    if a.len() == 1 {
                        Transform::Floor(Box::new(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "ceiling" => {
                    if a.len() == 1 {
                        Transform::Ceiling(Box::new(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "round" => {
                    if a.len() == 1 {
                        let b = a.pop().unwrap();
                        Transform::Round(Box::new(b), None)
                    } else if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        Transform::Round(Box::new(c), Some(Box::new(b)))
                    } else {
                        // Wrong number of arguments
                        Transform::Error(ErrorKind::ParseError, String::from("wrong number of arguments"))
                    }
                }
                "current-date-time" => {
                    if a.len() == 0 {
                        Transform::CurrentDateTime
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "current-date" => {
                    if a.len() == 0 {
                        Transform::CurrentDate
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "current-time" => {
                    if a.len() == 0 {
                        Transform::CurrentTime
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "format-date-time" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        Transform::FormatDateTime(Box::new(c), Box::new(b), None, None, None)
                    } else if a.len() == 5 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        let d = a.pop().unwrap();
                        let e = a.pop().unwrap();
                        let f = a.pop().unwrap();
                        Transform::FormatDateTime(Box::new(f), Box::new(e), Some(Box::new(d)), Some(Box::new(c)), Some(Box::new(b)))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "format-date" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        Transform::FormatDate(Box::new(c), Box::new(b), None, None, None)
                    } else if a.len() == 5 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        let d = a.pop().unwrap();
                        let e = a.pop().unwrap();
                        let f = a.pop().unwrap();
                        Transform::FormatDate(Box::new(f), Box::new(e), Some(Box::new(d)), Some(Box::new(c)), Some(Box::new(b)))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "format-time" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        Transform::FormatTime(Box::new(c), Box::new(b), None, None, None)
                    } else if a.len() == 5 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        let d = a.pop().unwrap();
                        let e = a.pop().unwrap();
                        let f = a.pop().unwrap();
                        Transform::FormatTime(Box::new(f), Box::new(e), Some(Box::new(d)), Some(Box::new(c)), Some(Box::new(b)))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "current-group" => {
                    if a.len() == 0 {
                        Transform::CurrentGroup
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "current-grouping-key" => {
                    if a.len() == 0 {
                        Transform::CurrentGroupingKey
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                _ => Transform::Error(ErrorKind::ParseError, String::from("undefined function")), // TODO: user-defined functions
            },
            _ => Transform::Error(ErrorKind::ParseError, String::from("unknown function")),
        },
    ))
}

// ArgumentList ::= '(' (Argument (',' Argument)*)? ')'
fn arglist<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Vec<Transform<N>>> + 'a> {
    Box::new(map(
        tuple3(
                tag("("),
                separated_list0(
                        map(tuple3(xpwhitespace(), tag(","), xpwhitespace()), |_| ()),
                        argument::<N>(),
                ),
                tag(")"),
        ),
        |(_, a, _)| a,
    ))
}

// Argument ::= ExprSingle | ArgumentPlaceHolder
// TODO: ArgumentPlaceHolder
fn argument<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(inspect("argument", expr_single_wrapper::<N>(true)))
}

// VarRef ::= '$' VarName
pub fn variable_reference<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(pair(tag("$"), qname()), |(_, qn)| {
        Transform::VariableReference(get_nt_localname(&qn))
    }))
}

// ParenthesizedExpr ::= '(' Expr? ')'
fn parenthesized_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(alt2(
        parenthesized_expr_empty::<N>(),
        parenthesized_expr_nonempty::<N>(),
    ))
}
fn parenthesized_expr_empty<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(tag("()"), |_| Transform::Empty))
}
fn parenthesized_expr_nonempty<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(delimited(
        tag("("),
        map(expr_wrapper::<N>(true), |e| e),
        tag(")"),
    ))
}

// ContextItemExpr ::= '.'
fn context_item<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(tag("."), |_| Transform::ContextItem))
}

// Literal ::= NumericLiteral | StringLiteral
pub(crate) fn literal<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(alt2(numeric_literal::<N>(), string_literal::<N>()))
}

// NumericLiteral ::= IntegerLiteral | DecimalLiteral | DoubleLiteral
fn numeric_literal<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(alt3(double_literal::<N>(), decimal_literal::<N>(), integer_literal::<N>()))
}
// IntegerLiteral ::= Digits
fn integer_literal<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(digit1(), |s: String| {
        let n = s.parse::<i64>().unwrap();
        Transform::Literal(Rc::new(Item::Value(Value::Integer(n))))
    }))
}
// DecimalLiteral ::= ('.' Digits) | (Digits '.' [0-9]*)
// Construct a double, but if that fails fall back to decimal
fn decimal_literal<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(alt2(decimal_literal_frac::<N>(), decimal_literal_comp::<N>()))
}
fn decimal_literal_frac<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
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
        Transform::Literal(Rc::new(Item::Value(i)))
    }))
}
fn decimal_literal_comp<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(tuple3(digit1(), tag("."), digit0()), |(w, _, f)| {
            let s = format!("{}.{}", w, f);
            let n = s.parse::<f64>();
            let i = match n {
                Ok(m) => Value::Double(m),
                Err(_) => Value::Decimal(Decimal::from_str(&s).unwrap()),
            };
            Transform::Literal(Rc::new(Item::Value(i)))
        }),
    )
}

// DoubleLiteral ::= (('.' Digits) | (Digits ('.' [0-9]*)?)) [eE] [+-]? Digits
// Construct a double
fn double_literal<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(alt2(double_literal_frac::<N>(), double_literal_comp::<N>()))
}

fn double_literal_frac<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
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
            Transform::Literal(Rc::new(Item::Value(Value::from(i))))
        },
    ))
}
fn double_literal_comp<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
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
            Transform::Literal(Rc::new(Item::Value(Value::from(i))))
        },
    ))
}

// StringLiteral ::= double- or single-quote delimited with double-delimiter escape
fn string_literal_double<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        delimited(
            anychar('"'),
            map(many0(alt2(map(tag("\"\""), |_| '"'), none_of("\""))), |v| {
                v.iter().collect::<String>()
            }),
            anychar('"'),
        ),
        |s| Transform::Literal(Rc::new(Item::Value(Value::from(s))))
    ))
}
fn string_literal_single<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        delimited(
            anychar('\''),
            map(many0(alt2(map(tag("''"), |_| '\''), none_of("'"))), |v| {
                v.iter().collect::<String>()
            }),
            anychar('\''),
        ),
        |s| Transform::Literal(Rc::new(Item::Value(Value::from(s))))
    ))
}
fn string_literal<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(alt2(string_literal_double::<N>(), string_literal_single::<N>()))
}

/// Return zero or more digits from the input stream. Be careful not to consume non-digit input.
fn digit0() -> impl Fn(ParseInput) -> ParseResult<String> {
    move |mut input| {
        match input.clone().position(|c| !(c >= '0' && c <= '9')) {
            Some(0) => {
                Err(ParseError::Combinator)
            }
            Some(pos) => {
                let result = (&mut input).take(pos).collect::<String>();
                Ok((input, result))
            }
            None => {
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
                    let (u, _) = t;
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
fn axisstep<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(pair(forwardaxis(), nodetest()), |(a, n)| {
        Transform::Step(NodeMatch {
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
    Box::new(map(tag("child"), |_| "child"))
}
fn axis_self() -> Box<dyn Fn(ParseInput) -> ParseResult<&'static str>> {
    Box::new(map(tag("self"), |_| "self"))
}

// PredicateList ::= Predicate*
pub fn predicate_list<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(many0(predicate::<N>()), |v| Transform::Compose(v)))
}

// Predicate ::= "[" expr "]"
fn predicate<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        tuple3(
            map(tuple3(xpwhitespace(), tag("["), xpwhitespace()), |_| ()),
            expr_wrapper::<N>(true),
            map(tuple3(xpwhitespace(), tag("]"), xpwhitespace()), |_| ()),
        ),
        |(_, e, _)| Transform::Filter(Box::new(e)),
    ))
}

// NodeTest ::= KindTest | NameTest
// NameTest ::= EQName | Wildcard
pub fn nodetest() -> Box<dyn Fn(ParseInput) -> ParseResult<NodeTest>> {
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

pub fn qname() -> Box<dyn Fn(ParseInput) -> ParseResult<NodeTest>> {
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

pub fn xpwhitespace() -> Box<dyn Fn(ParseInput) -> ParseResult<()>> {
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

/// Parse nested input.
///
/// Inspired by 'take_until_unbalanced' from parse_hyperlinks crate.
/// We can't use the parse_hyperlinks version since it only takes character delimiters.
/// Also, this function does not need to consider escaped brackets.
///
/// This function consumes the delimiters.
/// The start delimiter must be the first token in the input. Finding this sets the bracket count to 1.
/// After that there are 4 scenarios:
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
            counter += 1;
            if counter > 1000 {
                return Err(ParseError::Unknown { row: 0, col: 0 });
            }
            match (input.as_str().find(open), input.as_str().find(close)) {
                (Some(0), _) => {
                    bracket_counter += 1;
                    let _: Vec<_> = (&mut input).take(open.len()).collect();
                    match (input.as_str().find(&open), input.as_str().find(&close)) {
                        (_, None) => {
                            // Scenario 1
                            return Err(ParseError::Unbalanced);
                        }
                        (Some(o), Some(c)) => {
                            // Scenario 3/4
                            if o > c {
                                // Scenario 3
                                if bracket_counter == 1 {
                                    let _: Vec<_> = (&mut input).take(c + close.len()).collect();
                                    return Ok((input, ()));
                                } else {
                                    return Err(ParseError::Unbalanced);
                                }
                            } else {
                                // Scenario 4
                                bracket_counter += 1;
                                let _: Vec<_> = (&mut input).take(o + open.len()).collect();
                            }
                        }
                        (_, Some(c)) => {
                            // Scenario 2
                            match bracket_counter.cmp(&1) {
                                Ordering::Greater => {
                                    bracket_counter -= 1;
                                    let _: Vec<_> = (&mut input).take(c + close.len()).collect();
                                }
                                Ordering::Equal => {
                                    let _: Vec<_> = (&mut input).take(c + close.len()).collect();
                                    return Ok((input, ()));
                                }
                                Ordering::Less => {
                                    return Err(ParseError::Unbalanced);
                                }
                            }
                        }
                    }
                }
                (None, Some(c)) => {
                    // Scenario 2
                    match bracket_counter.cmp(&1) {
                        Ordering::Greater => {
                            bracket_counter -= 1;
                            let _: Vec<_> = (&mut input).take(c + close.len()).collect();
                        }
                        Ordering::Equal => {
                            let _: Vec<_> = (&mut input).take(c + close.len()).collect();
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

fn noop<N: Node>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>>> {
    Box::new(move |_| Err(ParseError::Combinator))
}
