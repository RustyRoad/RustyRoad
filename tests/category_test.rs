use rustyroad::database::{DataTypeCategory, DatabaseType, PostgresTypes};

#[test]
fn every_postgres_category_is_stored_under_its_category_name() {
    for category in DataTypeCategory::get_all_categories() {
        let types = category.get_data_types_from_data_type_category(DatabaseType::Postgres);
        let mapped = types
            .postgres
            .types
            .get(&category.to_string())
            .unwrap_or_else(|| panic!("missing PostgreSQL mapping for {category}"));

        assert!(
            !mapped.is_empty(),
            "empty PostgreSQL mapping for {category}"
        );
        assert_eq!(types.postgres.types.len(), 1);
    }
}

#[test]
fn representative_postgres_categories_contain_their_expected_types() {
    let cases = [
        (DataTypeCategory::Numeric, PostgresTypes::Numeric),
        (DataTypeCategory::DateTime, PostgresTypes::Timestamp),
        (DataTypeCategory::Text, PostgresTypes::Text),
        (DataTypeCategory::Boolean, PostgresTypes::Boolean),
        (DataTypeCategory::Geometric, PostgresTypes::Point),
        (DataTypeCategory::NetworkAddress, PostgresTypes::Inet),
        (DataTypeCategory::Json, PostgresTypes::JsonB),
        (DataTypeCategory::Search, PostgresTypes::TsVector),
        (DataTypeCategory::UUID, PostgresTypes::Uuid),
        (DataTypeCategory::BitString, PostgresTypes::Bit),
        (DataTypeCategory::Composite, PostgresTypes::TsQuery),
        (DataTypeCategory::Range, PostgresTypes::Int4Range),
        (DataTypeCategory::Other, PostgresTypes::Uuid),
    ];

    for (category, expected) in cases {
        let types = category.get_data_types_from_data_type_category(DatabaseType::Postgres);
        assert!(
            types.postgres.types[&category.to_string()].contains(&expected),
            "{category} did not include {expected:?}"
        );
    }
}
