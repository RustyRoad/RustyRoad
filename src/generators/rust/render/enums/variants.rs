//! The variants of one generated enum.

use crate::database::introspection::Enum;
use crate::generators::rust::casing;

/// Renders every variant, keeping each identifier distinct.
///
/// Two labels can fold to one identifier — `campaign_type` and `campaign-type` both become
/// `CampaignType` — which declares the same variant twice and fails to compile. Labels are
/// unique, so a numeric suffix separates them while each keeps its own rename.
pub(super) fn render(item: &Enum) -> String {
    let mut taken: Vec<String> = Vec::with_capacity(item.values.len());
    let mut rendered = String::new();

    for (index, value) in item.values.iter().enumerate() {
        let ident = unique(casing::to_pascal(value), &taken);
        rendered.push_str(&variant(value, &ident, index == 0));
        taken.push(ident);
    }

    rendered
}

/// Returns `candidate`, suffixed if it is already taken.
fn unique(candidate: String, taken: &[String]) -> String {
    if !taken.contains(&candidate) {
        return candidate;
    }

    // Starts at 2 because the unsuffixed identifier is conceptually the first.
    (2..)
        .map(|index| format!("{candidate}{index}"))
        .find(|suffixed| !taken.contains(suffixed))
        .expect("an unbounded range always yields an unused identifier")
}

/// Renders one variant, renamed when the identifier differs from the stored value.
///
/// The first declared variant is marked `#[default]`, since a derived `Default` needs one and
/// declaration order is the only ranking the database gives us.
fn variant(value: &str, ident: &str, is_default: bool) -> String {
    let default = if is_default { "\t#[default]\n" } else { "" };

    // `sqlx` and `serde` both derive the wire value from the identifier, so a converted variant
    // needs telling or it round-trips as the wrong label.
    if ident == value {
        return format!("{default}\t{ident},\n");
    }
    format!(
        "{default}\t#[sqlx(rename = \"{value}\")]\n\t#[serde(rename = \"{value}\")]\n\t{ident},\n"
    )
}
