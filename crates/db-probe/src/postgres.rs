use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use driftlens_core::schema::{
    ConstraintType, DatabaseSchema, DatabaseType, DbColumn, DbConstraint, DbIndex,
};

use crate::probe::DbProbe;

pub struct PostgresProbe {
    pool: PgPool,
}

impl PostgresProbe {
    pub async fn connect(database_url: &str) -> Result<Self> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl DbProbe for PostgresProbe {
    async fn introspect(&self, schema: &str, tables: &[String]) -> Result<Vec<DatabaseSchema>> {
        let mut results = Vec::new();

        for table in tables {
            let columns = fetch_columns(&self.pool, schema, table).await?;
            let indexes = fetch_indexes(&self.pool, schema, table).await?;
            let constraints = fetch_constraints(&self.pool, schema, table).await?;

            results.push(DatabaseSchema {
                table_name: table.clone(),
                schema_name: schema.to_string(),
                columns,
                indexes,
                constraints,
                db_type: DatabaseType::Postgres,
            });
        }

        Ok(results)
    }
}

struct ColumnRow {
    column_name: Option<String>,
    data_type: Option<String>,
    is_nullable: Option<String>,
    character_maximum_length: Option<i32>,
    numeric_precision: Option<i32>,
    numeric_scale: Option<i32>,
    column_default: Option<String>,
}

async fn fetch_columns(pool: &PgPool, schema: &str, table: &str) -> Result<Vec<DbColumn>> {
    let rows = sqlx::query_as!(
        ColumnRow,
        r#"
        SELECT
            column_name,
            data_type,
            is_nullable,
            character_maximum_length,
            numeric_precision,
            numeric_scale,
            column_default
        FROM information_schema.columns
        WHERE table_schema = $1
          AND table_name   = $2
        ORDER BY ordinal_position
        "#,
        schema,
        table
    )
    .fetch_all(pool)
    .await?;

    let columns = rows
        .into_iter()
        .map(|row| DbColumn {
            name: row.column_name.unwrap_or_default(),
            data_type: row.data_type.unwrap_or_default(),
            nullable: row.is_nullable.as_deref() == Some("YES"),
            character_maximum_length: row.character_maximum_length.map(|v| v as u32),
            numeric_precision: row.numeric_precision.map(|v| v as u32),
            numeric_scale: row.numeric_scale.map(|v| v as u32),
            column_default: row.column_default,
        })
        .collect();

    Ok(columns)
}

async fn fetch_indexes(pool: &PgPool, schema: &str, table: &str) -> Result<Vec<DbIndex>> {
    let rows = sqlx::query!(
        r#"
        SELECT
            i.relname AS "index_name!: String",
            ix.indisunique AS "is_unique!: bool",
            array_agg(a.attname ORDER BY array_position(ix.indkey, a.attnum)) AS "columns!: Vec<String>"
        FROM pg_class t
        JOIN pg_index ix     ON t.oid = ix.indrelid
        JOIN pg_class i      ON i.oid = ix.indexrelid
        JOIN pg_attribute a  ON a.attrelid = t.oid AND a.attnum = ANY(ix.indkey)
        JOIN pg_namespace n  ON n.oid = t.relnamespace
        WHERE n.nspname = $1
          AND t.relname = $2
          AND t.relkind = 'r'
        GROUP BY i.relname, ix.indisunique
        ORDER BY i.relname
        "#,
        schema,
        table
    )
    .fetch_all(pool)
    .await?;

    let indexes = rows
        .into_iter()
        .map(|row| DbIndex {
            name: row.index_name,
            columns: row.columns,
            unique: row.is_unique,
        })
        .collect();

    Ok(indexes)
}

async fn fetch_constraints(pool: &PgPool, schema: &str, table: &str) -> Result<Vec<DbConstraint>> {
    let rows = sqlx::query!(
        r#"
        SELECT
            tc.constraint_name AS "constraint_name!: String",
            tc.constraint_type AS "constraint_type!: String",
            array_agg(kcu.column_name ORDER BY kcu.ordinal_position) AS "columns!: Vec<String>"
        FROM information_schema.table_constraints tc
        JOIN information_schema.key_column_usage kcu
          ON tc.constraint_name = kcu.constraint_name
         AND tc.table_schema    = kcu.table_schema
        WHERE tc.table_schema = $1
          AND tc.table_name   = $2
        GROUP BY tc.constraint_name, tc.constraint_type
        ORDER BY tc.constraint_name
        "#,
        schema,
        table
    )
    .fetch_all(pool)
    .await?;

    let constraints = rows
        .into_iter()
        .filter_map(|row| {
            let constraint_type = match row.constraint_type.as_str() {
                "PRIMARY KEY" => ConstraintType::PrimaryKey,
                "FOREIGN KEY" => ConstraintType::ForeignKey,
                "UNIQUE" => ConstraintType::Unique,
                "CHECK" => ConstraintType::Check,
                _ => return None,
            };

            Some(DbConstraint {
                name: row.constraint_name,
                constraint_type,
                columns: row.columns,
            })
        })
        .collect();

    Ok(constraints)
}
