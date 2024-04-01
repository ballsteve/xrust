/*!
A Rust implementation of the [XQuery and XPath Data Model 3.1](https://www.w3.org/TR/xpath-datamodel-31/) and [XSLT 3.0](http://www.w3.org/TR/xslt-30/). The idea is to separate the syntax from the semantics. A [Transform] performs the semantics; an XPath expression or XSL Stylesheet is the syntax that is mapped to a [Transform].

## Transformation

A [Transform] is used to create a [Sequence], starting with a [Context].

A [Sequence] is the basic data type in XPath. It is an ordered collection of zero or more [Item]s, implemented as a Rust vector, i.e. ```Vec<Rc<Item>>```. An [Item] is a [Node], Function, or atomic [Value].

Once a [Context] is configured, it can be used to execute a [Transform] using the evaluate method. The return result is a new [Sequence].

## Trees

The [Transform] engine reads a tree structure as its source document and produces a tree structure as its result document. The tree needs to be both navigable and mutable. Tree nodes are defined by the [Item] module's [Node] trait.

The module trees::intmuttree is an implementation of the [Node] trait.

## Parsing XML

Parsing XML documents is done using the built-in parser combinator: [parser]. The parser supports XML Namespaces, and DTDs (entities, but not validation).

## XPath

Support for XPath involves mapping the XPath syntax to a [Transform]. The XPath parser maps an expression to a [Transform].

There is no support for abbreviated syntax, only full syntax.

### Status

Most of functionality for v1.0 is present, with some v2.0 and v3.1 features.

## XSLT

Support for XSLT involves mapping an XSL Stylesheet to a [Context]. The [xslt] module provides the ```from_document``` function that returns a [Context] populated with [Template]s, given an XSL Stylesheet document.

### Status

The XSLT implementation is bare-bones. It supports basic templating, literal result elements, element, text, attribute, comment and processing instruction creation, sequence, and messages. Also conditionals (if, choose), repetition (for-each, for-each-group), copying (copy, copy-of), and inclusion/importing.

NB, the library has not been extensively tested.

### External Resources

One aim of the library is to be useable in a WASM environment. To allow that, the library must not have dependencies on file and network I/O, since that is provided by the host browser environment. Where external resources, i.e. URLs, are required the application must provide a closure. In particular, closures must be provided for stylesheet inclusion and importing, as well as for messages.

## Plan

1. Complete the XPath 1.0 implementation.
2. Implement all v1.0 XSLT functionality.
3. Implement all XPath 3.1 data model and functions.
4. Complete the v3.1 XSLT engine.

## Contributions

We need your help!

- Download the crate and try it out. Tell us what you like or don't like. How can it be improved?
- Let us know what doesn't work. [Submit a bug report.](https://github.com/ballsteve/xrust/issues/new/choose)
- Do you need more documentation? There can never be enough!
- Add some tests.
- Write some code. The χrust Wiki has a [list of desired features](https://github.com/ballsteve/xrust/wiki/Help-Wanted).
- Donate resources (i.e. $$$)

*/

pub mod xdmerror;
pub use xdmerror::{Error, ErrorKind};

pub mod xmldecl;
pub mod externals;
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
pub use transform::context::Context;
pub use transform::template::Template;
pub use transform::Transform;

pub mod trees;
pub use trees::intmuttree::Document;

pub mod testutils;
pub mod validators;
