pub fn plural<'a>(singular: &'a str, plural: &'a str, count: u32) -> &'a str {
    match count {
        1 => singular,
        _ => plural,
    }
}

pub fn verse(n: u32) -> String {
    match n {
        0 => String::from(
            "No more bottles of beer on the wall, no more bottles of beer.\n\
                Go to the store and buy some more, 99 bottles of beer on the wall.\n",
        ),
        1 => String::from(
            "1 bottle of beer on the wall, 1 bottle of beer.\n\
                Take it down and pass it around, no more bottles of beer on the wall.\n",
        ),
        _ => {
            let n_minus_1 = n - 1;

            format!(
                "{n} bottles of beer on the wall, {n} bottles of beer.\n\
                Take one down and pass it around, {n_minus_1} {bottles} of beer on the wall.\n",
                n = n,
                n_minus_1 = n_minus_1,
                bottles = plural("bottle", "bottles", n_minus_1)
            )
        }
    }
}

pub fn sing(end: u32, start: u32) -> String {
    let mut result = vec![];

    for n in (start..=end).rev() {
        result.push(verse(n));
    }

    result.join("\n")
}
