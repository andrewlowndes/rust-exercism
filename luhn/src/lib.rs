/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code
        .chars()
        .any(|char| !(char.is_ascii_whitespace() || char.is_ascii_digit()))
        || code.trim() == "0"
    {
        return false;
    }

    code.chars()
        .rev()
        .filter(|char| char.is_ascii_digit())
        .enumerate()
        .map(|(index, char)| {
            let num = char.to_string().parse::<u32>().unwrap();
            let mut total = num;

            if index % 2 == 1 {
                total *= 2;

                if total > 9 {
                    total -= 9;
                }
            }

            total
        })
        .sum::<u32>()
        % 10
        == 0
}
