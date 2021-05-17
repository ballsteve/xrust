# χrust

XPath, XQuery, and XSLT for Rust

Pronounced "crust".

The goal of this project is to implement [XPath 3.1](https://www.w3.org/TR/xpath-31/), [XQuery]() and [XSLT 3.0](http://www.w3.org/TR/xslt-30/) in Rust.

Currently the project is a proof-of-concept. There is a rudimentary implementation of the [XQuery and XPath Data Model 3.1](https://www.w3.org/TR/xpath-datamodel-31/), along with an implementation of XPath which, roughly speaking, conforms to version 1.0 (with a few other features, such as FLWR expressions).

## Design

The library separates parsing from evaluation.

## The Plan

1. An XQuery processor.
2. An XSLT processor, implementing approximately v1.0 functionality.
3. Improve XDM, XPath.
4. Introduce all v3.0 features to the XSLT engine.

## Compliance

Status of [docs/compliance.md](https://github.com/ballsteve/xrust/blob/main/docs/compliance.md) with XDM, XPath, XQuery, and XSLT.


