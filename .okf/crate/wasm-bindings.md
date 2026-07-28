---
type: Component
title: WASM bindings crate
description: The medical-markdown-wasm workspace crate exposing parse_to_html, parse_to_json, and a combined parse to JavaScript via wasm-bindgen.
resource: https://github.com/folkengine/medical-markdown/tree/master/medical-markdown-wasm
tags: [wasm, browser, bindings]
timestamp: '2026-07-28T01:15:00Z'
---

# Purpose

`medical-markdown-wasm/` is a separate workspace crate (`cdylib` + `rlib`)
that compiles the Rust parser to WebAssembly so the
[interactive demo](/tooling/demo-deployment.md) runs the *same* parser in
the browser that backends embed natively.

# Surface

Three `#[wasm_bindgen]` functions, each building a fresh parser with the
default [code registry](/crate/code-registry.md):

| Function | Returns |
|----------|---------|
| `parse_to_html(input)` | Rendered HTML string |
| `parse_to_json(input)` | Pretty-printed structured JSON (flat shape) |
| `parse(input)` | JSON object `{ "html": "...", "structured": {...} }` |

The structured output is the flat map from the
[output contract](/crate/output-schema.md).

# Building

```bash
cargo install wasm-pack    # one-time
s/build-wasm               # builds to pkg/
```

CI builds it on every push ([CI pipeline](/tooling/ci-pipeline.md)), and
the Pages workflow rebuilds it for deployment.

# Citations

[1] [medical-markdown-wasm/src/lib.rs](https://github.com/folkengine/medical-markdown/blob/master/medical-markdown-wasm/src/lib.rs)
[2] [medical-markdown-wasm/Cargo.toml](https://github.com/folkengine/medical-markdown/blob/master/medical-markdown-wasm/Cargo.toml)
