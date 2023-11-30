//! # xrust::pattern
//!
//! Support for XPath patterns. This module provides both a parser to compile a pattern,
//! and an interpreter to determine if an item matches a compiled pattern.
//! Patterns are defined in XSLT 3.0 5.5.2

use std::convert::TryFrom;
use std::fmt;
use std::rc::Rc;

use crate::item::{Item, Node, NodeType, SequenceTrait};
use crate::parser::combinators::whitespace::xpwhitespace;
use crate::parser::xpath::literals::literal;
use crate::parser::xpath::nodetests::{nodetest, qualname_test};
use crate::parser::xpath::predicates::predicate_list;
use crate::parser::xpath::variables::variable_reference;
use crate::transform::context::{Context, ContextBuilder, StaticContext};
use crate::transform::{Axis, KindTest, NameTest, NodeTest, Transform, WildcardOrName};
use crate::value::Value;
use crate::xdmerror::{Error, ErrorKind};

use crate::parser::combinators::alt::{alt2, alt4, alt6};
use crate::parser::combinators::list::{separated_list0, separated_list1};
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::pair::pair;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::{tuple2, tuple3};
use crate::parser::{ParseError, ParseInput, ParseResult, ParserState};
//use crate::parser::combinators::debug::inspect;

/// An XPath pattern. A pattern most frequently appears as the value of a match attribute.
/// A pattern is either a predicate pattern or a selection pattern.
///
/// A predicate pattern matches the current item if all of the predicates evaluate to true.
///
/// A selection pattern is subset of XPath path expressions.
#[derive(Clone)]
pub enum Pattern<N: Node> {
    Predicate(Transform<N>),
    Selection(Path),
    Error(Error),
}

impl<N: Node> Pattern<N> {
    /// Returns whether the given item matches the pattern.
    /// TODO: return dynamic errors
    pub fn matches<F: FnMut(&str) -> Result<(), Error>>(
        &self,
        ctxt: &Context<N>,
        stctxt: &mut StaticContext<F>,
        i: &Rc<Item<N>>) -> bool {
        match self {
            Pattern::Predicate(t) => ContextBuilder::from(ctxt)
                .current(vec![i.clone()])
                .build()
                .dispatch(stctxt, t)
                .unwrap_or(vec![Rc::new(Item::Value(Value::from(false)))])
                .to_bool(),
            Pattern::Selection(p) => {
                // First step is the terminal case,
                // next steps are non-terminal
                p.t.as_ref().map_or(false, |((term, nonterm), nt)| {
                    if is_match(term, nt, i) {
                        // TODO: select item depending on non-terminal axis
                        find_node(nonterm, i.clone())
                            .map_or(false, |f| nonterminal(p.next.clone(), &f))
                    } else {
                        false
                    }
                })
            }
            _ => false, // not yet implemented
        }
    }
}

fn find_node<N: Node>(a: &Axis, i: Rc<Item<N>>) -> Option<Rc<Item<N>>> {
    match a {
        Axis::SelfDocument => match &*i {
            Item::Node(n) => {
                if n.node_type() == NodeType::Document {
                    Some(i)
                } else {
                    None
                }
            }
            _ => None,
        },
        Axis::Parent => match &*i {
            Item::Node(n) => n.parent().map(|p| Rc::new(Item::Node(p))),
            _ => None,
        },
        _ => None, // todo
    }
}

fn nonterminal<N: Node>(p: Option<Rc<Path>>, i: &Rc<Item<N>>) -> bool {
    p.map_or(
        true, // all steps have succeeded so far
        |q| {
            let ((term, nonterm), nt) = q.t.as_ref().unwrap();
            if is_match(&term, &nt, i) {
                find_node(nonterm, i.clone()).map_or(
                    false, // couldn't find the next node
                    |p| nonterminal(q.next.clone(), &p),
                )
            } else {
                false
            }
        },
    )
}

fn is_match<N: Node>(a: &Axis, nt: &NodeTest, i: &Rc<Item<N>>) -> bool {
    match a {
        Axis::SelfDocument => {
            // Select item only if it is a document-type node
            match &**i {
                Item::Node(n) => {
                    if n.node_type() == NodeType::Document {
                        //eprintln!("item is a document-type node, nt.matches=={}", nt.matches(i));
                        nt.matches(i)
                    } else {
                        false
                    }
                }
                _ => false,
            }
        }
        Axis::SelfAxis => {
            // Select item if it is an element-type node
            nt.matches(i)
        }
        Axis::Parent => {
            // Select the parent node
            match &**i {
                Item::Node(n) => n
                    .parent()
                    .map_or(false, |p| nt.matches(&Rc::new(Item::Node(p)))),
                _ => false,
            }
        }
        _ => false, // todo
    }
}

impl<N: Node> fmt::Display for Pattern<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pattern::Predicate(t) => write!(f, "Pattern::Predicate t==\"{}\"", t),
            Pattern::Selection(p) => write!(f, "Pattern::Selection path=\"{:?}\"", p),
            Pattern::Error(e) => write!(f, "Pattern::Error error=\"{:?}\"", e),
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Path {
    //    Each step in the Path consists of (terminal, non-terminal) axes and a NodeTest
    // If next == None, then the terminal axis is used.
    // Otherwise the non-terminal axis applies.
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
impl<N: Node> TryFrom<&str> for Pattern<N> {
    type Error = Error;
    fn try_from(e: &str) -> Result<Self, <crate::pattern::Pattern<N> as TryFrom<&str>>::Error> {
        if e == "" {
            Err(Error::new(
                ErrorKind::TypeError,
                String::from("empty string is not allowed as an XPath pattern"),
            ))
        } else {
            let state = ParserState::new(None, None);
            match pattern::<N>((e, state)) {
                Ok(((rem, _), f)) => {
                    if rem.is_empty() {
                        Ok(f)
                    } else {
                        Err(Error::new(
                            ErrorKind::Unknown,
                            format!("extra characters found: \"{:?}\"", rem),
                        ))
                    }
                }
                Err(err) => Err(Error::new(ErrorKind::Unknown, format!("{:?}", err))),
            }
        }
    }
}

impl<'a, N: Node> TryFrom<String> for Pattern<N> {
    type Error = Error;
    fn try_from(
        e: String,
    ) -> Result<Self, <crate::pattern::Pattern<N> as TryFrom<&'a str>>::Error> {
        Pattern::try_from(e.as_str())
    }
}

// Pattern30 ::= PredicatePattern | UnionExprP ;
fn pattern<N: Node>(input: ParseInput) -> ParseResult<Pattern<N>> {
    alt2(predicate_pattern::<N>(), union_expr_pattern())(input)
}

// PredicatePattern ::= "." PredicateList
// Context must match all predicates
fn predicate_pattern<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<N>> + 'a>
{
    Box::new(map(
        pair(
            map(tuple3(xpwhitespace(), tag("."), xpwhitespace()), |_| ()),
            predicate_list::<N>(),
        ),
        |(_, p)| Pattern::Predicate(p),
    ))
}

// UnionExprP ::= IntersectExceptExprP (("union" | "|") IntersectExceptExprP)*
// A union expression matches if any of its components is a match. This creates a branching structure in the compilation of the Pattern<N>.
fn union_expr_pattern<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<N>> + 'a>
{
    Box::new(map(
        separated_list1(
            map(
                tuple3(xpwhitespace(), alt2(tag("union"), tag("|")), xpwhitespace()),
                |_| (),
            ),
            intersect_except_expr_pattern::<N>(),
        ),
        |mut v| {
            if v.len() == 1 {
                v.pop().unwrap()
            } else {
                Pattern::Selection(Path::new())
            }
        },
    ))
}

// NB. Rust *really* doesn't like recursive types, so we must force it to lazily evaluate arguments to avoid stack overflow.
fn union_expr_wrapper<N: Node>(b: bool) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<N>>> {
    Box::new(move |input| {
        if b {
            union_expr_pattern::<N>()(input)
        } else {
            noop()(input)
        }
    })
}

fn noop<N: Node>() -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<N>>> {
    Box::new(move |_| Err(ParseError::Combinator))
}

// IntersectExceptExprP ::= PathExprP (("intersect" | "except") PathExprP)*
// intersect and except not yet supported
fn intersect_except_expr_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<N>> + 'a> {
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
            path_expr_pattern::<N>(),
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
fn path_expr_pattern<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<N>> + 'a>
{
    Box::new(alt4(
        rooted_path_pattern::<N>(),
        absolutedescendant_expr_pattern(),
        absolutepath_expr_pattern(),
        relativepath_expr_pattern::<N>(),
    ))
}

// RootedPath ::= (VarRef | FunctionCallP) PredicateList (("/" | "//") RelativePathExprP)?
fn rooted_path_pattern<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<N>> + 'a>
{
    Box::new(map(
        tuple3(
            alt2(variable_reference_pattern::<N>(), function_call_pattern()),
            predicate_list::<N>(),
            alt2(
                absolutedescendant_expr_pattern::<N>(),
                absolutepath_expr_pattern(),
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
fn variable_reference_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<N>> + 'a> {
    Box::new(map(variable_reference::<N>(), |r| Pattern::Predicate(r)))
}

// ('//' RelativePathExpr?)
fn absolutedescendant_expr_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<N>> + 'a> {
    Box::new(map(
        pair(tag("//"), relativepath_expr_pattern::<N>()),
        |(_, _r)| {
            Pattern::Error(Error::new(
                ErrorKind::NotImplemented,
                String::from("absolute descendant path in a pattern has not been implemented"),
            ))
        },
    ))
}

// ('/' RelativePathExpr?)
fn absolutepath_expr_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<N>> + 'a> {
    Box::new(map(
        pair(map(tag("/"), |_| "/"), opt(relativepath_expr_pattern::<N>())),
        |(d, r)| match (d, r) {
            ("/", None) => {
                // Matches the root node
                Pattern::Selection(PathBuilder::new()
                    .step(
                        Axis::SelfDocument,
                        Axis::SelfDocument,
                        NodeTest::Kind(KindTest::Document),
                    )
                    .build())
            }
            ("/", Some(_a)) => Pattern::Error(Error::new(
                ErrorKind::NotImplemented,
                String::from("absolute path in a pattern has not been implemented"),
            )),
            _ => Pattern::Error(Error::new(
                ErrorKind::Unknown,
                String::from("unable to parse pattern")
            ))
        },
    ))
}

// RelativePathExpr ::= StepExpr (('/' | '//') StepExpr)*
fn relativepath_expr_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<N>> + 'a> {
    Box::new(map(
        pair(
            step_expr_pattern::<N>(),
            many0(tuple2(
                alt2(
                    map(tuple3(xpwhitespace(), tag("//"), xpwhitespace()), |_| "//"),
                    map(tuple3(xpwhitespace(), tag("/"), xpwhitespace()), |_| "/"),
                ),
                step_expr_pattern::<N>(),
            )),
        ),
        |(a, b)| {
            if b.is_empty() {
                // this is the terminal step
                a
            } else {
                let mut ap = match a {
                    Pattern::Selection(p) => p,
                    _ => panic!("relative path may only contain steps"),
                };
                for (_c, d) in b {
                    match d {
                        Pattern::Selection(mut p) => {
                            p.next = Some(Rc::new(ap));
                            ap = p.clone();
                        }
                        _ => panic!("relative path can only contain steps"),
                    }
                }
                Pattern::Selection(ap)
            }
        },
    ))
}

// StepExprP ::= PostfixExprExpr | AxisStepP
fn step_expr_pattern<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<N>> + 'a>
{
    Box::new(alt2(postfix_expr_pattern::<N>(), axis_step_pattern::<N>()))
}

// PostfixExprP ::= ParenthesizedExprP PredicateList
// TODO: predicates
fn postfix_expr_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<N>> + 'a> {
    Box::new(map(
        tuple2(paren_expr_pattern(), predicate_list::<N>()),
        |(p, _)| p,
    ))
}

// ParenthesizedExprP ::= "(" UnionExprP ")"
fn paren_expr_pattern<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<N>> + 'a>
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
fn axis_step_pattern<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<N>> + 'a>
{
    Box::new(map(
        tuple2(forward_step_pattern(), predicate_list::<N>()),
        |(f, _p)| f, // TODO: pass predicate back to caller
    ))
}

// ForwardStepP ::= (ForwardAxisP NodeTest) | AbbrevForwardStep
// Returns the node test, the terminal axis and the non-terminal axis
// TODO: abbreviated step
fn forward_step_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<N>> + 'a> {
    Box::new(map(
        tuple2(forward_axis_pattern(), nodetest()),
        |((a, c), nt)| Pattern::Selection(PathBuilder::new().step(a, c, nt).build()),
    ))
}

// ForwardAxisP ::= ("child" | "descendant" | "attribute" | "self" | "descendant-or-self" | "namespace" ) "::"
// Returns a pair: the axis to match this step, and the axis for the previous step
// TODO: abbreviated step
fn forward_axis_pattern() -> Box<dyn Fn(ParseInput) -> ParseResult<(Axis, Axis)>> {
    Box::new(map(
        tuple2(
            alt6(
                map(tag("child"), |_| (Axis::SelfAxis, Axis::Parent)),
                map(tag("descendant"), |_| (Axis::SelfAxis, Axis::Ancestor)),
                map(tag("attribute"), |_| (Axis::SelfAttribute, Axis::Parent)),
                map(tag("self"), |_| (Axis::SelfAxis, Axis::SelfAxis)),
                map(tag("descendant-or-self"), |_| {
                    (Axis::SelfAxis, Axis::Ancestor)
                }),
                map(tag("namespace"), |_| (Axis::SelfNamespace, Axis::Parent)),
            ),
            tag("::"),
        ),
        |(a, _)| a,
    ))
}

// FunctionCallP ::= OuterFunctionName ArgumentListP
fn function_call_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<N>> + 'a> {
    Box::new(map(
        tuple2(outer_function_name(), argument_list_pattern::<N>()),
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
fn argument_list_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<N>> + 'a> {
    Box::new(map(
        tuple3(
            map(tuple3(xpwhitespace(), tag("("), xpwhitespace()), |_| ()),
            separated_list0(
                map(tuple3(xpwhitespace(), tag(","), xpwhitespace()), |_| ()),
                argument_pattern::<N>(),
            ),
            map(tuple3(xpwhitespace(), tag(")"), xpwhitespace()), |_| ()),
        ),
        |(_, _a, _)| {
            // TODO
            Pattern::Error(Error::new(
                ErrorKind::NotImplemented,
                String::from("argument list in a pattern has not been implemented"),
            ))
        },
    ))
}

// ArgumentP ::= VarRef | Literal
fn argument_pattern<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<N>> + 'a> {
    Box::new(alt2(
        variable_reference_pattern::<N>(),
        literal_pattern::<N>(),
    ))
}

// literal
fn literal_pattern<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Pattern<N>> + 'a> {
    Box::new(map(literal::<N>(), |l| Pattern::Predicate(l)))
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
        map(qualname_test(), |q| q),
    ))
}
