use crate::database::{create_migration, Database, DatabaseType, find_migration_dir, MySqlTypes, PostgresTypes, SqliteTypes};
use crate::generators::create_file;
use crate::helpers::helpers::get_project_name_from_rustyroad_toml;
use color_eyre::{eyre::Error, Result};
use std::fs;
use std::path::Path;

pub async fn create_update_model(model_name: &str) -> Result<(), Error> {
    // check if the current directory is a rustyroad project
    // create the controller
    // check if the current directory is a rustyroad project
    let current_dir = fs::read_dir(".").unwrap();
    let mut has_rustyroad_toml = false;

    // check if the current directory has a rustyroad.toml file
    for entry in current_dir {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap();
        if file_name == "rustyroad.toml" {
            has_rustyroad_toml = true;
        }
    }
    // rustyroad.toml file will be used to store the project name and other project information
    // if the current directory does not have a rustyroad.toml file, it will return an error
    if !has_rustyroad_toml {
        println!(
            "This is not a rustyroad project. Please run this command in a rustyroad project."
        );
        // end the function
        return Ok(());
    }

    // ask the user what types they want to add to the model
    let mut types = String::new();

    println!("What types would you like to add to the model? (separate with commas)");

    std::io::stdin().read_line(&mut types).unwrap();

    let types = types.trim().to_string();

    let types = types.split(",").collect::<Vec<&str>>();

    let mut fields = String::new();

    println!("What fields would you like to add to the model? (separate with commas)");

    std::io::stdin().read_line(&mut fields).unwrap();

    let fields = fields.trim().to_string();

    let fields = fields.split(",").collect::<Vec<&str>>();

    let mut contents = String::new();

    contents.push_str("use serde::{Serialize, Deserialize};\n");

    contents.push_str("#[derive(Serialize, Deserialize)]\n");

    contents.push_str("#[derive(Debug)]\n");

    contents.push_str("#[derive(Clone)]\n");

    contents.push_str("#[derive(Queryable)]\n");

    contents.push_str("#[derive(Insertable)]\n");

    contents.push_str("#[table_name = \"");

    contents.push_str(&model_name);

    contents.push_str("\"]\n");

    contents.push_str("pub struct ");

    contents.push_str(&model_name);

    contents.push_str(" {\n");

    for (i, field) in fields.iter().enumerate() {
        contents.push_str("    pub ");
        contents.push_str(field);
        contents.push_str(": ");
        contents.push_str(types[i]);
        contents.push_str(",\n");
    }

    contents.push_str("}\n");

    contents.push_str("impl ");

    contents.push_str(&model_name);

    contents.push_str(" {\n");

    contents.push_str("    pub fn new(");

    for (i, field) in fields.iter().enumerate() {
        contents.push_str(field);
        contents.push_str(": ");
        contents.push_str(types[i]);
        contents.push_str(", ");
    }

    contents.push_str(") -> Self {\n");

    contents.push_str("        Self {\n");

    for (_i, field) in fields.iter().enumerate() {
        contents.push_str("            ");
        contents.push_str(field);
        contents.push_str(",\n");
    }

    contents.push_str("        }\n");

    contents.push_str("    }\n");

    contents.push_str("}\n");

    // create the migration

    // create the controller
    create_file(&format!("./src/models/{}.rs", model_name)).unwrap();

    // write the contents to the file
    fs::write(&format!("./src/models/{}.rs", model_name), contents)?;

    Ok(())
}

/// # Name: create_base_model
///
/// # Arguments
///
/// * `model_name` - The name of the model to create
///
/// # Description
///
/// This function will create a base model when a user runs the model command in the CLI.
///
/// # Example
///
/// ```
/// rustyroad::writers::models::create_base_model("User");
///
/// ```
///
/// # Returns
///
/// This function returns a Result of type (), or an error.
///
/// # Errors
///
/// This function will return an error if the current directory is not a rustyroad project.
pub async fn create_base_model(model_name: &str) -> Result<(), Error> {
    println!("Creating base model: {}", model_name);

    // check to see if this is a rustyroad project
    get_project_name_from_rustyroad_toml().expect("This is not a rustyroad project.");

    println!("This is a rustyroad project.");

    // based on the newly created migration, create the model
    // search the migrations folder for the migration that was just created
    let mut migration_dir = "./config/database/migrations".to_string();

    println!("Searching for migration: {}", model_name);

    let migration_dir_result = find_migration_dir(migration_dir.clone(), model_name.to_string());

    println!("Migration dir result: {:?}", migration_dir_result);

    if migration_dir_result.is_err() {
        println!("Creating migration: {}", model_name);
        // Pass an empty Vec for columns, as this path doesn't get them from CLI
        create_migration(model_name, Vec::new()).await.unwrap();
        println!("Searching for migration: {}", model_name);
        migration_dir = find_migration_dir(migration_dir, model_name.to_string()).unwrap();
        println!("Migration dir: {}", migration_dir);
    } else {
        println!("Migration found: {}", model_name);
        migration_dir = migration_dir_result.unwrap();
    }

    if std::path::Path::new(&migration_dir).exists() {
        println!("The migration exists.");
    } else {
        println!("The migration does not exist.");
    }

    // read the file path and get the contents of the migration
    let file_path_string = migration_dir + "/up.sql";
    let file_path = std::path::Path::new(&file_path_string);
    // validate the file path
    let file_path_bool = std::path::Path::is_file(&file_path);

    println!("File path: {:?}", file_path_bool);

    // get database type
    let database_type = Database::get_database_from_rustyroad_toml().unwrap().database_type;


    let struct_from_sql = match database_type {
        DatabaseType::Postgres => generate_struct_from_postgres_sql(file_path, &PostgresTypes::BigInt),
        DatabaseType::Mysql => generate_struct_from_mysql_sql(file_path, &MySqlTypes::TinyInt),
        DatabaseType::Sqlite => generate_struct_from_sqlite_sql(file_path, &SqliteTypes::Integer),
        DatabaseType::Mongo => todo!(),
    };

    //write the struct to the file
    let struct_from_sql = struct_from_sql.unwrap();

    let mut contents = String::new();

    contents.push_str("use serde::{Serialize, Deserialize};\n");

    contents.push_str("#[derive(Serialize, Deserialize)]\n");

    contents.push_str("#[derive(Debug)]\n");

    contents.push_str("#[derive(Clone)]\n");

    contents.push_str("#[derive(Queryable)]\n");

    contents.push_str("#[derive(Insertable)]\n");

    contents.push_str("#[table_name = \"");

    contents.push_str(&model_name);

    contents.push_str("\"]\n");

    contents.push_str("pub struct ");

    contents.push_str(&model_name);

    contents.push_str(" {\n");

    contents.push_str(&struct_from_sql);

    contents.push_str("}\n");

    contents.push_str("impl ");

    contents.push_str(&model_name);

    contents.push_str(" {\n");

    contents.push_str("    pub fn new(");

    // parse the sql and categorize it
    parse_and_categorize_sql(file_path).unwrap();

    // create the controller
    create_file(&format!("./src/models/{}.rs", model_name)).unwrap();

    // write the contents to the file
    fs::write(&format!("./src/models/{}.rs", model_name), &contents)?;

    println!("Contents: {}", contents);

    Ok(())
}



fn generate_struct_from_postgres_sql(file_path: &Path, db_type: &PostgresTypes) -> Result<String, Error> {
    // Read the SQL file
    let sql = fs::read_to_string(file_path)?;

    // Split the SQL into lines
    let lines: Vec<&str> = sql.lines().collect();

    // Initialize an empty string to hold the struct definition
    let mut struct_def = String::new();

    // Iterate over the lines of the SQL
    for line in lines {
        // If the line starts with "CREATE TABLE", extract the table name and start a new struct
        if line.to_uppercase().starts_with("CREATE TABLE") {
            let table_name = line.split_whitespace().nth(2).unwrap();
            struct_def.push_str(&format!("\n#[derive(Debug, sqlx::FromRow)]\nstruct {} {{\n", table_name));
        }
        // If the line starts with a word followed by a space and a word, treat it as a column definition
        else if line.chars().next().unwrap().is_alphanumeric() && line.contains(" ") {
            let parts: Vec<&str> = line.split_whitespace().take(2).collect();
            let column_name = parts[0];
            let column_type = match db_type {
                PostgresTypes::SmallInt | PostgresTypes::Integer => "i32",
                PostgresTypes::BigInt => "i64",
                PostgresTypes::Decimal | PostgresTypes::Numeric => "BigDecimal",
                PostgresTypes::Real => "f32",
                PostgresTypes::DoublePrecision => "f64",
                PostgresTypes::Serial | PostgresTypes::BigSerial => "u32",
                PostgresTypes::Money => "Money",
                PostgresTypes::Char | PostgresTypes::VarChar | PostgresTypes::Text => "String",
                PostgresTypes::ByteA => "Vec<u8>",
                PostgresTypes::Timestamp | PostgresTypes::TimestampWithTimeZone => "NaiveDateTime",
                PostgresTypes::Date => "NaiveDate",
                PostgresTypes::Time |  PostgresTypes::TimeWithoutTimeZone | PostgresTypes::TimestampWithoutTimeZone  => "NaiveTime",
                PostgresTypes::Interval => "Interval",
                PostgresTypes::Boolean => "bool",
                PostgresTypes::Json | PostgresTypes::JsonB => "serde_json::Value",
                PostgresTypes::Uuid => "Uuid",
                _ => "String", // Default to String for unknown types
            };
            struct_def.push_str(&format!("    {}: {},\n", column_name, column_type));
        }
        // If the line is ");", close the struct
        else if line.trim() == ");" {
            struct_def.push_str("}\n");
        }
    }

    Ok(struct_def)
}

fn generate_struct_from_mysql_sql(file_path: &Path, db_type: &MySqlTypes) -> Result<String, Error> {
    // Read the SQL file
    let sql = fs::read_to_string(file_path)?;

    // Split the SQL into lines
    let lines: Vec<&str> = sql.lines().collect();

    // Initialize an empty string to hold the struct definition
    let mut struct_def = String::new();

    // Iterate over the lines of the SQL
    for line in lines {
        // If the line starts with "CREATE TABLE", extract the table name and start a new struct
        if line.to_uppercase().starts_with("CREATE TABLE") {
            let table_name = line.split_whitespace().nth(2).unwrap();
            struct_def.push_str(&format!("\n#[derive(Debug, sqlx::FromRow)]\nstruct {} {{\n", table_name));
        }
        // If the line starts with a word followed by a space and a word, treat it as a column definition
        else if line.chars().next().unwrap().is_alphanumeric() && line.contains(" ") {
            let parts: Vec<&str> = line.split_whitespace().take(2).collect();
            let column_name = parts[0];
            let column_type = match db_type {
                MySqlTypes::TinyInt | MySqlTypes::SmallInt | MySqlTypes::MediumInt | MySqlTypes::Int | MySqlTypes::BigInt => "i32",
                MySqlTypes::Float | MySqlTypes::Double | MySqlTypes::Decimal => "BigDecimal",
                MySqlTypes::Bit => "bool",
                MySqlTypes::Char | MySqlTypes::VarChar | MySqlTypes::Binary | MySqlTypes::VarBinary | MySqlTypes::TinyBlob | MySqlTypes::Blob | MySqlTypes::MediumBlob | MySqlTypes::LongBlob | MySqlTypes::TinyText | MySqlTypes::Text | MySqlTypes::MediumText | MySqlTypes::LongText | MySqlTypes::Enum | MySqlTypes::Set => "String",
                MySqlTypes::Date | MySqlTypes::DateTime | MySqlTypes::Time | MySqlTypes::Timestamp | MySqlTypes::Year => "NaiveDateTime",
                MySqlTypes::Geometry | MySqlTypes::Point | MySqlTypes::LineString | MySqlTypes::Polygon | MySqlTypes::MultiPoint | MySqlTypes::MultiLineString | MySqlTypes::MultiPolygon | MySqlTypes::GeometryCollection => "String",
                MySqlTypes::Json | MySqlTypes::BinaryJson => "serde_json::Value",
                MySqlTypes::BitField => "String",
                MySqlTypes::NewDecimal => "BigDecimal",
                MySqlTypes::Boolean => "bool",
                MySqlTypes::NotFound => "String",
                MySqlTypes::EnumInner => "String",
                MySqlTypes::SetInner => "String",
                MySqlTypes::GeometryInner => "String"
            };
            struct_def.push_str(&format!("    {}: {},\n", column_name, column_type));
        }
        // If the line is ");", close the struct
        else if line.trim() == ");" {
            struct_def.push_str("}\n");
        }
    }

    Ok(struct_def)
}

fn generate_struct_from_sqlite_sql(file_path: &Path, db_type: &SqliteTypes) -> Result<String, Error> {
    // Read the SQL file
    let sql = fs::read_to_string(file_path)?;

    // Split the SQL into lines
    let lines: Vec<&str> = sql.lines().collect();

    // Initialize an empty string to hold the struct definition
    let mut struct_def = String::new();

    // Iterate over the lines of the SQL
    for line in lines {
        // If the line starts with "CREATE TABLE", extract the table name and start a new struct
        if line.to_uppercase().starts_with("CREATE TABLE") {
            let table_name = line.split_whitespace().nth(2).unwrap();
            struct_def.push_str(&format!("\n#[derive(Debug, sqlx::FromRow)]\nstruct {} {{\n", table_name));
        }
        // If the line starts with a word followed by a space and a word, treat it as a column definition
        else if line.chars().next().unwrap().is_alphanumeric() && line.contains(" ") {
            let parts: Vec<&str> = line.split_whitespace().take(2).collect();
            let column_name = parts[0];
            let column_type = match db_type {
                SqliteTypes::Integer | SqliteTypes::Real | SqliteTypes::Numeric => "BigDecimal",
                SqliteTypes::Date | SqliteTypes::Time | SqliteTypes::DateTime => "NaiveDateTime",
                SqliteTypes::Text => "String",
                SqliteTypes::Blob => "Vec<u8>",
                SqliteTypes::Boolean => "bool",
                SqliteTypes::Null => "String",
            };
            struct_def.push_str(&format!("    {}: {},\n", column_name, column_type));
        }
        // If the line is ");", close the struct
        else if line.trim() == ");" {
            struct_def.push_str("}\n");
        }
    }

    Ok(struct_def)
}

use regex::Regex;
fn parse_and_categorize_sql(file_path: &Path) -> Result<(), Error> {
    // Read the SQL file
    let sql = fs::read_to_string(file_path)?;

    // Define regex patterns for SQL keywords and constraints
    let select_pattern = Regex::new(r"(?i)SELECT").unwrap();
    let insert_pattern = Regex::new(r"(?i)INSERT").unwrap();
    let update_pattern = Regex::new(r"(?i)UPDATE").unwrap();
    let delete_pattern = Regex::new(r"(?i)DELETE").unwrap();
    let foreign_key_pattern = Regex::new(r"(?i)FOREIGN KEY").unwrap();
    let null_pattern = Regex::new(r"(?i)NULL").unwrap();
    let unique_pattern = Regex::new(r"(?i)UNIQUE").unwrap();
    let create_table_pattern = Regex::new(r"(?i)CREATE TABLE").unwrap();
    let create_index_pattern = Regex::new(r"(?i)CREATE INDEX").unwrap();
    let alter_table_pattern = Regex::new(r"(?i)ALTER TABLE").unwrap();
    let drop_table_pattern = Regex::new(r"(?i)DROP TABLE").unwrap();
    let drop_index_pattern = Regex::new(r"(?i)DROP INDEX").unwrap();


    // Split the SQL into statements
    let statements: Vec<&str> = sql.split(';').collect();

    // Iterate over the statements
    for statement in statements {
        if select_pattern.is_match(statement) {
            println!("SELECT statement: {}", statement);
        } else if insert_pattern.is_match(statement) {
            println!("INSERT statement: {}", statement);
        } else if update_pattern.is_match(statement) {
            println!("UPDATE statement: {}", statement);
        } else if delete_pattern.is_match(statement) {
            println!("DELETE statement: {}", statement);
        }  else if foreign_key_pattern.is_match(statement) {
            println!("FOREIGN KEY constraint found: {}", statement);
        } else if null_pattern.is_match(statement) {
            println!("NULL constraint found: {}", statement);
        } else if unique_pattern.is_match(statement) {
            println!("UNIQUE constraint found: {}", statement);
        } else if create_table_pattern.is_match(statement) {
            println!("CREATE TABLE statement: {}", statement);
        } else if create_index_pattern.is_match(statement) {
            println!("CREATE INDEX statement: {}", statement);
        } else if alter_table_pattern.is_match(statement) {
            println!("ALTER TABLE statement: {}", statement);
        } else if drop_table_pattern.is_match(statement) {
            println!("DROP TABLE statement: {}", statement);
        } else if drop_index_pattern.is_match(statement) {
            println!("DROP INDEX statement: {}", statement);
        } else {
            println!("Other statement: {}", statement);
        }
    }

    Ok(())
}