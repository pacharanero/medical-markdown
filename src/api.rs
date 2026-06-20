//! Ergonomic one-call entry points for embedding Medical Markdown as a
//! backend library.
//!
//! These wrap the `MarkdownIt` setup boilerplate so consumers (such as
//! GitEHR) can go from a Markdown body straight to structured data or HTML
//! without registering plugins by hand.
//!
//! ```rust
//! let doc = medical_markdown::parse("PC/ chest pain\nHPC/ started 2 hours ago");
//! assert!(doc.has_codes());
//! assert_eq!(doc.structured()["PC"]["notes"], "chest pain");
//! let _html = doc.html();
//! ```

use markdown_it::{MarkdownIt, Node};
use serde_json::Value;

use crate::extract::extract_structured_data;
use crate::plugin::{self, MedicalSection, parse_med_code};
use crate::registry::CodeRegistry;

/// A parsed Medical Markdown document.
///
/// Holds the parsed AST and provides the operations a backend consumer
/// needs: structured extraction, HTML rendering, and cheap predicates.
/// Construct one with [`parse`] or [`parse_with_registry`].
pub struct ParsedDocument {
    ast: Node,
}

impl ParsedDocument {
    /// Extract structured clinical data as JSON.
    ///
    /// Plain-prose bodies with no clinical codes extract to an empty object;
    /// this never errors.
    pub fn structured(&self) -> Value {
        extract_structured_data(&self.ast)
    }

    /// Render the document to semantic HTML.
    pub fn html(&self) -> String {
        self.ast.render()
    }

    /// Borrow the underlying parsed AST, for consumers that want to walk it
    /// directly.
    pub fn ast(&self) -> &Node {
        &self.ast
    }

    /// Whether the document contains at least one clinical code section.
    ///
    /// Unlike the free [`has_codes`] function, this is exact: it reflects
    /// what the parser actually recognised.
    pub fn has_codes(&self) -> bool {
        self.ast
            .children
            .iter()
            .any(|child| child.node_value.downcast_ref::<MedicalSection>().is_some())
    }

    /// Whether the document contains no clinical code sections.
    ///
    /// Convenience inverse of [`ParsedDocument::has_codes`].
    pub fn is_empty(&self) -> bool {
        !self.has_codes()
    }
}

fn build_parser(registry: Option<CodeRegistry>) -> MarkdownIt {
    let mut md = MarkdownIt::new();
    markdown_it::plugins::cmark::add(&mut md);
    match registry {
        Some(registry) => plugin::add_with_registry(&mut md, registry),
        None => plugin::add(&mut md),
    }
    md
}

/// Parse a Markdown body using the built-in clinical code vocabulary.
///
/// This is the one-call entry point that replaces the build-a-`MarkdownIt`,
/// register-`cmark`, register-`medical_markdown`, parse dance.
pub fn parse(body: &str) -> ParsedDocument {
    let md = build_parser(None);
    ParsedDocument {
        ast: md.parse(body),
    }
}

/// Parse a Markdown body using a caller-supplied [`CodeRegistry`].
///
/// The registry is cloned into the parser, so a long-lived consumer can hold
/// one registry and reuse it across many calls.
pub fn parse_with_registry(body: &str, registry: &CodeRegistry) -> ParsedDocument {
    let md = build_parser(Some(registry.clone()));
    ParsedDocument {
        ast: md.parse(body),
    }
}

/// Cheaply check whether a body looks like it contains any clinical codes,
/// without a full parse.
///
/// This is a fast line scan intended for the "should we even offer a
/// structured view?" decision, where most plain-prose bodies should answer
/// `false` quickly. It is a heuristic: it can over-report a `CODE/` line that
/// sits inside a fenced code block. For an exact answer, call
/// [`parse`] and then [`ParsedDocument::has_codes`].
pub fn has_codes(body: &str) -> bool {
    body.lines()
        .any(|line| parse_med_code(line.trim()).is_some())
}
