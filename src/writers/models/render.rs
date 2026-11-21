//! Render SQL-derived models without nesting their declarations or mixing ORM APIs.

mod constructor;
mod declaration;
mod derives;
mod guards;

use std::io;

/// SQL generators return one complete struct. Finish it before any file is written.
pub(super) fn render_base_model(model_name: &str, source: &str) -> io::Result<String> {
    declaration::Declaration::parse(model_name, source)?.render()
}

#[cfg(test)]
#[path = "render_tests.rs"]
mod tests;
