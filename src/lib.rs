/*!
A Rust implementation of the [XQuery and XPath Data Model 3.1](https://www.w3.org/TR/xpath-datamodel-31/).

The library separates parsing from evaluation. An expression is compiled to create a '[Sequence] [Constructor]'. This constructor is then applied to a source document to produce a [Sequence].

A [Sequence] is an ordered collection of zero or more [Item]s, implemented as a Rust Vector. An [Item] is a [Node], Function or atomic [Value].

See the [xslt](xslt/index.html) module for an example of how to evaluate an XSL stylesheet.

## Trees

The evaluator needs a tree that is both navigable and mutable. The [Item] module defines the [Node] trait that defines what the tree structure looks like. The module [intmuttree] is an implementation of the [Node] trait.

## Parsing XML

Parsing XML documents is done using a parser combinator: [parser].

## Status

For XPath it provides most of v1.0 functionality, with some v2.0 and v3.1 features.

The XSLT implementation is bare-bones. It supports basic templating, literal result elements, attributes, and text. Also conditionals (if, choose), repetition (for-each, for-each-group), copying (copy, copy-of), and inclusion/importing.

NB, the library has not been extensively tested.

## Plan

1. Complete the XPath 1.0 implementation.
2. Implement all v1.0 XSLT functionality.
3. Implement all XPath 3.1 data model and functions.
4. Complete the v3.1 XSLT engine.

## Goals / Future Work

- The library should always return errors, i.e. it should not panic
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

pub mod output;
mod parsepicture;
pub mod qname;

pub mod value;
pub use value::Value;
pub mod item;
pub use item::{Item, Node, Sequence, SequenceTrait};

pub mod pattern;
pub use pattern::Pattern;

#[cfg(feature = "xslt")]
pub mod xslt;

pub mod parser;

pub mod transform;

pub mod trees;
pub use trees::intmuttree::Document;

pub mod testutils;
