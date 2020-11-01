use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T: PartialEq>(first_len: usize, first_list: &[T], second_list: &[T]) -> bool {
    first_len == 0
        || second_list
            .windows(first_len)
            .any(|sublist| sublist == first_list)
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let first_len = first_list.len();
    let second_len = second_list.len();

    match first_len.cmp(&second_len) {
        Ordering::Equal => {
            if first_list == second_list {
                return Comparison::Equal;
            }
        }
        Ordering::Less => {
            if is_sublist(first_len, first_list, second_list) {
                return Comparison::Sublist;
            }
        }
        Ordering::Greater => {
            if is_sublist(second_len, second_list, first_list) {
                return Comparison::Superlist;
            }
        }
    }

    Comparison::Unequal
}
