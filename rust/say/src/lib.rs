pub fn encode(n: u64) -> String {
    match n == 0 {
        true => "zero".to_string(),
        false => number_name(n)
    }
}

fn number_name(mut n: u64) -> String {
    let mut numbers: Vec<String> = vec![];

    let places = ["", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion"];
    let mut index = 0;

    while n != 0 {
        let chunk = n % 1000;
        n /= 1000;

        let current = convert(chunk);

        if !current.is_empty() {
            numbers.push(format!("{} {}", current, places[index]));
        }

        index += 1;
    }
    numbers.reverse();
    numbers.join(" ")
        .trim()
        .to_string()
}

fn convert(n: u64) -> String {
    match n >= 100 {
        true => format!("{} hundred {}", ones(n / 100), convert_less_than_100(n % 100)),
        false => convert_less_than_100(n)
    }
}

fn convert_less_than_100(n: u64) -> String {
    match n {
        0..=9 => ones(n),
        11..=19 => teens(n),
        10 | 20 | 30 | 40 | 50 | 60 | 70 | 80 | 90 => tens(n),
        _ => format!("{}-{}", tens(n - (n % 10)), ones(n % 10)),
    }
}

fn ones(n: u64) -> String {
    match n {
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        _ => "".to_string(),
    }
}

fn teens(n: u64) -> String {
    match n {
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        15 => "fifteen".to_string(),
        18 => "eighteen".to_string(),
        x => format!("{}teen", ones(x % 10))
    }
}

fn tens(n: u64) -> String {
    match n {
        10 => "ten".to_string(),
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        80 => "eighty".to_string(),
        x => format!("{}ty", ones(x / 10)),
    }
}