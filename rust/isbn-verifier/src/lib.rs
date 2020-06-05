/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let digits: Vec<(usize, char)> = isbn.chars()
        .rev()
        .enumerate()
        .filter(|(_, c)| c.is_ascii_alphanumeric())
        .collect();
    let size = digits.len();

    size == 10
        && digits.iter()
        .all(|(i, c)|
            match i {
                0 => c.is_ascii_digit() || *c == 'X',
                _ => c.is_ascii_digit()
            }
        )
        && digits.iter()
        .map(|(_, c)|
            match c {
                'X' => 10,
                _ => c.to_digit(10)
                    .unwrap()
            }
        )
        .zip(1..=10u32)
        .map(|(d, n)| d * n)
        .sum::<u32>() % 11 == 0
}
