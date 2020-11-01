pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut i = n;
    let mut steps = 0;

    while i != 1 {
        if i % 2 == 0 {
            i = i / 2;
        } else {
            i = i * 3 + 1;
        }

        steps += 1;
    }

    Some(steps)
}
