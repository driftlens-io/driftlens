use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftLensConfig {
    pub database: DatabaseConfig,
    pub model: ModelConfig,
    pub rules: RulesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub schema: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub source_dirs: Vec<String>,
    pub bytecode_dirs: Vec<String>,
    pub strategy: ModelStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelStrategy {
    Source,
    Bytecode,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesConfig {
    pub blocking_severities: Vec<String>,
    pub ignore_extra_columns: bool,
    pub ignore_extra_indexes: bool,
    pub overrides: Vec<RuleOverride>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleOverride {
    pub table: String,
    pub column: Option<String>,
    pub ignore: Option<bool>,
    pub severity: Option<String>,
}
