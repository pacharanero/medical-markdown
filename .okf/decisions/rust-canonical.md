---
type: Decision
title: Rust is the canonical implementation
description: The Rust crate is the single authoritative implementation; the original Python package was removed after its behaviour was captured as conformance fixtures.
tags: [decision, rust, python, conformance]
timestamp: '2026-07-28T01:15:00Z'
---

# Decision

The Rust crate in this repository is the **single canonical
implementation** of Medical Markdown. The original Python package has been
removed.

# Rationale and mechanism

Rather than keeping two implementations in sync, the Python behaviour was
captured as language-agnostic fixtures in the
[gold-standard conformance suite](/spec/conformance-suite.md), which pins
behaviour **as data**: a behaviour is part of the spec when there is a
fixture for it. This made deleting the Python tree safe — the spec no
longer lives in any implementation's code.

Consequences:

- New behaviour is specified by adding a fixture *first*; the
  implementation is correct when it reproduces every fixture.
- Future implementations in other languages conform by reproducing the
  same fixtures, not by porting Rust code.
- The crate is built as a plugin on `markdown-it`
  ([parser plugin](/crate/parser-plugin.md)) rather than a vendored parser,
  which the first consumer explicitly wants preserved
  ([GitEHR requirements](/spec/gitehr-embedding.md)).

# Citations

[1] [ROADMAP — Direction](https://github.com/folkengine/medical-markdown/blob/master/ROADMAP.md)
[2] [README — Canonical implementation](https://github.com/folkengine/medical-markdown/blob/master/README.md)
