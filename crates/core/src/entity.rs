use serde::{Deserialize, Serialize};

/// Represents a JPA-mapped Java entity and its expected database structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityModel {
    pub entity_name: String,
    pub table_name: String,
    pub schema: Option<String>,
    pub columns: Vec<ColumnModel>,
    pub indexes: Vec<IndexModel>,
    pub unique_constraints: Vec<UniqueConstraint>,
    pub source: ModelSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnModel {
    pub field_name: String,
    pub column_name: String,
    pub nullable: bool,
    pub unique: bool,
    pub length: Option<u32>,
    pub precision: Option<u32>,
    pub scale: Option<u32>,
    pub insertable: bool,
    pub updatable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexModel {
    pub name: Option<String>,
    pub columns: Vec<String>,
    pub unique: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueConstraint {
    pub name: Option<String>,
    pub columns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelSource {
    SourceFile { path: String },
    Bytecode { path: String },
    Merged,
}
