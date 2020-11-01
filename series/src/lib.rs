pub fn series(digits: &str, len: usize) -> Vec<String> {
    let num_digits = digits.chars().count();

    if len == 0 {
        return vec!["".to_string(); num_digits + 1];
    } else if len > num_digits {
        return vec![];
    }

    digits
        .chars()
        .collect::<Vec<_>>()
        .windows(len)
        .map(|chars| chars.iter().collect::<String>())
        .collect::<Vec<_>>()
}
