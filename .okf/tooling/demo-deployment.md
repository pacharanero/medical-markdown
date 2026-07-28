---
type: Deployment
title: Interactive demo and GitHub Pages deployment
description: The demo.html page served locally by s/demo and deployed to GitHub Pages by demo.yml, running the Rust parser compiled to WASM.
resource: https://github.com/folkengine/medical-markdown/blob/master/.github/workflows/demo.yml
tags: [demo, github-pages, wasm, deployment]
timestamp: '2026-07-28T01:15:00Z'
---

# What ships

A single static `demo.html` that loads the Rust parser compiled to WASM
from `pkg/` (the [WASM bindings crate](/crate/wasm-bindings.md)). The live
deployment is at <https://pacharanero.github.io/medical-markdown/>.

# Pages workflow (`demo.yml`)

- Triggers on pushes to `master` that touch `demo.html`, the WASM crate,
  the Rust parser (`src/**`, `Cargo.toml`, `Cargo.lock`), or the workflow
  itself; also manual dispatch.
- Each run rebuilds the WASM from source (`s/build-wasm`), assembles a
  minimal `demo-dist/` (just `demo.html` + `pkg/` + a generated root
  `index.html` that redirects to `demo.html`), and hands it to Pages as a
  **workflow artifact — there is no `gh-pages` branch**.
- One-off repo prerequisite: Settings → Pages → Source must be
  "GitHub Actions"; if left at "Deploy from a branch", `deploy-pages`
  fails with "Get Pages site failed".
- Actions are SHA-pinned per the same house style as the
  [CI pipeline](/tooling/ci-pipeline.md).

# Local development

`s/demo` serves the repository on the first free port in 8000–8030 and
opens the demo (pass a port to override, e.g. `s/demo 9000`). If
`s/build-wasm` has been run, the demo uses the Rust parser via WASM;
otherwise it **falls back to a built-in JS parser** inside `demo.html`.

# Citations

[1] [.github/workflows/demo.yml](https://github.com/folkengine/medical-markdown/blob/master/.github/workflows/demo.yml)
[2] [demo.html](https://github.com/folkengine/medical-markdown/blob/master/demo.html)
[3] [README — Interactive demo](https://github.com/folkengine/medical-markdown/blob/master/README.md)
