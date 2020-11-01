pub fn number(user_number: &str) -> Option<String> {
    let clean_str = user_number.trim_start_matches('+').trim_start_matches('1');
    let result = clean_str
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<Vec<_>>();

    if result.len() == 10
        && result[0].to_digit(10).unwrap() > 1
        && result[3].to_digit(10).unwrap() > 1
    {
        Some(result.iter().collect::<String>())
    } else {
        None
    }
}
