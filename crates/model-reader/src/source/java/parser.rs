use std::path::Path;

use anyhow::Result;
use regex::Regex;
use walkdir::WalkDir;

use driftlens_core::entity::EntityModel;

use super::conventions::camel_to_snake;
use crate::source::traits::LanguageParser;

pub struct JavaParser;

impl JavaParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for JavaParser {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageParser for JavaParser {
    fn supports(&self, file_extension: &str) -> bool {
        file_extension == "java"
    }

    fn parse_file(&self, source: &str, path: &Path) -> Option<EntityModel> {
        parse_file(source, path)
    }

    fn parse_directory(&self, dir: &Path) -> Result<Vec<EntityModel>> {
        parse_directory(dir)
    }
}

pub fn parse_directory(source_dir: &Path) -> Result<Vec<EntityModel>> {
    let parser = JavaParser::new();
    let mut entities = Vec::new();

    for entry in WalkDir::new(source_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map_or(false, |ext| parser.supports(ext.to_str().unwrap_or("")))
        })
    {
        let content = std::fs::read_to_string(entry.path())?;
        if let Some(entity) = parse_file(&content, entry.path()) {
            entities.push(entity);
        }
    }

    Ok(entities)
}

pub fn parse_file(source: &str, path: &Path) -> Option<EntityModel> {
    // Must have @Entity
    if !source.contains("@Entity") {
        return None;
    }

    let table_name = extract_table_name(source)?;
    let schema = extract_schema_name(source);
    let class_name = extract_class_name(source)?;

    Some(EntityModel {
        entity_name: class_name,
        table_name,
        schema,
        columns: vec![],
        indexes: vec![],
        unique_constraints: vec![],
        source: driftlens_core::entity::ModelSource::SourceFile {
            path: path.to_string_lossy().to_string(),
        },
    })
}

fn extract_table_name(source: &str) -> Option<String> {
    // @Table(name = "table_name") or @Table(name="table_name")
    let re = Regex::new(r#"@Table\s*\([^)]*name\s*=\s*"([^"]+)""#).ok()?;
    if let Some(cap) = re.captures(source) {
        return Some(cap[1].to_string());
    }

    // @Table without name — fallback to class name convention
    let class_name = extract_class_name(source)?;
    Some(camel_to_snake(&class_name))
}

fn extract_schema_name(source: &str) -> Option<String> {
    let re = Regex::new(r#"@Table\s*\([^)]*schema\s*=\s*"([^"]+)""#).ok()?;
    re.captures(source).map(|cap| cap[1].to_string())
}

fn extract_class_name(source: &str) -> Option<String> {
    let re = Regex::new(r"public\s+class\s+(\w+)").ok()?;
    re.captures(source).map(|cap| cap[1].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_table_name_explicit() {
        let source = r#"
            @Entity
            @Table(name = "abstinence")
            public class Abstinence {}
        "#;
        let entity = parse_file(source, Path::new("Abstinence.java")).unwrap();
        assert_eq!(entity.table_name, "abstinence");
    }

    #[test]
    fn test_extract_table_name_no_spaces() {
        let source = r#"
            @Entity
            @Table(name="office")
            public class Office {}
        "#;
        let entity = parse_file(source, Path::new("Office.java")).unwrap();
        assert_eq!(entity.table_name, "office");
    }

    #[test]
    fn test_extract_class_name() {
        let source = r#"
            @Entity
            @Table(name = "segment")
            public class Segment {}
        "#;
        let entity = parse_file(source, Path::new("Segment.java")).unwrap();
        assert_eq!(entity.entity_name, "Segment");
    }

    #[test]
    fn test_skip_non_entity() {
        let source = r#"
            public class GoalStatus {
                ACTIVE, INACTIVE
            }
        "#;
        assert!(parse_file(source, Path::new("GoalStatus.java")).is_none());
    }

    #[test]
    fn test_extract_schema_name() {
        let source = r#"
            @Entity
            @Table(name = "users", schema = "myschema")
            public class User {}
        "#;
        let entity = parse_file(source, Path::new("User.java")).unwrap();
        assert_eq!(entity.schema, Some("myschema".to_string()));
    }

    #[test]
    fn test_table_name_fallback_to_convention() {
        // @Entity without @Table — fallback to snake_case of class name
        let source = r#"
            @Entity
            public class UserProfile {}
        "#;
        let entity = parse_file(source, Path::new("UserProfile.java")).unwrap();
        assert_eq!(entity.table_name, "user_profile");
    }

    #[test]
    fn test_language_parser_trait() {
        let parser = JavaParser::new();
        assert!(parser.supports("java"));
        assert!(!parser.supports("py"));
        assert!(!parser.supports("go"));
    }

    #[test]
    fn test_parse_all_merito_entities() {
        let entities = vec![
            (
                r#"@Entity @Table(name = "abstinence") public class Abstinence {}"#,
                "abstinence",
            ),
            (
                r#"@Entity @Table(name = "address") public class Address {}"#,
                "address",
            ),
            (
                r#"@Entity @Table(name = "appointment") public class Appointment {}"#,
                "appointment",
            ),
            (
                r#"@Entity @Table(name = "contact") public class Contact {}"#,
                "contact",
            ),
            (
                r#"@Entity @Table(name = "document") public class Document {}"#,
                "document",
            ),
            (
                r#"@Entity @Table(name = "goal") public class Goal {}"#,
                "goal",
            ),
            (
                r#"@Entity @Table(name = "office") public class Office {}"#,
                "office",
            ),
            (
                r#"@Entity @Table(name = "routine") public class Routine {}"#,
                "routine",
            ),
            (
                r#"@Entity @Table(name = "segment") public class Segment {}"#,
                "segment",
            ),
            (r#"@Entity @Table(name = "tag") public class Tag {}"#, "tag"),
            (
                r#"@Entity @Table(name = "task") public class Task {}"#,
                "task",
            ),
        ];

        for (source, expected_table) in entities {
            let entity = parse_file(source, Path::new("Test.java")).unwrap();
            assert_eq!(entity.table_name, expected_table);
        }
    }
}
