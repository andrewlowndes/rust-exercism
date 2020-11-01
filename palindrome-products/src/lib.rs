#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    total: u64,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome { total: a * b }
    }

    pub fn value(&self) -> u64 {
        self.total
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.total = a * b;
    }

    pub fn product(total: u64) -> Self {
        Palindrome { total }
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_value = None;
    let mut max_value = None;

    (min..=max).for_each(|i| {
        (i..=max).for_each(|j| {
            let product = i * j;
            let product_str = product.to_string();
            let reverse_str = product_str.chars().rev().collect::<String>();

            if product_str == reverse_str {
                if min_value.is_none() || product < min_value.unwrap() {
                    min_value = Some(product);
                }

                if max_value.is_none() || product > max_value.unwrap() {
                    max_value = Some(product);
                }
            }
        });
    });

    Some((
        Palindrome::product(min_value?),
        Palindrome::product(max_value?),
    ))
}
