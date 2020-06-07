use std::fmt::Display;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: Display> Luhn for T {
    fn valid_luhn(&self) -> bool {
        is_valid(self.to_string())
    }
}

fn is_valid(input: String) -> bool {
    let digits: Vec<char> = input.chars()
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
