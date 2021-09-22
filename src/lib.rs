/*!
A Rust implementation of the [XQuery and XPath Data Model 3.1](https://www.w3.org/TR/xpath-datamodel-31/).

The library separates parsing from evaluation. An expression is compiled to create a '[Sequence] [Constructor]'. This constructor is then applied to a source document to produce a [Sequence].

A [Sequence] is an ordered collection of zero or more [Item]s, implemented as a Rust Vector. An [Item] is a Node, Function or atomic [Value].

See the [graphitem](impls/graphitem/index.html) module for an example of how to evaluate an XSL stylesheet.

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

pub mod xdmgraph;

pub mod impls {
  pub mod graphitem;
}

mod parsecommon;

pub mod parsexml;
//pub use parsexml::parse;

pub mod xpath;
pub use xpath::parse;

pub mod evaluate;
pub use evaluate::{StaticContext, static_analysis, DynamicContext, evaluate, Constructor};

pub mod xslt;
pub use xslt::from_document;
