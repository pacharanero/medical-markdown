# Medical Markdown - specification and embedding requirements

This document captures the requirements that surfaced while designing the embedding of Medical Markdown into [GitEHR](https://github.com/gitehr) as a backend Rust crate dependency. GitEHR is the first real consumer of the library, and the integration exposed several API ergonomics gaps and missing features. Items are grouped by priority for the GitEHR integration; the existing `ROADMAP.md` remains the broader roadmap and these should be reconciled into it.

## Context: how GitEHR uses the crate

GitEHR stores each clinical encounter as a journal entry: YAML front matter plus a free-text Markdown body. The body is the single source of truth and is treated as canonical; GitEHR extracts structured clinical data from it *on demand* and never stores a separate structured copy. So the crate is used as a pure backend function, body text in, structured data out, with HTML rendering needed only for display surfaces (GUI). Most calls will be extraction, not rendering, and many journal bodies will contain no `CODE/` lines at all.

## Priority 1: library ergonomics for backend embedding

These block clean embedding and should come first.

- **One-call entry points.** Today a consumer must build a `MarkdownIt`, register `cmark`, register `medical_markdown`, parse, then call `extract_structured_data(&ast)`. Provide convenience functions that encapsulate the boilerplate, e.g. `medical_markdown::parse(body) -> ParsedDocument` where `ParsedDocument` exposes `.structured() -> Value`, `.html() -> String`, and `.is_empty()/.has_codes()`. Also a registry-aware variant `parse_with_registry(body, &registry)`. GitEHR otherwise has to duplicate the same five-line setup in every call site.
- **`has_codes(body: &str) -> bool`** (cheap, no full parse if feasible). GitEHR needs to decide whether to offer a structured view for an entry; most plain-prose entries should answer `false` quickly.
- **Registry loading from a string / reader, not only a file path.** `CodeRegistry::from_json(path: &Path)` reads a file directly. GitEHR holds its code registry as repository-controlled config that it may have already read into memory or pulled from git. Add `from_json_str(&str)` (and ideally `from_reader`), keeping `from_json` as a thin wrapper. Small change, large impact on embeddability.
- **Feature-gate rendering vs extraction.** A backend extraction dependency should not have to pull in everything needed for HTML rendering if it never renders. Consider Cargo features (e.g. `render` on by default, extraction always available) so GitEHR's CLI/MCP can depend on a lean extraction-only build.

## Priority 2: a stable, typed, versioned output contract

GitEHR persists derived structure into State files and will diff it, so the output shape is a contract, not an implementation detail.

- **Typed output alongside `serde_json::Value`.** Expose a typed model, e.g. `MedicalDocument { sections: Vec<Section { code, heading, notes, subsections: Vec<SubSection { code, heading, notes }>, source: Span }> }`, with the JSON form as a serialisation of it. Rust consumers (GitEHR) want to pattern-match on typed sections rather than index stringly-typed JSON; the JSON stays available for MCP/wire use.
- **Document and version the extraction schema.** The `{ "PC": { "notes": ... }, "_source_map": {...} }` shape, including the `_source_map` key and the `CODE.SUBCODE` key convention, should be specified and carry a `schema_version`. GitEHR will store/compare this across time and needs a stable contract with an explicit version to migrate against.
- **Full source spans, not just a start line.** `_source_map` currently records a starting line per section. For editor integration, non-destructive editing, and the "git-friendly structured diff" roadmap item, GitEHR needs each section/sub-section to expose its full byte (or line) span (start..end) in the source so structured fields map back to exact body regions.

## Priority 3: validation and diagnostics

- **`validate(body, &registry) -> Vec<Diagnostic>`.** Extraction is currently infallible and silently ignores unrecognised codes. GitEHR wants an advisory pass at commit time: "line 5: unrecognised code `XYZ/`". Diagnostics should carry a line/span, a severity, and a message, without making extraction itself fallible.

## Priority 4: clinical syntax that the State-projection phase depends on

GitEHR's payoff feature is promoting extracted items into longitudinal State files (problem list, medication list). That needs richer section syntax than free-text notes. In rough priority order for GitEHR:

- **`RX/` medication syntax** - structured-enough medication lines (`RX/ amoxicillin 500mg TDS 7/7`) that extract drug, dose, frequency, and duration, feeding `state/medications.md`.
- **SNOMED-CT annotation** - inline code annotation such as `IMP/ #73211009 diabetes mellitus`, extracting the coded concept alongside the human text, feeding `state/problems.md` and downstream FHIR/openEHR export.
- **Vital signs with units** - `BP/ 120/80 mmHg`, `PR/ 72 bpm`, extracted as typed measurements.

These are already on `ROADMAP.md`; this note records that GitEHR's State integration is blocked on the first two specifically, so they are the highest-value items to pull forward.

## Non-requirements / things already fine

- Ordering is already preserved (`serde_json` `preserve_order`); good, GitEHR relies on it.
- Plain-prose bodies already extract to an empty structure cleanly; keep that behaviour - it must never error on non-medical-markdown input.
- No vendoring of the parser; the plugin-on-`markdown-it` architecture is exactly what GitEHR wants and should be preserved.
- The MIT licence is correct for adoption and is why GitEHR (AGPL-3.0) can depend on it without licence friction.
