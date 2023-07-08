#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_types_for_database() {
        let mut types_for_db = TypesForDatabase::new();

        // Add a PostgreSQL type
        types_for_db.add_postgres_type("Text".to_string(), vec![PostgresTypes::Text]);
        assert_eq!(
            types_for_db.postgres.types.get("Text"),
            Some(&vec![PostgresTypes::Text])
        );

        // Add a MySQL type
        let types = types_for_db
            .add_mysql_type("Numeric".to_string(), vec![MySqlTypes::Decimal(10, 2)])
            .unwrap();
        assert_eq!(types, &mut vec![MySqlTypes::Decimal(10, 2)]);
        assert_eq!(
            types_for_db.mysql.types.get("Numeric"),
            Some(&vec![MySqlTypes::Decimal(10, 2)])
        );

        // Add a SQLite type
        types_for_db.add_sqlite_type("Text".to_string(), vec![SqliteTypes::Text]);
        assert_eq!(
            types_for_db.sqlite.types.get("Text"),
            Some(&vec![SqliteTypes::Text])
        );
    }
}