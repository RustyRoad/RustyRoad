//! HTTP verbs used by generated operations.

/// The HTTP verb an operation uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::generators::typescript::heyapi) enum Verb {
    Get,
    Post,
    Patch,
    Delete,
}

impl Verb {
    /// Returns the lowercase method name OpenAPI keys operations by.
    pub(in crate::generators::typescript::heyapi) fn method(self) -> &'static str {
        match self {
            Self::Get => "get",
            Self::Post => "post",
            Self::Patch => "patch",
            Self::Delete => "delete",
        }
    }

    /// Returns `true` when the verb carries a request body.
    pub(in crate::generators::typescript::heyapi) fn has_body(self) -> bool {
        matches!(self, Self::Post | Self::Patch)
    }
}
