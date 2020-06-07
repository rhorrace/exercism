pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {
    if !command.starts_with("What is ") || !command.ends_with("?") {
        None
    } else {
        let trimmed = command.replace("?", "")
            .replace("!", "")
            .replace("What is ", "")
            .replace("by ", "")
            .replace("raised to the", "pow")
            .replace("multiplied", "times")
            .replace("divided", "div")
            .replace("power", "")
            .replace("th", "")
            .replace("nd", "")
            .replace("rd", "")
            .replace("to", "");
        let mut strings: Vec<String> = trimmed.trim()
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if strings.len() % 2 == 0 {
            None
        } else {
            while strings.len() > 1 {
                let trio: Vec<String> = strings.iter()
                    .take(3)
                    .map(|s| s.clone())
                    .collect();
                if !is_valid_trio(&trio) {
                    return None;
                }
                let a: i32 = trio[0].parse()
                    .unwrap();
                let b: i32 = trio[2].parse()
                    .unwrap();
                let c = do_op(a, b, trio[1].as_str());
                let mut new_strings = vec![c.to_string()];
                new_strings.append(&mut strings[3..].to_vec());
                strings = new_strings;
            }
            match strings[0].parse::<i32>() {
                Ok(x) => Some(x),
                Err(_) => None
            }
        }
    }
}

fn is_valid_trio(trio: &[String]) -> bool {
    trio[0].parse::<i32>().is_ok() &&
        trio[2].parse::<i32>().is_ok() &&
        is_valid_op(trio[1].as_str())
}

fn is_valid_op(op: &str) -> bool {
    match op {
        "plus" | "minus" | "times" | "div" | "pow" => true,
        _ => false
    }
}

fn do_op(a: i32, b: i32, op: &str) -> i32 {
    match op {
        "plus" => add(a, b),
        "minus" => sub(a, b),
        "times" => mul(a, b),
        "div" => div(a, b),
        "pow" => pow(a, b),
        _ => 0
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

fn div(a: i32, b: i32) -> i32 {
    a / b
}

fn pow(a: i32, b: i32) -> i32 {
    a.pow(b as u32)
}