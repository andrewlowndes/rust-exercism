#[macro_use]
extern crate lazy_static;

use rand::random;
use std::char;
use std::collections::HashSet;
use std::sync::RwLock;

pub struct Robot {
    name: String,
}

lazy_static! {
    static ref EXISTING_NAMES: RwLock<HashSet<String>> = RwLock::new(HashSet::new());
}

fn random_uppercase() -> char {
    let char_code = (random::<f64>() * 25.0) as u32 + 65; // 65-90 A-Z range
    char::from_u32(char_code).unwrap()
}

fn random_digit() -> char {
    let char_code = (random::<f64>() * 9.0) as u32 + 48; // 48-57 0-9 range
    char::from_u32(char_code).unwrap()
}

fn generate_name() -> String {
    (1..=2).map(|_| random_uppercase()).collect::<String>()
        + &(1..=3).map(|_| random_digit()).collect::<String>()
}

fn unique_name() -> String {
    let mut new_name: String;

    loop {
        new_name = generate_name();

        if !EXISTING_NAMES.read().unwrap().contains(&new_name) {
            EXISTING_NAMES.write().unwrap().insert(new_name.clone());
            return new_name;
        }
    }
}

impl Default for Robot {
    fn default() -> Self {
        Self::new()
    }
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: unique_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = unique_name();
    }
}
