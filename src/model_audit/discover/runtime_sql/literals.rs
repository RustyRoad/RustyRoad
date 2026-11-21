//! Skip Rust string/character bodies, including raw strings with arbitrary hashes.

/// Return a literal's end or None for code (including lifetime apostrophes).
/// Example: `end("r##\"query()\"##", 0)` consumes the entire raw string.
pub(super) fn end(source: &str, start: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut at = start;
    if matches!(bytes.get(at), Some(b'b' | b'c')) {
        at += 1;
    }
    if bytes.get(at) == Some(&b'r') {
        at += 1;
        let hashes = at;
        while bytes.get(at) == Some(&b'#') {
            at += 1;
        }
        if bytes.get(at) != Some(&b'"') {
            return None;
        }
        let suffix = format!("\"{}", "#".repeat(at - hashes));
        return Some(
            source[at + 1..]
                .find(&suffix)
                .map_or(source.len(), |end| at + 1 + end + suffix.len()),
        );
    }
    if bytes.get(at) == Some(&b'"') {
        at += 1;
        while at < bytes.len() {
            match bytes[at] {
                b'\\' => at = (at + 2).min(bytes.len()),
                b'"' => return Some(at + 1),
                _ => at += 1,
            }
        }
        return Some(source.len());
    }
    if bytes.get(at) != Some(&b'\'') {
        return None;
    }
    at += 1;
    if bytes.get(at) == Some(&b'\\') {
        // Escaped chars can contain unicode braces; a lifetime never starts '\.
        while at < bytes.len() && bytes[at] != b'\n' {
            match bytes[at] {
                b'\\' => at = (at + 2).min(bytes.len()),
                b'\'' => return Some(at + 1),
                _ => at += 1,
            }
        }
        // An escaped opening cannot be a lifetime. Keep malformed character
        // contents opaque, but resume scanning code on the next source line.
        Some(at)
    } else {
        at += source.get(at..)?.chars().next()?.len_utf8();
        (bytes.get(at) == Some(&b'\'')).then_some(at + 1)
    }
}
