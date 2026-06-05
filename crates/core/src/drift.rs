use serde::{Deserialize, Serialize};

/// The full result of a drift check run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftReport {
    pub summary: DriftSummary,
    pub findings: Vec<DriftFinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftSummary {
    pub total: usize,
    pub critical: usize,
    pub warning: usize,
    pub info: usize,
    pub tables_checked: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftFinding {
    pub table: String,
    pub severity: DriftSeverity,
    pub kind: DriftKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DriftSeverity {
    Critical,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DriftKind {
    // Critical
    TableMissing,
    ColumnMissing {
        column: String,
    },
    ColumnTypeMismatch {
        column: String,
        expected: String,
        found: String,
    },
    NullabilityConflict {
        column: String,
        model_nullable: bool,
        db_nullable: bool,
    },

    // Warning
    LengthMismatch {
        column: String,
        model_length: u32,
        db_length: u32,
    },
    PrecisionMismatch {
        column: String,
        model_precision: u32,
        db_precision: u32,
    },
    IndexMissing {
        columns: Vec<String>,
    },
    UniqueConstraintMissing {
        columns: Vec<String>,
    },

    // Info
    ExtraColumn {
        column: String,
    },
    ExtraIndex {
        index_name: String,
    },
}
