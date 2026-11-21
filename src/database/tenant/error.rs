use std::{error::Error, fmt};

/// Redacted tenant-provisioning diagnostic with a stable phase and SQLSTATE.
/// SQL messages are excluded because they can contain credential-bearing SQL.
/// # Examples
/// ```
/// use rustyroad::database::tenant::ProvisionError;
/// let error = ProvisionError::invalid("tenant_identity");
/// assert!(error.to_string().contains("tenant_identity"));
/// ```
#[derive(Debug)]
pub struct ProvisionError {
    pub phase: &'static str,
    pub sqlstate: Option<String>,
}

/// Creates validation and database diagnostics without carrying credentials.
/// # Examples
/// ```
/// let e = rustyroad::database::tenant::ProvisionError::invalid("ownership");
/// assert!(e.sqlstate.is_none());
/// ```
impl ProvisionError {
    /// Records a failed invariant before a mutation, without serializing inputs.
    /// # Examples
    /// ```
    /// let e = rustyroad::database::tenant::ProvisionError::invalid("postgres_only");
    /// assert_eq!(e.phase, "postgres_only");
    /// ```
    pub fn invalid(phase: &'static str) -> Self {
        Self { phase, sqlstate: None }
    }

    /// Preserves SQLSTATE while discarding SQL text and server error messages.
    /// # Examples
    /// ```
    /// let e = rustyroad::database::tenant::ProvisionError::database(
    ///     "connect", &sqlx::Error::RowNotFound,
    /// );
    /// assert_eq!(e.phase, "connect");
    /// ```
    pub fn database(phase: &'static str, error: &sqlx::Error) -> Self {
        let sqlstate = error.as_database_error()
            .and_then(|error| error.code()).map(|code| code.into_owned());
        Self { phase, sqlstate }
    }
}

/// Formats a diagnostic that is safe to return without exposing server SQL.
/// # Examples
/// ```
/// let e = rustyroad::database::tenant::ProvisionError::invalid("ownership");
/// assert!(format!("{e}").starts_with("TENANT_PROVISIONING"));
/// ```
impl fmt::Display for ProvisionError {
    /// Formats phase and optional SQLSTATE only.
    /// # Examples
    /// ```
    /// let e = rustyroad::database::tenant::ProvisionError::invalid("identity");
    /// assert!(e.to_string().contains("identity"));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TENANT_PROVISIONING: {}; SQLSTATE={:?}", self.phase, self.sqlstate)
    }
}

/// Supports error propagation without retaining a credential-bearing source.
/// # Examples
/// ```
/// let e: Box<dyn std::error::Error> = Box::new(
///     rustyroad::database::tenant::ProvisionError::invalid("identity"),
/// );
/// assert!(e.source().is_none());
/// ```
impl Error for ProvisionError {}
