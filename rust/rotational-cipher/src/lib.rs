pub fn rotate(input: &str, key: i8) -> String {
    input.chars()
        .map(|c|
            match c.is_ascii_uppercase() {
                true => convert(c.to_ascii_lowercase(), key).to_ascii_uppercase(),
                false => convert(c, key),
            })
        .collect()
}

fn convert(c: char, key: i8) -> char {
    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars()
        .collect();
    match letters.contains(&c) {
        true => {
            let mut i: i64 = letters.iter()
                .position(|&l| l == c)
                .unwrap() as i64;
            i = (((i + key as i64) % 26) + 26) % 26;
            letters[i as usize]
        }
        false => c
    }
}
