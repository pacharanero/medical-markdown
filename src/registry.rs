//! Runtime code registry supporting built-in and custom clinical codes.

use std::collections::HashMap;

use markdown_it::parser::extset::MarkdownItExt;
use serde::{Deserialize, Serialize};

use crate::codes::{CLINICAL_CODES, CodeCategory};

/// A clinical code with owned strings, suitable for runtime-loaded codes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnedClinicalCode {
    /// Short code used in medical markdown (e.g. "PC")
    pub code: String,
    /// Full clinical heading (e.g. "Presenting Complaint")
    pub heading: String,
    /// Category for grouping related codes
    pub category: CodeCategory,
}

/// A registry of clinical codes with O(1) lookup.
///
/// Created with [`CodeRegistry::default()`] to get the 34 built-in codes,
/// or loaded from JSON with [`CodeRegistry::from_json_str`] (in-memory),
/// [`CodeRegistry::from_reader`], or [`CodeRegistry::from_json`] (file path).
#[derive(Debug, Clone)]
pub struct CodeRegistry {
    codes: HashMap<String, OwnedClinicalCode>,
}

impl MarkdownItExt for CodeRegistry {}

impl Default for CodeRegistry {
    fn default() -> Self {
        let mut codes = HashMap::new();
        for c in CLINICAL_CODES {
            codes.insert(
                c.code.to_string(),
                OwnedClinicalCode {
                    code: c.code.to_string(),
                    heading: c.heading.to_string(),
                    category: c.category,
                },
            );
        }
        CodeRegistry { codes }
    }
}

impl CodeRegistry {
    /// Look up a clinical code by its abbreviation.
    pub fn lookup(&self, code: &str) -> Option<&OwnedClinicalCode> {
        self.codes.get(code)
    }

    /// Build a code registry from a JSON string held in memory.
    ///
    /// This is the primary loader for embedding consumers (such as GitEHR)
    /// that hold their code registry as in-memory config rather than a file
    /// on disk. The string should contain an array of objects with `code`,
    /// `heading`, and `category` fields:
    ///
    /// ```json
    /// [
    ///   { "code": "RESP", "heading": "Respiratory", "category": "Examination" }
    /// ]
    /// ```
    pub fn from_json_str(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let entries: Vec<OwnedClinicalCode> = serde_json::from_str(json)?;
        let mut codes = HashMap::new();
        for entry in entries {
            codes.insert(entry.code.clone(), entry);
        }
        Ok(CodeRegistry { codes })
    }

    /// Build a code registry by reading JSON from any [`std::io::Read`] source.
    pub fn from_reader<R: std::io::Read>(
        mut reader: R,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut json = String::new();
        reader.read_to_string(&mut json)?;
        Self::from_json_str(&json)
    }

    /// Load a code registry from a JSON file.
    ///
    /// Thin wrapper over [`CodeRegistry::from_json_str`]; see it for the
    /// expected JSON shape.
    pub fn from_json(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        Self::from_json_str(&contents)
    }

    /// Merge another registry into this one. Codes from `other` override
    /// existing codes with the same abbreviation.
    pub fn merge(&mut self, other: &CodeRegistry) {
        for (code, entry) in &other.codes {
            self.codes.insert(code.clone(), entry.clone());
        }
    }
}
