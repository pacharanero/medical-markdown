//! Clinical code definitions mapping shorthand codes to full clinical headings.

use serde::{Deserialize, Serialize};

/// A clinical code with its abbreviation and full heading name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicalCode {
    /// Short code used in medical markdown (e.g. "PC")
    pub code: &'static str,
    /// Full clinical heading (e.g. "Presenting Complaint")
    pub heading: &'static str,
    /// Category for grouping related codes
    pub category: CodeCategory,
}

/// Categories for clinical codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeCategory {
    /// History-taking codes (PC, HPC, PMH, etc.)
    History,
    /// Physical examination codes (OE, RS, CVS, etc.)
    Examination,
    /// Assessment and plan codes (IMP, PLAN, etc.)
    Assessment,
    /// Vital signs (PR, BP, RR, etc.)
    Vitals,
    /// Other codes (TODO, etc.)
    Other,
}

/// Look up a clinical code by its abbreviation.
pub fn lookup(code: &str) -> Option<&'static ClinicalCode> {
    CLINICAL_CODES.iter().find(|c| c.code == code)
}

/// All recognised clinical codes.
pub static CLINICAL_CODES: &[ClinicalCode] = &[
    // History
    ClinicalCode { code: "PC", heading: "Presenting Complaint", category: CodeCategory::History },
    ClinicalCode { code: "HPC", heading: "History of Presenting Complaint", category: CodeCategory::History },
    ClinicalCode { code: "PMH", heading: "Past Medical History", category: CodeCategory::History },
    ClinicalCode { code: "DH", heading: "Drug History", category: CodeCategory::History },
    ClinicalCode { code: "FH", heading: "Family History", category: CodeCategory::History },
    ClinicalCode { code: "SH", heading: "Social History", category: CodeCategory::History },
    ClinicalCode { code: "ALLG", heading: "Allergies", category: CodeCategory::History },
    ClinicalCode { code: "ROS", heading: "Review of Systems", category: CodeCategory::History },

    // Examination
    ClinicalCode { code: "OE", heading: "On Examination", category: CodeCategory::Examination },
    ClinicalCode { code: "RS", heading: "Respiratory System", category: CodeCategory::Examination },
    ClinicalCode { code: "CVS", heading: "Cardiovascular System", category: CodeCategory::Examination },
    ClinicalCode { code: "GI", heading: "Gastrointestinal System", category: CodeCategory::Examination },
    ClinicalCode { code: "CNS", heading: "Central Nervous System", category: CodeCategory::Examination },
    ClinicalCode { code: "PNS", heading: "Peripheral Nervous System", category: CodeCategory::Examination },
    ClinicalCode { code: "MSK", heading: "Musculoskeletal System", category: CodeCategory::Examination },
    ClinicalCode { code: "DERM", heading: "Dermatology", category: CodeCategory::Examination },
    ClinicalCode { code: "ENT", heading: "Ear, Nose and Throat", category: CodeCategory::Examination },
    ClinicalCode { code: "EYE", heading: "Ophthalmology", category: CodeCategory::Examination },
    ClinicalCode { code: "GU", heading: "Genitourinary System", category: CodeCategory::Examination },
    ClinicalCode { code: "HS", heading: "Heart Sounds", category: CodeCategory::Examination },

    // Vitals
    ClinicalCode { code: "PR", heading: "Pulse Rate", category: CodeCategory::Vitals },
    ClinicalCode { code: "BP", heading: "Blood Pressure", category: CodeCategory::Vitals },
    ClinicalCode { code: "RR", heading: "Respiratory Rate", category: CodeCategory::Vitals },
    ClinicalCode { code: "TEMP", heading: "Temperature", category: CodeCategory::Vitals },
    ClinicalCode { code: "SPO2", heading: "Oxygen Saturation", category: CodeCategory::Vitals },
    ClinicalCode { code: "GCS", heading: "Glasgow Coma Scale", category: CodeCategory::Vitals },
    ClinicalCode { code: "AVPU", heading: "AVPU Score", category: CodeCategory::Vitals },

    // Assessment
    ClinicalCode { code: "IMP", heading: "Impression", category: CodeCategory::Assessment },
    ClinicalCode { code: "DIAG", heading: "Diagnosis", category: CodeCategory::Assessment },
    ClinicalCode { code: "DDX", heading: "Differential Diagnosis", category: CodeCategory::Assessment },
    ClinicalCode { code: "PLAN", heading: "Plan", category: CodeCategory::Assessment },
    ClinicalCode { code: "IX", heading: "Investigations", category: CodeCategory::Assessment },
    ClinicalCode { code: "RX", heading: "Treatment", category: CodeCategory::Assessment },

    // Other
    ClinicalCode { code: "TODO", heading: "Jobs List", category: CodeCategory::Other },
];
