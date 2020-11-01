pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let max = upper_bound as usize;
    let mut possible_primes = vec![true; max];
    let mut actual_primes = vec![];

    for n in 2..=max {
        if possible_primes[n - 1] {
            actual_primes.push(n as u64);
            (n..=max)
                .step_by(n)
                .for_each(|prime| possible_primes[prime - 1] = false);
        }
    }

    actual_primes
}
