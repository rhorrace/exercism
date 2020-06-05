/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    (1..26).find(|&v| a * v % 26 == 1)
        .ok_or_else(|| AffineCipherError::NotCoprime(a))?;
    let encoded = plaintext.to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| {
            let v = c as i32;
            if v < 97 {
                return v as u8;
            }
            (((a * (v - 97) + b) % 26) as u8) + 97
        })
        .collect::<Vec<u8>>()
        .chunks(5)
        .map(|v| String::from_utf8(v.to_vec())
            .unwrap())
        .collect::<Vec<String>>()
        .join(" ");
    Ok(encoded)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let x = (1..26).find(|&v| a * v % 26 == 1)
        .ok_or_else(|| AffineCipherError::NotCoprime(a))?;
    let xs: Vec<u8> = ciphertext.to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| {
            let v = c as i32;
            if v < 97 {
                return v as u8;
            }
            let mut i = (x * (((v - 97) as i32) - b)) % 26;
            if i < 0 {
                i += 26;
            }
            (i + 97) as u8
        })
        .collect();
    Ok(String::from_utf8(xs)
        .unwrap())
}
