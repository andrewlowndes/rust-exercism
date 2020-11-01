use std::cmp::Ordering;

pub fn find<T: PartialEq + Ord + Sized, C: AsRef<[T]>>(arr: C, key: T) -> Option<usize> {
    let array = arr.as_ref();
    let mut num_items = array.len();
    let mut mid = num_items / 2;
    let mut remaining_items = array;
    let mut start = 0;

    while num_items > 1 {
        match remaining_items[mid].cmp(&key) {
            Ordering::Equal => return Some(start + mid),
            Ordering::Less => {
                remaining_items = remaining_items.split_at(mid).1;
                num_items -= mid;
                start += mid;
            }
            Ordering::Greater => {
                remaining_items = remaining_items.split_at(mid).0;
                num_items = mid;
            }
        }

        mid = num_items / 2;
    }

    if remaining_items.is_empty() || remaining_items[mid] != key {
        return None;
    }

    Some(start + mid)
}
