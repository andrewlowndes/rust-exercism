use std::collections::HashMap;

const VALID_NUCLEOTIDE: &[char] = &['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !VALID_NUCLEOTIDE.contains(&nucleotide) {
        return Err(nucleotide);
    }

    if let Some(invalid_char) = dna.chars().find(|char| !VALID_NUCLEOTIDE.contains(char)) {
        return Err(invalid_char);
    }

    Ok(dna.chars().filter(|char| char == &nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut results = HashMap::new();

    for nucleotide in VALID_NUCLEOTIDE {
        results.insert(*nucleotide, 0);
    }

    for char in dna.chars() {
        if !VALID_NUCLEOTIDE.contains(&char) {
            return Err(char);
        }

        let old_count = results.get(&char).copied().unwrap();
        results.insert(char, old_count + 1);
    }

    Ok(results)
}
