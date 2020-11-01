#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64)
    }
}

const ONE_YEAR_EARTH_SECONDS: f64 = 31557600f64;

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! planets {
    ($([$name: ident, $earth_years: expr]),*) => {
        $(
            pub struct $name;

            impl Planet for $name {
                fn years_during(d: &Duration) -> f64 {
                    d.0 / ($earth_years * ONE_YEAR_EARTH_SECONDS)
                }
            }
        )*
    };
}

planets!(
    [Mercury, 0.2408467f64],
    [Earth, 1.0f64],
    [Venus, 0.61519726f64],
    [Mars, 1.8808158f64],
    [Jupiter, 11.862615f64],
    [Saturn, 29.447498f64],
    [Uranus, 84.016846f64],
    [Neptune, 164.79132f64]
);
