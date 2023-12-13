/*! # Parse XPath expressions

An XPath expression parser using the xrust parser combinator that produces a xrust transformation.

```rust
use xrust::parser::xpath::parse;
# use xrust::item::Node;
# fn do_parse<N: Node>() {
let t = parse::<N>("/child::A/child::B/child::C").expect("unable to parse XPath expression");
# }
```

"t" now contains a [Transform] that will return "C" elements that have a "B" parent and an "A" grandparent in the source document.

To evaluate the transformation we need a Context with a source document as its current item.

```rust
# use std::rc::Rc;
# use xrust::xdmerror::Error;
use xrust::item::{Sequence, SequenceTrait, Item, Node, NodeType};
use xrust::trees::intmuttree::{Document, NodeBuilder};
use xrust::parser::xpath::parse;
use xrust::transform::context::{Context, ContextBuilder, StaticContext};
# type F = Box<dyn FnMut(&str) -> Result<(), Error>>;

let t = parse("/child::A/child::B/child::C")
    .expect("unable to parse XPath expression");

let source = Document::try_from(("<A><B><C/></B><B><C/></B></A>".to_string(), None, None))
    .expect("unable to parse XML")
    .content[0]
    .clone();
let mut doc = NodeBuilder::new(NodeType::Document).build();
doc.push(source).expect("unable to attach root node");
let context = ContextBuilder::new()
    .current(vec![Rc::new(Item::Node(doc))])
    .build();
let sequence = context.dispatch(&mut StaticContext::<F>::new(), &t)
    .expect("evaluation failed");
assert_eq!(sequence.len(), 2);
assert_eq!(sequence.to_xml(), "<C></C><C></C>")
```
*/

mod compare;
mod context;
mod expressions;
mod flwr;
mod functions;
pub(crate) mod literals;
mod logic;
mod nodes;
pub(crate) mod nodetests;
mod numbers;
pub(crate) mod predicates;
mod strings;
pub(crate) mod support;
mod types;
pub(crate) mod variables;

use crate::parser::combinators::alt::alt4;
use crate::parser::combinators::list::separated_list1;
use crate::parser::combinators::map::map;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::tuple3;
use crate::parser::combinators::whitespace::xpwhitespace;
//use crate::parser::combinators::debug::inspect;
use crate::parser::xpath::flwr::{for_expr, if_expr, let_expr};
use crate::parser::xpath::logic::or_expr;
use crate::parser::xpath::support::noop;
use crate::parser::{ParseError, ParseInput, ParseResult, ParserState};

use crate::item::Node;
use crate::transform::Transform;
use crate::xdmerror;

pub fn parse<N: Node>(input: &str) -> Result<Transform<N>, xdmerror::Error> {
    // Shortcut for empty
    if input.is_empty() {
        return Ok(Transform::Empty);
    }

    let state = ParserState::new(None, None);
    match xpath_expr((input, state)) {
        Ok((_, x)) => Ok(x),
        Err(err) => match err {
            ParseError::Combinator => Result::Err(xdmerror::Error::new(
                xdmerror::ErrorKind::ParseError,
                "Unrecoverable parser error.".to_string(),
            )),
            ParseError::NotWellFormed => Result::Err(xdmerror::Error::new(
                xdmerror::ErrorKind::ParseError,
                "Unrecognised extra characters.".to_string(),
            )),
            ParseError::MissingNameSpace => Result::Err(xdmerror::Error::new(
                xdmerror::ErrorKind::ParseError,
                "Missing namespace declaration.".to_string(),
            )),
            ParseError::Notimplemented => Result::Err(xdmerror::Error::new(
                xdmerror::ErrorKind::ParseError,
                "Unimplemented feature.".to_string(),
            )),
            _ => Err(xdmerror::Error::new(
                xdmerror::ErrorKind::Unknown,
                "Unknown error".to_string(),
            )),
        },
    }
}

fn xpath_expr<N: Node>(input: ParseInput) -> ParseResult<Transform<N>> {
    match expr::<N>()(input) {
        Err(err) => Err(err),
        Ok(((input1, state1), e)) => {
            //Check nothing remaining in iterator, nothing after the end of the root node.
            if input1.is_empty() {
                Ok(((input1, state1), e))
            } else {
                Err(ParseError::NotWellFormed)
            }
        }
    }
}
// Implementation note: cannot use opaque type because XPath expressions are recursive, and Rust *really* doesn't like recursive opaque types. Dynamic trait objects aren't ideal, but compiling XPath expressions is a one-off operation so that shouldn't cause a major performance issue.
// Implementation note 2: since XPath is recursive, must lazily evaluate arguments to avoid stack overflow.
pub fn expr<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(map(
        separated_list1(
            map(tuple3(xpwhitespace(), tag(","), xpwhitespace()), |_| ()),
            expr_single::<N>(),
        ),
        |mut v| {
            if v.len() == 1 {
                v.pop().unwrap()
            } else {
                Transform::SequenceItems(v)
            }
        },
    ))
}

pub(crate) fn expr_wrapper<N: Node>(
    b: bool,
) -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>>> {
    Box::new(move |input| {
        if b {
            expr::<N>()(input)
        } else {
            noop::<N>()(input)
        }
    })
}

// ExprSingle ::= ForExpr | LetExpr | QuantifiedExpr | IfExpr | OrExpr
fn expr_single<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>> + 'a> {
    Box::new(alt4(or_expr(), let_expr(), for_expr(), if_expr()))
}

pub(crate) fn expr_single_wrapper<N: Node>(
    b: bool,
) -> Box<dyn Fn(ParseInput) -> ParseResult<Transform<N>>> {
    Box::new(move |input| {
        if b {
            expr_single::<N>()(input)
        } else {
            noop::<N>()(input)
        }
    })
}
