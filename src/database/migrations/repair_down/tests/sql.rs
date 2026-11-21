use super::super::sql;

#[test]
fn reverses_schema_statements_and_columns_in_order() {
    let undo = sql::infer(concat!(
        "BEGIN; CREATE TABLE app.example (id INTEGER); ",
        "ALTER TABLE app.example ADD COLUMN first TEXT, ADD COLUMN second INTEGER; ",
        "CREATE INDEX example_first ON app.example (first); COMMIT;",
    ))
    .unwrap();
    let expected = concat!(
        "BEGIN;\n",
        "DROP INDEX app.example_first RESTRICT;\n",
        "ALTER TABLE app.example DROP COLUMN second RESTRICT;\n",
        "ALTER TABLE app.example DROP COLUMN first RESTRICT;\n",
        "DROP TABLE app.example RESTRICT;\n",
        "COMMIT;\n",
    );
    assert!(undo.ends_with(expected), "{undo}");
}

#[test]
fn retains_postgres_identifier_quoting_and_only() {
    let undo = sql::infer(concat!(
        "ALTER TABLE ONLY \"App\".\"Mixed Case\" ADD COLUMN \"Text\" TEXT; ",
        "CREATE INDEX \"Mixed Index\" ON \"App\".\"Mixed Case\" (\"Text\");",
    ))
    .unwrap();
    assert!(undo.contains("DROP INDEX \"App\".\"Mixed Index\" RESTRICT;"));
    assert!(undo.contains("ALTER TABLE ONLY \"App\".\"Mixed Case\" DROP COLUMN \"Text\" RESTRICT;"));
}

#[test]
fn conditional_ddl_requires_reviewed_ownership() {
    for source in [
        "CREATE TABLE IF NOT EXISTS example (id INTEGER);",
        "CREATE INDEX IF NOT EXISTS example_idx ON example (id);",
        "ALTER TABLE example ADD COLUMN IF NOT EXISTS id INTEGER;",
        "ALTER TABLE IF EXISTS example ADD COLUMN id INTEGER;",
    ] {
        assert!(sql::infer(source).is_err(), "{source}");
    }
}

#[test]
fn unsupported_operations_never_produce_partial_undo() {
    for source in [
        "CREATE TABLE example (id INTEGER); INSERT INTO example VALUES (1);",
        "CREATE TABLE example (id INTEGER); UPDATE example SET id = 2;",
        "DELETE FROM example;",
        "DROP TABLE example;",
        "ALTER TABLE example ADD COLUMN id INTEGER, DROP COLUMN old;",
        "ALTER TABLE example RENAME COLUMN old TO new;",
        "CREATE INDEX CONCURRENTLY example_idx ON example (id);",
        "CREATE TEMP TABLE example (id INTEGER);",
        "CREATE TABLE example AS SELECT 1 AS id;",
        "CREATE TABLE example (id INTEGER) INHERITS (parent);",
        "CREATE VIEW example AS SELECT 1 AS id;",
        "SET search_path TO app; CREATE TABLE example (id INTEGER);",
        "BEGIN; CREATE TABLE example (id INTEGER); ROLLBACK;",
        "BEGIN; BEGIN; CREATE TABLE example (id INTEGER); COMMIT; COMMIT;",
    ] {
        assert!(sql::infer(source).is_err(), "{source}");
    }
}

#[test]
fn empty_invalid_and_non_postgres_identifiers_are_rejected() {
    for source in [
        "",
        "-- comment only\n;",
        "BEGIN; COMMIT;",
        "CREATE TABLE example (id INTEGER); nonsense trailing tokens;",
        "CREATE TABLE example (id INTEGER);\0",
        "CREATE TABLE catalog.app.example (id INTEGER);",
        "CREATE TABLE `example` (id INTEGER);",
    ] {
        assert!(sql::infer(source).is_err(), "{source:?}");
    }
}
