use super::quote::{skip_quoted, starts_quote};

pub(in crate::database::migrations::sql_migration_converter) fn split_outside(
    sql: &str,
    delim: char,
) -> Vec<String> {
    let chars: Vec<char> = sql.chars().collect();
    let (mut parts, mut cur, mut depth, mut i) = (Vec::new(), String::new(), 0i32, 0usize);
    while i < chars.len() {
        let c = chars[i];
        if starts_quote(&chars, i) {
            let end = skip_quoted(&chars, i);
            cur.extend(&chars[i..end]);
            i = end;
            continue;
        }
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
        if c == delim && depth <= 0 {
            parts.push(std::mem::take(&mut cur));
        } else {
            cur.push(c);
        }
        i += 1;
    }
    parts.push(cur);
    parts
        .into_iter()
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
        .collect()
}
