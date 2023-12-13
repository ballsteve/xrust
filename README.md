# χrust

XPath, XQuery, and XSLT for Rust

Pronounced "crust".

The goal of this project is to implement [XPath 3.1](https://www.w3.org/TR/xpath-31/), [XQuery 3.1](https://www.w3.org/TR/xquery-31/) and [XSLT 3.0](http://www.w3.org/TR/xslt-30/) in Rust. Also included is an XML parser, using a parser combinator inspired by Nom.

Currently the project is a proof-of-concept. There is a rudimentary implementation of the [XQuery and XPath Data Model 3.1](https://www.w3.org/TR/xpath-datamodel-31/), along with an implementation of XPath which, roughly speaking, conforms to version 1.0 (with a few other features, such as FLWR expressions). The XSLT implementation can evaluate basic XSL stylesheets, but is still incomplete wrt the XSLT v1.0 specification.

## Design

The library separates parsing from evaluation. The XPath and XSLT are parsed (or "compiled") into an internal representation, a "transformation", which is then evaluated.

This means it won't matter how an expression is written; it will be compiled into the same internal form. For example, the XPath expression:

```xpath
if $a then "a is true" else ""
```

will result in the same internal format as:

```xml
<xsl:if test="$a">
  a is true
</xsl:if>
```

This approach means that the XPath and XSLT modules are simply mappings from their respective syntaxes to an χrust transformation.

## The Plan

1. Complete the XPath 1.0 implementation.
2. Implement all XSLT v1.0 functionality.
3. Improve XDM, XPath; achieve v2.0-v3.1 compliance.
4. Introduce all v3.0 features to the XSLT engine.

NB. We're picking the low-hanging fruit first. So major, fundamental features of the languages are being implemented to begin with. The fine detail will be added later. So although we're aiming for v1.0 functionality as a baseline, the eventual desire to implement all of v3.0 dictates that some more advanced features will be implemented sooner rather than later.

## Documentation

See the [XSLT module](xrust/xslt/index.html) for an example of how to evaluate an XSL stylesheet.

## Examples

* [Integration](https://github.com/ballsteve/xrust/blob/main/examples/ixml.rs) with [Invisible XML](https://www.w3.org/community/ixml/2021/03/19/welcome-to-ixml/).
* [An example](https://github.com/ballsteve/xrust/blob/main/examples/issue-30.rs) using XPath, but not XSLT.

## Compliance

Status of [docs/compliance.md](https://github.com/ballsteve/xrust/blob/main/docs/compliance.md) with XDM, XPath, XQuery, and XSLT.


