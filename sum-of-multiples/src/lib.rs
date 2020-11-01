use itertools::Itertools;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if limit < 2 {
        return 0;
    }

    let denom = limit - 1;

    factors
        .iter()
        .filter(|factor| factor > &&0)
        .flat_map(|factor| (1..=(denom / factor)).map(move |item| item * factor))
        .unique()
        .sum()
}
