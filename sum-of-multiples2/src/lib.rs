use itertools::Itertools;

fn triangle_number(n: u32) -> u32 {
    (n * n + n) / 2
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if limit < 2 {
        return 0;
    }

    let divisor = limit - 1;

    let sum_of_factors: u32 = factors.iter()
        .map(|factor| triangle_number(&divisor / factor) * factor)
        .sum();
    
    let duplicates: u32 = (2..=factors.len())
        .flat_map(|i| {
            factors.iter()
                .combinations(i)
                .map(|combination| combination.iter().fold(1, |acc, &x| acc * x))
                .filter(|item| item < &limit)
        })
        .sum();

    sum_of_factors - duplicates
}
