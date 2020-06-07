#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    let invalids: Vec<char> = string_digits.chars()
        .filter(|c| !c.is_ascii_digit())
        .collect();

    if !invalids.is_empty() {
        Err(Error::InvalidDigit(invalids[0]))
    } else if span > string_digits.len() {
        Err(Error::SpanTooLong)
    } else if span == 0 || string_digits.is_empty() {
        Ok(1)
    } else {
        Ok(string_digits.chars()
            .map(|c| c.to_digit(10)
                .unwrap() as u64)
            .collect::<Vec<u64>>()
            .windows(span)
            .map(|numbers| numbers.iter()
                .product::<u64>())
            .max()
            .unwrap())
    }
}
