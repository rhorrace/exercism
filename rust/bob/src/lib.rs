pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    let contains_letters = trimmed.chars()
        .any(|c| c.is_alphabetic());
    let is_uppercased = trimmed == trimmed.to_uppercase();
    if trimmed.is_empty() {
        "Fine. Be that way!"
    } else if trimmed.ends_with("?") {
        if is_uppercased && contains_letters{
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else {
        if is_uppercased && contains_letters {
            "Whoa, chill out!"
        } else {
            "Whatever."
        }
    }
}
