//! Route metadata shared by the paths and schema generators.

mod build;
mod verb;

pub(super) use build::operations;
pub(super) use verb::Verb;

/// One generated API operation.
pub(super) struct Operation {
    /// Function name, matching the route's `operationId`.
    ///
    /// Hey API derives SDK function names from this, so it must stay stable.
    pub name: String,
    /// Tag grouping the operation, which Hey API uses to organize the SDK.
    pub tag: String,
    pub verb: Verb,
    /// URL with an `{id}` placeholder where applicable.
    pub url: String,
    /// Row schema this operation returns, if any.
    pub row_type: Option<String>,
    /// Whether the response is an array of rows.
    pub is_collection: bool,
    /// TypeScript type of the path parameter, if the route takes one.
    pub path_param: Option<&'static str>,
    /// Whether a missing row yields a 404.
    pub can_be_missing: bool,
    /// Status code returned on success.
    pub success: u16,
}
