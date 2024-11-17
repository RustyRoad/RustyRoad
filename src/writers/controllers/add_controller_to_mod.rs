use std::fs;
use std::path::Path;
use regex::Regex;
use eyre::Error;

/// # Name: add_module_declaration
/// This function adds a new module declaration and use declaration to the given path.
/// # Arguments
/// * model_name: String
/// * file_path: &Path
/// # Returns:
/// * Result<(), Error>
/// # Example:
/// ```
/// use rustyroad::writers::add_module_declaration;
/// use std::path::Path;
/// use eyre::Error;
///
/// let model_name = "test".to_string();
/// let file_path = Path::new("./src/controllers/mod.rs");
/// add_module_declaration(model_name, &file_path);
/// ```
pub fn add_module_declaration(model_name: String, file_path: &Path) -> Result<(), Error> {
    // Read the file into a string
    let mut contents = fs::read_to_string(file_path)
        .map_err(|e| Error::msg(format!("Failed to read file: {:?}, error: {}", file_path, e)))?;

    // Prepare the new controller mod and use declarations with newlines
    let new_controller_mod = format!("\npub mod {};\n", model_name);
    let new_controller_use = format!("\npub use {}::*;\n", model_name);

    // Regular expressions
    let re_mod = Regex::new(r"pub mod \w+;\n?").unwrap(); // Optional newline at the end
    let re_use = Regex::new(r"pub use \w+::\*;\n?").unwrap(); // Optional newline at the end

    // Find positions
    let last_mod_end_pos = re_mod
        .find_iter(&contents)
        .last()
        .map_or(0, |m| m.end());

    let last_use_end_pos = re_use
        .find_iter(&contents)
        .last()
        .map_or(0, |m| m.end());

    // Insert the new controller mod and use declarations
    contents.insert_str(last_mod_end_pos, &new_controller_mod);
    contents.insert_str(last_use_end_pos + new_controller_mod.len(), &new_controller_use);

    // Write the updated contents back to the file
    fs::write(file_path, contents)
        .map_err(|e| Error::msg(format!("Failed to write to file: {:?}, error: {}", file_path, e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_add_controller_to_module_with_mixed_declarations() {
        let model_name = "test".to_string();
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("mod.rs");

        let mut file = File::create(&file_path).unwrap();
        // Simulating the mixed up mod and use declarations
        writeln!(file, "pub mod index;\n").unwrap();
        writeln!(file, "pub use page::*;").unwrap();
        writeln!(file, "pub use index::*;pub mod dashboard;").unwrap();
        writeln!(file, "pub use dashboard::*;pub mod login;\n").unwrap();
        writeln!(file, "pub mod page;").unwrap();
        writeln!(file, "pub use login::*;").unwrap();

        let result = add_module_declaration(model_name, &file_path);
        assert!(result.is_ok());

        let modified_contents = fs::read_to_string(&file_path).unwrap();
        assert!(modified_contents.contains("pub mod test;\n"));
        assert!(modified_contents.contains("pub use test::*;\n"));
        // Further assertions can be added to verify the correct organization of declarations
    }
}
