#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter()
        .flat_map(|&v| convert_u32_to_bytes(v))
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result: Vec<u32> = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        let mut j = i;
        while j < bytes.len() && bytes[j] & 128 != 0 {
            j += 1;
        }
        if j == bytes.len() {
            return Err(Error::IncompleteNumber);
        }
        match convert_bytes_to_u32(&bytes[i..=j]) {
            Ok(v) => result.push(v),
            Err(e) => return Err(e)
        }
        i = j + 1;
    }

    Ok(result)
}

fn convert_u32_to_bytes(mut num: u32) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();

    while num & !127 != 0 {
        bytes.push((num & 127) as u8 | 128);
        num >>= 7;
    }
    bytes.push(num as u8 | 128);
    bytes[0] &= 127;
    bytes.reverse();

    bytes
}

fn convert_bytes_to_u32(bytes: &[u8]) -> Result<u32, Error> {
    let mut value = 0u32;
    let overflow = bytes.iter()
        .all(|&b| {
            match value & 0xfe000000 != 0 {
                true => false,
                false => {
                    value = (value << 7) | (b & 127) as u32;
                    true
                }
            }
        });
    match overflow {
        false => Err(Error::Overflow),
        true => Ok(value),
    }
}
