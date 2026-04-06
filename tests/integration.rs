use medical_markdown::extract_structured_data;
use pretty_assertions::assert_eq;

fn parse(input: &str) -> markdown_it::Node {
    let md = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(md);
    medical_markdown::add(md);
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
    assert_eq!(data["PLAN"]["notes"], "Take a paracetamol and sit by the fire");
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
