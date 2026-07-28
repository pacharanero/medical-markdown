---
type: Data Contract
title: Structured extraction output contract
description: The two extraction shapes — the typed, versioned MedicalDocument (schema_version 1) to persist, and the flat JSON map for wire/MCP use.
resource: https://github.com/folkengine/medical-markdown/blob/master/docs/output-schema.md
tags: [rust, schema, contract, extraction, gitehr]
timestamp: '2026-07-28T01:15:00Z'
---

# Two shapes, one parse

| Shape | Produced by | Consumer | Versioned |
|-------|-------------|----------|-----------|
| Typed `MedicalDocument` | `extract_document(&ast)` / `ParsedDocument::document()` | Rust consumers that persist and diff (GitEHR) — **the shape to persist** | Yes — `SCHEMA_VERSION` = 1 |
| Flat map (`serde_json::Value`) | `extract_structured_data(&ast)` / `ParsedDocument::structured()` | MCP/wire convenience projection | No |

Both preserve document order (`serde_json` `preserve_order`) and both
extract plain prose to an empty result without erroring.

# Typed document (`MedicalDocument`)

```json
{
  "schema_version": 1,
  "sections": [
    { "code": "OE", "heading": "On Examination", "notes": "Alert",
      "subsections": [
        { "code": "RS", "heading": "Respiratory System", "notes": "Clear",
          "source": { "start_line": 3, "end_line": 3 } }
      ],
      "source": { "start_line": 2, "end_line": 3 } }
  ]
}
```

- `notes` are whitespace-normalised (newlines and whitespace runs collapse
  to single spaces).
- `subsections` is omitted from serialisation when empty.
- `source` spans are **inclusive 1-based line ranges**; a section's span
  covers its notes and all sub-sections. Line numbers were chosen over byte
  offsets because they are robust to CRLF/LF and tab normalisation;
  byte-offset spans are deferred to a future schema version.
- Bump `SCHEMA_VERSION` only when consumers must migrate.

# Flat map

```json
{
  "PC": { "notes": "Headache" },
  "OE": { "notes": "Alert", "RS": "Clear" },
  "_source_map": { "PC": 1, "OE": 2, "OE.RS": 3 }
}
```

Keyed by section code; sub-sections appear as string entries inside their
parent. `_source_map` maps `CODE` and `CODE.SUBCODE` keys to 1-based start
lines and is omitted entirely when there are no sections. This shape is
unversioned — treat it as a projection of the typed document.

The contract exists because GitEHR diffs persisted structure across time —
see [GitEHR embedding requirements](/spec/gitehr-embedding.md). Behaviour
is pinned by the [conformance suite](/spec/conformance-suite.md); the types
live in [the crate's model module](/crate/library-api.md).

# Citations

[1] [docs/output-schema.md](https://github.com/folkengine/medical-markdown/blob/master/docs/output-schema.md)
[2] [src/model.rs](https://github.com/folkengine/medical-markdown/blob/master/src/model.rs)
