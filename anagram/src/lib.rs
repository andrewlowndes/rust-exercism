use std::collections::HashSet;

fn sorted_chars(word: &str) -> String {
    let mut chars = word.chars().collect::<Vec<char>>();
    chars.sort_unstable();
    chars.iter().collect()
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let compare_word = sorted_chars(lowercase_word.as_str());

    possible_anagrams
        .to_owned()
        .into_iter()
        .filter(|item| {
            let compare_item = item.to_lowercase();
            compare_item != lowercase_word && sorted_chars(compare_item.as_str()) == compare_word
        })
        .collect()
}
