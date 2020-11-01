#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

const VALID_DNA: &[char] = &['A', 'C', 'G', 'T'];
const VALID_RNA: &[char] = &['A', 'C', 'G', 'U'];

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        if let Some(invalid_char_index) = dna.chars().position(|char| !VALID_DNA.contains(&char)) {
            Err(invalid_char_index)
        } else {
            Ok(DNA(dna.to_string()))
        }
    }

    pub fn into_rna(self) -> RNA {
        let rna_str: String = self
            .0
            .chars()
            .map(|char| match &char {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => ' ',
            })
            .collect();

        RNA(rna_str)
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        if let Some(invalid_char_index) = rna.chars().position(|char| !VALID_RNA.contains(&char)) {
            Err(invalid_char_index)
        } else {
            Ok(RNA(rna.to_string()))
        }
    }
}
