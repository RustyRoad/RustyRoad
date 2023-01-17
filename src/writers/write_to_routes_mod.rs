use crate::writers::general;

/// Name: write_to_routes_mod
/// Description: Writes to the routes/mod.rs file
/// Calls the create_module function from the general module writer
pub fn write_to_routes_mod(file_name: &String) -> Result<(), std::io::Error> {
    let mut components = Vec::new();
    components.push("index".to_string());
    //components.push("about".to_string());
    //components.push("contact".to_string());
    general::create_module(file_name, components)
}
