#![feature(option_result_contains)]

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut expected = vec![];

    for char in string.chars() {
        match &char {
            '{' => expected.push('}'),
            '[' => expected.push(']'),
            '(' => expected.push(')'),
            '}' | ']' | ')' => {
                if !expected.pop().contains(&char) {
                    return false;
                }
            }
            _ => {}
        }
    }

    expected.is_empty()
}
