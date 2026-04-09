//! The markdown-it block rule plugin for medical markdown syntax.

use markdown_it::parser::block::{BlockRule, BlockState};
use markdown_it::parser::inline::InlineRoot;
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

use crate::codes;

// ---------------------------------------------------------------------------
// AST node types
// ---------------------------------------------------------------------------

/// A top-level clinical section (e.g. `PC/ chest pain`).
///
/// Renders as `<section class="med-section med-{code}"><h2>…</h2><p>…</p></section>`.
#[derive(Debug)]
pub struct MedicalSection {
    /// The clinical code (e.g. "PC")
    pub code: String,
    /// The full heading text (e.g. "Presenting Complaint")
    pub heading: String,
}

/// A nested clinical sub-section (e.g. `    RS/ clear bilaterally` under `OE/`).
///
/// Renders as `<section class="med-subsection med-{code}"><h3>…</h3><p>…</p></section>`.
#[derive(Debug)]
pub struct MedicalSubSection {
    /// The clinical code (e.g. "RS")
    pub code: String,
    /// The full heading text (e.g. "Respiratory System")
    pub heading: String,
}

/// Free-text notes within a clinical section.
#[derive(Debug)]
pub struct MedicalNotes {
    pub text: String,
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

impl NodeValue for MedicalSection {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let class = format!("med-section med-{}", self.code.to_lowercase());
        let mut attrs = node.attrs.clone();
        attrs.push(("class", class));
        attrs.push(("data-med-code", self.code.clone()));

        fmt.cr();
        fmt.open("section", &attrs);
        fmt.cr();
        fmt.open("h2", &[]);
        fmt.text(&self.heading);
        fmt.close("h2");
        fmt.cr();
        fmt.contents(&node.children);
        fmt.close("section");
        fmt.cr();
    }
}

impl NodeValue for MedicalSubSection {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let class = format!("med-subsection med-{}", self.code.to_lowercase());
        let mut attrs = node.attrs.clone();
        attrs.push(("class", class));
        attrs.push(("data-med-code", self.code.clone()));

        fmt.cr();
        fmt.open("section", &attrs);
        fmt.cr();
        fmt.open("h3", &[]);
        fmt.text(&self.heading);
        fmt.close("h3");
        fmt.cr();
        fmt.contents(&node.children);
        fmt.close("section");
        fmt.cr();
    }
}

impl NodeValue for MedicalNotes {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        fmt.open("p", &node.attrs);
        if node.children.is_empty() {
            fmt.text(&self.text);
        } else {
            fmt.contents(&node.children);
        }
        fmt.close("p");
        fmt.cr();
    }
}

// ---------------------------------------------------------------------------
// Block rule: parses medical markdown lines
// ---------------------------------------------------------------------------

/// Matches a line like `CODE/ notes text`.
/// Returns (code, notes_text) or None.
/// This works on already-trimmed lines (from `state.get_line()`).
fn parse_med_code(line: &str) -> Option<(&str, &str)> {
    let slash_pos = line.find('/')?;
    if slash_pos == 0 {
        return None;
    }

    let code = &line[..slash_pos];

    // Code must be all alphanumeric and contain at least one uppercase letter
    if !code.chars().all(|c| c.is_ascii_alphanumeric()) {
        return None;
    }
    if !code.chars().any(|c| c.is_ascii_uppercase()) {
        return None;
    }

    let notes = line[slash_pos + 1..].trim();
    Some((code, notes))
}

/// Check whether a line in the source has real indentation (spaces/tabs before content).
/// We compare `first_nonspace` against `line_start` in the raw source.
fn raw_indent(state: &BlockState, line: usize) -> usize {
    let offsets = &state.line_offsets[line];
    offsets.first_nonspace - offsets.line_start
}

fn make_notes_node(text: String, state: &BlockState) -> Node {
    let mapping = vec![(0, state.line_offsets[state.line].first_nonspace)];
    let mut node = Node::new(MedicalNotes { text: text.clone() });
    node.children
        .push(Node::new(InlineRoot::new(text, mapping)));
    node
}

struct MedicalBlockScanner;

impl BlockRule for MedicalBlockScanner {
    fn run(state: &mut BlockState) -> Option<(Node, usize)> {
        // Only match lines at the block's base indent level (not indented sub-sections)
        let first_line = state.get_line(state.line);
        let (code, first_notes) = parse_med_code(first_line)?;

        // Must be at the base indent level (not a code block or indented content)
        let my_indent = raw_indent(state, state.line);
        if my_indent > 0 && state.line_indent(state.line) >= 0 {
            // This line has real leading whitespace — it's a sub-section.
            // Sub-sections are only consumed as children of a parent section.
            return None;
        }

        let heading = codes::lookup(code)
            .map(|c| c.heading.to_string())
            .unwrap_or_else(|| code.to_string());

        let mut section_node = Node::new(MedicalSection {
            code: code.to_string(),
            heading,
        });

        let mut notes_parts: Vec<String> = Vec::new();
        if !first_notes.is_empty() {
            notes_parts.push(first_notes.to_string());
        }

        let mut lines_consumed = 1;
        let mut line = state.line + 1;

        while line < state.line_max {
            if state.is_empty(line) {
                break;
            }

            let current = state.get_line(line);
            let indent = raw_indent(state, line);

            if let Some((sub_code, sub_notes)) = parse_med_code(current) {
                if indent == 0 {
                    // New top-level section — stop
                    break;
                }

                // Indented medical code — it's a sub-section
                let sub_heading = codes::lookup(sub_code)
                    .map(|c| c.heading.to_string())
                    .unwrap_or_else(|| sub_code.to_string());

                // Flush pending notes before the sub-section
                if !notes_parts.is_empty() {
                    let text = notes_parts.join(" ");
                    section_node.children.push(make_notes_node(text, state));
                    notes_parts.clear();
                }

                let mut sub_notes_parts: Vec<String> = Vec::new();
                if !sub_notes.is_empty() {
                    sub_notes_parts.push(sub_notes.to_string());
                }

                lines_consumed += 1;
                line += 1;

                // Consume continuation lines for the sub-section
                while line < state.line_max {
                    if state.is_empty(line) {
                        break;
                    }
                    let cont = state.get_line(line);
                    if parse_med_code(cont).is_some() {
                        break;
                    }
                    sub_notes_parts.push(cont.trim().to_string());
                    lines_consumed += 1;
                    line += 1;
                }

                let sub_text = sub_notes_parts.join(" ");
                let mut sub_node = Node::new(MedicalSubSection {
                    code: sub_code.to_string(),
                    heading: sub_heading,
                });
                if !sub_text.is_empty() {
                    sub_node.children.push(make_notes_node(sub_text, state));
                }
                section_node.children.push(sub_node);
            } else {
                // Continuation line
                notes_parts.push(current.trim().to_string());
                lines_consumed += 1;
                line += 1;
            }
        }

        // Flush remaining notes
        if !notes_parts.is_empty() {
            let text = notes_parts.join(" ");
            section_node.children.push(make_notes_node(text, state));
        }

        Some((section_node, lines_consumed))
    }
}

/// Register the medical markdown plugin with a [`MarkdownIt`] parser.
///
/// This adds a block-level rule that recognises `CODE/ notes` syntax.
/// Call this after [`markdown_it::plugins::cmark::add`] so that standard
/// Markdown still works alongside medical codes.
pub fn add(md: &mut MarkdownIt) {
    md.block.add_rule::<MedicalBlockScanner>();
}
