pub fn abbreviate(phrase: &str) -> String {
    phrase.split(|c: char| !c.is_alphabetic() && c != '\'')
        .flat_map(|w| {
            if w.len() <= 4 {
                return w.chars()
                    .filter(|c| !c.is_ascii_punctuation())
                    .take(1)
                    .collect::<Vec<char>>();
            }
            let mut chars: Vec<char> = Vec::new();
            for (i, char) in w.chars()
                .enumerate() {
                if i == 0 || char.is_uppercase() {
                    chars.push(char)
                }
            }
            chars
        })
        .collect::<String>()
        .to_uppercase()
}
