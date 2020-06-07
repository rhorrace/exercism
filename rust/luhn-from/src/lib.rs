use std::fmt::Display;

pub struct Luhn {
    digits: Vec<u32>
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        match self.digits.len() <= 1 {
            true => false,
            false => self.digits.iter()
                .enumerate()
                .map(|(i, &d)| {
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
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: Display> From<T> for Luhn {
    fn from(input: T) -> Self {
        let mut numbers: Vec<u32> = Vec::new();
        let digits: Vec<char> = input.to_string()
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_punctuation())
            .collect();

        if digits.iter()
            .all(|c| c.is_ascii_digit()) {
            numbers = digits.iter()
                .map(|c| c.to_digit(10)
                    .unwrap())
                .rev()
                .collect();
        }

        Self {
            digits: numbers
        }
    }
}
