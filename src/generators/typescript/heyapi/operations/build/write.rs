//! Mutating operations.

use super::super::{Operation, Verb};

/// Builds the create.
pub(super) fn create(entity: &str, url: &str) -> Operation {
    Operation {
        name: format!("create{entity}"),
        tag: entity.to_string(),
        verb: Verb::Post,
        url: url.to_string(),
        row_type: Some(entity.to_string()),
        is_collection: false,
        path_param: None,
        can_be_missing: false,
        success: 201,
    }
}

/// Builds the partial update.
pub(super) fn update(entity: &str, url: &str, key_type: &'static str) -> Operation {
    Operation {
        name: format!("update{entity}"),
        tag: entity.to_string(),
        verb: Verb::Patch,
        url: url.to_string(),
        row_type: Some(entity.to_string()),
        is_collection: false,
        path_param: Some(key_type),
        can_be_missing: true,
        success: 200,
    }
}

/// Builds the delete, whose 204 carries no body.
pub(super) fn delete(entity: &str, url: &str, key_type: &'static str) -> Operation {
    Operation {
        name: format!("delete{entity}"),
        tag: entity.to_string(),
        verb: Verb::Delete,
        url: url.to_string(),
        row_type: None,
        is_collection: false,
        path_param: Some(key_type),
        can_be_missing: true,
        success: 204,
    }
}
