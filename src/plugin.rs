//! The markdown-it block rule plugin for medical markdown syntax.

use markdown_it::parser::block::{BlockRule, BlockState};
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

use crate::codes;
use crate::registry::CodeRegistry;

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
    /// 1-based line number in the source document
    pub source_line: usize,
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
    /// 1-based line number in the source document
    pub source_line: usize,
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
        // Transparent container — children are block-level elements
        // (paragraphs, lists, etc.) produced by the full markdown parser.
        fmt.contents(&node.children);
    }
}

// ---------------------------------------------------------------------------
// Block rule: parses medical markdown lines
// ---------------------------------------------------------------------------

/// Matches a line like `CODE/ notes text`.
/// Returns (code, notes_text) or None.
/// This works on already-trimmed lines (from `state.get_line()`).
pub(crate) fn parse_med_code(line: &str) -> Option<(&str, &str)> {
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

fn make_notes_node(text: String, md: &MarkdownIt) -> Node {
    let mut node = Node::new(MedicalNotes { text: text.clone() });
    // Parse notes as full markdown (block + inline) so that lists,
    // multiple paragraphs, and other block elements work correctly.
    let mut parsed = md.parse(&text);
    node.children = std::mem::take(&mut parsed.children);
    node
}

/// Look up a heading for a code, checking the registry in md.ext first,
/// then falling back to the built-in static codes.
fn lookup_heading(state: &BlockState, code: &str) -> String {
    if let Some(registry) = state.md.ext.get::<CodeRegistry>() {
        return registry
            .lookup(code)
            .map(|c| c.heading.clone())
            .unwrap_or_else(|| code.to_string());
    }
    codes::lookup(code)
        .map(|c| c.heading.to_string())
        .unwrap_or_else(|| code.to_string())
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

        let heading = lookup_heading(state, code);

        let mut section_node = Node::new(MedicalSection {
            code: code.to_string(),
            heading,
            source_line: state.line + 1, // 1-based
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
                let sub_heading = lookup_heading(state, sub_code);

                // Flush pending notes before the sub-section
                if !notes_parts.is_empty() {
                    let text = notes_parts.join("\n");
                    section_node.children.push(make_notes_node(text, state.md));
                    notes_parts.clear();
                }

                let sub_source_line = line + 1; // 1-based

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

                let sub_text = sub_notes_parts.join("\n");
                let mut sub_node = Node::new(MedicalSubSection {
                    code: sub_code.to_string(),
                    heading: sub_heading,
                    source_line: sub_source_line,
                });
                if !sub_text.is_empty() {
                    sub_node.children.push(make_notes_node(sub_text, state.md));
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
            let text = notes_parts.join("\n");
            section_node.children.push(make_notes_node(text, state.md));
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

/// Register the medical markdown plugin with a custom [`CodeRegistry`].
///
/// The registry is stored in the parser's extension storage and used
/// for code lookups during parsing. This allows custom codes to be
/// recognised alongside (or instead of) the built-in set.
pub fn add_with_registry(md: &mut MarkdownIt, registry: CodeRegistry) {
    md.ext.insert(registry);
    md.block.add_rule::<MedicalBlockScanner>();
}
