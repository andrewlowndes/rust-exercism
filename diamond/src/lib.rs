const ASCII_UPPERCASE_START: u8 = b'A';

pub fn get_diamond(c: char) -> Vec<String> {
    let num = c as u8 - ASCII_UPPERCASE_START;
    let padding = vec![' '; num as usize].iter().collect::<String>();

    let mut result = vec![String::new() + &padding + "A" + &padding];

    (1..=num).for_each(|offset| {
        let mut part = padding.chars().collect::<Vec<_>>();
        part[(num - offset) as usize] = (ASCII_UPPERCASE_START + offset) as char;
        result.push(part.iter().collect::<String>() + " " + &part.iter().rev().collect::<String>());
    });

    result
        .iter()
        .chain(result.iter().rev().skip(1))
        .cloned()
        .collect::<Vec<_>>()
}
