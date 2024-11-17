pub mod helpers {
    use color_eyre::eyre::Result;
    use eyre::Error;
    use std::fs;
    use toml::Value;
    use std::path::Path;

    pub fn get_project_name_from_rustyroad_toml() -> Result<String, Error> {
        let file = fs::read_to_string("rustyroad.toml")
            .unwrap_or_else(|_| panic!("Error: Could not find rustyroad.toml"));
        let toml: Value = toml::from_str(&file).unwrap();
        let project_table = toml["rustyroad_project"].as_table().unwrap();
        Ok(project_table["name"].as_str().unwrap().to_string())
    }

    pub fn capitalize_first(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }



    use regex::Regex;
    use lazy_static::lazy_static;
    use std::collections::HashSet;


    lazy_static! {
    static ref IMPORT_REGEX: Regex = Regex::new(r"use [a-zA-Z_]+::\{[a-zA-Z_:, ]+\};").unwrap();
    static ref MODULE_REGEX: Regex = Regex::new(r"use [a-zA-Z_]+::").unwrap();
}

    /// Adds a new import item to an existing module import statement or creates a new import statement if not present.
    ///
    /// This function checks if the specified module is already imported in the given content.
    /// If the module is imported with curly braces, it adds the new item to this import.
    /// If the specific item is already imported, it leaves the content unchanged.
    /// Otherwise, it adds a new import statement for the specified item.
    ///
    /// # Arguments
    ///
    /// * `contents` - A string slice that holds the current content where imports are to be added or updated.
    /// * `module` - The name of the module to which the import item belongs.
    /// * `import_item` - The import item that needs to be added to the module import statement.
    ///
    /// # Returns
    ///
    /// Returns a `String` with the updated content.
    /// If the module is already imported with curly braces, the new item is added to the existing import.
    /// If the specific item is already imported, the content is left unchanged.
    /// Otherwise, a new import statement is added for the specified item.
    ///
    /// # Example
    ///
    /// ```
    /// use rustyroad::helpers::helpers::add_or_update_import;
    ///
    /// let contents = "use rustyroad::writers::create_create_controller_in_new_folder;";
    /// let module = "rustyroad::writers";
    /// let import_item = "create_create_controller_in_new_folder";
    ///
    /// let updated_contents = add_or_update_import(contents, module, import_item);
    ///
    /// assert_eq!(updated_contents, "use rustyroad::writers::{create_create_controller_in_new_folder};");
    /// ```


    pub fn add_or_update_import(contents: &str, module: &str, import_item: &str) -> String {
        let import_regex = Regex::new(&format!(r"use {}::\{{[a-zA-Z_:, ]*\}};", module)).unwrap();

        let mut crate_imports = Vec::new();
        let mut other_imports = Vec::new();
        let mut non_import_lines = Vec::new();
        let mut found = false;

        for line in contents.lines() {
            if import_regex.is_match(line) {
                found = true;
                let mut items: HashSet<&str> = line.trim_start_matches(&format!("use {}::{{", module))
                    .trim_end_matches("};")
                    .split(", ")
                    .collect();

                items.insert(import_item);

                let new_import_line = format!("use {}::{{{}}};", module, items.into_iter().collect::<Vec<&str>>().join(", "));
                if new_import_line.starts_with("use crate::") {
                    crate_imports.push(new_import_line);
                } else {
                    other_imports.push(new_import_line);
                }
            } else {
                non_import_lines.push(line.to_string());
            }
        }

        if !found && module.starts_with("crate::") {
            crate_imports.push(format!("use {}::{{{}}};", module, import_item));
        } else if !found {
            other_imports.push(format!("use {}::{{{}}};", module, import_item));
        }

        let mut sorted_imports = crate_imports;
        sorted_imports.extend(other_imports);

        let mut updated_contents = sorted_imports.join("\n");
        if !updated_contents.is_empty() {
            updated_contents.push('\n');
        }

        updated_contents + &non_import_lines.join("\n")


    }

    /// # Name: determine_controller_path
    /// ### Description:
    /// - Determines the path to the controller file
    /// ### Parameters:
    /// - model_name: &str
    /// ### Returns:
    /// - String
    /// ### Example:
    /// ```
    /// use rustyroad::helpers::helpers::determine_controller_path;
    ///
    /// let path = determine_controller_path("page");
    /// ```
    pub fn determine_controller_path(model_name: &str) -> String {
        // Logic to determine the controller path
        let path = format!("./src/controllers/{}.rs", model_name);
        if Path::new(&path).exists() {
            path
        } else {
            format!("./src/controllers/{}/{}.rs", model_name, model_name)
        }
    }

    pub fn prompt_to_create_controller(path: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("The controller at path {} does not exist. Would you like to create it? (y/n)", path);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim().eq_ignore_ascii_case("y") {
            let parent_dir = Path::new(path).parent().ok_or("Failed to find parent directory")?;
            fs::create_dir_all(parent_dir)?;
            println!("Creating controller at path {}", path);
            fs::File::create(path)?;
        } else {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Controller creation aborted by user")));
        }
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_add_or_update_import() {
          let mut base_content = String::from("use actix_web::{HttpResponse, Responder};\nuse some_other_module;\n");

            // Add various imports
            base_content = add_or_update_import(&base_content, "actix_web", "get");

            base_content = add_or_update_import(&base_content, "actix_web", "web");

            base_content = add_or_update_import(&base_content, "crate", "models");

            base_content = add_or_update_import(&base_content, "crate", "helpers");
            base_content = add_or_update_import(&base_content, "tera", "Context");

            base_content = add_or_update_import(&base_content, "tera", "Tera");

            base_content = add_or_update_import(&base_content, "actix_identity", "Identity");

            base_content = add_or_update_import(&base_content, "user", "UserLogin");


            // Re-add the same imports to check for duplicates
            base_content = add_or_update_import(&base_content, "actix_web", "get");

            base_content = add_or_update_import(&base_content, "tera", "Context");

            base_content = add_or_update_import(&base_content, "actix_web", "web::Json");
            base_content = add_or_update_import(&base_content, "actix_web", "web::Path");

            // Split content into lines for index comparison
            let lines: Vec<&str> = base_content.lines().collect();

            // Find indices of specific imports
            let crate_import_index = lines.iter().position(|&line| line.starts_with("use crate::")).expect("Failed to find 'use crate::'");
            let _actix_web_import_index = lines.iter().position(|&line| line.starts_with("use actix_web::")).expect("Failed to find 'use actix_web::'");
            let tera_import_index = base_content.find("use tera::").expect("Failed to find 'use tera::'");
            let actix_identity_import_index = base_content.find("use actix_identity::").expect("Failed to find 'use actix_identity::'");
            let user_import_index = base_content.find("use user::").expect("Failed to find 'use user::'");

            //assert!(crate_import_index < actix_web_import_index);
            assert!(crate_import_index < tera_import_index);
            assert!(crate_import_index < actix_identity_import_index);
            assert!(crate_import_index < user_import_index);

            // Check for no duplicate imports
            assert_eq!(base_content.matches("use actix_web::{get,").count(), 1);
            assert_eq!(base_content.matches("use tera::Context;").count(), 1);
        }
    }

}
