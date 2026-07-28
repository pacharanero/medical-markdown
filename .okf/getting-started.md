---
type: Guide
title: Getting started with the Medical Markdown knowledge bundle
description: Orientation for the Medical Markdown project and a map of this knowledge bundle.
tags: [getting-started, overview]
timestamp: '2026-07-28T01:15:00Z'
---

# What this project is

Medical Markdown is a Markdown-like specification for writing free-text
clinical notes that parse into structured data, using Academy of Medical
Royal Colleges / PRSB document headings. A clinician writes shorthand such
as `PC/ chest pain` and the parser produces both semantic HTML and
structured JSON keyed by clinical code.

The repository holds the **single canonical implementation**: a Rust crate
built as a `markdown-it` plugin (see
[Rust is the canonical implementation](/decisions/rust-canonical.md)),
plus a WASM build for the browser demo and a CLI (`medmd`).

# Where to start

- The language itself: [Medical Markdown language](/spec/language.md)
- What "correct" means: [conformance suite](/spec/conformance-suite.md)
- Embedding the crate as a library: [library API](/crate/library-api.md)
  and the [structured output contract](/crate/output-schema.md)
- Why the work is shaped this way:
  [GitEHR embedding requirements](/spec/gitehr-embedding.md)

# Bundle map

- `/spec/` — the language, its gold-standard fixtures, and consumer-driven
  requirements.
- `/crate/` — the Rust crate's surfaces: API, parser plugin, code registry,
  output contract, CLI, WASM bindings.
- `/decisions/` — recorded decisions and their rationale.
- `/tooling/` — dev scripts, CI, and demo deployment.

# Citations

[1] [README](https://github.com/folkengine/medical-markdown/blob/master/README.md)
[2] [ROADMAP](https://github.com/folkengine/medical-markdown/blob/master/ROADMAP.md)
