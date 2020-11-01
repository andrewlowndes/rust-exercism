use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut results = HashSet::new();

    (3..sum - 2).for_each(|c| {
        let c_total = c * c;

        (2..(sum - c)).for_each(|b| {
            let a = sum - (c + b);

            if a > 0 && a < b && (a * a + b * b == c_total) {
                results.insert([a, b, c]);
            }
        });
    });

    results
}
