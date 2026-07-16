use super::error::MigrationValidationError;
use super::identity::{DATABASE_PREFIX, USER_PREFIX};

pub(super) fn database(name: &str) -> Result<(), MigrationValidationError> {
    identifier(name, DATABASE_PREFIX, "database")
}

pub(super) fn user(name: &str) -> Result<(), MigrationValidationError> {
    identifier(name, USER_PREFIX, "user")
}

pub(super) fn password(value: &str) -> Result<(), MigrationValidationError> {
    if value.len() >= 32
        && value
            .chars()
            .all(|character| character.is_ascii_alphanumeric())
    {
        return Ok(());
    }
    Err(MigrationValidationError::new(
        "Refusing to use an unsafe validation database password",
    ))
}

fn identifier(value: &str, prefix: &str, kind: &str) -> Result<(), MigrationValidationError> {
    let characters_are_safe = value.chars().all(|character| {
        character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
    });
    if value.starts_with(prefix) && value.len() > prefix.len() && characters_are_safe {
        return Ok(());
    }
    Err(MigrationValidationError::new(format!(
        "Refusing to create or drop unsafe validation {kind} name '{value}'"
    )))
}
