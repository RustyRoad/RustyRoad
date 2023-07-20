use std::io::{self, Error};

use crate::database::{
    category::DataTypeCategory,
    databasetype::{DatabaseType, DatabaseTypeTrait, PostgresDatabaseType},
    postgres_types::PostgresTypes,
    types_for_database::TypesForDatabase,
    Database,
};

/// # Name: column_loop_test
/// ## Description:
/// This function is used to test the column loop
/// ## Arguments:
/// * `num_columns` - The number of columns to create
/// * `migration_name` - The name of the migration
/// ## Returns:
/// * `Result<String, Error>` - The up sql contents
/// ## Example
/// ```
/// use rustyroad::database::migrations::column_loop;
/// use rustyline::DefaultEditor;
/// use rustyroad::database::{column_loop_test, PostgresTypes};
///
///
/// let column_loop = column_loop_test(1, String::from("test"));
///
/// match column_loop {
///    Ok(column_loop) => {
///       // Assert the column_loop is equal the generated sql with the column name and column type
///      assert_eq!(column_loop, String::from("CREATE TABLE test (test Integer[] NOT NULL UNIQUE);"));
///   },
///  Err(e) => {
///    panic!("Failed to get column loop: {}", e.to_string());
///   }
/// }
/// ```
pub fn column_loop_test(num_columns: i32, migration_name: String) -> Result<String, Error> {
    let mut column_string = String::new();

    let database = Database::get_database_from_rustyroad_toml().map_err(|_| {
        io::Error::new(
            io::ErrorKind::Other,
            "Failed to get database from rustyroad.toml",
        )
    })?;
    let database_type = database.database_type;
    let _types_for_database = TypesForDatabase::new();

    for _ in 0..num_columns {
        let column_name = "test".to_string();
        let column_type = "1".to_string();
        let column_category = "1".to_string();
        let column_constraints = "UNIQUE".to_string();
        let nullable = "n".to_string();

        let mut all_available_db_types: Vec<DataTypeCategory> =
            DataTypeCategory::get_all_categories();

        all_available_db_types.sort();

        let column_type_input = column_category;
        let column_type_index = column_type_input
            .trim()
            .parse::<usize>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid input for 1"))?;

        let data_type_category = all_available_db_types
            .get(column_type_index - 1)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid input 2"))?;

        let data_types_for_category =
            data_type_category.get_data_types_from_data_type_category(database_type.clone());

        let database_types = match database_type {
            DatabaseType::Postgres => PostgresDatabaseType
                .get_database_types(&data_types_for_category, &data_type_category),
            DatabaseType::Mysql => todo!("Implement MySqlDatabaseType.get_database_types"),
            DatabaseType::Sqlite => todo!("Implement SqliteDatabaseType.get_database_types"),
            DatabaseType::Mongo => todo!("Implement MongoDatabaseType.get_database_types"),
        };

        let mut all_available_db_types: Vec<DataTypeCategory> =
            DataTypeCategory::get_all_categories();

        all_available_db_types.sort();

        let column_type_input = column_type;
        let column_type_index = column_type_input
            .trim()
            .parse::<usize>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid input 3"))?;

        // Get the selected type
        let selected_type = match database_type {
            DatabaseType::Postgres => {
                let selected_database_type =
                    database_types.get(column_type_index - 1).ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidInput, "Invalid input 4")
                    })?;
                let types = selected_database_type
                    .postgres
                    .types
                    .get(&data_type_category.to_string());
                match types {
                    Some(types) => types.get(column_type_index - 1).ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidInput, "Invalid input 5")
                    }),
                    None => Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Invalid input 6",
                    )),
                }
            }
            DatabaseType::Mysql => todo!("Implement MySqlDatabaseType column_type mapping"),
            DatabaseType::Sqlite => todo!("Implement SqliteDatabaseType column_type mapping"),
            DatabaseType::Mongo => todo!("Implement MongoDatabaseType column_type mapping"),
        }?;

        let nullable_input = nullable;
        let nullable = match nullable_input.trim().to_lowercase().as_str() {
            "y" => "NULL",
            "n" => "NOT NULL",
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid input 7",
                ))
            }
        };

        match data_type_category {
            DataTypeCategory::Array => {
                let array_type = "2".to_string();
                let array_type_index = array_type
                    .trim()
                    .parse::<usize>()
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid input 8"))?;

                println!("array_type_index: {}", array_type_index); // Add this line
                println!("length of database_types: {}", database_types.len()); // Add this line
                println!("database_types: {:?}", database_types); // Add this line

                // destructure database_types into postgres, mysql, sqlite, and mongo

                let array_type_matched = match database_type {
                    DatabaseType::Postgres => {
                        let new_value1 = database_types
                            .iter()
                            .map(|x| x.clone().postgres)
                            .collect::<Vec<_>>()
                            .iter()
                            .map(|x| x.types.clone())
                            .collect::<Vec<_>>();

                        let new_value2 = new_value1
                            .iter()
                            .map(|x| x.get(&data_type_category.to_string()));

                        let new_value3 =
                            new_value2.map(|x| x.unwrap().get(array_type_index - 1).unwrap());

                        let new_value4 = new_value3.map(|x| x.clone());

                        new_value4.collect::<Vec<_>>()
                    }
                    DatabaseType::Mysql => {
                        todo!("Implement MySqlDatabaseType column_type mapping")
                    }
                    DatabaseType::Sqlite => {
                        todo!("Implement SqliteDatabaseType column_type mapping")
                    }
                    DatabaseType::Mongo => {
                        todo!("Implement MongoDatabaseType column_type mapping")
                    }
                };

                println!("array_type_matched: {:?}", array_type_matched); // Add this line

                let array_type_matchedoo = match &array_type_matched[0] {
                    PostgresTypes::Array(inner_boxed_postgres_type) => {
                        match **inner_boxed_postgres_type {
                            PostgresTypes::BigInt => PostgresTypes::BigInt,
                            PostgresTypes::Bit => PostgresTypes::Bit,
                            PostgresTypes::Boolean => PostgresTypes::Boolean,
                            PostgresTypes::Box => PostgresTypes::Box,
                            PostgresTypes::SmallInt => PostgresTypes::SmallInt,
                            PostgresTypes::Integer => PostgresTypes::Integer,
                            PostgresTypes::Decimal => PostgresTypes::Decimal,
                            PostgresTypes::Real => PostgresTypes::Real,
                            PostgresTypes::DoublePrecision => PostgresTypes::DoublePrecision,
                            PostgresTypes::Numeric => PostgresTypes::Numeric,
                            PostgresTypes::SmallSerial => PostgresTypes::SmallSerial,
                            PostgresTypes::Serial => PostgresTypes::Serial,
                            PostgresTypes::BigSerial => PostgresTypes::BigSerial,
                            PostgresTypes::Money => PostgresTypes::Money,
                            PostgresTypes::VarChar => PostgresTypes::VarChar,
                            PostgresTypes::CharVarying => PostgresTypes::CharVarying,
                            PostgresTypes::CharacterVarying => PostgresTypes::CharacterVarying,
                            PostgresTypes::Char => PostgresTypes::Char,
                            PostgresTypes::Character => PostgresTypes::Character,
                            PostgresTypes::Text => PostgresTypes::Text,
                            PostgresTypes::ByteA => PostgresTypes::ByteA,
                            PostgresTypes::Timestamp => PostgresTypes::Timestamp,
                            PostgresTypes::TimestampWithoutTimeZone => {
                                PostgresTypes::TimestampWithoutTimeZone
                            }
                            PostgresTypes::TimestampWithTimeZone => {
                                PostgresTypes::TimestampWithTimeZone
                            }
                            PostgresTypes::Date => PostgresTypes::Date,
                            PostgresTypes::Time => PostgresTypes::Time,
                            PostgresTypes::TimeWithoutTimeZone => {
                                PostgresTypes::TimeWithoutTimeZone
                            }
                            PostgresTypes::TimeWithTimeZone => PostgresTypes::TimeWithTimeZone,
                            PostgresTypes::Interval => PostgresTypes::Interval,
                            PostgresTypes::Enum => PostgresTypes::Enum,
                            PostgresTypes::Point => PostgresTypes::Point,
                            PostgresTypes::Line => PostgresTypes::Line,
                            PostgresTypes::Lseg => PostgresTypes::Lseg,
                            PostgresTypes::Path => PostgresTypes::Path,
                            PostgresTypes::PathOpen => PostgresTypes::PathOpen,
                            PostgresTypes::Polygon => PostgresTypes::Polygon,
                            PostgresTypes::Circle => PostgresTypes::Circle,
                            PostgresTypes::Inet => PostgresTypes::Inet,
                            PostgresTypes::Cidr => PostgresTypes::Cidr,
                            PostgresTypes::MacAddr => PostgresTypes::MacAddr,
                            PostgresTypes::MacAddr8 => PostgresTypes::MacAddr8,
                            PostgresTypes::BitVarying => PostgresTypes::BitVarying,
                            PostgresTypes::TsVector => PostgresTypes::TsVector,
                            PostgresTypes::TsQuery => PostgresTypes::TsQuery,
                            PostgresTypes::Xml => PostgresTypes::Xml,
                            PostgresTypes::Json => PostgresTypes::Json,
                            PostgresTypes::JsonB => PostgresTypes::JsonB,
                            PostgresTypes::Uuid => PostgresTypes::Uuid,
                            PostgresTypes::PgLsn => PostgresTypes::PgLsn,
                            PostgresTypes::PgSnapshot => PostgresTypes::PgSnapshot,
                            PostgresTypes::TxidSnapshot => PostgresTypes::TxidSnapshot,
                            PostgresTypes::Int4Range => PostgresTypes::Int4Range,
                            PostgresTypes::Int8Range => PostgresTypes::Int8Range,
                            PostgresTypes::NumRange => PostgresTypes::NumRange,
                            PostgresTypes::TsRange => PostgresTypes::TsRange,
                            PostgresTypes::TstzRange => PostgresTypes::TstzRange,
                            PostgresTypes::DateRange => PostgresTypes::DateRange,
                            PostgresTypes::Array(_) => todo!(),
                        }
                    }
                    _ => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "Invalid input 15",
                        ));
                    }
                };

                column_string.push_str(&format!(
                    "{} {:?}[] {} {}",
                    column_name, array_type_matchedoo, nullable, column_constraints
                ));
            }
            _ => {
                column_string
                    .push_str(&format!("{} {:?} {}", column_name, selected_type, nullable));
            }
        };
    }

    let up_sql_contents = format!("CREATE TABLE {} ({});", migration_name, column_string);

    Ok(up_sql_contents)
}

#[cfg(test)]
mod tests {
    use crate::database::migrations::column_loop;


    #[tokio::test]
    async fn test_column_loop() {
        // Set the number of columns and migration name
        let num_columns = 2;
        let migration_name = "test_migration".to_string();

        // Run the column_loop function
        let result = column_loop(num_columns, migration_name);

        // Check if the result is Ok
        assert!(result.is_ok());
    }
}
