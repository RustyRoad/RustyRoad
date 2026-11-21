/// Mutation performed after selecting a related row by its unique identity.
/// # Examples
/// ```toml
/// operation = "create"
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelatedOperation {
    Create,
    Update,
}

/// A foreign-key write with explicit value columns and a unique related lookup.
/// Omitted creation columns retain database defaults. Updates require predicates.
/// # Examples
/// ```toml
/// [models.rust_related_writes.create_passkey_for_username]
/// table = "passkeys"
/// foreign_key = "passkeys_user_id_fkey"
/// lookup = "username"
/// operation = "create"
/// columns = ["id", "credential_id", "passkey"]
/// ```
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RelatedWrite {
    pub table: String,
    pub foreign_key: String,
    pub lookup: String,
    pub operation: RelatedOperation,
    pub columns: Vec<String>,
    #[serde(default)]
    pub predicates: Vec<String>,
    #[serde(default)]
    pub clock: Option<String>,
}
