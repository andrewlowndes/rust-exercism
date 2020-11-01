use luhn::is_valid;

pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        is_valid(self.0.as_str())
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn(input.to_string())
    }
}
