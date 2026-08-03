//! Read operations.

use super::super::{Operation, Verb};

/// Builds the collection read.
pub(super) fn list(entity: &str, url: &str) -> Operation {
    Operation {
        name: format!("list{entity}"),
        tag: entity.to_string(),
        verb: Verb::Get,
        url: url.to_string(),
        row_type: Some(entity.to_string()),
        is_collection: true,
        path_param: None,
        can_be_missing: false,
        success: 200,
    }
}

/// Builds the single-row read.
pub(super) fn get(entity: &str, url: &str, key_type: &'static str) -> Operation {
    Operation {
        name: format!("get{entity}"),
        tag: entity.to_string(),
        verb: Verb::Get,
        url: url.to_string(),
        row_type: Some(entity.to_string()),
        is_collection: false,
        path_param: Some(key_type),
        can_be_missing: true,
        success: 200,
    }
}
