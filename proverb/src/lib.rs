pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::new();
    let num_items = list.len();

    if num_items > 0 {
        for i in 0..(num_items - 1) {
            let item = list[i];
            let next_item = list[i + 1];

            result += &format!("For want of a {} the {} was lost.\n", item, next_item)[..];
        }

        result += &format!("And all for the want of a {}.", list[0])[..];
    }

    result
}
