use std::fs;
use crate::database::{create_migration};
use crate::generators::create_file;


// so we are actually going to use this to create an update d
pub async fn create_update_model(model_name: &str) -> color_eyre::Result<(), color_eyre::eyre::Error> {
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

    // look for an existing model with the same name
    let model_dir = fs::read_dir("./src/models").unwrap();

    let mut has_model = false;

    for entry in model_dir {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap();
        if file_name == format!("{}.rs", model_name) {
            has_model = true;
        }
    }



    // if !has_model create a file with the model name
   let mut contents = if !has_model {
            create_file(&format!("./src/models/{}.rs", model_name)).unwrap_or_else(|why| {
                println!("Failed to create file: {:?}", why.to_string());
            });
            create_migration(model_name).await.expect("Failed to create migration")
        } else  {
       fs::read_to_string(&format!("./src/models/{}.rs", model_name))?
    };

println!("You're model is: {}", &model_name);
    println!("The contents of the model are: {}", contents);

    let raw_update_model = format!("

    pub async fn update_{}(id: i32, {}_update: {}) -> Result<{}, Error> {{
        let conn = establish_connection();
        let result = diesel::update({}::table.find(id))
            .set({}_update)
            .get_result(&conn);
        match result {{
            Ok({}_update) => Ok({}_update),
            Err(e) => Err(e),
        }}
    }}

    ", model_name, model_name, model_name, model_name, model_name, model_name, model_name, model_name);

    let update_model = raw_update_model.replace("{}", &model_name);
    println!("Update model is: {}", update_model);

    // add the update model to the contents
    contents.push_str(&update_model);

    // write the contents to the file
    fs::write(&format!("./src/models/{}.rs", model_name), contents)?;


    Ok(())
}