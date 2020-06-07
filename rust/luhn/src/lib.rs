/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let digits: Vec<char> = code.chars()
        .filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_punctuation())
        .rev()
        .collect();
    if digits.len() <= 1 ||
        digits.iter()
            .any(|c| !c.is_ascii_digit()) {
        false
    } else {
        digits.iter()
            .map(|&c| c.to_digit(10)
                .unwrap())
            .enumerate()
            .map(|(i, d)| {
                if i % 2 == 1 {
                    let new_d = d * 2;
                    match new_d {
                        0..=9 => new_d,
                        _ => new_d - 9
                    }
                } else {
                    d
                }
            })
            .sum::<u32>() % 10 == 0
    }
}
