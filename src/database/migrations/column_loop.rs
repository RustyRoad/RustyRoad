use rustyline::DefaultEditor;
use std::io::{self, Error};
use std::ops::Deref;

use crate::database::{
    category::DataTypeCategory,
    databasetype::{DatabaseType, DatabaseTypeTrait, PostgresDatabaseType},
    Database, PostgresTypes,
};

/// # Name: column_loop
/// ## Description
/// This function loops through the number of columns the user wants to add to the table.
/// ## Parameters
/// * `num_columns` - The number of columns the user wants to add to the table.
/// * `migration_name` - The name of the migration.
/// ## Returns
/// A string containing the column definitions for the migration.
/// ## Example
/// ```
/// use rustyroad::database::migrations::column_loop;
/// use rustyline::DefaultEditor;
/// use rustyroad::database::{column_loop_test, PostgresTypes};
///
///
/// let column_loop = column_loop_test(1, String::from("test"));
/// // Assert the column_loop is equal the generated sql with the column name and column type
/// assert_eq!(column_loop.unwrap(), String::from("CREATE TABLE test (test Integer[] NOT NULL UNIQUE);"));
/// ```
/// ## Notes
/// * This function is used in the `create_migration` function in `src\database\migrations\create_migration.rs`.
pub fn column_loop(num_columns: i32, migration_name: String) -> Result<String, Error> {
    // Initialize the rustyline Editor with the default helper and in-memory history
    let mut rl = DefaultEditor::new().unwrap_or_else(|why| {
        panic!("Failed to create rustyline editor: {}", why.to_string());
    });

    let mut column_string = String::new();

    let database = Database::get_database_from_rustyroad_toml()
        .expect("Failed to get database from rustyroad.toml");
    let database_type = database.database_type;
    println!("Database Type: {:?}", database_type);
    // let types_for_database = TypesForDatabase::new();

    for _ in 0..num_columns {
        let column_name = rl.readline("Enter the name of the column: ").unwrap();

        let mut all_available_db_types: Vec<DataTypeCategory> =
            DataTypeCategory::get_all_categories();

        all_available_db_types.sort();
        println!("Column Types: 1");
        for (index, column_type) in all_available_db_types.iter().enumerate() {
            println!("{}. {}", index + 1, column_type);
        }

        let column_type_input = rl
            .readline("Select the category that contains your type. ")
            .unwrap();
        let column_type_index = column_type_input.trim().parse::<usize>().unwrap();
        let data_type_category = all_available_db_types
            .get(column_type_index - 1)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid input"))?;

        println!("You selected 1: {}", data_type_category.clone().to_string());

        // print database type
        println!("Database Type: {:?}", database_type);

        // check if data_type_category matches an array type

        let data_types_for_category =
            data_type_category.clone().get_data_types_from_data_type_category(database_type.clone());

        let database_types = match database_type {
            DatabaseType::Postgres => PostgresDatabaseType
                .get_database_types(&data_types_for_category, &data_type_category),
            DatabaseType::Mysql => todo!("Implement MySqlDatabaseType.get_database_types"),
            DatabaseType::Sqlite => todo!("Implement SqliteDatabaseType.get_database_types"),
            DatabaseType::Mongo => todo!("Implement MongoDatabaseType.get_database_types"),
        };

        let database_types_hash_map = database_types
            .iter()
            .map(|x| x.clone().postgres)
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.types.clone())
            .collect::<Vec<_>>();

        let database_type_impl_iter_option = database_types_hash_map
            .iter()
            .map(|x| x.get(&data_type_category.to_string()));

        let impl_iter_db_types = database_type_impl_iter_option.into_iter();

        let value4 = impl_iter_db_types.map(|x| x.unwrap());

        let types_for_category = value4.collect::<Vec<_>>();

        let all_postgres_types = types_for_category
            .iter()
            .flat_map(|types_vec| types_vec.iter())
            .cloned()
            .collect::<Vec<PostgresTypes>>();

        for (index, x) in all_postgres_types.iter().enumerate() {
            println!("{}. {:?}", index + 1, x);
        }

        // Get the user's choice for the type of the column
        let new_type_input = rl
            .readline("Enter the type of the column: ")
            .unwrap_or_else(|why| {
                panic!("Failed to get user input: {}", why.to_string());
            });

        let new_type_index = new_type_input
            .trim()
            .parse::<usize>()
            .unwrap_or_else(|why| {
                panic!("Failed to parse user input: {}", why.to_string());
            });
        println!("You selected 1: {}", new_type_index);
        // Get the selected type
        let selected_type = match database_type {
            DatabaseType::Postgres => {
                let postgres_types = database_types
                    .iter()
                    .map(|x| x.clone().postgres)
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|x| x.types.clone())
                    .collect::<Vec<_>>();

                let postgres_types_hash_map = postgres_types
                    .iter()
                    .map(|x| x.get(&data_type_category.to_string()));

                let postgres_types_hash_map_iter =
                    postgres_types_hash_map.map(|x| x.unwrap().get(new_type_index - 1).unwrap());

                let postgres_types_hash_map_iter_clone =
                    postgres_types_hash_map_iter.map(|x| x.clone());

                let selected_type_vector = postgres_types_hash_map_iter_clone.collect::<Vec<_>>();
                selected_type_vector
            }
            DatabaseType::Mysql => todo!("Implement MySqlDatabaseType column_type mapping"),
            DatabaseType::Sqlite => todo!("Implement SqliteDatabaseType column_type mapping"),
            DatabaseType::Mongo => todo!("Implement MongoDatabaseType column_type mapping"),
        };

        println!("Type You selected: {:?}", selected_type[0].clone());



        let nullable_input = rl.readline("Is the column nullable? (y/n): ").unwrap();
        let nullable = match nullable_input.trim().to_lowercase().as_str() {
            "y" => "NULL",
            "n" => "NOT NULL",
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input")),
        };

        // ask for column constraints
        println!("Enter column constraints (e.g. PRIMARY KEY, UNIQUE, NOT NULL, DEFAULT, CHECK, REFERENCES): ");
        let mut column_constraints = String::new();
        io::stdin()
            .read_line(&mut column_constraints)
            .expect("Failed to read line");






        match data_type_category {
            DataTypeCategory::Array => {
                let array_type_index = selected_type[0].clone();

                match array_type_index {
                    PostgresTypes::Array(thatthis) => {

                        let array_type = thatthis.deref();


                        column_string.push_str(&format!(
                            "{} {:?}[] {} {},",
                            column_name, array_type, nullable, column_constraints
                        ));
                    }
                    _ => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "Invalid input",
                        ))
                    }
                }
            }
            _ => {
                column_string.push_str(&format!(
                    r#"{} {:?} {} {},"#,
                    column_name, selected_type[0].clone(), nullable, column_constraints
                ));
            }
        }


        println!("Column type selected: {:?}", selected_type);
    }

    let up_sql_contents = format!(
        r#"
    CREATE TABLE {} (
    {}
    );"#,
        migration_name, column_string.trim_end_matches(',')
    );

    Ok(up_sql_contents)
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
        let result = column_loop(num_columns, migration_name);

        // Check if the result is Ok
        assert!(result.is_ok());
    }
}
