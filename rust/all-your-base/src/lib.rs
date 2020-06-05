use crate::Error::InvalidDigit;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        Err(Error::InvalidInputBase)
    } else if to_base < 2 {
        Err(Error::InvalidOutputBase)
    } else if number.iter().any(|&d| d >= from_base) {
        let invalid_digit: u32 = *number.iter()
            .filter(|&&d| d >= from_base)
            .next()
            .unwrap();
        Err(InvalidDigit(invalid_digit))
    } else {
        let digits_10 = number.iter()
            .rev()
            .enumerate()
            .map(|(i, d)| d * from_base.pow(i as u32))
            .sum();
        Ok(calculate_to_base(digits_10, to_base))
    }
}

fn calculate_to_base(mut number: u32, to_base: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();
    if number == 0 {
        vec![0]
    } else {
        while number != 0 {
            digits.push(number % to_base);
            number /= to_base;
        }
        digits.reverse();
        digits
    }
}
