#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

fn get_factors(num: u64) -> Vec<u64> {
    (2..num).filter(|i| num % i == 0).collect()
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let factors = get_factors(num);
    let total = factors.iter().sum::<u64>() + 1;

    let classification = {
        if factors.len() == 0 || total < num {
            Classification::Deficient
        } else if total == num {
            Classification::Perfect
        } else {
            Classification::Abundant
        }
    };

    Some(classification)
}
