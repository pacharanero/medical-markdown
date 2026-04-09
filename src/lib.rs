//! # Medical Markdown
//!
//! A [`markdown-it`] plugin that extends Markdown with clinical note-taking syntax.
//!
//! Clinicians can write shorthand like `PC/ chest pain` and Medical Markdown
//! will parse it into both rendered HTML and structured clinical data.
//!
//! ## Quick Start
//!
//! ```rust
//! let md = &mut markdown_it::MarkdownIt::new();
//! markdown_it::plugins::cmark::add(md);
//! medical_markdown::add(md);
//!
//! let ast = md.parse("PC/ chest pain\nHPC/ started 2 hours ago");
//! let html = ast.render();
//!
//! // Extract structured data from the AST
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

mod codes;
mod extract;
mod plugin;

pub use codes::{ClinicalCode, CLINICAL_CODES};
pub use extract::extract_structured_data;
pub use plugin::{add, MedicalNotes, MedicalSection, MedicalSubSection};
