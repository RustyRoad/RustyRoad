//! History forms a linear parent chain.

use super::support::sqlite;
use crate::database::versions::{lifecycle, query};

#[tokio::test]
async fn history_is_a_linear_parent_chain() {
    let connection = sqlite().await;

    for version in ["01_first", "02_second", "03_third"] {
        lifecycle::start(&connection, "main", version, &[])
            .await
            .unwrap();
        lifecycle::complete(&connection, "main").await.unwrap();
    }

    // The first migration is the root; each later one points at its predecessor.
    assert_eq!(query::parent_of(&connection, "01_first").await.unwrap(), None);
    assert_eq!(
        parent(&connection, "02_second").await.as_deref(),
        Some("01_first")
    );
    assert_eq!(
        parent(&connection, "03_third").await.as_deref(),
        Some("02_second")
    );
}

/// Reads a migration's parent.
async fn parent(
    connection: &crate::database::DatabaseConnection,
    name: &str,
) -> Option<String> {
    query::parent_of(connection, name).await.unwrap()
}
