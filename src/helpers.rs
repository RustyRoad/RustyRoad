pub mod helpers {
    use color_eyre::eyre::Result;
    use eyre::Error;
    use std::fs;
    use toml::Value;

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
}
