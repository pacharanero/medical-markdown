---
type: Dev Tooling
title: Development scripts (s/)
description: The s/ directory of thin wrapper scripts that standardise install, dev-run, test, lint, build, WASM build, and demo serving.
resource: https://github.com/folkengine/medical-markdown/tree/master/s
tags: [tooling, scripts, workflow]
timestamp: '2026-07-28T01:15:00Z'
---

# Scripts

All routine tasks go through the `s/` directory ("scripts-to-rule-them-all"
style); CI runs the same scripts, so local and CI behaviour match.

| Script | Purpose |
|--------|---------|
| `s/install` | Install dependencies and build the project |
| `s/dev` | Run the [`medmd` CLI](/crate/cli-medmd.md) in development mode |
| `s/test` | Run the test suite (`cargo test`), including the [conformance suite](/spec/conformance-suite.md) |
| `s/lint` | Run clippy and check formatting |
| `s/build` | Optimised release binary at `target/release/medmd` |
| `s/build-wasm` | Build the [WASM package](/crate/wasm-bindings.md) to `pkg/` (requires `wasm-pack`) |
| `s/demo` | Serve the [interactive demo](/tooling/demo-deployment.md) on the first free port in 8000–8030 (pass a port to override) |

# Citations

[1] [README — All scripts](https://github.com/folkengine/medical-markdown/blob/master/README.md)
