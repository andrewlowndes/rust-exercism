pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut prime_index = 1;
    let mut primes_found = vec![3];

    let mut i = 5;
    while prime_index < n {
        if !primes_found.iter().any(|prime| i % prime == 0) {
            primes_found.push(i);
            prime_index += 1;
        }

        i += 2;
    }

    *primes_found.last().unwrap()
}
