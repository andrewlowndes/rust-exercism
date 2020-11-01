#![feature(array_map)]

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

const SCORES: &[(u8, &[char])] = &[
    (1, &['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']),
    (2, &['D', 'G']),
    (3, &['B', 'C', 'M', 'P']),
    (4, &['F', 'H', 'V', 'W', 'Y']),
    (5, &['K']),
    (8, &['J', 'X']),
    (10, &['Q', 'Z']),
];

lazy_static! {
    static ref SCORE_BY_CHAR: HashMap<char, u64> = {
        let mut m = HashMap::new();

        SCORES.iter().for_each(|(score, chars)| {
            chars.iter().for_each(|c| {
                m.insert(*c, *score as u64);
            });
        });

        m
    };
}

pub fn score(word: &str) -> u64 {
    word.to_ascii_uppercase()
        .chars()
        .map(|c| *SCORE_BY_CHAR.get(&c).or(Some(&0)).unwrap())
        .sum()
}
