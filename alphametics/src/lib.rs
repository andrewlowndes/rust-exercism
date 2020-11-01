use std::collections::{HashMap, HashSet};
use std::iter;

fn factorial(num: u32) -> u32 {
    match num {
        1 => 1,
        0 => 1,
        _ => (2..=num).product(),
    }
}

fn heap_permutations(possible_values: &[u8], count: usize) -> impl Iterator<Item = Vec<u8>> {
    let num_possibles = possible_values.len() as u32;
    let mut values = possible_values.to_vec();
    let mut indexes = vec![0; num_possibles as usize];

    let num_results = factorial(num_possibles);

    let mut i = 0 as usize;

    iter::once(values[..count].to_vec()).chain((1..num_results).map(move |_| {
        while indexes[i] >= i {
            indexes[i] = 0;
            i += 1;
        }

        if i % 2 == 0 {
            values.swap(0, i);
        } else {
            values.swap(indexes[i], i);
        }

        indexes[i] += 1;
        i = 0;

        values[..count].to_vec()
    }))
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let unique_chars = input
        .chars()
        .filter(|input_char| input_char.is_ascii_alphabetic())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    if unique_chars.len() > 10 {
        return None;
    }

    let mut chars_map: HashMap<char, u8> = HashMap::new();
    unique_chars.iter().enumerate().for_each(|(key, val)| {
        chars_map.insert(*val, key as u8);
    });

    let mut number_groups = input
        .split(|input_char: char| !input_char.is_ascii_alphabetic())
        .filter(|word| word != &"")
        .map(|word| word.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let non_zero_indexes = number_groups
        .iter()
        .filter(|group| group.len() > 1)
        .map(|group| chars_map.get(group.first().unwrap()).unwrap())
        .collect::<HashSet<_>>();

    let equal_chars = number_groups.pop()?;

    let solution = heap_permutations(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], unique_chars.len()).find(
        |combination| {
            if combination
                .iter()
                .enumerate()
                .any(|(index, value)| *value == 0 && non_zero_indexes.contains(&(index as u8)))
            {
                return false;
            }

            let chars_to_int = |chars: &Vec<char>| {
                chars
                    .iter()
                    .map(|value| combination[*chars_map.get(value).unwrap() as usize].to_string())
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap()
            };

            let total = number_groups.iter().map(chars_to_int).sum::<u64>();

            let expected_total = chars_to_int(&equal_chars);

            total == expected_total
        },
    )?;

    Some(
        unique_chars
            .into_iter()
            .zip(solution.into_iter())
            .collect::<HashMap<char, u8>>(),
    )
}
