use rand::prelude::*;

fn mod_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;

    for _ in 0..exponent {
        result = (result * base) % modulus;
    }

    result
}

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_pow(b_pub, a, p)
}
