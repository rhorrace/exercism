pub fn number(user_number: &str) -> Option<String> {
    let digits: Vec<char> = user_number.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();
    if digits.iter()
        .any(|c| !c.is_ascii_digit()) {
        None
    } else {
        let numbers: Vec<u32> = digits.iter()
            .map(|c| c.to_digit(10)
                .unwrap())
            .skip_while(|&n| n == 1)
            .collect();
        if numbers.len() != 10 || numbers[0] < 2 || numbers[3] < 2 {
            None
        } else {
            Some(numbers.iter()
                .map(|n| n.to_string())
                .collect::<String>())
        }
    }
}
