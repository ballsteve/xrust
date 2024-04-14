//! Functions that produces nodes, or sets of nodes.

use crate::item::Node;
use crate::parser::combinators::alt::{alt2, alt3, alt4, alt5};
use crate::parser::combinators::list::separated_list1;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::pair::pair;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::{tuple2, tuple3};
use crate::parser::combinators::whitespace::xpwhitespace;
use crate::parser::{ParseError, ParseInput};
//use crate::parser::combinators::debug::inspect;
use crate::parser::xpath::expressions::postfix_expr;
use crate::parser::xpath::nodetests::nodetest;
use crate::parser::xpath::predicates::predicate_list;
use crate::parser::xpath::types::instanceof_expr;
use crate::transform::{Axis, NameTest, NodeMatch, NodeTest, Transform, WildcardOrName};

// UnionExpr ::= IntersectExceptExpr ( ('union' | '|') IntersectExceptExpr)*
pub(crate) fn union_expr<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a> {
    Box::new(map(
        separated_list1(
            map(
                tuple3(xpwhitespace(), alt2(tag("union"), tag("|")), xpwhitespace()),
                |_| (),
            ),
            intersectexcept_expr::<N>(),
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
fn intersectexcept_expr<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a> {
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

pub(crate) fn path_expr<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a> {
    Box::new(alt3(
        absolutedescendant_expr::<N>(),
        absolutepath_expr::<N>(),
        relativepath_expr::<N>(),
    ))
}

// ('//' RelativePathExpr?)
fn absolutedescendant_expr<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a> {
    Box::new(map(pair(tag("//"), relativepath_expr::<N>()), |(_, r)| {
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
    }))
}

// ('/' RelativePathExpr?)
fn absolutepath_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a>
{
    Box::new(map(
        pair(tag("/"), opt(relativepath_expr::<N>())),
        |(_, r)| match r {
            Some(a) => Transform::Compose(vec![Transform::Root, a]),
            None => Transform::Root,
        },
    ))
}

// RelativePathExpr ::= StepExpr (('/' | '//') StepExpr)*
fn relativepath_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a>
{
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
fn step_expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a> {
    Box::new(alt2(postfix_expr::<N>(), axisstep::<N>()))
}

// AxisStep ::= (ReverseStep | ForwardStep) PredicateList
fn axisstep<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a> {
    Box::new(map(
        pair(
            pair(alt2(forwardaxis(), reverseaxis()), nodetest()),
            predicate_list()
        ),
        |((a, n), pl)| {
            Transform::Compose(vec![
                Transform::Step(NodeMatch {
                    axis: Axis::from(a),
                    nodetest: n,
                }),
                pl
            ])
        },
    ))
}

// ForwardAxis ::= ('child' | 'descendant' | 'attribute' | 'self' | 'descendant-or-self' | 'following-sibling' | 'following' | 'namespace') '::'
fn forwardaxis<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, &'static str), ParseError> + 'a> {
    Box::new(map(
        //    alt8(
        pair(
            // need alt8
            alt2(
                alt4(
                    map(tag("child"), |_| "child"),
                    map(tag("descendant"), |_| "descendant"),
                    map(tag("descendant-or-self"), |_| "descendant-or-self"),
                    map(tag("attribute"), |_| "attribute"),
                ),
                alt4(
                    map(tag("self"), |_| "self"),
                    map(tag("following"), |_| "following"),
                    map(tag("following-sibling"), |_| "following-sibling"),
                    map(tag("namespace"), |_| "namespace"),
                ),
            ),
            tag("::"),
        ),
        |(a, _)| a,
    ))
}

// ReverseAxis ::= ('parent' | 'ancestor' | 'ancestor-or-self' | 'preceding-sibling' | 'preceding' ) '::'
fn reverseaxis<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, &'static str), ParseError> + 'a> {
    Box::new(map(
        //    alt8(
        pair(
            // need alt8
            alt5(
                map(tag("parent"), |_| "parent"),
                map(tag("ancestor"), |_| "ancestor"),
                map(tag("ancestor-or-self"), |_| "ancestor-or-self"),
                map(tag("preceding"), |_| "preceding"),
                map(tag("preceding-sibling"), |_| "preceding-sibling"),
            ),
            tag("::"),
        ),
        |(a, _)| a,
    ))
}
