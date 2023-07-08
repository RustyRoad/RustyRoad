use rustyline::DefaultEditor;
use std::io::{self, Error};

use crate::database::{Database, types_for_database::TypesForDatabase, category::DataTypeCategory, databasetype::{PostgresDatabaseType, DatabaseType, DatabaseTypeTrait}};

pub async fn column_loop(num_columns: i32, migration_name: String) -> Result<(), Error> {
    // Initialize the rustyline Editor with the default helper and in-memory history
    let mut rl = DefaultEditor::new().unwrap_or_else(|why| {
        panic!("Failed to create rustyline editor: {}", why.to_string());
    });

    let database = Database::get_database_from_rustyroad_toml().await?;
    let database_type = database.database_type;
    let _types_for_database = TypesForDatabase::new();

    for _ in 0..num_columns {
        let column_name = rl.readline("Enter the name of the column: ").unwrap();

        let mut all_available_db_types: Vec<DataTypeCategory> =
            DataTypeCategory::get_all_categories();
        all_available_db_types.sort();
        println!("Column Types:");
        for (index, column_type) in all_available_db_types.iter().enumerate() {
            println!("{}. {}", index + 1, column_type);
        }

        let column_type_input = rl.readline("Enter the type of the column: ").unwrap();
        let column_type_index = column_type_input.trim().parse::<usize>().unwrap();
        let data_type_category = all_available_db_types
            .get(column_type_index - 1)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid input"))?;

        println!("You selected: {}", data_type_category);

        let data_types_for_category =
            data_type_category.get_data_types_from_data_type_category(database_type.clone());
        let database_types = match database_type {
            DatabaseType::Postgres => PostgresDatabaseType
                .get_database_types(&data_types_for_category, &data_type_category),
            DatabaseType::Mysql => todo!("Implement MySqlDatabaseType.get_database_types"),
            DatabaseType::Sqlite => todo!("Implement SqliteDatabaseType.get_database_types"),
            DatabaseType::Mongo => todo!("Implement MongoDatabaseType.get_database_types"),
        };

        println!("Available types for the selected category:");
        for (index, database_type) in database_types.iter().enumerate() {
            println!("{:#?}. {:#?}", index + 1, database_type);
        }

        let new_type_input = rl.readline("Enter the type of the column: ").unwrap();
        let new_type_index = new_type_input.trim().parse::<usize>().unwrap();
        let new_type = database_types
            .get(new_type_index - 1)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid input"))?;

        println!("You selected: {:?}", new_type);

        let _column_constraints = rl
            .readline("Enter the constraints of the column: ")
            .unwrap();

        let nullable_input = rl.readline("Is the column nullable? (y/n): ").unwrap();
        let nullable = match nullable_input.trim().to_lowercase().as_str() {
            "y" => true,
            "n" => false,
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input")),
        };

        let column_type = match database_type {
            DatabaseType::Postgres => &new_type.postgres.types,
            DatabaseType::Mysql => todo!("Implement MySqlDatabaseType column_type mapping"),
            DatabaseType::Sqlite => todo!("Implement SqliteDatabaseType column_type mapping"),
            DatabaseType::Mongo => todo!("Implement MongoDatabaseType column_type mapping"),
        };

        println!("You selected: {:?}", column_type);

        let up_sql_contents = match nullable {
            true => format!(
                "CREATE TABLE IF NOT EXISTS {} ({} {:?} NULL);",
                migration_name, column_name, column_type
            ),
            false => format!(
                "CREATE TABLE IF NOT EXISTS {} ({} {:?} NOT NULL);",
                migration_name, column_name, column_type
            ),
        };

        println!("{}", up_sql_contents);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_column_loop() {
        // Set the number of columns and migration name
        let num_columns = 2;
        let migration_name = "test_migration".to_string();

        // Run the column_loop function
        let result = column_loop(num_columns, migration_name).await;

        // Check if the result is Ok
        assert!(result.is_ok());
    }
}
