use std::path::Path;

use anyhow::Result;
use driftlens_core::entity::EntityModel;

/// Common interface for all language parsers.
/// Implement this trait to add support for a new language or ORM.
///
/// # Example
///
/// ```
/// // To add Python/SQLAlchemy support:
/// // 1. Create crates/model-reader/src/source/python/
/// // 2. Implement LanguageParser for a PythonParser struct
/// // 3. Register it in source/mod.rs
/// ```
pub trait LanguageParser {
    /// Returns true if this parser can handle the given file extension.
    /// Examples: "java", "py", "go", "rb"
    fn supports(&self, file_extension: &str) -> bool;

    /// Parses a single source file and returns an EntityModel if the file
    /// contains a valid entity definition. Returns None if the file does
    /// not contain an entity (e.g. enums, DTOs, utilities).
    fn parse_file(&self, source: &str, path: &Path) -> Option<EntityModel>;

    /// Walks a directory recursively, finds all supported files,
    /// and returns all entity models found.
    fn parse_directory(&self, dir: &Path) -> Result<Vec<EntityModel>>;
}
