use anyhow::Result;
use driftlens_core::schema::DatabaseSchema;

/// Common interface for all database probes.
/// Implement this trait to add support for a new database.
#[async_trait::async_trait]
pub trait DbProbe {
    async fn introspect(&self, schema: &str, tables: &[String]) -> Result<Vec<DatabaseSchema>>;
}