use super::error::MigrationValidationError;

pub(super) fn require_test(environment: &str) -> Result<(), MigrationValidationError> {
    if environment == "test" {
        return Ok(());
    }
    Err(MigrationValidationError::new(format!(
        "Refusing to validate migrations with ENVIRONMENT='{environment}'.\n\n\
         Migration validation is allowed only with ENVIRONMENT=test (or ENV=test) and reads \
         rustyroad.test.toml. It never runs against the dev, shared test, or production database."
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_test_is_allowed() {
        assert!(require_test("test").is_ok());
        for value in ["dev", "production", "prod", "staging", "TEST"] {
            assert!(require_test(value).is_err());
        }
    }
}
