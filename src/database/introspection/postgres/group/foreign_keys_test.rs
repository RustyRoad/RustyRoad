//! Read-only PostgreSQL regression: interleaved rows with reused constraint names.

use super::foreign_keys;
use crate::database::introspection::{assemble::attach, ForeignKey, Schema, Table};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

/// Opens an explicitly selected test connection without logging credentials.
async fn pool() -> sqlx::PgPool {
    let path = std::env::var("RUSTYROAD_FK_TEST_CONFIG")
        .expect("set RUSTYROAD_FK_TEST_CONFIG to a development rustyroad.toml");
    let source = std::fs::read_to_string(path).expect("read test configuration");
    let config: toml::Value = toml::from_str(&source).expect("parse test configuration");
    let db = &config["database"];
    let text = |key: &str| db[key].as_str().expect("database configuration string");
    let options = PgConnectOptions::new()
        .host(text("database_host"))
        .port(text("database_port").parse().expect("database port"))
        .username(text("database_user"))
        .password(text("database_password"))
        .database(text("database_name"));
    PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .expect("connect to test PostgreSQL")
}

/// Same names must not merge owners, ordered column pairs, targets, or actions.
#[tokio::test]
#[ignore = "requires RUSTYROAD_FK_TEST_CONFIG; executes only a read-only VALUES query"]
async fn foreign_keys_preserve_table_scope_pairs_and_actions() {
    let pool = pool().await;
    let rows = sqlx::query(
        r#"SELECT table_name, name, column_name, foreign_table, foreign_column,
                  on_delete::"char" AS on_delete, on_update::"char" AS on_update
             FROM (VALUES
               (1, 'first', 'shared_fk', 'z_id', 'parent_one', 'b_id', 'c', 'r'),
               (2, 'second', 'shared_fk', 'media_id', 'media_library', 'id', 'n', 'c'),
               (3, 'first', 'shared_fk', 'a_id', 'parent_one', 'a_id', 'c', 'r'),
               (4, 'first', 'other_fk', 'owner_id', 'users', 'id', 'a', 'd')
             ) AS rows(position, table_name, name, column_name, foreign_table,
                       foreign_column, on_delete, on_update)
             ORDER BY position"#,
    )
    .fetch_all(&pool)
    .await
    .expect("read PostgreSQL fixture rows");
    let mut grouped = foreign_keys(rows);
    assert_eq!(
        grouped.len(),
        3,
        "same names on different tables stay separate"
    );
    // The tuple is authoritative, not the redundant legacy table field.
    for parts in grouped.values_mut() {
        parts.table.clear();
    }
    let mut schema = Schema {
        tables: ["second", "first"]
            .map(|name| Table {
                name: name.into(),
                ..Table::default()
            })
            .to_vec(),
        ..Schema::default()
    };
    attach::foreign_keys(&mut schema, grouped);
    assert_eq!(
        schema.table("first").unwrap().foreign_keys,
        vec![
            ForeignKey {
                name: "other_fk".into(),
                columns: vec!["owner_id".into()],
                foreign_table: "users".into(),
                foreign_columns: vec!["id".into()],
                on_delete: None,
                on_update: Some("set default".into()),
            },
            ForeignKey {
                name: "shared_fk".into(),
                columns: vec!["z_id".into(), "a_id".into()],
                foreign_table: "parent_one".into(),
                foreign_columns: vec!["b_id".into(), "a_id".into()],
                on_delete: Some("cascade".into()),
                on_update: Some("restrict".into()),
            },
        ]
    );
    assert_eq!(
        schema.table("second").unwrap().foreign_keys,
        vec![ForeignKey {
            name: "shared_fk".into(),
            columns: vec!["media_id".into()],
            foreign_table: "media_library".into(),
            foreign_columns: vec!["id".into()],
            on_delete: Some("set null".into()),
            on_update: Some("cascade".into()),
        }]
    );
    let mut missing = Schema::default();
    attach::foreign_keys(&mut missing, foreign_keys(Vec::new()));
    assert!(missing.tables.is_empty());
    pool.close().await;
}
