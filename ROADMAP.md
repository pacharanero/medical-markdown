# Medical Markdown Roadmap

## Completed

- [x] Rewrite from Python to Rust as a `markdown-it` plugin (no vendoring)
- [x] Block-level parser for `CODE/ notes` syntax with nested sub-sections
- [x] Structured JSON data extraction from parsed AST
- [x] Expanded clinical code vocabulary (34 codes across history, examination, vitals, assessment)
- [x] Semantic HTML output with `<section>`, `data-med-code` attributes, CSS classes
- [x] CLI tool (`medmd`) with `--html` and `--json` output modes
- [x] Integration tests mirroring original Python test cases

## Phase 1: Core Robustness

- [ ] Inline markdown support within notes (bold, links, etc. — already works via `markdown-it`)
- [ ] Configurable/extensible code registry (load custom codes from TOML/JSON)
- [ ] Preserve section ordering in JSON output (use `IndexMap` or similar)
- [ ] Source map support for error reporting (line numbers in structured data)
- [ ] Comprehensive edge-case tests (empty documents, malformed codes, deeply nested)
- [ ] CI pipeline with `cargo test`, `clippy`, `rustfmt`

## Phase 2: Clinical Terminology Integration

- [ ] SNOMED CT code annotation — `IMP/ #73211009 Diabetes mellitus`
- [ ] Terminology validation against a local SNOMED subset
- [ ] openEHR archetype mapping — map medical markdown sections to archetype paths
- [ ] FHIR resource generation — `extract_fhir()` producing FHIR Composition/Observation resources
- [ ] Bidirectional conversion — structured data back to medical markdown

## Phase 3: GitEHR Integration

- [ ] Library API for embedding in GitEHR backend (Rust crate dependency)
- [ ] WASM build for browser-based editor support
- [ ] Real-time parsing mode for editor integration (incremental/partial document parsing)
- [ ] Git-friendly diff support — structured diffs that understand section boundaries
- [ ] Audit trail metadata — author, timestamp, clinical context in structured output

## Phase 4: LLM Integration

- [ ] LLM prompt templates for free-text → medical markdown conversion
- [ ] Validation layer — LLM output checked against code registry and terminology
- [ ] Autocomplete/suggestion API — given partial input, suggest completions
- [ ] Voice-to-medical-markdown pipeline (speech → text → structured markdown)

## Phase 5: Extended Specification

- [ ] Vital signs with units — `BP/ 120/80 mmHg`, `PR/ 72 bpm`
- [ ] Medication syntax — `RX/ amoxicillin 500mg TDS 7/7`
- [ ] TODO/jobs list syntax with `@assign` and `#context` tags
- [ ] Pick-list support via REST terminology server integration
- [ ] Clinician-specific code suggestions based on usage patterns
