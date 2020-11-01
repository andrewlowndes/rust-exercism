use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    aliases: HashMap<String, String>,
    values: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            values: vec![],
            aliases: HashMap::new(),
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.values.clone()
    }

    fn pop(&mut self) -> Result<i32, Error> {
        self.values.pop().ok_or(Error::StackUnderflow)
    }

    fn peek(&self) -> Result<i32, Error> {
        self.values
            .last()
            .map(|item| *item)
            .ok_or(Error::StackUnderflow)
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let clean_input = input.to_ascii_lowercase();

        let mut words = clean_input.split_ascii_whitespace();

        while let Some(word) = words.next() {
            if let Ok(parsed_number) = word.parse::<i32>() {
                self.values.push(parsed_number);
                continue;
            }

            if self.aliases.contains_key(word) {
                let alias_contents = self.aliases.get(word).unwrap().to_string();
                self.eval(&alias_contents)?;
                continue;
            }

            match word {
                ":" => {
                    let alias_name = words.next().ok_or(Error::InvalidWord)?;
                    let mut has_ending = false;
                    let alias_words = words
                        .by_ref()
                        .take_while(|item| {
                            has_ending = *item == ";";
                            !has_ending
                        })
                        .map(|item| {
                            self.aliases
                                .get(item)
                                .cloned()
                                .unwrap_or_else(|| item.to_string())
                        })
                        .collect::<Vec<_>>();

                    if alias_name.parse::<i32>().is_ok() || alias_words.is_empty() || !has_ending {
                        return Err(Error::InvalidWord);
                    }

                    self.aliases
                        .insert(alias_name.to_string(), alias_words.join(" "));
                }
                "+" => {
                    let first = self.pop()?;
                    let second = self.pop()?;
                    self.values.push(second + first)
                }
                "-" => {
                    let first = self.pop()?;
                    let second = self.pop()?;
                    self.values.push(second - first);
                }
                "*" => {
                    let first = self.pop()?;
                    let second = self.pop()?;
                    self.values.push(second * first);
                }
                "/" => {
                    let first = self.pop()?;
                    let second = self.pop()?;

                    if first == 0 {
                        return Err(Error::DivisionByZero);
                    }

                    self.values.push(second / first);
                }
                "dup" => {
                    self.values.push(self.peek()?);
                }
                "drop" => {
                    self.pop().or(Err(Error::StackUnderflow))?;
                }
                "swap" => {
                    if self.values.len() < 2 {
                        return Err(Error::StackUnderflow);
                    }

                    let last_index = self.values.len() - 1;
                    self.values.swap(last_index - 1, last_index);
                }
                "over" => {
                    if self.values.len() < 2 {
                        return Err(Error::StackUnderflow);
                    }

                    self.values.push(self.values[self.values.len() - 2]);
                }
                _ => {
                    return Err(Error::UnknownWord);
                }
            }
        }

        Ok(())
    }
}
