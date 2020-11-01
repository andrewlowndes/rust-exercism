extern crate strum;
extern crate strum_macros;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct Allergies(u8);

#[derive(Debug, PartialEq, EnumIter)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergen {
    pub fn value(&self) -> u8 {
        match *self {
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16,
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128,
        }
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score.rem_euclid(256) as u8)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_value = allergen.value();
        self.0 & allergen_value == allergen_value
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::iter()
            .filter(|allergen| self.is_allergic_to(allergen))
            .collect()
    }
}
