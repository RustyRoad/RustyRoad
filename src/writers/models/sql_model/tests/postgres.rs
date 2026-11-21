use super::*;

#[test]
fn mixed_types_and_blank_lines_use_column_declarations() {
    let actual = fields(
        r#"
        -- Deliberately indented, with blank lines and embedded punctuation.
        CREATE TABLE public.widgets (
            id BIGSERIAL PRIMARY KEY,

            name TEXT NOT NULL DEFAULT 'semi;colon',
            price NUMERIC(12,2),
            enabled BOOLEAN NOT NULL,
            created TIMESTAMP(3) WITH TIME ZONE,
            labels TEXT[],
            CONSTRAINT valid_price CHECK (price >= 0)
        );
        CREATE INDEX widgets_name ON public.widgets(name);
        COMMENT ON TABLE public.widgets IS 'not another;statement';
    "#,
        DatabaseType::Postgres,
    );
    assert_eq!(
        actual,
        vec![
            ("id".into(), "i64".into()),
            ("name".into(), "String".into()),
            ("price".into(), "Option<sqlx::types::Decimal>".into()),
            ("enabled".into(), "bool".into()),
            (
                "created".into(),
                "Option<chrono::DateTime<chrono::Utc>>".into()
            ),
            ("labels".into(), "Option<Vec<String>>".into()),
        ]
    );
}

#[test]
fn composite_primary_keys_are_required_but_unique_columns_are_not() {
    let actual = fields(
        "CREATE TABLE t (a UUID, b INT, c TEXT UNIQUE, PRIMARY KEY(a,b))",
        DatabaseType::Postgres,
    );
    assert_eq!(actual[0].1, "uuid::Uuid");
    assert_eq!(actual[1].1, "i32");
    assert_eq!(actual[2].1, "Option<String>");
}
