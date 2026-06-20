# Medical Markdown Roadmap

## Direction: Rust is the canonical implementation

The Rust crate is now the single, authoritative implementation of Medical Markdown. The original Python package under `medical-markdown-python/` is retained only as a historical reference and is slated for removal once nothing depends on it. New behaviour is specified by the language-agnostic conformance suite under `tests/conformance/` (see [its README](tests/conformance/README.md)), not by either implementation's code. The conformance fixtures are the gold standard: a behaviour is "in the spec" when there is a fixture for it.

Goals driving the near-term roadmap:

- Make the Rust crate cleanly embeddable as a backend dependency, with GitEHR as the first real consumer (see [`spec.md`](spec.md)).
- Capture every behaviour the Python implementation had (and more) as conformance fixtures, so the Python tree can be deleted with confidence.
- Keep non-medical-markdown input a first-class case: plain prose must always extract to an empty structure and never error.

## Completed

- [x] Rewrite from Python to Rust as a `markdown-it` plugin (no vendoring)
- [x] Block-level parser for `CODE/ notes` syntax with nested sub-sections
- [x] Structured JSON data extraction from parsed AST
- [x] Expanded clinical code vocabulary (34 codes across history, examination, vitals, assessment)
- [x] Semantic HTML output with `<section>`, `data-med-code` attributes, CSS classes
- [x] CLI tool (`medmd`) with `--html` and `--json` output modes
- [x] Integration tests mirroring original Python test cases
- [x] WASM build for browser-based editor support
- [x] Inline markdown support within notes (bold, links, etc. - already works via `markdown-it`)
- [x] Configurable/extensible code registry (load custom codes from JSON)
- [x] Preserve section ordering in JSON output (via `serde_json` `preserve_order` feature)
- [x] Source map support for error reporting (line numbers in structured data)
- [x] Comprehensive edge-case tests (empty documents, malformed codes, deeply nested)
- [x] CI pipeline with `cargo test`, `clippy`, `rustfmt`

## Canonical conformance suite (Rust-only consolidation)

- [x] Language-agnostic gold-standard fixtures (`tests/conformance/`): paired `input.md` + `expected.json` cases that define the extraction contract independently of any implementation
- [x] Rust harness that runs every fixture (`tests/conformance.rs`)
- [ ] Port any remaining Python-only behaviour into fixtures, then mark Python ready for deletion
- [ ] Remove `medical-markdown-python/` once no consumer depends on it

## GitEHR embedding - Priority 1: library ergonomics

These block clean embedding of the crate as a backend dependency and come first. See [`spec.md`](spec.md) "Priority 1".

- [x] One-call entry points: `medical_markdown::parse(body) -> ParsedDocument` exposing `.structured()`, `.html()`, `.is_empty()`, `.has_codes()`, plus a `parse_with_registry(body, &registry)` variant
- [x] `has_codes(body: &str) -> bool` cheap pre-check so consumers can decide whether to offer a structured view without a full parse
- [x] Registry loading from memory: `CodeRegistry::from_json_str(&str)` and `from_reader`, with `from_json(path)` kept as a thin wrapper
- [ ] Feature-gate rendering vs extraction (`render` feature, on by default) so an extraction-only backend can build lean. Note: `markdown-it` is required for extraction too, so this mainly gates the rendering convenience surface rather than shrinking the dependency tree

## GitEHR embedding - Priority 2: stable, typed, versioned output contract

GitEHR persists derived structure into State files and diffs it across time, so the output shape is a contract. See [`spec.md`](spec.md) "Priority 2".

- [ ] Typed output model alongside `serde_json::Value`, e.g. `MedicalDocument { sections: Vec<Section { code, heading, notes, subsections, source }> }`, with the JSON form as its serialisation
- [ ] Document and version the extraction schema: specify the `{ "CODE": { "notes": ... }, "_source_map": {...} }` shape, the `CODE.SUBCODE` key convention, and add an explicit `schema_version`
- [ ] Full source spans (start..end byte or line range) per section and sub-section in `_source_map`, not just a start line, to support non-destructive editing and structured diffs

## GitEHR embedding - Priority 3: validation and diagnostics

- [ ] `validate(body, &registry) -> Vec<Diagnostic>` advisory pass (line/span, severity, message) for unrecognised codes etc., without making extraction itself fallible

## GitEHR embedding - Priority 4: clinical syntax for State projection

GitEHR's payoff feature promotes extracted items into longitudinal State files (problem list, medication list). The first two are the specific blockers for that work. See [`spec.md`](spec.md) "Priority 4".

- [ ] `RX/` medication syntax - extract drug, dose, frequency, duration from lines like `RX/ amoxicillin 500mg TDS 7/7`, feeding `state/medications.md`
- [ ] SNOMED-CT inline annotation - `IMP/ #73211009 diabetes mellitus`, extracting the coded concept alongside human text, feeding `state/problems.md` and FHIR/openEHR export
- [ ] Vital signs with units - `BP/ 120/80 mmHg`, `PR/ 72 bpm`, extracted as typed measurements

## Internationalization and Localization

- [ ] Support for non-English clinical codes and terminology
- [ ] Localization of output (e.g., date formats, units)
- [ ] Unicode support for multilingual text in notes and codes
- [ ] Testing with international character sets and right-to-left languages
- [ ] Documentation and examples in multiple languages

## Clinical Terminology Integration

- [ ] Terminology validation against a local SNOMED subset
- [ ] FHIR resource generation - `extract_fhir()` producing FHIR Composition/Observation resources
- [ ] openEHR archetype mapping - map medical markdown sections to archetype paths

## LLM Integration

- [ ] LLM prompt templates for free-text to medical markdown conversion
- [ ] Validation layer - LLM output checked against code registry and terminology
- [ ] Autocomplete/suggestion API - given partial input, suggest completions
- [ ] Voice-to-medical-markdown pipeline (speech to text to structured markdown)

## Further extended specification

- [ ] Real-time parsing mode for editor integration (incremental/partial document parsing)
- [ ] Git-friendly diff support - structured diffs that understand section boundaries (builds on full source spans above)
- [ ] Audit trail metadata - author, timestamp, clinical context in structured output
- [ ] TODO/jobs list syntax with `@assign` and `#context` tags - depends on deployment within a richer EHR system with access to staff registries
- [ ] Pick-list support via REST terminology server integration
- [ ] Clinician-specific code suggestions based on usage patterns
