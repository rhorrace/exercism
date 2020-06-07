use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();

    words.replace(" '", " ")
        .replace("' ", " ")
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '\'' {
            c
        } else {
            ' '
        })
        .collect::<String>()
        .split_whitespace()
        .for_each(|word| {
            *counts.entry(word.to_lowercase()).or_insert(0) += 1;
        });

    counts
}
