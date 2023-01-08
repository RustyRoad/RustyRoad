// RustyRoad

// This is the main file for the RustyRoad project.
// It is the entry point for the program.

use rustyroad::Project;
fn main() {
    Project::initial_prompt().expect("Failed to create project");
}
