#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }

    let mut nums = vec![];

    for char in string_digits.chars() {
        if let Some(num) = char.to_digit(10) {
            nums.push(num as u64);
        } else {
            return Err(Error::InvalidDigit(char));
        }
    }

    if string_digits.is_empty() || span == 0 {
        return Ok(1);
    }

    Ok(nums
        .windows(span)
        .map(|nums| nums.iter().product())
        .max()
        .unwrap())
}
