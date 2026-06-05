use driftlens_db_probe::probe::DbProbe;
use driftlens_db_probe::PostgresProbe;

fn database_url() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set to run integration tests")
}

#[tokio::test]
async fn test_introspect_users_table() {
    let probe = PostgresProbe::connect(&database_url())
        .await
        .expect("Failed to connect to database");

    let schemas = probe
        .introspect("public", &["users".to_string()])
        .await
        .expect("Failed to introspect schema");

    assert_eq!(schemas.len(), 1);

    let users = &schemas[0];
    assert_eq!(users.table_name, "users");

    let column_names: Vec<&str> = users.columns.iter().map(|c| c.name.as_str()).collect();

    assert!(column_names.contains(&"id"));
    assert!(column_names.contains(&"username"));
    assert!(column_names.contains(&"email"));
    assert!(column_names.contains(&"password"));
    assert!(column_names.contains(&"first_access"));

    let id_col = users.columns.iter().find(|c| c.name == "id").unwrap();
    assert!(!id_col.nullable);

    let email_col = users.columns.iter().find(|c| c.name == "email").unwrap();
    assert!(!email_col.nullable);
}

#[tokio::test]
async fn test_introspect_post_table() {
    let probe = PostgresProbe::connect(&database_url())
        .await
        .expect("Failed to connect to database");

    let schemas = probe
        .introspect("public", &["post".to_string()])
        .await
        .expect("Failed to introspect schema");

    assert_eq!(schemas.len(), 1);

    let post = &schemas[0];
    assert_eq!(post.table_name, "post");

    let column_names: Vec<&str> = post.columns.iter().map(|c| c.name.as_str()).collect();

    assert!(column_names.contains(&"id"));
    assert!(column_names.contains(&"title"));
    assert!(column_names.contains(&"status"));
    assert!(column_names.contains(&"author_id"));

    let status_col = post.columns.iter().find(|c| c.name == "status").unwrap();
    assert_eq!(status_col.data_type, "smallint");
    assert!(!status_col.nullable);

    let author_col = post.columns.iter().find(|c| c.name == "author_id").unwrap();
    assert!(!author_col.nullable);
}

#[tokio::test]
async fn test_introspect_multiple_tables() {
    let probe = PostgresProbe::connect(&database_url())
        .await
        .expect("Failed to connect to database");

    let tables = vec![
        "users".to_string(),
        "post".to_string(),
        "comment".to_string(),
    ];

    let schemas = probe
        .introspect("public", &tables)
        .await
        .expect("Failed to introspect schemas");

    assert_eq!(schemas.len(), 3);

    for schema in &schemas {
        assert!(!schema.columns.is_empty());
        assert!(!schema.table_name.is_empty());
    }
}

#[tokio::test]
async fn test_indexes_and_constraints() {
    let probe = PostgresProbe::connect(&database_url())
        .await
        .expect("Failed to connect to database");

    let schemas = probe
        .introspect("public", &["users".to_string()])
        .await
        .expect("Failed to introspect schema");

    let users = &schemas[0];

    assert!(!users.indexes.is_empty());
    assert!(!users.constraints.is_empty());

    let has_pk = users.constraints.iter().any(|c| {
        matches!(
            c.constraint_type,
            driftlens_core::schema::ConstraintType::PrimaryKey
        )
    });

    assert!(has_pk, "users table should have a primary key constraint");
}
