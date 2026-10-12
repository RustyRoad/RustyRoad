pub(super) fn dollar_tag_len(chars: &[char], i: usize) -> Option<usize> {
    if chars.get(i) != Some(&'$') {
        return None;
    }
    let mut j = i + 1;
    if matches!(chars.get(j), Some(c) if c.is_ascii_alphabetic() || *c == '_') {
        while matches!(chars.get(j), Some(c) if c.is_ascii_alphanumeric() || *c == '_') {
            j += 1;
        }
    }
    (chars.get(j) == Some(&'$')).then(|| j - i + 1)
}

pub(super) fn skip_quoted(chars: &[char], i: usize) -> usize {
    let open = chars[i];
    if open == '$' {
        let Some(len) = dollar_tag_len(chars, i) else {
            return i + 1;
        };
        let tag = &chars[i..i + len];
        for k in i + len..=chars.len().saturating_sub(len) {
            if &chars[k..k + len] == tag {
                return k + len;
            }
        }
        return chars.len();
    }
    let mut k = i + 1;
    while k < chars.len() && chars[k] != open {
        k += 1;
    }
    (k + 1).min(chars.len())
}

pub(super) fn starts_quote(chars: &[char], i: usize) -> bool {
    chars.get(i).is_some_and(|c| matches!(*c as u32, 34 | 39))
        || (chars.get(i) == Some(&'$') && dollar_tag_len(chars, i).is_some())
}
