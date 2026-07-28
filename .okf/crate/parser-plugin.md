---
type: Component
title: markdown-it parser plugin
description: The block-level markdown-it rule that parses CODE/ lines into MedicalSection, MedicalSubSection, and MedicalNotes AST nodes and renders semantic HTML.
resource: https://github.com/folkengine/medical-markdown/blob/master/src/plugin.rs
tags: [rust, parser, markdown-it, html]
timestamp: '2026-07-28T01:15:00Z'
---

# Architecture

Medical Markdown is implemented as a **plugin on the `markdown-it` crate**,
not a vendored or forked parser. `add(md)` (or `add_with_registry`)
registers a block rule alongside the standard CommonMark rules, so inline
Markdown (bold, links, …) inside notes keeps working for free. This
plugin-not-fork architecture is a deliberate keeper — GitEHR depends on it
(see [non-requirements](/spec/gitehr-embedding.md)).

# AST node types

| Node | Meaning | Carries |
|------|---------|---------|
| `MedicalSection` | Top-level `PC/ …` line | `code`, `heading`, `source_line`, `end_line` |
| `MedicalSubSection` | Indented `RS/ …` under a section | `code`, `heading`, `source_line`, `end_line` |
| `MedicalNotes` | Free text within a section | `text` |

Line numbers are 1-based and inclusive; a section's `end_line` covers its
own notes and all sub-sections. These feed the spans in the
[structured output contract](/crate/output-schema.md).

# Rendering

- Section → `<section class="med-section med-{code}" data-med-code="CODE">`
  with an `<h2>` heading.
- Sub-section → `<section class="med-subsection med-{code}">` with `<h3>`.

The `data-med-code` attribute and `med-*` CSS classes are the hooks display
surfaces style against.

Headings come from the [code registry](/crate/code-registry.md); an
unrecognised code renders with the code itself as heading. The syntax rules
the plugin implements are specified in
[the language spec](/spec/language.md) and pinned by the
[conformance suite](/spec/conformance-suite.md).

# Citations

[1] [src/plugin.rs](https://github.com/folkengine/medical-markdown/blob/master/src/plugin.rs)
[2] [markdown-it crate](https://crates.io/crates/markdown-it)
