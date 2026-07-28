---
type: API Surface
title: Library API (embedding entry points)
description: One-call entry points — parse, parse_with_registry, has_codes — and the ParsedDocument handle for structured data, HTML, and AST access.
resource: https://github.com/folkengine/medical-markdown/blob/master/src/api.rs
tags: [rust, api, embedding]
timestamp: '2026-07-28T01:15:00Z'
---

# Purpose

The `src/api.rs` module wraps the `MarkdownIt` setup boilerplate so a
backend consumer (such as GitEHR — see
[GitEHR embedding requirements](/spec/gitehr-embedding.md)) goes straight
from a Markdown body to structured data or HTML.

# Examples

```rust
let doc = medical_markdown::parse(body);

if doc.has_codes() {
    let document = doc.document();  // typed MedicalDocument: versioned, spans
    let data = doc.structured();    // flat serde_json::Value for MCP/wire use
    let html = doc.html();          // semantic HTML for display surfaces
}
```

With a custom in-memory vocabulary (reuse the registry across calls):

```rust
let registry = medical_markdown::CodeRegistry::from_json_str(codes_json)?;
let doc = medical_markdown::parse_with_registry(body, &registry);
```

# Surface

| Item | Role |
|------|------|
| `parse(body) -> ParsedDocument` | Parse with the built-in vocabulary |
| `parse_with_registry(body, &registry)` | Parse with a custom [code registry](/crate/code-registry.md) |
| `has_codes(body) -> bool` | Cheap pre-check, no full parse; most plain-prose bodies answer `false` quickly |
| `ParsedDocument::document()` | Typed, versioned `MedicalDocument` — the shape to persist ([contract](/crate/output-schema.md)) |
| `ParsedDocument::structured()` | Flat `serde_json::Value` projection for wire/MCP |
| `ParsedDocument::html()` | Semantic HTML rendering |
| `ParsedDocument::ast()` | Borrow the underlying `markdown_it::Node` AST |

Lower-level pieces stay public for consumers that want direct control:
`add` / `add_with_registry` register the [parser plugin](/crate/parser-plugin.md)
on a `MarkdownIt` instance, and `extract_document` / `extract_structured_data`
walk an AST.

Plain-prose bodies with no clinical codes always extract to an empty
structure and never error.

# Citations

[1] [src/api.rs](https://github.com/folkengine/medical-markdown/blob/master/src/api.rs)
[2] [src/lib.rs exports](https://github.com/folkengine/medical-markdown/blob/master/src/lib.rs)
