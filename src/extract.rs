//! Structured data extraction from a parsed medical markdown AST.

use crate::model::{MedicalDocument, SCHEMA_VERSION, Section, Span, SubSection};
use crate::plugin::{MedicalNotes, MedicalSection, MedicalSubSection};
use markdown_it::Node;
use serde_json::{Map, Value};

/// Walk the AST and extract structured clinical data as JSON.
///
/// Returns a JSON object where each top-level key is a clinical code,
/// and the value contains `notes` plus any nested sub-section codes.
///
/// # Example output
///
/// ```json
/// {
///   "PC": { "notes": "chest pain" },
///   "OE": {
///     "notes": "Alert and oriented",
///     "RS": "Clear bilaterally",
///     "CVS": "Heart sounds normal"
///   }
/// }
/// ```
pub fn extract_structured_data(root: &Node) -> Value {
    let mut result = Map::new();
    let mut source_map = Map::new();

    for child in &root.children {
        if let Some(section) = child.node_value.downcast_ref::<MedicalSection>() {
            let mut entry = Map::new();

            source_map.insert(
                section.code.clone(),
                Value::Number(section.source_line.into()),
            );

            // Collect notes and sub-sections from children
            let mut notes_parts: Vec<String> = Vec::new();

            for grandchild in &child.children {
                if let Some(sub) = grandchild.node_value.downcast_ref::<MedicalSubSection>() {
                    // Collect sub-section notes
                    let sub_notes = collect_notes(&grandchild.children);
                    entry.insert(sub.code.clone(), Value::String(sub_notes));

                    let sub_key = format!("{}.{}", section.code, sub.code);
                    source_map.insert(sub_key, Value::Number(sub.source_line.into()));
                } else if let Some(notes) = grandchild.node_value.downcast_ref::<MedicalNotes>() {
                    notes_parts.push(normalize_whitespace(&notes.text));
                }
            }

            entry.insert("notes".to_string(), Value::String(notes_parts.join(" ")));
            result.insert(section.code.clone(), Value::Object(entry));
        }
    }

    if !source_map.is_empty() {
        result.insert("_source_map".to_string(), Value::Object(source_map));
    }

    Value::Object(result)
}

/// Walk the AST and extract structured clinical data as a typed, versioned
/// [`MedicalDocument`].
///
/// This is the primary representation for Rust consumers: it carries a
/// [`SCHEMA_VERSION`], preserves document order, and exposes full source
/// [`Span`]s for each section and sub-section. The flat
/// [`extract_structured_data`] shape remains available for MCP/wire use.
///
/// Documents with no clinical codes (plain prose) extract to an empty
/// `sections` list; this never errors.
pub fn extract_document(root: &Node) -> MedicalDocument {
    let mut sections = Vec::new();

    for child in &root.children {
        if let Some(section) = child.node_value.downcast_ref::<MedicalSection>() {
            let mut notes_parts: Vec<String> = Vec::new();
            let mut subsections: Vec<SubSection> = Vec::new();

            for grandchild in &child.children {
                if let Some(sub) = grandchild.node_value.downcast_ref::<MedicalSubSection>() {
                    subsections.push(SubSection {
                        code: sub.code.clone(),
                        heading: sub.heading.clone(),
                        notes: collect_notes(&grandchild.children),
                        source: Span {
                            start_line: sub.source_line,
                            end_line: sub.end_line,
                        },
                    });
                } else if let Some(notes) = grandchild.node_value.downcast_ref::<MedicalNotes>() {
                    notes_parts.push(normalize_whitespace(&notes.text));
                }
            }

            sections.push(Section {
                code: section.code.clone(),
                heading: section.heading.clone(),
                notes: notes_parts.join(" "),
                subsections,
                source: Span {
                    start_line: section.source_line,
                    end_line: section.end_line,
                },
            });
        }
    }

    MedicalDocument {
        schema_version: SCHEMA_VERSION,
        sections,
    }
}

fn collect_notes(children: &[Node]) -> String {
    let mut parts: Vec<String> = Vec::new();
    for child in children {
        if let Some(notes) = child.node_value.downcast_ref::<MedicalNotes>() {
            parts.push(normalize_whitespace(&notes.text));
        }
    }
    parts.join(" ")
}

/// Replace newlines with spaces and collapse runs of whitespace.
fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}
