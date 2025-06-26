const SEPARATORS: &[char] = &[' ', '\t', '\n'];
const DELIMITER: char = '\n';

pub fn split(target: &str) -> Vec<&str> {
    target
        .split(SEPARATORS)
        .filter(|e| !e.trim().is_empty())
        .collect()
}

pub fn is_sep(c: char) -> bool {
    SEPARATORS.contains(&c)
}

pub fn is_n(c: char) -> bool {
    c == DELIMITER
}
