//! Functions that produces nodes, or sets of nodes.

use crate::item::Node;
use crate::parser::combinators::alt::{alt2, alt3, alt4, alt5};
//use crate::parser::combinators::debug::inspect;
use crate::parser::combinators::list::separated_list1;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::pair::pair;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::{tuple2, tuple3};
use crate::parser::combinators::whitespace::xpwhitespace;
use crate::parser::xpath::expressions::postfix_expr;
use crate::parser::xpath::nodetests::{kindtest, nodetest};
use crate::parser::xpath::predicates::predicate_list;
use crate::parser::xpath::types::instanceof_expr;
use crate::parser::{ParseError, ParseInput, StaticState};
use crate::transform::{
    Axis, KindTest, NameTest, NodeMatch, NodeTest, Transform, WildcardOrName,
    WildcardOrNamespaceUri,
};
use qualname::{NamespacePrefix, NamespaceUri};

// UnionExpr ::= IntersectExceptExpr ( ('union' | '|') IntersectExceptExpr)*
pub(crate) fn union_expr<'a, N: Node + 'a, L>() -> Box<
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
        separated_list1(
            map(
                tuple3(xpwhitespace(), alt2(tag("union"), tag("|")), xpwhitespace()),
                |_| (),
            ),
            intersectexcept_expr::<N, L>(),
        ),
        |mut v| {
            if v.len() == 1 {
                v.pop().unwrap()
            } else {
                Transform::Union(v)
            }
        },
    ))
}

// IntersectExceptExpr ::= InstanceOfExpr ( ('intersect' | 'except') InstanceOfExpr)*
fn intersectexcept_expr<'a, N: Node + 'a, L>() -> Box<
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
        pair(
            instanceof_expr::<N, L>(),
            many0(tuple2(
                tuple3(
                    xpwhitespace(),
                    alt2(tag("intersect"), tag("except")),
                    xpwhitespace(),
                ),
                instanceof_expr::<N, L>(),
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

pub(crate) fn path_expr<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(alt3(
        absolutedescendant_expr::<N, L>(),
        absolutepath_expr::<N, L>(),
        relativepath_expr::<N, L>(),
    ))
}

// ('//' RelativePathExpr?)
fn absolutedescendant_expr<'a, N: Node + 'a, L>() -> Box<
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
        pair(tag("//"), relativepath_expr::<N, L>()),
        |(_, r)| {
            Transform::Compose(vec![
                Transform::Step(NodeMatch {
                    axis: Axis::DescendantOrSelfOrRoot,
                    nodetest: NodeTest::Name(NameTest::Wildcard(
                        WildcardOrNamespaceUri::Wildcard,
                        WildcardOrName::Wildcard,
                    )),
                }),
                r,
            ])
        },
    ))
}

// ('/' RelativePathExpr?)
fn absolutepath_expr<'a, N: Node + 'a, L>() -> Box<
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
        pair(tag("/"), opt(relativepath_expr::<N, L>())),
        |(_, r)| match r {
            Some(a) => Transform::Compose(vec![Transform::Root, a]),
            None => Transform::Root,
        },
    ))
}

// RelativePathExpr ::= StepExpr (('/' | '//') StepExpr)*
fn relativepath_expr<'a, N: Node + 'a, L>() -> Box<
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
        pair(
            step_expr::<N, L>(),
            many0(tuple2(
                alt2(
                    map(tuple3(xpwhitespace(), tag("//"), xpwhitespace()), |_| "//"),
                    map(tuple3(xpwhitespace(), tag("/"), xpwhitespace()), |_| "/"),
                ),
                step_expr::<N, L>(),
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
                                nodetest: NodeTest::Name(NameTest::Wildcard(
                                    WildcardOrNamespaceUri::Wildcard,
                                    WildcardOrName::Wildcard,
                                )),
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
fn step_expr<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(alt4(
        abbreviated_parent::<N, L>(),
        abbreviated_kindtest::<N, L>(),
        postfix_expr::<N, L>(),
        axisstep::<N, L>(),
    ))
}

// AxisStep ::= (ReverseStep | ForwardStep) PredicateList
fn axisstep<'a, N: Node + 'a, L>() -> Box<
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
        pair(
            alt2(
                pair(alt2(forwardaxis(), reverseaxis()), nodetest()),
                pair(abbreviated_axisstep(), nodetest()),
            ),
            predicate_list(),
        ),
        |((a, n), pl)| {
            Transform::Compose(vec![
                Transform::Step(NodeMatch {
                    axis: Axis::from(a),
                    nodetest: n,
                }),
                pl,
            ])
        },
    ))
}

fn abbreviated_parent<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(tag(".."), |_| {
        Transform::Step(NodeMatch::new(Axis::Parent, NodeTest::Kind(KindTest::Any)))
    }))
}
fn abbreviated_kindtest<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(pair(abbreviated_axisstep(), kindtest()), |(a, n)| {
        Transform::Step(NodeMatch {
            axis: Axis::from(a),
            nodetest: n,
        })
    }))
}

fn abbreviated_axisstep<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, &'static str), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    Box::new(no_input("child"))
}
pub fn no_input<'a, A: Clone + 'a, N: Node, L>(
    val: A,
) -> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, A), ParseError> + 'a
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |input, _ss| Ok((input, val.clone()))
}
// ForwardAxis ::= ('child' | 'descendant' | 'attribute' | 'self' | 'descendant-or-self' | 'following-sibling' | 'following' | 'namespace') '::'
fn forwardaxis<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, &'static str), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(alt2(
        //    alt8(
        map(
            pair(
                // need alt8
                alt2(
                    alt4(
                        map(tag("child"), |_| "child"),
                        map(tag("descendant-or-self"), |_| "descendant-or-self"),
                        map(tag("descendant"), |_| "descendant"),
                        map(tag("attribute"), |_| "attribute"),
                    ),
                    alt4(
                        map(tag("self"), |_| "self"),
                        map(tag("following-sibling"), |_| "following-sibling"),
                        map(tag("following"), |_| "following"),
                        map(tag("namespace"), |_| "namespace"),
                    ),
                ),
                tag("::"),
            ),
            |(a, _)| a,
        ),
        map(tag("@"), |_| "attribute"),
    ))
}

// ReverseAxis ::= ('parent' | 'ancestor' | 'ancestor-or-self' | 'preceding-sibling' | 'preceding' ) '::'
fn reverseaxis<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, &'static str), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(
        //    alt8(
        pair(
            // need alt8
            alt5(
                map(tag("parent"), |_| "parent"),
                map(tag("ancestor-or-self"), |_| "ancestor-or-self"),
                map(tag("ancestor"), |_| "ancestor"),
                map(tag("preceding-sibling"), |_| "preceding-sibling"),
                map(tag("preceding"), |_| "preceding"),
            ),
            tag("::"),
        ),
        |(a, _)| a,
    ))
}
