pub fn is_valid_isbn(isbn: &str) -> bool {
    let chars = isbn.replace("-", "").chars().collect::<Vec<_>>();

    if chars.len() != 10 {
        return false;
    }

    let mut product = 0;

    for (i, substr) in chars.iter().rev().enumerate() {
        let num = {
            if i == 0 && *substr == 'X' {
                10
            } else {
                let parsed = substr.to_digit(10);

                if parsed.is_none() {
                    return false;
                }

                parsed.unwrap()
            }
        };

        product += (i as u32 + 1) * num;
    }

    product % 11 == 0
}
