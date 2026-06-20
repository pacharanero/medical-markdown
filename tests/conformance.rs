//! Runs the canonical gold-standard conformance suite.
//!
//! For every `tests/conformance/cases/<name>.md` this parses the input with
//! the built-in clinical code vocabulary and asserts that the extracted
//! structured data equals the sibling `<name>.json`. The fixtures, not this
//! file, define correct behaviour; see `tests/conformance/README.md`.

use std::fs;
use std::path::Path;

use medical_markdown::parse;

#[test]
fn conformance_cases() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/conformance/cases");

    let mut md_paths: Vec<_> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "md"))
        .collect();
    md_paths.sort();

    assert!(
        !md_paths.is_empty(),
        "no conformance cases found in {}",
        dir.display()
    );

    for md_path in md_paths {
        let name = md_path.file_stem().unwrap().to_string_lossy().into_owned();
        let json_path = md_path.with_extension("json");

        let input = fs::read_to_string(&md_path).unwrap();
        let expected_raw = fs::read_to_string(&json_path)
            .unwrap_or_else(|_| panic!("case `{name}` is missing its expected JSON file"));
        let expected: serde_json::Value = serde_json::from_str(&expected_raw)
            .unwrap_or_else(|e| panic!("case `{name}` has invalid expected JSON: {e}"));

        let actual = parse(&input).structured();

        assert_eq!(actual, expected, "conformance mismatch for case `{name}`");
    }
}
