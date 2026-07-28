---
type: Component
title: Clinical code registry
description: The built-in vocabulary of 34 clinical codes in five categories, and the runtime CodeRegistry for loading and merging custom codes from JSON.
resource: https://github.com/folkengine/medical-markdown/blob/master/src/registry.rs
tags: [rust, codes, vocabulary, registry]
timestamp: '2026-07-28T01:15:00Z'
---

# Built-in vocabulary

`src/codes.rs` defines `CLINICAL_CODES`: 34 static codes mapping shorthand
to full AoMRC/PRSB clinical headings (e.g. `PC` → "Presenting Complaint",
`PMH` → "Past Medical History"). Each code has a `CodeCategory`:

| Category | Examples |
|----------|----------|
| `History` | `PC`, `HPC`, `PMH`, `DH`, `FH` |
| `Examination` | `OE`, `RS`, `CVS` |
| `Vitals` | `PR`, `BP`, `RR` |
| `Assessment` | `IMP`, `PLAN` |
| `Other` | `TODO` |

# Runtime registry

`CodeRegistry` (in `src/registry.rs`) wraps the vocabulary in a `HashMap`
for O(1) lookup and implements `MarkdownItExt`, so the
[parser plugin](/crate/parser-plugin.md) reads it from parser state.

Constructors, in order of preference for embedding:

- `CodeRegistry::default()` — the 34 built-in codes.
- `CodeRegistry::from_json_str(&str)` — primary loader for embedding
  consumers holding config in memory (added for GitEHR — see
  [requirements](/spec/gitehr-embedding.md)).
- `CodeRegistry::from_reader(r)` — any `Read` source.
- `CodeRegistry::from_json(path)` — thin file-path wrapper.

`merge(&other)` overlays another registry; **a custom code with the same
abbreviation as a built-in one wins**. The JSON format is an array of
`{ "code", "heading", "category" }` objects, as used by the
[CLI's](/crate/cli-medmd.md) `--codes` flag.

# Citations

[1] [src/codes.rs](https://github.com/folkengine/medical-markdown/blob/master/src/codes.rs)
[2] [src/registry.rs](https://github.com/folkengine/medical-markdown/blob/master/src/registry.rs)
[3] [README — custom codes](https://github.com/folkengine/medical-markdown/blob/master/README.md)
