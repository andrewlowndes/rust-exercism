use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();

    h.iter().for_each(|(score, letters)| {
        letters.iter().for_each(|letter| {
            result.insert(letter.to_ascii_lowercase(), *score);
        });
    });

    result
}
