const ASCII_UPPER_START: i32 = 65;
const ASCII_LOWER_START: i32 = 97;
const ASCII_CHARS: i32 = 26;

pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|char| match char {
            'a'..='z' => {
                (((char as i32 - ASCII_LOWER_START) + key as i32).rem_euclid(ASCII_CHARS)
                    + ASCII_LOWER_START) as u8 as char
            }
            'A'..='Z' => {
                (((char as i32 - ASCII_UPPER_START) + key as i32).rem_euclid(ASCII_CHARS)
                    + ASCII_UPPER_START) as u8 as char
            }
            _ => char,
        })
        .collect::<String>()
}
