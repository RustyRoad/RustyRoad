use rustyroad::Project;

pub fn main() {
    Project::initial_prompt().expect("Failed to create project");
}
