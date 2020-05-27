pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets: Vec<char> = Vec::new();
    for c in string.trim().chars() {
        if "[({".contains(c){
            brackets.push(c);
        } else if "])}".contains(c) {
            if brackets.is_empty() {
                return false;
            } else if c == ']' && *brackets.last().unwrap() == '[' {
                brackets.pop();
            } else if c == ')' && *brackets.last().unwrap() == '(' {
                brackets.pop();
            } else if c == '}' && *brackets.last().unwrap() == '{' {
                brackets.pop();
            } else {
                return false;
            }
        }
    }
    brackets.is_empty()
}
