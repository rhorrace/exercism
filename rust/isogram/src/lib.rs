use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut visited: HashSet<char> = HashSet::new();
    candidate.to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .all(|c|
            match visited.contains(&c) {
                true => false,
                false => {
                    visited.insert(c);
                    true
                }
            })
}
