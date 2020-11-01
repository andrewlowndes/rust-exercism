use luhn::is_valid;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        is_valid(self.to_string().as_str())
    }
}
