use std::fmt::{Display, Formatter, Result};

const RANGES: &[&str] = &["I", "V", "X", "L", "C", "D", "M", "", ""];

const CHARS_LAYOUT: &[&[u8]] = &[
    &[0],
    &[0, 0],
    &[0, 0, 0],
    &[0, 1],
    &[1],
    &[1, 0],
    &[1, 0, 0],
    &[1, 0, 0, 0],
    &[0, 2],
];

pub struct Roman(u32);

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        let str = self.0.to_string();
        let scale = str.len();

        let result = str
            .chars()
            .map(|char| char.to_digit(10).unwrap())
            .enumerate()
            .filter(|(_, num)| *num != 0)
            .fold("".to_string(), |acc, (index, num)| {
                let range_start = (scale - 1 - index as usize) * 2;
                let range = &RANGES[range_start..=range_start + 2];

                acc + &CHARS_LAYOUT[num as usize - 1]
                    .iter()
                    .fold("".to_string(), |acc, range_index| {
                        acc + range[*range_index as usize]
                    })
            });

        write!(_f, "{}", result)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman(num)
    }
}
