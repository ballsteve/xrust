# AGENTS.md

Guidance for AI coding agents (and human contributors skimming for context)
working in this repository.

## Project overview

χrust ("crust") implements **XPath 3.1** and **XSLT 3.0** in
Rust, along with its own XML parser (a parser-combinator implementation
inspired by Nom). The project currently has functional parity with XPath 1.0
and XSLT 1.0 — all v1.0 elements and functions are implemented — but it is
**not spec-compliant with v1.0**, because internally it already uses the
v3.0 data model. Work is progressing incrementally toward v2.0 and then
v3.1/v3.0 compliance; see "Roadmap & compliance" below before assuming a
missing feature is a bug.

χrust is part of the **markup-rs** family of GNOME-hosted Rust projects
(`https://gitlab.gnome.org/World/Rust/markup-rs`). Modules are being
gradually extracted into sibling crates — `qualname` (Qualified Name
handling) was the first. Before adding or modifying functionality, check
whether it logically belongs in this crate or in a sibling crate in the
`markup-rs` group.

Two downstream applications consume this crate and are useful for
understanding real-world usage:
- `xrusty` — command-line interface (`https://gitlab.gnome.org/World/Rust/markup-rs/xrusty`)
- `xrunchy` — WASM/web usage (`https://gitlab.gnome.org/balls/xrunchy`)

## Canonical repository

Development happens on **GitLab**: `https://gitlab.gnome.org/World/Rust/markup-rs/xrust`.
The GitHub mirror (`ballsteve/xrust`) is read-only — **issues and merge
requests must go to GitLab, not GitHub**. If you're an agent operating
against the GitHub mirror, do not open a PR there; note that it needs to be
submitted to GitLab instead.

## Architecture

The library's core design idea: **parsing/compiling is separated from
evaluation**. XPath expressions and XSLT stylesheets are both parsed down
into the same internal representation — a "transformation" — which is then
evaluated. This means syntactically different but semantically equivalent
inputs compile to identical internal form. For example, the XPath
expression:

```
if $a then "a is true" else ""
```

and the XSLT:

```xml
<xsl:if test="$a">a is true</xsl:if>
```

compile to the same internal transformation. **Implication for agents**:
XPath-engine work and XSLT-engine work often converge in the same shared
compilation/evaluation code — don't assume a fix in one syntax module is
isolated from the other.

### Trees (`smite`)

The tree is the fundamental data structure that transformations operate on.
χrust's tree implementation is called **smite** (an backronym of "tuple
struct enum interior mutability"). Key properties:
- **Mutable** via the interior mutability pattern.
- **Fully navigable**: from any node you can reach children, parent,
  siblings, and attributes.

When editing tree code, preserve these two properties — code elsewhere in
the engine assumes both.

### XML namespaces

Namespace support is deliberately split into three parts — know which layer
you're in before changing namespace-related code:
1. The **XML parser** uses a flat `NamespaceMap` (prefix → URI) while
   constructing in-scope namespaces during parsing.
2. The **smite tree** has a dedicated `Namespace` node type representing a
   namespace declaration in the tree itself.
3. **Transformations** can carry their own flat `NamespaceMap`, built from
   the tree's `Namespace` nodes via a provided convenience routine, for use
   during evaluation.

`NamespaceNode` objects are `Rc`-shared, since they're referenced often but
rarely mutated — follow this convention for similar shared, rarely-changing
data rather than deep-cloning.

## Security model — read before touching resource access

XML documents and XSLT stylesheets can reference external resources (e.g.
external entities, `document()`, includes), which is a real security
surface. χrust is **secure by default**: a resource is inaccessible unless
an application explicitly grants access via a **security policy**.

- A **security feature** is a named check that determines whether access to
  a resource is permitted, and optionally constrains its use (e.g. a size
  limit on an external entity).
- A **security policy** is a set of features an application configures.
- When a module needs a resource, it looks up the relevant feature in the
  active policy and evaluates it before proceeding.

**Rule for any change that touches external resource access**: it must go
through this security-feature/policy mechanism. Never add a code path that
fetches, reads, or expands an external resource without checking the
appropriate security feature first — that would weaken the "secure by
default" guarantee the project explicitly documents. See
`https://docs.rs/xrust/latest/xrust/security/index.html` for the existing
mechanism before adding a new resource type.

## Build & test

```bash
cargo build
cargo test           # Runs project specific unit tests
cargo test-xml       # Runs W3C XML conformance test suite. Ignored tests are functionality not yet implemented. You must enable the feature "test-conformance-xml" to run XML conformance tests.
cargo test-xmlid     # Runs W3C XML ID conformance test suite.  You must enable the feature "test-conformance-xmlid" to run XML ID conformance tests.
cargo bench          # benches/ directory exists — check before assuming no perf tests
```

Known issue: as of the v2.0 release notes, the project has **unusually long
compile times**, under active investigation. If a build feels slow, that's
a known characteristic, not necessarily something to "fix" as a side effect
of an unrelated change. For this reason, it is advised to run `cargo check` during iteration.


Repo layout:
```
src/        core library
tests/      test suite. The tests/conformance folder is very large and only contains XML and DTD files used for the tests. Read specific files from failing tests but otherwise avoid reading this directory
examples/   usage examples (see examples/ixml.rs for Invisible XML
            integration, examples/issue-30.rs for XPath-only usage)
benches/    benchmarks
docs/       project docs, including compliance.md
.cargo/     cargo config
```

## Roadmap & compliance

Check `docs/compliance.md` in this repo before treating a spec gap as a
bug — it tracks current standards-implementation status for XDM, XPath,
XQuery, and XSLT. The project's stated plan, in order:

1. Complete XPath 1.0 implementation — **done**
2. Implement all XSLT 1.0 functionality — **done**
3. Improve XDM/XPath toward v2.0 compliance
4. Add v2.0 features to the XSLT engine
5. Improve XDM/XPath toward v3.1 compliance
6. Add remaining v3.0 features to the XSLT engine

Fundamental/major features are prioritized before fine detail, per this
plan — keep new work aligned with the current stage rather than jumping
ahead to advanced v3.0 features out of order, unless specifically asked.

## Coding conventions

- Match the `smite` interior-mutability and `Rc`-sharing patterns already
  used for tree nodes and namespace nodes rather than introducing new
  ownership patterns for similar data.
- This crate is published on crates.io with real downstream consumers
  (`xrusty`, `xrunchy`, others). Treat public API changes as
  backward-compatibility-sensitive; call out breaking changes explicitly
  rather than making them silently.

## Contribution workflow

- License: Apache-2.0.
- Submit changes as merge requests to the `dev` branch on GitLab
  (`https://gitlab.gnome.org/World/Rust/markup-rs/xrust`), not GitHub.
- Please run `cargo fmt` before submitting a merge request.
- Please ensure all tests pass before submitting a merge request.
  If a test does not pass for any reason, please explicitly call out the failure in the merge request description.

## Things to avoid

- Don't reimplement functionality that has already been extracted into a
  sibling `markup-rs` crate (e.g. Qualified Name handling now lives in
  `qualname`) — check the `markup-rs` group first.
- Don't add external-resource access that bypasses the security
  feature/policy mechanism.
- Don't assume XPath-only or XSLT-only changes are isolated from the other,
  given the shared internal "transformation" representation.
- Don't treat unimplemented v2.0/v3.0/v3.1 features as bugs — check
  `docs/compliance.md` and the roadmap above first.

## Unsafe code policy

Avoid `unsafe`. If you believe it is necessary, call it out explicitly
in your merge request and justify why a safe alternative is infeasible.
