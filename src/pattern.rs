/*! # Support for XPath patterns.

This module provides both a parser to compile a [Pattern], and an interpreter to determine if an item matches a compiled pattern.

Patterns are defined in XSLT 3.0 5.5.2.

A string can be compiled as [Pattern] by using the ```try_from``` associated function.

```rust
# use xrust::item::Node;
use xrust::pattern::Pattern;

# fn compile<N: Node>() {
let p: Pattern<'i, I, N> = Pattern::try_from("child::foobar")
        .expect("unable to compile pattern");
# ()
# }
```

An [Item] can then be tested to see if it matches the [Pattern]. To do that, it is necessary to have a transformation [Context].

```rust
# use std::rc::Rc;
# use xrust::ErrorKind;
# use xrust::xdmerror::Error;
# use xrust::item::{Item, NodeType};
# use xrust::pattern::Pattern;
# use xrust::transform::context::{Context, StaticContext, StaticContextBuilder};
# use xrust::Node;
# use xrust::trees::smite::RNode;
# type F = Box<dyn FnMut(&str) -> Result<(), Error>>;
let p = Pattern::try_from("/").expect("unable to compile pattern");
let n = Item::Node(RNode::new_document());

// Create a static context
let mut static_context = StaticContextBuilder::new()
    .message(|_| Ok(()))
    .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
    .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
   .build();

// This pattern matches the root node
assert_eq!(p.matches(&Context::new(), &mut static_context, &n), true)
```

```rust
# use std::rc::Rc;
# use xrust::xdmerror::{Error, ErrorKind};
# use xrust::item::{Item, NodeType};
# use xrust::pattern::Pattern;
# use xrust::transform::context::{Context, StaticContext, StaticContextBuilder};
# use xrust::Node;
# use xrust::trees::smite::RNode;
# type F = Box<dyn FnMut(&str) -> Result<(), Error>>;
let p = Pattern::try_from("child::foobar").expect("unable to compile pattern");
let n = Item::Node(RNode::new_document());
// Create a static context
# let mut static_context = StaticContextBuilder::new()
#    .message(|_| Ok(()))
#    .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
#    .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
#   .build();
// This pattern will not match because "n" is not an element named "foobar"
assert_eq!(p.matches(&Context::new(), &mut static_context, &n), false)
```

*/

use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use url::Url;

use crate::item::{Item, Node, NodeType, Sequence, SequenceTrait};
use crate::parser::combinators::whitespace::xpwhitespace;
use crate::parser::xpath::literals::literal;
use crate::parser::xpath::nodetests::{nodetest, qualname_test};
use crate::parser::xpath::predicates::predicate_list;
use crate::parser::xpath::variables::variable_reference;
use crate::qname::Interner;
use crate::transform::context::{Context, ContextBuilder, StaticContext};
use crate::transform::{Axis, KindTest, NameTest, NodeTest, Transform, WildcardOrName};
use crate::value::Value;
use crate::xdmerror::{Error, ErrorKind};

use crate::parser::combinators::alt::{alt2, alt4, alt6};
//use crate::parser::combinators::debug::inspect;
use crate::parser::combinators::list::{separated_list0, separated_list1};
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::pair::pair;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::{tuple2, tuple3};
use crate::parser::{ParseError, ParseInput, ParserState};

/// An XPath pattern. A pattern most frequently appears as the value of a match attribute.
/// A pattern is either a predicate pattern or a selection pattern.
///
/// A predicate pattern matches the current item if all of the predicates evaluate to true.
///
/// A selection pattern is subset of XPath path expressions.
#[derive(Clone)]
pub enum Pattern<'i, I: Interner, N: Node> {
    Predicate(Transform<'i, I, N>),
    Selection(Path),
    Error(Error),
}

impl<'i, I: Interner, N: Node> Pattern<'i, I, N> {
    /// Returns whether the Pattern is of type error.
    pub fn is_err(&self) -> bool {
        if let Pattern::Selection(s) = self {
            s.is_err()
        } else {
            matches!(self, Pattern::Error(_))
        }
    }
    pub fn get_err(&self) -> Option<Error> {
        if let Pattern::Selection(s) = self {
            s.get_err()
        } else if let Pattern::Error(e) = self {
            Some(e.clone())
        } else {
            None
        }
    }
    /// Returns whether the given item matches the pattern.
    /// TODO: return dynamic errors
    pub fn matches<
        F: FnMut(&str) -> Result<(), Error>,
        G: FnMut(&str) -> Result<N, Error>,
        H: FnMut(&Url) -> Result<String, Error>,
    >(
        &self,
        ctxt: &Context<'i, I, N>,
        stctxt: &mut StaticContext<N, F, G, H>,
        i: &Item<N>,
    ) -> bool {
        match self {
            Pattern::Predicate(t) => ContextBuilder::from(ctxt)
                .context(vec![i.clone()])
                .build()
                .dispatch(stctxt, t)
                .unwrap_or(vec![Item::Value(Rc::new(Value::from(false)))])
                .to_bool(),
            Pattern::Selection(p) => path_match::<I, N>(p, i),
            _ => false, // not yet implemented
        }
    }
    /// Find the NodeTest for the terminal step
    pub fn terminal_node_test(&self) -> (Axis, Axis, NodeTest) {
        if let Pattern::Selection(sel) = self {
            branch_terminal_node_test(sel)
        } else {
            (
                Axis::SelfDocument,
                Axis::SelfDocument,
                NodeTest::Kind(KindTest::Document),
            )
        }
    }
}

fn branch_terminal_node_test(b: &Branch) -> (Axis, Axis, NodeTest) {
    match b {
        Branch::SingleStep(t) => (t.terminal, t.non_terminal, t.nt.clone()),
        Branch::RelPath(r) => branch_terminal_node_test(&r[0]),
        Branch::Union(u) => branch_terminal_node_test(&u[0]), // TODO: should be all of the alternatives
        Branch::Error(_) => (
            Axis::SelfDocument,
            Axis::SelfDocument,
            NodeTest::Kind(KindTest::Document),
        ),
    }
}

// Entry point for matching a Pattern.
// The given Item is the initial context.
fn path_match<I: Interner, N: Node>(p: &Path, i: &Item<N>) -> bool {
    !branch_match::<I, N>(p, vec![i.clone()]).is_empty()
}
// Match a branch in the Pattern. Each Item in the Sequence is tested.
// This results in a new context.
fn branch_match<I: Interner, N: Node>(p: &Path, s: Sequence<N>) -> Sequence<N> {
    // First step is the terminal case,
    // next steps are non-terminal
    match p {
        Branch::SingleStep(t) => s
            .iter()
            .filter(|i| is_match::<I, N>(&t.terminal, &t.nt, i))
            .flat_map(|i| find_seq(&t.non_terminal, i))
            .collect(),
        Branch::RelPath(r) => {
            // A series of steps
            // Each step selects a new context for the next step
            r.iter().fold(s, |ctxt, b| {
                let new_ctxt = ctxt
                    .iter()
                    .cloned()
                    .flat_map(|i| branch_match::<I, N>(b, vec![i]))
                    .collect();
                new_ctxt
            })
        }
        Branch::Union(u) => {
            // If any match, then the whole matches
            u.iter()
                .flat_map(|b| {
                    s.iter()
                        .cloned()
                        .flat_map(|i| branch_match::<I, N>(b, vec![i]))
                        .collect::<Sequence<N>>()
                })
                .collect()
        }
        Branch::Error(_) => vec![],
    }
}

fn find_seq<N: Node>(a: &Axis, i: &Item<N>) -> Sequence<N> {
    match a {
        Axis::SelfDocument => match i {
            Item::Node(n) => {
                if n.node_type() == NodeType::Document {
                    vec![i.clone()]
                } else {
                    vec![]
                }
            }
            _ => vec![],
        },
        Axis::SelfAxis => vec![i.clone()],
        Axis::Parent => match i {
            Item::Node(n) => n.parent().map_or(vec![], |p| vec![Item::Node(p)]),
            _ => vec![],
        },
        _ => vec![], // todo
    }
}

fn is_match<I: Interner, N: Node>(a: &Axis, nt: &NodeTest, i: &Item<N>) -> bool {
    match a {
        Axis::SelfDocument => {
            // Select item only if it is a document-type node
            match i {
                Item::Node(n) => {
                    if n.node_type() == NodeType::Document {
                        nt.matches::<I, N>(i)
                    } else {
                        false
                    }
                }
                _ => false,
            }
        }
        Axis::SelfAxis => {
            // Select item if it is an element-type node
            nt.matches::<I, N>(i)
        }
        Axis::Parent => {
            // Select the parent node
            match i {
                Item::Node(n) => n
                    .parent()
                    .map_or(false, |p| nt.matches::<I, N>(&Item::Node(p))),
                _ => false,
            }
        }
        _ => false, // todo
    }
}

impl<'i, I: Interner, N: Node> Debug for Pattern<'i, I, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Pattern::Predicate(t) => write!(f, "Pattern::Predicate t==\"{:?}\"", t),
            Pattern::Selection(p) => write!(f, "Pattern::Selection path=\"{:?}\"", p),
            Pattern::Error(e) => write!(f, "Pattern::Error error=\"{:?}\"", e),
        }
    }
}

// A Path is a tree structure, but does not need to be mutable.
// It also does not need to be fully navigable.

// A Path is a Branch.
// A Branch::Union is caused by a union ("|") operator -
// if any of the union branches match then the Path matches.
// If the vector is empty then there is no match.
// A Rel(ative)Path is caused by the "/" character.
// The terminal case is a single Step.
pub type Path = Branch;
#[derive(Clone, Debug)]
pub enum Branch {
    SingleStep(Step),
    RelPath(Vec<Branch>),
    Union(Vec<Branch>),
    Error(Error),
}

impl Branch {
    pub fn terminal_node_test(&self) -> (Axis, Axis, NodeTest) {
        branch_terminal_node_test(self)
    }
    /// Check whether the Branch is an error or contains an error
    pub fn is_err(&self) -> bool {
        match self {
            Branch::Error(_) => true,
            Branch::SingleStep(_) => false,
            Branch::RelPath(r) => r.iter().any(|f| f.is_err()),
            Branch::Union(u) => u.iter().any(|f| f.is_err()),
        }
    }
    /// Get any error in the Branch
    pub fn get_err(&self) -> Option<Error> {
        match self {
            Branch::Error(e) => Some(e.clone()),
            Branch::SingleStep(_) => None,
            Branch::RelPath(r) => r.iter().fold(None, |v, f| v.or_else(|| f.get_err())),
            Branch::Union(u) => u.iter().fold(None, |v, f| v.or_else(|| f.get_err())),
        }
    }
}

// * == Branch::SingleStep(*)
// *|node() == Branch::Union(vec![Branch::SingleStep(*), Branch::SingleStep(node())])
// a/b == Branch::RelPath(vec![Branch::SingleStep(a), Branch::SingleStep(b)])
// a/b|c/d == Branch::Union(vec![
//   Branch::RelPath(vec![Branch::SingleStep(a),Branch::SingleStep(b)]),
//   Branch::RelPath(vec![Branch::SingleStep(c),Branch::SingleStep(d)]),
// ])
// a/(b|c)/d (matches a/b/d or a/c/d) == Branch::RelPath(vec![
//   Branch::SingleStep(a),
//   Branch::Union(vec![Branch::SingleStep(b),Branch::SingleStep(c)]),
//   Branch::SingleStep(d)
// ]
// a/ (b/c | (d/e|f/g)) / (h|i) |j == Branch::Union(vec![
//   Branch::RelPath(vec![
//     Branch::SingleStep(a),
//     Branch::RelPath(vec![
//       Branch::Union(vec![
//         Branch::RelPath(vec![Branch::SingleStep(b), Branch::SingleStep(c)]),
//         Branch::Union(vec![
//           Branch::RelPath(vec![Branch::SingleStep(d), Branch::SingleStep(e)]),
//           Branch::RelPath(vec![Branch::SingleStep(f), Branch::SingleStep(g)]),
//         ]),
//       ]),
//       Branch::Union(vec![
//         Branch::SingleStep(h),
//         Branch::SingleStep(i),
//       ])
//     ]),
//   ]),
//   Branch::SingleStep(j),
// ]

// A step in the Path consists of (terminal, non-terminal) axes and a NodeTest
// If this is the last step, then the terminal axis is used.
// Otherwise the non-terminal axis applies.
#[derive(Clone, Debug)]
pub struct Step {
    terminal: Axis,
    non_terminal: Axis,
    nt: NodeTest,
}

impl Step {
    pub fn new(terminal: Axis, non_terminal: Axis, nt: NodeTest) -> Self {
        Step {
            terminal,
            non_terminal,
            nt,
        }
    }
    pub fn get_ref(&self) -> (&Axis, &Axis, &NodeTest) {
        (&self.terminal, &self.non_terminal, &self.nt)
    }
}

/// Compile an XPath pattern.
impl<'a, 'i, I: Interner, N: Node> TryFrom<(&'a str, &'i I)> for Pattern<'i, I, N> {
    type Error = Error;
    fn try_from(
        e: (&'a str, &'i I),
    ) -> Result<Self, <crate::pattern::Pattern<'i, I, N> as TryFrom<(&'a str, &'i I)>>::Error> {
        if e.0.is_empty() {
            Err(Error::new(
                ErrorKind::TypeError,
                String::from("empty string is not allowed as an XPath pattern"),
            ))
        } else {
            let state = ParserState::new(None, None, None, e.1);
            match pattern::<I, N>((e.0, state)) {
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

/// Compile an XPath pattern. Uses the supplied [Node] to resolve in-scope XML Namespaces.
impl<'a, 'i, I: Interner, N: Node> TryFrom<(&'a str, N, &'i I)> for Pattern<'i, I, N> {
    type Error = Error;
    fn try_from(
        e: (&'a str, N, &'i I),
    ) -> Result<Self, <crate::pattern::Pattern<'i, I, N> as TryFrom<(&'a str, &'i I)>>::Error> {
        if e.0.is_empty() {
            Err(Error::new(
                ErrorKind::TypeError,
                String::from("empty string is not allowed as an XPath pattern"),
            ))
        } else {
            let state = ParserState::new(None, Some(e.1), None, e.2);
            match pattern::<I, N>((e.0, state)) {
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

/// Compile an XPath Pattern. XML Namespaces will not be resolved.
impl<'a, 'i, I: Interner, N: Node> TryFrom<(String, &'i I)> for Pattern<'i, I, N> {
    type Error = Error;
    fn try_from(
        e: (String, &'i I),
    ) -> Result<Self, <Pattern<'i, I, N> as TryFrom<(&'a str, &'i I)>>::Error> {
        Pattern::try_from((e.0.as_str(), e.1))
    }
}
impl<'a, 'i, I: Interner, N: Node> TryFrom<(String, N, &'i I)> for Pattern<'i, I, N> {
    type Error = Error;
    fn try_from(
        e: (String, N, &'i I),
    ) -> Result<Self, <Pattern<'i, I, N> as TryFrom<(&'a str, N, &'i I)>>::Error> {
        Pattern::try_from((e.0.as_str(), e.1, e.2))
    }
}

// Pattern30 ::= PredicatePattern | UnionExprP ;
fn pattern<'a, 'i: 'a, I: Interner + 'i, N: Node + 'a>(
    input: ParseInput<'a, 'i, I, N>,
) -> Result<(ParseInput<'a, 'i, I, N>, Pattern<'i, I, N>), ParseError> {
    alt2(predicate_pattern::<I, N>(), union_expr_pattern())(input)
}

// PredicatePattern ::= "." PredicateList
// Context must match all predicates
fn predicate_pattern<'a, 'i: 'a, I: Interner + 'i, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Pattern<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(map(
        pair(
            map(tuple3(xpwhitespace(), tag("."), xpwhitespace()), |_| ()),
            predicate_list::<I, N>(),
        ),
        |(_, p)| Pattern::Predicate(p),
    ))
}

// UnionExprP ::= IntersectExceptExprP (("union" | "|") IntersectExceptExprP)*
// A union expression matches if any of its components is a match. This creates a branching structure in the compilation of the Pattern<'i, I, N>.
fn union_expr_pattern<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Pattern<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(map(
        separated_list1(
            map(
                tuple3(xpwhitespace(), alt2(tag("union"), tag("|")), xpwhitespace()),
                |_| (),
            ),
            intersect_except_expr_pattern::<I, N>(),
        ),
        |v| {
            Pattern::Selection(Branch::Union(v))
            //            if v.len() == 1 {
            //                v.pop().unwrap()
            //            } else {
            //                Pattern::Selection(vec![v])
            //            }
        },
    ))
}

// NB. Rust *really* doesn't like recursive types, so we must force it to lazily evaluate arguments to avoid stack overflow.
fn union_expr_wrapper<'a, 'i: 'a, I: Interner, N: Node + 'a>(
    b: bool,
) -> Box<
    dyn Fn(
        ParseInput<'a, 'i, I, N>,
    ) -> Result<(ParseInput<'a, 'i, I, N>, Pattern<'i, I, N>), ParseError>,
> {
    Box::new(move |input| {
        if b {
            union_expr_pattern::<I, N>()(input)
        } else {
            noop()(input)
        }
    })
}

fn noop<'a, 'i, I: Interner, N: Node>() -> Box<
    dyn Fn(
        ParseInput<'a, 'i, I, N>,
    ) -> Result<(ParseInput<'a, 'i, I, N>, Pattern<'i, I, N>), ParseError>,
> {
    Box::new(move |_| Err(ParseError::Combinator))
}

// IntersectExceptExprP ::= PathExprP (("intersect" | "except") PathExprP)*
// intersect and except not yet supported
fn intersect_except_expr_pattern<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Path), ParseError> + 'a,
> {
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
            path_expr_pattern::<I, N>(),
        ),
        |mut v| {
            if v.len() == 1 {
                v.pop().unwrap()
            } else {
                // intersect/except not implemented
                Branch::Error(Error::new(
                    ErrorKind::NotImplemented,
                    String::from("intersect or except in a pattern has not been implemented"),
                ))
            }
        },
    ))
}

// PathExprP ::= RootedPath | ("/" RelativePathExprP) | ("//" RelativePathExprP) | RelativePathExprP
fn path_expr_pattern<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Path), ParseError> + 'a,
> {
    Box::new(alt4(
        rooted_path_pattern::<I, N>(),
        absolutedescendant_expr_pattern(),
        absolutepath_expr_pattern(),
        relativepath_expr_pattern::<I, N>(),
    ))
}

// RootedPath ::= (VarRef | FunctionCallP) PredicateList (("/" | "//") RelativePathExprP)?
fn rooted_path_pattern<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Path), ParseError> + 'a,
> {
    Box::new(map(
        tuple3(
            alt2(
                variable_reference_pattern::<I, N>(),
                function_call_pattern(),
            ),
            predicate_list::<I, N>(),
            alt2(
                absolutedescendant_expr_pattern::<I, N>(),
                absolutepath_expr_pattern(),
            ),
        ),
        |(_a, _b, _c)| {
            Branch::Error(Error::new(
                ErrorKind::NotImplemented,
                String::from("rooted path in a pattern has not been implemented"),
            ))
        },
    ))
}

// Variable Reference
fn variable_reference_pattern<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Path), ParseError> + 'a,
> {
    Box::new(map(variable_reference::<I, N>(), |_| {
        Branch::Error(Error::new(
            ErrorKind::NotImplemented,
            "variable reference not yet supported",
        ))
    }))
}

// ('//' RelativePathExpr?)
fn absolutedescendant_expr_pattern<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Path), ParseError> + 'a,
> {
    Box::new(map(
        pair(tag("//"), relativepath_expr_pattern::<I, N>()),
        |(_, _r)| {
            Branch::Error(Error::new(
                ErrorKind::NotImplemented,
                String::from("absolute descendant path in a pattern has not been implemented"),
            ))
        },
    ))
}

// ('/' RelativePathExpr?)
fn absolutepath_expr_pattern<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Path), ParseError> + 'a,
> {
    Box::new(map(
        pair(
            map(tag("/"), |_| "/"),
            opt(relativepath_expr_pattern::<I, N>()),
        ),
        |(d, r)| match (d, r.clone()) {
            ("/", None) => {
                // Matches the root node
                Branch::SingleStep(Step::new(
                    Axis::SelfDocument,
                    Axis::SelfDocument,
                    NodeTest::Kind(KindTest::Document),
                ))
            }
            ("/", Some(Branch::SingleStep(s))) => Branch::RelPath(vec![
                Branch::SingleStep(s),
                Branch::SingleStep(Step::new(
                    Axis::SelfDocument,
                    Axis::SelfDocument,
                    NodeTest::Kind(KindTest::Document),
                )),
            ]),
            ("/", Some(Branch::RelPath(mut a))) => {
                /*a.insert(
                    0,
                    Branch::SingleStep(Step::new(
                        Axis::SelfDocument,
                        Axis::SelfDocument,
                        NodeTest::Kind(KindTest::Document),
                    )),
                );*/
                a.push(Branch::SingleStep(Step::new(
                    Axis::SelfDocument,
                    Axis::SelfDocument,
                    NodeTest::Kind(KindTest::Document),
                )));
                Branch::RelPath(a)
            }
            ("/", Some(Branch::Union(u))) => Branch::RelPath(vec![
                Branch::Union(u),
                Branch::SingleStep(Step::new(
                    Axis::SelfDocument,
                    Axis::SelfDocument,
                    NodeTest::Kind(KindTest::Document),
                )),
            ]),
            _ => Branch::Error(Error::new(
                ErrorKind::Unknown,
                String::from("unable to parse pattern"),
            )),
        },
    ))
}

// RelativePathExpr ::= StepExpr (('/' | '//') StepExpr)*
fn relativepath_expr_pattern<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Path), ParseError> + 'a,
> {
    Box::new(map(
        pair(
            step_expr_pattern::<I, N>(),
            many0(tuple2(
                alt2(
                    map(tuple3(xpwhitespace(), tag("//"), xpwhitespace()), |_| "//"),
                    map(tuple3(xpwhitespace(), tag("/"), xpwhitespace()), |_| "/"),
                ),
                step_expr_pattern::<I, N>(),
            )),
        ),
        |(a, b)| {
            if b.is_empty() {
                // this is the terminal step
                a
            } else {
                // TODO: handle "//" separator
                let mut result = vec![a];
                for (_c, d) in b {
                    result.insert(0, d);
                }
                Branch::RelPath(result)
            }
        },
    ))
}

// StepExprP ::= PostfixExprExpr | AxisStepP
fn step_expr_pattern<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Path), ParseError> + 'a,
> {
    Box::new(alt2(
        postfix_expr_pattern::<I, N>(),
        axis_step_pattern::<I, N>(),
    ))
}

// PostfixExprP ::= ParenthesizedExprP PredicateList
// TODO: predicates
fn postfix_expr_pattern<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Path), ParseError> + 'a,
> {
    Box::new(map(
        tuple2(paren_expr_pattern(), predicate_list::<I, N>()),
        |(p, _)| p,
    ))
}

// ParenthesizedExprP ::= "(" UnionExprP ")"
fn paren_expr_pattern<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Path), ParseError> + 'a,
> {
    Box::new(map(
        tuple3(
            tuple3(xpwhitespace(), tag("("), xpwhitespace()),
            union_expr_wrapper(true),
            tuple3(xpwhitespace(), tag(")"), xpwhitespace()),
        ),
        |(_, u, _)| {
            if let Pattern::Selection(sel) = u {
                sel
            } else {
                Branch::Error(Error::new(
                    ErrorKind::TypeError,
                    "expression must be a selection",
                ))
            }
        },
    ))
}

// AxisStepP ::= ForwardStepP PredicateList
// TODO: predicate
fn axis_step_pattern<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Path), ParseError> + 'a,
> {
    Box::new(map(
        tuple2(forward_step_pattern(), predicate_list::<I, N>()),
        |(f, _p)| f, // TODO: pass predicate back to caller
    ))
}

// ForwardStepP ::= (ForwardAxisP NodeTest) | AbbrevForwardStep
// Returns the node test, the terminal axis and the non-terminal axis
fn forward_step_pattern<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Path), ParseError> + 'a,
> {
    Box::new(map(
        alt2(
            tuple2(forward_axis_pattern(), nodetest()),
            abbrev_forward_step(),
        ),
        |((a, c), nt)| Branch::SingleStep(Step::new(a, c, nt)),
    ))
}

// AbbrevForwardStep ::= "@"? NodeTest
fn abbrev_forward_step<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, ((Axis, Axis), NodeTest)), ParseError>
        + 'a,
> {
    Box::new(map(tuple2(opt(tag("@")), nodetest()), |(a, nt)| {
        a.map_or_else(
            || {
                // not an attribute
                ((Axis::SelfAxis, Axis::Parent), nt.clone())
            },
            |_| {
                // attribute
                ((Axis::SelfAttribute, Axis::Parent), nt.clone())
            },
        )
    }))
}

// ForwardAxisP ::= ("child" | "descendant" | "attribute" | "self" | "descendant-or-self" | "namespace" ) "::"
// Returns a pair: the axis to match this step, and the axis for the previous step
fn forward_axis_pattern<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, (Axis, Axis)), ParseError>
        + 'a,
> {
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
fn function_call_pattern<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Path), ParseError> + 'a,
> {
    Box::new(map(
        tuple2(outer_function_name(), argument_list_pattern::<I, N>()),
        |(_n, _a)| {
            // TODO
            Branch::Error(Error::new(
                ErrorKind::NotImplemented,
                String::from("function call in a pattern has not been implemented"),
            ))
        },
    ))
}

// ArgumentListP ::= "(" (ArgumentP ("," ArgumentP)*)? ")"
fn argument_list_pattern<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Path), ParseError> + 'a,
> {
    Box::new(map(
        tuple3(
            map(tuple3(xpwhitespace(), tag("("), xpwhitespace()), |_| ()),
            separated_list0(
                map(tuple3(xpwhitespace(), tag(","), xpwhitespace()), |_| ()),
                argument_pattern::<I, N>(),
            ),
            map(tuple3(xpwhitespace(), tag(")"), xpwhitespace()), |_| ()),
        ),
        |(_, _a, _)| {
            // TODO
            Branch::Error(Error::new(
                ErrorKind::NotImplemented,
                String::from("argument list in a pattern has not been implemented"),
            ))
        },
    ))
}

// ArgumentP ::= VarRef | Literal
fn argument_pattern<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Path), ParseError> + 'a,
> {
    Box::new(alt2(
        variable_reference_pattern::<I, N>(),
        literal_pattern::<I, N>(),
    ))
}

// literal
fn literal_pattern<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Path), ParseError> + 'a,
> {
    Box::new(map(literal::<I, N>(), |_| {
        Branch::Error(Error::new(ErrorKind::NotImplemented, "not yet implemented"))
    }))
}

// OuterFunctionName ::= "doc" | "id" | "element-with-id" | "key" | "root" | URIQualifiedName
fn outer_function_name<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, NodeTest), ParseError>
        + 'a,
> {
    Box::new(alt6(
        map(tag("doc"), |_| {
            NodeTest::Name(NameTest {
                ns: None,
                prefix: None,
                name: Some(WildcardOrName::Name(Rc::new(Value::from("doc")))),
            })
        }),
        map(tag("id"), |_| {
            NodeTest::Name(NameTest {
                ns: None,
                prefix: None,
                name: Some(WildcardOrName::Name(Rc::new(Value::from("id")))),
            })
        }),
        map(tag("element-with-id"), |_| {
            NodeTest::Name(NameTest {
                ns: None,
                prefix: None,
                name: Some(WildcardOrName::Name(Rc::new(Value::from(
                    "element-with-id",
                )))),
            })
        }),
        map(tag("key"), |_| {
            NodeTest::Name(NameTest {
                ns: None,
                prefix: None,
                name: Some(WildcardOrName::Name(Rc::new(Value::from("key")))),
            })
        }),
        map(tag("root"), |_| {
            NodeTest::Name(NameTest {
                ns: None,
                prefix: None,
                name: Some(WildcardOrName::Name(Rc::new(Value::from("root")))),
            })
        }),
        map(qualname_test(), |q| q),
    ))
}
