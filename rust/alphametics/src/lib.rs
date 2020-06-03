use std::collections::{HashMap, BTreeSet};

struct Alphametic {
    addends: Vec<Vec<char>>,
    digits: [Option<usize>; 10],
    first_letters: BTreeSet<char>,
    letter_digits: Vec<usize>,
    letters: Vec<char>,
    positions: Vec<usize>,
    sum: Vec<char>,
}

impl Alphametic {
    pub fn new(mut operands: Vec<&str>) -> Self {
        let first_letters: BTreeSet<char> = operands.iter()
            .map(|w| w.chars()
                .next()
                .unwrap())
            .collect();
        let sum: Vec<char> = operands.pop()
            .unwrap()
            .chars()
            .rev()
            .collect();
        let addends: Vec<Vec<char>> = operands.iter()
            .map(|w| w.chars()
                .rev()
                .collect())
            .collect();
        let mut unique_position_letters: Vec<BTreeSet<char>> = Vec::new();

        for i in 0..sum.len() {
            let mut letter_set: BTreeSet<char> = addends.iter()
                .filter_map(|addend| addend.get(i))
                .copied()
                .collect();
            letter_set.insert(sum[i]);
            for previous_set in unique_position_letters.iter()
                .take(i) {
                letter_set = letter_set.difference(previous_set)
                    .copied()
                    .collect();
            }
            unique_position_letters.push(letter_set);
        }
        let positions: Vec<usize> = std::iter::once(0)
            .chain(unique_position_letters.iter()
                .scan(0, |s, row| {
                    *s += row.len();
                    Some(*s)
                }))
            .collect();
        let letters: Vec<char> = unique_position_letters.into_iter()
            .flat_map(|row| row.into_iter())
            .collect();
        let letter_digits = vec![0; letters.len()];
        let mut digits = [None; 10];
        (0..10).for_each(|i| digits[i] = Some(i));

        Self {
            addends,
            digits,
            first_letters,
            letter_digits,
            letters,
            positions,
            sum,
        }
    }

    pub fn solve(&mut self) -> Option<(&Vec<char>, &Vec<usize>)> {
        match self.try_solve_index(0, 0) {
            true => Some((&self.letters, &self.letter_digits)),
            false => None,
        }
    }

    fn deinit(&mut self, i: usize) {
        let (start, end) = self.position_range(i);
        for i in start..end {
            let value = self.letter_digits[i];
            self.digits[value] = Some(value);
        }
    }

    fn get_letter_digit(&self, letter: char) -> usize {
        let pos = self.letters.iter()
            .position(|&c| c == letter)
            .unwrap();
        self.letter_digits[pos]
    }

    fn init(&mut self, i: usize) {
        let (start, end) = self.position_range(i);
        for index in start..end {
            self.set_new_digit_for_letter_at(index);
        }
    }

    fn position_range(&self, i: usize) -> (usize, usize) {
        (
            self.positions[i],
            self.positions[i + 1],
        )
    }

    fn set_new_digit_for_letter_at(&mut self, index: usize) {
        let start = match self.first_letters.contains(&self.letters[index]) {
            true => 1,
            false => 0,
        };
        self.letter_digits[index] = self.take_next_available_digit(start).unwrap();
    }

    fn take_next_available_digit(&mut self, mut digit: usize) -> Option<usize> {
        loop {
            if digit > 9 {
                return None;
            }
            match self.digits[digit].take() {
                Some(digit) => return Some(digit),
                None => digit += 1,
            }
        }
    }

    fn try_incrementing_digit_at(&mut self, index: usize) -> bool {
        let i = self.letter_digits[index];

        self.digits[i] = Some(i);
        match self.take_next_available_digit(i + 1) {
            Some(digit) => {
                self.letter_digits[index] = digit;
                true
            }
            None => false
        }
    }

    fn try_increment(&mut self, index: usize) -> bool {
        let (start, end) = self.position_range(index);

        for i in (start..end).rev() {
            if self.try_incrementing_digit_at(i) {
                for j in (i + 1)..end {
                    self.set_new_digit_for_letter_at(j);
                }
                return true;
            }
        }
        false
    }

    fn try_solve_index(&mut self, index: usize, accumulator: usize) -> bool {
        let mut sum;
        if index == self.sum.len() {
            return true;
        }
        self.init(index);
        loop {
            sum = self.addends.iter()
                .filter_map(|w| w.get(index))
                .fold(accumulator, |acc, &c| acc + self.get_letter_digit(c));
            if sum % 10 == self.get_letter_digit(self.sum[index])
                && self.try_solve_index(index + 1, sum / 10) {
                return true;
            }

            if !self.try_increment(index) {
                self.deinit(index);
                return false;
            }
        }
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut alphametic = parse(input);
    let (letters, digits) = alphametic.solve()?;
    Some(format(letters, digits))
}

fn format(letters: &[char], digits: &[usize]) -> HashMap<char, u8> {
    letters.iter()
        .zip(digits.iter())
        .map(|(&l, &v)| (l, v as u8))
        .collect()
}

fn parse(input: &str) -> Alphametic {
    let operands: Vec<&str> = input
        .split(|c: char| !c.is_alphabetic())
        .filter(|w| !w.is_empty())
        .collect();
    Alphametic::new(operands)
}
