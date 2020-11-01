use std::collections::HashSet;

const ALPHABET: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn is_pangram(sentence: &str) -> bool {
    let mut remaining_chars = ALPHABET.iter().collect::<HashSet<_>>();

    sentence
        .to_ascii_lowercase()
        .chars()
        .any(|char| remaining_chars.remove(&char) && remaining_chars.is_empty())
}
