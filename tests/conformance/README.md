# Medical Markdown conformance suite

This directory is the **gold standard** for Medical Markdown. It defines the extraction contract in a language-agnostic way, independent of any single implementation. The Rust crate is the canonical implementation, but the *specification* of correct behaviour lives here, in data, not in code.

A behaviour is "in the spec" when there is a fixture for it. When the language grows (for example `RX/` medication parsing or SNOMED annotation), add a fixture here first; the implementation is correct when it reproduces every fixture.

## Layout

Each case is a pair of files in `cases/` sharing a base name:

- `cases/<name>.md` - the input Markdown body
- `cases/<name>.json` - the exact structured data that `extract_structured_data` (and the `parse(body).structured()` convenience) must produce for that input, using the built-in clinical code vocabulary

The expected JSON is the full extraction output, including the `_source_map` key where sections are present. Cases are compared as JSON values, so object key ordering is not asserted here (ordering has dedicated tests in `tests/integration.rs`).

## Running

The Rust harness `tests/conformance.rs` discovers every `cases/*.md`, parses it with the default registry, extracts, and asserts equality against the sibling `.json`. Run it with:

```sh
cargo test --test conformance
```

## Adding a case

1. Write `cases/<name>.md` with the input.
2. Write `cases/<name>.json` with the expected extraction output.
3. Run `cargo test --test conformance`.

Keep cases small and focused on one behaviour each. This suite captures the behaviour of the original Python implementation (now removed) and is the place to specify new behaviour as the language grows.
