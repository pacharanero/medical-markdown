# Rust crate

* [Library API (embedding entry points)](library-api.md) - One-call entry points — parse, parse_with_registry, has_codes — and the ParsedDocument handle for structured data, HTML, and AST access.
* [markdown-it parser plugin](parser-plugin.md) - The block-level markdown-it rule that parses CODE/ lines into MedicalSection, MedicalSubSection, and MedicalNotes AST nodes and renders semantic HTML.
* [Clinical code registry](code-registry.md) - The built-in vocabulary of 34 clinical codes in five categories, and the runtime CodeRegistry for loading and merging custom codes from JSON.
* [Structured extraction output contract](output-schema.md) - The two extraction shapes — the typed, versioned MedicalDocument (schema_version 1) to persist, and the flat JSON map for wire/MCP use.
* [medmd CLI](cli-medmd.md) - Command-line tool reading Medical Markdown from a file or stdin and emitting HTML, structured JSON, or both, with custom-code support.
* [WASM bindings crate](wasm-bindings.md) - The medical-markdown-wasm workspace crate exposing parse_to_html, parse_to_json, and a combined parse to JavaScript via wasm-bindgen.
