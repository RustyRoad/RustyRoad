//! Builders carrying size, precision, or mode options.

use super::parse::modifier;
use super::Builder;

/// Builds a length-carrying builder such as `varchar({ length: 255 })`.
pub(super) fn sized(import: &'static str, lowered: &str) -> Builder {
    match modifier(lowered) {
        Some(length) => Builder::with_options(import, format!("{{ length: {length} }}")),
        None => Builder::plain(import),
    }
}

/// Builds `numeric({ precision, scale })`.
pub(super) fn numeric(lowered: &str) -> Builder {
    let Some(inner) = modifier(lowered) else {
        return Builder::plain("numeric");
    };

    let mut parts = inner.split(',').map(str::trim);
    let precision = parts.next().unwrap_or_default();
    let options = match parts.next() {
        Some(scale) => format!("{{ precision: {precision}, scale: {scale} }}"),
        None => format!("{{ precision: {precision} }}"),
    };

    Builder::with_options("numeric", options)
}

/// Builds a timestamp builder carrying mode, precision, and timezone.
///
/// `mode: 'string'` keeps the value as the driver returned it, avoiding a Date
/// round trip that would lose the original precision.
pub(super) fn timestamp(lowered: &str, with_timezone: bool) -> Builder {
    let mut options = vec!["mode: 'string'".to_string()];
    if let Some(precision) = modifier(lowered) {
        options.push(format!("precision: {precision}"));
    }
    if with_timezone {
        options.push("withTimezone: true".to_string());
    }

    Builder::with_options("timestamp", format!("{{ {} }}", options.join(", ")))
}
