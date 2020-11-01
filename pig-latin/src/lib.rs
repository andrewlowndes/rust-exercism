#![feature(option_result_contains)]

const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];

const SUFFIX: &str = "ay";

pub fn translate(input: &str) -> String {
    input
        .split_ascii_whitespace()
        .map(|word| {
            if word.is_empty() {
                return "".to_string();
            }

            //rule 1
            if VOWELS.contains(&word.chars().next().unwrap())
                || word.starts_with("xr")
                || word.starts_with("yt")
            {
                return word.to_string() + SUFFIX;
            }

            let consonant_cluster = word
                .chars()
                .take_while(|char| !VOWELS.contains(char))
                .collect::<String>();
            let remaining_word = word[consonant_cluster.len() as usize..].to_string();

            //rule 3
            if remaining_word.starts_with('u') && consonant_cluster.ends_with('q') {
                return remaining_word[1..].to_string() + &consonant_cluster + "u" + SUFFIX;
            }

            //rule 4
            let y_pos = consonant_cluster.find('y');
            if y_pos.map(|item| item > 0).contains(&true) {
                let parts = consonant_cluster.split_at(y_pos.unwrap());
                return parts.1.to_string() + &remaining_word + parts.0 + SUFFIX;
            }

            //rule 2
            if !consonant_cluster.is_empty() {
                return remaining_word + &consonant_cluster + SUFFIX;
            }

            word.to_string()
        })
        .collect::<Vec<_>>()
        .join(" ")
}
