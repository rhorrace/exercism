use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    // This field is here to make the template compile and not to
    // complain about unused type lifetime parameter "'a". Once you start
    // solving the exercise, delete this field and the 'std::marker::PhantomData'
    // import.
    codons: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        match self.codons.get(codon) {
            Some(&protein) => Some(protein),
            None => None,
        }
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        if rna.len() % 3 != 0 ||
            rna.chars()
                .any(|c| !"AUGC".contains(c)) {
            None
        } else {
            Some(
                rna.chars()
                    .collect::<Vec<char>>()
                    .chunks(3)
                    .map(|codon| codon.iter()
                        .collect::<String>())
                    .map(|s| self.name_for(s.as_str()))
                    .take_while(|s| !s.unwrap()
                        .contains("stop"))
                    .map(|s| s.unwrap())
                    .collect::<Vec<&str>>()
            )
        }
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut codons: HashMap<&'a str, &'a str> = HashMap::new();
    pairs.iter()
        .for_each(|(coding, protein)| {
            codons.insert(coding, protein);
        });
    CodonsInfo {
        codons
    }
}
