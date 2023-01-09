// RustyRoad

// This is the main file for the RustyRoad project.
// It is the entry point for the program.

use rustyroad::Project;
//! Fast and easy queue abstraction.
//!
//! Provides an abstraction over a queue.  When the abstraction is used
//! there are these advantages:
//! - Fast
//! - [`Easy`]
//!
//! [`Easy`]: http://thatwaseasy.example.com

//! The main function for the RustyRoad project.
pub fn main() {
    Project::initial_prompt().expect("Failed to create project");
}
