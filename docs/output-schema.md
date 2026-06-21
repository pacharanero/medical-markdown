# Structured extraction output schema

Medical Markdown extracts a clinical document body into structured data in two shapes. Both are produced from the same parse; choose by consumer.

- **Typed document** - `MedicalDocument`, the versioned contract for Rust consumers (such as GitEHR) that persist and diff derived structure. Produced by `extract_document(&ast)` or `ParsedDocument::document()`. This is the shape to persist.
- **Flat map** - a `serde_json::Value` convenience shape for MCP/wire use. Produced by `extract_structured_data(&ast)` or `ParsedDocument::structured()`. Unversioned; treat as a projection of the typed document.

Both shapes preserve document order, and both extract plain-prose (non-medical) input to an empty result without erroring.

## Typed document (`MedicalDocument`)

The current schema version is **1** (`medical_markdown::SCHEMA_VERSION`). Every serialised document carries its `schema_version` so consumers can detect and migrate older persisted data. Bump the version only when the shape changes in a way consumers must migrate against.

### Shape

```json
{
  "schema_version": 1,
  "sections": [
    {
      "code": "OE",
      "heading": "On Examination",
      "notes": "Alert",
      "subsections": [
        {
          "code": "RS",
          "heading": "Respiratory System",
          "notes": "Clear",
          "source": { "start_line": 3, "end_line": 3 }
        }
      ],
      "source": { "start_line": 2, "end_line": 3 }
    }
  ]
}
```

### Fields

- `schema_version` (integer) - the schema version this document was produced under.
- `sections` (array) - top-level clinical sections, in document order.
  - `code` (string) - the clinical code as written, e.g. `"PC"`. Unrecognised codes are preserved verbatim.
  - `heading` (string) - the full clinical heading for the code, falling back to the code itself when unrecognised.
  - `notes` (string) - the section's free-text notes, whitespace-normalised (newlines and runs of whitespace collapse to single spaces).
  - `subsections` (array) - nested sub-sections, in document order. **Omitted** from the serialised form when empty.
    - `code`, `heading`, `notes` - as above, for the sub-section.
    - `source` - the sub-section's span.
  - `source` - the section's span, covering the section's own notes and all of its sub-sections.

### Spans

A `source` span is an inclusive range of **1-based line numbers** into the source body:

- `start_line` - first line of the section/sub-section, inclusive.
- `end_line` - last line, inclusive.

Spans let a consumer map a structured field back to the exact region of the body for non-destructive editing or structured diffing. Line numbers are used rather than byte offsets because they are robust to source normalisation (CRLF/LF, tab handling). Byte-offset spans may be added in a future schema version if a consumer needs sub-line precision.

## Flat map (`extract_structured_data`)

The flat shape is a JSON object keyed by clinical code. It is convenient for wire/MCP use but is **not** the persistence contract; it is unversioned and carries only the start line of each section.

```json
{
  "PC": { "notes": "Headache" },
  "OE": {
    "notes": "Alert",
    "RS": "Clear",
    "CVS": "Normal"
  },
  "_source_map": {
    "PC": 1,
    "OE": 2,
    "OE.RS": 3,
    "OE.CVS": 4
  }
}
```

Conventions:

- Each top-level key is a section code; its value is an object with a `notes` string and one entry per sub-section code (sub-section value is its notes string).
- `_source_map` maps each section code, and each `CODE.SUBCODE` pair, to its 1-based start line. It is omitted entirely when there are no sections.
- Insertion order is preserved (the crate enables `serde_json`'s `preserve_order`).

For full spans (start and end), use the typed document.
