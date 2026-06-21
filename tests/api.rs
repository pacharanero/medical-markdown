//! Tests for the ergonomic one-call embedding API (Priority 1 in spec.md).

use medical_markdown::{CodeRegistry, has_codes, parse, parse_with_registry};
use pretty_assertions::assert_eq;

#[test]
fn parse_extracts_structured_data() {
    let doc = parse("PC/ chest pain\nHPC/ started 2 hours ago");

    assert_eq!(doc.structured()["PC"]["notes"], "chest pain");
    assert_eq!(doc.structured()["HPC"]["notes"], "started 2 hours ago");
}

#[test]
fn parse_renders_html() {
    let doc = parse("PC/ chest pain");
    let html = doc.html();

    assert!(html.contains("<h2>Presenting Complaint</h2>"));
    assert!(html.contains("data-med-code=\"PC\""));
}

#[test]
fn parsed_document_has_codes_is_exact() {
    assert!(parse("PC/ chest pain").has_codes());
    assert!(!parse("PC/ chest pain").is_empty());

    let prose = parse("Just some ordinary clinical prose, no codes here.");
    assert!(!prose.has_codes());
    assert!(prose.is_empty());
}

#[test]
fn has_codes_quick_check() {
    // Positive: a clinical code line, at base indent or indented
    assert!(has_codes("PC/ chest pain"));
    assert!(has_codes("notes\n    RS/ clear"));

    // Negative: plain prose, lowercase pseudo-codes, no slash
    assert!(!has_codes("The patient is feeling much better today."));
    assert!(!has_codes("pc/ chest pain"));
    assert!(!has_codes("PC chest pain"));
    assert!(!has_codes(""));
}

#[test]
fn parse_with_registry_uses_custom_codes() {
    let custom = r#"[
        { "code": "NEURO", "heading": "Neurological Examination", "category": "Examination" }
    ]"#;
    let mut registry = CodeRegistry::default();
    registry.merge(&CodeRegistry::from_json_str(custom).unwrap());

    let doc = parse_with_registry("NEURO/ Cranial nerves intact", &registry);

    assert!(doc.html().contains("Neurological Examination"));
    assert_eq!(doc.structured()["NEURO"]["notes"], "Cranial nerves intact");
}

#[test]
fn registry_is_reusable_across_calls() {
    // parse_with_registry borrows the registry, so it can be reused.
    let registry = CodeRegistry::default();

    let first = parse_with_registry("PC/ Headache", &registry);
    let second = parse_with_registry("IMP/ Migraine", &registry);

    assert_eq!(first.structured()["PC"]["notes"], "Headache");
    assert_eq!(second.structured()["IMP"]["notes"], "Migraine");
}

#[test]
fn registry_from_reader() {
    let custom = r#"[
        { "code": "OBS", "heading": "Observation", "category": "Examination" }
    ]"#;
    let registry = CodeRegistry::from_reader(custom.as_bytes()).unwrap();

    assert_eq!(registry.lookup("OBS").unwrap().heading, "Observation");
}
