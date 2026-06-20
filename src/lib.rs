//! # Medical Markdown
//!
//! A [`markdown-it`] plugin that extends Markdown with clinical note-taking syntax.
//!
//! Clinicians can write shorthand like `PC/ chest pain` and Medical Markdown
//! will parse it into both rendered HTML and structured clinical data.
//!
//! ## Quick Start
//!
//! The one-call entry point handles parser setup for you:
//!
//! ```rust
//! let doc = medical_markdown::parse("PC/ chest pain\nHPC/ started 2 hours ago");
//!
//! assert!(doc.has_codes());
//! assert_eq!(doc.structured()["PC"]["notes"], "chest pain");
//! let _html = doc.html();
//! ```
//!
//! For finer control, register the plugin on a [`markdown_it::MarkdownIt`]
//! directly and walk the AST yourself:
//!
//! ```rust
//! let md = &mut markdown_it::MarkdownIt::new();
//! markdown_it::plugins::cmark::add(md);
//! medical_markdown::add(md);
//!
//! let ast = md.parse("PC/ chest pain\nHPC/ started 2 hours ago");
//! let data = medical_markdown::extract_structured_data(&ast);
//! assert_eq!(data["PC"]["notes"], "chest pain");
//! ```
//!
//! ## Syntax
//!
//! Medical Markdown uses `CODE/` prefixes to denote clinical sections:
//!
//! ```text
//! PC/ chest pain, worse on exertion
//! HPC/ Patient reports 2-hour history of central chest pain.
//! Pain radiates to left arm.
//! OE/ Alert and oriented
//!     RS/ Clear bilaterally
//!     CVS/ Heart sounds normal, no murmurs
//! IMP/ Possible ACS
//! PLAN/ ECG, troponin, aspirin 300mg
//! ```
//!
//! - Top-level codes become `<section>` + `<h2>` elements
//! - Indented codes become nested `<section>` + `<h3>` elements (sub-examinations)
//! - Continuation lines (no code prefix) are appended to the current section
//! - A blank line ends the current section
//!
//! [`markdown-it`]: https://crates.io/crates/markdown-it

mod api;
mod codes;
mod extract;
mod plugin;
mod registry;

pub use api::{ParsedDocument, has_codes, parse, parse_with_registry};
pub use codes::{CLINICAL_CODES, ClinicalCode, CodeCategory};
pub use extract::extract_structured_data;
pub use plugin::{MedicalNotes, MedicalSection, MedicalSubSection, add, add_with_registry};
pub use registry::{CodeRegistry, OwnedClinicalCode};
