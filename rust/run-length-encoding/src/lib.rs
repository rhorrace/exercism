pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return "".to_string();
    }
    let mut letters: Vec<(char, u64)> = Vec::new();
    source.chars()
        .for_each(|c| {
            let size = letters.len();
            if letters.is_empty() {
                letters.push((c, 1));
            } else if letters[size - 1].0 == c {
                letters[size - 1].1 += 1;
            } else {
                letters.push((c, 1));
            }
        });
    letters.iter()
        .map(|(c, n)|
            if *n == 1 {
                format!("{}", c)
            } else {
                format!("{}{}", n, c)
            })
        .collect::<Vec<String>>()
        .join("")
}

pub fn decode(source: &str) -> String {
    if source.is_empty() {
        return "".to_string();
    }
    let mut decoded: String = String::new();
    let mut number: String = String::new();
    source.chars()
        .for_each(|c| {
            if c.is_ascii_digit() {
                number.push(c);
            } else if number.is_empty() {
                decoded.push(c);
            } else {
                let n = number.parse::<usize>()
                    .unwrap();
                let letters = vec![c; n].iter()
                    .collect::<String>();
                decoded += letters.as_str();
                number.clear();
            }
        });
    decoded
}
