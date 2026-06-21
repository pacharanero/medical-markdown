use medical_markdown::{CodeRegistry, extract_structured_data};
use pretty_assertions::assert_eq;

fn parse(input: &str) -> markdown_it::Node {
    let md = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(md);
    medical_markdown::add(md);
    md.parse(input)
}

fn parse_with_registry(input: &str, registry: CodeRegistry) -> markdown_it::Node {
    let md = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(md);
    medical_markdown::add_with_registry(md, registry);
    md.parse(input)
}

/// Equivalent to Python test_simple: single code line
#[test]
fn simple_single_code() {
    let ast = parse("PC/ Mobility issues");
    let data = extract_structured_data(&ast);

    assert_eq!(data["PC"]["notes"], "Mobility issues");
}

/// Equivalent to Python test_less_simple: multi-line notes
#[test]
fn multiline_notes() {
    let ast = parse("PC/ Mobility issues across multiple lines until\nwe find a newline");
    let data = extract_structured_data(&ast);

    assert_eq!(
        data["PC"]["notes"],
        "Mobility issues across multiple lines until we find a newline"
    );
}

/// Full consultation with nested sub-sections (from Python simple.txt)
#[test]
fn full_consultation_with_subsections() {
    let input = "\
PC/ Mobility issues
HPC/ None
OE/ Pt is definitely not moving
    RS/ None
    CVS/ Not working
IMP/ Patient is very ill
PLAN/ Take a paracetamol and sit by the fire";

    let ast = parse(input);
    let data = extract_structured_data(&ast);

    assert_eq!(data["PC"]["notes"], "Mobility issues");
    assert_eq!(data["HPC"]["notes"], "None");
    assert_eq!(data["OE"]["notes"], "Pt is definitely not moving");
    assert_eq!(data["OE"]["RS"], "None");
    assert_eq!(data["OE"]["CVS"], "Not working");
    assert_eq!(data["IMP"]["notes"], "Patient is very ill");
    assert_eq!(
        data["PLAN"]["notes"],
        "Take a paracetamol and sit by the fire"
    );
}

/// Multi-line notes with nested sub-sections (from Python less_simple.txt)
#[test]
fn multiline_with_subsections() {
    let input = "\
PC/ Mobility issues across multiple lines until
we find a newline
HPC/ None
    RS/ None
    CVS/ None";

    let ast = parse(input);
    let data = extract_structured_data(&ast);

    assert_eq!(
        data["PC"]["notes"],
        "Mobility issues across multiple lines until we find a newline"
    );
    assert_eq!(data["HPC"]["notes"], "None");
    assert_eq!(data["HPC"]["RS"], "None");
    assert_eq!(data["HPC"]["CVS"], "None");
}

/// Unrecognised codes should still be parsed (just use the code as heading)
#[test]
fn unknown_code_passthrough() {
    let ast = parse("NEURO/ Cranial nerves intact");
    let data = extract_structured_data(&ast);

    assert_eq!(data["NEURO"]["notes"], "Cranial nerves intact");
}

/// Normal markdown should pass through unchanged
#[test]
fn regular_markdown_passthrough() {
    let ast = parse("# Regular heading\n\nSome **bold** text.\n");
    let html = ast.render();

    assert!(html.contains("<h1>Regular heading</h1>"));
    assert!(html.contains("<strong>bold</strong>"));
}

/// Medical markdown mixed with regular markdown
#[test]
fn mixed_content() {
    let input = "\
# Patient Notes

PC/ Chest pain

Some additional **regular** markdown notes.
";

    let ast = parse(input);
    let html = ast.render();
    let data = extract_structured_data(&ast);

    assert_eq!(data["PC"]["notes"], "Chest pain");
    assert!(html.contains("<h1>Patient Notes</h1>"));
    assert!(html.contains("Presenting Complaint"));
    assert!(html.contains("<strong>regular</strong>"));
}

/// HTML output has correct semantic structure
#[test]
fn html_structure() {
    let ast = parse("PC/ Chest pain");
    let html = ast.render();

    assert!(html.contains("<section"));
    assert!(html.contains("class=\"med-section med-pc\""));
    assert!(html.contains("data-med-code=\"PC\""));
    assert!(html.contains("<h2>Presenting Complaint</h2>"));
    assert!(html.contains("Chest pain"));
    assert!(html.contains("</section>"));
}

/// Empty notes are handled
#[test]
fn empty_notes() {
    let ast = parse("PC/");
    let data = extract_structured_data(&ast);

    assert_eq!(data["PC"]["notes"], "");
}

/// Multiple sections in sequence
#[test]
fn sequential_sections() {
    let input = "\
PC/ Headache
HPC/ 3 day history
PMH/ Hypertension, diabetes
DH/ Metformin 500mg BD
ALLG/ NKDA
IMP/ Migraine
PLAN/ Analgesia and review";

    let ast = parse(input);
    let data = extract_structured_data(&ast);

    assert_eq!(data["PC"]["notes"], "Headache");
    assert_eq!(data["HPC"]["notes"], "3 day history");
    assert_eq!(data["PMH"]["notes"], "Hypertension, diabetes");
    assert_eq!(data["DH"]["notes"], "Metformin 500mg BD");
    assert_eq!(data["ALLG"]["notes"], "NKDA");
    assert_eq!(data["IMP"]["notes"], "Migraine");
    assert_eq!(data["PLAN"]["notes"], "Analgesia and review");
}

// =========================================================================
// Phase 1: Edge-case tests + inline markdown proof
// =========================================================================

/// Inline bold within medical notes
#[test]
fn inline_bold_in_notes() {
    let ast = parse("PC/ chest pain with **severe** onset");
    let html = ast.render();

    assert!(html.contains("<strong>severe</strong>"));
    assert!(html.contains("Presenting Complaint"));
}

/// Inline italic within medical notes
#[test]
fn inline_italic_in_notes() {
    let ast = parse("IMP/ probable *acute* coronary syndrome");
    let html = ast.render();

    assert!(html.contains("<em>acute</em>"));
}

/// Inline link within medical notes
#[test]
fn inline_link_in_notes() {
    let ast = parse("PLAN/ refer to [NICE guidelines](https://nice.org.uk)");
    let html = ast.render();

    assert!(html.contains("<a href=\"https://nice.org.uk\">NICE guidelines</a>"));
}

/// Inline code within medical notes
#[test]
fn inline_code_in_notes() {
    let ast = parse("DH/ prescribed `metformin` 500mg");
    let html = ast.render();

    assert!(html.contains("<code>metformin</code>"));
}

/// JSON keys preserve document insertion order (not alphabetical)
#[test]
fn json_preserves_insertion_order() {
    let input = "\
PC/ Headache
HPC/ 3 day history
OE/ Alert
IMP/ Migraine
PLAN/ Review";

    let ast = parse(input);
    let data = extract_structured_data(&ast);
    let json_str = serde_json::to_string(&data).unwrap();

    let pc_pos = json_str.find("\"PC\"").unwrap();
    let hpc_pos = json_str.find("\"HPC\"").unwrap();
    let oe_pos = json_str.find("\"OE\"").unwrap();
    let imp_pos = json_str.find("\"IMP\"").unwrap();
    let plan_pos = json_str.find("\"PLAN\"").unwrap();

    assert!(pc_pos < hpc_pos, "PC should appear before HPC");
    assert!(hpc_pos < oe_pos, "HPC should appear before OE");
    assert!(oe_pos < imp_pos, "OE should appear before IMP");
    assert!(imp_pos < plan_pos, "IMP should appear before PLAN");
}

/// Empty document produces empty structured data
#[test]
fn empty_document() {
    let ast = parse("");
    let data = extract_structured_data(&ast);
    let html = ast.render();

    assert_eq!(data, serde_json::json!({}));
    assert!(html.trim().is_empty());
}

/// Whitespace-only document produces empty structured data
#[test]
fn whitespace_only_document() {
    let ast = parse("   \n\n  \n");
    let data = extract_structured_data(&ast);

    assert_eq!(data, serde_json::json!({}));
}

/// Lowercase codes are NOT parsed as medical sections
#[test]
fn lowercase_code_not_parsed() {
    let ast = parse("pc/ chest pain");
    let data = extract_structured_data(&ast);

    assert_eq!(data, serde_json::json!({}));
}

/// Codes with spaces are not parsed as medical sections
#[test]
fn code_with_spaces_not_parsed() {
    let ast = parse("P C/ chest pain");
    let data = extract_structured_data(&ast);

    assert_eq!(data, serde_json::json!({}));
}

/// Text without a slash is regular markdown, not a medical section
#[test]
fn no_slash_is_regular_markdown() {
    let ast = parse("PC chest pain");
    let data = extract_structured_data(&ast);
    let html = ast.render();

    assert_eq!(data, serde_json::json!({}));
    assert!(html.contains("<p>PC chest pain</p>"));
}

/// Many subsections under a single parent
#[test]
fn many_subsections() {
    let input = "\
OE/ General appearance normal
    RS/ Clear bilaterally
    CVS/ HS I+II+0
    GI/ Soft non-tender
    CNS/ GCS 15
    MSK/ Full ROM";

    let ast = parse(input);
    let data = extract_structured_data(&ast);

    assert_eq!(data["OE"]["notes"], "General appearance normal");
    assert_eq!(data["OE"]["RS"], "Clear bilaterally");
    assert_eq!(data["OE"]["CVS"], "HS I+II+0");
    assert_eq!(data["OE"]["GI"], "Soft non-tender");
    assert_eq!(data["OE"]["CNS"], "GCS 15");
    assert_eq!(data["OE"]["MSK"], "Full ROM");
}

/// Subsection with multi-line continuation
#[test]
fn subsection_continuation_lines() {
    let input = "\
OE/ Alert
    RS/ Bilateral air entry equal,
no added sounds, good expansion";

    let ast = parse(input);
    let data = extract_structured_data(&ast);

    assert_eq!(
        data["OE"]["RS"],
        "Bilateral air entry equal, no added sounds, good expansion"
    );
}

/// Trailing whitespace on code line is trimmed
#[test]
fn trailing_whitespace_trimmed() {
    let ast = parse("PC/ chest pain   ");
    let data = extract_structured_data(&ast);

    assert_eq!(data["PC"]["notes"], "chest pain");
}

/// Bullet lists in notes render as HTML lists, not italics
#[test]
fn bullet_list_in_notes() {
    let input = "\
PLAN/
* Serial ECGs
* repeat troponin at 6hrs
* aspirin 300mg stat";

    let ast = parse(input);
    let html = ast.render();

    assert!(html.contains("<ul>"), "expected <ul> in: {html}");
    assert!(html.contains("<li>Serial ECGs</li>"));
    assert!(html.contains("<li>repeat troponin at 6hrs</li>"));
    assert!(html.contains("<li>aspirin 300mg stat</li>"));
    assert!(
        !html.contains("<em>"),
        "asterisks should not become italics: {html}"
    );
}

/// Bullet list with leading text on the code line
#[test]
fn bullet_list_with_leading_text() {
    let input = "\
PLAN/ do the following:
* ECG
* bloods";

    let ast = parse(input);
    let html = ast.render();

    assert!(html.contains("do the following:"));
    assert!(html.contains("<ul>"));
    assert!(html.contains("<li>ECG</li>"));
    assert!(html.contains("<li>bloods</li>"));
}

/// Codes with empty notes including subsections
#[test]
fn codes_with_no_notes() {
    let input = "\
OE/
    RS/
    CVS/";

    let ast = parse(input);
    let data = extract_structured_data(&ast);

    assert_eq!(data["OE"]["notes"], "");
    assert_eq!(data["OE"]["RS"], "");
    assert_eq!(data["OE"]["CVS"], "");
}

/// Source map contains correct line numbers for sections
#[test]
fn source_map_line_numbers() {
    let input = "\
PC/ Headache
HPC/ 3 day history
OE/ Alert
    RS/ Clear
    CVS/ Normal
IMP/ Migraine";

    let ast = parse(input);
    let data = extract_structured_data(&ast);
    let source_map = &data["_source_map"];

    assert_eq!(source_map["PC"], 1);
    assert_eq!(source_map["HPC"], 2);
    assert_eq!(source_map["OE"], 3);
    assert_eq!(source_map["OE.RS"], 4);
    assert_eq!(source_map["OE.CVS"], 5);
    assert_eq!(source_map["IMP"], 6);
}

/// Source map not present for empty documents
#[test]
fn source_map_absent_for_empty() {
    let ast = parse("");
    let data = extract_structured_data(&ast);

    assert!(data.get("_source_map").is_none());
}

/// Source map with blank lines between sections
#[test]
fn source_map_with_gaps() {
    let input = "PC/ Headache\n\nIMP/ Migraine";

    let ast = parse(input);
    let data = extract_structured_data(&ast);
    let source_map = &data["_source_map"];

    assert_eq!(source_map["PC"], 1);
    assert_eq!(source_map["IMP"], 3);
}

/// Multiple blank lines between sections
#[test]
fn multiple_blank_lines_between_sections() {
    let input = "PC/ Headache\n\n\nIMP/ Migraine";

    let ast = parse(input);
    let data = extract_structured_data(&ast);

    assert_eq!(data["PC"]["notes"], "Headache");
    assert_eq!(data["IMP"]["notes"], "Migraine");
}

// =========================================================================
// Code registry tests
// =========================================================================

/// Custom registry recognises custom codes
#[test]
fn custom_registry_code() {
    let mut registry = CodeRegistry::default();
    let custom_codes_json = r#"[
        { "code": "NEURO", "heading": "Neurological Examination", "category": "Examination" }
    ]"#;
    let temp_dir = std::env::temp_dir();
    let codes_path = temp_dir.join("test-custom-codes.json");
    std::fs::write(&codes_path, custom_codes_json).unwrap();
    let custom = CodeRegistry::from_json(&codes_path).unwrap();
    registry.merge(&custom);

    let ast = parse_with_registry("NEURO/ Cranial nerves intact", registry);
    let html = ast.render();
    let data = extract_structured_data(&ast);

    assert!(html.contains("Neurological Examination"));
    assert_eq!(data["NEURO"]["notes"], "Cranial nerves intact");
}

/// Default registry still works via add_with_registry
#[test]
fn default_registry_via_add_with_registry() {
    let registry = CodeRegistry::default();
    let ast = parse_with_registry("PC/ Chest pain", registry);
    let html = ast.render();

    assert!(html.contains("Presenting Complaint"));
}

/// Custom code overrides built-in heading
#[test]
fn custom_registry_overrides_builtin() {
    let custom_json = r#"[
        { "code": "PC", "heading": "Chief Complaint", "category": "History" }
    ]"#;
    let temp_dir = std::env::temp_dir();
    let codes_path = temp_dir.join("test-override-codes.json");
    std::fs::write(&codes_path, custom_json).unwrap();

    let mut registry = CodeRegistry::default();
    let custom = CodeRegistry::from_json(&codes_path).unwrap();
    registry.merge(&custom);

    let ast = parse_with_registry("PC/ Headache", registry);
    let html = ast.render();

    assert!(html.contains("Chief Complaint"));
    assert!(!html.contains("Presenting Complaint"));
}
