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
    if !source.contains("@Entity") {
        return None;
    }

    let table_name = extract_table_name(source)?;
    let schema = extract_schema_name(source);
    let class_name = extract_class_name(source)?;
    let columns = extract_columns(source);

    Some(EntityModel {
        entity_name: class_name,
        table_name,
        schema,
        columns,
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

fn extract_columns(source: &str) -> Vec<driftlens_core::entity::ColumnModel> {
    let mut columns = Vec::new();
    let field_blocks = extract_field_blocks(source);

    for block in field_blocks {
        if let Some(column) = parse_field_block(&block) {
            columns.push(column);
        }
    }

    columns
}

/// Splits the class body into individual field blocks.
/// Each block contains the annotations + field declaration for one field.
fn extract_field_blocks(source: &str) -> Vec<String> {
    let mut blocks: Vec<String> = Vec::new();

    let class_body_start = match source.find('{') {
        Some(pos) => pos,
        None => return blocks,
    };

    let body = &source[class_body_start..];
    let lines: Vec<&str> = body.lines().collect();

    let mut current_block: Vec<&str> = Vec::new();

    for line in &lines {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        // Method or inner class — flush and skip
        if (trimmed.starts_with("public ")
            || trimmed.starts_with("private ")
            || trimmed.starts_with("protected "))
            && trimmed.contains('(')
        {
            if !current_block.is_empty() && current_block.iter().any(|l| is_field_declaration(l)) {
                blocks.push(current_block.join("\n"));
            }
            current_block.clear();
            continue;
        }

        // Field declaration — add to current block then flush
        if is_field_declaration(trimmed) {
            current_block.push(trimmed);
            blocks.push(current_block.join("\n"));
            current_block.clear();
            continue;
        }

        // Annotation or modifier — if we have a pending block with a field, flush first
        if trimmed.starts_with('@') && current_block.iter().any(|l| is_field_declaration(l)) {
            blocks.push(current_block.join("\n"));
            current_block.clear();
        }

        current_block.push(trimmed);
    }

    // Push last block
    if !current_block.is_empty() && current_block.iter().any(|l| is_field_declaration(l)) {
        blocks.push(current_block.join("\n"));
    }

    blocks
}

fn is_field_declaration(line: &str) -> bool {
    let trimmed = line.trim();
    (trimmed.starts_with("private ")
        || trimmed.starts_with("public ")
        || trimmed.starts_with("protected "))
        && !trimmed.contains('(')
        && trimmed.ends_with(';')
}

fn parse_field_block(block: &str) -> Option<driftlens_core::entity::ColumnModel> {
    // Skip reverse side of relationships — no physical column
    if block.contains("mappedBy") {
        return None;
    }

    // Skip @Transient
    if block.contains("@Transient") {
        return None;
    }

    // Skip collections without @JoinColumn (e.g. @OneToMany List<X>)
    if (block.contains("List<") || block.contains("Set<") || block.contains("Map<"))
        && !block.contains("@JoinColumn")
    {
        return None;
    }

    let field_declaration = block.lines().find(|l| is_field_declaration(l))?;
    let (java_type, field_name) = extract_type_and_name(field_declaration)?;

    // Column name — from @Column(name=) or @JoinColumn(name=) or convention
    let column_name = if let Some(name) = extract_attribute(block, "@Column", "name") {
        name
    } else if let Some(name) = extract_attribute(block, "@JoinColumn", "name") {
        name
    } else {
        camel_to_snake(&field_name)
    };

    let is_id = block.contains("@Id");

    // Nullable
    let nullable = if is_id {
        false
    } else if let Some(val) = extract_attribute(block, "@Column", "nullable") {
        val != "false"
    } else if let Some(val) = extract_attribute(block, "@JoinColumn", "nullable") {
        val != "false"
    } else if block.contains("optional = false") {
        false
    } else {
        true
    };

    // Length from @Column(length = N)
    let length = extract_attribute(block, "@Column", "length").and_then(|v| v.parse::<u32>().ok());

    // Enum mapping
    let enum_mapping = if block.contains("EnumType.STRING") {
        Some(super::types::EnumMapping::String)
    } else if block.contains("@Enumerated") {
        Some(super::types::EnumMapping::Ordinal)
    } else {
        None
    };

    // ID generation strategy
    let id_strategy = if block.contains("GenerationType.IDENTITY") {
        Some("IDENTITY")
    } else if block.contains("GenerationType.UUID") {
        Some("UUID")
    } else {
        None
    };

    // FK fields always map to varchar (references String id)
    let effective_type = if block.contains("@JoinColumn") && !is_id {
        "String".to_string()
    } else {
        java_type
    };

    let type_mapping =
        super::types::map_java_type(&effective_type, enum_mapping.as_ref(), length, id_strategy);

    // Skip unresolvable types silently — will be reported as unsupported
    if type_mapping.sql_type == "unknown" {
        return None;
    }

    Some(driftlens_core::entity::ColumnModel {
        field_name,
        column_name,
        nullable,
        unique: block.contains("unique = true"),
        length: type_mapping.length,
        precision: type_mapping.precision,
        scale: type_mapping.scale,
        insertable: !block.contains("insertable = false"),
        updatable: !block.contains("updatable = false"),
    })
}

fn extract_type_and_name(field_declaration: &str) -> Option<(String, String)> {
    let trimmed = field_declaration
        .trim()
        .trim_start_matches("private ")
        .trim_start_matches("public ")
        .trim_start_matches("protected ")
        .trim_start_matches("static ")
        .trim_start_matches("final ");

    let without_semi = trimmed.trim_end_matches(';').split('=').next()?.trim();

    let parts: Vec<&str> = without_semi.splitn(2, ' ').collect();
    if parts.len() != 2 {
        return None;
    }

    let java_type = parts[0]
        .trim()
        .split('<')
        .next()
        .unwrap_or(parts[0])
        .to_string();
    let name = parts[1].trim().to_string();

    Some((java_type, name))
}

fn extract_attribute(block: &str, annotation: &str, attribute: &str) -> Option<String> {
    let pattern = format!(
        "{}[^)]*{}\\s*=\\s*\"?([^\",\\)\\s]+)\"?",
        regex::escape(annotation),
        regex::escape(attribute)
    );
    let re = Regex::new(&pattern).ok()?;
    re.captures(block).map(|cap| cap[1].trim().to_string())
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

    #[test]
    fn test_simple_columns() {
        let source = r#"
        @Entity
        @Table(name = "segment")
        public class Segment {
            @Id
            @GeneratedValue(strategy = GenerationType.UUID)
            private String id;

            @Column(name = "name")
            private String name;

            @Column(name = "created_at")
            private Instant createdAt;
        }
    "#;

        let entity = parse_file(source, Path::new("Segment.java")).unwrap();
        let col_names: Vec<&str> = entity
            .columns
            .iter()
            .map(|c| c.column_name.as_str())
            .collect();

        assert!(col_names.contains(&"id"));
        assert!(col_names.contains(&"name"));
        assert!(col_names.contains(&"created_at"));

        let id = entity
            .columns
            .iter()
            .find(|c| c.column_name == "id")
            .unwrap();
        assert!(!id.nullable);
    }

    #[test]
    fn test_column_without_name_uses_convention() {
        let source = r#"
        @Entity
        @Table(name = "message")
        public class Message {
            @Id
            private Long id;

            private String content;

            private LocalDateTime sentAt;
        }
    "#;

        let entity = parse_file(source, Path::new("Message.java")).unwrap();
        let col_names: Vec<&str> = entity
            .columns
            .iter()
            .map(|c| c.column_name.as_str())
            .collect();

        assert!(col_names.contains(&"sent_at"));
        assert!(col_names.contains(&"content"));
    }

    #[test]
    fn test_fk_column() {
        let source = r#"
        @Entity
        @Table(name = "routine")
        public class Routine {
            @Id
            private String id;

            @ManyToOne(fetch = FetchType.EAGER, optional = false)
            @JoinColumn(name = "responsibility_id", nullable = false)
            private Responsibility responsibility;
        }
    "#;

        let entity = parse_file(source, Path::new("Routine.java")).unwrap();
        let fk = entity
            .columns
            .iter()
            .find(|c| c.column_name == "responsibility_id")
            .unwrap();
        assert!(!fk.nullable);
    }

    #[test]
    fn test_enum_string_column() {
        let source = r#"
        @Entity
        @Table(name = "roles")
        public class Role {
            @Id
            private String id;

            @Enumerated(EnumType.STRING)
            @Column(length = 20)
            private ERole name;
        }
    "#;

        let entity = parse_file(source, Path::new("Role.java")).unwrap();
        let col = entity
            .columns
            .iter()
            .find(|c| c.column_name == "name")
            .unwrap();
        assert_eq!(col.length, Some(20));
    }

    #[test]
    fn test_skip_one_to_many_mapped_by() {
        let source = r#"
        @Entity
        @Table(name = "document")
        public class Document {
            @Id
            private String id;

            @OneToMany(mappedBy = "document", cascade = CascadeType.ALL)
            private List<DocumentReader> readers = new ArrayList<>();
        }
    "#;

        let entity = parse_file(source, Path::new("Document.java")).unwrap();
        let col_names: Vec<&str> = entity
            .columns
            .iter()
            .map(|c| c.column_name.as_str())
            .collect();
        assert!(!col_names.contains(&"readers"));
    }
}
