/// Ordered metrics, optionally grouped by one existing database column.
/// Omitting group_by returns one aggregate row, even when no rows match.
/// # Examples
/// ```toml
/// [models.rust_group_metrics.ab_variant_event_metrics]
/// table = "ab_events"
/// group_by = "variant_id"
/// metrics = [{kind = "count", column = "event_type", equals = "impression"}]
/// [models.rust_group_metrics.platform_status_counts]
/// table = "platform_trash_zone_mappings"
/// filter_by = "platform"
/// metrics = [{kind = "count_all"}, {kind = "count", column = "status", equals = "ACTIVE"}]
/// ```
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroupMetrics {
    pub table: String,
    #[serde(default)]
    pub group_by: String,
    /// Optional runtime text equality parameter, cast to its catalog enum if needed.
    #[serde(default)]
    pub filter_by: Option<String>,
    pub metrics: Vec<ConditionalMetric>,
}

/// A conditional count, sum, or nullable maximum timestamp.
/// Sum casts after aggregation so decimal accumulation retains its precision.
/// CountAll always returns i64; ungrouped conditional counts also return i64.
/// # Examples
/// ```toml
/// [[models.rust_group_metrics.ab_variant_event_metrics.metrics]]
/// kind = "sum"
/// column = "event_type"
/// equals = "conversion"
/// value = "value"
/// ```
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ConditionalMetric {
    CountAll,
    MaxTimestamp {
        column: String,
    },
    Count {
        column: String,
        equals: String,
    },
    Sum {
        column: String,
        equals: String,
        value: String,
    },
}
