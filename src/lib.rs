/*!
A Rust implementation of the [XQuery and XPath Data Model 3.1](https://www.w3.org/TR/xpath-datamodel-31/).

The library separates parsing from evaluation. An expression is compiled to create a '[Sequence] [Constructor]'. This constructor is then applied to a source document to produce a [Sequence].

A [Sequence] is an ordered collection of zero or more [Item]s, implemented as a Rust Vector. An [Item] is a Node, Function or atomic [Value].

See the [graphitem](impls/graphitem/index.html) module for an example of how to evaluate an XSL stylesheet.

## Status

The project, so far, is a proof-of-concept.

For XPath it provides most of v1.0 functionality, with some v2.0 and v3.1 features.

The XSLT implementation is bare-bones. It supports basic templating, literal result elements, attributes, and text. Also conditionals (if, choose), repetition (for-each, for-each-group), and copying (copy, copy-of).

NB, the library has not been extensively tested.

## Plan

1. Complete the XPath 1.0 implementation.
2. Implement all v1.0 XSLT functionality.
3. Implement all XPath 3.1 data model and functions.
4. Complete the v3.1 XSLT engine.

## Goals / Future Work

- The library should always return errors, i.e. it should not panic
- The library uses dynamic Trait objects for tree navigation and construction, but this imposes a runtime penalty. Use monomorphisation to use traits statically at compile-time.
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

#![feature(test)]
#![feature(associated_type_defaults)]

extern crate test;

pub mod xdmerror;
pub use xdmerror::{Error, ErrorKind};

pub mod qname;
mod parsepicture;
mod output;

pub mod value;
pub use value::Value;
pub mod node;
pub use node::Node;
pub mod item;
pub use item::{Sequence, SequenceTrait, Item};

//pub mod xdmgraph;

pub mod impls {
//  pub mod graphitem;

//  pub mod ga;
}

mod parsecommon;

pub mod parsexml;
pub use parsexml::parse;

//pub mod xpath;
//pub use xpath::parse;

//pub mod evaluate;
//pub use evaluate::{StaticContext, Evaluator, Constructor};

//pub mod xslt;
//pub use xslt::from_document;
