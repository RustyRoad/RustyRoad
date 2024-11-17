#[cfg(test)]
mod tests {
    use rustyroad::database::{DataTypeCategory, DatabaseType, PostgresTypes};

    #[tokio::test]
    async fn test_get_data_from_data_type_category_postgres_numeric() {
        let data_type_category = DataTypeCategory::Numeric;
        let database_type = DatabaseType::Postgres;
        let types_for_database =
            data_type_category.get_data_types_from_data_type_category(database_type);
        assert_eq!(types_for_database.postgres.types.len(), 9);
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Numeric.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Decimal.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Money.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::SmallInt.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Integer.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::BigInt.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Real.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::DoublePrecision.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::SmallSerial.to_string()));
    }

    #[tokio::test]
    async fn test_get_data_from_data_type_category_postgres_datetime() {
        let data_type_category = DataTypeCategory::DateTime;
        let database_type = DatabaseType::Postgres;
        let types_for_database =
            data_type_category.get_data_types_from_data_type_category(database_type);
        assert_eq!(types_for_database.postgres.types.len(), 5);
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Date.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Time.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Timestamp.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::TimestampWithTimeZone.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Interval.to_string()));
    }

    #[tokio::test]
    async fn test_get_data_from_data_type_category_postgres_text() {
        let data_type_category = DataTypeCategory::Text;
        let database_type = DatabaseType::Postgres;
        let types_for_database =
            data_type_category.get_data_types_from_data_type_category(database_type);
        assert_eq!(types_for_database.postgres.types.len(), 5);
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Char.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::VarChar.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Text.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::ByteA.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Xml.to_string()));
    }

    #[tokio::test]
    async fn test_get_data_from_data_type_category_postgres_geometric() {
        let data_type_category = DataTypeCategory::Geometric;
        let database_type = DatabaseType::Postgres;
        let types_for_database =
            data_type_category.get_data_types_from_data_type_category(database_type);
        assert_eq!(types_for_database.postgres.types.len(), 7);
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Point.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Line.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Lseg.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Box.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Path.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Polygon.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Circle.to_string()));
    }

    #[tokio::test]
    async fn test_get_data_from_data_type_category_postgres_network_address() {
        let data_type_category = DataTypeCategory::NetworkAddress;
        let database_type = DatabaseType::Postgres;
        let types_for_database =
            data_type_category.get_data_types_from_data_type_category(database_type);
        assert_eq!(types_for_database.postgres.types.len(), 2);
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Cidr.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Inet.to_string()));
    }

    #[tokio::test]
    async fn test_get_data_from_data_type_category_postgres_json() {
        let data_type_category = DataTypeCategory::Json;
        let database_type = DatabaseType::Postgres;
        let types_for_database =
            data_type_category.get_data_types_from_data_type_category(database_type);
        assert_eq!(types_for_database.postgres.types.len(), 2);
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Json.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::JsonB.to_string()));
    }

    #[tokio::test]
    async fn test_get_data_from_data_type_category_postgres_search() {
        let data_type_category = DataTypeCategory::Search;
        let database_type = DatabaseType::Postgres;
        let types_for_database =
            data_type_category.get_data_types_from_data_type_category(database_type);
        assert_eq!(types_for_database.postgres.types.len(), 1);
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::TsQuery.to_string()));
    }

    #[tokio::test]
    async fn test_get_data_from_data_type_category_postgres_uuid() {
        let data_type_category = DataTypeCategory::UUID;
        let database_type = DatabaseType::Postgres;
        let types_for_database =
            data_type_category.get_data_types_from_data_type_category(database_type);
        assert_eq!(types_for_database.postgres.types.len(), 1);
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Uuid.to_string()));
    }

    #[tokio::test]
    async fn test_get_data_from_data_type_category_postgres_bit_string() {
        let data_type_category = DataTypeCategory::BitString;
        let database_type = DatabaseType::Postgres;
        let types_for_database =
            data_type_category.get_data_types_from_data_type_category(database_type);
        assert_eq!(types_for_database.postgres.types.len(), 2);
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Bit.to_string()));
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::BitVarying.to_string()));
    }

    #[tokio::test]
    async fn test_get_data_from_data_type_category_postgres_composite() {
        let data_type_category = DataTypeCategory::Composite;
        let database_type = DatabaseType::Postgres;
        let types_for_database =
            data_type_category.get_data_types_from_data_type_category(database_type);
        assert_eq!(types_for_database.postgres.types.len(), 1);
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Character.to_string()));
    }

    /// PostGres Interval
    /// - Passes
    #[tokio::test]
    async fn test_get_data_from_data_type_category_postgres_interval() {
        let data_type_category = DataTypeCategory::Interval;
        let database_type = DatabaseType::Postgres;
        let types_for_database =
            data_type_category.get_data_types_from_data_type_category(database_type);
        assert_eq!(types_for_database.postgres.types.len(), 1);
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Interval.to_string()));
    }

    /// PostGres Money
    /// - Passes
    #[tokio::test]
    async fn test_get_data_from_data_type_category_postgres_money() {
        let data_type_category = DataTypeCategory::Money;
        let database_type = DatabaseType::Postgres;
        let types_for_database =
            data_type_category.get_data_types_from_data_type_category(database_type);
        assert_eq!(types_for_database.postgres.types.len(), 1);
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Money.to_string()));
    }

    #[tokio::test]
    async fn test_get_data_from_data_type_category_postgres_other() {
        let data_type_category = DataTypeCategory::Other;
        let database_type = DatabaseType::Postgres;
        let types_for_database =
            data_type_category.get_data_types_from_data_type_category(database_type);
        assert_eq!(types_for_database.postgres.types.len(), 1);
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Xml.to_string()));
    }

    #[tokio::test]
    async fn test_get_data_from_data_type_category_postgres_array() {
        let data_type_category = DataTypeCategory::Array;
        let database_type = DatabaseType::Postgres;
        let types_for_database =
            data_type_category.get_data_types_from_data_type_category(database_type);

        assert_eq!(types_for_database.postgres.types.len(), 1);
        // print the types
        for (key, value) in types_for_database.postgres.types.iter() {
            println!("{:?}: {:?}", key, value);
        }
        assert!(types_for_database
            .postgres
            .types
            .contains_key(&PostgresTypes::Array(Box::new(PostgresTypes::Integer)).to_string()));
    }

    #[tokio::test]
    async fn fn_test_get_data_from_data_type_category_mysql_bit_string() {
        let data_type_category = DataTypeCategory::BitString;
        let database_type = DatabaseType::Mysql;
        let types_for_database =
            data_type_category.get_data_types_from_data_type_category(database_type);
        assert_eq!(types_for_database.mysql.types.len(), 0);
    }
}
