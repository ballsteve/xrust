//! # xrust::pattern
//!
//! Support for XPath patterns. This modules provides both a parser to compile a pattern, and an interpreter to determine if an item matches a compiled pattern.
// Patterns are defined in XSLT 3.0 5.5.2

use std::convert::TryFrom;
use std::rc::Rc;
use std::fmt;
use std::fmt::Formatter;

use std::marker::PhantomData;

use crate::evaluate::{ArithmeticOperator, Axis, NameTest, NodeMatch, NodeTest, WildcardOrName};
use crate::item::{Item, Node, Sequence, SequenceTrait};
use crate::transcomb::{Combinator, Context, ContextBuilder, TransResult};
use crate::xpath::{literal, nodetest, predicate_list, qname, variable_reference, xpwhitespace};
//use crate::transcomb::{
//    arithmetic, boolean, ceiling, compose, contains, context, current_date, current_date_time,
//    current_group, current_grouping_key, current_time, declare_variable, empty, floor, format_date,
//    format_date_time, format_time, general_comparison, last, literal as tc_literal, local_name,
//    name, normalize_space, not, not_implemented, number, position, reference_variable, root, round,
//    starts_with, step, string, substring, substring_after, substring_before, sum, switch, tc_and,
//    tc_concat, tc_count, tc_false, tc_loop, tc_or, tc_range, tc_sequence, tc_true, translate,
//    value_comparison, Combinator, Context, TransResult,
//};
use crate::value::Value;
use crate::value::*;
use crate::xdmerror::*;
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::parser::combinators::alt::{alt2, alt3, alt4, alt5, alt6};
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

/// An XPath pattern. A pattern most frequently appears as the value of a match attribute. A paatern is either a predicate pattern or a selection pattern.
///
/// A predicate pattern matches the current item if all of the predicates evaluate to true.
///
/// A selection pattern is subset of XPath path expressions.
pub enum Pattern<'a, N: Node, F>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Predicate(Combinator<'a, N, F>),
    Selection(Path),
    Error(Error),
    Unused(PhantomData<F>),
}

impl<'a, N: Node, F> Pattern<'a, N, F>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    /// Returns whether the given item matches the pattern.
    /// TODO: return dynamic errors
    pub fn matches(&self, i: Rc<Item<N>>) -> bool {
        eprintln!("Pattern::matches item is a {} named {}", i.item_type(), i.name().to_string());
        match self {
            Pattern::Predicate(p) => {
                match p(&mut ContextBuilder::new().sequence(vec![i.clone()]).build()) {
                    Ok(s) => s.to_bool(),
                    Err(_) => false, // this is where we would like to propagate the error
                }
            }
            Pattern::Selection(p) => {
                // First step is the terminal case,
                // next steps are non-terminal
                eprintln!("Selection pattern");
                p.t.as_ref().map_or(
                    false,
                    |((term, nonterm), nt)| {
                        eprintln!("check terminal step first");
                        if is_match(term, nt, i.clone()) {
                            // TODO: select item depending on non-terminal axis
                            find_node(nonterm, i.clone()).map_or(
                                false,
                                |f| nonterminal(p.next.clone(), f)
                            )
                        } else {
                            false
                        }
                    }
                )
            }
            _ => false, // not yet implemented
        }
    }
}

fn find_node<N: Node>(a: &Axis, i: Rc<Item<N>>) -> Option<Rc<Item<N>>> {
    eprintln!("find_node: axis {} item is a {} named {}", a, i.item_type(), i.name().to_string());
    match a {
        Axis::Parent => {
            match &*i {
                Item::Node(n) => {
                    n.parent().map(|p| Rc::new(Item::Node(p)))
                }
                _ => None,
            }
        }
        _ => None, // todo
    }
}

fn nonterminal<N: Node>(p: Option<Rc<Path>>, i: Rc<Item<N>>) -> bool {
    eprintln!("nonterminal step 1==\"{:?}\"\nitem is a {} named {}", p, i.item_type(), i.name().to_string());
    p.map_or(
        true, // all steps have succeeded so far
        |q| {
            let ((term, nonterm), nt) = q.t.as_ref().unwrap();
            if is_match(&term, &nt, i.clone()) {
                find_node(nonterm, i.clone()).map_or(
                    false, // couldn't find the next node
                    |p| nonterminal(q.next.clone(), p)
                )
            } else { false }
        }
    )
}

fn is_match<N: Node>(a: &Axis, nt: &NodeTest, i: Rc<Item<N>>) -> bool {
    eprintln!("is_match axis==\"{}\" nt=\"{}\" item is a {} named {}", a, nt, i.item_type(), i.name().to_string());
    match a {
        Axis::Selfaxis => {
            // Select item if it is an element-type node
            if i.is_element_node() {
                nt.matches(i)
            } else {
                false
            }
        }
        Axis::Parent => {
            // Select the parent node
            match &*i {
                Item::Node(n) => {
                    n.parent().map_or(
                        false,
                        |p| nt.matches(Rc::new(Item::Node(p)))
                    )
                }
                _ => false,
            }
        }
        _ => false // todo
    }
}

impl<'a, N: Node, F> fmt::Debug for Pattern<'a, N, F>
    where
        F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pattern::Predicate(_) => f.write_str("Pattern::Predicate"),
            Pattern::Selection(p) => f.write_str(format!("Pattern::Selection path=\"{:?}\"", p).as_str()),
            Pattern::Error(e) => f.write_str(format!("Pattern::Error error=\"{:?}\"", e).as_str()),
            _ => f.write_str("some other Pattern"),
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Path {
    //    steps: Vec<Step>,
    pub t: Option<((Axis, Axis), NodeTest)>,
    pub next: Option<Rc<Path>>,
}

impl Path {
    pub fn new() -> Self {
        //Path { t: None, next: None }
        Default::default()
    }
}

pub struct PathBuilder(Path);
impl PathBuilder {
    pub fn new() -> Self {
        PathBuilder(Path::new())
    }
    pub fn step(mut self, t: Axis, l: Axis, nt: NodeTest) -> Self {
        self.0.t = Some(((t, l), nt));
        self
    }
    pub fn build(self) -> Path {
        self.0
    }
}

/// Compile an XPath pattern.
impl<'a, N: Node, F> TryFrom<&str> for Pattern<'a, N, F>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    type Error = Error;
    fn try_from(
        e: &str,
    ) -> Result<Self, <crate::pattern::Pattern<'a, N, F> as TryFrom<&str>>::Error> {
        if e == "" {
            Err(Error::new(
                ErrorKind::TypeError,
                String::from("empty string is not allowed as an XPath pattern"),
            ))
        } else {
            let mut input = ParseInput::new(e);
            match pattern::<N, F>(input) {
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
}

// Pattern30 ::= PredicatePattern | UnionExprP ;
fn pattern<'a, N: Node, F>(input: ParseInput) -> ParseResult<Pattern<'a, N, F>>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    alt2(predicate_pattern::<N, F>(), union_expr_pattern())(input)
}

// PredicatePattern ::= "." PredicateList
// Context must match all predicates
fn predicate_pattern<'a, N: Node, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<'a, N, F>> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        pair(
            map(tuple3(xpwhitespace(), tag("."), xpwhitespace()), |_| ()),
            predicate_list::<N, F>(),
        ),
        |(_, p)| Pattern::Predicate(p),
    ))
}

// UnionExprP ::= IntersectExceptExprP (("union" | "|") IntersectExceptExprP)*
// A union expression matches if any of its components is a match. This creates a branching structure in the compilation of the pattern.
fn union_expr_pattern<'a, N: Node, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<'a, N, F>> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        separated_list1(
            map(
                tuple3(xpwhitespace(), alt2(tag("union"), tag("|")), xpwhitespace()),
                |_| (),
            ),
            intersect_except_expr_pattern::<N, F>(),
        ),
        |mut v| {
            if v.len() == 1 {
                eprintln!("union_expr_pattern: singleton union");
                v.pop().unwrap()
            } else {
                eprintln!("union_expr_pattern: too many union arms, not yet supported");
                Pattern::Selection(Path::new())
            }
        },
    ))
}

fn union_expr_wrapper<'a, N: Node, F>(
    b: bool,
) -> impl Fn(ParseInput) -> ParseResult<Pattern<'a, N, F>> + 'a
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    move |input| {
        if b {
            union_expr_pattern::<N, F>()(input)
        } else {
            noop()(input)
        }
    }
}

fn noop<'a, N: Node, F>() -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<'a, N, F>> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(move |_| Err(ParseError::Combinator))
}

// IntersectExceptExprP ::= PathExprP (("intersect" | "except") PathExprP)*
// intersect and except not yet supported
fn intersect_except_expr_pattern<'a, N: Node, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<'a, N, F>> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        separated_list1(
            map(
                tuple3(
                    xpwhitespace(),
                    alt2(tag("intersect"), tag("except")),
                    xpwhitespace(),
                ),
                |_| (),
            ),
            path_expr_pattern(),
        ),
        |mut v| {
            if v.len() == 1 {
                v.pop().unwrap()
            } else {
                // intersect/except not implemented
                Pattern::Error(Error::new(
                    ErrorKind::NotImplemented,
                    String::from("intersect or except in a pattern has not been implemented"),
                ))
            }
        },
    ))
}

// PathExprP ::= RootedPath | ("/" RelativePathExprP) | ("//" RelativePathExprP) | RelativePathExprP
fn path_expr_pattern<'a, N: Node, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<'a, N, F>> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(alt4(
        rooted_path_pattern(),
        absolutedescendant_expr_pattern(),
        absolutepath_expr_pattern(),
        relativepath_expr_pattern(),
    ))
}

// RootedPath ::= (VarRef | FunctionCallP) PredicateList (("/" | "//") RelativePathExprP)?
fn rooted_path_pattern<'a, N: Node, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<'a, N, F>> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        tuple3(
            alt2(
                variable_reference_pattern::<N, F>(),
                function_call_pattern(),
            ),
            predicate_list::<N, F>(),
            alt2(
                absolutedescendant_expr_pattern::<N, F>(),
                absolutepath_expr_pattern::<N, F>(),
            ),
        ),
        |(_a, _b, _c)| {
            Pattern::Error(Error::new(
                ErrorKind::NotImplemented,
                String::from("rooted path in a pattern has not been implemented"),
            ))
        },
    ))
}

// Variable Reference
fn variable_reference_pattern<'a, N: Node, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<'a, N, F>> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(variable_reference(), |r| Pattern::Predicate(r)))
}

// ('//' RelativePathExpr?)
fn absolutedescendant_expr_pattern<'a, N: Node, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<'a, N, F>> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        pair(tag("//"), relativepath_expr_pattern::<N, F>()),
        |(_, r)| {
            Pattern::Error(Error::new(
                ErrorKind::NotImplemented,
                String::from("absolute descendant path in a pattern has not been implemented"),
            ))
        },
    ))
}

// ('/' RelativePathExpr?)
fn absolutepath_expr_pattern<'a, N: Node, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<'a, N, F>> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        pair(tag("/"), opt(relativepath_expr_pattern::<N, F>())),
        |(_, r)| match r {
            Some(a) => Pattern::Error(Error::new(
                ErrorKind::NotImplemented,
                String::from("absolute path in a pattern has not been implemented"),
            )),
            None => Pattern::Error(Error::new(
                ErrorKind::NotImplemented,
                String::from("root path in a pattern has not been implemented"),
            )),
        },
    ))
}

// RelativePathExpr ::= StepExpr (('/' | '//') StepExpr)*
fn relativepath_expr_pattern<'a, N: Node, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<'a, N, F>> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        pair(
            step_expr_pattern::<N, F>(),
            many0(tuple2(
                alt2(
                    map(tuple3(xpwhitespace(), tag("//"), xpwhitespace()), |_| "//"),
                    map(tuple3(xpwhitespace(), tag("/"), xpwhitespace()), |_| "/"),
                ),
                step_expr_pattern::<N, F>(),
            )),
        ),
        |(a, b)| {
            eprintln!("relativepath_expr_pattern");
            if b.is_empty() {
                // this is the terminal step
                a
            } else {
                let mut ap = match a {
                    Pattern::Selection(p) => p,
                    _ => panic!("relative path may only contain steps")
                };
                for (_c, d) in b {
                    match d {
                        Pattern::Selection(mut p) => {
                            p.next = Some(Rc::new(ap));
                            ap = p.clone();
                        }
                        _ => panic!("relative path can only contain steps")
                    }
                }
                Pattern::Selection(ap)
            }
        },
    ))
}

// StepExprP ::= PostfixExprExpr | AxisStepP
fn step_expr_pattern<'a, N: Node, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<'a, N, F>> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(alt2(postfix_expr_pattern(), axis_step_pattern()))
}

// PostfixExprP ::= ParenthesizedExprP PredicateList
// TODO: predicates
fn postfix_expr_pattern<'a, N: Node, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<'a, N, F>> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        tuple2(paren_expr_pattern(), predicate_list::<N, F>()),
        |(p, _)| p,
    ))
}

// ParenthesizedExprP ::= "(" UnionExprP ")"
fn paren_expr_pattern<'a, N: Node, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<'a, N, F>> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        tuple3(
            tuple3(xpwhitespace(), tag("("), xpwhitespace()),
            union_expr_wrapper(true),
            tuple3(xpwhitespace(), tag(")"), xpwhitespace()),
        ),
        |(_, u, _)| u,
    ))
}

// AxisStepP ::= ForwardStepP PredicateList
// TODO: predicate
fn axis_step_pattern<'a, N: Node, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<'a, N, F>> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        tuple2(forward_step_pattern(), predicate_list::<N, F>()),
        |(f, _p)| f, // TODO: pass predicate back to caller
    ))
}

// ForwardStepP ::= (ForwardAxisP NodeTest) | AbbrevForwardStep
// Returns the node test, the terminal axis and the non-terminal axis
// TODO: abbreviated step
fn forward_step_pattern<'a, N: Node, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<'a, N, F>> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        tuple2(forward_axis_pattern(), nodetest()),
        |((a, c), nt)| {
            eprintln!("forward_step_pattern a=\"{}\" c=\"{}\" nt=\"{}\"", a, c, nt);
            Pattern::Selection(
                PathBuilder::new()
                    .step(a, c, nt)
                    .build()
            )
        },
    ))
}

// ForwardAxisP ::= ("child" | "descendant" | "attribute" | "self" | "descendant-or-self" | "namespace" ) "::"
// Returns a pair: the axis to match this step, and the axis for the previous step
// TODO: abbreviated step
fn forward_axis_pattern(
) -> Box<dyn Fn(ParseInput) -> ParseResult<(Axis, Axis)>>
{
    Box::new(map(
        tuple2(
            alt6(
                map(tag("child"), |_| (Axis::Selfaxis, Axis::Parent)),
                map(tag("descendant"), |_| (Axis::Selfaxis, Axis::Ancestor)),
                map(tag("attribute"), |_| (Axis::SelfAttribute, Axis::Parent)),
                map(tag("self"), |_| (Axis::Selfaxis, Axis::Selfaxis)),
                map(tag("descendant-or-self"), |_| {
                    (Axis::Selfaxis, Axis::Ancestor)
                }),
                map(tag("namespace"), |_| (Axis::SelfNamespace, Axis::Parent)),
            ),
            tag("::"),
        ),
        |(a, _)| a,
    ))
}

// FunctionCallP ::= OuterFunctionName ArgumentListP
fn function_call_pattern<'a, N: Node, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<'a, N, F>> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        tuple2(outer_function_name(), argument_list_pattern::<N, F>()),
        |(_n, _a)| {
            // TODO
            Pattern::Error(Error::new(
                ErrorKind::NotImplemented,
                String::from("function call in a pattern has not been implemented"),
            ))
        },
    ))
}

// ArgumentListP ::= "(" (ArgumentP ("," ArgumentP)*)? ")"
fn argument_list_pattern<'a, N: Node, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<'a, N, F>> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(
        tuple3(
            map(tuple3(xpwhitespace(), tag("("), xpwhitespace()), |_| ()),
            separated_list0(
                map(tuple3(xpwhitespace(), tag(","), xpwhitespace()), |_| ()),
                argument_pattern::<N, F>(),
            ),
            map(tuple3(xpwhitespace(), tag(")"), xpwhitespace()), |_| ()),
        ),
        |(_, a, _)| {
            // TODO
            Pattern::Error(Error::new(
                ErrorKind::NotImplemented,
                String::from("argument list in a pattern has not been implemented"),
            ))
        },
    ))
}

// ArgumentP ::= VarRef | Literal
fn argument_pattern<'a, N: Node, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<'a, N, F>> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(alt2(variable_reference_pattern(), literal_pattern()))
}

// literal
fn literal_pattern<'a, N: Node, F>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<'a, N, F>> + 'a>
where
    F: Fn(&mut Context<'a, N, F>) -> TransResult<'a, N> + 'a,
{
    Box::new(map(literal(), |l| Pattern::Predicate(l)))
}

// OuterFunctionName ::= "doc" | "id" | "element-with-id" | "key" | "root" | URIQualifiedName
fn outer_function_name() -> Box<dyn Fn(ParseInput) -> ParseResult<NodeTest>> {
    Box::new(alt6(
        map(tag("doc"), |_| {
            NodeTest::Name(NameTest {
                ns: None,
                prefix: None,
                name: Some(WildcardOrName::Name(String::from("doc"))),
            })
        }),
        map(tag("id"), |_| {
            NodeTest::Name(NameTest {
                ns: None,
                prefix: None,
                name: Some(WildcardOrName::Name(String::from("id"))),
            })
        }),
        map(tag("element-with-id"), |_| {
            NodeTest::Name(NameTest {
                ns: None,
                prefix: None,
                name: Some(WildcardOrName::Name(String::from("element-with-id"))),
            })
        }),
        map(tag("key"), |_| {
            NodeTest::Name(NameTest {
                ns: None,
                prefix: None,
                name: Some(WildcardOrName::Name(String::from("key"))),
            })
        }),
        map(tag("root"), |_| {
            NodeTest::Name(NameTest {
                ns: None,
                prefix: None,
                name: Some(WildcardOrName::Name(String::from("root"))),
            })
        }),
        map(qname(), |q| q),
    ))
}
