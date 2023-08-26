use crate::writers::write_to_file;
use color_eyre::eyre::Result;

/// # Name: create_module
/// # Description: Creates a module
/// # Arguments:
/// ## * `name` - The name of the module -```String```
/// ## * `components` - The components of the module - ```<Vec<String>>```
/// This function will be use to generate modules.
/// This function will take two arguments, the name of the module, and the different components that will be in the module.
/// The components should be a vector of strings.
/// The function will create a template that gets passed to the writer.
/// The vector of strings will be looped through to create the template that gets passed to the writer
pub fn write_to_module(name: &String, components: Vec<String>) -> Result<(), std::io::Error> {
    let mut template = String::new();

    for component in &components {
        template.push_str(&format!("pub mod {};", component));
    }

    // Add an empty line
    template.push_str(
        "
",
    );

    // use the modules after they are created

    for component in &components {
        template.push_str(&format!("pub use {}::*;", component));
    }

    // write the template to the file
    write_to_file(&name, template.as_bytes())
}
