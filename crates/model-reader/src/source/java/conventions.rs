/// Converts a Java camelCase field name to SQL snake_case column name.
///
/// Examples:
///   createdAt     → created_at
///   cpfCnpj       → cpf_cnpj
///   sentAt        → sent_at
///   firstName     → first_name
///   id            → id
pub fn camel_to_snake(name: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = name.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_word() {
        assert_eq!(camel_to_snake("id"), "id");
        assert_eq!(camel_to_snake("name"), "name");
    }

    #[test]
    fn test_two_words() {
        assert_eq!(camel_to_snake("createdAt"), "created_at");
        assert_eq!(camel_to_snake("sentAt"), "sent_at");
        assert_eq!(camel_to_snake("firstName"), "first_name");
    }

    #[test]
    fn test_multiple_words() {
        assert_eq!(camel_to_snake("cpfCnpj"), "cpf_cnpj");
        assert_eq!(camel_to_snake("streetName"), "street_name");
        assert_eq!(camel_to_snake("corporateReason"), "corporate_reason");
    }

    #[test]
    fn test_already_snake() {
        assert_eq!(camel_to_snake("id"), "id");
    }
}
