---
type: Requirements
title: GitEHR embedding requirements
description: Requirements from embedding the crate into GitEHR, the first real consumer, grouped in four priorities with current status.
tags: [gitehr, embedding, requirements, roadmap]
timestamp: '2026-07-28T01:15:00Z'
---

# Context

[GitEHR](https://github.com/gitehr) is the first real consumer of the
crate. It stores each clinical encounter as YAML front matter plus a
free-text Markdown body; the body is canonical and structured data is
extracted *on demand*, never stored separately. So the crate is used as a
pure backend function — body in, structure out — with HTML needed only for
display surfaces. Many bodies contain no `CODE/` lines at all.

# Priorities and status

## Priority 1 — library ergonomics (done)

- ✅ One-call entry points: `parse(body)` → `ParsedDocument`, plus
  `parse_with_registry` — see [library API](/crate/library-api.md).
- ✅ `has_codes(body)` cheap pre-check without a full parse.
- ✅ Registry loading from memory: `CodeRegistry::from_json_str` /
  `from_reader` — see [code registry](/crate/code-registry.md).
- ⬜ Feature-gate rendering vs extraction (`render` Cargo feature). Note:
  `markdown-it` is needed for extraction too, so this mainly gates the
  rendering convenience surface.

## Priority 2 — stable, typed, versioned output contract (done)

GitEHR persists derived structure into State files and diffs it across
time, so the output shape is a contract — see
[structured output contract](/crate/output-schema.md).

- ✅ Typed `MedicalDocument` with `schema_version` alongside the flat JSON.
- ✅ Schema documented in `docs/output-schema.md`.
- ✅ Full inclusive line spans per section and sub-section.
- ⬜ Byte-offset spans (deferred; line spans are robust to normalisation).

## Priority 3 — validation and diagnostics (open)

- ⬜ `validate(body, &registry) -> Vec<Diagnostic>` advisory pass
  ("line 5: unrecognised code `XYZ/`") without making extraction fallible.

## Priority 4 — clinical syntax for State projection (open, highest value)

GitEHR's payoff feature promotes extracted items into longitudinal State
files (problem list, medication list). Blocked specifically on the first
two:

- ⬜ `RX/` medication syntax (`RX/ amoxicillin 500mg TDS 7/7` → drug, dose,
  frequency, duration) feeding `state/medications.md`.
- ⬜ SNOMED-CT inline annotation (`IMP/ #73211009 diabetes mellitus`)
  feeding `state/problems.md` and FHIR/openEHR export.
- ⬜ Vital signs with units (`BP/ 120/80 mmHg`) as typed measurements.

# Non-requirements (already right, keep them)

- Ordering preserved via `serde_json` `preserve_order` — GitEHR relies on it.
- Plain prose extracts cleanly to empty; must never error.
- Plugin-on-`markdown-it` architecture (no vendoring) is what GitEHR wants.
- The [MIT licence](/decisions/mit-license.md) lets AGPL-3.0 GitEHR depend
  on the crate without friction.

# Citations

[1] [spec.md — embedding requirements](https://github.com/folkengine/medical-markdown/blob/master/spec.md)
[2] [ROADMAP.md — GitEHR sections](https://github.com/folkengine/medical-markdown/blob/master/ROADMAP.md)
