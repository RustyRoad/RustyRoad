use super::error::MigrationValidationError;

type Result = std::result::Result<(), MigrationValidationError>;

pub(super) fn combine(first: Result, second: Result) -> Result {
    match (first, second) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(error), Ok(())) | (Ok(()), Err(error)) => Err(error),
        (Err(first), Err(second)) => Err(join(first, second)),
    }
}

pub(super) fn after_validation(validation: Result, cleanup: Result) -> Result {
    match (validation, cleanup) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(error), Ok(())) | (Ok(()), Err(error)) => Err(error),
        (Err(validation), Err(cleanup)) => Err(join(validation, cleanup)),
    }
}

fn join(
    first: MigrationValidationError,
    second: MigrationValidationError,
) -> MigrationValidationError {
    MigrationValidationError::new(format!("{first}\nAdditionally, cleanup failed: {second}"))
}
