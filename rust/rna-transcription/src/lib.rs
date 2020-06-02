#[derive(Debug, PartialEq)]
pub struct DNA {
    strand: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    strand: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        for (i, c) in dna.chars()
            .enumerate() {
            if !"ATCG".contains(c) {
                return Err(i);
            }
        }
        Ok(DNA {
            strand: dna.to_string()
        })
    }

    pub fn into_rna(self) -> RNA {
        let rna_strand = self.strand.chars()
            .map(|c|
                match c {
                    'A' => 'U',
                    'C' => 'G',
                    'G' => 'C',
                    'T' => 'A',
                    _   => c
                })
            .collect::<String>();
        RNA {
            strand: rna_strand,
        }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        for (i, c) in rna.chars()
            .enumerate() {
            if !"AUCG".contains(c) {
                return Err(i);
            }
        }
        Ok(RNA {
            strand: rna.to_string()
        })
    }
}
