---
type: CI Pipeline
title: CI pipeline
description: GitHub Actions workflow running lint/test/build, the WASM build, and zizmor workflow-security checks, with all actions SHA-pinned.
resource: https://github.com/folkengine/medical-markdown/blob/master/.github/workflows/ci.yml
tags: [ci, github-actions, security]
timestamp: '2026-07-28T01:15:00Z'
---

# Jobs

`ci.yml` runs on pushes to `master`, pull requests, and manual dispatch,
with per-ref concurrency cancellation and read-only `contents` permission.

| Job | What it does |
|-----|--------------|
| Test & Lint | `s/lint` (clippy + rustfmt), `s/test`, `s/build` via the [dev scripts](/tooling/dev-scripts.md) |
| WASM Build | `s/build-wasm` with `wasm-pack` installed via `taiki-e/install-action` (pinned `wasm-pack@0.15.0`) |
| GitHub Actions security | `zizmor --strict-collection .` (pinned `zizmor@1.27.0`) audits the workflows themselves |

# House style

- Every action is **pinned by commit SHA** with a matching `# vX.Y.Z`
  comment (e.g. `actions/checkout@9c091bb… # v7.0.0`).
- `persist-credentials: false` on every checkout.
- `dtolnay/rust-toolchain` requires an explicit `toolchain: stable` input.
- The [demo deployment](/tooling/demo-deployment.md) is a separate
  workflow, not a CI job.

# Citations

[1] [.github/workflows/ci.yml](https://github.com/folkengine/medical-markdown/blob/master/.github/workflows/ci.yml)
[2] [zizmor](https://github.com/zizmorcore/zizmor)
