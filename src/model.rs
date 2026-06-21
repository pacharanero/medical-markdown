//! The typed, versioned output contract for structured extraction.
//!
//! [`MedicalDocument`] is the primary representation for Rust consumers (such
//! as GitEHR) that want to pattern-match on typed sections rather than index a
//! stringly-typed JSON object. It serialises to a stable, versioned JSON shape;
//! see `docs/output-schema.md` for the contract.
//!
//! The flat [`crate::extract_structured_data`] `serde_json::Value` shape remains
//! available for MCP/wire use.

use serde::{Deserialize, Serialize};

/// Version of the structured extraction schema produced by
/// [`crate::extract_document`].
///
/// Bump this whenever the shape of [`MedicalDocument`] (or its serialisation)
/// changes in a way consumers must migrate against. Persisted documents carry
/// this value so a consumer can detect and migrate older data.
pub const SCHEMA_VERSION: u32 = 1;

/// An inclusive source span, in 1-based line numbers.
///
/// `start_line` and `end_line` bound the lines of the original body that the
/// section (or sub-section) occupies, so a consumer can map a structured field
/// back to the exact region of the source for editing or structured diffing.
/// Line numbers (rather than byte offsets) are used because they are robust to
/// source normalisation such as CRLF/LF and tab handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    /// 1-based line of the first line of the span, inclusive.
    pub start_line: usize,
    /// 1-based line of the last line of the span, inclusive.
    pub end_line: usize,
}

/// A nested clinical sub-section (e.g. `RS/` under `OE/`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubSection {
    /// The clinical code (e.g. "RS").
    pub code: String,
    /// The full clinical heading, falling back to the code if unrecognised.
    pub heading: String,
    /// The free-text notes for this sub-section, whitespace-normalised.
    pub notes: String,
    /// The sub-section's span in the source body.
    pub source: Span,
}

/// A top-level clinical section (e.g. `PC/`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Section {
    /// The clinical code (e.g. "PC").
    pub code: String,
    /// The full clinical heading, falling back to the code if unrecognised.
    pub heading: String,
    /// The free-text notes for this section, whitespace-normalised.
    pub notes: String,
    /// Nested sub-sections, in document order. Empty when there are none.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subsections: Vec<SubSection>,
    /// The section's span in the source body, covering its notes and all of
    /// its sub-sections.
    pub source: Span,
}

/// A parsed medical document: the typed, versioned structured extraction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MedicalDocument {
    /// The schema version this document was produced under; see
    /// [`SCHEMA_VERSION`].
    pub schema_version: u32,
    /// The clinical sections, in document order.
    pub sections: Vec<Section>,
}

impl MedicalDocument {
    /// Whether the document contains no clinical sections.
    pub fn is_empty(&self) -> bool {
        self.sections.is_empty()
    }
}
