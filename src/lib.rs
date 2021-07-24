/*!
A Rust implementation of the [XQuery and XPath Data Model 3.1](https://www.w3.org/TR/xpath-datamodel-31/).

The library separates parsing from evaluation. An expression is compiled to create a '[Sequence] [Constructor]'. This constructor is then applied to a source document to produce a [Sequence].

A [Sequence] is an ordered collection of zero or more [Item]s, implemented as a Rust Vector. An [Item] is a Node, Function or atomic [Value].

```rust
# use std::rc::Rc;
# use xrust::item::Item;
# use xrust::evaluate::{DynamicContext, evaluate};
# use xrust::xpath::parse;

let xdoc = roxmltree::Document::parse("<Test/>").expect("parsing XML failed");
let d = vec![Rc::new(Item::XNode(xdoc.root().first_child().unwrap()))];
let xpath = parse("/child::Test").expect("XPath parsing failed");
let dc = DynamicContext::new();
let seq = evaluate(
  &dc,
  Some(d), Some(0),
  &xpath
).expect("evaluation failed");
assert_eq!(seq.len(), 1);
assert_eq!(seq[0].to_name(), "Test");
```

An explanation of the above example:

1. The [roxmltree crate](https://crates.io/crates/roxmltree) is used to create an XML document.
2. A [Sequence] is created with one item: the document we just created.
3. Parse an XPath expression. This results in a sequence constructor.
4. Evaluate the sequence constructor.
    1) A default [DynamicContext] is provided.
    2) The [Sequence] created earlier is provided as the initial context for the evaluation.
5. The evaluation should return a sequence with one item.
6. That item is the root element of the original XML document.

See the [xslt] module for an example of how to evaluate an XSL stylesheet.

## Status

The project, so far, is a proof-of-concept.

For XPath it provides most of v1.0 functionality, with some v2.0 and v3.2 features.

For XSLT, the implementation has barely started. To begin with it only supports literal result elements and literal text.

The library has not been extensively tested.

## Plan

1. ~~~Implement a simple XQuery application.~~~ (We've decided to go straight to XSLT)
2. Implement a v3.1 XSLT engine.

## Goals / Future Work

- The library should always return errors, i.e. it should not panic
- The library should use Traits for tree navigation and construction
- Make the library more idiomatically Rust

## Contributions

We need your help!

- Download the crate and try it out. Tell us what you like or don't like. How can it be improved?
- Let us know what doesn't work. [Submit a bug report.](https://github.com/ballsteve/xrust/issues/new/choose)
- Do you need more documentation? There can never be enough!
- Add some tests.
- Write some code.
- Donate resources (i.e. $$$)

*/

pub mod xdmerror;
pub use xdmerror::{Error, ErrorKind};

pub mod item;
pub use item::{Sequence, SequenceTrait, Item, Value, Document, Node};

mod impls {
  mod xdmlibxml;
  pub use xdmlibxml::*;
  //mod xdmrox;
  //pub use xdmrox::*;
}

mod parsecommon;

//pub mod xpath;
//pub use xpath::parse;

pub mod evaluate;
pub use evaluate::{StaticContext, static_analysis, DynamicContext, evaluate, Constructor};

//pub mod xslt;
//pub use xslt::from_xnode;
