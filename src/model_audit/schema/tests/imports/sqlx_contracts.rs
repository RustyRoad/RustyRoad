//! Source-only SQLx fixtures establish declared names, not compiled codec behavior.
use super::audits;

#[test]
fn named_enum_with_variant_renaming_supplies_storage() {
    for rule in [
        "lowercase",
        "snake_case",
        "UPPERCASE",
        "SCREAMING_SNAKE_CASE",
        "kebab-case",
        "camelCase",
        "PascalCase",
    ] {
        let result = audits(&format!(
            r#"
            use sqlx::Type as Codec;
            #[derive(Codec)]
            #[sqlx(type_name = "RoomState", rename_all = "{rule}")]
            enum State {{ Ready, Waiting }}
            struct Row {{ id: i32, state: Option<State> }}
        "#
        ));
        let row = result.last().unwrap();
        assert!(row.blockers.is_empty(), "{:?}", row.blockers);
        assert_eq!(row.desired_columns[1].sql_type, "\"RoomState\"");
        assert!(row.desired_columns[1].nullable);
    }
}

#[test]
fn literal_manual_postgres_contract_supplies_storage() {
    let result = audits(
        r#"
        use sqlx::{Type as Codec, Postgres as Pg, postgres::PgTypeInfo as Info};
        struct Point;
        impl Codec<Pg> for Point {
            fn type_info() -> Info { Info::with_name("geometry") }
        }
        struct Row { id: i32, point: Point }
    "#,
    );
    let row = result.last().unwrap();
    assert!(row.blockers.is_empty(), "{:?}", row.blockers);
    assert_eq!(row.desired_columns[1].sql_type, "\"geometry\"");
}

mod rejected;

mod configured;
