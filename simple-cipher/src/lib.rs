use rand::random;

const ASCII_RANGE: u32 = 26;
const ASCII_START: u32 = 97;

fn valid_key(key: &str) -> bool {
    key.len() > 0 && key.chars().all(|char| char.is_ascii_lowercase())
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    if !valid_key(key) {
        return None;
    }

    let key_chars = key.chars().collect::<Vec<_>>();

    Some(
        s.to_ascii_lowercase()
            .chars()
            .enumerate()
            .map(|(char_index, raw_char)| {
                let offset = key_chars[char_index % key_chars.len()] as u32 - ASCII_START;
                ((raw_char as u32 - ASCII_START + offset).rem_euclid(ASCII_RANGE) + ASCII_START)
                    as u8 as char
            })
            .collect::<String>(),
    )
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if !valid_key(key) {
        return None;
    }

    let key_chars = key.chars().collect::<Vec<_>>();

    Some(
        s.to_ascii_lowercase()
            .chars()
            .enumerate()
            .map(|(char_index, raw_char)| {
                let offset = key_chars[char_index % key_chars.len()] as u32 - ASCII_START;
                ((raw_char as i32 - ASCII_START as i32 - offset as i32)
                    .rem_euclid(ASCII_RANGE as i32) as u32
                    + ASCII_START) as u8 as char
            })
            .collect::<String>(),
    )
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = (0..s.len().max(100))
        .map(|_| ((random::<f32>() * ASCII_RANGE as f32) as u32 + ASCII_START) as u8 as char)
        .collect::<String>();
    let result = encode(&key, s).unwrap();
    (key, result)
}
