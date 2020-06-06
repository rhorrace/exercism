/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain.to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(convert)
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter()
            .collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(convert)
        .collect::<String>()
}

fn convert(c: char) -> char {
    match c {
        'a' => 'z',
        'b' => 'y',
        'c' => 'x',
        'd' => 'w',
        'e' => 'v',
        'f' => 'u',
        'g' => 't',
        'h' => 's',
        'i' => 'r',
        'j' => 'q',
        'k' => 'p',
        'l' => 'o',
        'm' => 'n',
        'n' => 'm',
        'o' => 'l',
        'p' => 'k',
        'q' => 'j',
        'r' => 'i',
        's' => 'h',
        't' => 'g',
        'u' => 'f',
        'v' => 'e',
        'w' => 'd',
        'x' => 'c',
        'y' => 'b',
        'z' => 'a',
        _ => c,
    }
}
