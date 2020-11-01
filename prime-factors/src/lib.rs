pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = vec![];

    let mut result = n;

    while (result as f64) % 2.0 == 0.0 {
        result /= 2;
        factors.push(2);
    }

    for i in 3..((result as f64).sqrt() as u64 + 1) {
        while (result as f64) % (i as f64) == 0.0 {
            result /= i;
            factors.push(i);
        }
    }

    if result > 2 {
        factors.push(result);
    }

    factors
}
