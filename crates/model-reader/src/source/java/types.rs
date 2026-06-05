/// Represents how a Java enum is mapped to SQL.
#[derive(Debug, Clone, PartialEq)]
pub enum EnumMapping {
    /// @Enumerated(EnumType.ORDINAL) or no @Enumerated → smallint
    Ordinal,
    /// @Enumerated(EnumType.STRING) → varchar
    String,
}

/// The result of mapping a Java type to its SQL equivalent.
#[derive(Debug, Clone)]
pub struct TypeMapping {
    pub sql_type: std::string::String,
    pub length: Option<u32>,
    pub precision: Option<u32>,
    pub scale: Option<u32>,
}

impl TypeMapping {
    fn new(sql_type: &str) -> Self {
        Self {
            sql_type: sql_type.to_string(),
            length: None,
            precision: None,
            scale: None,
        }
    }

    fn with_length(mut self, length: u32) -> Self {
        self.length = Some(length);
        self
    }
}

/// Maps a Java type to its SQL column type.
///
/// # Arguments
/// - `java_type` — simple class name (e.g. "String", "Instant", "GoalStatus")
/// - `enum_mapping` — present when the field has @Enumerated
/// - `column_length` — value from @Column(length = N)
/// - `id_strategy` — "IDENTITY" or "UUID" when field has @GeneratedValue
pub fn map_java_type(
    java_type: &str,
    enum_mapping: Option<&EnumMapping>,
    column_length: Option<u32>,
    id_strategy: Option<&str>,
) -> TypeMapping {
    // Enums — type depends on @Enumerated annotation
    if let Some(mapping) = enum_mapping {
        return match mapping {
            EnumMapping::String => {
                TypeMapping::new("character varying").with_length(column_length.unwrap_or(255))
            }
            EnumMapping::Ordinal => TypeMapping::new("smallint"),
        };
    }

    match java_type {
        // String
        "String" => TypeMapping::new("character varying").with_length(column_length.unwrap_or(255)),

        // Integer types
        "Long" | "long" => {
            if id_strategy == Some("IDENTITY") {
                TypeMapping::new("bigint")
            } else {
                TypeMapping::new("bigint")
            }
        }
        "Integer" | "int" => TypeMapping::new("integer"),
        "Short" | "short" => TypeMapping::new("smallint"),

        // Boolean
        "Boolean" | "boolean" => TypeMapping::new("boolean"),

        // Decimal
        "BigDecimal" => TypeMapping {
            sql_type: "numeric".to_string(),
            length: None,
            precision: Some(19),
            scale: Some(2),
        },
        "Double" | "double" => TypeMapping::new("double precision"),
        "Float" | "float" => TypeMapping::new("real"),

        // Date / Time
        "LocalDate" => TypeMapping::new("date"),
        "LocalDateTime" => TypeMapping::new("timestamp without time zone"),
        "Instant" => TypeMapping::new("timestamp with time zone"),
        "LocalTime" => TypeMapping::new("time without time zone"),

        // Other
        "UUID" => TypeMapping::new("uuid"),
        "byte[]" => TypeMapping::new("bytea"),

        // Unknown — caller decides what to do
        _ => TypeMapping::new("unknown"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_default_length() {
        let m = map_java_type("String", None, None, None);
        assert_eq!(m.sql_type, "character varying");
        assert_eq!(m.length, Some(255));
    }

    #[test]
    fn test_string_explicit_length() {
        let m = map_java_type("String", None, Some(14), None);
        assert_eq!(m.sql_type, "character varying");
        assert_eq!(m.length, Some(14));
    }

    #[test]
    fn test_instant() {
        let m = map_java_type("Instant", None, None, None);
        assert_eq!(m.sql_type, "timestamp with time zone");
    }

    #[test]
    fn test_local_date() {
        let m = map_java_type("LocalDate", None, None, None);
        assert_eq!(m.sql_type, "date");
    }

    #[test]
    fn test_local_date_time() {
        let m = map_java_type("LocalDateTime", None, None, None);
        assert_eq!(m.sql_type, "timestamp without time zone");
    }

    #[test]
    fn test_boolean() {
        let m = map_java_type("Boolean", None, None, None);
        assert_eq!(m.sql_type, "boolean");
        let m2 = map_java_type("boolean", None, None, None);
        assert_eq!(m2.sql_type, "boolean");
    }

    #[test]
    fn test_long() {
        let m = map_java_type("Long", None, None, None);
        assert_eq!(m.sql_type, "bigint");
    }

    #[test]
    fn test_enum_ordinal() {
        let m = map_java_type("GoalStatus", Some(&EnumMapping::Ordinal), None, None);
        assert_eq!(m.sql_type, "smallint");
    }

    #[test]
    fn test_enum_string_default_length() {
        let m = map_java_type("ERole", Some(&EnumMapping::String), None, None);
        assert_eq!(m.sql_type, "character varying");
        assert_eq!(m.length, Some(255));
    }

    #[test]
    fn test_enum_string_explicit_length() {
        let m = map_java_type("ERole", Some(&EnumMapping::String), Some(20), None);
        assert_eq!(m.sql_type, "character varying");
        assert_eq!(m.length, Some(20));
    }

    #[test]
    fn test_unknown_type() {
        let m = map_java_type("SomeCustomClass", None, None, None);
        assert_eq!(m.sql_type, "unknown");
    }
}
