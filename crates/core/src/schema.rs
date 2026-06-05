use serde::{Deserialize, Serialize};

/// Represents the real schema as introspected from the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSchema {
    pub table_name: String,
    pub schema_name: String,
    pub columns: Vec<DbColumn>,
    pub indexes: Vec<DbIndex>,
    pub constraints: Vec<DbConstraint>,
    pub db_type: DatabaseType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbColumn {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub character_maximum_length: Option<u32>,
    pub numeric_precision: Option<u32>,
    pub numeric_scale: Option<u32>,
    pub column_default: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbIndex {
    pub name: String,
    pub columns: Vec<String>,
    pub unique: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbConstraint {
    pub name: String,
    pub constraint_type: ConstraintType,
    pub columns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintType {
    PrimaryKey,
    ForeignKey,
    Unique,
    Check,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseType {
    Postgres,
    Oracle,
}