/*!
A Rust implementation of the [XQuery and XPath Data Model 3.1](https://www.w3.org/TR/xpath-datamodel-31/).

The library separates parsing from evaluation. An expression is compiled to create a '[Sequence] [Constructor]'. This constructor is then applied to a source document to produce a [Sequence].

A [Sequence] is an ordered collection of zero or more [Item]s, implemented as a Rust Vector. An [Item] is a Node, Function or atomic [Value].

```rust
# use std::rc::Rc;
# use xrust::item::{Item, Document};
# use xrust::evaluate::{DynamicContext, evaluate};
# use xrust::xpath::parse;
# use libxml::tree::{NodeType as libxmlNodeType, Document as libxmlDocument, Node as libxmlNode, set_node_rc_guard};
# use libxml::parser::Parser;

# set_node_rc_guard(4);

let mut dc = DynamicContext::new();

let p = Parser::default();
let doc = p.parse_string("<Test/>").expect("parsing XML failed");
let rgdoc = Rc::new(doc) as Rc<dyn Document>;

dc.set_doc(Rc::clone(&rgdoc));

let s = vec![Rc::new(Item::Document(Rc::clone(&rgdoc)))];

let xpath = parse("/child::Test").expect("XPath parsing failed");
let seq = evaluate(
  &dc,
  Some(s), Some(0),
  &xpath
).expect("evaluation failed");
assert_eq!(seq.len(), 1);
assert_eq!(seq[0].to_name().get_localname(), "Test");
```

An explanation of the above example:

1. The [libxml crate](https://crates.io/crates/libxml) is used to create an XML document.
2. The libxml Document is cast to a generic [Document].
3. The [Document] is set as the context document in the [DynamicContext].
4. A [Sequence] is created with one item: the document we just created.
5. Parse an XPath expression. This results in a sequence constructor.
6. Evaluate the sequence constructor.
    1) The [DynamicContext] created earlier is used.
    2) The [Sequence] created earlier is provided as the initial context for the evaluation.
7. The evaluation should return a sequence with one item.
8. That item is the root element of the original XML document.

See the [xslt] module for an example of how to evaluate an XSL stylesheet.

## Status

The project, so far, is a proof-of-concept.

For XPath it provides most of v1.0 functionality, with some v2.0 and v3.1 features.

For XSLT, the implementation has barely started. To begin with it only supports literal result elements and literal text.

The library has not been extensively tested.

## Plan

1. ~~~Implement a simple XQuery application.~~~ (We've decided to go straight to XSLT)
2. Implement a v3.1 XSLT engine.

## Goals / Future Work

- The library should always return errors, i.e. it should not panic
- ~~~The library should use Traits for tree navigation and construction~~~ Done!
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

mod xdmgraph;

mod impls {
  mod graphitem;
}

mod parsecommon;

pub mod xpath;
pub use xpath::parse;

pub mod evaluate;
pub use evaluate::{StaticContext, static_analysis, DynamicContext, evaluate, Constructor};

pub mod xslt;
pub use xslt::from_document;
