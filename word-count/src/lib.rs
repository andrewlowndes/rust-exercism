use std::collections::HashMap;

const CONTRACTION_CHAR: char = '\'';

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut results = HashMap::new();

    words
        .to_ascii_lowercase()
        .split(|char: char| !(char.is_ascii_alphanumeric() || char == CONTRACTION_CHAR))
        .map(|word| word.trim_matches(CONTRACTION_CHAR).to_string())
        .filter(|word| !word.is_empty())
        .for_each(|word| {
            let new_count = results.get(&word).unwrap_or(&0) + 1;
            results.insert(word, new_count);
        });

    results
}
