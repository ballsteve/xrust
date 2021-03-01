//! # xdm
//!
//! A Rust implementation of the XQuery and XPath Data Model 3.1.
//! https://www.w3.org/TR/xpath-datamodel-31/
//!
//! NB. This library is independent of XML and JSON. This is so that
//! documents in either format (or other formats) can be loaded
//! into xdm and operated upon by XPath, XQuery or XSLT.

//! An Item is a Node, Function or Atomic Value.
//! A Sequence is an ordered collection of zero or more Items.
//! Sequences do not nest.

mod xdmerror;
pub use xdmerror::{Error, ErrorKind};

mod item;
pub use item::{Item, Value};

mod parsecommon;
mod parsexml;

//mod sequence;
//pub use sequence::Sequence;

//mod rox_adaptor;

//mod xpath;
//pub use xpath::parse;

//mod evaluate;
//pub use evaluate::{DynamicContext, cons_literal};
