use super::quote::dollar_tag_len;

pub(in crate::database::migrations::sql_migration_converter) fn dollar_body(
    stmt: &str,
) -> Option<String> {
    let chars: Vec<char> = stmt.chars().collect();
    let start = (0..chars.len()).find(|&i| dollar_tag_len(&chars, i).is_some())?;
    let len = dollar_tag_len(&chars, start)?;
    let tag = &chars[start..start + len];
    let end = (start + len..=chars.len().checked_sub(len)?)
        .rev()
        .find(|&k| &chars[k..k + len] == tag)?;
    Some(chars[start + len..end].iter().collect())
}
