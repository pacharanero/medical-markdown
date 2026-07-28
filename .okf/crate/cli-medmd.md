---
type: CLI Tool
title: medmd CLI
description: Command-line tool reading Medical Markdown from a file or stdin and emitting HTML, structured JSON, or both, with custom-code support.
resource: https://github.com/folkengine/medical-markdown/blob/master/src/bin/medmd.rs
tags: [rust, cli]
timestamp: '2026-07-28T01:15:00Z'
---

# Usage

```bash
s/dev examples/consultation.md          # HTML + JSON (default: both)
s/dev examples/consultation.md --json   # structured JSON only
s/dev examples/consultation.md --html   # HTML only
echo "PC/ chest pain" | s/dev -         # read from stdin
s/dev input.txt --codes custom-codes.json --json
```

`s/dev` runs `medmd` via `cargo run` in development; `s/build` produces an
optimised binary at `target/release/medmd` (see
[dev scripts](/tooling/dev-scripts.md)).

# Behaviour

- Input: first positional argument that isn't a flag; `-` or no argument
  reads stdin.
- `--json` / `--html` select one output; with neither flag, both are
  printed (HTML first).
- `--codes <path.json>` loads custom codes and merges them over the
  built-in set via the [code registry](/crate/code-registry.md) — a custom
  code with a built-in abbreviation takes precedence.
- Errors (unreadable input, bad codes file) print to stderr and exit 1.

# Citations

[1] [src/bin/medmd.rs](https://github.com/folkengine/medical-markdown/blob/master/src/bin/medmd.rs)
[2] [examples/consultation.md](https://github.com/folkengine/medical-markdown/blob/master/examples/consultation.md)
