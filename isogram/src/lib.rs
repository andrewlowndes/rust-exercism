use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let chars = candidate
        .chars()
        .filter(char::is_ascii_alphabetic)
        .map(|char| char.to_ascii_lowercase())
        .collect::<Vec<_>>();

    chars.iter().collect::<HashSet<_>>().len() == chars.len()
}
