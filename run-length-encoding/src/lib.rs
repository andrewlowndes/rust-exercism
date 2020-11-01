#![feature(split_inclusive)]

pub fn encode(source: &str) -> String {
    let mut result = String::new();

    let mut source_chars = source.chars();
    let first_char = source_chars.next();

    if first_char.is_none() {
        return "".to_string();
    }

    let mut last_char = first_char.unwrap();
    let mut num_chars = 1;

    source_chars.for_each(|sub_str| {
        if sub_str == last_char {
            num_chars += 1;
        } else {
            if num_chars > 1 {
                result += &(num_chars.to_string() + &last_char.to_string());
            } else {
                result += &last_char.to_string();
            }

            last_char = sub_str;
            num_chars = 1;
        }
    });

    match num_chars {
        1 => result + &last_char.to_string(),
        d if d > 1 => result + &(num_chars.to_string() + &last_char.to_string()),
        _ => result,
    }
}

pub fn decode(source: &str) -> String {
    source
        .split_inclusive(|sub_str: char| !sub_str.is_numeric())
        .map(|pair| {
            if pair.len() == 1 {
                return pair.to_string();
            }

            let last_index = (pair.len() - 1) as usize;
            let parts = pair.split_at(last_index);
            let num = parts.0.parse::<usize>().unwrap();

            parts.1.repeat(num)
        })
        .collect()
}
