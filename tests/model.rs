//! Tests for the typed, versioned output contract (Priority 2 in spec.md).

use medical_markdown::{MedicalDocument, SCHEMA_VERSION, parse};
use pretty_assertions::assert_eq;

#[test]
fn document_carries_schema_version() {
    let doc = parse("PC/ chest pain").document();
    assert_eq!(doc.schema_version, SCHEMA_VERSION);
}

#[test]
fn document_exposes_typed_sections() {
    let doc = parse("PC/ Headache\nIMP/ Migraine").document();

    assert_eq!(doc.sections.len(), 2);
    assert_eq!(doc.sections[0].code, "PC");
    assert_eq!(doc.sections[0].heading, "Presenting Complaint");
    assert_eq!(doc.sections[0].notes, "Headache");
    assert!(doc.sections[0].subsections.is_empty());
    assert_eq!(doc.sections[1].code, "IMP");
}

#[test]
fn document_preserves_order() {
    let doc = parse("PC/ a\nHPC/ b\nOE/ c\nIMP/ d\nPLAN/ e").document();
    let codes: Vec<&str> = doc.sections.iter().map(|s| s.code.as_str()).collect();
    assert_eq!(codes, ["PC", "HPC", "OE", "IMP", "PLAN"]);
}

#[test]
fn typed_sub_sections_with_spans() {
    let input = "\
PC/ Headache
OE/ Alert
    RS/ Clear
    CVS/ Normal
IMP/ Migraine";

    let doc = parse(input).document();

    // PC: line 1 only.
    let pc = &doc.sections[0];
    assert_eq!((pc.source.start_line, pc.source.end_line), (1, 1));

    // OE spans its own line plus the two sub-sections (lines 2-4).
    let oe = &doc.sections[1];
    assert_eq!(oe.code, "OE");
    assert_eq!((oe.source.start_line, oe.source.end_line), (2, 4));
    assert_eq!(oe.subsections.len(), 2);

    let rs = &oe.subsections[0];
    assert_eq!(rs.code, "RS");
    assert_eq!(rs.notes, "Clear");
    assert_eq!((rs.source.start_line, rs.source.end_line), (3, 3));

    let cvs = &oe.subsections[1];
    assert_eq!(cvs.code, "CVS");
    assert_eq!((cvs.source.start_line, cvs.source.end_line), (4, 4));

    // IMP after the sub-sections.
    let imp = &doc.sections[2];
    assert_eq!(imp.code, "IMP");
    assert_eq!((imp.source.start_line, imp.source.end_line), (5, 5));
}

#[test]
fn span_covers_multiline_notes() {
    let input = "PC/ Mobility issues across multiple lines until\nwe find a newline";
    let doc = parse(input).document();

    let pc = &doc.sections[0];
    assert_eq!(
        pc.notes,
        "Mobility issues across multiple lines until we find a newline"
    );
    assert_eq!((pc.source.start_line, pc.source.end_line), (1, 2));
}

#[test]
fn plain_prose_is_empty_document() {
    let doc = parse("Just ordinary clinical prose with no codes.").document();
    assert!(doc.is_empty());
    assert!(doc.sections.is_empty());
    assert_eq!(doc.schema_version, SCHEMA_VERSION);
}

#[test]
fn document_round_trips_through_json() {
    let doc = parse("PC/ Headache\nOE/ Alert\n    RS/ Clear").document();

    let json = serde_json::to_string(&doc).unwrap();
    let back: MedicalDocument = serde_json::from_str(&json).unwrap();

    assert_eq!(doc, back);
}

#[test]
fn serialised_shape_is_stable() {
    let doc = parse("PC/ Headache").document();
    let value = serde_json::to_value(&doc).unwrap();

    assert_eq!(value["schema_version"], 1);
    assert_eq!(value["sections"][0]["code"], "PC");
    assert_eq!(value["sections"][0]["heading"], "Presenting Complaint");
    assert_eq!(value["sections"][0]["notes"], "Headache");
    assert_eq!(value["sections"][0]["source"]["start_line"], 1);
    assert_eq!(value["sections"][0]["source"]["end_line"], 1);
    // Empty subsections are omitted from the serialised form.
    assert!(value["sections"][0].get("subsections").is_none());
}
