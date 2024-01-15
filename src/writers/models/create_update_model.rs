use std::fs;
use crate::database::{create_migration, find_migration_dir};
use crate::generators::create_file;
use color_eyre::{eyre::Error, Result};
use crate::helpers::helpers::get_project_name_from_rustyroad_toml;

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
    create_file(&format!("./src/models/{}.rs", model_name));

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

    // check to see if this is a rustyroad project
    get_project_name_from_rustyroad_toml().expect("This is not a rustyroad project.");

    create_migration(model_name).await.expect("Could not create migration");

    // based on the newly created migration, create the model
    // search the migrations folder for the migration that was just created
    let mut migration_dir = "./config/database/migrations".to_string();
    migration_dir = find_migration_dir(migration_dir, model_name.to_string()).expect("Could not find migration dir");

    // read the contents of the migration
    let contents = fs::read_to_string(&migration_dir).expect("Could not read migration file");

    // get the fields from the migration

    let mut fields = String::new();

    let mut types = String::new();

    let mut is_field = false;

    let mut is_type = false;

    for c in contents.chars() {
        if c == '(' {
            is_field = true;
        }
        if c == ')' {
            is_field = false;
        }
        if is_field {
            if c != '(' && c != ')' && c != ',' {
                fields.push(c);
            }
        }
        if c == '[' {
            is_type = true;
        }
        if c == ']' {
            is_type = false;
        }
        if is_type {
            if c != '[' && c != ']' && c != ',' {
                types.push(c);
            }
        }
    }

    let fields = fields.split(",").collect::<Vec<&str>>();

    let types = types.split(",").collect::<Vec<&str>>();

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

    Ok(())
}

