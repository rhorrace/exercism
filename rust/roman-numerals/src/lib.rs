use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    roman: String,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.roman)
    }
}

impl From<u32> for Roman {
    fn from(mut num: u32) -> Self {
        let mut roman = String::new();

        if num >= 1000 {
            let size = (num / 1000) as usize;
            roman.push_str(&vec!["M"; size].join(""));
            num %= 1000;
        }

        let numerals: String = num.to_string()
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)|
                match i {
                    0 => to_roman(c, &['I','V','X']),
                    1 => to_roman(c, &['X','L','C']),
                    2 => to_roman(c, &['C','D','M']),
                    _ => format!("")
                }
            )
            .collect::<String>()
            .chars()
            .rev()
            .collect();

        roman.push_str(numerals.as_str());

        Self {
            roman
        }
    }
}

fn to_roman(digit: char, numerals: &[char]) -> String {
    match digit {
        '9' => format!("{}{}", numerals[2], numerals[0]),
        '8' => format!("{}{}{}{}", numerals[0], numerals[0], numerals[0], numerals[1]),
        '7' => format!("{}{}{}", numerals[0], numerals[0], numerals[1]),
        '6' => format!("{}{}", numerals[0], numerals[1]),
        '5' => format!("{}", numerals[1]),
        '4' => format!("{}{}", numerals[1], numerals[0]),
        '3' => format!("{}{}{}", numerals[0], numerals[0], numerals[0]),
        '2' => format!("{}{}", numerals[0], numerals[0]),
        '1' => format!("{}", numerals[0]),
        _ => format!(""),
    }
}