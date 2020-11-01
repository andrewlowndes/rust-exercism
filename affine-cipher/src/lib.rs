use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const ALPHABET_SIZE: i32 = 26;
const ENCODE_CHUNK_SIZE: usize = 5;
const ASCII_ALPHABETIC_START: i32 = 97;

fn calculate_mmi(a: i32, b: i32) -> i32 {
    //extended euclidean algorithm (https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm)
    let mut r = (a, b);
    let mut s = (1, 0);
    let mut quotient;

    while r.1 != 0 {
        quotient = r.0 / r.1;
        r = (r.1, r.0 - quotient * r.1);
        s = (s.1, s.0 - quotient * s.1);
    }

    s.0
}

fn encode_char(char: char, a: i32, b: i32) -> char {
    (((a * (char as i32 - ASCII_ALPHABETIC_START) as i32 + b).rem_euclid(ALPHABET_SIZE))
        + ASCII_ALPHABETIC_START) as u8 as char
}

fn decode_char(char: char, mmi: i32, b: i32) -> char {
    (((mmi * (char as i32 - ASCII_ALPHABETIC_START - b)).rem_euclid(ALPHABET_SIZE))
        + ASCII_ALPHABETIC_START) as u8 as char
}

fn factors(a: i32) -> HashSet<i32> {
    (2..a).filter(move |num| a % num == 0).collect()
}

fn are_coprime(a: i32, b: i32) -> bool {
    b % a != 0 && factors(a).intersection(&factors(b)).count() == 0
}

pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !are_coprime(a, ALPHABET_SIZE) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    Ok(plaintext
        .to_ascii_lowercase()
        .chars()
        .filter(|char| char::is_alphanumeric(*char))
        .map(|char| match char {
            'a'..='z' => encode_char(char, a, b),
            _ => char,
        })
        .collect::<Vec<_>>()
        .chunks(ENCODE_CHUNK_SIZE)
        .map(|chars| chars.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" "))
}

pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !are_coprime(a, ALPHABET_SIZE) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let mmi = calculate_mmi(a, ALPHABET_SIZE);

    Ok(ciphertext
        .chars()
        .filter(|char| char::is_alphanumeric(*char))
        .map(|char| match char {
            'a'..='z' => decode_char(char, mmi, b),
            _ => char,
        })
        .collect::<String>())
}
