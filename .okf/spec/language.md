---
type: Specification
title: Medical Markdown language
description: The CODE/ shorthand syntax for clinical sections, its parsing rules, and the basic vs extended specification split.
tags: [spec, syntax, parsing]
timestamp: '2026-07-28T01:15:00Z'
---

# Syntax

Medical Markdown is standard (Daring Fireball–style) Markdown plus `CODE/`
prefixes that denote clinical sections:

```text
PC/ chest pain, worse on exertion
HPC/ Patient reports 2-hour history of central chest pain.
Pain radiates to left arm.
OE/ Alert and oriented
    RS/ Clear bilaterally
    CVS/ Heart sounds normal, no murmurs
IMP/ Possible ACS
PLAN/ ECG, troponin, aspirin 300mg
```

# Parsing rules

- Top-level `CODE/` lines become `<section>` + `<h2>` elements.
- Indented `CODE/` lines become nested sub-sections (`<section>` + `<h3>`),
  e.g. `RS/` under `OE/`.
- Continuation lines (no code prefix) append to the current section's notes.
- A blank line ends the current section.
- Codes are uppercase; lowercase `pc/` is not parsed as a code
  (pinned by the `lowercase-not-parsed` conformance fixture).
- Unrecognised codes pass through: the code is preserved verbatim and its
  heading falls back to the code itself (`unknown-code-passthrough` fixture).
- Plain prose with no codes always extracts to an empty structure and never
  errors — non-medical input is a first-class case.

Recognised codes and their full clinical headings come from the
[code registry](/crate/code-registry.md); every rule above is pinned as data
in the [conformance suite](/spec/conformance-suite.md).

# Basic vs extended specification

- **Basic** — asynchronous parsing only: text is written, saved, and parsed
  later, with no real-time interaction. This is what the Rust crate
  implements.
- **Extended** — adds synchronous, real-time features such as terminology
  pick lists fed by a REST terminology server, constrained by document
  section, the clinician's usage history, or local guidance. Not yet
  implemented; tracked on the roadmap.

Planned syntax growth (see [GitEHR requirements](/spec/gitehr-embedding.md)):
`RX/` structured medication lines, inline SNOMED-CT annotation
(`IMP/ #73211009 diabetes mellitus`), and vital signs with units.

# Citations

[1] [README — basic and extended specification](https://github.com/folkengine/medical-markdown/blob/master/README.md)
[2] [Crate rustdoc syntax summary](https://github.com/folkengine/medical-markdown/blob/master/src/lib.rs)
[3] [Project wiki](https://github.com/open-health-hub/medical-markdown/wiki)
