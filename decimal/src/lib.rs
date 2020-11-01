#![feature(option_result_contains)]

use std::cmp::{Ord, Ordering};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Decimal {
    is_negative: bool,
    whole: Vec<u8>,
    decimal: Vec<u8>,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let parts = input.split('.').take(2).collect::<Vec<_>>();

        let whole = parts[0]
            .to_string()
            .trim_start_matches(|char| ['0', '+', '-'].contains(&char))
            .chars()
            .map(|char| char.to_digit(10).map(|num| num as u8))
            .collect::<Option<Vec<u8>>>()?;

        let decimal = parts
            .get(1)
            .unwrap_or(&"0")
            .to_string()
            .trim_end_matches('0')
            .chars()
            .map(|char| char.to_digit(10).map(|num| num as u8))
            .collect::<Option<Vec<u8>>>()?;

        let is_negative = parts[0].chars().next().contains(&'-');

        Some(Decimal {
            is_negative,
            whole,
            decimal,
        })
    }

    pub fn get_equal_parts(&self, other: &Self) -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
        let mut self_decimal = self.decimal.to_vec();
        let mut other_decimal = other.decimal.to_vec();

        if self.decimal.len() > other.decimal.len() {
            other_decimal.resize_with(self.decimal.len(), Default::default);
        } else {
            self_decimal.resize_with(other.decimal.len(), Default::default);
        }

        let mut other_whole = other.whole.iter().cloned().rev().collect::<Vec<u8>>();
        let mut self_whole = self.whole.iter().cloned().rev().collect::<Vec<u8>>();

        if self.whole.len() > other.whole.len() {
            other_whole.resize_with(self.whole.len(), Default::default);
        } else {
            self_whole.resize_with(other.whole.len(), Default::default);
        }

        (self_whole, self_decimal, other_whole, other_decimal)
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_negative && !other.is_negative {
            return Ordering::Less;
        } else if !self.is_negative && other.is_negative {
            return Ordering::Greater;
        }

        let cmp = self
            .whole
            .cmp(&other.whole)
            .then_with(|| self.decimal.cmp(&other.decimal));

        if self.is_negative {
            cmp.reverse()
        } else {
            cmp
        }
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.is_negative && !other.is_negative {
            return other
                - Decimal {
                    is_negative: false,
                    ..self
                };
        } else if !self.is_negative && other.is_negative {
            return self
                - Decimal {
                    is_negative: false,
                    ..other
                };
        }

        let mut carry = 0;

        let (self_whole, self_decimal, other_whole, other_decimal) = self.get_equal_parts(&other);

        let mut decimal = self_decimal
            .iter()
            .zip(other_decimal.iter())
            .rev()
            .map(|(num, other_num)| {
                let result = num + other_num + carry;
                carry = if result > 9 { 1 } else { 0 };
                result.rem_euclid(10)
            })
            .skip_while(|item| *item == 0 as u8)
            .collect::<Vec<_>>();

        decimal.reverse();

        let mut whole = self_whole
            .iter()
            .zip(other_whole.iter())
            .map(|(num, other_num)| {
                let result = num + other_num + carry;
                carry = if result > 9 { 1 } else { 0 };
                result.rem_euclid(10)
            })
            .collect::<Vec<_>>();

        if carry > 0 {
            whole.push(carry);
        }

        whole = whole
            .into_iter()
            .rev()
            .skip_while(|item| *item == 0)
            .collect::<Vec<_>>();

        Decimal {
            is_negative: self.is_negative && other.is_negative,
            whole,
            decimal,
        }
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if other.is_negative {
            let positive_other = Decimal {
                is_negative: false,
                ..other
            };

            if self.is_negative {
                return positive_other
                    - Decimal {
                        is_negative: false,
                        ..self
                    };
            }

            return self + positive_other;
        } else if self.is_negative {
            return self
                + Decimal {
                    is_negative: true,
                    ..other
                };
        } else if other > self {
            return Decimal {
                is_negative: true,
                ..(other - self)
            };
        }

        let (self_whole, self_decimal, other_whole, other_decimal) = self.get_equal_parts(&other);

        let mut carry: u8 = 0;

        let mut decimal = self_decimal
            .iter()
            .zip(other_decimal.iter())
            .rev()
            .map(|(num, other_num)| {
                let result = *num as i8 - *other_num as i8 - carry as i8;
                carry = if result < 0 { 1 } else { 0 };
                result.rem_euclid(10) as u8
            })
            .skip_while(|item| *item == 0 as u8)
            .collect::<Vec<_>>();

        decimal.reverse();

        let mut whole = self_whole
            .iter()
            .zip(other_whole.iter())
            .map(|(num, other_num)| {
                let result = *num as i8 - *other_num as i8 - carry as i8;
                carry = if result < 0 { 1 } else { 0 };
                result.rem_euclid(10) as u8
            })
            .collect::<Vec<_>>();

        if carry > 0 {
            whole.push(carry as u8);
        }

        whole = whole
            .into_iter()
            .rev()
            .skip_while(|item| *item == 0)
            .collect::<Vec<_>>();

        Decimal {
            is_negative: self.is_negative,
            whole,
            decimal,
        }
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut full_self = self.whole.clone();
        full_self.append(&mut self.decimal.clone());
        full_self.reverse();

        let mut full_other = other.whole.clone();
        full_other.append(&mut other.decimal.clone());
        full_other.reverse();

        if full_self.len() > full_other.len() {
            full_other.resize_with(full_self.len(), Default::default);
        } else {
            full_self.resize_with(full_other.len(), Default::default);
        }

        let mut result: Vec<u8> = vec![0; full_self.len() + full_other.len()];

        full_other
            .iter()
            .enumerate()
            .for_each(|(index, other_val)| {
                let mut carry = 0;
                let mut i = index;

                full_self.iter().for_each(|self_val| {
                    let total = other_val * self_val + carry;
                    carry = total / 10;
                    result[i] += total.rem_euclid(10);
                    i += 1;
                });

                while carry > 0 {
                    result[i] += carry.rem_euclid(10);
                    carry /= 10;
                    i += 1;
                }
            });

        result = result
            .into_iter()
            .rev()
            .skip_while(|i| *i == 0)
            .collect::<Vec<_>>();

        let (whole, decimal) =
            result.split_at(result.len() - (self.decimal.len() + other.decimal.len()));

        let mut decimal_trimmed = decimal
            .iter()
            .rev()
            .cloned()
            .skip_while(|item| *item == 0)
            .collect::<Vec<_>>();

        decimal_trimmed.reverse();

        Decimal {
            is_negative: self.is_negative ^ other.is_negative,
            whole: whole.to_vec(),
            decimal: decimal_trimmed,
        }
    }
}
