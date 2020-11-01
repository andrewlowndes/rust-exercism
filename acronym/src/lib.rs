pub fn abbreviate(phrase: &str) -> String {
    let mut last_char = ' ';

    phrase
        .chars()
        .filter(|char| {
            let is_acronym_char = char.is_alphabetic()
                && (last_char.is_whitespace()
                    || (last_char.is_ascii_punctuation() && last_char != '\''))
                || char.is_uppercase() && last_char.is_lowercase();

            last_char = *char;
            is_acronym_char
        })
        .map(|char| char.to_ascii_uppercase())
        .collect()
}
