use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();
    let mut letters: Vec<char> = word.to_lowercase()
        .chars()
        .collect();
    letters.sort();
    possible_anagrams.iter()
        .for_each(|anagram| {
            let mut ana_letters: Vec<char> = anagram.to_lowercase()
                .chars()
                .collect();
            ana_letters.sort();
            if ana_letters == letters && anagram.to_lowercase() != word.to_lowercase() {
                anagrams.insert(anagram);
            }
        });
    anagrams
}
