---
type: Test Suite
title: Gold-standard conformance suite
description: Language-agnostic fixtures in tests/conformance/ that define the extraction contract; a behaviour is in the spec when a fixture exists for it.
resource: https://github.com/folkengine/medical-markdown/tree/master/tests/conformance
tags: [spec, testing, conformance, gold-standard]
timestamp: '2026-07-28T01:15:00Z'
---

# Role

`tests/conformance/` is the **gold standard** for Medical Markdown. It
defines the extraction contract as data, independent of any implementation.
The rule: *a behaviour is "in the spec" when there is a fixture for it.*
New language features (e.g. `RX/`, SNOMED annotation) get a fixture first;
an implementation is correct when it reproduces every fixture.

The suite preserves the behaviour of the original Python implementation,
which allowed the Python tree to be deleted — see
[Rust is the canonical implementation](/decisions/rust-canonical.md).

# Layout

Each case is a pair in `cases/` sharing a base name:

- `cases/<name>.md` — input Markdown body
- `cases/<name>.json` — exact output `extract_structured_data` must produce
  with the built-in vocabulary, including `_source_map` where sections exist

Comparison is by JSON value, so key ordering is not asserted here (ordering
has dedicated tests in `tests/integration.rs`).

# Current cases

| Case | Behaviour pinned |
|------|------------------|
| `single-code` | One `CODE/` line extracts to one section |
| `multiline-notes` | Continuation lines append to the current section |
| `subsections` | Indented codes nest under their parent section |
| `blank-lines-between` | A blank line ends the current section |
| `empty-document` | Empty input → empty structure |
| `empty-notes` | A code with no notes still yields a section |
| `plain-prose` | Non-medical prose → empty structure, no error |
| `lowercase-not-parsed` | Lowercase `pc/` is not a code |
| `unknown-code-passthrough` | Unrecognised codes preserved verbatim |
| `full-consultation` | End-to-end realistic consultation |

# Running and extending

```sh
cargo test --test conformance
```

The Rust harness (`tests/conformance.rs`) discovers every `cases/*.md`,
parses with the default registry, and asserts equality against the sibling
`.json`. To add a behaviour: write the `.md` and `.json` pair, keep it small
and focused on one behaviour, and run the harness.

# Citations

[1] [Conformance suite README](https://github.com/folkengine/medical-markdown/blob/master/tests/conformance/README.md)
