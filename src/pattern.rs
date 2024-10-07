/*! # Support for XPath patterns.

This module provides both a parser to compile a [Pattern], and an interpreter to determine if an item matches a compiled pattern.

Patterns are defined in XSLT 3.0 5.5.2.

A string can be compiled as [Pattern] by using the ```try_from``` associated function.

```rust
# use xrust::item::Node;
use xrust::pattern::Pattern;

# fn compile<N: Node>() {
let p: Pattern<N> = Pattern::try_from("child::foobar")
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
use crate::parser::{ParseError, ParseInput, ParserState};
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
    /// Returns whether the Pattern is of type error.
    pub fn is_err(&self) -> bool {
        match self {
            Pattern::Error(_) => true,
            _ => false,
        }
    }
    pub fn get_err(&self) -> Option<Error> {
        if let Pattern::Error(e) = self {
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
        ctxt: &Context<N>,
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
            Pattern::Selection(p) => {
                // First step is the terminal case,
                // next steps are non-terminal
                let mut pit = p.iter();
                pit.next().map_or(false, |q| {
                    if is_match(&q.terminal, &q.nt, i) {
                        let mut seq: Sequence<N> = find_seq(&q.non_terminal, i);
                        loop {
                            if let Some(s) = pit.next() {
                                let new_seq = seq
                                    .iter()
                                    .filter(|f| is_match(&s.terminal, &s.nt, f))
                                    .fold(vec![], |mut acc, m| {
                                        let mut new_seq = find_seq(&s.non_terminal, m);
                                        acc.append(&mut new_seq);
                                        acc
                                    });
                                seq = new_seq;
                            } else {
                                break;
                            }
                        }
                        !seq.is_empty()
                    } else {
                        false
                    }
                })
            }
            _ => false, // not yet implemented
        }
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

fn is_match<N: Node>(a: &Axis, nt: &NodeTest, i: &Item<N>) -> bool {
    match a {
        Axis::SelfDocument => {
            // Select item only if it is a document-type node
            match i {
                Item::Node(n) => {
                    if n.node_type() == NodeType::Document {
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
            match i {
                Item::Node(n) => n.parent().map_or(false, |p| nt.matches(&Item::Node(p))),
                _ => false,
            }
        }
        _ => false, // todo
    }
}

impl<N: Node> Debug for Pattern<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Pattern::Predicate(t) => write!(f, "Pattern::Predicate t==\"{:?}\"", t),
            Pattern::Selection(p) => write!(f, "Pattern::Selection path=\"{:?}\"", p),
            Pattern::Error(e) => write!(f, "Pattern::Error error=\"{:?}\"", e),
        }
    }
}

//    Each step in the Path consists of (terminal, non-terminal) axes and a NodeTest
// If this is the last step, then the terminal axis is used.
// Otherwise the non-terminal axis applies.
pub type Path = Vec<Step>;

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
impl<N: Node> TryFrom<&str> for Pattern<N> {
    type Error = Error;
    fn try_from(e: &str) -> Result<Self, <crate::pattern::Pattern<N> as TryFrom<&str>>::Error> {
        if e.is_empty() {
            Err(Error::new(
                ErrorKind::TypeError,
                String::from("empty string is not allowed as an XPath pattern"),
            ))
        } else {
            let state = ParserState::new(None, None, None);
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

/// Compile an XPath pattern. Uses the supplied [Node] to resolve in-scope XML Namespaces.
impl<N: Node> TryFrom<(&str, N)> for Pattern<N> {
    type Error = Error;
    fn try_from(e: (&str, N)) -> Result<Self, <crate::pattern::Pattern<N> as TryFrom<&str>>::Error> {
        if e.0.is_empty() {
            Err(Error::new(
                ErrorKind::TypeError,
                String::from("empty string is not allowed as an XPath pattern"),
            ))
        } else {
            let state = ParserState::new(None, Some(e.1), None);
            match pattern::<N>((e.0, state)) {
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
    fn try_from(e: String) -> Result<Self, <Pattern<N> as TryFrom<&'a str>>::Error> {
        Pattern::try_from(e.as_str())
    }
}
impl<'a, N: Node> TryFrom<(String, N)> for Pattern<N> {
    type Error = Error;
    fn try_from(e: (String, N)) -> Result<Self, <Pattern<N> as TryFrom<(&'a str, N)>>::Error> {
        Pattern::try_from((e.0.as_str(), e.1))
    }
}

// Pattern30 ::= PredicatePattern | UnionExprP ;
fn pattern<N: Node>(input: ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError> {
    alt2(predicate_pattern::<N>(), union_expr_pattern())(input)
}

// PredicatePattern ::= "." PredicateList
// Context must match all predicates
fn predicate_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError> + 'a> {
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
fn union_expr_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError> + 'a> {
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
fn union_expr_wrapper<N: Node>(
    b: bool,
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError>> {
    Box::new(move |input| {
        if b {
            union_expr_pattern::<N>()(input)
        } else {
            noop()(input)
        }
    })
}

fn noop<N: Node>() -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError>>
{
    Box::new(move |_| Err(ParseError::Combinator))
}

// IntersectExceptExprP ::= PathExprP (("intersect" | "except") PathExprP)*
// intersect and except not yet supported
fn intersect_except_expr_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError> + 'a> {
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
fn path_expr_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError> + 'a> {
    Box::new(alt4(
        rooted_path_pattern::<N>(),
        absolutedescendant_expr_pattern(),
        absolutepath_expr_pattern(),
        relativepath_expr_pattern::<N>(),
    ))
}

// RootedPath ::= (VarRef | FunctionCallP) PredicateList (("/" | "//") RelativePathExprP)?
fn rooted_path_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError> + 'a> {
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
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError> + 'a> {
    Box::new(map(variable_reference::<N>(), |r| Pattern::Predicate(r)))
}

// ('//' RelativePathExpr?)
fn absolutedescendant_expr_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError> + 'a> {
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
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError> + 'a> {
    Box::new(map(
        pair(
            map(tag("/"), |_| "/"),
            opt(relativepath_expr_pattern::<N>()),
        ),
        |(d, r)| match (d, r) {
            ("/", None) => {
                // Matches the root node
                Pattern::Selection(vec![Step::new(
                    Axis::SelfDocument,
                    Axis::SelfDocument,
                    NodeTest::Kind(KindTest::Document),
                )])
            }
            ("/", Some(a)) => {
                if let Pattern::Selection(mut b) = a {
                    b.push(Step::new(
                        Axis::SelfDocument,
                        Axis::SelfDocument,
                        NodeTest::Kind(KindTest::Document),
                    ));
                    Pattern::Selection(b)
                } else {
                    panic!("pattern must be a selection")
                }
            }
            _ => Pattern::Error(Error::new(
                ErrorKind::Unknown,
                String::from("unable to parse pattern"),
            )),
        },
    ))
}

// RelativePathExpr ::= StepExpr (('/' | '//') StepExpr)*
fn relativepath_expr_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError> + 'a> {
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
                if let Pattern::Selection(mut ap) = a {
                    // TODO: handle "//" separator
                    for (_c, d) in b {
                        match d {
                            Pattern::Selection(p) => {
                                p.into_iter().for_each(|s| ap.insert(0, s));
                            }
                            _ => panic!("relative path can only contain steps"),
                        }
                    }
                    Pattern::Selection(ap)
                } else {
                    panic!("pattern must be a selection")
                }
            }
        },
    ))
}

// StepExprP ::= PostfixExprExpr | AxisStepP
fn step_expr_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError> + 'a> {
    Box::new(alt2(postfix_expr_pattern::<N>(), axis_step_pattern::<N>()))
}

// PostfixExprP ::= ParenthesizedExprP PredicateList
// TODO: predicates
fn postfix_expr_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError> + 'a> {
    Box::new(map(
        tuple2(paren_expr_pattern(), predicate_list::<N>()),
        |(p, _)| p,
    ))
}

// ParenthesizedExprP ::= "(" UnionExprP ")"
fn paren_expr_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError> + 'a> {
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
fn axis_step_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError> + 'a> {
    Box::new(map(
        tuple2(forward_step_pattern(), predicate_list::<N>()),
        |(f, _p)| f, // TODO: pass predicate back to caller
    ))
}

// ForwardStepP ::= (ForwardAxisP NodeTest) | AbbrevForwardStep
// Returns the node test, the terminal axis and the non-terminal axis
fn forward_step_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError> + 'a> {
    Box::new(map(
        alt2(
            tuple2(forward_axis_pattern(), nodetest()),
            abbrev_forward_step(),
        ),
        |((a, c), nt)| Pattern::Selection(vec![Step::new(a, c, nt)]),
    ))
}

// AbbrevForwardStep ::= "@"? NodeTest
fn abbrev_forward_step<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, ((Axis, Axis), NodeTest)), ParseError> + 'a>
{
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
fn forward_axis_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, (Axis, Axis)), ParseError> + 'a> {
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
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError> + 'a> {
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
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError> + 'a> {
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
fn argument_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError> + 'a> {
    Box::new(alt2(
        variable_reference_pattern::<N>(),
        literal_pattern::<N>(),
    ))
}

// literal
fn literal_pattern<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Pattern<N>), ParseError> + 'a> {
    Box::new(map(literal::<N>(), |l| Pattern::Predicate(l)))
}

// OuterFunctionName ::= "doc" | "id" | "element-with-id" | "key" | "root" | URIQualifiedName
fn outer_function_name<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, NodeTest), ParseError> + 'a> {
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
                name: Some(WildcardOrName::Name(Rc::new(Value::from("element-with-id")))),
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
