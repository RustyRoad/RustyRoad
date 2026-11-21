//! Ignore whitespace and nested comments while keeping original source offsets.

/// Consume one trivia run; `end("/* /* nested */ */", 0)` reaches its end.
pub(super) fn end(source: &str, start: usize) -> Option<usize> {
    let tail = &source[start..];
    if tail.chars().next()?.is_whitespace() {
        return Some(
            start
                + tail
                    .chars()
                    .take_while(|ch| ch.is_whitespace())
                    .map(char::len_utf8)
                    .sum::<usize>(),
        );
    }
    if tail.starts_with("//") {
        return Some(tail.find('\n').map_or(source.len(), |end| start + end));
    }
    if !tail.starts_with("/*") {
        return None;
    }
    let bytes = source.as_bytes();
    let mut depth = 1;
    let mut at = start + 2;
    while at + 1 < bytes.len() {
        match &bytes[at..at + 2] {
            b"/*" => {
                depth += 1;
                at += 2;
            }
            b"*/" => {
                depth -= 1;
                at += 2;
                if depth == 0 {
                    return Some(at);
                }
            }
            _ => at += 1,
        }
    }
    Some(source.len())
}
