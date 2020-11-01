#![feature(fn_traits)]

use std::fmt::Display;
use std::ops::Rem;

pub struct Matcher<T> {
    matcher: Box<dyn Fn(T) -> bool>,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new(_matcher: impl Fn(T) -> bool + 'static, _subs: &str) -> Matcher<T> {
        Matcher {
            matcher: Box::new(_matcher),
            subs: _subs.to_string(),
        }
    }
}

pub struct Fizzy<T: Clone + Display + 'static> {
    matchers: Vec<Matcher<T>>,
}

impl<T: Clone + Display + 'static> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy { matchers: vec![] }
    }

    pub fn add_matcher(mut self, _matcher: Matcher<T>) -> Self {
        self.matchers.push(_matcher);
        self
    }

    pub fn apply(self, _iter: impl Iterator<Item = T>) -> impl Iterator<Item = String> {
        _iter.map(move |item| {
            let matches = self
                .matchers
                .iter()
                .filter(|matcher| matcher.matcher.call((item.clone(),)))
                .collect::<Vec<_>>();

            if matches.is_empty() {
                item.to_string()
            } else {
                matches
                    .iter()
                    .map(|matcher| matcher.subs.to_string())
                    .collect::<String>()
            }
        })
    }
}

impl<T: Clone + Display + 'static> Default for Fizzy<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub fn fizz_buzz<T: Rem<T, Output = T> + From<u8> + PartialEq + Clone + Display>() -> Fizzy<T> {
    Fizzy::new()
        .add_matcher(Matcher::new(|i: T| i % T::from(3) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|i: T| i % T::from(5) == T::from(0), "buzz"))
}
