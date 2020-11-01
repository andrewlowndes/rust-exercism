#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    for num in number {
        if *num >= from_base {
            return Err(Error::InvalidDigit(*num));
        }
    }

    let total = number
        .iter()
        .rev()
        .enumerate()
        .map(|(index, val)| from_base.pow(index as u32) * val)
        .sum::<u32>();

    let mut log = (total as f64).log(to_base as f64);

    let mut size = log.floor() as usize;
    let mut amount = to_base.pow(log as u32);
    let mut remainder = total % amount;

    let mut result = vec![0; size + 1];
    result[size] = total / amount;

    while remainder > 0 {
        log = (remainder as f64).log(to_base as f64);
        size = log.floor() as usize;
        amount = to_base.pow(log as u32);
        result[size] = remainder / amount;
        remainder %= amount;
    }

    result.reverse();

    Ok(result)
}
