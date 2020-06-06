// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    if input.lines()
        .count() % 4 != 0 {
        Err(Error::InvalidRowCount(input.lines().count()))
    } else if input.lines()
        .nth(0)
        .unwrap_or("")
        .len() % 3 != 0 {
        Err(Error::InvalidColumnCount(
            input.lines()
                .nth(0)
                .unwrap_or("")
                .len()
        ))
    } else {
        Ok(digits(input))
    }
}

fn digits(input: &str) -> String {
    input.lines()
        .collect::<Vec<&str>>()
        .chunks(4)
        .into_iter()
        .map(|line| line_digits(&line.join("\n")))
        .collect::<Vec<String>>()
        .join(",")
}

fn line_digits(line: &str) -> String {
    let mut line_digits = String::new();
    let rows = line.lines().collect::<Vec<&str>>();
    let size = rows.get(0)
        .unwrap_or(&"")
        .len();
    for i in (0..size).step_by(3) {
        let line: &str = &[
            &rows[0][i..i + 3],
            &rows[1][i..i + 3],
            &rows[2][i..i + 3],
            &rows[3][i..i + 3],
        ].concat();
        line_digits.push(ocr_to_digit(line));
    }
    line_digits
}

fn ocr_to_digit(line: &str) -> char {
    match line {
        " _ | ||_|   " => '0',
        "     |  |   " => '1',
        " _  _||_    " => '2',
        " _  _| _|   " => '3',
        "   |_|  |   " => '4',
        " _ |_  _|   " => '5',
        " _ |_ |_|   " => '6',
        " _   |  |   " => '7',
        " _ |_||_|   " => '8',
        " _ |_| _|   " => '9',
        _ => '?'
    }
}