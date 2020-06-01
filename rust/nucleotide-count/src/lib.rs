use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut sum = 0;
    if !"ACGT".contains(nucleotide) {
        return Err(nucleotide);
    }
    for c in dna.chars() {
        if !"ACGT".contains(c) {
            return Err(c);
        } else if c == nucleotide {
            sum += 1;
        }
    }
    Ok(sum)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    counts.insert('A', 0);
    counts.insert('C', 0);
    counts.insert('G', 0);
    counts.insert('T', 0);
    for c in dna.chars() {
        if let Some(count) = counts.get_mut(&c) {
            *count += 1;
        } else {
            return Err(c);
        }
    }
    Ok(counts)
}
