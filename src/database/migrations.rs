use super::database::Database;
use crate::database::DataTypeCategory;
use crate::database::*;
use crate::generators::create_file;
use crate::writers::write_to_file;
use crate::Project;
use chrono::prelude::*;
use rustyline::DefaultEditor;
use sqlx::{Acquire, Connection, Executor, MySql, Postgres, postgres::PgPoolOptions, query, Transaction};
use std::error::{Error as StdError, Error};
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::{create_dir_all, DirEntry};
use std::fs::{self};
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::path::Path;
use std::pin::Pin;
use std::println;
use std::sync::Arc;

const CONSTRAINTS: &[&str] = &["PRIMARY KEY", "NOT NULL", "FOREIGN KEY"];

/// ## Name: create_migration
/// ### Description: Creates a migration file
/// #### Parameters:
/// - name: [`&str`] - the name of the migration
/// - Returns: [`Result<(), std::io::Error>`]
/// - if the migration was created successfully: [`Ok(())`]
/// - if there was an error creating the migration: [`Err(std::io::Error)`]
///
/// ### Example:
/// ```rust
/// use rustyroad::database::migrations::create_migration;
///
/// create_migration("create_users_table").unwrap();
/// ```
pub fn create_migration(name: &str) -> Result<(), io::Error> {
        let name = name.to_owned();


        let path = std::env::current_dir().unwrap();

        if std::fs::read_to_string(path.join("rustyroad.toml")).is_err() {
            println!("This is why you can't have nice things");
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "This is why you can't have nice things",
            ));
        }

        match std::fs::create_dir("config/database") {
            Ok(_) => {}
            Err(_) => {}
        }

        // Check for migrations folder
        // If it doesn't exist, create it
        match std::fs::create_dir("config/database/migrations") {
            Ok(_) => {}
            Err(_) => {}
        }

        // Create directory with timestamp and name of migration
        // Then create up and down files

        let folder_name = format!(
            "config/database/migrations/{}-{}",
            Local::now().format("%Y%m%d%H%M%S"),
            name
        );

        match std::fs::create_dir(&folder_name) {
            Ok(_) => {}
            Err(_) => {
                println!("Migration already exists");
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Migration already exists",
                ));
            }
        }

        create_file(&format!("{}/up.sql", folder_name).to_string())
            .unwrap_or_else(|why| panic!("Couldn't create {}: {}", &name, why.to_string()));

        let up_file = format!("{}/up.sql", folder_name).to_string();

        let down_file = format!("{}/down.sql", folder_name).to_string();

        // Create the down.sql file
        create_file(&format!("{}/down.sql", folder_name).to_string())
            .unwrap_or_else(|why| panic!("Couldn't create {}: {}", &name, why.to_string()));

        // Initialize the rustyline Editor with the default helper and in-memory history
        let mut rl = DefaultEditor::new().unwrap_or_else(|why| {
            panic!("Failed to create rustyline editor: {}", why.to_string());
        });

        // // Prompt the user for SQL queries for the up.sql file
        let mut up_sql_contents = String::new();
        let mut down_sql_contents = String::new();

        let table_name = name.to_string();

        // ask the user how many columns they want to add

        let num_columns = rl
            .readline("Enter the number of columns: ")
            .unwrap_or_else(|why| {
                panic!("Failed to read number of columns: {}", why.to_string());
            });

        // loop through the number of columns and ask the user for the column name, type, and constraints

        for _ in 0..num_columns.parse::<i32>().unwrap() {
            let column_name = rl
                .readline("Enter the name of the column: ")
                .unwrap_or_else(|why| {
                    panic!("Failed to read column name: {}", why.to_string());
                });

            let database: Database = Database::get_database_from_rustyroad_toml()?;

            let database_type: DatabaseType = database.clone().database_type;

            // initialize data types
            let mut all_available_db_types_for_postgres: Vec<DataTypeCategory> =
                DataTypeCategory::get_all_categories();
            // match the available column types to the database type
            // need to get the values from the enum of the database type
            all_available_db_types_for_postgres.sort();
            println!("Column Types: ");

            for (index, column_type) in all_available_db_types_for_postgres
                .clone()
                .into_iter()
                .enumerate()
            {
                println!("{}. {}", index + 1, column_type);
            }

            // prompt the user for the column type
            let column_type = rl
                // Ask the user for the column type and give them a list of available types
                .readline("Enter the type of the column: ")
                .unwrap_or_else(|why| {
                    panic!("Failed to read column type: {}", why);
                });

            // match the column type to the data type
            let data_type_category = match column_type.trim() {
                "1" => DataTypeCategory::Array,
                "2" => DataTypeCategory::BitString,
                "3" => DataTypeCategory::Boolean,
                "4" => DataTypeCategory::Composite,
                "5" => DataTypeCategory::DateTime,
                "6" => DataTypeCategory::Geometric,
                "7" => DataTypeCategory::Interval,
                "8" => DataTypeCategory::Json,
                "9" => DataTypeCategory::Money,
                "10" => DataTypeCategory::NetworkAddress,
                "11" => DataTypeCategory::Numeric,
                "12" => DataTypeCategory::Other,
                "13" => DataTypeCategory::Range,
                "14" => DataTypeCategory::Search,
                "15" => DataTypeCategory::Text,
                "16" => DataTypeCategory::UUID,
                _ => {
                    println!("Invalid input. Please enter a number between 1 and 16.");
                    continue; // Ask again if input is invalid
                }
            };

            //print what the user just selected
            println!("You selected: {}", data_type_category);

            // # Name: data_types_for_category
            // ### Description: Holds the types for the database and the selcted category
            let data_types_for_category =
                data_type_category.get_data_types_from_data_type_category(database_type);

            let database_types = TypesForDatabase::get_postgres_types(&data_types_for_category, &data_type_category);

            // Print the available types for the selected category
            println!("Available types for the selected category: ");
            for (index, database_type) in database_types.clone().into_iter().enumerate() {
                println!("{:#?}. {:#?}", index + 1, database_type);
            }

            // prompt the user for the column type
            let new_type = rl
                // Ask the user for the column type and give them a list of available types
                .readline("Enter the type of the column: ")
                .unwrap_or_else(|why| panic!("Failed to read column type: {}", why.to_string()));
            //bug is here, this won't work because the category is dynamic .

            // the item we are matching agaains is a subcategory not a main.
            // Ie. VarChar is a subcategory of text.
            // VarChar is the Postgres type, text is the category for rustyroad.

            // we need to match the number the user selectes against the database type
            let column_type = match new_type.trim() {
                "1" => database_types[0].clone(),
                "2" => database_types[1].clone(),
                "3" => database_types[2].clone(),
                "4" => database_types[3].clone(),
                "5" => database_types[4].clone(),
                "6" => database_types[5].clone(),
                "7" => database_types[6].clone(),
                "8" => database_types[7].clone(),
                "9" => database_types[8].clone(),
                "10" => database_types[9].clone(),
                "11" => database_types[10].clone(),
                "12" => database_types[11].clone(),
                "13" => database_types[12].clone(),
                "14" => database_types[13].clone(),
                "15" => database_types[14].clone(),
                "16" => database_types[15].clone(),
                _ => {
                    println!("Invalid input. Please enter a number between 1 and 16.");
                    continue; // Ask again if input is invalid
                }
            };

            // print what the user just selected
            println!("You selected: {}", column_type);

            // prompt the user for the column constraints
            let column_constraints = rl
                .readline("Enter the constraints of the column: ")
                .unwrap_or_else(|why| {
                    panic!("Failed to read column constraints: {}", why.to_string());
                });

            // Ask if the column is nullable
            let nullable_input = rl
                .readline("Is the column nullable? (y/n): ")
                .unwrap_or_else(|why| {
                    panic!("Failed to read nullable: {}", why.to_string());
                });

            // Convert input to bool
            let nullable = match nullable_input.trim().to_lowercase().as_str() {
                "y" => true,
                "n" => false,
                _ => {
                    println!("Invalid input. Please enter 'y' for yes or 'n' for no.");
                    continue; // Ask again if input is invalid
                }
            };

            // validate the column type to sql type
            match column_type {
                postgres_types::PostgresTypes::VarChar => {
                    // ask the user for the length of the string
                    let string_length = rl
                        .readline("Enter the length of the string: ")
                        .unwrap_or_else(|why| {
                            panic!("Failed to read string length: {}", why.to_string());
                        });
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} VARCHAR({}) NULL {};",
                                table_name, column_name, string_length, column_constraints
                            ));

                            // mapp contraints to sql

                            up_sql_contents.push('\n');
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} VARCHAR({}) NOT NULL {};",
                                table_name, column_name, string_length, column_constraints
                            ));
                            up_sql_contents.push('\n');

                        }
                    }

                    continue;
                }

                PostgresTypes::SmallInt => {
                  match nullable {
                    true => {
                        // add the sql and constraints to the up.sql file
                        // if the column is nullable
                        up_sql_contents.push_str(&format!(
                            "ALTER TABLE {} ADD COLUMN {} SMALLINT NULL {};",
                            table_name, column_name, column_constraints
                        ));

                        // mapp contraints to sql

                        up_sql_contents.push('\n');
                    }
                    false => {
                        // add the column to the up.sql file
                        up_sql_contents.push_str(&format!(
                            "ALTER TABLE {} ADD COLUMN {} SMALLINT NOT NULL {};",
                            table_name, column_name, column_constraints
                        ));
                        up_sql_contents.push('\n');
                    }
                  }
                }
                PostgresTypes::Integer => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} INTEGER NULL {};",
                                table_name, column_name, column_constraints
                            ));

                            // mapp contraints to sql

                            up_sql_contents.push('\n');
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} INTEGER NOT NULL {};",
                                table_name, column_name, column_constraints
                            ));
                            up_sql_contents.push('\n');
                        }
                    }
                }
                PostgresTypes::BigInt => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} BIGINT NULL {};",
                                table_name, column_name, column_constraints
                            ));

                            // mapp contraints to sql

                            up_sql_contents.push('\n');
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} BIGINT NOT NULL {};",
                                table_name, column_name, column_constraints
                            ));
                            up_sql_contents.push('\n');
                        }
                    }
                }
                PostgresTypes::Decimal => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} DECIMAL NULL {};",
                                table_name, column_name, column_constraints
                            ));

                            // mapp contraints to sql

                            up_sql_contents.push('\n');
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} DECIMAL NOT NULL {};",
                                table_name, column_name, column_constraints
                            ));
                            up_sql_contents.push('\n');
                        }
                    }
                }
                PostgresTypes::Real => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} REAL NULL {};",
                                table_name, column_name, column_constraints
                            ));

                            // mapp contraints to sql

                            up_sql_contents.push('\n');
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} REAL NOT NULL {};",
                                table_name, column_name, column_constraints
                            ));
                            up_sql_contents.push('\n');
                        }
                    }
                }
                PostgresTypes::DoublePrecision => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} DOUBLE PRECISION NULL {};",
                                table_name, column_name, column_constraints
                            ));

                            // mapp contraints to sql

                            up_sql_contents.push('\n');
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} DOUBLE PRECISION NOT NULL {};",
                                table_name, column_name, column_constraints
                            ));
                            up_sql_contents.push('\n');
                        }
                    }
                }
                PostgresTypes::Numeric => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} NUMERIC NULL {};",
                                table_name, column_name, column_constraints
                            ));

                            // mapp contraints to sql

                            up_sql_contents.push('\n');
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} NUMERIC NOT NULL {};",
                                table_name, column_name, column_constraints
                            ));
                            up_sql_contents.push('\n');
                        }
                    }
                }
                PostgresTypes::SmallSerial => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} SMALLSERIAL NULL {};",
                                table_name, column_name, column_constraints
                            ));

                            // mapp contraints to sql

                            up_sql_contents.push('\n');
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} SMALLSERIAL NOT NULL {};",
                                table_name, column_name, column_constraints
                            ));
                            up_sql_contents.push('\n');
                        }
                    }
                }
                PostgresTypes::Serial => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} SERIAL NULL {};",
                                table_name, column_name, column_constraints
                            ));

                            // mapp contraints to sql

                            up_sql_contents.push('\n');
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} SERIAL NOT NULL {};",
                                table_name, column_name, column_constraints
                            ));
                            up_sql_contents.push('\n');
                        }
                    }
                }
                PostgresTypes::BigSerial => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} BIGSERIAL NULL {};",
                                table_name, column_name, column_constraints
                            ));

                            // mapp contraints to sql

                            up_sql_contents.push('\n');
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} BIGSERIAL NOT NULL {};",
                                table_name, column_name, column_constraints
                            ));
                            up_sql_contents.push('\n');
                        }
                    }
                }
                PostgresTypes::Money => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} MONEY NULL {};",
                                table_name, column_name, column_constraints
                            ));

                            // mapp contraints to sql

                            up_sql_contents.push('\n');
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} MONEY NOT NULL {};",
                                table_name, column_name, column_constraints
                            ));
                            up_sql_contents.push('\n');
                        }
                    }
                }
                PostgresTypes::CharVarying => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} VARCHAR NULL {};",
                                table_name, column_name, column_constraints
                            ));

                            // mapp contraints to sql

                            up_sql_contents.push('\n');
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} VARCHAR NOT NULL {};",
                                table_name, column_name, column_constraints
                            ));
                            up_sql_contents.push('\n');
                        }
                    }
                }
                PostgresTypes::CharacterVarying => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} VARCHAR NULL {};",
                                table_name, column_name, column_constraints
                            ));

                            // mapp contraints to sql

                            up_sql_contents.push('\n');
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} VARCHAR NOT NULL {};",
                                table_name, column_name, column_constraints
                            ));
                            up_sql_contents.push('\n');
                        }
                    }
                }
                PostgresTypes::Char => {
                    // add the column to the up.sql file
                    up_sql_contents.push_str(&format!(
                        "ALTER TABLE {} ADD COLUMN {} CHAR NOT NULL {};",
                        table_name, column_name, column_constraints
                    ));
                    up_sql_contents.push('\n');
                }
                PostgresTypes::Character => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} CHAR NULL {};",
                                table_name, column_name, column_constraints
                            ));

                            // mapp contraints to sql

                            up_sql_contents.push('\n');
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} CHAR NOT NULL {};",
                                table_name, column_name, column_constraints
                            ));
                            up_sql_contents.push('\n');
                        }
                    }
                }
                PostgresTypes::Text => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} TEXT NULL {};",
                                table_name, column_name, column_constraints
                            ));

                            // mapp contraints to sql

                            up_sql_contents.push('\n');
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} TEXT NOT NULL {};",
                                table_name, column_name, column_constraints
                            ));
                            up_sql_contents.push('\n');
                        }
                    }
                }
                PostgresTypes::ByteA => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} BYTEA NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} BYTEA NOT NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::Timestamp => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} TIMESTAMP NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} TIMESTAMP NOT NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::TimestampWithoutTimeZone => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} TIMESTAMP NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} TIMESTAMP NOT NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::TimestampWithTimeZone => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} TIMESTAMP NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} TIMESTAMP NOT NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::Date => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} DATE NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} DATE NOT NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::Time => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} TIME NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} TIME NOT NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::TimeWithoutTimeZone => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} TIME NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} TIME NOT NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::TimeWithTimeZone => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} TIME NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} TIME NOT NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::Interval => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} INTERVAL NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} INTERVAL NOT NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::Boolean => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} BOOLEAN NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} BOOLEAN NOT NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::Enum => {
                  match nullable {
                    true => {
                        // add the sql and constraints to the up.sql file
                        // if the column is nullable
                        up_sql_contents.push_str(
                            &format!(
                                "\
                                ALTER TABLE {} ADD COLUMN {} {} NULL {};
                                ",  &table_name, column_name, column_type, column_constraints
                            )
                        );
                    }
                    false => {
                        // add the column to the up.sql file
                        up_sql_contents.push_str(
                            &format!(
                                "\
                                ALTER TABLE {} ADD COLUMN {} {} NOT NULL {};
                                ",  &table_name, column_name, column_type, column_constraints
                            )
                        );
                    }
                  }
                }
                PostgresTypes::Point => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} POINT NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} POINT NOT NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::Line => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} LINE NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} LINE NOT NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::Lseg => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} LSEG NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} LSEG NOT NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::Box => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} BOX NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} BOX NOT NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::Path => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} PATH NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} PATH NOT NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::PathOpen => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} PATH OPEN NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} PATH OPEN NOT NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::Polygon => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} POLYGON NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} POLYGON NOT NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::Circle => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "\
                                    ALTER TABLE {} ADD COLUMN {} CIRCLE NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!("\
                                    ALTER TABLE {} ADD COLUMN {} CIRCLE NOT NULL {};
                                    ",  &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::Inet => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} INET NULL {};",
                                &table_name, column_name, column_constraints
                            ));
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} INET NOT NULL {};",
                                &table_name, column_name, column_constraints
                            ));
                        }
                    }
                }
                PostgresTypes::Cidr => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} CIDR NULL {};",
                                &table_name, column_name, column_constraints
                            ));
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} CIDR NOT NULL {};",
                                &table_name, column_name, column_constraints
                            ));
                        }
                    }
                }
                PostgresTypes::MacAddr => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} MACADDR NULL {};",
                                &table_name, column_name, column_constraints
                            ));
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} MACADDR NOT NULL {};",
                                &table_name, column_name, column_constraints
                            ));
                        }
                    }
                }
                PostgresTypes::MacAddr8 => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} MACADDR8 NULL {};",
                                &table_name, column_name, column_constraints
                            ));
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} MACADDR8 NOT NULL {};",
                                &table_name, column_name, column_constraints
                            ));
                        }
                    }
                }
                PostgresTypes::Bit => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} BIT NULL {};",
                                &table_name, column_name, column_constraints
                            ));
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} BIT NOT NULL {};",
                                &table_name, column_name, column_constraints
                            ));
                        }
                    }
                }
                PostgresTypes::BitVarying => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} BIT VARYING NULL {};",
                                &table_name, column_name, column_constraints
                            ));
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} BIT VARYING NOT NULL {};",
                                &table_name, column_name, column_constraints
                            ));
                        }
                    }
                }
                PostgresTypes::TsVector => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} TSVECTOR NULL {};",
                                &table_name, column_name, column_constraints
                            ));
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!(
                                "ALTER TABLE {} ADD COLUMN {} TSVECTOR NOT NULL {};",
                                &table_name, column_name, column_constraints
                            ));
                        }
                    }
                }
                PostgresTypes::TsQuery => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!("ALTER TABLE {} ADD COLUMN {} TSQUERY NULL {};", &table_name, column_name, column_constraints));
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!("ALTER TABLE {} ADD COLUMN {} TSQUERY NOT NULL {};", &table_name, column_name, column_constraints));
                        }
                    }
                }
                PostgresTypes::Xml => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "ALTER TABLE {} ADD COLUMN {} XML NULL {};",
                                    &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "ALTER TABLE {} ADD COLUMN {} XML NOT NULL {};",
                                    &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::Json => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "ALTER TABLE {} ADD COLUMN {} JSON NULL {};",
                                    &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "ALTER TABLE {} ADD COLUMN {} JSON NOT NULL {};",
                                    &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::JsonB => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "ALTER TABLE {} ADD COLUMN {} JSONB NULL {};",
                                    &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "ALTER TABLE {} ADD COLUMN {} JSONB NOT NULL {};",
                                    &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::Uuid => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "ALTER TABLE {} ADD COLUMN {} UUID NULL {};",
                                    &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "ALTER TABLE {} ADD COLUMN {} UUID NOT NULL {};",
                                    &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::PgLsn => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!(
                                    "ALTER TABLE {} ADD COLUMN {} PGLSN NULL {};",
                                    &table_name, column_name, column_constraints
                                )
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!(
                                    "ALTER TABLE {} ADD COLUMN {} PGLSN NOT NULL {};",
                                    &table_name, column_name, column_constraints
                                )
                            );
                        }
                    }
                }
                PostgresTypes::PgSnapshot => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!("ALTER TABLE {} ADD COLUMN {} PG_SNAPSHOT NULL {};", &table_name, column_name, column_constraints)
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!("ALTER TABLE {} ADD COLUMN {} PG_SNAPSHOT NOT NULL {};", &table_name, column_name, column_constraints)
                            );
                        }
                    }
                }
                PostgresTypes::TxidSnapshot => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(
                                &format!("ALTER TABLE {} ADD COLUMN {} TXID_SNAPSHOT NULL {};", &table_name, column_name, column_constraints)
                            );
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(
                                &format!("ALTER TABLE {} ADD COLUMN {} TXID_SNAPSHOT NOT NULL {};", &table_name, column_name, column_constraints)
                            );
                        }
                    }
                }
                PostgresTypes::Int4Range => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!("ALTER TABLE {} ADD COLUMN {} INT4RANGE NULL {};", &table_name, column_name, column_constraints));
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!("ALTER TABLE {} ADD COLUMN {} INT4RANGE NOT NULL {};", &table_name, column_name, column_constraints));
                        }
                    }
                }
                PostgresTypes::Int8Range => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!("ALTER TABLE {} ADD COLUMN {} INT8RANGE NULL {};", &table_name, column_name, column_constraints));
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!("ALTER TABLE {} ADD COLUMN {} INT8RANGE NOT NULL {};", &table_name, column_name, column_constraints));
                        }
                    }
                }
                PostgresTypes::NumRange => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!("ALTER TABLE {} ADD COLUMN {} NUMRANGE NULL {};", &table_name, column_name, column_constraints));
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!("ALTER TABLE {} ADD COLUMN {} NUMRANGE NOT NULL {};", &table_name, column_name, column_constraints));
                        }
                    }
                }

                PostgresTypes::TsRange => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!("ALTER TABLE {} ADD COLUMN {} TSRANGE NULL {};", &table_name, column_name, column_constraints));
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!("ALTER TABLE {} ADD COLUMN {} TSRANGE NOT NULL {};", &table_name, column_name, column_constraints));
                        }
                    }
                }
                PostgresTypes::TstzRange => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!("ALTER TABLE {} ADD COLUMN {} TSTZRANGE NULL {};", &table_name, column_name, column_constraints));
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!("ALTER TABLE {} ADD COLUMN {} TSTZRANGE NOT NULL {};", &table_name, column_name, column_constraints));
                        }
                    }
                }
                PostgresTypes::DateRange => {
                    match nullable {
                        true => {
                            // add the sql and constraints to the up.sql file
                            // if the column is nullable
                            up_sql_contents.push_str(&format!("ALTER TABLE {} ADD COLUMN {} DATERANGE NULL {};", &table_name, column_name, column_constraints));
                        }
                        false => {
                            // add the column to the up.sql file
                            up_sql_contents.push_str(&format!("ALTER TABLE {} ADD COLUMN {} DATERANGE NOT NULL {};", &table_name, column_name, column_constraints));
                        }
                    }
                }
                PostgresTypes::Array(_) => {
                   up_sql_contents = create_array(up_sql_contents.clone(), nullable).unwrap_or_else(
                    |err| { panic!("Error creating array: {}", err) });
                },

            }
        }

        // WRITE UP.SQL
        write_to_file(&up_file, &up_sql_contents.into_bytes()).unwrap();

        // Add Down.sql Statements to drop the table and the columns
        // Add the drop table statement to the down.sql file
         down_sql_contents.push_str(&format!("DROP TABLE {};", table_name));
        write_to_file(&down_file, &down_sql_contents.into_bytes()).unwrap();


        Ok(())
}


/// ## Name: initialize_migration
/// ### Description: Creates the initial migration directory and the up.sql and down.sql files for the initial migration
/// ### Arguments:
/// * [`&project`] - The project struct that contains the project name and the project path
///
/// ### Returns:
/// * [`Result<(), CustomMigrationError>`] - Returns a result with a unit type or a CustomMigrationError
/// ### Example:
/// ```rust
/// use rustyroad::database::migrations::initial_migration;
///
/// let project = Project {
///    name: "test".to_string(),
///   path: "/home/user/test".to_string(),
///   // .. rest of the struct
/// };
/// let result = initialize_migration(&project);
///
/// assert!(result.is_ok());
/// ```
pub fn initialize_migration(project: &Project) -> Result<(), ErrorKind> {
    // create the migrations directory
    let sql = "
       CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
  );";
    let migrations_dir = Path::new(&project.initial_migration_directory).join("migrations");

    if !migrations_dir.exists() {
        create_dir_all(&migrations_dir).unwrap_or_else(|why| {
            panic!("Couldn't create migrations directory: {}", why.to_string())
        });
    }

    // create the up.sql file
    let up_file = &project.initial_migration_up;

    // write the up.sql file
    write_to_file(&up_file, sql.as_bytes())
        .unwrap_or_else(|why| panic!("Couldn't write to up.sql: {}", why.to_string()));

    let sql_to_down = "
    DROP TABLE IF EXISTS users;
    ";

    // create the down.sql file
    let down_file = &project.initial_migration_down;

    // write the down.sql file
    write_to_file(&down_file, sql_to_down.as_bytes())
        .unwrap_or_else(|why| panic!("Couldn't write to down.sql: {}", why.to_string()));

    Ok(())
}
// Write the user-entered SQL queries to the up.sql file

#[derive(Debug)]
pub enum CustomMigrationError {
    MigrationError(MigrationError),
    IoError(std::io::Error),
    RunError(Box<dyn StdError + Send + Sync>),
}

impl Display for CustomMigrationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MigrationError(err) => Display::fmt(err, f),
            Self::RunError(err) => Display::fmt(err, f),
            Self::IoError(err) => Display::fmt(err, f),
        }
    }
}

impl StdError for CustomMigrationError {}

impl From<MigrationError> for CustomMigrationError {
    fn from(err: MigrationError) -> Self {
        Self::MigrationError(err)
    }
}

impl From<Box<dyn StdError + Send + Sync>> for CustomMigrationError {
    fn from(err: Box<dyn StdError + Send + Sync>) -> Self {
        Self::RunError(err)
    }
}

/// ## Name: run_migration
/// ### Description: Runs a migration
/// #### Parameters:
/// - name: [`&str`] - the name of the migration
/// - Returns: [`Result<(), std::io::Error>`]
/// - if the migration was created successfully: [`Ok(())`]
/// - if there was an error creating the migration: [`Err(std::io::Error)`]
///
/// ### Example:
/// ```rust
/// use rustyroad::database::migrations::run_migration;
///
/// run_migration("create_users_table").unwrap();
/// ```
pub async fn run_migration(
    migration_name: String
) -> Result<(), CustomMigrationError> {

    // get the database
    let database: Database = Database::get_database_from_rustyroad_toml().unwrap();

    let migrations_dir_path = format!("./config/database/migrations");
    // find the folder that has the name of the migration in the migrations directory with the latest timestamp
    let migration_dir_selected =find_migration_dir(migrations_dir_path.clone(), migration_name.clone()).unwrap_or_else(
        |why| panic!("Couldn't find migration directory: {}", why.to_string())
    );
    // Generate the path to the migrations directory at runtime
    let migration_dir = migrations_dir_path + &migration_dir_selected;
    println!("Migration directory: {:?}", migration_dir);
    // Get migration files from the specified directory
    let mut migration_files: Vec<_> = fs::read_dir(migration_dir)
        .unwrap_or_else(|why| panic!("Couldn't read migrations directory: {}", why.to_string()))
        .filter_map(Result::ok)
        .collect();
    // Sort the migration files based on their name (to apply them in order)
    migration_files.sort_by_key(|entry| entry.file_name());

    // Print the path to the migration directory and the migration name
    println!("Migration directory path: {:?}", &migrations_dir_path);
    println!("Migration name: {:?}", &migration_name.clone());

    // create the connection pool
    let  connection = Database::create_database_connection(&database).await.unwrap_or_else(|why| {
        panic!("Couldn't create database connection: {}", why.to_string())
    });

    // Determine the type of database and run the migrations
    match connection {
        DatabaseConnection::Sqlite(mut connection) =>
            execute_migration_with_connection(connection, migration_files).unwrap_or_else(|why| {
                panic!("Couldn't execute migration: {}", why.to_string())
            }),

        DatabaseConnection::Pg(mut connection) => {
            // Iterate over the migration files
            for entry in migration_files
        }
        DatabaseConnection::MySql(mut connection) => {

        }
    }

    println!("Migration executed successfully");
    Ok(())
}



/// Create a new migration file
/// # Arguments
/// * `up_sql_contents` - The contents of the up.sql file
///
/// # Returns
/// * `Result<String, Box<dyn Error>>` - The up_sql_content with Array column added or an error
///
/// # Example
/// ```
/// use rustyroad::RustyRoad;
/// let nullable = false;
/// let up_sql_contents = String::from("CREATE TABLE IF NOT EXISTS users (
///    id SERIAL PRIMARY KEY,
///   username VARCHAR(255) NOT NULL,
///  password VARCHAR(255) NOT NULL,
/// email VARCHAR(255) NOT NULL,
/// created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
/// );
/// ");
/// let array = rustyroad::database::create_array(up_sql_contents, nullable);
/// assert!(array.is_ok());
/// ```
///
pub fn create_array(up_sql_contents: String, nullable: bool) -> Result<String, Box<dyn Error>> {
    // ask the user how many dimensions the array should have
    let  dimensions = String::new();
    // Initialize the rustyline Editor with the default helper and in-memory history
    let mut rl = DefaultEditor::new().unwrap_or_else(|why| {
        panic!("Failed to create rustyline editor: {}", why.to_string());
    });
    rl.readline_with_initial(
        "How many dimensions should the array have? ",
        (dimensions.as_str(), ""),
    )
        .unwrap_or_else(|why| panic!("Failed to read input: {}", why.to_string()));

    // ask the user for the type of the array
    let  array_type = String::new();
    rl.readline_with_initial(
        "What type should the array have? ",
        (array_type.as_str(), ""),
    )
        .unwrap_or_else(|why| panic!("Failed to read input: {}", why.to_string()));

    // ask the user for the name of the array
    let  array_name = String::new();

    rl.readline_with_initial(
        "What should the name of the array be? ",
        (array_name.as_str(), ""),
    ).unwrap_or_else(|why| panic!("Failed to read input: {}", why.to_string()));

    // ask the user for the size of the array
    let  array_size = String::new();
    rl.readline_with_initial(
        "What should the size of the array be? ",
        (array_size.as_str(), ""),
    ).unwrap_or_else(|why| panic!("Failed to read input: {}", why.to_string()));

    // add the array to the up_sql_contents
    let mut up_sql_contents = up_sql_contents;

    match nullable {
        true => {
            up_sql_contents.push_str(&format!(
                "ALTER TABLE users ADD COLUMN {} {} ARRAY[{}] NULL;\n",
                array_name, array_type, array_size
            ));
        }
        false => {
            up_sql_contents.push_str(&format!(
                "ALTER TABLE users ADD COLUMN {} {} ARRAY[{}] NOT NULL;\n",
                array_name, array_type, array_size
            ));
        }
    }

    Ok(up_sql_contents)
}


/// # Name: find_migration_dir
/// ### Description: Find the migration directory of a given migration name
/// ### This is used in case there are multiple migrations with the same name and different timestamps
/// ### If there are multiple migrations with the same name and different timestamps, the user will be prompted to choose one
/// ### If there is only one migration with the given name, the user will not be prompted to choose one
/// Given: A migration name and a rustyroad project
/// When: The user wants to execute a migration
/// Then: The user will be prompted to choose a migration if there are multiple migrations with the same name and different timestamps
/// Then: The user will not be prompted to choose a migration if there is only one migration with the given name
/// Then: The path to the migration directory will be returned
/// # Arguments
/// * `migrations_dir_path` - The path to the migrations directory
/// * `migration_name` - The name of the migration
/// # Returns
/// * `Result<String, Error>` - The path to the migration directory or an error
pub fn find_migration_dir(migrations_dir_path: String, migration_name: String) -> Result<String, Box<dyn Error>> {
    // Initialize the rustyline Editor with the default helper and in-memory history
    let mut rl = DefaultEditor::new().unwrap_or_else(|why| {
        panic!("Failed to create rustyline editor: {}", why.to_string());
    });

    // get all the migration directories
    let mut migration_dirs = Vec::new();
    for entry in fs::read_dir(migrations_dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            migration_dirs.push(path);
        }
    }

    // filter the migration directories by the migration name
    let mut filtered_migration_dirs = Vec::new();
    for migration_dir in migration_dirs {
        let migration_dir_name = migration_dir.file_name()?.to_str().ok_or("Failed to convert OsStr to str")?;
        if migration_dir_name.contains(&migration_name) {
            filtered_migration_dirs.push(migration_dir);
        }
    }

    // if there is only one migration directory with the given name, return it
    if filtered_migration_dirs.len() == 1 {
        return Ok(filtered_migration_dirs[0].to_str().unwrap().to_string());
    }

    // if there are multiple migration directories with the given name, prompt the user to choose one
    if filtered_migration_dirs.len() > 1 {
        let mut migration_dir_names = Vec::new();
        for migration_dir in &filtered_migration_dirs {
            let migration_dir_name = migration_dir.file_name().unwrap().to_str().unwrap();
            migration_dir_names.push(migration_dir_name);
        }
        let mut migration_dir_name = rl.readline_with_initial(
            "Which migration do you want to execute? ",
            (migration_dir_names[0], ""),
        ).unwrap_or_else(|why| panic!("Failed to read input: {}", why.to_string()));

        print!("You chose: {}", migration_dir_name);

        for migration_dir in filtered_migration_dirs {
            let migration_dir_name_from_list = migration_dir.file_name()?.to_str().ok_or("Failed to convert OsStr to str")?;
            if migration_dir_name == migration_dir_name_from_list {
                return Ok(migration_dir.to_str().ok_or("Failed to convert PathBuf to str")?.to_string());
            }
        }

    }

    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to find migration directory")))
}

#[derive(Debug)]
pub enum MigrationError {
    Io(std::io::Error),
    Sql(sqlx::Error),
}

impl From<std::io::Error> for MigrationError {
    fn from(err: std::io::Error) -> MigrationError {
        MigrationError::Io(err)
    }
}

impl From<sqlx::Error> for MigrationError {
    fn from(err: sqlx::Error) -> MigrationError {
        MigrationError::Sql(err)
    }
}


async fn execute_migration_with_connection(
    connection: &DatabaseConnection,
    migration_files: Vec<DirEntry>,
) -> Result<String, MigrationError> {
    for entry in migration_files {
        let path = entry.path();
        // Ignore non-SQL files
        if path.extension() != Some(std::ffi::OsStr::new("sql")) {
            continue;
        }
        let mut file = fs::File::open(&path)?;
        let mut sql = String::new();
        file.read_to_string(&mut sql)?;

        // Split the SQL statements and execute each one separately
        let sql_statements: Vec<&str> = sql.split(';').collect();
        for statement in sql_statements {
            if statement.trim().is_empty() {
                continue;
            }
            // Execute the SQL statement
            match connection {
                DatabaseConnection::Pg(connection) => {
                    let cloned_connection = Arc::clone(connection);
                    cloned_connection.transaction(|connection| {
                        Box::pin(async move {
                            connection.execute(statement).await?;
                            Ok(())
                        }) as Pin<Box<dyn futures_util::Future<Output = Result<(), MigrationError>> + std::marker::Send>>
                    }).await?;
                }
                DatabaseConnection::MySql(connection) => {
                    let cloned_connection = Arc::clone(connection);
                    cloned_connection.transaction(|connection| {
                        Box::pin(async move {
                            connection.execute(statement).await?;
                            Ok(())
                        }) as Pin<Box<dyn futures_util::Future<Output = Result<(), MigrationError>> + std::marker::Send>>
                    }).await?;
                }
                DatabaseConnection::Sqlite(connection) => {
                    let cloned_connection = Arc::clone(connection);
                    cloned_connection.transaction(|connection| {
                        Box::pin(async move {
                            connection.execute(statement).await?;
                            Ok(())
                        }) as Pin<Box<dyn futures_util::Future<Output = Result<(), MigrationError>> + std::marker::Send>>
                    }).await?;
                }
            }
        }
    }
    Ok("Successfully executed migration".to_string())
}