const ASCII_A_START: u32 = 97;
const ASCII_NUM_CHARS: u32 = 26;
const ENCODE_CHUNK_SIZE: usize = 5;

fn swap_char(char: char) -> char {
    (ASCII_A_START + (ASCII_NUM_CHARS - 1 - (char as u32 - ASCII_A_START))) as u8 as char
}

pub fn encode(plain: &str) -> String {
    plain
        .to_ascii_lowercase()
        .chars()
        .filter(|char| char.is_ascii_alphanumeric())
        .map(|char| match char {
            'a'..='z' => swap_char(char),
            _ => char,
        })
        .collect::<Vec<_>>()
        .chunks(ENCODE_CHUNK_SIZE)
        .map(|chars| chars.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|char| char.is_ascii_alphanumeric())
        .map(|char| match char {
            'a'..='z' => swap_char(char),
            _ => char,
        })
        .collect::<String>()
}
