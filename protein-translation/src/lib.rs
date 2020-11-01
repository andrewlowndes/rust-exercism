use std::collections::{HashMap, HashSet};

const STOP_CODON_NAME: &str = "stop codon";

pub struct CodonsInfo<'a> {
    stop_codons: HashSet<&'a str>,
    codon_names: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codon_names.get(codon).cloned()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.chars()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|chunk| chunk.iter().collect::<String>())
            .take_while(|chunk| !self.stop_codons.contains(chunk.as_str()))
            .map(|codon| self.codon_names.get(codon.as_str()).cloned())
            .collect::<Option<Vec<_>>>()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut codon_names = HashMap::new();
    let mut stop_codons = HashSet::new();

    for (codon, name) in pairs {
        codon_names.insert(codon, name);

        if name == STOP_CODON_NAME {
            stop_codons.insert(codon);
        }
    }

    CodonsInfo {
        codon_names,
        stop_codons,
    }
}
