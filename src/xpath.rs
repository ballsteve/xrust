//! # xrust::xpath
//!
//! An XPath parser as a nom parser combinator.

extern crate nom;
use crate::evaluate::{
    ArithmeticOperand, ArithmeticOperator, Axis, Constructor, Function, KindTest, NameTest,
    NodeMatch, NodeTest, WildcardOrName,
};
use crate::item::Node;
use crate::parsecommon::*;
use crate::value::*;
use crate::xdmerror::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::*,
    character::complete::{char, none_of},
    combinator::{complete, map, opt, recognize},
    error::{Error as NomError, ErrorKind as NomErrorKind},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, tuple},
    Err as NomErr, IResult,
};
use rust_decimal::Decimal;
use std::str::FromStr;

// Expr ::= ExprSingle (',' ExprSingle)* ;
// we need to unpack each primary_expr
fn expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        separated_list1(tuple((xpwhitespace, tag(","), xpwhitespace)), expr_single),
        |v| {
            let mut s = Vec::new();
            for i in v {
                for j in i {
                    s.push(j)
                }
            }
            s
        },
    )(input)
}

// ExprSingle ::= ForExpr | LetExpr | QuantifiedExpr | IfExpr | OrExpr
fn expr_single<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    alt((
        or_expr, let_expr, for_expr, if_expr,
        // TODO: other branches
    ))(input)
}

// IfExpr ::= 'if' '(' Expr ')' 'then' ExprSingle 'else' ExprSingle
fn if_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        tuple((
            tag("if"),
            xpwhitespace,
            tag("("),
            xpwhitespace,
            expr,
            xpwhitespace,
            tag(")"),
            xpwhitespace,
            tag("then"),
            xpwhitespace,
            expr_single,
            xpwhitespace,
            tag("else"),
            xpwhitespace,
            expr_single,
        )),
        |(_, _, _, _, i, _, _, _, _, _, t, _, _, _, e)| vec![Constructor::Switch(vec![i, t], e)],
    )(input)
}

// ForExpr ::= SimpleForClause 'return' ExprSingle
fn for_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        tuple((
            simple_for_clause,
            tuple((xpwhitespace, tag("return"), xpwhitespace)),
            expr_single,
        )),
        |(f, _, e)| vec![Constructor::Loop(f, e)],
    )(input)
}

// SimpleForClause ::= 'for' SimpleForBinding (',' SimpleForBinding)*
// SimpleForBinding ::= '$' VarName 'in' ExprSingle
fn simple_for_clause<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        tuple((
            tag("for"),
            xpwhitespace,
            separated_list1(
                tuple((xpwhitespace, tag(","), xpwhitespace)),
                tuple((
                    tag("$"),
                    qname,
                    xpwhitespace,
                    tag("in"),
                    xpwhitespace,
                    expr_single,
                )),
            ),
        )),
        |(_, _, b)| {
            b.iter()
                .map(|(_, v, _, _, _, e)| {
                    Constructor::VariableDeclaration(get_nt_localname(v), e.to_vec())
                })
                .collect()
        },
    )(input)
}

// LetExpr ::= SimpleLetClause 'return' ExprSingle
fn let_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        tuple((
            simple_let_clause,
            tuple((xpwhitespace, tag("return"), xpwhitespace)),
            expr_single,
        )),
        |(mut l, _, mut e)| {
            // Variable declaration
            // expression
            l.append(&mut e);
            l
        },
    )(input)
}

// SimpleLetClause ::= 'let' SimpleLetBinding (',' SimpleLetBinding)*
// SimpleLetBinding ::= '$' VarName ':=' ExprSingle
fn simple_let_clause<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        tuple((
            tag("let"),
            xpwhitespace,
            separated_list1(
                tuple((xpwhitespace, tag(","), xpwhitespace)),
                tuple((
                    tag("$"),
                    qname,
                    xpwhitespace,
                    tag(":="),
                    xpwhitespace,
                    expr_single,
                )),
            ),
        )),
        |(_, _, b)| {
            b.iter()
                .map(|(_, v, _, _, _, e)| {
                    Constructor::VariableDeclaration(get_nt_localname(v), e.to_vec())
                })
                .collect()
        },
    )(input)
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
fn or_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        separated_list1(tuple((xpwhitespace, tag("or"), xpwhitespace)), and_expr),
        |v: Vec<Vec<Constructor<N>>>| {
            if v.len() == 1 {
                let mut s = Vec::new();
                for i in v {
                    for j in i {
                        s.push(j)
                    }
                }
                s
            } else {
                vec![Constructor::Or(v)]
            }
        },
    )(input)
}

// AndExpr ::= ComparisonExpr ('and' ComparisonExpr)*
fn and_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        separated_list1(
            tuple((xpwhitespace, tag("and"), xpwhitespace)),
            comparison_expr,
        ),
        |v: Vec<Vec<Constructor<N>>>| {
            if v.len() == 1 {
                let mut s = Vec::new();
                for i in v {
                    for j in i {
                        s.push(j)
                    }
                }
                s
            } else {
                vec![Constructor::And(v)]
            }
        },
    )(input)
}

// ComparisonExpr ::= StringConcatExpr ( (ValueComp | GeneralComp | NodeComp) StringConcatExpr)?
fn comparison_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        pair(
            stringconcat_expr,
            opt(pair(
                alt((
                    tuple((xpwhitespace, tag("="), xpwhitespace)),
                    tuple((xpwhitespace, tag("!="), xpwhitespace)),
                    tuple((xpwhitespace, tag("<"), xpwhitespace)),
                    tuple((xpwhitespace, tag("<="), xpwhitespace)),
                    tuple((xpwhitespace, tag(">"), xpwhitespace)),
                    tuple((xpwhitespace, tag(">="), xpwhitespace)),
                    tuple((xpwhitespace, tag("eq"), xpwhitespace)),
                    tuple((xpwhitespace, tag("ne"), xpwhitespace)),
                    tuple((xpwhitespace, tag("lt"), xpwhitespace)),
                    tuple((xpwhitespace, tag("le"), xpwhitespace)),
                    tuple((xpwhitespace, tag("gt"), xpwhitespace)),
                    tuple((xpwhitespace, tag("ge"), xpwhitespace)),
                    tuple((xpwhitespace, tag("is"), xpwhitespace)),
                    tuple((xpwhitespace, tag("<<"), xpwhitespace)),
                    tuple((xpwhitespace, tag(">>"), xpwhitespace)),
                )),
                stringconcat_expr,
            )),
        ),
        |(v, o)| {
            match o {
                None => v,
                Some(((_a, b, _c), t)) => {
                    match b {
                        "=" => vec![Constructor::GeneralComparison(Operator::Equal, vec![v, t])],
                        "!=" => vec![Constructor::GeneralComparison(
                            Operator::NotEqual,
                            vec![v, t],
                        )],
                        "<" => vec![Constructor::GeneralComparison(
                            Operator::LessThan,
                            vec![v, t],
                        )],
                        "<=" => vec![Constructor::GeneralComparison(
                            Operator::LessThanEqual,
                            vec![v, t],
                        )],
                        ">" => vec![Constructor::GeneralComparison(
                            Operator::GreaterThan,
                            vec![v, t],
                        )],
                        ">=" => vec![Constructor::GeneralComparison(
                            Operator::GreaterThanEqual,
                            vec![v, t],
                        )],
                        "eq" => vec![Constructor::ValueComparison(Operator::Equal, vec![v, t])],
                        "ne" => vec![Constructor::ValueComparison(Operator::NotEqual, vec![v, t])],
                        "lt" => vec![Constructor::ValueComparison(Operator::LessThan, vec![v, t])],
                        "le" => vec![Constructor::ValueComparison(
                            Operator::LessThanEqual,
                            vec![v, t],
                        )],
                        "gt" => vec![Constructor::ValueComparison(
                            Operator::GreaterThan,
                            vec![v, t],
                        )],
                        "ge" => vec![Constructor::ValueComparison(
                            Operator::GreaterThanEqual,
                            vec![v, t],
                        )],
                        "is" => vec![Constructor::ValueComparison(Operator::Is, vec![v, t])], //
                        "<<" => vec![Constructor::ValueComparison(Operator::Before, vec![v, t])], // TODO: use appropriate constructor
                        ">>" => vec![Constructor::ValueComparison(Operator::After, vec![v, t])],  //
                        _ => vec![], // Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not a valid comparison operator")}),
                    }
                }
            }
        },
    )(input)
}

// StringConcatExpr ::= RangeExpr ( '||' RangeExpr)*
fn stringconcat_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        separated_list1(tuple((xpwhitespace, tag("||"), xpwhitespace)), range_expr),
        |v| {
            if v.len() == 1 {
                let mut s = Vec::new();
                for i in v {
                    for j in i {
                        s.push(j)
                    }
                }
                s
            } else {
                vec![Constructor::Concat(v)]
            }
        },
    )(input)
}

// RangeExpr ::= AdditiveExpr ( 'to' AdditiveExpr)?
fn range_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        pair(
            additive_expr,
            opt(tuple((
                tuple((xpwhitespace, tag("to"), xpwhitespace)),
                additive_expr,
            ))),
        ),
        |(v, o)| match o {
            None => v,
            Some((_t, u)) => {
                vec![Constructor::Range(vec![v, u])]
            }
        },
    )(input)
}

// For additive and multiplicative expressions,
// passing the expression to be operated upon to the evaluator
// is quite awkward.
// TODO: find a better way

// AdditiveExpr ::= MultiplicativeExpr ( ('+' | '-') MultiplicativeExpr)*
fn additive_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        pair(
            multiplicative_expr,
            many0(tuple((
                alt((
                    tuple((xpwhitespace, tag("+"), xpwhitespace)),
                    tuple((xpwhitespace, tag("-"), xpwhitespace)),
                )),
                multiplicative_expr,
            ))),
        ),
        |(a, b)| {
            if b.len() == 0 {
                a
            } else {
                // The arguments to the constructor are the items to be summed
                // These are pair-wise items: first is the operator as a string literal,
                // second is the value
                let mut r: Vec<ArithmeticOperand<N>> = Vec::new();

                r.push(ArithmeticOperand {
                    op: ArithmeticOperator::Noop,
                    operand: a,
                });

                for ((_x, c, _y), d) in b {
                    r.push(ArithmeticOperand {
                        op: ArithmeticOperator::from(c),
                        operand: d,
                    });
                }
                vec![Constructor::Arithmetic(r)]
            }
        },
    )(input)
}
// MultiplicativeExpr ::= UnionExpr ( ('*' | 'div' | 'idiv' | 'mod') UnionExpr)*
fn multiplicative_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        pair(
            union_expr,
            many0(tuple((
                alt((
                    tuple((xpwhitespace, tag("*"), xpwhitespace)),
                    tuple((xpwhitespace, tag("div"), xpwhitespace)),
                    tuple((xpwhitespace, tag("idiv"), xpwhitespace)),
                    tuple((xpwhitespace, tag("mod"), xpwhitespace)),
                )),
                union_expr,
            ))),
        ),
        |(a, b)| {
            if b.len() == 0 {
                a
            } else {
                // The arguments to the constructor are the items to be summed
                // These are pair-wise items: first is the operator as a string literal,
                // second is the value
                let mut r: Vec<ArithmeticOperand<N>> = Vec::new();

                r.push(ArithmeticOperand {
                    op: ArithmeticOperator::Noop,
                    operand: a,
                });

                for ((_x, c, _y), d) in b {
                    r.push(ArithmeticOperand {
                        op: ArithmeticOperator::from(c),
                        operand: d,
                    });
                }
                vec![Constructor::Arithmetic(r)]
            }
        },
    )(input)
}

// UnionExpr ::= IntersectExceptExpr ( ('union' | '|') IntersectExceptExpr)*
fn union_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        separated_list1(
            alt((
                tuple((xpwhitespace, tag("union"), xpwhitespace)),
                tuple((xpwhitespace, tag("|"), xpwhitespace)),
            )),
            intersectexcept_expr,
        ),
        |v| {
            if v.len() == 1 {
                let mut s = Vec::new();
                for i in v {
                    for j in i {
                        s.push(j)
                    }
                }
                s
            } else {
                vec![Constructor::NotImplemented("union_expr".to_string())]
            }
        },
    )(input)
}

// IntersectExceptExpr ::= InstanceOfExpr ( ('intersect' | 'except') InstanceOfExpr)*
fn intersectexcept_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        pair(
            instanceof_expr,
            many0(tuple((
                alt((
                    tuple((xpwhitespace, tag("intersect"), xpwhitespace)),
                    tuple((xpwhitespace, tag("except"), xpwhitespace)),
                )),
                instanceof_expr::<N>,
            ))),
        ),
        |(a, b)| {
            if b.len() == 0 {
                a
            } else {
                // The arguments to the intersectexcept function are the sequences to be operated upon.
                // These are pair-wise items: first is the operator as a string literal,
                // second is the value
                //	let mut r = Vec::new();

                //        r.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::from(""))))]);
                //	r.push(a);

                //	for ((_x, c, _y), d) in b {
                //	  r.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String(c.to_string()))))]);
                //	  r.push(d);
                //	}
                //        vec![SequenceConstructor::new(cons_intersectexcept).set_args(Some(r))]
                vec![Constructor::NotImplemented("intersectexcept".to_string())]
            }
        },
    )(input)
}

// InstanceOfExpr ::= TreatExpr ( 'instance' 'of' SequenceType)?
fn instanceof_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        pair(
            treat_expr,
            opt(tuple((
                xpwhitespace,
                tag("instance"),
                xpwhitespace,
                tag("of"),
                xpwhitespace,
                sequencetype_expr::<N>,
            ))),
        ),
        |(u, v)| {
            match v {
                None => u,
                Some(_t) => {
                    //let mut r = Vec::new();
                    //r.push(u);
                    //let (_a, _b, _c, _d, _e, st) = t;
                    //r.push(st);
                    //vec![SequenceConstructor::new(cons_instanceof).set_args(Some(r))]
                    vec![Constructor::NotImplemented("instance_of".to_string())]
                }
            }
        },
    )(input)
}

// SequenceType ::= ( 'empty-sequence' '(' ')' | (ItemType OccurrenceIndicator?)
// TODO: implement this parser fully
fn sequencetype_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(tag("empty-sequence()"), |_v| Vec::new())(input)
}

// TreatExpr ::= CastableExpr ( 'treat' 'as' SequenceType)?
fn treat_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        pair(
            castable_expr,
            opt(tuple((
                xpwhitespace,
                tag("treat"),
                xpwhitespace,
                tag("as"),
                xpwhitespace,
                sequencetype_expr::<N>,
            ))),
        ),
        |(u, v)| {
            match v {
                None => u,
                Some(_t) => {
                    //let mut r = Vec::new();
                    //r.push(u);
                    //let (_a, _b, _c, _d, _e, st) = t;
                    //r.push(st);
                    //vec![SequenceConstructor::new(cons_treat).set_args(Some(r))]
                    vec![Constructor::NotImplemented("treat".to_string())]
                }
            }
        },
    )(input)
}

// CastableExpr ::= CastExpr ( 'castable' 'as' SingleType)?
fn castable_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        pair(
            cast_expr,
            opt(tuple((
                xpwhitespace,
                tag("castable"),
                xpwhitespace,
                tag("as"),
                xpwhitespace,
                singletype_expr::<N>,
            ))),
        ),
        |(u, v)| {
            match v {
                None => u,
                Some(_t) => {
                    //let mut r = Vec::new();
                    //r.push(u);
                    //let (_a, _b, _c, _d, _e, st) = t;
                    //r.push(st);
                    //vec![SequenceConstructor::new(cons_castable).set_args(Some(r))]
                    vec![Constructor::NotImplemented("castable".to_string())]
                }
            }
        },
    )(input)
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
fn singletype_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        pair(qname, opt(tuple((xpwhitespace, tag("?"), xpwhitespace)))),
        |(_u, _v)| Vec::new(),
    )(input)
}

// CastExpr ::= ArrowExpr ( 'cast' 'as' SingleType)?
fn cast_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        pair(
            arrow_expr,
            opt(tuple((
                xpwhitespace,
                tag("cast"),
                xpwhitespace,
                tag("as"),
                xpwhitespace,
                singletype_expr::<N>,
            ))),
        ),
        |(u, v)| {
            match v {
                None => u,
                Some(_t) => {
                    //let mut r = Vec::new();
                    //r.push(u);
                    //let (_a, _b, _c, _d, _e, st) = t;
                    //r.push(st);
                    //vec![SequenceConstructor::new(cons_cast).set_args(Some(r))]
                    vec![Constructor::NotImplemented("cast".to_string())]
                }
            }
        },
    )(input)
}

// ArrowExpr ::= UnaryExpr ( '=>' ArrowFunctionSpecifier ArgumentList)*
fn arrow_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        pair(
            unary_expr,
            many0(tuple((
                xpwhitespace,
                tag("=>"),
                xpwhitespace,
                arrowfunctionspecifier::<N>,
                xpwhitespace,
                opt(argumentlist::<N>),
            ))),
        ),
        |(u, v)| {
            if v.len() == 0 {
                u
            } else {
                //vec![SequenceConstructor::new(cons_arrow)]
                vec![Constructor::NotImplemented("arrow".to_string())]
            }
        },
    )(input)
}

// ArrowFunctionSpecifier ::= EQName | VarRef | ParenthesizedExpr
// TODO: finish this parser with EQName and VarRef
fn arrowfunctionspecifier<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(alt((qname_expr::<N>, parenthesized_expr)), |_v| Vec::new())(input)
}
fn qname_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(qname, |q| match q {
        NodeTest::Name(NameTest {
            name: Some(WildcardOrName::Name(localpart)),
            ns: None,
            prefix: None,
        }) => {
            vec![Constructor::Literal(Value::from(localpart))]
        }
        _ => {
            vec![Constructor::Literal(Value::from("invalid qname"))]
        }
    })(input)
}

// ArgumentList ::= '(' (Argument (',' Argument)*)? ')'
// TODO: finish this parser with actual arguments
fn argumentlist<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        tag("()"),
        //tuple((
        //tag("("),
        //xpwhitespace,
        //tag(")"),
        //)),
        |_v| Vec::new(),
    )(input)
}

// UnaryExpr ::= ('-' | '+')* ValueExpr
fn unary_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        pair(many0(alt((tag("-"), tag("+")))), value_expr),
        |(u, v)| {
            if u.len() == 0 {
                v
            } else {
                //let mut a = Vec::new();
                //for i in u {
                //a.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String(String::from(i)))))]);
                //}
                //a.push(v);
                //vec![SequenceConstructor::new(cons_unary).set_args(Some(a))]
                vec![Constructor::NotImplemented("unary".to_string())]
            }
        },
    )(input)
}

// ValueExpr (SimpleMapExpr) ::= PathExpr ('!' PathExpr)*
fn value_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        pair(path_expr, many0(tuple((tag("!"), path_expr::<N>)))),
        |(u, v)| {
            if v.len() == 0 {
                u
            } else {
                //let mut s = Vec::new();
                //s.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::from(""))))]);
                //s.push(u);
                //for (a, b) in v {
                //s.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::from(a))))]);
                //s.push(b);
                //}
                //vec![SequenceConstructor::new(cons_simplemap).set_args(Some(s))]
                vec![Constructor::NotImplemented("value".to_string())]
            }
        },
    )(input)
}

// PathExpr ::= ('/' RelativePathExpr?) | ('//' RelativePathExpr) | RelativePathExpr
fn path_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    alt((
        absolute_descendant_expr,
        absolute_path_expr,
        relativepath_expr,
    ))(input)
}
// ('/' RelativePathExpr?)
fn absolute_path_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(pair(tag("/"), opt(relativepath_expr)), |(_u, v)| {
        match v {
            Some(a) => {
                if a.len() == 1 {
                    match &a[0] {
                        Constructor::Path(w) => {
                            let mut x = vec![vec![Constructor::Root]];
                            for y in w {
                                x.push(y.to_vec())
                            }
                            vec![Constructor::Path(x)]
                        }
                        _ => {
                            vec![Constructor::Path(vec![vec![Constructor::Root], a])]
                        }
                    }
                } else {
                    // Error
                    vec![]
                }
            }
            None => {
                vec![Constructor::Root]
            }
        }
    })(input)
}
// ('//' RelativePathExpr)
fn absolute_descendant_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(pair(tag("//"), relativepath_expr::<N>), |(_u, _v)| {
        vec![
            Constructor::Root,
            Constructor::NotImplemented("absolute_descendant".to_string()),
        ]
        // TODO: process v to implement descendant-or-self
    })(input)
}

// RelativePathExpr ::= StepExpr (('/' | '//') StepExpr)*
fn relativepath_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        pair(
            step_expr,
            many0(tuple((
                alt((
                    tuple((xpwhitespace, tag("//"), xpwhitespace)),
                    tuple((xpwhitespace, tag("/"), xpwhitespace)),
                )),
                step_expr,
            ))),
        ),
        |(a, b)| {
            if b.len() == 0 {
                a
            } else {
                let mut r = Vec::new();

                r.push(a);

                for ((_x, c, _y), d) in b {
                    match c {
                        "/" => r.push(d),
                        _ => {
                            // Insert a descendant-or-self::* step
                            r.push(vec![Constructor::Step(
                                NodeMatch {
                                    axis: Axis::DescendantOrSelf,
                                    nodetest: NodeTest::Name(NameTest {
                                        ns: None,
                                        prefix: None,
                                        name: Some(WildcardOrName::Wildcard),
                                    }),
                                },
                                vec![],
                            )]);
                            r.push(d)
                        }
                    }
                }

                vec![Constructor::Path(r)]
            }
        },
    )(input)
}
// For debugging: a version of the above function that steps through the parsing
//fn relativepath_expr_dbg(newinput: &str) -> IResult<&str, Vec<Constructor>> {
//  let myin = newinput;
//  let (myin, a) = step_expr(myin)?;
//  let mut r = Vec::new();
//
//  r.push(vec![Constructor::Literal(Value::String(""))]);
//  r.push(a);
//
//  loop {
//    if myin.len() == 0 {
//      break
//    }
//    let (myin, (_x, c, _y)) = alt((
//      tuple((xpwhitespace, tag("//"), xpwhitespace)),
//      tuple((xpwhitespace, tag("/"), xpwhitespace)),
//    ))(myin)?;
//    r.push(vec![Constructor::Literal(Value::String(c))]);
//
//    let (_myin, d) = step_expr(myin)?;
//    r.push(d);
//    break;
//  }
//
//  Ok((myin, vec![Constructor::NotImplemented("relpathdbg".to_string())]))
//}

// StepExpr ::= PostfixExpr | AxisStep
fn step_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    alt((
        postfix_expr, // These two return different objects; we need to switch between them
        axisstep,     // TODO: define an enum that allows us to do the switch
    ))(input)
}

// AxisStep ::= (ReverseStep | ForwardStep) PredicateList
// TODO: predicates
fn axisstep<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        pair(alt((reversestep, forwardstep)), predicate_list),
        |(a, p)| {
            if p.is_empty() {
                a
            } else {
                // Insert predicates into step
                if a.len() == 1 {
                    match &a[0] {
                        Constructor::Step(nm, _) => {
                            vec![Constructor::Step(nm.clone(), p)]
                        }
                        _ => {
                            // error
                            vec![]
                        }
                    }
                } else {
                    // error
                    vec![]
                }
            }
        },
    )(input)
}

// PredicateList ::= Predicate*
fn predicate_list<N: Node>(input: &str) -> IResult<&str, Vec<Vec<Constructor<N>>>> {
    many0(predicate)(input)
}

// Predicate ::= '[' Expr ']'
fn predicate<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(tuple((tag("["), expr, tag("]"))), |(_, e, _)| e)(input)
}

// ForwardStep ::= (ForwardAxis NodeTest) | AbbrevForwardStep
// TODO: abbreviated step
fn forwardstep<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(pair(forwardaxis::<N>, nodetest), |(a, n)| {
        vec![Constructor::Step(
            NodeMatch {
                axis: Axis::from(a),
                nodetest: n,
            },
            vec![],
        )]
    })(input)
}
// ReverseStep ::= (ReverseAxis NodeTest) | AbbrevReverseStep
// TODO: abbreviated step
fn reversestep<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(pair(reverseaxis::<N>, nodetest), |(a, n)| {
        vec![Constructor::Step(
            NodeMatch {
                axis: Axis::from(a),
                nodetest: n,
            },
            vec![],
        )]
    })(input)
}

// ForwardAxis ::= ('child' | 'descendant' | 'attribute' | 'self' | 'descendant-or-self' | 'following-sibling' | 'following' | 'namespace') '::'
fn forwardaxis<N: Node>(input: &str) -> IResult<&str, &str> {
    map(
        pair(
            alt((
                tag("child"),
                tag("descendant-or-self"),
                tag("descendant"),
                tag("attribute"),
                tag("self"),
                tag("following-sibling"),
                tag("following"),
                tag("namespace"),
            )),
            tag("::"),
        ),
        |(a, _b)| a,
    )(input)
}
// ReverseAxis ::= ('parent' | 'ancestor' | 'ancestor-or-self' | 'preceding' | 'preceding-sibling') '::'
fn reverseaxis<N: Node>(input: &str) -> IResult<&str, &str> {
    map(
        pair(
            alt((
                tag("parent"),
                tag("ancestor-or-self"),
                tag("ancestor"),
                tag("preceding-sibling"),
                tag("preceding"),
            )),
            tag("::"),
        ),
        |(a, _b)| a,
    )(input)
}

fn qname(input: &str) -> IResult<&str, NodeTest> {
    alt((prefixed_name, unprefixed_name))(input)
}
fn unprefixed_name(input: &str) -> IResult<&str, NodeTest> {
    map(ncname, |localpart| {
        NodeTest::Name(NameTest {
            ns: None,
            prefix: None,
            name: Some(WildcardOrName::Name(String::from(localpart))),
        })
    })(input)
}
fn prefixed_name(input: &str) -> IResult<&str, NodeTest> {
    map(
        tuple((ncname, tag(":"), ncname)),
        |(prefix, _, localpart)| {
            NodeTest::Name(NameTest {
                ns: None,
                prefix: Some(String::from(prefix)),
                name: Some(WildcardOrName::Name(String::from(localpart))),
            })
        },
    )(input)
}

// NodeTest ::= KindTest | NameTest
fn nodetest(input: &str) -> IResult<&str, NodeTest> {
    alt((kindtest, nametest))(input)
}

// KindTest ::= DocumentTest | ElementTest | AttributeTest | SchemaElementTest | SchemaAttributeTest | PITest | CommentTest | TextTest | NamespaceNodeTest | AnyKindTest
fn kindtest(input: &str) -> IResult<&str, NodeTest> {
    alt((
        documenttest,
        elementtest,
        attributetest,
        schemaelementtest,
        schemaattributetest,
        pitest,
        commenttest,
        texttest,
        namespacenodetest,
        anykindtest,
    ))(input)
}
// DocumentTest ::= 'document-node' '(' (ElementTest | SchemaElementTest)? ')'
// TODO: capture the element test
fn documenttest(input: &str) -> IResult<&str, NodeTest> {
    map(
        tuple((
            tag("document-node"),
            xpwhitespace,
            tag("("),
            opt(alt((elementtest, schemaelementtest))),
            xpwhitespace,
            tag(")"),
        )),
        |(_, _, _, _t, _, _)| NodeTest::Kind(KindTest::DocumentTest),
    )(input)
}
// ElementTest ::= 'element' '(' (ElementNameOrWildcard (',' TypeName '?'?)?)? ')'
// TODO: capture element name or wildcard, typename
fn elementtest(input: &str) -> IResult<&str, NodeTest> {
    map(
        tuple((
            tag("element"),
            xpwhitespace,
            tag("("),
            xpwhitespace,
            tag(")"),
        )),
        |(_, _, _, _, _)| NodeTest::Kind(KindTest::ElementTest),
    )(input)
}
// AttributeTest ::= 'attribute' '(' (AttribNameOrWildcard (',' TypeName)?)? ')'
// TODO: capture attribnameOrWildcard and typename
fn attributetest(input: &str) -> IResult<&str, NodeTest> {
    map(
        tuple((
            tag("attribute"),
            xpwhitespace,
            tag("("),
            xpwhitespace,
            tag(")"),
        )),
        |(_, _, _, _, _)| NodeTest::Kind(KindTest::AttributeTest),
    )(input)
}
// SchemaElementTest ::= 'schema-element' '(' ElementDeclaration ')'
// TODO: capture elementDeclaration
fn schemaelementtest(input: &str) -> IResult<&str, NodeTest> {
    map(
        tuple((
            tag("schema-element"),
            xpwhitespace,
            tag("("),
            xpwhitespace,
            tag(")"),
        )),
        |(_, _, _, _, _)| NodeTest::Kind(KindTest::SchemaElementTest),
    )(input)
}
// SchemaAttributeTest ::= 'schema-attribute' '(' AttributeDeclaration ')'
// TODO: capture attribute declaration
fn schemaattributetest(input: &str) -> IResult<&str, NodeTest> {
    map(
        tuple((
            tag("schema-attribute"),
            xpwhitespace,
            tag("("),
            xpwhitespace,
            tag(")"),
        )),
        |(_, _, _, _, _)| NodeTest::Kind(KindTest::SchemaAttributeTest),
    )(input)
}
// PITest ::= 'processing-instruction' '(' (NCName | StringLiteral)? ')'
// TODO: capture PI name
fn pitest(input: &str) -> IResult<&str, NodeTest> {
    map(
        tuple((
            tag("processing-instruction"),
            xpwhitespace,
            tag("("),
            xpwhitespace,
            tag(")"),
        )),
        |(_, _, _, _, _)| NodeTest::Kind(KindTest::PITest),
    )(input)
}
// CommentTest ::= 'comment' '(' ')'
fn commenttest(input: &str) -> IResult<&str, NodeTest> {
    map(
        tuple((
            tag("comment"),
            xpwhitespace,
            tag("("),
            xpwhitespace,
            tag(")"),
        )),
        |(_, _, _, _, _)| NodeTest::Kind(KindTest::CommentTest),
    )(input)
}
// TextTest ::= 'text' '(' ')'
fn texttest(input: &str) -> IResult<&str, NodeTest> {
    map(
        tuple((tag("text"), xpwhitespace, tag("("), xpwhitespace, tag(")"))),
        |(_, _, _, _, _)| NodeTest::Kind(KindTest::TextTest),
    )(input)
}
// NamespaceNodeTest ::= 'namespace-node' '(' ')'
fn namespacenodetest(input: &str) -> IResult<&str, NodeTest> {
    map(
        tuple((
            tag("namespace-node"),
            xpwhitespace,
            tag("("),
            xpwhitespace,
            tag(")"),
        )),
        |(_, _, _, _, _)| NodeTest::Kind(KindTest::NamespaceNodeTest),
    )(input)
}
// AnyKindTest := 'node' '(' ')'
fn anykindtest(input: &str) -> IResult<&str, NodeTest> {
    map(
        tuple((tag("node"), xpwhitespace, tag("("), xpwhitespace, tag(")"))),
        |(_, _, _, _, _)| NodeTest::Kind(KindTest::AnyKindTest),
    )(input)
}
// NameTest ::= EQName | Wildcard
// TODO: allow EQName rather than QName
fn nametest(input: &str) -> IResult<&str, NodeTest> {
    alt((qname, wildcard))(input)
}

// Wildcard ::= '*' | (NCName ':*') | ('*:' NCName) | (BracedURILiteral '*')
// TODO: more specific wildcards
fn wildcard(input: &str) -> IResult<&str, NodeTest> {
    map(tag("*"), |_w| {
        NodeTest::Name(NameTest {
            ns: Some(WildcardOrName::Wildcard),
            prefix: None,
            name: Some(WildcardOrName::Wildcard),
        })
    })(input)
}

// PostfixExpr ::= PrimaryExpr (Predicate | ArgumentList | Lookup)*
// TODO: predicates, arg list, lookup
fn postfix_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    primary_expr(input)
}

// PrimaryExpr ::= Literal | VarRef | ParenthesizedExpr | ContextItemExpr | FunctionCall | FunctionItemExpr | MapConstructor | ArrayConstructor | UnaryLookup
// TODO: finish this parser
fn primary_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    alt((
        literal,
        context_item,
        parenthesized_expr,
        function_call,
        variable_reference,
    ))(input)
}

// VarRef ::= '$' VarName
fn variable_reference<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(pair(tag("$"), qname), |(_, v)| {
        vec![Constructor::VariableReference(get_nt_localname(&v))]
    })(input)
}

// FunctionCall ::= EQName ArgumentList
fn function_call<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(pair(qname, arglist), |(n, a)| match n {
        NodeTest::Name(NameTest {
            name: Some(WildcardOrName::Name(localpart)),
            ns: None,
            prefix: None,
        }) => {
            vec![Constructor::FunctionCall(
                Function::new(localpart, vec![], None),
                a,
            )]
        }
        _ => {
            vec![Constructor::Literal(Value::from("invalid qname"))]
        }
    })(input)
}

// ArgumentList ::= '(' (Argument (',' Argument)*)? ')'
fn arglist<N: Node>(input: &str) -> IResult<&str, Vec<Vec<Constructor<N>>>> {
    map(
        tuple((
            tag("("),
            separated_list0(tuple((xpwhitespace, tag(","), xpwhitespace)), argument),
            tag(")"),
        )),
        |(_, a, _)| a,
    )(input)
}

// Argument ::= ExpreSingle | ArgumentPlaceHolder
// TODO: ArgumentPlaceHolder
fn argument<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    expr_single(input)
}

// Literal ::= NumericLiteral | StringLiteral
fn literal<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    alt((numeric_literal, string_literal))(input)
}

// NumericLiteral ::= IntegerLiteral | DecimalLiteral | DoubleLiteral
fn numeric_literal<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    alt((double_literal, decimal_literal, integer_literal))(input)
}
// IntegerLiteral ::= Digits
fn integer_literal<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(digit1, |s: &str| {
        let n = s.parse::<i64>().unwrap();
        vec![Constructor::Literal(Value::Integer(n))]
    })(input)
}
// DecimalLiteral ::= ('.' Digits) | (Digits '.' [0-9]*)
// Construct a double, but if that fails fall back to decimal
fn decimal_literal<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        alt((
            recognize(complete(pair(tag("."), digit1))),
            recognize(complete(tuple((digit1, tag("."), digit0)))),
        )),
        |s: &str| {
            let n = s.parse::<f64>();
            let i = match n {
                Ok(m) => Value::Double(m),
                Err(_) => Value::Decimal(Decimal::from_str(s).unwrap()),
            };
            vec![Constructor::Literal(i)]
        },
    )(input)
}
// DoubleLiteral ::= (('.' Digits) | (Digits ('.' [0-9]*)?)) [eE] [+-]? Digits
// Construct a double
fn double_literal<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(
        recognize(tuple((
            alt((
                recognize(complete(pair(tag("."), digit1))),
                recognize(complete(tuple((digit1, tag("."), digit0)))),
            )),
            one_of("eE"),
            opt(one_of("+-")),
            digit1,
        ))),
        |s: &str| {
            let n = s.parse::<f64>();
            let i = match n {
                Ok(m) => Value::Double(m),
                Err(_) => panic!("unable to convert to double"),
            };
            vec![Constructor::Literal(i)]
        },
    )(input)
}

// StringLiteral ::= double- or single-quote delimited with double-delimiter escape
fn string_literal_double(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        map(
            many0(alt((map(tag("\"\""), |_| '"'), none_of("\"")))),
            |v| v.iter().collect::<String>(),
        ),
        char('"'),
    )(input)
}
fn string_literal_single(input: &str) -> IResult<&str, String> {
    delimited(
        char('\''),
        map(many0(alt((map(tag("''"), |_| '\''), none_of("'")))), |v| {
            v.iter().collect::<String>()
        }),
        char('\''),
    )(input)
}
fn string_literal<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(alt((string_literal_double, string_literal_single)), |s| {
        vec![Constructor::Literal(Value::from(s))]
    })(input)
}
// ContextItemExpr ::= '.'
fn context_item<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    map(tag("."), |_| vec![Constructor::ContextItem])(input)
}
// ParenthesizedExpr ::= '(' Expr? ')'
fn parenthesized_expr<N: Node>(input: &str) -> IResult<&str, Vec<Constructor<N>>> {
    delimited(
        tag("("),
        map(opt(expr), |e| match e {
            Some(v) => v,
            None => Vec::new(),
        }),
        tag(")"),
    )(input)
}

// Whitespace, including comments
fn xpwhitespace(input: &str) -> IResult<&str, &str> {
    recognize(many0(alt((multispace1, xpcomment))))(input)
}
fn xpcomment(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        multispace0,
        take_until_balanced("(:", ":)"),
        multispace0,
    )))(input)
}

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
fn take_until_balanced<'a>(
    open: &'a str,
    close: &'a str,
) -> impl Fn(&str) -> IResult<&str, &str> + 'a {
    move |i: &str| {
        let mut index = 0;
        let mut bracket_counter = 0;
        if i.starts_with(open) {
            index += open.len();
            bracket_counter += 1;
            loop {
                match (&i[index..].find(open), &i[index..].find(close)) {
                    (_, None) => {
                        // Scenario 1
                        return Result::Err(NomErr::Error(NomError {
                            input: i,
                            code: NomErrorKind::TakeUntil,
                        }));
                    }
                    (None, Some(c)) => {
                        // Scenario 2
                        if bracket_counter > 1 {
                            bracket_counter -= 1;
                            index += c + close.len();
                        } else if bracket_counter == 1 {
                            index += c + close.len();
                            return Ok((&i[index..], &i[0..index]));
                        } else {
                            return Result::Err(NomErr::Error(NomError {
                                input: i,
                                code: NomErrorKind::TakeUntil,
                            }));
                        }
                    }
                    (Some(o), Some(c)) => {
                        // Scenario 3/4
                        if o > c {
                            // Scenario 3
                            if bracket_counter == 1 {
                                index += c + close.len();
                                return Ok((&i[index..], &i[0..index]));
                            } else {
                                return Result::Err(NomErr::Error(NomError {
                                    input: i,
                                    code: NomErrorKind::TakeUntil,
                                }));
                            }
                        } else {
                            // Scenario 4
                            bracket_counter += 1;
                            index += o + open.len();
                        }
                    }
                }
            }
            // unreachable!();
        } else {
            Result::Err(NomErr::Error(NomError {
                input: i,
                code: NomErrorKind::TakeUntil,
            }))
        }
    }
}

/// Parse an XPath expression. The result is a Sequence constructor.
pub fn parse<N: Node>(e: &str) -> Result<Vec<Constructor<N>>, crate::xdmerror::Error> {
    match expr(e) {
        Ok((rest, value)) => {
            if rest == "" {
                Result::Ok(value)
            } else {
                Result::Err(Error {
                    kind: ErrorKind::Unknown,
                    message: String::from(format!(
                        "extra characters after expression: \"{}\"",
                        rest
                    )),
                })
            }
        }
        Err(nom::Err::Error(c)) => Result::Err(Error {
            kind: ErrorKind::Unknown,
            message: format!("parser error: {:?}", c),
        }),
        Err(nom::Err::Incomplete(_)) => Result::Err(Error {
            kind: ErrorKind::Unknown,
            message: String::from("incomplete input"),
        }),
        Err(nom::Err::Failure(_)) => Result::Err(Error {
            kind: ErrorKind::Unknown,
            message: String::from("unrecoverable parser error"),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xpcomment_1() {
        assert_eq!(xpcomment("(: my comment :)"), Ok(("", "(: my comment :)")))
    }
    #[test]
    fn xpcomment_2() {
        assert_eq!(
            xpcomment(" \t(: my comment :)\n"),
            Ok(("", " \t(: my comment :)\n"))
        )
    }
    #[test]
    fn xpcomment_3() {
        assert_eq!(
            xpcomment("(: my comment :)XYZ"),
            Ok(("XYZ", "(: my comment :)"))
        )
    }
    #[test]
    fn xpcomment_4() {
        assert_eq!(
            xpcomment("(:outer(:inner:)outer:)"),
            Ok(("", "(:outer(:inner:)outer:)"))
        )
    }
    #[test]
    fn xpcomment_5() {
        assert_eq!(
            xpcomment("(:outer(:inner  outer:)"),
            Result::Err(NomErr::Error(NomError {
                input: "(:outer(:inner  outer:)",
                code: NomErrorKind::TakeUntil
            }))
        )
    }
    #[test]
    fn nomxpath_parse_ws_comment_1() {
        assert_eq!(xpwhitespace(" \n\t"), Ok(("", " \n\t")));
        assert_eq!(xpwhitespace("(: foobar :)"), Ok(("", "(: foobar :)")));
        assert_eq!(
            xpwhitespace("(: outer (: inner :) outer :)"),
            Ok(("", "(: outer (: inner :) outer :)"))
        );
        assert_eq!(xpwhitespace(" (: foobar :) "), Ok(("", " (: foobar :) ")));
        assert_eq!(xpwhitespace("X(: foobar :)"), Ok(("X(: foobar :)", "")));
    }
}
