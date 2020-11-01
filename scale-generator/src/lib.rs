const SCALE_SHARPS: &[&str] = &[
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];
const SCALE_FLATS: &[&str] = &[
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

const TONIC_FLATS: &[&str] = &[
    "Bb", "Db", "Eb", "F", "Gb", "Ab", "d", "g", "c", "f", "bb", "eb",
];

const DEFAULT_STEPS: &[u8] = &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

#[derive(Debug)]
pub enum Error {
    InvalidInterval(char),
    InvalidTonic,
}

pub struct Scale {
    use_flats: bool,
    start: u8,
    steps: Vec<u8>,
}

//from https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
fn title_case(str: &str) -> String {
    let mut char = str.chars();
    match char.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + char.as_str(),
    }
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let mut scale = Self::chromatic(tonic)?;

        scale.steps = intervals
            .chars()
            .map(|char| match char {
                'm' => Ok(1),
                'M' => Ok(2),
                'A' => Ok(3),
                _ => Err(Error::InvalidInterval(char)),
            })
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(scale)
    }

    fn get_scale(use_flats: bool) -> &'static [&'static str] {
        if use_flats {
            SCALE_FLATS
        } else {
            SCALE_SHARPS
        }
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        let tonic_str = title_case(tonic);
        let use_flats = TONIC_FLATS.contains(&tonic);
        let start = Self::get_scale(use_flats)
            .iter()
            .position(|item| item == &tonic_str)
            .ok_or(Error::InvalidTonic)?;

        Ok(Scale {
            use_flats,
            start: start as u8,
            steps: DEFAULT_STEPS.to_vec(),
        })
    }

    pub fn enumerate(&self) -> Vec<String> {
        let scale = Self::get_scale(self.use_flats);
        let mut index = self.start as usize;
        let mut result = vec![];

        for step in &self.steps {
            result.push(scale[index].to_string());
            index = (index + *step as usize) % scale.len();
        }

        result
    }
}
