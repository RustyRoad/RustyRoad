// CrabbyRails

// This is the main file for the CrabbyRails project.
// It is the entry point for the program.

use crabbyrail::Project;
fn main() {
    Project::initial_prompt().expect("Failed to create project");
}
