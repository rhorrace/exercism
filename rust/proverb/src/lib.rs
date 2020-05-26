pub fn build_proverb(list: &[&str]) -> String {
    let n = list.len();
    let mut lines: Vec<String> = Vec::new();
    if n == 0 {
        return "".to_string();
    } else if n > 1 {
        for i in 0..n - 1 {
            lines.push(format!("For want of a {} the {} was lost.", list[i], list[i + 1]));
        }
    }
    lines.push(format!("And all for the want of a {}.", list[0]));
    lines.join("\n")
}
