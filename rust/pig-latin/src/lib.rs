pub fn translate(input: &str) -> String {
    input.split(' ')
        .map(convert)
        .collect::<Vec<String>>()
        .join(" ")
}

fn convert(input: &str) -> String {
    rearrange(input) + "ay"
}

fn rearrange(input: &str) -> String {
    match input.starts_with(|c| is_vowel(c)) ||
        input.starts_with("xr") ||
        input.starts_with("yt") {
        true => input.to_string(),
        false => {
            let split_point: usize = find_split_point(input);
            input[split_point..].to_string() + &input[0..split_point]
        }
    }
}

fn find_split_point(input: &str) -> usize {
    if input.starts_with("y") {
        1
    } else if input.contains("qu") {
        input.find(|c| is_aeioy(c))
            .unwrap()
    } else {
        input.find(|c| is_vowel(c) || c == 'y')
            .unwrap()
    }
}

fn is_vowel(letter: char) -> bool {
    match letter {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn is_aeioy(letter: char) -> bool {
    match letter {
        'a' | 'e' | 'i' | 'o' | 'y' => true,
        _ => false,
    }
}
