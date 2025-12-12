# χrust

[![crates.io](https://img.shields.io/crates/v/xrust.svg)](https://crates.io/crates/xrust)
[![Released API docs](https://docs.rs/xrust/badge.svg)](https://docs.rs/xrust)
[![ALv2 licensed](https://img.shields.io/badge/license-ALv2-blue.svg)](./LICENSE)

XPath, XQuery, and XSLT for Rust

Pronounced "crust".

The goal of this project is to implement [XPath 3.1](https://www.w3.org/TR/xpath-31/), [XQuery 3.1](https://www.w3.org/TR/xquery-31/) and [XSLT 3.0](http://www.w3.org/TR/xslt-30/) in Rust.
Also included is an XML parser, using a parser combinator inspired by Nom.

Currently, the project has achieved the functional equivalent of XPath 1.0 and XSLT 1.0. That is, all of the elements and functions in v1.0 XPath and XSLT have been implemented.
However, it is not *compliant* with XPath/XSLT v1.0. This is because it implements the v3.0 data model.

In addition to the (rudimentary) implementation of the [XQuery and XPath Data Model 3.1](https://www.w3.org/TR/xpath-datamodel-31/) data model, a few other features of XPath and XSLT 2.0/3.0 have been implemented, such as FLWR expressions, grouping (i.e. xsl:for-each-group), and user-defined functions.

## Umbrella Group

χrust is part of the [markup-rs](https://gitlab.gnome.org/World/Rust/markup-rs) family of projects providing support for the XML tech stack in Rust.

The various modules in χrust are being slowly picked apart and moved into separate projects in the markup-rs group. The Qualified Name module was the first to undergo this transition, and is now the qualname crate.

## Canonical Repository

Development of χrust takes place on [Gitlab](https://gitlab.gnome.org/World/Rust/markup-rs/xrust). The github repository is not mirrored (at the moment), so we try and keep the two repositories in sync.

## Version 2.0 Release Notes

When building χrust v2.0 we have noticed unusually long compile times. If your application is adversely affected by this then please let us know. We're working on identifying the cause and developing a solution.

## Design

The library separates parsing from evaluation. XPath expressions and XSLT stylesheets are parsed (or "compiled") into an internal representation, a "transformation", which is then evaluated.

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

### Trees

The "Tree" is the fundamental data object for transformations.
χrust provides a tree implementation, smite, that is both mutable and fully navigable.
By "fully navigable" we mean that from any given node you can access its children, parent, siblings, or attributes.
It achieves mutability by using the interior mutability pattern.

### XML Namespaces

Support for XML Namespaces is in three parts:

1. The XML parser uses a flat mapping (NamespaceMap) of prefix to URI while it constructs the set of in-scope namespaces.
2. The smite tree implementation has a Namespace type node that represents an XML Namespace declaration.
3. Transformations can store the flat mapping (NamespaceMap) of prefix to URI so it can use that, if required, during the transformation.

There is a convenience routine that builds the flat mapping, i.e. a NamespaceMap, from the Namespace node declarations.

NamespaceNode objects are Rc-shared because they are often used but rarely change.

## The Plan

1. Complete the XPath 1.0 implementation. (Done!)
2. Implement all XSLT v1.0 functionality. (Done!)
3. Improve XDM, XPath; achieve v2.0 compliance.
4. Add v2.0 features to the XSLT engine.
4. Further improve XDM, XPath; achieve v3.1 compliance.
5. Add remaining v3.0 features to the XSLT engine.

NB. We're picking the low-hanging fruit first. So major, fundamental features of the languages are being implemented to begin with. The fine detail will be added later.
Although the eventual desire is to implement all of XSLT v3.0 functionality, some more advanced features will be implemented sooner rather than later.

## Documentation

See the [XSLT module](https://docs.rs/xrust/latest/xrust/xslt/index.html) for an example of how to evaluate an XSL stylesheet.

## Examples

* [Integration](https://gitlab.gnome.org/World/Rust/markup-rs/xrust/-/blob/main/examples/ixml.rs) with [Invisible XML](https://www.w3.org/community/ixml/2021/03/19/welcome-to-ixml/).
* [An example](https://gitlab.gnome.org/World/Rust/markup-rs/xrust/-/blob/main/examples/issue-30.rs) using XPath, but not XSLT.

## Compliance

Status of [standards implementation](https://gitlab.gnome.org/World/Rust/markup-rs/xrust/-/blob/main/docs/compliance.md) for XDM, XPath, XQuery, and XSLT.

## Release Notes

| Releases      | Notes                                                                                    |
|---------------|------------------------------------------------------------------------------------------|
| Version 2.0.0 | Use qualname::QName to intern Qualified Names. Update all APIs to use QNames. Improve XML Namespace support. Improve error reporting. Upgrade benchmarks. |
| Version 1.3.0 | Implement xsl:for-each-group/@group-starting-with. Improved serialisation of XML, including canonical format. Fix bug in adding attribute node to result tree. Support predicates for postfix expressions. |
| Version 1.2.5 | Fixed disable-output-escaping. |
| Version 1.2.4 | Improved performance of DTD validation. Bug fixes for ANT declaration and white space handling. |
| Version 1.2.3 | Fixed handling of '<' character in attribute value.                                      |
| Version 1.2.2 | Fixed node() kind test. Fixed union in Pattern. Fixed relative path in Pattern.          |
| Version 1.2.1 | Fixed xsl:sort elements. Fixed 'already borrowed' bug in smite.                          |
| Version 1.2   | Added ID/IDREFS support. Added XPath 2.0 functions: avg(), min(), max().                 |
| Version 1.1   | Added new_document() to Node trait. Tree documents can now be created in one step. eg. smite::RNode::new_document() |
|               | XML Namespace support has been redesigned. Node names are now Rc-shared.                 |
