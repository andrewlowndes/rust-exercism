#![feature(exclusive_range_pattern)]

const SINGLES: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const TEENS: &[&str] = &[
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TEN_SCALE: &[&str] = &[
    "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const HUNDRED: u64 = 100;
const THOUSAND: u64 = 1_000;
const MILLION: u64 = 1_000_000;
const BILLION: u64 = 1_000_000_000;
const TRILLION: u64 = 1_000_000_000_000;
const QUADRILLION: u64 = 1_000_000_000_000_000;
const QUINTILLION: u64 = 1_000_000_000_000_000_000;

fn large_num_to_string(n: u64, scale: u64, text: &str) -> String {
    let mut result = vec![encode(n / scale), text.to_string()];
    let remainder = n % scale;

    if remainder > 0 {
        result.push(encode(remainder));
    }

    result.join(" ")
}

pub fn encode(n: u64) -> String {
    match n {
        QUINTILLION..=u64::MAX => large_num_to_string(n, QUINTILLION, "quintillion"),
        QUADRILLION..QUINTILLION => large_num_to_string(n, QUADRILLION, "quadrillion"),
        TRILLION..QUADRILLION => large_num_to_string(n, TRILLION, "trillion"),
        BILLION..TRILLION => large_num_to_string(n, BILLION, "billion"),
        MILLION..BILLION => large_num_to_string(n, MILLION, "million"),
        THOUSAND..MILLION => large_num_to_string(n, THOUSAND, "thousand"),
        HUNDRED..THOUSAND => large_num_to_string(n, HUNDRED, "hundred"),
        20..HUNDRED => {
            let mut result = vec![TEN_SCALE[((n / 10) - 2) as usize]];
            let remainder = n % 10;

            if remainder > 0 {
                result.append(&mut vec!["-", SINGLES[remainder as usize]]);
            }

            result.join("")
        }
        10..20 => TEENS[(n - 10) as usize].to_string(),
        0..10 => SINGLES[n as usize].to_string(),
    }
}
